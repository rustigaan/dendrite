//! # Dendrite
//!
//! Crate `dendrite` is a [Rust](https://www.rust-lang.org) library to connect to [AxonServer](https://axoniq.io/product-overview/axon-server).
//!
//! See the GitHub project [dendrite2go/archetype-rust-axon](https://github.com/dendrite2go/archetype-rust-axon) for an example of how to use this code.

pub mod axon_utils;
pub mod axon_server;

#[macro_export]
macro_rules! register {
    ($registry:ident, $handler:ident) => {
        $registry.register(&$handler)
    };
}
