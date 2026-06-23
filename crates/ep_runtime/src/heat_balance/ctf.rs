//! CTF conduction source-order ownership notes.

/// EnergyPlus source file for CTF conduction state used by heat balance.
pub const CTF_SOURCE_FILE: &str = "src/EnergyPlus/HeatBalanceSurfaceManager.cc";

/// Current Rust owner for inside/outside CTF history advancement.
pub const CTF_HISTORY_OWNER_STAGE: &str = "UpdateThermalHistories";

/// Current Rust owner for inside/outside conduction report timing.
pub const CTF_REPORT_OWNER_STAGE: &str = "ReportSurfaceHeatBalance";
