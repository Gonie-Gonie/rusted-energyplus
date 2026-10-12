//! Arbitrary IDF/epJSON run orchestration.
//!
//! This crate owns the user-facing `eplus-rs run <input>` pipeline boundary:
//! input staging, IDF-to-epJSON conversion, typed compile, support assessment,
//! Rust runtime dispatch, optional EnergyPlus oracle baseline, comparison, and
//! run-summary/report artifact generation.

#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]
#![recursion_limit = "1024"]

mod clk02_trace;
mod clk03_trace;
mod clk06_sky_trace;
mod clock_trace;
mod config;
mod ctf_initialization_outputs;
mod diagnostics;
mod geo02_trace;
mod geo03_trace;
mod geometry_trace;
mod oracle;
mod ordinary_design_day_input;
mod ordinary_equipment_input;
mod ordinary_input_processor;
mod ordinary_project_data;
mod ordinary_schedule_input;
mod ordinary_sizing_input;
mod outputs;
mod pipeline;
mod porting_scope;
mod psy02_state_json;
mod psy02_trace;
mod psychrometrics_trace;
mod sch01_trace;
mod support;
mod support_registry;
mod warmup_report_controls;
mod weather_solar_controls;
mod zon01_trace;

pub use config::*;
pub use diagnostics::*;
pub use oracle::*;
pub use pipeline::*;
pub use porting_scope::{PortingScope, PortingScopeTrace, inspect_porting_scope};
pub use support::*;
