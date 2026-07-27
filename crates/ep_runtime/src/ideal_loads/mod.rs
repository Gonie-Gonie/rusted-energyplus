//! IdealLoads/PurchasedAir compatibility path.

#![allow(clippy::if_same_then_else, clippy::too_many_arguments)]

mod binding;
mod calc;
mod coupled_output;
mod coupled_runtime;
mod coupling;
mod dispatch;
mod humidistat;
mod init;
mod input;
mod meters;
mod outdoor_air;
mod report;
mod runtime;
mod sizing;
mod update;

pub use binding::*;
pub use calc::*;
pub(crate) use coupled_output::append_direct_zone_purchased_air_hourly_output_series;
pub use coupled_output::{
    DirectZonePurchasedAirHourlyOutputError,
    ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_COOLING_SETPOINT_RATE,
    ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_HEATING_SETPOINT_RATE,
};
pub use coupled_runtime::*;
pub use coupling::*;
pub use dispatch::*;
pub use humidistat::*;
pub use init::*;
pub use input::*;
pub use meters::*;
pub use outdoor_air::*;
pub use report::*;
pub use runtime::*;
pub use sizing::*;
pub use update::*;

#[cfg(test)]
mod sizing_tests;
