//! Generated from `proto/axon_server/*.proto` using `prost`.
//!
//! This module contains a Rust implementation of the AxonSever.

pub use common::{ErrorMessage, FlowControl, SerializedObject};

pub mod common {
    tonic::include_proto!("io.axoniq.axonserver.grpc.common");
}

pub mod command {
    tonic::include_proto!("io.axoniq.axonserver.grpc.command");
}

pub mod control {
    tonic::include_proto!("io.axoniq.axonserver.grpc.control");
}

pub mod event {
    tonic::include_proto!("io.axoniq.axonserver.grpc.event");
}

pub mod query {
    tonic::include_proto!("io.axoniq.axonserver.grpc.query");
}
