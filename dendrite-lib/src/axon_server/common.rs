/// Describes a serialized object
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedObject {
    /// The type identifier of the serialized object.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// The revision of the serialized form of the given type.
    #[prost(string, tag = "2")]
    pub revision: ::prost::alloc::string::String,
    /// The actual data representing the object in serialized form.
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// The value of a MetaData entry.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaDataValue {
    /// The data of the MetaData entry, depending on the type of data it contains.
    #[prost(oneof = "meta_data_value::Data", tags = "1, 2, 3, 4, 5")]
    pub data: ::core::option::Option<meta_data_value::Data>,
}
/// Nested message and enum types in `MetaDataValue`.
pub mod meta_data_value {
    /// The data of the MetaData entry, depending on the type of data it contains.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// The text value of the Meta Data entry.
        #[prost(string, tag = "1")]
        TextValue(::prost::alloc::string::String),
        /// The numeric value of the Meta Data entry.
        #[prost(sint64, tag = "2")]
        NumberValue(i64),
        /// The boolean value of the Meta Data entry.
        #[prost(bool, tag = "3")]
        BooleanValue(bool),
        /// The floating point value of the Meta Data entry.
        #[prost(double, tag = "4")]
        DoubleValue(f64),
        /// The binary value of the Meta Data entry.
        #[prost(message, tag = "5")]
        BytesValue(super::SerializedObject),
    }
}
/// An instruction for routing components when routing or processing a message.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessingInstruction {
    /// The type of processing message.
    #[prost(enumeration = "ProcessingKey", tag = "1")]
    pub key: i32,
    /// The value associated with the processing key.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<MetaDataValue>,
}
/// Message containing details of an error
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorMessage {
    /// A human readable message explaining the error
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// A description of the location (client component, server) where the error occurred
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
    /// A collection of messages providing more details about root causes of the error
    #[prost(string, repeated, tag = "3")]
    pub details: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An Error Code identifying the type of error
    #[prost(string, tag = "4")]
    pub error_code: ::prost::alloc::string::String,
}
/// Message used for Flow Control instruction, providing the counterpart with additional permits for sending messages
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowControl {
    /// The ClientID of the component providing additional permits
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    /// The number of permits to provide
    #[prost(int64, tag = "3")]
    pub permits: i64,
}
/// Message describing instruction acknowledgement
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionAck {
    /// The identifier of the instruction
    #[prost(string, tag = "1")]
    pub instruction_id: ::prost::alloc::string::String,
    /// Indicator whether the instruction was acknowledged successfully
    #[prost(bool, tag = "2")]
    pub success: bool,
    /// Set if instruction acknowledgement failed.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<ErrorMessage>,
}
/// Message describing the result of the execution of an instruction
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionResult {
    /// The identifier of the instruction
    #[prost(string, tag = "1")]
    pub instruction_id: ::prost::alloc::string::String,
    /// Indicator whether the instruction was processed successfully
    #[prost(bool, tag = "2")]
    pub success: bool,
    /// Cause of instruction handling failure.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<ErrorMessage>,
}
/// An enumeration of possible keys for processing instructions.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum ProcessingKey {
    /// key indicating that the attached value should be used for consistent routing.
    RoutingKey = 0,
    /// key indicating that the attached value indicates relative priority of this message.
    Priority = 1,
    /// key indicating that the accompanied message has a finite validity. The attached value contains the number of milliseconds.
    Timeout = 2,
    /// key indicating that the requester expects at most the given number of results from this message. Use -1 for unlimited.
    NrOfResults = 3,
}
impl ProcessingKey {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProcessingKey::RoutingKey => "ROUTING_KEY",
            ProcessingKey::Priority => "PRIORITY",
            ProcessingKey::Timeout => "TIMEOUT",
            ProcessingKey::NrOfResults => "NR_OF_RESULTS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROUTING_KEY" => Some(Self::RoutingKey),
            "PRIORITY" => Some(Self::Priority),
            "TIMEOUT" => Some(Self::Timeout),
            "NR_OF_RESULTS" => Some(Self::NrOfResults),
            _ => None,
        }
    }
}
/// Defines status values for a scheduled task
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum TaskStatus {
    /// Task is scheduled for execution
    Scheduled = 0,
    /// Task execution completed successfully
    Completed = 1,
    /// Task execution failed with non transient exception
    Failed = 2,
    /// Task execution is in progress
    Running = 3,
    /// Task execution is in progress
    Cancelled = 4,
}
impl TaskStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaskStatus::Scheduled => "SCHEDULED",
            TaskStatus::Completed => "COMPLETED",
            TaskStatus::Failed => "FAILED",
            TaskStatus::Running => "RUNNING",
            TaskStatus::Cancelled => "CANCELLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCHEDULED" => Some(Self::Scheduled),
            "COMPLETED" => Some(Self::Completed),
            "FAILED" => Some(Self::Failed),
            "RUNNING" => Some(Self::Running),
            "CANCELLED" => Some(Self::Cancelled),
            _ => None,
        }
    }
}
