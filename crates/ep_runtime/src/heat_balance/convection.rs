//! Heat-balance convection source-order ownership notes.

/// Current inside convection routine family used by the compatibility lane.
pub const INSIDE_CONVECTION_SOURCE: &str =
    "src/EnergyPlus/HeatBalanceSurfaceManager.cc::CalcHeatBalanceInsideSurf";

/// Current outside convection routine family used by the compatibility lane.
pub const OUTSIDE_CONVECTION_SOURCE: &str =
    "src/EnergyPlus/HeatBalanceSurfaceManager.cc::CalcHeatBalanceOutsideSurf";
