//! Arbitrary IDF/epJSON run orchestration.
//!
//! This crate owns the user-facing `eplus-rs run <input>` pipeline boundary:
//! input staging, IDF-to-epJSON conversion, typed compile, support assessment,
//! Rust runtime dispatch, optional EnergyPlus oracle baseline, comparison, and
//! run-summary/report artifact generation.

#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]
#![recursion_limit = "1024"]

mod clock_trace;
mod config;
mod diagnostics;
mod geo02_trace;
mod geo03_trace;
mod geometry_trace;
mod oracle;
mod outputs;
mod pipeline;
mod porting_scope;
mod psy02_state_json;
mod psy02_trace;
mod psychrometrics_trace;
mod support;
mod support_registry;
mod zon01_trace;

pub use config::*;
pub use diagnostics::*;
pub use oracle::*;
pub use pipeline::*;
pub use porting_scope::{PortingScope, PortingScopeTrace, inspect_porting_scope};
pub use support::*;
