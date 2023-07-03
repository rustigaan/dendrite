/// An instruction from the components that provides the Command Handler towards AxonServer.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandSubscription {
    /// A unique identifier for this subscription. This identifier is returned in Acknowledgements to allow
    /// pipelining of subscription messages
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
    /// This information is interpreted by Axon Server in relation to the other connected nodes' values.
    /// Used to balance the dispatching of commands. If set to 0, Axon Server consider 100 as default value.
    #[prost(int32, tag = "5")]
    pub load_factor: i32,
}
/// Generated client implementations.
pub mod command_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// The CommandService defines the gRPC requests necessary for subscribing command handlers, and dispatching commands.
    #[derive(Debug, Clone)]
    pub struct CommandServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CommandServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CommandServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CommandServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Opens a stream allowing clients to register command handlers and receive commands.
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
        /// Dispatches the given command, returning the result of command execution
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
}
/// Generated server implementations.
pub mod command_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CommandServiceServer.
    #[async_trait]
    pub trait CommandService: Send + Sync + 'static {
        /// Server streaming response type for the OpenStream method.
        type OpenStreamStream: futures_core::Stream<Item = Result<super::CommandProviderInbound, tonic::Status>>
            + Send
            + 'static;
        /// Opens a stream allowing clients to register command handlers and receive commands.
        async fn open_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::CommandProviderOutbound>>,
        ) -> Result<tonic::Response<Self::OpenStreamStream>, tonic::Status>;
        /// Dispatches the given command, returning the result of command execution
        async fn dispatch(
            &self,
            request: tonic::Request<super::Command>,
        ) -> Result<tonic::Response<super::CommandResponse>, tonic::Status>;
    }
    /// The CommandService defines the gRPC requests necessary for subscribing command handlers, and dispatching commands.
    #[derive(Debug)]
    pub struct CommandServiceServer<T: CommandService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CommandService> CommandServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CommandServiceServer<T>
    where
        T: CommandService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/io.axoniq.axonserver.grpc.command.CommandService/OpenStream" => {
                    #[allow(non_camel_case_types)]
                    struct OpenStreamSvc<T: CommandService>(pub Arc<T>);
                    impl<T: CommandService>
                        tonic::server::StreamingService<super::CommandProviderOutbound>
                        for OpenStreamSvc<T>
                    {
                        type Response = super::CommandProviderInbound;
                        type ResponseStream = T::OpenStreamStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::CommandProviderOutbound>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).open_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.command.CommandService/Dispatch" => {
                    #[allow(non_camel_case_types)]
                    struct DispatchSvc<T: CommandService>(pub Arc<T>);
                    impl<T: CommandService> tonic::server::UnaryService<super::Command> for DispatchSvc<T> {
                        type Response = super::CommandResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Command>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).dispatch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DispatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: CommandService> Clone for CommandServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CommandService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CommandService> tonic::server::NamedService for CommandServiceServer<T> {
        const NAME: &'static str = "io.axoniq.axonserver.grpc.command.CommandService";
    }
}
