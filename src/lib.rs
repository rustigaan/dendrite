//! # Dendrite
//!
//! Crate `dendrite` is a [Rust](https://www.rust-lang.org) library to connect to [AxonServer](https://axoniq.io/product-overview/axon-server).
//!
//! See the GitHub project [dendrite2go/dendrite](https://github.com/dendrite2go/rustic-dendrite) for an example of how to use this code.
//!
//! The example modules will be split off into a separate project.

pub mod axon_utils;
pub mod axon_server;
pub mod grpc_example;
pub mod elastic_search_utils;
pub mod example_api;
pub mod example_command;
pub mod example_event;
pub mod example_query;
