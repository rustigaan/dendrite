/// An instruction from Application Node to the AxonServer platform
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformInboundInstruction {
    /// Instruction identifier. If this identifier is set, this instruction will be acknowledged via outbound stream
    #[prost(string, tag = "5")]
    pub instruction_id: ::prost::alloc::string::String,
    /// The actual instruction to send
    #[prost(
        oneof = "platform_inbound_instruction::Request",
        tags = "1, 2, 3, 4, 6"
    )]
    pub request: ::core::option::Option<platform_inbound_instruction::Request>,
}
/// Nested message and enum types in `PlatformInboundInstruction`.
pub mod platform_inbound_instruction {
    /// The actual instruction to send
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Information about the client being connected.
        ///This information is used by AxonServer to monitor the topology of connected applications.
        #[prost(message, tag = "1")]
        Register(super::ClientIdentification),
        /// Information about Tracking Processors defined in the application.
        ///This information is used by AxonServer to monitor the progress of Tracking Processors across instances.
        #[prost(message, tag = "2")]
        EventProcessorInfo(super::EventProcessorInfo),
        /// This heartbeat is used by AxonServer in order to check if the connection is still alive
        #[prost(message, tag = "3")]
        Heartbeat(super::Heartbeat),
        /// Acknowledgement of previously sent instruction via outbound stream
        #[prost(message, tag = "4")]
        Ack(super::super::common::InstructionAck),
        /// The result of the execution of an instruction
        #[prost(message, tag = "6")]
        Result(super::super::common::InstructionResult),
    }
}
/// An instruction or information from the AxonServer Platform to the Application Node
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformOutboundInstruction {
    /// Instruction identifier. If this identifier is set, this instruction will be acknowledged via inbound stream
    #[prost(string, tag = "12")]
    pub instruction_id: ::prost::alloc::string::String,
    /// The actual instruction or information
    #[prost(
        oneof = "platform_outbound_instruction::Request",
        tags = "1, 3, 4, 5, 6, 7, 8, 9, 10, 11"
    )]
    pub request: ::core::option::Option<platform_outbound_instruction::Request>,
}
/// Nested message and enum types in `PlatformOutboundInstruction`.
pub mod platform_outbound_instruction {
    /// The actual instruction or information
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Information provided by AxonServer which provides information about the AxonServer node the application is connected with
        #[prost(message, tag = "1")]
        NodeNotification(super::NodeInfo),
        /// A request from AxonServer to the Application to migrate its connection to another node.
        ///Clients SHOULD honor this request by closing their current connection, and using the GetPlatformServer RPC
        ///to request a new destination.
        #[prost(message, tag = "3")]
        RequestReconnect(super::RequestReconnect),
        /// Instruction from AxonServer to Pause a Tracking Event Processor.
        #[prost(message, tag = "4")]
        PauseEventProcessor(super::EventProcessorReference),
        /// Instruction from AxonServer to Start a Tracking Event Processor.
        #[prost(message, tag = "5")]
        StartEventProcessor(super::EventProcessorReference),
        /// Instruction from AxonServer to Release a specific segment in a Tracking Event Processor
        #[prost(message, tag = "6")]
        ReleaseSegment(super::EventProcessorSegmentReference),
        /// A request from AxonServer for status information of a specific Tracking Event Processor
        #[prost(message, tag = "7")]
        RequestEventProcessorInfo(super::EventProcessorReference),
        /// Instruction to split a Segment in a Tracking Event Processor
        #[prost(message, tag = "8")]
        SplitEventProcessorSegment(super::EventProcessorSegmentReference),
        /// Instruction to merge two Segments in a Tracking Event Processor
        #[prost(message, tag = "9")]
        MergeEventProcessorSegment(super::EventProcessorSegmentReference),
        /// This heartbeat is used by AxonFramework in order to check if the connection is still alive
        #[prost(message, tag = "10")]
        Heartbeat(super::Heartbeat),
        /// Acknowledgement of previously sent instruction via inbound stream
        #[prost(message, tag = "11")]
        Ack(super::super::common::InstructionAck),
    }
}
/// Message send when AxonServer requests the client to re-establish its connection with the Platform
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestReconnect {}
/// Message containing connection information of the node to Connect with
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformInfo {
    /// The connection details of the node the client should connect with
    #[prost(message, optional, tag = "1")]
    pub primary: ::core::option::Option<NodeInfo>,
    /// Flag indicating that the connection may be reused to connect. When true, the client _may_ reuse the connection
    ///established for the GetPlatformServer request for subsequent requests.
    #[prost(bool, tag = "2")]
    pub same_connection: bool,
}
/// Message containing connection information for an AxonServer Node
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    /// The host name to use when connecting to this node
    #[prost(string, tag = "1")]
    pub host_name: ::prost::alloc::string::String,
    /// The port number for gRPC connections
    #[prost(int32, tag = "2")]
    pub grpc_port: i32,
    /// The port number for HTTP connections
    #[prost(int32, tag = "3")]
    pub http_port: i32,
    /// The version identifier of the API
    #[prost(int32, tag = "4")]
    pub version: i32,
    /// The unique name of the node to connect with, for purpose of debugging
    #[prost(string, tag = "5")]
    pub node_name: ::prost::alloc::string::String,
}
/// Message containing details about the Client Application
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientIdentification {
    /// A unique identifier for this client instance. Is used to distinguish different instances of the same component
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// The name of the component. Several instances of the same component should share this name
    #[prost(string, tag = "2")]
    pub component_name: ::prost::alloc::string::String,
    /// Any tags associated with the client, which may provide hints and preferences for setting up connections
    #[prost(map = "string, string", tag = "3")]
    pub tags:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Axon framework version used by the client application instance
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
}
/// Message containing information about the status of a Tracking Event Processor
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventProcessorInfo {
    /// The logical name of this processor.
    #[prost(string, tag = "1")]
    pub processor_name: ::prost::alloc::string::String,
    /// The mode in which this processor is reading Events, for example: 'Tracking' or 'Subscribing'
    #[prost(string, tag = "2")]
    pub mode: ::prost::alloc::string::String,
    /// The number of threads currently actively processing Events
    #[prost(int32, tag = "3")]
    pub active_threads: i32,
    /// Flag indicating whether the processor is running
    #[prost(bool, tag = "4")]
    pub running: bool,
    /// Flag indicating whether the processor, when stopped, did so because of an irrecoverable Error
    #[prost(bool, tag = "5")]
    pub error: bool,
    /// Status details of each of the Segments for which Events are being processed. This is only provided by Tracking
    ///Event Processors.
    #[prost(message, repeated, tag = "6")]
    pub segment_status: ::prost::alloc::vec::Vec<event_processor_info::SegmentStatus>,
    /// The number of threads the processor has available to assign to Segments.
    ///Will report 0 if all threads are assigned a Segment.
    #[prost(int32, tag = "7")]
    pub available_threads: i32,
    /// The Token Store Identifier if available. This is only provided by Tracking Event Processors.
    #[prost(string, tag = "8")]
    pub token_store_identifier: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EventProcessorInfo`.
pub mod event_processor_info {
    /// Message containing information about the status of a Segment of a Tracking Event Processor
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SegmentStatus {
        /// The ID of the Segment for which the status is reported
        #[prost(int32, tag = "1")]
        pub segment_id: i32,
        /// Indicates whether the Segment has "Caught Up" with the Head of the Event Stream
        #[prost(bool, tag = "2")]
        pub caught_up: bool,
        /// Indicates whether the Segment is "Replaying" historic events after a Reset.
        #[prost(bool, tag = "3")]
        pub replaying: bool,
        /// The fraction this segment processes. A fraction of 2 means 1/2, 4 means 1/4, etc.
        #[prost(int32, tag = "4")]
        pub one_part_of: i32,
        /// The approximate position of the token in the stream.
        #[prost(int64, tag = "5")]
        pub token_position: i64,
        /// Information about the error state of the Segment, if applicable.
        #[prost(string, tag = "6")]
        pub error_state: ::prost::alloc::string::String,
    }
}
/// Message providing reference to an Event Processor
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventProcessorReference {
    /// The name of the Event Processor
    #[prost(string, tag = "1")]
    pub processor_name: ::prost::alloc::string::String,
}
/// Message providing reference to a Segment of an Event Processor
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventProcessorSegmentReference {
    /// The name of the Event Processor
    #[prost(string, tag = "1")]
    pub processor_name: ::prost::alloc::string::String,
    /// The identifier of the Segment
    #[prost(int32, tag = "2")]
    pub segment_identifier: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {}
#[doc = r" Generated client implementations."]
pub mod platform_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service describing operations for connecting to the AxonServer platform."]
    #[doc = ""]
    #[doc = "Clients are expected to use this service on any of the Platform's Admin nodes to obtain connection information of the"]
    #[doc = "node that it should set up the actual connection with. On that second node, the clients should open an instruction"]
    #[doc = "stream (see OpenStream), so that AxonServer and the client application can exchange information and instructions."]
    pub struct PlatformServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlatformServiceClient<tonic::transport::Channel> {
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
    impl<T> PlatformServiceClient<T>
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
        #[doc = " Obtains connection information for the Server that a Client should use for its connections. "]
        pub async fn get_platform_server(
            &mut self,
            request: impl tonic::IntoRequest<super::ClientIdentification>,
        ) -> Result<tonic::Response<super::PlatformInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.control.PlatformService/GetPlatformServer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Opens an instruction stream to the Platform, allowing AxonServer to provide management instructions to the application "]
        pub async fn open_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::PlatformInboundInstruction>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PlatformOutboundInstruction>>,
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
                "/io.axoniq.axonserver.grpc.control.PlatformService/OpenStream",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for PlatformServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PlatformServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PlatformServiceClient {{ ... }}")
        }
    }
}
