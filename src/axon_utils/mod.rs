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

use anyhow::{anyhow,Result};
use log::debug;
use prost::Message;
use tonic::transport::Channel;

mod command_submit;
mod command_worker;
mod connection;
mod event_processor;
mod event_query;
mod handler_registry;
mod query_processor;
mod query_submit;

pub use crate::axon_server::SerializedObject;
pub use command_submit::init as init_command_sender;
pub use command_worker::command_worker as command_worker;
pub use command_worker::{AggregateContext, AggregateDefinition, AggregateRegistry, EmitApplicableEventsAndResponse, TheAggregateRegistry, create_aggregate_definition, emit, emit_events_and_response, emit_events, empty_aggregate_registry};
pub use connection::wait_for_server as wait_for_server;
pub use handler_registry::empty_handler_registry as empty_handler_registry;
pub use handler_registry::{HandlerRegistry,TheHandlerRegistry};
pub use event_processor::{TokenStore,event_processor};
pub use event_query::query_events;
pub use query_processor::{QueryContext,QueryResult,query_processor};

/// A handle for AxonServer.
#[derive(Debug, Clone)]
pub struct AxonServerHandle {
    pub display_name: String,
    pub conn: Channel,
}

/// A connection to AxonServer.
#[derive(Debug,Clone)]
pub struct AxonConnection {
    pub id: String,
    pub conn: Channel,
}

/// Describes a message that can be serialized to a mutable `Vec<u8>`.
pub trait VecU8Message {
    fn encode_u8(&self, buf: &mut Vec<u8>) -> Result<()>;
}

impl<T> VecU8Message for T
where T: Message + Sized
{
    fn encode_u8(&self, buf: &mut Vec<u8>) -> Result<()> {
        self.encode(buf).map_err(|e| anyhow!("Prost encode error: {:?}: {:?}", e.required_capacity(), e.remaining()))
    }
}

/// Trait that is implemented by an object that can be used to send commands to AxonServer.
#[tonic::async_trait]
pub trait CommandSink {
    async fn send_command(&self, command_type: &str, command: Box<&(dyn VecU8Message + Sync)>) -> Result<Option<SerializedObject>>;
}

/// Trait that is implemented by an object that can be used to send queries to AxonServer.
#[tonic::async_trait]
pub trait QuerySink {
    async fn send_query<'a>(&self, query_type: &str, query: Box<&(dyn VecU8Message + Sync)>) -> Result<Vec<SerializedObject>>;
}

/// Converts a `prost::Message` to an Axon `SerializedObject`.
pub fn axon_serialize<T: Message>(type_name: &str, message: &T) -> Result<SerializedObject> {
    let mut buf = Vec::new();
    message.encode(& mut buf)?;
    let result = SerializedObject {
        r#type: type_name.to_string(),
        revision: "".to_string(),
        data: buf,
    };
    debug!("Encoded output: {:?}", result);
    Ok(result)
}

/// Describes a `Message` that is applicable to a particular projection type.
pub trait ApplicableTo<Projection> where Self: VecU8Message + Send + Sync + std::fmt::Debug {

    /// Applies this message to the given projection.
    fn apply_to(self: &Self, projection: &mut Projection) -> Result<()>;

    /// Creates a box with a clone of this message.
    fn box_clone(self: &Self) -> Box<dyn ApplicableTo<Projection>>;
}

/// Describes a `Message` that is asynchronously applicable to a particular projection type.
#[tonic::async_trait]
pub trait AsyncApplicableTo<Projection> where Self: VecU8Message + Send + Sync + std::fmt::Debug {

    /// Applies this message to the given projection.
    async fn apply_to(self: &Self, projection: &mut Projection) -> Result<()>;

    fn box_clone(self: &Self) -> Box<dyn AsyncApplicableTo<Projection>>;
}
