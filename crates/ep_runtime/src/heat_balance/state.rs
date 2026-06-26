//! Heat-balance trace and diagnostic state value types.

use super::algorithm::HeatBalanceZoneAirAlgorithm;
use ep_model::{
    ConstructionId, MaterialId, MaterialSurfaceRoughness, OutsideBoundaryCondition,
    SimulationModel, SurfaceId, SurfaceType, ZoneId,
};

const ENERGYPLUS_ZONE_INITIAL_TEMP_C: f64 = 23.0;

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

/// Initial heat-balance state shell for the EnergyPlus porting path.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceState {
    /// Current zone timestep index.
    pub timestep_index: usize,
    /// Per-zone heat-balance state.
    pub zones: Vec<ZoneHeatBalanceState>,
    /// Per-surface heat-balance state.
    pub surfaces: Vec<SurfaceHeatBalanceState>,
    pub(crate) surface_indexes: HeatBalanceSurfaceIndexes,
    /// Most recent per-slot CTF history terms, captured before CTF histories advance.
    pub last_ctf_history_slot_terms: Vec<HeatBalanceCtfHistorySlotSample>,
    /// Most recent per-slot CTF history terms, captured after CTF histories advance.
    pub last_ctf_history_slot_terms_after_advance: Vec<HeatBalanceCtfHistorySlotSample>,
    /// Most recent inside surface heat-balance iteration count.
    pub last_inside_surface_iteration_count: u32,
    /// Final max inside-surface temperature change from the most recent iteration loop.
    pub last_inside_surface_iteration_max_delta_c: f64,
    /// Surface that controlled the final max inside-surface temperature change.
    pub last_inside_surface_iteration_max_delta_surface_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct HeatBalanceSurfaceIndexes {
    pub(crate) surfaces_by_zone: Vec<Vec<usize>>,
    pub(crate) surfaces_by_construction: Vec<Vec<usize>>,
    pub(crate) opaque_surfaces: Vec<usize>,
    pub(crate) fenestration_surfaces: Vec<usize>,
    pub(crate) ctf_surfaces: Vec<usize>,
    pub(crate) no_mass_surfaces: Vec<usize>,
}

impl HeatBalanceSurfaceIndexes {
    pub(crate) fn from_model_surfaces(
        model: &SimulationModel,
        surfaces: &[SurfaceHeatBalanceState],
    ) -> Self {
        let mut surfaces_by_zone = vec![Vec::new(); model.typed.zones.len()];
        let mut surfaces_by_construction = vec![Vec::new(); model.typed.constructions.len()];
        let mut opaque_surfaces = Vec::new();
        let mut ctf_surfaces = Vec::new();
        let mut no_mass_surfaces = Vec::new();

        for (surface_index, surface) in surfaces.iter().enumerate() {
            if let Some(zone_surfaces) = surfaces_by_zone.get_mut(surface.zone_id.0 as usize) {
                zone_surfaces.push(surface_index);
            }
            if let Some(construction_surfaces) =
                surfaces_by_construction.get_mut(surface.construction_id.0 as usize)
            {
                construction_surfaces.push(surface_index);
            }
            opaque_surfaces.push(surface_index);
            ctf_surfaces.push(surface_index);
            if surface.heat_capacity_j_per_m2_k.is_none() {
                no_mass_surfaces.push(surface_index);
            }
        }

        Self {
            surfaces_by_zone,
            surfaces_by_construction,
            opaque_surfaces,
            fenestration_surfaces: Vec::new(),
            ctf_surfaces,
            no_mass_surfaces,
        }
    }

    pub(crate) fn surfaces_for_zone(&self, zone_id: ZoneId) -> &[usize] {
        self.surfaces_by_zone
            .get(zone_id.0 as usize)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

/// Per-zone heat-balance state shell.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneHeatBalanceState {
    /// Zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// Current mean air temperature in C.
    pub mean_air_temperature_c: f64,
    /// Last zone-timestep average mean air temperature in C.
    pub zone_timestep_average_air_temperature_c: f64,
    /// Previous mean air temperature history in C.
    pub previous_mean_air_temperatures_c: [f64; 3],
    /// Previous system-timestep mean air temperature history in C.
    pub previous_system_mean_air_temperatures_c: [f64; 3],
    /// Number of adaptive system timesteps used in the previous zone timestep.
    pub previous_system_timestep_count: u32,
    /// Current zone air humidity ratio in kgWater/kgDryAir.
    pub air_humidity_ratio: f64,
    /// Last zone-timestep average humidity ratio in kgWater/kgDryAir.
    pub zone_timestep_average_air_humidity_ratio: f64,
    /// Previous zone air humidity-ratio history in kgWater/kgDryAir.
    pub previous_air_humidity_ratios: [f64; 3],
    /// Previous system-timestep humidity-ratio history in kgWater/kgDryAir.
    pub previous_system_air_humidity_ratios: [f64; 3],
    /// Zone volume in cubic meters.
    pub volume_m3: f64,
    /// Air heat capacity in J/K.
    pub air_heat_capacity_j_per_k: f64,
    /// First hour-ending convective internal gain in W.
    pub convective_internal_gain_w: f64,
    /// Sum of opaque surface conductance for this zone in W/K.
    pub opaque_surface_conductance_w_per_k: f64,
    /// Current opaque surface heat gain to the zone in W.
    pub opaque_surface_heat_gain_w: f64,
    /// Current opaque outside-face surface conduction aggregate in W.
    pub opaque_surface_outside_conduction_w: f64,
    /// EnergyPlus `SumHA`: inside convection conductance sum in W/K.
    pub sum_ha_w_per_k: f64,
    /// EnergyPlus `SumHATsurf`: inside convection temperature sum in W.
    pub sum_hat_surf_w: f64,
    /// EnergyPlus `SumHATref`: reference-air convection temperature sum in W.
    pub sum_hat_ref_w: f64,
    /// EnergyPlus zone-air temperature coefficient snapshot for diagnostics.
    pub zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients,
    /// Optional EnergyPlus system-timestep averaged surface convection report in W.
    pub system_timestep_average_surface_convection_report_w: Option<f64>,
    /// Optional EnergyPlus system-timestep averaged air storage report in W.
    pub system_timestep_average_air_storage_report_w: Option<f64>,
}

/// Surface CTF coefficients and history constants.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceCtfState {
    /// CTF outside/X coefficient at time zero in W/m2-K.
    pub outside_0_w_per_m2_k: f64,
    /// CTF cross/Y coefficient at time zero in W/m2-K.
    pub cross_0_w_per_m2_k: f64,
    /// CTF inside/Z coefficient at time zero in W/m2-K.
    pub inside_0_w_per_m2_k: f64,
    /// Inside CTF history constant part in W/m2.
    pub const_in_part_w_per_m2: f64,
    /// Outside CTF history constant part in W/m2.
    pub const_out_part_w_per_m2: f64,
    /// CTF outside/X history coefficients in W/m2-K.
    pub outside_history_w_per_m2_k: Vec<f64>,
    /// CTF cross/Y history coefficients in W/m2-K.
    pub cross_history_w_per_m2_k: Vec<f64>,
    /// CTF inside/Z history coefficients in W/m2-K.
    pub inside_history_w_per_m2_k: Vec<f64>,
    /// CTF flux history coefficients.
    pub flux_history: Vec<f64>,
    /// Previous outside face temperature history in C.
    pub outside_temperature_history_c: Vec<f64>,
    /// Previous inside face temperature history in C.
    pub inside_temperature_history_c: Vec<f64>,
    /// Previous outside conduction flux history in W/m2.
    pub outside_flux_history_w_per_m2: Vec<f64>,
    /// Previous inside conduction flux history in W/m2.
    pub inside_flux_history_w_per_m2: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SurfaceExteriorReportTerms {
    pub(crate) convection_heat_gain_rate_w: f64,
    pub(crate) convection_heat_gain_rate_per_area_w_per_m2: f64,
    pub(crate) convection_coefficient_w_per_m2_k: f64,
    pub(crate) net_thermal_radiation_heat_gain_rate_w: f64,
    pub(crate) net_thermal_radiation_heat_gain_rate_per_area_w_per_m2: f64,
    pub(crate) thermal_radiation_to_air_coefficient_w_per_m2_k: f64,
    pub(crate) thermal_radiation_to_sky_coefficient_w_per_m2_k: f64,
    pub(crate) thermal_radiation_to_ground_coefficient_w_per_m2_k: f64,
    pub(crate) solar_radiation_heat_gain_rate_w: f64,
    pub(crate) solar_radiation_heat_gain_rate_per_area_w_per_m2: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SurfaceOutsideBalanceDiagnostics {
    pub(crate) report_temperature_c: f64,
    pub(crate) coefficient_surface_temperature_c: f64,
    pub(crate) convection_reference_temperature_c: f64,
    pub(crate) equivalent_radiant_temperature_c: f64,
    pub(crate) outside_radiation_coefficient_w_per_m2_k: f64,
    pub(crate) quick_net_inside_source_w_per_m2: f64,
    pub(crate) quick_inside_balance_term_w_per_m2: f64,
    pub(crate) quick_numerator_w_per_m2: f64,
    pub(crate) quick_denominator_w_per_m2_k: f64,
    pub(crate) quick_coupling_factor: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SurfaceIncidentSolarComponents {
    pub(crate) beam_w_per_m2: f64,
    pub(crate) sky_diffuse_w_per_m2: f64,
    pub(crate) ground_diffuse_w_per_m2: f64,
}

impl SurfaceIncidentSolarComponents {
    pub(crate) fn total_w_per_m2(self) -> f64 {
        self.beam_w_per_m2 + self.sky_diffuse_w_per_m2 + self.ground_diffuse_w_per_m2
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SurfaceBoundaryBalanceResult {
    pub(crate) temperature_c: f64,
    pub(crate) exterior_report_terms: SurfaceExteriorReportTerms,
    pub(crate) outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct InsideConvectionCoefficientInputState {
    pub(crate) inside_face_temperature_c: f64,
    pub(crate) reference_air_temperature_c: f64,
}

/// Per-surface heat-balance state shell.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceHeatBalanceState {
    /// Surface ID.
    pub surface_id: SurfaceId,
    /// Owning zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// Surface type.
    pub surface_type: SurfaceType,
    /// Outside boundary condition.
    pub outside_boundary_condition: OutsideBoundaryCondition,
    /// Optional outside boundary object name.
    pub outside_boundary_condition_object_name: Option<String>,
    /// Resolved adjacent surface for interzone surface boundaries.
    pub outside_boundary_target_surface_id: Option<SurfaceId>,
    /// Resolved adjacent zone for interzone surface, zone, or space boundaries.
    pub outside_boundary_target_zone_id: Option<ZoneId>,
    /// Resolved construction ID.
    pub construction_id: ConstructionId,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// Outside layer material ID.
    pub outside_layer_material_id: MaterialId,
    /// EnergyPlus-normalized outside layer material name.
    pub outside_layer_material_name: String,
    /// Outside layer surface roughness used by EnergyPlus exterior convection.
    pub outside_layer_roughness: MaterialSurfaceRoughness,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// Surface azimuth in degrees clockwise from north.
    pub azimuth_deg: f64,
    /// Surface tilt in degrees using EnergyPlus orientation conventions.
    pub tilt_deg: f64,
    /// Area-normalized thermal resistance in m2-K/W.
    pub thermal_resistance_m2_k_per_w: f64,
    /// Area-normalized heat capacity in J/m2-K when available.
    pub heat_capacity_j_per_m2_k: Option<f64>,
    /// Outside layer thermal absorptance used by exterior diagnostic forcing.
    pub thermal_absorptance: f64,
    /// Inside layer thermal absorptance used by interior radiant exchange/source terms.
    pub inside_thermal_absorptance: f64,
    /// Outside layer solar absorptance used by exterior diagnostic forcing.
    pub solar_absorptance: f64,
    /// Surface conductance in W/K.
    pub conductance_w_per_k: f64,
    /// Current inside convection coefficient in W/m2-K.
    pub inside_convection_coefficient_w_per_m2_k: f64,
    /// Inside face temperature used to calculate the current inside convection coefficient in C.
    pub inside_convection_input_inside_face_temperature_c: f64,
    /// Reference air temperature used to calculate the current inside convection coefficient in C.
    pub inside_convection_input_reference_air_temperature_c: f64,
    /// Reference air temperature used by the last inside convection solve in C.
    pub inside_reference_air_temperature_c: f64,
    /// Outside-face temperature used by the last inside CTF solve in C.
    pub inside_ctf_outside_temperature_c: f64,
    /// EnergyPlus `SurfQdotRadIntGainsInPerArea` source term in W/m2.
    pub inside_radiant_internal_gain_w_per_m2: f64,
    /// EnergyPlus `SurfOpaqQRadSWInAbs` absorbed inside shortwave term in W/m2.
    pub inside_shortwave_absorbed_w_per_m2: f64,
    /// EnergyPlus `SurfQAdditionalHeatSourceInside` term in W/m2.
    pub inside_additional_heat_source_w_per_m2: f64,
    /// EnergyPlus `SurfQdotRadHVACInPerArea` source term in W/m2.
    pub inside_radiant_hvac_w_per_m2: f64,
    /// EnergyPlus `SurfQdotRadNetLWInPerArea` source term in W/m2.
    pub inside_net_longwave_w_per_m2: f64,
    /// Surface CTF coefficients and history constants.
    pub ctf: SurfaceCtfState,
    /// Current opaque heat transfer to the owning zone in W.
    pub heat_gain_to_zone_w: f64,
    /// EnergyPlus-shaped outside-face report terms from the exterior balance.
    pub(crate) outside_report_terms: SurfaceExteriorReportTerms,
    /// Diagnostic outside-face balance terms from the exterior balance.
    pub(crate) outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics,
    /// Current inside face temperature in C.
    pub inside_face_temperature_c: f64,
    /// Current outside face temperature in C.
    pub outside_face_temperature_c: f64,
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

/// Inputs for advancing the first heat-balance timestep shell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceStepInput {
    /// Outdoor dry-bulb temperature in C for this timestep.
    pub outdoor_dry_bulb_c: f64,
    /// EnergyPlus-style hour ending, 1-24.
    pub hour_ending: u32,
    /// Timestep duration in seconds.
    pub timestep_seconds: f64,
}

/// Initial CTF temperature/flux history seeding used by diagnostic heat-balance traces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceCtfInitialHistoryPolicy {
    /// Existing Rust diagnostic seed: current boundary temperature and steady U-value flux.
    BoundaryTemperatureAndUValue,
    /// EnergyPlus 26.1 style InitHeatBalance seed: SurfInitialTemp inside
    /// histories, boundary outside histories, and steady U-value flux histories.
    EnergyPlusSurfInitial,
}

/// Source used for zone opaque conduction report variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceZoneConductionReportSource {
    /// Use the zone heat-balance state values captured during correction.
    ZoneState,
    /// Sum the same per-surface report rates used by surface conduction outputs.
    SurfaceReport,
}

/// Sampling mode used for zone air heat-balance report variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceZoneAirReportSampling {
    /// Average reported values over the zone timesteps in each hour.
    Average,
    /// Report the last system state in each hour for source-order probes.
    LastSystemState,
}

/// Timing for zone-air correction during interleaved surface-balance probes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceSurfaceLoopZoneAirCorrection {
    /// Correct zone air after every surface loop pass.
    EachSurfaceIteration,
    /// Correct zone air once after the inside surface loop converges.
    AfterSurfaceLoop,
}

/// Options for the heat-balance zone-air diagnostic trace.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceSimulationOptions {
    /// Number of hourly weather samples to execute.
    pub sample_count: usize,
    /// Initial zone mean air temperature in C.
    pub initial_zone_air_temperature_c: f64,
    /// Optional run-period warmup loop.
    pub warmup: HeatBalanceWarmupOptions,
    /// Zone-air temperature algorithm for diagnostic probes.
    pub zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    /// Number of inside/outside surface-balance passes per zone timestep.
    pub surface_iteration_count: u32,
    /// Optional frozen inside-convection coefficient re-evaluation interval.
    pub inside_hconv_reevaluation_interval: Option<u32>,
    /// Initial CTF temperature/flux history seeding policy.
    pub ctf_initial_history_policy: HeatBalanceCtfInitialHistoryPolicy,
    /// Source used for zone opaque conduction report variables.
    pub zone_conduction_report_source: HeatBalanceZoneConductionReportSource,
    /// Sampling mode used for zone air heat-balance report variables.
    pub zone_air_report_sampling: HeatBalanceZoneAirReportSampling,
    /// Timing for zone-air correction during interleaved surface-balance probes.
    pub surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
}

/// Warmup settings for heat-balance diagnostic traces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceWarmupOptions {
    /// Whether to execute a warmup loop before reported samples are recorded.
    pub enabled: bool,
    /// Minimum number of repeated warmup days.
    pub minimum_days: u32,
    /// Maximum number of repeated warmup days.
    pub maximum_days: u32,
    /// Zone end-state convergence tolerance in delta C.
    pub temperature_convergence_tolerance_delta_c: f64,
}

impl HeatBalanceWarmupOptions {
    /// Creates disabled warmup settings.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            minimum_days: 0,
            maximum_days: 0,
            temperature_convergence_tolerance_delta_c: 0.0,
        }
    }
}

impl HeatBalanceSimulationOptions {
    /// Creates options with a fixed hourly sample count.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            initial_zone_air_temperature_c: ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            warmup: HeatBalanceWarmupOptions::disabled(),
            zone_air_algorithm: HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            surface_iteration_count: 1,
            inside_hconv_reevaluation_interval: None,
            ctf_initial_history_policy:
                HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue,
            zone_conduction_report_source: HeatBalanceZoneConductionReportSource::ZoneState,
            zone_air_report_sampling: HeatBalanceZoneAirReportSampling::Average,
            surface_loop_zone_air_correction:
                HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
        }
    }

    /// Creates options with a run-period warmup loop based on typed Building settings.
    #[must_use]
    pub fn hourly_samples_with_model_warmup(model: &SimulationModel, sample_count: usize) -> Self {
        let Some(building) = model.typed.building.as_ref() else {
            return Self::hourly_samples(sample_count);
        };
        let minimum_days = building.minimum_number_of_warmup_days;
        let maximum_days = building.maximum_number_of_warmup_days.max(minimum_days);
        Self {
            sample_count,
            initial_zone_air_temperature_c: ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            warmup: HeatBalanceWarmupOptions {
                enabled: maximum_days > 0,
                minimum_days,
                maximum_days,
                temperature_convergence_tolerance_delta_c: building
                    .temperature_convergence_tolerance_delta_c,
            },
            zone_air_algorithm: HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            surface_iteration_count: 1,
            inside_hconv_reevaluation_interval: None,
            ctf_initial_history_policy:
                HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue,
            zone_conduction_report_source: HeatBalanceZoneConductionReportSource::ZoneState,
            zone_air_report_sampling: HeatBalanceZoneAirReportSampling::Average,
            surface_loop_zone_air_correction:
                HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
        }
    }

    /// Returns options with an explicit zone-air diagnostic algorithm.
    #[must_use]
    pub const fn with_zone_air_algorithm(
        mut self,
        zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    ) -> Self {
        self.zone_air_algorithm = zone_air_algorithm;
        self
    }

    /// Returns options with an elevated warmup minimum day count for diagnostics.
    #[must_use]
    pub fn with_warmup_minimum_days(mut self, minimum_days: u32) -> Self {
        if self.warmup.enabled {
            self.warmup.minimum_days = minimum_days;
            self.warmup.maximum_days = self.warmup.maximum_days.max(minimum_days);
        }
        self
    }

    /// Returns options with an explicit surface-balance iteration count.
    #[must_use]
    pub const fn with_surface_iteration_count(mut self, surface_iteration_count: u32) -> Self {
        self.surface_iteration_count = if surface_iteration_count == 0 {
            1
        } else {
            surface_iteration_count
        };
        self
    }

    /// Returns options with a frozen inside-convection coefficient re-evaluation interval.
    #[must_use]
    pub const fn with_inside_hconv_reevaluation_interval(
        mut self,
        inside_hconv_reevaluation_interval: Option<u32>,
    ) -> Self {
        self.inside_hconv_reevaluation_interval = match inside_hconv_reevaluation_interval {
            Some(0) => None,
            interval => interval,
        };
        self
    }

    /// Returns options with an explicit initial CTF history seed policy.
    #[must_use]
    pub const fn with_ctf_initial_history_policy(
        mut self,
        ctf_initial_history_policy: HeatBalanceCtfInitialHistoryPolicy,
    ) -> Self {
        self.ctf_initial_history_policy = ctf_initial_history_policy;
        self
    }

    /// Returns options with an explicit zone opaque conduction report source.
    #[must_use]
    pub const fn with_zone_conduction_report_source(
        mut self,
        zone_conduction_report_source: HeatBalanceZoneConductionReportSource,
    ) -> Self {
        self.zone_conduction_report_source = zone_conduction_report_source;
        self
    }

    /// Returns options with an explicit zone air heat-balance report sampling mode.
    #[must_use]
    pub const fn with_zone_air_report_sampling(
        mut self,
        zone_air_report_sampling: HeatBalanceZoneAirReportSampling,
    ) -> Self {
        self.zone_air_report_sampling = zone_air_report_sampling;
        self
    }

    /// Returns options with explicit zone-air correction timing in the surface loop.
    #[must_use]
    pub const fn with_surface_loop_zone_air_correction(
        mut self,
        surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
    ) -> Self {
        self.surface_loop_zone_air_correction = surface_loop_zone_air_correction;
        self
    }
}
