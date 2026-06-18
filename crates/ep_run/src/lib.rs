//! Arbitrary IDF/epJSON run orchestration.
//!
//! This crate owns the user-facing `eplus-rs run <input>` pipeline boundary:
//! input staging, IDF-to-epJSON conversion, typed compile, support assessment,
//! Rust runtime dispatch, optional EnergyPlus oracle baseline, comparison, and
//! run-summary/report artifact generation.

mod config;
mod diagnostics;
mod oracle;
mod outputs;
mod pipeline;
mod support;

pub use config::*;
pub use diagnostics::*;
pub use oracle::*;
pub use pipeline::*;
pub use support::*;
