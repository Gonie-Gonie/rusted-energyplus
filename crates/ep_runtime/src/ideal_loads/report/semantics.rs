//! ReportPurchasedAir output semantics.

/// Source-order boundary for IdealLoads rate outputs.
pub const IDEAL_LOADS_RATE_OUTPUT_SOURCE: &str = "ReportPurchasedAir after UpdatePurchasedAir";
/// Timestep source for IdealLoads rate output rows.
pub const IDEAL_LOADS_RATE_OUTPUT_TIMESTEP_SOURCE: &str = "Detailed system timestep values";
/// Timestep source for IdealLoads report-energy output rows.
pub const IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE: &str =
    "ReportPurchasedAir rate * TimeStepSysSec";
/// Claim policy for IdealLoads report-energy output rows.
pub const IDEAL_LOADS_ENERGY_OUTPUT_LEVEL_POLICY: &str =
    "diagnostic-only until rate-to-energy parity is separately proven";
/// Claim policy for IdealLoads fuel-energy output rows.
pub const IDEAL_LOADS_FUEL_ENERGY_OUTPUT_LEVEL_POLICY: &str =
    "diagnostic-only until fuel-efficiency path is separately proven";
