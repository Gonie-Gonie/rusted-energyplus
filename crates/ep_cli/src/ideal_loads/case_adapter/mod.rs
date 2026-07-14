//! IdealLoads conformance-case input adaptation.

mod time_axis;

pub(super) use time_axis::{
    IdealLoadsTimestepContext, ideal_loads_sample_timestep_hours,
    ideal_loads_sample_timestep_seconds, ideal_loads_timestep_context,
};
