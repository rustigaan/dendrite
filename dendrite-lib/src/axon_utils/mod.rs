//! # Axon Utilities
//!
//! Module `axon_utils` exports items that can be used to create a complete Event Sourced CQRS
//! application that uses [AxonServer](https://axoniq.io/product-overview/axon-server) as an event store.
//! The whole back-end can be packaged as a single application, but when growing load or complexity
//! demands it, aspects can be taken out and converted to microservices that can be scaled horizontally.
//!
//! The basic parts of a dendrite application are:
//! * Command API — _accepts commands on a gRPC API and forwards them (again over gRPC to AxonServer)_
//! * Command worker — _subscribes to commands, verifies them against the command projection and sends emitted events to AxonServer_
//! * Event processor — _subscribes to events and builds a query model from them (there are likely to be multiple query models for a single application)_
//! * Query API — _accepts queries on a gRPC API and forwards them (again over gRPC to AxonServer)_
//! * Query processor — _subscribes to queries, executes them against a query model and pass back the results_

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::intellij_work_around::Debuggable;
use anyhow::{anyhow, Result};
use async_channel::{Sender,Receiver,bounded};
use log::{debug, error, info};
use prost::Message;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use futures_util::TryFutureExt;
use futures_util::FutureExt;
use tokio::select;
use tokio::signal::unix::Signal;
use tokio::task::JoinHandle;
use tonic::{Response, Status};
use tonic::transport::Channel;
use uuid::Uuid;

mod command_submit;
mod command_worker;
mod connection;
mod event_processor;
mod event_query;
mod handler_registry;
mod query_processor;
mod query_submit;

use crate::axon_server::command::command_service_client::CommandServiceClient;
use crate::axon_server::command::{Command,CommandResponse};
pub use crate::axon_server::SerializedObject;
use crate::axon_utils::handler_registry::PinFuture;
pub use command_submit::init as init_command_sender;
pub use command_submit::SubmitCommand;
pub use command_worker::command_worker;
pub use command_worker::{
    create_aggregate_definition, emit, emit_events, emit_events_and_response,
    empty_aggregate_registry, AggregateContext, AggregateContextTrait, AggregateDefinition,
    AggregateRegistry, EmitApplicableEventsAndResponse, TheAggregateRegistry,
};
pub use connection::{platform_worker, platform_worker_for, wait_for_server};
pub use event_processor::{event_processor, TokenStore};
pub use event_query::query_events;
pub use handler_registry::empty_handler_registry;
pub use handler_registry::{HandleBuilder, HandlerRegistry, TheHandlerRegistry};
pub use query_processor::{query_processor, QueryContext, QueryResult};
use crate::axon_utils::WorkerCommand::Unsubscribe;

struct WorkerRegistry {
    workers: HashMap<Uuid,WorkerHandle>,
    notifications: Receiver<Uuid>,
}

impl Debug for WorkerRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[WorkerRegistry:")?;
        for worker in self.workers.values() {
            f.write_str(&format!("{:?}->{:?},", worker.id, worker.label))?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

/// A handle for AxonServer.
#[derive(Debug, Clone)]
pub struct AxonServerHandle {
    pub display_name: String,
    pub client_id: String,
    pub conn: Channel,
    pub notify: Sender<Uuid>,
    registry: Arc<Mutex<WorkerRegistry>>,
}

impl AxonServerHandle {
    fn has_workers(&self) -> Result<bool> {
        let registry = self.registry.lock();
        let registry = registry.map_err(|e| anyhow!(e.to_string()))?;
        debug!("Remaining workers: {:?}", &registry.workers);
        Ok(!registry.workers.is_empty())
    }

    fn remove_worker(&self, id: &Uuid) -> Result<()> {
        let mut registry = self.registry.lock();
        let registry = registry.as_mut().map_err(|e| anyhow!(e.to_string()))?;
        registry.workers.remove(id);
        Ok(())
    }

    async fn get_stopped_worker(&self) -> Result<Uuid> {
        let stopped_worker_receiver = self.get_stopped_worker_receiver()?;
        stopped_worker_receiver.recv().await.map_err(|e| anyhow!(e.to_string()))
    }

    async fn get_stopped_worker_with_signal(&self, signal_option: &mut Option<Signal>) -> Result<Uuid> {
        match signal_option {
            Some(signal) => {
                let stopped_worker_receiver = self.get_stopped_worker_receiver()?;
                select! {
                    id = stopped_worker_receiver.recv() => id.map_err(|e| anyhow!(e.to_string())),
                    _ = signal.recv() => Ok(Uuid::new_v4())
                }
            },
            None => self.get_stopped_worker().await
        }
    }

    fn get_stopped_worker_receiver(&self) -> Result<Receiver<Uuid>> {
        let registry = self.registry.lock();
        let registry = registry.map_err(|e| anyhow!(e.to_string()))?;
        Ok(registry.notifications.clone())
    }
}

#[derive(Eq,PartialEq)]
pub enum WorkerCommand {
    Unsubscribe,
    Stop,
}

pub struct WorkerHandle {
    id: Uuid,
    join_handle: Option<Pin<Box<dyn Future<Output=Result<()>> + Send>>>,
    control_channel: Sender<WorkerCommand>,
    label: String,
}

pub struct WorkerControl {
    control_channel: Receiver<WorkerCommand>,
    label: String,
}

impl WorkerControl {
    pub fn get_label(&self) -> &str {
        &*self.label
    }
    pub fn get_control_channel(&self) -> Receiver<WorkerCommand> {
        self.control_channel.clone()
    }
}

impl WorkerHandle {
    pub fn get_id(&self) -> Uuid {
        self.id
    }
    pub fn get_join_handle(&mut self) -> &mut Option<Pin<Box<dyn Future<Output=Result<()>> + Send>>> {
        &mut self.join_handle
    }
    pub fn get_control_channel(&self) -> &Sender<WorkerCommand> {
        &self.control_channel
    }
}

impl Debug for WorkerHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[WorkerHandle:")?;
        self.id.fmt(f)?;
        f.write_str(":")?;
        self.label.fmt(f)?;
        f.write_str("]")?;
        Ok(())
    }
}

impl AxonServerHandle {
    pub fn spawn_ref<T, S: Into<String>>(&self, label: S, task: &'static (dyn Fn(AxonServerHandle, WorkerControl) -> T)) -> Result<()>
    where
        T: Future<Output=()> + Send + 'static
    {
        let label = label.into();
        let notify = self.notify.clone();
        let id = Uuid::new_v4();
        let (tx, rx) = bounded(10);
        let worker_control = WorkerControl {
            control_channel: rx,
            label: label.clone(),
        };
        let worker_future = (Box::new(task))((*self).clone(), worker_control);
        let join_handle = spawn_worker(worker_future, notify, id).map_err(Into::into);
        let handle = WorkerHandle {
            id, join_handle: Some(Box::pin(join_handle)), control_channel: tx, label
        };
        let mut registry = self.registry.lock();
        let registry = registry.as_mut().map_err(|e| anyhow!(e.to_string()))?;
        registry.workers.insert(id, handle);
        Ok(())
    }
    pub fn spawn<S: Into<String>>(&self, label: S, task: Box<dyn FnOnce(AxonServerHandle, WorkerControl) -> PinFuture<()>>) -> Result<Uuid> {
        let label = label.into();
        let notify = self.notify.clone();
        let id = Uuid::new_v4();
        let (tx, rx) = bounded(10);
        let worker_control = WorkerControl {
            control_channel: rx,
            label: label.clone(),
        };
        let worker_future = (task)((*self).clone(), worker_control);
        let join_handle = spawn_worker(worker_future, notify, id).map_err(Into::into);
        let handle = WorkerHandle {
            id, join_handle: Some(Box::pin(join_handle)), control_channel: tx, label
        };
        let mut registry = self.registry.lock();
        let registry = registry.as_mut().map_err(|e| anyhow!(e.to_string()))?;
        registry.workers.insert(id, handle);
        Ok(id)
    }
}

fn spawn_worker<T>(future: T, notify: Sender<Uuid>, id: Uuid) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::spawn(future.then(move |result| {
        async move {
            if let Err(e) = notify.send(id.clone()).await {
                debug!("Termination notification failed for worker: {:?}: {:?}", id.clone(), e);
            }
            info!("Worker stopped: {:?}", id);
            result
        }
    }))
}

pub trait AxonServerHandleTrait: Sync + AxonServerHandleAsyncTrait {
    fn client_id(&self) -> &str;
    fn display_name(&self) -> &str;
}
#[tonic::async_trait]
pub trait AxonServerHandleAsyncTrait
{
    async fn dispatch(&self, request: Command)
        -> Result<Response<CommandResponse>, Status>;
    async fn join_workers(&self) -> Result<()>;
    async fn join_workers_with_signal(&self, terminate: &mut Option<Signal>) -> Result<()>;
}

impl AxonServerHandleTrait for AxonServerHandle {
    fn client_id(&self) -> &str {
        &self.client_id
    }
    fn display_name(&self) -> &str {
        &self.display_name
    }
}

#[tonic::async_trait]
impl AxonServerHandleAsyncTrait for AxonServerHandle
{
    async fn dispatch(&self, request: Command) -> Result<Response<CommandResponse>, Status>
    {
        let mut client = CommandServiceClient::new(self.conn.clone());
        client.dispatch(request).await
    }
    async fn join_workers(&self) -> Result<()> {
        let mut never: Option<Signal> = None;
        self.join_workers_with_signal(&mut never).await
    }
    async fn join_workers_with_signal(&self, terminate: &mut Option<Signal>) -> Result<()> {
        if !self.has_workers()? {
            return Ok(());
        }
        let stopped_worker = self.get_stopped_worker_with_signal(terminate).await?;
        self.remove_worker(&stopped_worker)?;
        let senders = {
            let mut registry = self.registry.lock();
            let registry = registry.as_mut().map_err(|e| anyhow!(e.to_string()))?;
            let mut pairs = Vec::new();
            for worker in registry.workers.values() {
                info!("Worker: {:?}: {:?}", &worker.id, &worker.label);
                pairs.push((worker.id, worker.control_channel.clone()));
            }
            pairs
        };
        for (worker_id, sender) in senders {
            info!("{:?}: Stopping", worker_id);
            sender.send(Unsubscribe).await.map_err(|e| {error!("Error while sending 'Unsubscribe': {:?}: {:?}", e, worker_id); ()}).ok();
            sender.send(WorkerCommand::Stop).await.map_err(|e| {error!("Error while sending 'Stop': {:?}: {:?}", e, worker_id); ()}).ok();
        }
        while self.has_workers()? {
            let stopped_worker = self.get_stopped_worker().await?;
            self.remove_worker(&stopped_worker)?;
        }
        Ok(())
    }
}

/// Describes a message that can be serialized to a mutable `Vec<u8>`.
pub trait VecU8Message {
    fn encode_u8(&self, buf: &mut Vec<u8>) -> Result<()>;
}

impl<T> VecU8Message for T
where
    T: Message + Sized,
{
    fn encode_u8(&self, buf: &mut Vec<u8>) -> Result<()> {
        self.encode(buf).map_err(|e| {
            anyhow!(
                "Prost encode error: {:?}: {:?}",
                e.required_capacity(),
                e.remaining()
            )
        })
    }
}

/// Trait that is implemented by an object that can be used to send commands to AxonServer.
#[tonic::async_trait]
pub trait CommandSink {
    #[deprecated(since = "0.8.0", note = "Use struct `SubmitCommand` instead")]
    async fn send_command(
        &self,
        command_type: &str,
        command: &(dyn VecU8Message + Sync),
    ) -> Result<Option<SerializedObject>>;
}

/// Trait that is implemented by an object that can be used to send queries to AxonServer.
#[tonic::async_trait]
pub trait QuerySink {
    async fn send_query<'a>(
        &self,
        query_type: &str,
        query: &(dyn VecU8Message + Sync),
    ) -> Result<Vec<SerializedObject>>;
}

/// Converts a `prost::Message` to an Axon `SerializedObject`.
pub fn axon_serialize<T: Message>(type_name: &str, message: &T) -> Result<SerializedObject> {
    let mut buf = Vec::new();
    message.encode(&mut buf)?;
    let result = SerializedObject {
        r#type: type_name.to_string(),
        revision: "".to_string(),
        data: buf,
    };
    debug!("Encoded output: {:?}", Debuggable::from(&result));
    Ok(result)
}

/// Describes a `Message` that is applicable to a particular projection type.
pub trait ApplicableTo<Projection, Metadata>
where
    Self: VecU8Message + Send + Sync + Debug,
{
    /// Applies this message to the given projection.
    fn apply_to(self, metadata: Metadata, projection: &mut Projection) -> Result<()>; // the self type is implicit

    /// Creates a box with a clone of this message.
    fn box_clone(&self) -> Box<dyn ApplicableTo<Projection, Metadata>>;
}

/// Describes a `Message` that is asynchronously applicable to a particular projection type.
#[tonic::async_trait]
pub trait AsyncApplicableTo<Projection, Metadata>
where
    Self: VecU8Message + Send + Sync + Debug,
{
    /// Applies this message to the given projection.
    async fn apply_to(self, metadata: Metadata, projection: &mut Projection) -> Result<()>;

    fn box_clone(&self) -> Box<dyn AsyncApplicableTo<Projection, Metadata>>;
}
