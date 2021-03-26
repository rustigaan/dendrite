/// Message containing Query related instructions for Axon Server
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProviderOutbound {
    /// Instruction identifier. If this identifier is set, this instruction will be acknowledged via inbound stream
    #[prost(string, tag = "8")]
    pub instruction_id: ::prost::alloc::string::String,
    /// The actual instruction to send
    #[prost(
        oneof = "query_provider_outbound::Request",
        tags = "1, 2, 3, 4, 5, 6, 7"
    )]
    pub request: ::core::option::Option<query_provider_outbound::Request>,
}
/// Nested message and enum types in `QueryProviderOutbound`.
pub mod query_provider_outbound {
    /// The actual instruction to send
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Registers a Query Handler with AxonServer
        #[prost(message, tag = "1")]
        Subscribe(super::QuerySubscription),
        /// Unregisters a Query Handler with AxonServer
        #[prost(message, tag = "2")]
        Unsubscribe(super::QuerySubscription),
        /// Grant permits to AxonServer to send a number of messages to the client
        #[prost(message, tag = "3")]
        FlowControl(super::super::common::FlowControl),
        /// Sends a Response to a Query received via the inbound stream
        #[prost(message, tag = "4")]
        QueryResponse(super::QueryResponse),
        /// Indicator that all responses for Query have been sent
        #[prost(message, tag = "5")]
        QueryComplete(super::QueryComplete),
        /// Sends a response for a Subscription Query that has been received via the inbound stream
        #[prost(message, tag = "6")]
        SubscriptionQueryResponse(super::SubscriptionQueryResponse),
        /// Acknowledgement of previously sent instruction via inbound stream
        #[prost(message, tag = "7")]
        Ack(super::super::common::InstructionAck),
    }
}
/// Queries or Query related instructions from AxonServer for the connected application
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProviderInbound {
    /// Instruction identifier. If this identifier is set, this instruction will be acknowledged via outbound stream
    #[prost(string, tag = "4")]
    pub instruction_id: ::prost::alloc::string::String,
    /// The actual query or instruction
    #[prost(oneof = "query_provider_inbound::Request", tags = "1, 2, 3")]
    pub request: ::core::option::Option<query_provider_inbound::Request>,
}
/// Nested message and enum types in `QueryProviderInbound`.
pub mod query_provider_inbound {
    /// The actual query or instruction
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Acknowledgement of previously sent instruction via outbound stream
        #[prost(message, tag = "1")]
        Ack(super::super::common::InstructionAck),
        /// Represents an incoming Query, for which this component is expected to provide a response
        #[prost(message, tag = "2")]
        Query(super::QueryRequest),
        /// Represents an incoming Subscription Query, for which this component is expected to provide a response and updates
        #[prost(message, tag = "3")]
        SubscriptionQueryRequest(super::SubscriptionQueryRequest),
    }
}
/// Message indicating that all available responses to an incoming Query have been provided.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryComplete {
    /// A unique identifier for this message
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    /// The identifier of the incoming query to complete
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message representing an incoming Query
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    /// The message ID of the incoming Query
    #[prost(string, tag = "1")]
    pub message_identifier: ::prost::alloc::string::String,
    /// The name of the Query to execute
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// The timestamp of the Query creation
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    /// A payload accompanying the Query
    #[prost(message, optional, tag = "4")]
    pub payload: ::core::option::Option<super::common::SerializedObject>,
    /// Meta Data providing contextual information of the Query
    #[prost(map = "string, message", tag = "5")]
    pub meta_data:
        ::std::collections::HashMap<::prost::alloc::string::String, super::common::MetaDataValue>,
    /// An object describing the expectations of the Response Type
    #[prost(message, optional, tag = "6")]
    pub response_type: ::core::option::Option<super::common::SerializedObject>,
    /// Any instructions for components Routing or Handling the Query
    #[prost(message, repeated, tag = "7")]
    pub processing_instructions: ::prost::alloc::vec::Vec<super::common::ProcessingInstruction>,
    /// The unique identifier of the client instance dispatching the query
    #[prost(string, tag = "8")]
    pub client_id: ::prost::alloc::string::String,
    /// The Name of the Component dispatching the query
    #[prost(string, tag = "9")]
    pub component_name: ::prost::alloc::string::String,
}
/// Message that represents the Response to a Query
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    /// The unique identifier of the Response Message
    #[prost(string, tag = "1")]
    pub message_identifier: ::prost::alloc::string::String,
    /// An Error Code identifying the type of error, if any
    #[prost(string, tag = "2")]
    pub error_code: ::prost::alloc::string::String,
    /// A detailed description of the error, if any
    #[prost(message, optional, tag = "3")]
    pub error_message: ::core::option::Option<super::common::ErrorMessage>,
    /// The Payload of the Response Message
    #[prost(message, optional, tag = "4")]
    pub payload: ::core::option::Option<super::common::SerializedObject>,
    /// Any Meta Data describing the context of the Response Message
    #[prost(map = "string, message", tag = "5")]
    pub meta_data:
        ::std::collections::HashMap<::prost::alloc::string::String, super::common::MetaDataValue>,
    /// Any instructions for components Routing or Handling the Response Message
    #[prost(message, repeated, tag = "6")]
    pub processing_instructions: ::prost::alloc::vec::Vec<super::common::ProcessingInstruction>,
    /// The unique identifier of the Query to which this is a response
    #[prost(string, tag = "7")]
    pub request_identifier: ::prost::alloc::string::String,
}
/// Message that represents a Subscription Query
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionQuery {
    /// A unique identifier for this subscription
    #[prost(string, tag = "1")]
    pub subscription_identifier: ::prost::alloc::string::String,
    /// The number of messages the Server may send before needing to await additional permits
    #[prost(int64, tag = "2")]
    pub number_of_permits: i64,
    /// The Query describing the desire for information
    #[prost(message, optional, tag = "3")]
    pub query_request: ::core::option::Option<QueryRequest>,
    /// A description of the type of Object expected as Update Responses
    #[prost(message, optional, tag = "4")]
    pub update_response_type: ::core::option::Option<super::common::SerializedObject>,
}
/// A message containing an Update of a Query Subscription Response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpdate {
    /// The unique identifier of this Update
    #[prost(string, tag = "2")]
    pub message_identifier: ::prost::alloc::string::String,
    /// The object representing the Update
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<super::common::SerializedObject>,
    /// Meta Data providing contextual information of the Update
    #[prost(map = "string, message", tag = "4")]
    pub meta_data:
        ::std::collections::HashMap<::prost::alloc::string::String, super::common::MetaDataValue>,
    /// The identifier of the Client instance providing the Update
    #[prost(string, tag = "5")]
    pub client_id: ::prost::alloc::string::String,
    /// The Component Name of the Client providing the Update
    #[prost(string, tag = "6")]
    pub component_name: ::prost::alloc::string::String,
    /// An Error Code identifying the type of error, if any
    #[prost(string, tag = "7")]
    pub error_code: ::prost::alloc::string::String,
    /// A detailed description of the error, if any
    #[prost(message, optional, tag = "8")]
    pub error_message: ::core::option::Option<super::common::ErrorMessage>,
}
/// Message indicating that all relevant Updates have been sent for a Subscription Query, and that no further Updates are available
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpdateComplete {
    /// The identifier of the Client instance providing the Update
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    /// The Component Name of the Client providing the Update
    #[prost(string, tag = "3")]
    pub component_name: ::prost::alloc::string::String,
}
/// Message indicating that an Error occurred and that no Updates will be sent for a Subscription Query
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpdateCompleteExceptionally {
    /// The identifier of the Client instance providing the Update
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    /// The Component Name of the Client providing the Update
    #[prost(string, tag = "3")]
    pub component_name: ::prost::alloc::string::String,
    /// The Code describing the type of Error that occurred
    #[prost(string, tag = "5")]
    pub error_code: ::prost::alloc::string::String,
    /// A detailed description of the error, if available
    #[prost(message, optional, tag = "6")]
    pub error_message: ::core::option::Option<super::common::ErrorMessage>,
}
/// Message describing possible interactions for a Subscription Query
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionQueryRequest {
    /// The actual request. The Subscription Query is opened using a `subscribe`, which opens the flow of updates. Once
    ///successful, the `get_initial_result` retrieves the initial result of the subscription. For the server to send
    ///more updates than the initial number of permits, use the `flow_control` request to send more permits.
    #[prost(oneof = "subscription_query_request::Request", tags = "1, 2, 3, 4")]
    pub request: ::core::option::Option<subscription_query_request::Request>,
}
/// Nested message and enum types in `SubscriptionQueryRequest`.
pub mod subscription_query_request {
    /// The actual request. The Subscription Query is opened using a `subscribe`, which opens the flow of updates. Once
    ///successful, the `get_initial_result` retrieves the initial result of the subscription. For the server to send
    ///more updates than the initial number of permits, use the `flow_control` request to send more permits.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Start a Subscription Query with the given details.
        #[prost(message, tag = "1")]
        Subscribe(super::SubscriptionQuery),
        /// Ends a previously started Subscription Query with the given details
        #[prost(message, tag = "2")]
        Unsubscribe(super::SubscriptionQuery),
        /// Requests the initial result of a subscription query to be sent. This should always be done after opening the
        ///subscription query itself, to remove concurrency conflicts with Update messages.
        #[prost(message, tag = "3")]
        GetInitialResult(super::SubscriptionQuery),
        /// Allows the Server to provide additional Updates to be sent. Only the `number_of_permits` field needs to be
        ///set on this message.
        #[prost(message, tag = "4")]
        FlowControl(super::SubscriptionQuery),
    }
}
/// Represents a Response Message for a Subscription Query
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionQueryResponse {
    /// The unique identifier for this message
    #[prost(string, tag = "1")]
    pub message_identifier: ::prost::alloc::string::String,
    /// The identifier of the subscription query this is a response for
    #[prost(string, tag = "2")]
    pub subscription_identifier: ::prost::alloc::string::String,
    /// The actual response. The `initial_result` message is sent as a response to `get_initial_result`. An `update`
    ///messages is sent for each update available for the query, even before the Initial Result is supplied. The
    ///`complete` or `complete_exceptionally` are sent when the publishing side completed the Subscription Query,
    ///either regularly (`complete`) or because an error occurred (`complete_exceptionally`).
    #[prost(oneof = "subscription_query_response::Response", tags = "3, 4, 5, 6")]
    pub response: ::core::option::Option<subscription_query_response::Response>,
}
/// Nested message and enum types in `SubscriptionQueryResponse`.
pub mod subscription_query_response {
    /// The actual response. The `initial_result` message is sent as a response to `get_initial_result`. An `update`
    ///messages is sent for each update available for the query, even before the Initial Result is supplied. The
    ///`complete` or `complete_exceptionally` are sent when the publishing side completed the Subscription Query,
    ///either regularly (`complete`) or because an error occurred (`complete_exceptionally`).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// Provides an Initial Response
        #[prost(message, tag = "3")]
        InitialResult(super::QueryResponse),
        /// Provides an Update Response
        #[prost(message, tag = "4")]
        Update(super::QueryUpdate),
        /// Indicates the Query is complete, and no more Updates will be sent
        #[prost(message, tag = "5")]
        Complete(super::QueryUpdateComplete),
        /// Indicates the Query failed exceptionally, and no more Updates will be sent
        #[prost(message, tag = "6")]
        CompleteExceptionally(super::QueryUpdateCompleteExceptionally),
    }
}
/// Message containing details of a Registration of a Query Handler in a component
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubscription {
    /// The unique identifier of this Message
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    /// The name of the Query the Handler is subscribed to
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// The type of Result this Handler produces
    #[prost(string, tag = "3")]
    pub result_name: ::prost::alloc::string::String,
    /// The name of the Component containing the Query Handler
    #[prost(string, tag = "4")]
    pub component_name: ::prost::alloc::string::String,
    /// The unique identifier of the Client Instance containing the Query Handler
    #[prost(string, tag = "5")]
    pub client_id: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod query_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service providing operations for the Query Messaging component of AxonServer "]
    pub struct QueryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryServiceClient<tonic::transport::Channel> {
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
    impl<T> QueryServiceClient<T>
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
        #[doc = " Opens a Query- and Instruction stream to AxonServer. "]
        pub async fn open_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::QueryProviderOutbound>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::QueryProviderInbound>>,
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
                "/io.axoniq.axonserver.grpc.query.QueryService/OpenStream",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Sends a point-to-point or scatter-gather Query "]
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::QueryResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.query.QueryService/Query",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Opens a Subscription Query "]
        pub async fn subscription(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SubscriptionQueryRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SubscriptionQueryResponse>>,
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
                "/io.axoniq.axonserver.grpc.query.QueryService/Subscription",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for QueryServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for QueryServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QueryServiceClient {{ ... }}")
        }
    }
}
