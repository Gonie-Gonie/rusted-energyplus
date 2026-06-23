//! Heat-balance trace and diagnostic state value types.

use ep_model::ZoneId;

/// EnergyPlus zone-air temperature coefficient snapshot.
///
/// These fields mirror the predictor/corrector coefficient names in
/// `ZoneTempPredictorCorrector.cc`. They are diagnostic state until the full
/// zone-air predictor is wired into the heat-balance timestep.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneAirTemperatureCoefficients {
    /// EnergyPlus `TempDepCoef` in W/K.
    pub temp_dependent_coefficient_w_per_k: f64,
    /// EnergyPlus `TempIndCoef` in W.
    pub temp_independent_coefficient_w: f64,
    /// EnergyPlus `AirPowerCap = C_air / dt` in W/K.
    pub air_power_cap_w_per_k: f64,
    /// EnergyPlus third-order `TempHistoryTerm` in W.
    pub third_order_history_term_w: f64,
    /// EnergyPlus third-order `tempDepLoad` in W/K.
    pub third_order_temp_dependent_load_w_per_k: f64,
    /// EnergyPlus third-order `tempIndLoad` in W.
    pub third_order_temp_independent_load_w: f64,
}

impl ZoneAirTemperatureCoefficients {
    pub(crate) const ZERO: Self = Self {
        temp_dependent_coefficient_w_per_k: 0.0,
        temp_independent_coefficient_w: 0.0,
        air_power_cap_w_per_k: 0.0,
        third_order_history_term_w: 0.0,
        third_order_temp_dependent_load_w_per_k: 0.0,
        third_order_temp_independent_load_w: 0.0,
    };
}

/// One per-slot CTF history contribution captured for a heat-balance timestep.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceCtfHistorySlotSample {
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// One-based CTF history slot index.
    pub slot_index: usize,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// CTF outside/X history coefficient for this slot in W/m2-K.
    pub outside_history_coefficient_w_per_m2_k: f64,
    /// CTF cross/Y history coefficient for this slot in W/m2-K.
    pub cross_history_coefficient_w_per_m2_k: f64,
    /// CTF inside/Z history coefficient for this slot in W/m2-K.
    pub inside_history_coefficient_w_per_m2_k: f64,
    /// CTF flux history coefficient for this slot.
    pub flux_history_coefficient: f64,
    /// Previous outside face temperature in C for this slot.
    pub outside_temperature_history_c: f64,
    /// Previous inside face temperature in C for this slot.
    pub inside_temperature_history_c: f64,
    /// Previous outside conduction flux in W/m2 for this slot.
    pub outside_flux_history_w_per_m2: f64,
    /// Previous inside conduction flux in W/m2 for this slot.
    pub inside_flux_history_w_per_m2: f64,
    /// Inside-face temperature-history contribution in W.
    pub inside_temperature_term_w: f64,
    /// Inside-face flux-history contribution in W.
    pub inside_flux_term_w: f64,
    /// Inside-face total history contribution in W.
    pub inside_total_term_w: f64,
    /// Outside-face temperature-history contribution in reported W sign.
    pub outside_temperature_term_w: f64,
    /// Outside-face flux-history contribution in reported W sign.
    pub outside_flux_term_w: f64,
    /// Outside-face total history contribution in reported W sign.
    pub outside_total_term_w: f64,
}

/// First reported hourly sample CTF history contribution averaged by slot.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceCtfHistorySlotFirstSample {
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// One-based CTF history slot index.
    pub slot_index: usize,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// Number of zone timesteps averaged into the first hourly sample.
    pub timestep_count: usize,
    /// CTF outside/X history coefficient for this slot in W/m2-K.
    pub outside_history_coefficient_w_per_m2_k: f64,
    /// CTF cross/Y history coefficient for this slot in W/m2-K.
    pub cross_history_coefficient_w_per_m2_k: f64,
    /// CTF inside/Z history coefficient for this slot in W/m2-K.
    pub inside_history_coefficient_w_per_m2_k: f64,
    /// CTF flux history coefficient for this slot.
    pub flux_history_coefficient: f64,
    /// Average previous outside face temperature in C for this slot.
    pub outside_temperature_history_c: f64,
    /// Average previous inside face temperature in C for this slot.
    pub inside_temperature_history_c: f64,
    /// Average previous outside conduction flux in W/m2 for this slot.
    pub outside_flux_history_w_per_m2: f64,
    /// Average previous inside conduction flux in W/m2 for this slot.
    pub inside_flux_history_w_per_m2: f64,
    /// Average inside-face temperature-history contribution in W.
    pub inside_temperature_term_w: f64,
    /// Average inside-face flux-history contribution in W.
    pub inside_flux_term_w: f64,
    /// Average inside-face total history contribution in W.
    pub inside_total_term_w: f64,
    /// Average outside-face temperature-history contribution in reported W sign.
    pub outside_temperature_term_w: f64,
    /// Average outside-face flux-history contribution in reported W sign.
    pub outside_flux_term_w: f64,
    /// Average outside-face total history contribution in reported W sign.
    pub outside_total_term_w: f64,
}

/// One reported hourly sample CTF history contribution averaged by slot.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceCtfHistorySlotHourlySample {
    /// Zero-based hourly sample index.
    pub sample_index: usize,
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// One-based CTF history slot index.
    pub slot_index: usize,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// Number of zone timesteps averaged into the hourly sample.
    pub timestep_count: usize,
    /// CTF outside/X history coefficient for this slot in W/m2-K.
    pub outside_history_coefficient_w_per_m2_k: f64,
    /// CTF cross/Y history coefficient for this slot in W/m2-K.
    pub cross_history_coefficient_w_per_m2_k: f64,
    /// CTF inside/Z history coefficient for this slot in W/m2-K.
    pub inside_history_coefficient_w_per_m2_k: f64,
    /// CTF flux history coefficient for this slot.
    pub flux_history_coefficient: f64,
    /// Average previous outside face temperature in C for this slot.
    pub outside_temperature_history_c: f64,
    /// Average previous inside face temperature in C for this slot.
    pub inside_temperature_history_c: f64,
    /// Average previous outside conduction flux in W/m2 for this slot.
    pub outside_flux_history_w_per_m2: f64,
    /// Average previous inside conduction flux in W/m2 for this slot.
    pub inside_flux_history_w_per_m2: f64,
    /// Average inside-face temperature-history contribution in W.
    pub inside_temperature_term_w: f64,
    /// Average inside-face flux-history contribution in W.
    pub inside_flux_term_w: f64,
    /// Average inside-face total history contribution in W.
    pub inside_total_term_w: f64,
    /// Average outside-face temperature-history contribution in reported W sign.
    pub outside_temperature_term_w: f64,
    /// Average outside-face flux-history contribution in reported W sign.
    pub outside_flux_term_w: f64,
    /// Average outside-face total history contribution in reported W sign.
    pub outside_total_term_w: f64,
}

/// One surface-state sample captured after a zone timestep in the first reported hour.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSurfaceFirstSampleTrace {
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// One-based zone timestep within the first reported hourly sample.
    pub timestep_index: u32,
    /// Outdoor dry-bulb temperature used by this timestep in C.
    pub outdoor_dry_bulb_c: f64,
    /// Owning-zone mean air temperature after the timestep in C.
    pub zone_mean_air_temperature_c: f64,
    /// Inside face temperature after the timestep in C.
    pub inside_face_temperature_c: f64,
    /// Inside face temperature used to calculate inside hconv in C.
    pub inside_convection_input_inside_face_temperature_c: f64,
    /// Reference air temperature used to calculate inside hconv in C.
    pub inside_convection_input_reference_air_temperature_c: f64,
    /// Reported outside face temperature after the timestep in C.
    pub outside_face_temperature_c: f64,
    /// Inside-face convection heat gain rate in W.
    pub inside_convection_heat_gain_rate_w: f64,
    /// Inside-face net longwave heat gain rate in W.
    pub inside_net_surface_thermal_radiation_heat_gain_rate_w: f64,
    /// Inside-face conduction heat transfer rate in W.
    pub inside_conduction_rate_w: f64,
    /// Outside-face conduction heat transfer rate in W.
    pub outside_conduction_rate_w: f64,
    /// Surface heat storage rate in W.
    pub heat_storage_rate_w: f64,
    /// Outside-face convection heat gain rate in W.
    pub outside_convection_heat_gain_rate_w: f64,
    /// Outside-face net thermal radiation heat gain rate in W.
    pub outside_net_thermal_radiation_heat_gain_rate_w: f64,
    /// Outside-face solar radiation heat gain rate in W.
    pub outside_solar_radiation_heat_gain_rate_w: f64,
}

/// One inside-surface iteration sample captured after a zone timestep in the first reported hour.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSurfaceIterationFirstSampleTrace {
    /// One-based zone timestep within the first reported hourly sample.
    pub timestep_index: u32,
    /// Number of inside-surface heat-balance iterations executed in this timestep.
    pub inside_surface_iteration_count: u32,
    /// Final max inside-surface temperature change in C.
    pub max_inside_surface_delta_c: f64,
    /// Surface that controlled the final max inside-surface temperature change.
    pub max_delta_surface_name: Option<String>,
}

/// One inside-surface iteration sample captured after a zone timestep.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSurfaceIterationSampleTrace {
    /// Zero-based hourly output sample index.
    pub sample_index: usize,
    /// One-based zone timestep within the hourly sample.
    pub timestep_index: u32,
    /// Number of inside-surface heat-balance iterations executed in this timestep.
    pub inside_surface_iteration_count: u32,
    /// Final max inside-surface temperature change in C.
    pub max_inside_surface_delta_c: f64,
    /// Surface that controlled the final max inside-surface temperature change.
    pub max_delta_surface_name: Option<String>,
}

/// Per-zone zone-air state captured at a diagnostic boundary.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceZoneAirStateSample {
    /// Zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// Current mean air temperature in C.
    pub mean_air_temperature_c: f64,
    /// Last zone-timestep average mean air temperature in C.
    pub zone_timestep_average_air_temperature_c: f64,
    /// Previous zone-timestep mean-air-temperature history in C.
    pub previous_mean_air_temperatures_c: [f64; 3],
    /// Previous system-timestep mean-air-temperature history in C.
    pub previous_system_mean_air_temperatures_c: [f64; 3],
    /// Adaptive system timestep count used in the previous zone timestep.
    pub previous_system_timestep_count: u32,
    /// Current zone air humidity ratio in kgWater/kgDryAir.
    pub air_humidity_ratio: f64,
    /// Last zone-timestep average humidity ratio in kgWater/kgDryAir.
    pub zone_timestep_average_air_humidity_ratio: f64,
    /// Previous zone-timestep humidity-ratio history in kgWater/kgDryAir.
    pub previous_air_humidity_ratios: [f64; 3],
    /// Previous system-timestep humidity-ratio history in kgWater/kgDryAir.
    pub previous_system_air_humidity_ratios: [f64; 3],
    /// Zone air heat capacity in J/K.
    pub air_heat_capacity_j_per_k: f64,
    /// Latest zone-air coefficient snapshot.
    pub zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients,
}

/// Per-zone zone-air state captured at the end of one warmup day.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceWarmupDayEndZoneAirStateSample {
    /// One-based warmup day index.
    pub day_index: u32,
    /// Per-zone state at the end of the warmup day.
    pub state: HeatBalanceZoneAirStateSample,
}

/// Per-zone zone-air state captured for one timestep in the first reported hour.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceZoneAirFirstSampleTrace {
    /// Zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// One-based zone timestep within the first reported hourly sample.
    pub timestep_index: u32,
    /// Interpolated outdoor dry-bulb temperature in C used for the timestep.
    pub outdoor_dry_bulb_c: f64,
    /// Zone-timestep length in seconds.
    pub timestep_seconds: f64,
    /// Current mean air temperature in C.
    pub mean_air_temperature_c: f64,
    /// Zone-timestep average mean air temperature in C.
    pub zone_timestep_average_air_temperature_c: f64,
    /// Previous zone-timestep mean-air-temperature history in C.
    pub previous_mean_air_temperatures_c: [f64; 3],
    /// Previous system-timestep mean-air-temperature history in C.
    pub previous_system_mean_air_temperatures_c: [f64; 3],
    /// Adaptive system timestep count used in the previous zone timestep.
    pub previous_system_timestep_count: u32,
    /// Current zone air humidity ratio in kgWater/kgDryAir.
    pub air_humidity_ratio: f64,
    /// Zone-timestep average humidity ratio in kgWater/kgDryAir.
    pub zone_timestep_average_air_humidity_ratio: f64,
    /// Zone air heat capacity in J/K.
    pub air_heat_capacity_j_per_k: f64,
    /// Zone air power capacity recomputed from the active zone timestep.
    pub zone_timestep_air_power_cap_w_per_k: f64,
    /// Latest zone-air coefficient snapshot.
    pub zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients,
    /// Third-order temperature solution numerator in W.
    pub third_order_solution_numerator_w: f64,
    /// Third-order temperature solution denominator in W/K.
    pub third_order_solution_denominator_w_per_k: f64,
    /// Third-order temperature solution in C from the stored coefficients.
    pub third_order_solution_temperature_c: f64,
}
