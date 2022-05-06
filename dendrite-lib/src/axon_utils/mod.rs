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

use crate::intellij_work_around::Debuggable;
use anyhow::{anyhow, Result};
use log::debug;
use prost::Message;
use std::future::Future;
use std::pin::Pin;
use futures_core::Stream;
use tonic::{Request, Streaming};
use tonic::transport::Channel;

mod command_submit;
mod command_worker;
mod connection;
mod event_processor;
mod event_query;
mod handler_registry;
mod query_processor;
mod query_submit;

use crate::axon_server::command::command_service_client::CommandServiceClient;
use crate::axon_server::command::{Command, CommandProviderInbound, CommandProviderOutbound, CommandResponse};
use crate::axon_server::event::{Confirmation, Event, GetAggregateEventsRequest};
use crate::axon_server::event::event_store_client::EventStoreClient;
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

/// A handle for AxonServer.
#[derive(Debug, Clone)]
pub struct AxonServerHandle {
    pub display_name: String,
    pub client_id: String,
    pub conn: Channel,
}

impl AxonServerHandle {
    pub fn spawn_ref<T>(&self, task: &'static (dyn Fn(AxonServerHandle) -> T))
    where
        T: Future + Send + 'static,
        T::Output: Send,
    {
        tokio::spawn((Box::new(task))(self.clone()));
    }
    pub fn spawn(&self, task: Box<dyn FnOnce(AxonServerHandle) -> PinFuture<()> + Sync>) {
        tokio::spawn((task)(self.clone()));
    }
}

type AxonServerHandleTraitBox = Box<dyn AxonServerHandleTrait>;

type StaticCommandProviderOutboundStream = dyn Stream<Item = CommandProviderOutbound> + Send + 'static;
type CommandProviderOutboundStreamBox = Pin<Box<StaticCommandProviderOutboundStream>>;

struct ServerHandle {
    conn: Channel
}

#[tonic::async_trait]
trait ServerHandleTrait {
    async fn open_command_provider_inbound_stream(&self, request: CommandProviderOutboundStreamBox)
        -> Result<tonic::Response<Streaming<CommandProviderInbound>>, tonic::Status>;
}
#[tonic::async_trait]
impl ServerHandleTrait for ServerHandle
{
    async fn open_command_provider_inbound_stream(&self, request: CommandProviderOutboundStreamBox)
        -> Result<tonic::Response<Streaming<CommandProviderInbound>>, tonic::Status>
    {
        let request = Request::new(request as CommandProviderOutboundStreamBox);
        let mut client = CommandServiceClient::new(self.conn.clone());
        client.open_stream(request).await
    }
}

pub trait AxonServerHandleTrait: Send + Sync + std::fmt::Debug + AxonServerHandleAsyncTrait {
    fn client_id(&self) -> &str;
    fn display_name(&self) -> &str;
    fn box_clone(&self) -> AxonServerHandleTraitBox;
}
#[tonic::async_trait]
pub trait AxonServerHandleAsyncTrait
{
    async fn dispatch(&self, request: Command)
        -> Result<tonic::Response<CommandResponse>, tonic::Status>;
    async fn open_command_provider_inbound_stream(&self, request: CommandProviderOutboundStreamBox)
        -> Result<tonic::Response<Streaming<CommandProviderInbound>>, tonic::Status>;
    async fn list_aggregate_events(&self, request: GetAggregateEventsRequest)
        -> Result<tonic::Response<Streaming<Event>>, tonic::Status>;
    async fn append_events(&self, events: Vec<Event>)
        -> Result<tonic::Response<Confirmation>, tonic::Status>;
}
impl AxonServerHandleTrait for AxonServerHandle {
    fn client_id(&self) -> &str {
        &self.client_id
    }
    fn display_name(&self) -> &str {
        &self.display_name
    }
    fn box_clone(&self) -> AxonServerHandleTraitBox {
        Box::new(core::clone::Clone::clone(self))
    }
}
#[tonic::async_trait]
impl AxonServerHandleAsyncTrait for AxonServerHandle
{
    async fn dispatch(&self, request: Command)
        -> Result<tonic::Response<CommandResponse>, tonic::Status>
    {
        let mut client = CommandServiceClient::new(self.conn.clone());
        client.dispatch(request).await
    }
    async fn open_command_provider_inbound_stream(&self, request: CommandProviderOutboundStreamBox)
        -> Result<tonic::Response<Streaming<CommandProviderInbound>>, tonic::Status>
    {
        let mut client = CommandServiceClient::new(self.conn.clone());
        client.open_stream(request).await
    }
    async fn list_aggregate_events(&self, request: GetAggregateEventsRequest)
                                   -> Result<tonic::Response<Streaming<Event>>, tonic::Status>
    {
        let mut client = EventStoreClient::new(self.conn.clone());
        client.list_aggregate_events(request).await
    }
    async fn append_events(&self, events: Vec<Event>)
        -> Result<tonic::Response<Confirmation>, tonic::Status>
    {
        let mut client = EventStoreClient::new(self.conn.clone());
        let request = Request::new(futures_util::stream::iter(events));
        client.append_event(request).await
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
    Self: VecU8Message + Send + Sync + std::fmt::Debug,
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
    Self: VecU8Message + Send + Sync + std::fmt::Debug,
{
    /// Applies this message to the given projection.
    async fn apply_to(self, metadata: Metadata, projection: &mut Projection) -> Result<()>;

    fn box_clone(&self) -> Box<dyn AsyncApplicableTo<Projection, Metadata>>;
}
