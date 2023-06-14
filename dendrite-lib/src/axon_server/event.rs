/// Request message to schedule an event
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleEventRequest {
    /// timestamp when to publish the event
    #[prost(int64, tag = "1")]
    pub instant: i64,
    /// the event to publish
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
}
/// Request message to reschedule an event
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RescheduleEventRequest {
    /// optional token of scheduled event to cancel
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// timestamp when to publish the event
    #[prost(int64, tag = "2")]
    pub instant: i64,
    /// the event to publish
    #[prost(message, optional, tag = "3")]
    pub event: ::core::option::Option<Event>,
}
/// Request message to cancel an event
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelScheduledEventRequest {
    /// token of scheduled event to cancel
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
/// Token to manage a scheduled event
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleToken {
    /// Field defining the token identifier
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
/// Request message to receive the first Token (Tail Token) of the Event Stream
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFirstTokenRequest {}
/// Request message to receive the last Token (Head Token) of the Event Stream
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTokenRequest {}
/// Request message to receive the Token that starts streaming events from the given timestamp
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTokenAtRequest {
    /// Timestamp expressed as milliseconds since epoch
    #[prost(int64, tag = "1")]
    pub instant: i64,
}
/// Message containing the information necessary to track the position of events in the Event Stream
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingToken {
    /// The value of the Token
    #[prost(int64, tag = "1")]
    pub token: i64,
}
/// Message wrapping an Event and a Tracking Token
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventWithToken {
    /// The Token representing the position of this Event in the Stream
    #[prost(int64, tag = "1")]
    pub token: i64,
    /// The actual Event Message
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
}
/// Message providing the parameters for executing a Query against AxonServer.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEventsRequest {
    /// The query to execute against the Event Stream
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// The number of results AxonServer may send before new permits need to be provided
    #[prost(int64, tag = "2")]
    pub number_of_permits: i64,
    /// Whether to keep the query running against incoming events once the Head of the Stream is reached
    #[prost(bool, tag = "3")]
    pub live_events: bool,
    /// Indicates whether to force querying events from the leader node of an Axon Server. Forcing reads from leader
    /// reduces the staleness of the data read, but also puts extra burden on the leader, reducing overall scalability.
    /// <p>
    /// This property has no effect on connections to AxonServer SE.
    /// </p>
    #[prost(bool, tag = "4")]
    pub force_read_from_leader: bool,
}
/// A message describing a response to a Query request
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEventsResponse {
    /// The actual contents of this response
    #[prost(oneof = "query_events_response::Data", tags = "1, 2, 3")]
    pub data: ::core::option::Option<query_events_response::Data>,
}
/// Nested message and enum types in `QueryEventsResponse`.
pub mod query_events_response {
    /// The actual contents of this response
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Provided when the response contains the names of the columns the response contains. This message typically arrives first.
        #[prost(message, tag = "1")]
        Columns(super::ColumnsResponse),
        /// Provided when the response message contains results of the Query
        #[prost(message, tag = "2")]
        Row(super::RowResponse),
        /// Provided when all historic events have been included in the query results
        #[prost(message, tag = "3")]
        FilesCompleted(super::Confirmation),
    }
}
/// Message containing the names of the columns returned in a Query
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnsResponse {
    /// The names of the columns provided in the query
    #[prost(string, repeated, tag = "1")]
    pub column: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message providing Query Result data
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowResponse {
    /// The values which, when combined, uniquely update this row. Any previously received values with the same identifiers should be replaced with this value
    #[prost(message, repeated, tag = "1")]
    pub id_values: ::prost::alloc::vec::Vec<QueryValue>,
    /// The sorting values to use when sorting this response compared to the others.
    #[prost(message, repeated, tag = "2")]
    pub sort_values: ::prost::alloc::vec::Vec<QueryValue>,
    /// The actual data values for each of the columns, as a column name -> value mapping
    #[prost(map = "string, message", tag = "3")]
    pub values: ::std::collections::HashMap<::prost::alloc::string::String, QueryValue>,
}
/// Describes the combination of an Aggregate Identifier and first expected Sequence number when opening an Aggregate-specific Event Stream
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadHighestSequenceNrRequest {
    /// The Identifier of the Aggregate for which to load events
    #[prost(string, tag = "1")]
    pub aggregate_id: ::prost::alloc::string::String,
    /// The Sequence Number of the first event expected
    #[prost(int64, tag = "3")]
    pub from_sequence_nr: i64,
}
/// The highest Sequence Number found for the provided request
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadHighestSequenceNrResponse {
    /// The sequence number of the latest event
    #[prost(int64, tag = "1")]
    pub to_sequence_nr: i64,
}
/// A confirmation to a request from the client
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Confirmation {
    /// True when successful, otherwise false
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// Request describing the desire to read events for a specific Aggregate
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAggregateEventsRequest {
    /// The identifier of the aggregate to read events for
    #[prost(string, tag = "1")]
    pub aggregate_id: ::prost::alloc::string::String,
    /// The sequence number of the first event to receive
    #[prost(int64, tag = "2")]
    pub initial_sequence: i64,
    /// Whether a snapshot may be returned as first element in the stream
    #[prost(bool, tag = "3")]
    pub allow_snapshots: bool,
    /// The maximum sequence number (inclusive) of the events to retrieve, 0 means up to last event
    #[prost(int64, tag = "4")]
    pub max_sequence: i64,
    /// Hint for a minimum token to search events from
    #[prost(int64, tag = "5")]
    pub min_token: i64,
}
/// Request message to retrieve Snapshot Events for a specific Aggregate instance
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAggregateSnapshotsRequest {
    /// The identifier to fetch the snapshots for
    #[prost(string, tag = "1")]
    pub aggregate_id: ::prost::alloc::string::String,
    /// The minimal sequence number of the snapshots to retrieve
    #[prost(int64, tag = "2")]
    pub initial_sequence: i64,
    /// The maximum sequence number of the snapshots to retrieve
    #[prost(int64, tag = "3")]
    pub max_sequence: i64,
    /// The maximum number of results to stream
    #[prost(int32, tag = "4")]
    pub max_results: i32,
}
/// Request message to open an Event Stream from the Event Store.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventsRequest {
    /// The token to start streaming from
    #[prost(int64, tag = "1")]
    pub tracking_token: i64,
    /// The number of messages the server may send before it needs to wait for more permits
    #[prost(int64, tag = "2")]
    pub number_of_permits: i64,
    /// The unique identifier of this client instance. Used for monitoring.
    #[prost(string, tag = "3")]
    pub client_id: ::prost::alloc::string::String,
    /// The component name of this client instance. Used for monitoring.
    #[prost(string, tag = "4")]
    pub component_name: ::prost::alloc::string::String,
    /// The name of the processor requesting this stream. Used for monitoring.
    #[prost(string, tag = "5")]
    pub processor: ::prost::alloc::string::String,
    /// An enumeration of payload types that need to be blacklisted. The Server will stop sending messages of these
    /// types in order to reduce I/O. Note that the Server may occasionally send a blacklisted message to prevent
    /// time-outs and stale tokens on clients.
    #[prost(message, repeated, tag = "6")]
    pub blacklist: ::prost::alloc::vec::Vec<PayloadDescription>,
    /// Indicates whether to force reading events from the leader node of an Axon Server. Forcing reads from leader
    /// reduces the staleness of the data read, but also puts extra burden on the leader, reducing overall scalability.
    /// <p>
    /// This property has no effect on connections to AxonServer SE.
    /// </p>
    #[prost(bool, tag = "7")]
    pub force_read_from_leader: bool,
}
/// Message containing the information of an Event
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// The unique identifier of this event
    #[prost(string, tag = "1")]
    pub message_identifier: ::prost::alloc::string::String,
    /// The identifier of the Aggregate instance that published this event, if any
    #[prost(string, tag = "2")]
    pub aggregate_identifier: ::prost::alloc::string::String,
    /// The sequence number of the Event in the Aggregate instance that published it, if any
    #[prost(int64, tag = "3")]
    pub aggregate_sequence_number: i64,
    /// The Type of the Aggregate instance that published this Event, if any
    #[prost(string, tag = "4")]
    pub aggregate_type: ::prost::alloc::string::String,
    /// The timestamp of the Event
    #[prost(int64, tag = "5")]
    pub timestamp: i64,
    /// The Payload of the Event
    #[prost(message, optional, tag = "6")]
    pub payload: ::core::option::Option<super::common::SerializedObject>,
    /// The Meta Data of the Event
    #[prost(map = "string, message", tag = "7")]
    pub meta_data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::common::MetaDataValue,
    >,
    /// Flag indicating whether the Event is a snapshot Event
    #[prost(bool, tag = "8")]
    pub snapshot: bool,
}
/// Value used in Query Responses to represent a value in its original type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValue {
    /// The actual value, which can be one of string, 64 bit signed integer, boolean or 64 bits floating point
    #[prost(oneof = "query_value::Data", tags = "1, 2, 3, 4")]
    pub data: ::core::option::Option<query_value::Data>,
}
/// Nested message and enum types in `QueryValue`.
pub mod query_value {
    /// The actual value, which can be one of string, 64 bit signed integer, boolean or 64 bits floating point
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// The text value
        #[prost(string, tag = "1")]
        TextValue(::prost::alloc::string::String),
        /// The (64 bits) integer value
        #[prost(sint64, tag = "2")]
        NumberValue(i64),
        /// The boolean value
        #[prost(bool, tag = "3")]
        BooleanValue(bool),
        /// The (64 bits) floating point value
        #[prost(double, tag = "4")]
        DoubleValue(f64),
    }
}
/// Description of a Payload Type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadDescription {
    /// The type identifier of the Payload
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// The revision of the Payload Type
    #[prost(string, tag = "2")]
    pub revision: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod event_store_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service providing operations against the EventStore functionality of Axon Server
    #[derive(Debug, Clone)]
    pub struct EventStoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EventStoreClient<tonic::transport::Channel> {
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
    impl<T> EventStoreClient<T>
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
        ) -> EventStoreClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            EventStoreClient::new(InterceptedService::new(inner, interceptor))
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
        /// Accepts a stream of Events returning a Confirmation when completed.
        pub async fn append_event(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::Event>,
        ) -> Result<tonic::Response<super::Confirmation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/AppendEvent",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// Accepts a Snapshot event returning a Confirmation when completed.
        pub async fn append_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::Event>,
        ) -> Result<tonic::Response<super::Confirmation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/AppendSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the Events for a given aggregate. Results are streamed rather than returned at once.
        pub async fn list_aggregate_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAggregateEventsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::Event>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/ListAggregateEvents",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Retrieves the Snapshots for a given aggregate. Results are streamed rather than returned at once.
        pub async fn list_aggregate_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAggregateSnapshotsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::Event>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/ListAggregateSnapshots",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Retrieves the Events from a given tracking token. However, if several GetEventsRequests are sent in the stream
        ///only first one will create the tracker, others are used for increasing number of permits or blacklisting. Results
        ///are streamed rather than returned at once.
        pub async fn list_events(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::GetEventsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::EventWithToken>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/ListEvents",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Gets the highest sequence number for a specific aggregate.
        pub async fn read_highest_sequence_nr(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadHighestSequenceNrRequest>,
        ) -> Result<
            tonic::Response<super::ReadHighestSequenceNrResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/ReadHighestSequenceNr",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Performs a query on the event store, returns a stream of results. Input is a stream to allow flow control from the
        /// client
        pub async fn query_events(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::QueryEventsRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::QueryEventsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/QueryEvents",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Retrieves the first token available in event store (typically 0). Returns 0 when no events in store.
        pub async fn get_first_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFirstTokenRequest>,
        ) -> Result<tonic::Response<super::TrackingToken>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/GetFirstToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the last committed token in event store. Returns -1 when no events in store.
        pub async fn get_last_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastTokenRequest>,
        ) -> Result<tonic::Response<super::TrackingToken>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/GetLastToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the token of the first token of an event from specified time in event store. Returns -1 when no events in store.
        pub async fn get_token_at(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTokenAtRequest>,
        ) -> Result<tonic::Response<super::TrackingToken>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventStore/GetTokenAt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod event_scheduler_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to use AxonServer as a provider of an EventScheduler
    #[derive(Debug, Clone)]
    pub struct EventSchedulerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EventSchedulerClient<tonic::transport::Channel> {
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
    impl<T> EventSchedulerClient<T>
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
        ) -> EventSchedulerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            EventSchedulerClient::new(InterceptedService::new(inner, interceptor))
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
        ///  Schedule the given event for publication at the given time}. The returned ScheduleToken can be used to cancel the planned publication.
        pub async fn schedule_event(
            &mut self,
            request: impl tonic::IntoRequest<super::ScheduleEventRequest>,
        ) -> Result<tonic::Response<super::ScheduleToken>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventScheduler/ScheduleEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///  Cancel a scheduled event and schedule another in its place.
        pub async fn reschedule_event(
            &mut self,
            request: impl tonic::IntoRequest<super::RescheduleEventRequest>,
        ) -> Result<tonic::Response<super::ScheduleToken>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventScheduler/RescheduleEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///  Cancel the publication of a scheduled event. If the events has already been published, this method does nothing.
        pub async fn cancel_scheduled_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelScheduledEventRequest>,
        ) -> Result<
            tonic::Response<super::super::common::InstructionAck>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.axoniq.axonserver.grpc.event.EventScheduler/CancelScheduledEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod event_store_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with EventStoreServer.
    #[async_trait]
    pub trait EventStore: Send + Sync + 'static {
        /// Accepts a stream of Events returning a Confirmation when completed.
        async fn append_event(
            &self,
            request: tonic::Request<tonic::Streaming<super::Event>>,
        ) -> Result<tonic::Response<super::Confirmation>, tonic::Status>;
        /// Accepts a Snapshot event returning a Confirmation when completed.
        async fn append_snapshot(
            &self,
            request: tonic::Request<super::Event>,
        ) -> Result<tonic::Response<super::Confirmation>, tonic::Status>;
        ///Server streaming response type for the ListAggregateEvents method.
        type ListAggregateEventsStream: futures_core::Stream<
                Item = Result<super::Event, tonic::Status>,
            >
            + Send
            + 'static;
        /// Retrieves the Events for a given aggregate. Results are streamed rather than returned at once.
        async fn list_aggregate_events(
            &self,
            request: tonic::Request<super::GetAggregateEventsRequest>,
        ) -> Result<tonic::Response<Self::ListAggregateEventsStream>, tonic::Status>;
        ///Server streaming response type for the ListAggregateSnapshots method.
        type ListAggregateSnapshotsStream: futures_core::Stream<
                Item = Result<super::Event, tonic::Status>,
            >
            + Send
            + 'static;
        /// Retrieves the Snapshots for a given aggregate. Results are streamed rather than returned at once.
        async fn list_aggregate_snapshots(
            &self,
            request: tonic::Request<super::GetAggregateSnapshotsRequest>,
        ) -> Result<tonic::Response<Self::ListAggregateSnapshotsStream>, tonic::Status>;
        ///Server streaming response type for the ListEvents method.
        type ListEventsStream: futures_core::Stream<
                Item = Result<super::EventWithToken, tonic::Status>,
            >
            + Send
            + 'static;
        /// Retrieves the Events from a given tracking token. However, if several GetEventsRequests are sent in the stream
        ///only first one will create the tracker, others are used for increasing number of permits or blacklisting. Results
        ///are streamed rather than returned at once.
        async fn list_events(
            &self,
            request: tonic::Request<tonic::Streaming<super::GetEventsRequest>>,
        ) -> Result<tonic::Response<Self::ListEventsStream>, tonic::Status>;
        /// Gets the highest sequence number for a specific aggregate.
        async fn read_highest_sequence_nr(
            &self,
            request: tonic::Request<super::ReadHighestSequenceNrRequest>,
        ) -> Result<
            tonic::Response<super::ReadHighestSequenceNrResponse>,
            tonic::Status,
        >;
        ///Server streaming response type for the QueryEvents method.
        type QueryEventsStream: futures_core::Stream<
                Item = Result<super::QueryEventsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Performs a query on the event store, returns a stream of results. Input is a stream to allow flow control from the
        /// client
        async fn query_events(
            &self,
            request: tonic::Request<tonic::Streaming<super::QueryEventsRequest>>,
        ) -> Result<tonic::Response<Self::QueryEventsStream>, tonic::Status>;
        /// Retrieves the first token available in event store (typically 0). Returns 0 when no events in store.
        async fn get_first_token(
            &self,
            request: tonic::Request<super::GetFirstTokenRequest>,
        ) -> Result<tonic::Response<super::TrackingToken>, tonic::Status>;
        /// Retrieves the last committed token in event store. Returns -1 when no events in store.
        async fn get_last_token(
            &self,
            request: tonic::Request<super::GetLastTokenRequest>,
        ) -> Result<tonic::Response<super::TrackingToken>, tonic::Status>;
        /// Retrieves the token of the first token of an event from specified time in event store. Returns -1 when no events in store.
        async fn get_token_at(
            &self,
            request: tonic::Request<super::GetTokenAtRequest>,
        ) -> Result<tonic::Response<super::TrackingToken>, tonic::Status>;
    }
    /// Service providing operations against the EventStore functionality of Axon Server
    #[derive(Debug)]
    pub struct EventStoreServer<T: EventStore> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EventStore> EventStoreServer<T> {
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EventStoreServer<T>
    where
        T: EventStore,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/io.axoniq.axonserver.grpc.event.EventStore/AppendEvent" => {
                    #[allow(non_camel_case_types)]
                    struct AppendEventSvc<T: EventStore>(pub Arc<T>);
                    impl<
                        T: EventStore,
                    > tonic::server::ClientStreamingService<super::Event>
                    for AppendEventSvc<T> {
                        type Response = super::Confirmation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::Event>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).append_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AppendEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventStore/AppendSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct AppendSnapshotSvc<T: EventStore>(pub Arc<T>);
                    impl<T: EventStore> tonic::server::UnaryService<super::Event>
                    for AppendSnapshotSvc<T> {
                        type Response = super::Confirmation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Event>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).append_snapshot(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AppendSnapshotSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventStore/ListAggregateEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ListAggregateEventsSvc<T: EventStore>(pub Arc<T>);
                    impl<
                        T: EventStore,
                    > tonic::server::ServerStreamingService<
                        super::GetAggregateEventsRequest,
                    > for ListAggregateEventsSvc<T> {
                        type Response = super::Event;
                        type ResponseStream = T::ListAggregateEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAggregateEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_aggregate_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListAggregateEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventStore/ListAggregateSnapshots" => {
                    #[allow(non_camel_case_types)]
                    struct ListAggregateSnapshotsSvc<T: EventStore>(pub Arc<T>);
                    impl<
                        T: EventStore,
                    > tonic::server::ServerStreamingService<
                        super::GetAggregateSnapshotsRequest,
                    > for ListAggregateSnapshotsSvc<T> {
                        type Response = super::Event;
                        type ResponseStream = T::ListAggregateSnapshotsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAggregateSnapshotsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_aggregate_snapshots(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListAggregateSnapshotsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventStore/ListEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ListEventsSvc<T: EventStore>(pub Arc<T>);
                    impl<
                        T: EventStore,
                    > tonic::server::StreamingService<super::GetEventsRequest>
                    for ListEventsSvc<T> {
                        type Response = super::EventWithToken;
                        type ResponseStream = T::ListEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::GetEventsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventStore/ReadHighestSequenceNr" => {
                    #[allow(non_camel_case_types)]
                    struct ReadHighestSequenceNrSvc<T: EventStore>(pub Arc<T>);
                    impl<
                        T: EventStore,
                    > tonic::server::UnaryService<super::ReadHighestSequenceNrRequest>
                    for ReadHighestSequenceNrSvc<T> {
                        type Response = super::ReadHighestSequenceNrResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadHighestSequenceNrRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).read_highest_sequence_nr(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadHighestSequenceNrSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventStore/QueryEvents" => {
                    #[allow(non_camel_case_types)]
                    struct QueryEventsSvc<T: EventStore>(pub Arc<T>);
                    impl<
                        T: EventStore,
                    > tonic::server::StreamingService<super::QueryEventsRequest>
                    for QueryEventsSvc<T> {
                        type Response = super::QueryEventsResponse;
                        type ResponseStream = T::QueryEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::QueryEventsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventStore/GetFirstToken" => {
                    #[allow(non_camel_case_types)]
                    struct GetFirstTokenSvc<T: EventStore>(pub Arc<T>);
                    impl<
                        T: EventStore,
                    > tonic::server::UnaryService<super::GetFirstTokenRequest>
                    for GetFirstTokenSvc<T> {
                        type Response = super::TrackingToken;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFirstTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_first_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFirstTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventStore/GetLastToken" => {
                    #[allow(non_camel_case_types)]
                    struct GetLastTokenSvc<T: EventStore>(pub Arc<T>);
                    impl<
                        T: EventStore,
                    > tonic::server::UnaryService<super::GetLastTokenRequest>
                    for GetLastTokenSvc<T> {
                        type Response = super::TrackingToken;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLastTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_last_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLastTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventStore/GetTokenAt" => {
                    #[allow(non_camel_case_types)]
                    struct GetTokenAtSvc<T: EventStore>(pub Arc<T>);
                    impl<
                        T: EventStore,
                    > tonic::server::UnaryService<super::GetTokenAtRequest>
                    for GetTokenAtSvc<T> {
                        type Response = super::TrackingToken;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTokenAtRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_token_at(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTokenAtSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: EventStore> Clone for EventStoreServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: EventStore> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EventStore> tonic::server::NamedService for EventStoreServer<T> {
        const NAME: &'static str = "io.axoniq.axonserver.grpc.event.EventStore";
    }
}
/// Generated server implementations.
pub mod event_scheduler_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with EventSchedulerServer.
    #[async_trait]
    pub trait EventScheduler: Send + Sync + 'static {
        ///  Schedule the given event for publication at the given time}. The returned ScheduleToken can be used to cancel the planned publication.
        async fn schedule_event(
            &self,
            request: tonic::Request<super::ScheduleEventRequest>,
        ) -> Result<tonic::Response<super::ScheduleToken>, tonic::Status>;
        ///  Cancel a scheduled event and schedule another in its place.
        async fn reschedule_event(
            &self,
            request: tonic::Request<super::RescheduleEventRequest>,
        ) -> Result<tonic::Response<super::ScheduleToken>, tonic::Status>;
        ///  Cancel the publication of a scheduled event. If the events has already been published, this method does nothing.
        async fn cancel_scheduled_event(
            &self,
            request: tonic::Request<super::CancelScheduledEventRequest>,
        ) -> Result<
            tonic::Response<super::super::common::InstructionAck>,
            tonic::Status,
        >;
    }
    /// Service to use AxonServer as a provider of an EventScheduler
    #[derive(Debug)]
    pub struct EventSchedulerServer<T: EventScheduler> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EventScheduler> EventSchedulerServer<T> {
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EventSchedulerServer<T>
    where
        T: EventScheduler,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/io.axoniq.axonserver.grpc.event.EventScheduler/ScheduleEvent" => {
                    #[allow(non_camel_case_types)]
                    struct ScheduleEventSvc<T: EventScheduler>(pub Arc<T>);
                    impl<
                        T: EventScheduler,
                    > tonic::server::UnaryService<super::ScheduleEventRequest>
                    for ScheduleEventSvc<T> {
                        type Response = super::ScheduleToken;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScheduleEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).schedule_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScheduleEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventScheduler/RescheduleEvent" => {
                    #[allow(non_camel_case_types)]
                    struct RescheduleEventSvc<T: EventScheduler>(pub Arc<T>);
                    impl<
                        T: EventScheduler,
                    > tonic::server::UnaryService<super::RescheduleEventRequest>
                    for RescheduleEventSvc<T> {
                        type Response = super::ScheduleToken;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RescheduleEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).reschedule_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RescheduleEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.axoniq.axonserver.grpc.event.EventScheduler/CancelScheduledEvent" => {
                    #[allow(non_camel_case_types)]
                    struct CancelScheduledEventSvc<T: EventScheduler>(pub Arc<T>);
                    impl<
                        T: EventScheduler,
                    > tonic::server::UnaryService<super::CancelScheduledEventRequest>
                    for CancelScheduledEventSvc<T> {
                        type Response = super::super::common::InstructionAck;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelScheduledEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).cancel_scheduled_event(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelScheduledEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: EventScheduler> Clone for EventSchedulerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: EventScheduler> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EventScheduler> tonic::server::NamedService for EventSchedulerServer<T> {
        const NAME: &'static str = "io.axoniq.axonserver.grpc.event.EventScheduler";
    }
}
