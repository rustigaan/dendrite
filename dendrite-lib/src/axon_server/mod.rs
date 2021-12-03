//! Generated from `proto/axon_server/*.proto` using `prost`.
//!
//! This module contains a Rust implementation of the AxonSever.

pub mod command;
pub mod common;
pub mod control;
pub mod event;
pub mod query;

pub use common::{ErrorMessage, FlowControl, SerializedObject};
