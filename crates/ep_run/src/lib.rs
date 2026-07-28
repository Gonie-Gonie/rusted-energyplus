//! Arbitrary IDF/epJSON run orchestration.
//!
//! This crate owns the user-facing `eplus-rs run <input>` pipeline boundary:
//! input staging, IDF-to-epJSON conversion, typed compile, support assessment,
//! Rust runtime dispatch, optional EnergyPlus oracle baseline, comparison, and
//! run-summary/report artifact generation.

#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]
#![recursion_limit = "256"]

mod config;
mod diagnostics;
mod oracle;
mod outputs;
mod pipeline;
mod support;
mod support_registry;

pub use config::*;
pub use diagnostics::*;
pub use oracle::*;
pub use pipeline::*;
pub use support::*;
