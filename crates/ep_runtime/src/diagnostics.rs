//! Runtime diagnostic output variable names.

/// Diagnostic-only CTF component rate written for heat-balance source isolation.
pub const SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE: &str =
    "Surface CTF Inside Face Current Outside Temperature Term Rate";
/// Diagnostic-only CTF component rate written for heat-balance source isolation.
pub const SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE: &str =
    "Surface CTF Inside Face Current Inside Temperature Term Rate";
/// Diagnostic-only CTF component rate written for heat-balance source isolation.
pub const SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE: &str =
    "Surface CTF Inside Face History Term Rate";
/// Diagnostic-only CTF component rate written for heat-balance source isolation.
pub const SURFACE_CTF_INSIDE_HISTORY_TEMPERATURE_TERM_RATE_VARIABLE: &str =
    "Surface CTF Inside Face History Temperature Term Rate";
/// Diagnostic-only CTF component rate written for heat-balance source isolation.
pub const SURFACE_CTF_INSIDE_HISTORY_FLUX_TERM_RATE_VARIABLE: &str =
    "Surface CTF Inside Face History Flux Term Rate";
/// Diagnostic-only CTF component rate written for heat-balance source isolation.
pub const SURFACE_CTF_OUTSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE: &str =
    "Surface CTF Outside Face Current Outside Temperature Term Rate";
/// Diagnostic-only CTF component rate written for heat-balance source isolation.
pub const SURFACE_CTF_OUTSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE: &str =
    "Surface CTF Outside Face Current Inside Temperature Term Rate";
/// Diagnostic-only CTF component rate written for heat-balance source isolation.
pub const SURFACE_CTF_OUTSIDE_HISTORY_TERM_RATE_VARIABLE: &str =
    "Surface CTF Outside Face History Term Rate";
/// Diagnostic-only outside balance temperature used to build exterior report split terms.
pub const SURFACE_OUTSIDE_BALANCE_REPORT_TEMPERATURE_VARIABLE: &str =
    "Surface Outside Face Balance Report Temperature";
/// Diagnostic-only outside balance surface temperature used for exterior coefficient lookup.
pub const SURFACE_OUTSIDE_BALANCE_COEFFICIENT_TEMPERATURE_VARIABLE: &str =
    "Surface Outside Face Balance Coefficient Temperature";
/// Diagnostic-only outside convection reference temperature used by the balance.
pub const SURFACE_OUTSIDE_BALANCE_CONVECTION_REFERENCE_TEMPERATURE_VARIABLE: &str =
    "Surface Outside Face Balance Convection Reference Temperature";
/// Diagnostic-only equivalent radiant temperature used by the outside balance.
pub const SURFACE_OUTSIDE_BALANCE_EQUIVALENT_RADIANT_TEMPERATURE_VARIABLE: &str =
    "Surface Outside Face Balance Equivalent Radiant Temperature";
/// Diagnostic-only equivalent outside radiation coefficient used by the balance.
pub const SURFACE_OUTSIDE_BALANCE_RADIATION_COEFFICIENT_VARIABLE: &str =
    "Surface Outside Face Balance Radiation Heat Transfer Coefficient";
/// Diagnostic-only inside source term used by the quick outside balance.
pub const SURFACE_OUTSIDE_QUICK_BALANCE_INSIDE_SOURCE_TERM_VARIABLE: &str =
    "Surface Outside Face Quick Balance Inside Source Term";
/// Diagnostic-only inside balance term used by the quick outside balance.
pub const SURFACE_OUTSIDE_QUICK_BALANCE_INSIDE_BALANCE_TERM_VARIABLE: &str =
    "Surface Outside Face Quick Balance Inside Balance Term";
/// Diagnostic-only numerator used by the quick outside balance.
pub const SURFACE_OUTSIDE_QUICK_BALANCE_NUMERATOR_VARIABLE: &str =
    "Surface Outside Face Quick Balance Numerator";
/// Diagnostic-only denominator used by the quick outside balance.
pub const SURFACE_OUTSIDE_QUICK_BALANCE_DENOMINATOR_VARIABLE: &str =
    "Surface Outside Face Quick Balance Denominator";
/// Diagnostic-only inside/outside CTF coupling factor used by the quick outside balance.
pub const SURFACE_OUTSIDE_QUICK_BALANCE_COUPLING_FACTOR_VARIABLE: &str =
    "Surface Outside Face Quick Balance Coupling Factor";
/// Diagnostic-only Rust zone-air current MAT at the reported timestep.
pub const RUST_ZONE_AIR_CURRENT_TEMPERATURE_VARIABLE: &str = "Rust Zone Air Current Temperature";
/// Diagnostic-only Rust zone-timestep averaged MAT.
pub const RUST_ZONE_AIR_ZONE_TIMESTEP_AVERAGE_TEMPERATURE_VARIABLE: &str =
    "Rust Zone Air Zone Timestep Average Temperature";
/// Diagnostic-only Rust previous zone-timestep MAT history slot 1.
pub const RUST_ZONE_AIR_PREVIOUS_TEMPERATURE_1_VARIABLE: &str =
    "Rust Zone Air Previous Temperature 1";
/// Diagnostic-only Rust previous zone-timestep MAT history slot 2.
pub const RUST_ZONE_AIR_PREVIOUS_TEMPERATURE_2_VARIABLE: &str =
    "Rust Zone Air Previous Temperature 2";
/// Diagnostic-only Rust previous zone-timestep MAT history slot 3.
pub const RUST_ZONE_AIR_PREVIOUS_TEMPERATURE_3_VARIABLE: &str =
    "Rust Zone Air Previous Temperature 3";
/// Diagnostic-only Rust previous adaptive system-timestep MAT history slot 1.
pub const RUST_ZONE_AIR_PREVIOUS_SYSTEM_TEMPERATURE_1_VARIABLE: &str =
    "Rust Zone Air Previous System Temperature 1";
/// Diagnostic-only Rust adaptive system timestep count selected for the zone timestep.
pub const RUST_ZONE_AIR_SYSTEM_TIMESTEP_COUNT_VARIABLE: &str =
    "Rust Zone Air System Timestep Count";
/// Diagnostic-only Rust current zone air humidity ratio.
pub const RUST_ZONE_AIR_HUMIDITY_RATIO_VARIABLE: &str = "Rust Zone Air Humidity Ratio";
/// Diagnostic-only Rust zone-timestep averaged zone air humidity ratio.
pub const RUST_ZONE_AIR_ZONE_TIMESTEP_AVERAGE_HUMIDITY_RATIO_VARIABLE: &str =
    "Rust Zone Air Zone Timestep Average Humidity Ratio";
/// Diagnostic-only Rust zone air heat capacity.
pub const RUST_ZONE_AIR_HEAT_CAPACITY_VARIABLE: &str = "Rust Zone Air Heat Capacity";
/// Diagnostic-only Rust zone-timestep air power capacity.
pub const RUST_ZONE_AIR_ZONE_TIMESTEP_AIR_POWER_CAP_VARIABLE: &str =
    "Rust Zone Air Zone Timestep AirPowerCap";
/// Diagnostic-only Rust last zone-air correction air power capacity.
pub const RUST_ZONE_AIR_LAST_CORRECTION_AIR_POWER_CAP_VARIABLE: &str =
    "Rust Zone Air Last Correction AirPowerCap";
/// Diagnostic/report variable for EnergyPlus inside surface heat-balance iteration count.
pub const SURFACE_INSIDE_HEAT_BALANCE_ITERATION_COUNT_VARIABLE: &str =
    "Surface Inside Face Heat Balance Calculation Iteration Count";
