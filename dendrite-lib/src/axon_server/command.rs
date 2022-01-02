/// An instruction from the components that provides the Command Handler towards AxonServer.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CommandProviderOutbound {
    /// Instruction identifier. If this identifier is set, this instruction will be acknowledged via inbound stream
    #[prost(string, tag = "6")]
    pub instruction_id: ::prost::alloc::string::String,
    /// The instruction for AxonServer
    #[prost(oneof = "command_provider_outbound::Request", tags = "1, 2, 3, 4, 5")]
    pub request: ::core::option::Option<command_provider_outbound::Request>,
}
/// Nested message and enum types in `CommandProviderOutbound`.
pub mod command_provider_outbound {
    /// The instruction for AxonServer
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Instruction to subscribe this component as handler of a specific type of command
        #[prost(message, tag = "1")]
        Subscribe(super::CommandSubscription),
        /// Instruction to unsubscribe this component as handler of a specific type of command
        #[prost(message, tag = "2")]
        Unsubscribe(super::CommandSubscription),
        /// Instruction to increase the number of instructions AxonServer may send to this component
        #[prost(message, tag = "3")]
        FlowControl(super::super::common::FlowControl),
        /// Sends a result of Command processing
        #[prost(message, tag = "4")]
        CommandResponse(super::CommandResponse),
        /// Acknowledgement of previously sent instruction via inbound stream
        #[prost(message, tag = "5")]
        Ack(super::super::common::InstructionAck),
    }
}
/// An instruction or confirmation from AxonServer towards the component that provides the Command Handler
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CommandProviderInbound {
    /// Instruction identifier. If this identifier is set, this instruction will be acknowledged via outbound stream
    #[prost(string, tag = "3")]
    pub instruction_id: ::prost::alloc::string::String,
    /// The instruction from AxonServer for this component
    #[prost(oneof = "command_provider_inbound::Request", tags = "1, 2")]
    pub request: ::core::option::Option<command_provider_inbound::Request>,
}
/// Nested message and enum types in `CommandProviderInbound`.
pub mod command_provider_inbound {
    /// The instruction from AxonServer for this component
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Acknowledgement of previously sent instruction via outbound stream
        #[prost(message, tag = "1")]
        Ack(super::super::common::InstructionAck),
        /// A command for this component to process
        #[prost(message, tag = "2")]
        Command(super::Command),
    }
}
/// A message representing a Command that needs to be routed to a component capable of handling it
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Command {
    /// The unique identifier of the Command Message
    #[prost(string, tag = "1")]
    pub message_identifier: ::prost::alloc::string::String,
    /// The name of the command, used for routing it to a destination capable of handling it
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The time at which the command was dispatched
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    /// The payload of the Command, providing details on the instructions for the recipient
    #[prost(message, optional, tag = "4")]
    pub payload: ::core::option::Option<super::common::SerializedObject>,
    /// Meta Data entries of the Command Message, providing contextual information to the recipient
    #[prost(map = "string, message", tag = "5")]
    pub meta_data:
        ::std::collections::HashMap<::prost::alloc::string::String, super::common::MetaDataValue>,
    /// Instructions for AxonServer when routing this Command Message
    #[prost(message, repeated, tag = "6")]
    pub processing_instructions: ::prost::alloc::vec::Vec<super::common::ProcessingInstruction>,
    /// The unique identifier of the component dispatching this message
    #[prost(string, tag = "7")]
    pub client_id: ::prost::alloc::string::String,
    /// The name/type of the component dispatching this message
    #[prost(string, tag = "8")]
    pub component_name: ::prost::alloc::string::String,
}
/// Message representing the result of Command Handler execution
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    /// The unique identifier of the response message
    #[prost(string, tag = "1")]
    pub message_identifier: ::prost::alloc::string::String,
    /// An error code describing the error, if any
    #[prost(string, tag = "2")]
    pub error_code: ::prost::alloc::string::String,
    /// A detailed description of the error
    #[prost(message, optional, tag = "3")]
    pub error_message: ::core::option::Option<super::common::ErrorMessage>,
    /// The payload to provide as a result to the dispatcher
    #[prost(message, optional, tag = "4")]
    pub payload: ::core::option::Option<super::common::SerializedObject>,
    /// Any meta data entries providing contextual information back to the dispatcher
    #[prost(map = "string, message", tag = "5")]
    pub meta_data:
        ::std::collections::HashMap<::prost::alloc::string::String, super::common::MetaDataValue>,
    /// Instructions for AxonServer when routing this Command Response Message
    #[prost(message, repeated, tag = "6")]
    pub processing_instructions: ::prost::alloc::vec::Vec<super::common::ProcessingInstruction>,
    /// The unique identifier of the Command Message for which this is the response
    #[prost(string, tag = "7")]
    pub request_identifier: ::prost::alloc::string::String,
}
/// Message describing a component's capability of handling a command type
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CommandSubscription {
    /// A unique identifier for this subscription. This identifier is returned in Acknowledgements to allow
    ///pipelining of subscription messages
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    /// The name of the command the component can handle
    #[prost(string, tag = "2")]
    pub command: ::prost::alloc::string::String,
    /// The name/type of the component handling the command
    #[prost(string, tag = "3")]
    pub component_name: ::prost::alloc::string::String,
    /// The unique identifier of the component instance subscribing
    #[prost(string, tag = "4")]
    pub client_id: ::prost::alloc::string::String,
    /// A number that represents the client's relative load capacity compared to other clients.
    ///This information is interpreted by Axon Server in relation to the other connected nodes' values.
    ///Used to balance the dispatching of commands. If set to 0, Axon Server consider 100 as default value.
    #[prost(int32, tag = "5")]
    pub load_factor: i32,
}
#[doc = r" Generated client implementations."]
pub mod command_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The CommandService defines the gRPC requests necessary for subscribing command handlers, and dispatching commands. "]
    pub struct CommandServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CommandServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CommandServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Opens a stream allowing clients to register command handlers and receive commands. "]
        pub async fn open_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::CommandProviderOutbound>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CommandProviderInbound>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.command.CommandService/OpenStream",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Dispatches the given command, returning the result of command execution "]
        pub async fn dispatch(
            &mut self,
            request: impl tonic::IntoRequest<super::Command>,
        ) -> Result<tonic::Response<super::CommandResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.command.CommandService/Dispatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CommandServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CommandServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CommandServiceClient {{ ... }}")
        }
    }
}
