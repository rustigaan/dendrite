//! Generated from `proto/axon_server/*.proto` using `prost`.
//!
//! This module contains a Rust implementation of the AxonSever.

tonic::include_proto!("io.axoniq.axonserver.grpc");
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
