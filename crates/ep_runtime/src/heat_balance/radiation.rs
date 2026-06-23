//! Heat-balance radiation source-order ownership notes.

/// Current longwave/solar source-order owner for outside-face inputs.
pub const EXTERIOR_RADIATION_OWNER_STAGE: &str = "CalcHeatBalanceOutsideSurf";

/// Current longwave source-order owner for inside-face inputs.
pub const INTERIOR_RADIATION_OWNER_STAGE: &str = "CalcHeatBalanceInsideSurf";
