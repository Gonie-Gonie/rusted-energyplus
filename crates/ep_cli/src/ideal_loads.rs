#![allow(
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::type_complexity
)]

mod case_adapter;
mod commands;
mod reports;

pub(crate) use commands::{
    generate_ideal_loads_no_oa_sensible_report, generate_ideal_loads_outdoor_air_design_flow_report,
};

use reports::write_outdoor_air_artifacts;

use case_adapter::{
    IdealLoadsTimestepContext, ideal_loads_sample_timestep_hours,
    ideal_loads_sample_timestep_seconds, ideal_loads_timestep_context,
};

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Instant;

use ep_compare::{
    SeriesAlignment, SeriesComparisonStatus, SeriesDivergenceKind, SeriesSample, Tolerance,
    compare_series_samples_v2, load_eso_time_series, load_mtr_time_series_for_frequency,
};
use ep_compiler::compile_raw_model;
use ep_conformance::{
    ComparisonClass, ConformanceCase, EvidenceDomain, MeterRequest, OutputFrequency, OutputLevel,
    OutputRequest, SourceArtifact, VariableClass,
};
use ep_model::{
    AutoOrNumber, AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    DesignSpecificationOutdoorAirMethod, FirstHourInterpolationStartingValues, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsLimit, NormalizedName,
    OutdoorAirEconomizerType, OutputHandle, PeopleNumberCalculationMethod, ScheduleId,
    SimulationModel, SurfaceType, TypedModel, Zone,
};
use ep_runtime::schedules::hour_only_single_period_compact_schedule_segments;
use ep_runtime::{
    EpwRecord, IDEAL_LOADS_ENERGY_OUTPUT_LEVEL_POLICY, IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE,
    IDEAL_LOADS_FUEL_ENERGY_OUTPUT_LEVEL_POLICY, IDEAL_LOADS_METER_AGGREGATION_SOURCE,
    IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE, IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE,
    IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT, IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE,
    IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE, IDEAL_LOADS_RATE_OUTPUT_SOURCE,
    IDEAL_LOADS_RATE_OUTPUT_TIMESTEP_SOURCE, IDEAL_LOADS_RUNTIME_BINDING_SOURCE,
    IDEAL_LOADS_RUNTIME_STRING_LOOKUP_POLICY, IDEAL_LOADS_ZONE_EQUIPMENT_DISPATCH_PATH,
    IdealLoadsFeatureFlags, IdealLoadsMinimumOutdoorAirCompatInput, IdealLoadsOutdoorAirContext,
    IdealLoadsOutdoorAirNodeState, IdealLoadsOutdoorAirSensibleResult, IdealLoadsReportSnapshot,
    IdealLoadsSensibleLimitContext, IdealLoadsSensibleMode, IdealLoadsUnsupportedFeature,
    IdealLoadsZoneEquipmentDispatchValidation, IdealLoadsZoneState, NoOaHumidistatClosedLoopState,
    NoOaHumidistatZoneTimestepError, NoOaHumidistatZoneTimestepInput,
    NoOaThirdOrderMoistureDemandInput, OutputSeries, ResultStore, RuntimeMeterRequest,
    RuntimeOutputFrequency, RuntimeOutputRegistry, SimPurchasedAirCompatInput,
    SimPurchasedAirOutdoorAirCompatInput, ZONE_IDEAL_LOADS_ECONOMIZER_ACTIVE_TIME,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_ACTIVE_TIME, ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_HEATING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_COOLING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_HEATING_RATE, ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO,
    ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE, ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE, ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE, ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE, ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE, ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE, ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY_RATE, ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY_RATE, ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE, ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE, ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE, ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD,
    ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION, ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE,
    ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD, ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION,
    ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE, ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION,
    ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT,
    ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD,
    ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD, ZONE_THERMOSTAT_COOLING_SETPOINT_TEMPERATURE,
    ZONE_THERMOSTAT_HEATING_SETPOINT_TEMPERATURE, ZoneSysEnergyDemand,
    advance_no_oa_humidistat_zone_timestep_compat, calc_no_oa_third_order_moisture_demand_compat,
    classify_no_oa_no_limit_sensible_subset, classify_no_oa_sensible_subset,
    energyplus_moist_air_density_kg_per_m3, ideal_loads_facility_meter_binding,
    ideal_loads_zone_equipment_stages, load_epw_records, meter_rate_to_energy_j,
    purchased_air_source_order_stages, select_purchased_air_branch, sim_purchased_air_compat,
    sim_purchased_air_outdoor_air_compat, surface_area_m2, third_order_humidity_history_term,
    validate_ideal_loads_zone_equipment_dispatch,
};

use crate::conformance_artifacts::{
    BaselineSummary, ReportTimingSummary, append_timing_to_json_object, elapsed_seconds_since,
    generate_conformance_baseline_in_dir,
};
use crate::{
    comparison_class_label, evidence_domain_label, json_number, json_string, markdown_cell,
    output_frequency_label, output_level_label, source_artifact_label, variable_class_label,
};

const SYSTEM_NODE_TEMPERATURE: &str = "System Node Temperature";
const SYSTEM_NODE_HUMIDITY_RATIO: &str = "System Node Humidity Ratio";
const SYSTEM_NODE_MASS_FLOW_RATE: &str = "System Node Mass Flow Rate";
const ENVIRONMENT_KEY: &str = "Environment";
const SITE_OUTDOOR_AIR_BAROMETRIC_PRESSURE: &str = "Site Outdoor Air Barometric Pressure";
const ZONE_AIR_TEMPERATURE: &str = "Zone Air Temperature";
const ZONE_AIR_HUMIDITY_RATIO: &str = "Zone Air Humidity Ratio";
const ZONE_MEAN_AIR_HUMIDITY_RATIO: &str = "Zone Mean Air Humidity Ratio";
const ZONE_OTHER_EQUIPMENT_LATENT_GAIN_RATE: &str = "Zone Other Equipment Latent Gain Rate";
const ZONE_TOTAL_INTERNAL_LATENT_GAIN_RATE: &str = "Zone Total Internal Latent Gain Rate";
const ZONE_AIR_CO2_PREDICTED_LOAD_TO_SETPOINT_MASS_FLOW_RATE: &str =
    "Zone Air CO2 Predicted Load to Setpoint Mass Flow Rate";
const ZONE_SYSTEM_PREDICTED_SETPOINT_LOAD: &str =
    "Zone System Predicted Sensible Load to Setpoint Heat Transfer Rate";
const ZONE_SYSTEM_PREDICTED_HEATING_LOAD: &str =
    "Zone System Predicted Sensible Load to Heating Setpoint Heat Transfer Rate";
const ZONE_SYSTEM_PREDICTED_COOLING_LOAD: &str =
    "Zone System Predicted Sensible Load to Cooling Setpoint Heat Transfer Rate";
const IDEAL_LOADS_OUTDOOR_AIR_FLOW_ZONE_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_flow_zone_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_FLOW_PERSON_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_flow_person_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_FLOW_AREA_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_flow_area_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_AIR_CHANGES_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_air_changes_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_SUM_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_sum_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_MAXIMUM_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_maximum_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_DIFFERENTIAL_DRY_BULB_ECONOMIZER_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_differential_dry_bulb_economizer_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_DIFFERENTIAL_ENTHALPY_ECONOMIZER_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_differential_enthalpy_economizer_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEAT_RECOVERY_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_sensible_heat_recovery_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_ENTHALPY_HEAT_RECOVERY_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_enthalpy_heat_recovery_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_OCCUPANCY_DCV_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_occupancy_dcv_conformance_candidate_001";
const IDEAL_LOADS_OUTDOOR_AIR_CO2_DCV_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_outdoor_air_co2_dcv_conformance_candidate_001";
const IDEAL_LOADS_NO_OA_FACILITY_METER_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_no_oa_facility_meter_conformance_candidate_001";
const IDEAL_LOADS_NO_OA_FACILITY_METER_MONTHLY_RUN_PERIOD_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_no_oa_facility_meter_monthly_run_period_conformance_candidate_001";
const IDEAL_LOADS_NO_OA_FACILITY_METER_CONFORMANCE_METERS: &[&str] =
    &["DistrictHeatingWater:Facility", "DistrictCooling:Facility"];
const IDEAL_LOADS_HUMIDITY_FACILITY_METER_CONFORMANCE_FREQUENCIES: &[OutputFrequency] = &[
    OutputFrequency::Hourly,
    OutputFrequency::Monthly,
    OutputFrequency::RunPeriod,
];
const IDEAL_LOADS_CONSTANT_SUPPLY_HUMIDITY_COOLING_ANNUAL_METER_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_constant_supply_humidity_cooling_annual_meter_conformance_candidate_001";
const IDEAL_LOADS_CONSTANT_SUPPLY_HUMIDITY_HEATING_ANNUAL_METER_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_constant_supply_humidity_heating_annual_meter_conformance_candidate_001";
const IDEAL_LOADS_HUMIDISTAT_HUMIDIFICATION_ANNUAL_METER_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_humidistat_humidification_annual_meter_conformance_candidate_001";
const IDEAL_LOADS_HUMIDISTAT_DEHUMIDIFICATION_ANNUAL_METER_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_humidistat_dehumidification_annual_meter_conformance_candidate_001";
const IDEAL_LOADS_HUMIDITY_ANNUAL_FACILITY_METER_CONFORMANCE_CASE_IDS: &[&str] = &[
    IDEAL_LOADS_CONSTANT_SUPPLY_HUMIDITY_COOLING_ANNUAL_METER_CONFORMANCE_CASE_ID,
    IDEAL_LOADS_CONSTANT_SUPPLY_HUMIDITY_HEATING_ANNUAL_METER_CONFORMANCE_CASE_ID,
    IDEAL_LOADS_HUMIDISTAT_DEHUMIDIFICATION_ANNUAL_METER_CONFORMANCE_CASE_ID,
    IDEAL_LOADS_HUMIDISTAT_HUMIDIFICATION_ANNUAL_METER_CONFORMANCE_CASE_ID,
];
const IDEAL_LOADS_NO_OA_REPORT_ENERGY_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_no_oa_report_energy_conformance_candidate_001";
const IDEAL_LOADS_NO_OA_REPORT_ENERGY_CONFORMANCE_OUTPUTS: &[&str] = &[
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY,
];
const IDEAL_LOADS_NO_OA_REPORT_ENERGY_CONFORMANCE_POLICY: &str =
    "conformance for declared no-OA non-fuel ReportPurchasedAir energy rows only";
const IDEAL_LOADS_HUMIDITY_REPORT_PURCHASED_AIR_CONFORMANCE_CASE_IDS: &[&str] = &[
    "ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001",
    "ideal_loads_constant_supply_humidity_heating_conformance_candidate_001",
    "ideal_loads_humidistat_dehumidification_conformance_candidate_001",
    "ideal_loads_humidistat_humidification_conformance_candidate_001",
];
const IDEAL_LOADS_HUMIDITY_REPORT_ENERGY_CONFORMANCE_POLICY: &str =
    "conformance for declared no-OA humidity-control ReportPurchasedAir energy rows only";
const IDEAL_LOADS_HUMIDITY_FUEL_EFFICIENCY_CONFORMANCE_POLICY: &str =
    "conformance for declared no-OA humidity-control blank fuel-efficiency rows only";
const IDEAL_LOADS_HUMIDITY_RATE_CONFORMANCE_OUTPUTS: &[&str] = &[
    ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
    ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE,
    ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE,
    ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE,
];
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_constant_fuel_efficiency_conformance_candidate_001";
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_blank_fuel_efficiency_conformance_candidate_001";
const IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_CASE_ID: &str =
    "ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001";
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS: &[&str] = &[
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY,
];
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS: &[&str] = &[
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY,
];
const IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS: &[&str] = &[
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY,
];
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_POLICY: &str =
    "conformance for declared no-OA blank fuel-efficiency rows only";
const IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_POLICY: &str =
    "conformance for declared no-OA non-constant Schedule:Compact fuel-efficiency rows only";
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_POLICY: &str =
    "conformance for declared no-OA constant Schedule:Constant fuel-efficiency rows only";
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_REPORT_SOURCE: &str =
    "EnergyPlus ReportPurchasedAir blank fuel-efficiency schedule branch";
const IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_REPORT_SOURCE: &str =
    "EnergyPlus ReportPurchasedAir non-constant Schedule:Compact fuel-efficiency schedule branch";
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_REPORT_SOURCE: &str =
    "EnergyPlus ReportPurchasedAir constant Schedule:Constant fuel-efficiency schedule branch";
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_RATE_SOURCE: &str =
    "rust-ideal-loads-blank-fuel-efficiency";
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_ENERGY_SOURCE: &str =
    "rust-ideal-loads-blank-fuel-efficiency-time-step-energy";
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_RATE_SOURCE: &str =
    "rust-ideal-loads-constant-fuel-efficiency";
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_ENERGY_SOURCE: &str =
    "rust-ideal-loads-constant-fuel-efficiency-time-step-energy";
const IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_RATE_SOURCE: &str =
    "rust-ideal-loads-non-constant-fuel-efficiency";
const IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_ENERGY_SOURCE: &str =
    "rust-ideal-loads-non-constant-fuel-efficiency-time-step-energy";
const IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_REPORT_SOURCE: &str =
    "EnergyPlus ReportPurchasedAir blank fuel-efficiency schedule branch; diagnostic-only";
const IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_REPORT_SOURCE: &str = "EnergyPlus ReportPurchasedAir constant Schedule:Constant fuel-efficiency schedule branch; diagnostic-only";
const IDEAL_LOADS_FACILITY_METER_HOURLY_RUST_SOURCE: &str =
    "rust-ideal-loads-hourly-facility-meter-from-fuel-energy";
const IDEAL_LOADS_FACILITY_METER_MONTHLY_RUST_SOURCE: &str =
    "rust-ideal-loads-monthly-facility-meter-from-fuel-energy";
const IDEAL_LOADS_FACILITY_METER_ANNUAL_RUST_SOURCE: &str =
    "rust-ideal-loads-annual-facility-meter-from-fuel-energy";
const IDEAL_LOADS_FACILITY_METER_RUN_PERIOD_RUST_SOURCE: &str =
    "rust-ideal-loads-run-period-facility-meter-from-fuel-energy";
const IDEAL_LOADS_FACILITY_METER_DIAGNOSTIC_REPORT_SOURCE: &str =
    "EnergyPlus Output:Meter hourly MTR vs Rust aggregated fuel-energy diagnostic";
const IDEAL_LOADS_FACILITY_METER_CONFORMANCE_REPORT_SOURCE: &str =
    "EnergyPlus Output:Meter hourly MTR vs Rust aggregated fuel-energy conformance";
const IDEAL_LOADS_FACILITY_METER_MONTHLY_RUN_PERIOD_CONFORMANCE_REPORT_SOURCE: &str = "EnergyPlus Output:Meter monthly/annual/run-period MTR vs Rust aggregated fuel-energy conformance";
const IDEAL_LOADS_HUMIDITY_FACILITY_METER_CONFORMANCE_REPORT_SOURCE: &str = "EnergyPlus Output:Meter hourly/monthly/run-period MTR vs Rust aggregated fuel-energy conformance";
const IDEAL_LOADS_HUMIDITY_ANNUAL_FACILITY_METER_CONFORMANCE_REPORT_SOURCE: &str =
    "EnergyPlus Output:Meter annual full-year MTR vs Rust aggregated fuel-energy conformance";
const IDEAL_LOADS_FINITE_LIMIT_RECIRCULATION_STATE_SOURCE: &str = "EnergyPlus return/exhaust recirculation node same-call state for finite-limit no-OA mixed-air and report calculations";
const IDEAL_LOADS_HUMIDITY_CONTROL_RECIRCULATION_STATE_SOURCE: &str = "EnergyPlus return/exhaust recirculation node same-call state for no-OA humidity-control mixed-air calculations";
const IDEAL_LOADS_OUTDOOR_AIR_RECIRCULATION_STATE_SOURCE: &str = "EnergyPlus return/exhaust recirculation node same-call state for outdoor-air mixed-air, economizer, and heat-recovery calculations";
const IDEAL_LOADS_SOURCE_MAP_ANCHOR: &str = "docs/src/porting-map/ideal-loads-source-map.md";
const IDEAL_LOADS_NODE_OUTPUT_TIMESTAMP_ALIGNMENT: &str = "timestamp";
const IDEAL_LOADS_NO_OA_SOURCE_ORDER_WRAPPER: &str =
    "ep_runtime::ideal_loads::sim_purchased_air_compat";
const IDEAL_LOADS_OUTDOOR_AIR_SOURCE_ORDER_WRAPPER: &str =
    "ep_runtime::ideal_loads::sim_purchased_air_outdoor_air_compat";
const IDEAL_LOADS_INVOCATION_PATH: &str =
    "zone-equipment-validated source-order PurchasedAir wrapper";
const IDEAL_LOADS_DIRECT_CALC_HELPER_INVOCATION: bool = false;
const IDEAL_LOADS_ZONE_EQUIPMENT_EXECUTION_BOUNDARY: &str = "validated typed ZoneEquipmentManager path; report generator invokes source-order PurchasedAir wrapper";
const IDEAL_LOADS_FEATURE_DISPATCH_POLICY: &str = "compile feature flags select branch-specific source-order compat functions; unsupported active feature combinations emit diagnostics instead of approximate fallback";
const IDEAL_LOADS_PREBOUND_ID_CONTRACT: &str = "compile-stage IdealLoadsAirSystemId, ZoneId, supply NodeId, return NodeId, zone air NodeId, optional outdoor air NodeId, availability ScheduleId, heating availability ScheduleId, and cooling availability ScheduleId";
const IDEAL_LOADS_PSYCHROMETRIC_EVALUATION_POLICY: &str = "compatibility reports use source-order direct psychrometric evaluation with EnergyPlus Psat cache-temperature quantization; no reordering is enabled";
const IDEAL_LOADS_PSYCHROMETRIC_CACHE_POLICY: &str = "saturation-pressure evaluation mirrors EnergyPlus default PsyPsatFnTemp cache temperature-key truncation before the raw polynomial";
const IDEAL_LOADS_OUTPUT_HANDLE_REGISTRATION_POLICY: &str = "manifest output requests are resolved to stable OutputHandle values before IdealLoads comparison rows are evaluated";
const IDEAL_LOADS_OUTPUT_HANDLE_WRITE_POLICY: &str = "rate and node ResultStore series use pre-resolved OutputHandle values; meter rows use RuntimeMeterRegistry-resolved handles before aggregation";
const IDEAL_LOADS_DIAGNOSTIC_OUTPUT_REQUEST_POLICY: &str = "diagnostic rows are emitted only for manifest-declared diagnostic outputs or meters and are separated from conformance rows";
const IDEAL_LOADS_REPORT_EXPORT_ORDER_POLICY: &str = "compare artifacts are exported after IdealLoads calculations populate comparison rows, meter rows, and ResultStore";
const IDEAL_LOADS_DETAILED_OUTPUT_LOOKUP_POLICY: &str = "Detailed output key/variable lookup is confined to post-calculation report assembly; simulation calculations use typed IDs and pre-resolved handles";
const IDEAL_LOADS_DUPLICATE_OUTPUT_HANDLE_POLICY: &str = "duplicate manifest output requests fail during handle setup; duplicate ResultStore handles and identities fail ep_runtime::ResultStore::diagnostics";
const IDEAL_LOADS_TRACE_LEVEL_DEFAULT: &str = "default-conformance";
const IDEAL_LOADS_TRACE_LEVEL_SOURCE_DEFAULT: &str =
    "built-in default; override with case manifest [trace].level";
const IDEAL_LOADS_TRACE_LEVEL_SOURCE_MANIFEST: &str = "case manifest [trace].level";
const IDEAL_LOADS_TRACE_SIDE_EFFECT_POLICY: &str =
    "trace/report serialization only; calculations are complete before artifact rendering";
const IDEAL_LOADS_TRACE_RESULT_INVARIANCE_POLICY: &str = "trace level selects evidence payload only; ResultStore values are computed before report serialization";
const IDEAL_LOADS_TRACE_OVERHEAD_ACCOUNTING: &str = "trace/report serialization overhead is outside numerical conformance comparison and measured separately from simulation results";
const IDEAL_LOADS_NO_OA_TRACE_PAYLOAD: &str =
    "mode_counts, source-order demand inputs, selected branch, supply state, and report rates";
const IDEAL_LOADS_OUTDOOR_AIR_TRACE_PAYLOAD: &str = "source-order zone/recirculation/outdoor-air states, raw schedule/occupancy/CO2 minimum-flow inputs, resolved design/DCV minimum outdoor-air mass flow, mixed-air state, supply state, and report rates";

struct IdealLoadsDiagnosticContext<'a> {
    manifest: &'a ConformanceCase,
    baseline: &'a BaselineSummary,
    branch: &'static str,
    selected_purchased_air_branch: &'static str,
    declared_ideal_loads_branch: &'static str,
    inactive_branches: Vec<&'static str>,
    feature_flags: IdealLoadsFeatureFlags,
    zone_equipment_dispatch: IdealLoadsZoneEquipmentDispatchValidation,
    constant_shr_conformance_claim: bool,
    constant_supply_humidity_cooling_conformance_claim: bool,
    constant_supply_humidity_heating_conformance_claim: bool,
    humidistat_dehumidification_conformance_claim: bool,
    humidistat_humidification_conformance_claim: bool,
    humidity_annual_facility_meter_conformance_claim: bool,
    zone_name: String,
    zone_air_node_name: String,
    recirculation_node_name: Option<String>,
    system_name: String,
    supply_node_name: String,
    timestep: IdealLoadsTimestepContext,
    fuel_efficiency: IdealLoadsFuelEfficiencyContext,
    rows: Vec<IdealLoadsDiagnosticRow>,
    meter_rows: Vec<IdealLoadsMeterDiagnosticRow>,
    result_store: ResultStore,
    input_trace: IdealLoadsInputTrace,
    mode_counts: IdealLoadsModeCounts,
    moisture_predictor: Option<IdealLoadsMoisturePredictorSummary>,
}

#[derive(Clone)]
struct IdealLoadsMoisturePredictorSummary {
    promoted_input: bool,
    history_source: &'static str,
    latent_gain_source: &'static str,
    closed_loop_state_source: &'static str,
    history_residual_source: &'static str,
    humidifying_equivalent_history_delta_max: f64,
    dehumidifying_equivalent_history_delta_max: f64,
    humidifying_history_term: IdealLoadsMoistureHistoryTermComparison,
    dehumidifying_history_term: IdealLoadsMoistureHistoryTermComparison,
    zone_moisture_capacity_multiplier: f64,
    zone_multiplier: f64,
    closed_loop_humidifying_values: Vec<f64>,
    closed_loop_dehumidifying_values: Vec<f64>,
    closed_loop_results: Vec<IdealLoadsReportSnapshot>,
    latent_gain: Vec<IdealLoadsMoisturePredictorComparison>,
    humidifying: IdealLoadsMoisturePredictorComparison,
    dehumidifying: IdealLoadsMoisturePredictorComparison,
    closed_loop: Vec<IdealLoadsMoisturePredictorComparison>,
}

#[derive(Clone)]
struct IdealLoadsHumidistatClosedLoopSummary {
    comparisons: Vec<IdealLoadsMoisturePredictorComparison>,
    humidifying_values: Vec<f64>,
    dehumidifying_values: Vec<f64>,
    results: Vec<IdealLoadsReportSnapshot>,
}

#[derive(Clone)]
struct IdealLoadsMoisturePredictorComparison {
    variable: String,
    samples: usize,
    max_abs_delta: f64,
    rmse_delta: f64,
    max_rel_delta: f64,
    status: SeriesComparisonStatus,
    first_divergence: Option<ep_compare::SeriesDivergenceV2>,
}

#[derive(Clone)]
struct IdealLoadsMoistureHistoryTermComparison {
    demand: String,
    samples: usize,
    row_lag_minus_inferred_max_abs_delta: f64,
    row_lag_minus_inferred_rmse_delta: f64,
    row_lag_minus_inferred_mean_delta: f64,
    max_abs_row_lag_history_term: f64,
    max_abs_inferred_history_term: f64,
    largest_delta_sample: Option<IdealLoadsMoistureHistoryTermSample>,
}

#[derive(Clone)]
struct IdealLoadsMoistureHistoryTermSample {
    index: usize,
    timestamp: Option<String>,
    row_lag_history_term: f64,
    inferred_history_term: f64,
    row_lag_minus_inferred_delta: f64,
}

#[derive(Clone)]
struct IdealLoadsFuelEfficiencyContext {
    heating: f64,
    cooling: f64,
    heating_values: Vec<f64>,
    cooling_values: Vec<f64>,
    report_source: &'static str,
    rate_rust_source: &'static str,
    energy_rust_source: &'static str,
}

impl IdealLoadsFuelEfficiencyContext {
    fn blank(sample_count: usize) -> Self {
        Self {
            heating: 1.0,
            cooling: 1.0,
            heating_values: vec![1.0; sample_count],
            cooling_values: vec![1.0; sample_count],
            report_source: IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_REPORT_SOURCE,
            rate_rust_source: IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_RATE_SOURCE,
            energy_rust_source: IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_ENERGY_SOURCE,
        }
    }

    fn constant(heating: f64, cooling: f64, sample_count: usize) -> Self {
        Self {
            heating,
            cooling,
            heating_values: vec![heating; sample_count],
            cooling_values: vec![cooling; sample_count],
            report_source: IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_REPORT_SOURCE,
            rate_rust_source: IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_RATE_SOURCE,
            energy_rust_source: IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_ENERGY_SOURCE,
        }
    }

    fn non_constant(heating_values: Vec<f64>, cooling_values: Vec<f64>) -> Self {
        Self {
            heating: heating_values.first().copied().unwrap_or(1.0),
            cooling: cooling_values.first().copied().unwrap_or(1.0),
            heating_values,
            cooling_values,
            report_source: IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_REPORT_SOURCE,
            rate_rust_source: IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_RATE_SOURCE,
            energy_rust_source: IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_ENERGY_SOURCE,
        }
    }

    fn heating_at(&self, index: usize) -> f64 {
        self.heating_values
            .get(index)
            .copied()
            .unwrap_or(self.heating)
    }

    fn cooling_at(&self, index: usize) -> f64 {
        self.cooling_values
            .get(index)
            .copied()
            .unwrap_or(self.cooling)
    }
}

struct IdealLoadsOutdoorAirDiagnosticContext<'a> {
    manifest: &'a ConformanceCase,
    baseline: &'a BaselineSummary,
    branch: &'static str,
    feature_flags: IdealLoadsFeatureFlags,
    zone_equipment_dispatch: IdealLoadsZoneEquipmentDispatchValidation,
    zone_name: String,
    system_name: String,
    outdoor_air_spec_name: String,
    outdoor_air_method: DesignSpecificationOutdoorAirMethod,
    outdoor_air_node_name: String,
    recirculation_node_name: Option<String>,
    demand_controlled_ventilation_type: DemandControlledVentilationType,
    outdoor_air_economizer_type: OutdoorAirEconomizerType,
    heat_recovery_type: HeatRecoveryType,
    standard_air_density_kg_per_m3: f64,
    design_people_count: f64,
    current_people_count_min: f64,
    current_people_count_max: f64,
    co2_setpoint_required_mass_flow_rate_min_kg_per_s: f64,
    co2_setpoint_required_mass_flow_rate_max_kg_per_s: f64,
    zone_floor_area_m2: f64,
    zone_volume_m3: f64,
    flow_per_person_m3_per_s: f64,
    flow_per_area_m3_per_s: f64,
    flow_per_zone_m3_per_s: f64,
    air_changes_m3_per_s: f64,
    design_volume_flow_rate_m3_per_s: f64,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    outdoor_air_mass_flow_rate_min_kg_per_s: f64,
    outdoor_air_mass_flow_rate_max_kg_per_s: f64,
    timestep: IdealLoadsTimestepContext,
    sample_count: usize,
    rows: Vec<IdealLoadsDiagnosticRow>,
    result_store: ResultStore,
}

struct IdealLoadsInputTrace {
    sample_count: usize,
    zone_air_temperature: LoadedSeries,
    zone_air_temperature_warmup_tail: Option<[f64; 3]>,
    zone_node_temperature: LoadedSeries,
    zone_air_humidity_ratio: LoadedSeries,
    zone_air_humidity_ratio_warmup_tail: Option<[f64; 3]>,
    zone_mean_air_humidity_ratio: LoadedSeries,
    zone_mean_air_humidity_ratio_warmup_tail: Option<[f64; 3]>,
    zone_node_humidity_ratio: LoadedSeries,
    site_barometric_pressure: Option<LoadedSeries>,
    recirculation_node_temperature: LoadedSeries,
    recirculation_node_humidity_ratio: LoadedSeries,
    active_demand: LoadedSeries,
    heating_demand: LoadedSeries,
    cooling_demand: LoadedSeries,
    humidifying_moisture_demand: LoadedSeries,
    dehumidifying_moisture_demand: LoadedSeries,
}

#[derive(Clone)]
struct LoadedSeries {
    units: Option<String>,
    samples: Vec<SeriesSample>,
}

struct IdealLoadsDiagnosticRow {
    handle: OutputHandle,
    key: String,
    variable: String,
    frequency: OutputFrequency,
    variable_class: VariableClass,
    source: SourceArtifact,
    domain: Option<EvidenceDomain>,
    level: Option<OutputLevel>,
    units: String,
    oracle_units: Option<String>,
    rust_source: &'static str,
    tolerance: Tolerance,
    max_rmse_tolerance: Option<f64>,
    expected_samples: usize,
    observed_samples: usize,
    compared_samples: usize,
    max_abs_delta: f64,
    mean_abs_delta: f64,
    rmse_delta: f64,
    max_rel_delta: f64,
    alignment: SeriesAlignment,
    first_divergence: Option<ep_compare::SeriesDivergenceV2>,
    status: SeriesComparisonStatus,
}

type IdealLoadsOutputHandleMap = BTreeMap<(String, String, OutputFrequency), OutputHandle>;

struct IdealLoadsMeterDiagnosticRow {
    name: String,
    frequency: OutputFrequency,
    source: SourceArtifact,
    domain: EvidenceDomain,
    level: OutputLevel,
    units: String,
    oracle_units: Option<String>,
    rust_source: &'static str,
    tolerance: Tolerance,
    max_rmse_tolerance: Option<f64>,
    expected_samples: usize,
    observed_samples: usize,
    compared_samples: usize,
    max_abs_delta: f64,
    mean_abs_delta: f64,
    rmse_delta: f64,
    max_rel_delta: f64,
    alignment: SeriesAlignment,
    first_divergence: Option<ep_compare::SeriesDivergenceV2>,
    status: SeriesComparisonStatus,
}

#[derive(Clone, Copy, Debug, Default)]
struct IdealLoadsModeCounts {
    off: usize,
    deadband: usize,
    cooling: usize,
    heating: usize,
}

fn validate_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    if !matches!(
        manifest.comparison_class,
        ComparisonClass::DiagnosticOnly | ComparisonClass::Conformance
    ) {
        return Err(format!(
            "IdealLoads no-OA report requires diagnostic-only or conformance, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if manifest.comparison_class == ComparisonClass::DiagnosticOnly && manifest.conformance_claim {
        return Err(
            "diagnostic-only IdealLoads report must keep conformance_claim false".to_string(),
        );
    }
    if manifest.conformance_claim
        && !manifest_has_conformance_output(manifest)
        && !manifest_has_conformance_meter(manifest)
    {
        return Err(
            "conformance IdealLoads report requires at least one conformance-level output or meter"
                .to_string(),
        );
    }
    if manifest.outputs.is_empty() {
        return Err("IdealLoads no-OA report requires output requests".to_string());
    }
    for output in &manifest.outputs {
        if output.frequency != OutputFrequency::Detailed {
            return Err(format!(
                "IdealLoads no-OA report requires detailed outputs, got {} for {}",
                output_frequency_label(output.frequency),
                output.variable
            ));
        }
        if output.source != SourceArtifact::Eso {
            return Err(format!(
                "IdealLoads no-OA report requires ESO outputs, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
        if output.level == Some(OutputLevel::Conformance)
            && ideal_loads_fuel_energy_variable(&output.variable)
            && !manifest_is_blank_fuel_efficiency_conformance_candidate(manifest)
            && !manifest_is_constant_fuel_efficiency_conformance_candidate(manifest)
            && !manifest_is_non_constant_fuel_efficiency_conformance_candidate(manifest)
            && !manifest_is_humidity_report_purchased_air_conformance_candidate(manifest)
        {
            return Err(format!(
                "IdealLoads fuel-energy outputs remain diagnostic until fuel-efficiency path conformance is separately proven: {}",
                output.variable
            ));
        }
        if output.level == Some(OutputLevel::Conformance)
            && ideal_loads_report_energy_variable(&output.variable)
            && !manifest_is_no_oa_report_energy_conformance_candidate(manifest)
            && !manifest_is_humidity_report_purchased_air_conformance_candidate(manifest)
        {
            return Err(format!(
                "IdealLoads ReportPurchasedAir energy outputs can be conformance-level only in the declared report-energy candidate: {}",
                output.variable
            ));
        }
    }
    if manifest_is_declared_no_oa_facility_meter_conformance_candidate(manifest) {
        if manifest_has_conformance_output(manifest) {
            return Err(
                "IdealLoads facility meter conformance candidate must keep ESO output rows diagnostic"
                    .to_string(),
            );
        }
        for expected_meter in IDEAL_LOADS_NO_OA_FACILITY_METER_CONFORMANCE_METERS {
            for expected_frequency in required_facility_meter_conformance_frequencies(manifest) {
                if !manifest.meters.iter().any(|meter| {
                    meter.name.eq_ignore_ascii_case(expected_meter)
                        && meter.frequency == *expected_frequency
                        && meter.level == OutputLevel::Conformance
                }) {
                    return Err(format!(
                        "IdealLoads facility meter conformance candidate is missing {} conformance meter {expected_meter}",
                        output_frequency_label(*expected_frequency)
                    ));
                }
            }
        }
    }
    if manifest_is_no_oa_report_energy_conformance_candidate(manifest) {
        for output in manifest
            .outputs
            .iter()
            .filter(|output| output.level == Some(OutputLevel::Conformance))
        {
            if !is_declared_no_oa_report_energy_conformance_output(&output.variable) {
                return Err(format!(
                    "IdealLoads report-energy conformance candidate supports only declared non-fuel energy rows, got {}",
                    output.variable
                ));
            }
        }
        for expected_output in IDEAL_LOADS_NO_OA_REPORT_ENERGY_CONFORMANCE_OUTPUTS {
            if !manifest.outputs.iter().any(|output| {
                output.variable.eq_ignore_ascii_case(expected_output)
                    && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads report-energy conformance candidate is missing conformance output {expected_output}"
                ));
            }
        }
    }
    if manifest_is_humidity_report_purchased_air_conformance_candidate(manifest) {
        for expected_output in IDEAL_LOADS_HUMIDITY_RATE_CONFORMANCE_OUTPUTS {
            if !manifest.outputs.iter().any(|output| {
                output.variable.eq_ignore_ascii_case(expected_output)
                    && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads humidity-control rate conformance candidate is missing conformance output {expected_output}"
                ));
            }
        }
        for expected_output in IDEAL_LOADS_NO_OA_REPORT_ENERGY_CONFORMANCE_OUTPUTS {
            if !manifest.outputs.iter().any(|output| {
                output.variable.eq_ignore_ascii_case(expected_output)
                    && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads humidity-control ReportPurchasedAir conformance candidate is missing conformance output {expected_output}"
                ));
            }
        }
        for expected_output in IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS {
            if !manifest.outputs.iter().any(|output| {
                output.variable.eq_ignore_ascii_case(expected_output)
                    && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads humidity-control blank fuel-efficiency conformance candidate is missing conformance output {expected_output}"
                ));
            }
        }
    }
    if manifest_is_humidity_report_purchased_air_conformance_candidate(manifest) {
        for expected_meter in IDEAL_LOADS_NO_OA_FACILITY_METER_CONFORMANCE_METERS {
            for expected_frequency in IDEAL_LOADS_HUMIDITY_FACILITY_METER_CONFORMANCE_FREQUENCIES {
                if !manifest.meters.iter().any(|meter| {
                    meter.name.eq_ignore_ascii_case(expected_meter)
                        && meter.frequency == *expected_frequency
                        && meter.level == OutputLevel::Conformance
                }) {
                    return Err(format!(
                        "IdealLoads humidity-control conformance candidate is missing {} conformance meter {expected_meter}",
                        output_frequency_label(*expected_frequency)
                    ));
                }
            }
        }
    }
    if manifest_is_humidity_annual_facility_meter_conformance_candidate(manifest) {
        if manifest_has_conformance_output(manifest) {
            return Err(
                "IdealLoads humidity-control annual facility meter candidate must keep ESO output rows diagnostic"
                    .to_string(),
            );
        }
        for expected_meter in IDEAL_LOADS_NO_OA_FACILITY_METER_CONFORMANCE_METERS {
            if !manifest.meters.iter().any(|meter| {
                meter.name.eq_ignore_ascii_case(expected_meter)
                    && meter.frequency == OutputFrequency::Annual
                    && meter.level == OutputLevel::Conformance
            }) {
                return Err(format!(
                    "IdealLoads humidity-control annual facility meter candidate is missing annual conformance meter {expected_meter}"
                ));
            }
        }
    }
    if manifest_is_non_constant_fuel_efficiency_conformance_candidate(manifest) {
        for output in manifest
            .outputs
            .iter()
            .filter(|output| output.level == Some(OutputLevel::Conformance))
        {
            if !is_declared_non_constant_fuel_efficiency_conformance_output(&output.variable) {
                return Err(format!(
                    "IdealLoads non-constant fuel-efficiency conformance candidate supports only declared fuel-energy rows, got {}",
                    output.variable
                ));
            }
        }
        for expected_output in IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS {
            if !manifest.outputs.iter().any(|output| {
                output.variable.eq_ignore_ascii_case(expected_output)
                    && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads non-constant fuel-efficiency conformance candidate is missing conformance output {expected_output}"
                ));
            }
        }
    }
    if manifest_is_blank_fuel_efficiency_conformance_candidate(manifest) {
        for output in manifest
            .outputs
            .iter()
            .filter(|output| output.level == Some(OutputLevel::Conformance))
        {
            if !is_declared_blank_fuel_efficiency_conformance_output(&output.variable) {
                return Err(format!(
                    "IdealLoads blank fuel-efficiency conformance candidate supports only declared fuel-energy rows, got {}",
                    output.variable
                ));
            }
        }
        for expected_output in IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS {
            if !manifest.outputs.iter().any(|output| {
                output.variable.eq_ignore_ascii_case(expected_output)
                    && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads blank fuel-efficiency conformance candidate is missing conformance output {expected_output}"
                ));
            }
        }
    }
    if manifest_is_constant_fuel_efficiency_conformance_candidate(manifest) {
        for output in manifest
            .outputs
            .iter()
            .filter(|output| output.level == Some(OutputLevel::Conformance))
        {
            if !is_declared_constant_fuel_efficiency_conformance_output(&output.variable) {
                return Err(format!(
                    "IdealLoads constant fuel-efficiency conformance candidate supports only declared fuel-energy rows, got {}",
                    output.variable
                ));
            }
        }
        for expected_output in IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS {
            if !manifest.outputs.iter().any(|output| {
                output.variable.eq_ignore_ascii_case(expected_output)
                    && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads constant fuel-efficiency conformance candidate is missing conformance output {expected_output}"
                ));
            }
        }
    }

    for meter in &manifest.meters {
        if !is_supported_ideal_loads_meter_frequency(meter.frequency) {
            return Err(format!(
                "IdealLoads no-OA report supports hourly, monthly, annual, and run-period meter outputs, got {} for {}",
                output_frequency_label(meter.frequency),
                meter.name
            ));
        }
        if meter.source != SourceArtifact::Mtr {
            return Err(format!(
                "IdealLoads no-OA report requires MTR meter outputs, got {} for {}",
                source_artifact_label(meter.source),
                meter.name
            ));
        }
        if meter.level == OutputLevel::Diagnostic {
            continue;
        }
        let manifest_allows_meter_conformance =
            manifest_is_declared_no_oa_facility_meter_conformance_candidate(manifest)
                || manifest_is_humidity_report_purchased_air_conformance_candidate(manifest)
                || manifest_is_humidity_annual_facility_meter_conformance_candidate(manifest);
        if !manifest_allows_meter_conformance
            || !is_declared_no_oa_facility_meter_conformance_meter(&meter.name)
            || !facility_meter_frequency_allowed_for_manifest(manifest, meter.frequency)
            || meter.level != OutputLevel::Conformance
        {
            return Err(format!(
                "IdealLoads no-OA report supports conformance-level meters only for declared facility meter candidates: {} ({})",
                meter.name,
                output_frequency_label(meter.frequency)
            ));
        }
    }
    Ok(())
}

fn validate_outdoor_air_design_flow_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    let conformance_method = outdoor_air_conformance_method_for_manifest(manifest);
    let outdoor_air_conformance = conformance_method.is_some();
    if outdoor_air_conformance {
        if manifest.comparison_class != ComparisonClass::Conformance {
            return Err(format!(
                "IdealLoads outdoor-air conformance requires comparison_class=conformance, got {}",
                comparison_class_label(manifest.comparison_class)
            ));
        }
        for variable in OUTDOOR_AIR_CONFORMANCE_VARIABLES {
            if !manifest.outputs.iter().any(|output| {
                output.variable == *variable && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads outdoor-air conformance is missing conformance row for {variable}"
                ));
            }
        }
        for variable in OUTDOOR_AIR_HEAT_RECOVERY_RATE_CONFORMANCE_VARIABLES {
            if !manifest.outputs.iter().any(|output| {
                output.variable == *variable && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads outdoor-air conformance is missing heat-recovery rate conformance row for {variable}"
                ));
            }
        }
        for variable in OUTDOOR_AIR_ACTIVE_TIME_CONFORMANCE_VARIABLES {
            if !manifest.outputs.iter().any(|output| {
                output.variable == *variable && output.level == Some(OutputLevel::Conformance)
            }) {
                return Err(format!(
                    "IdealLoads outdoor-air conformance is missing active-time conformance row for {variable}"
                ));
            }
        }
    } else if manifest.comparison_class != ComparisonClass::DiagnosticOnly {
        return Err(format!(
            "IdealLoads outdoor-air design-flow report requires diagnostic-only unless it is an approved outdoor-air conformance candidate, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if !outdoor_air_conformance && manifest.conformance_claim {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic must keep conformance_claim false"
                .to_string(),
        );
    }
    if manifest.outputs.is_empty() {
        return Err("IdealLoads outdoor-air design-flow report requires outputs".to_string());
    }
    for output in &manifest.outputs {
        if output.frequency != OutputFrequency::Detailed {
            return Err(format!(
                "IdealLoads outdoor-air design-flow report requires detailed outputs, got {} for {}",
                output_frequency_label(output.frequency),
                output.variable
            ));
        }
        if output.source != SourceArtifact::Eso {
            return Err(format!(
                "IdealLoads outdoor-air design-flow report requires ESO outputs, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
        if outdoor_air_conformance {
            let expected_level = if outdoor_air_conformance_variable_for_manifest(
                manifest,
                output.variable.as_str(),
            ) {
                OutputLevel::Conformance
            } else {
                OutputLevel::Diagnostic
            };
            if output.level != Some(expected_level) {
                return Err(format!(
                    "IdealLoads outdoor-air conformance expects {} level for {}",
                    output_level_label(expected_level),
                    output.variable
                ));
            }
        } else if output.level != Some(OutputLevel::Diagnostic) {
            return Err(format!(
                "IdealLoads outdoor-air design-flow outputs must be diagnostic-level: {}",
                output.variable
            ));
        }
        if !matches!(
            output.variable.as_str(),
            ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE
                | ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE
                | ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE
                | ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE
                | ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE
                | ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO
                | ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE
                | ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_HEATING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_HEATING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_HEATING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_COOLING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_COOLING_RATE
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_COOLING_RATE
                | ZONE_IDEAL_LOADS_ECONOMIZER_ACTIVE_TIME
                | ZONE_IDEAL_LOADS_HEAT_RECOVERY_ACTIVE_TIME
        ) {
            return Err(format!(
                "IdealLoads outdoor-air design-flow report cannot produce Rust series for {}",
                output.variable
            ));
        }
    }
    Ok(())
}

const OUTDOOR_AIR_CONFORMANCE_VARIABLES: &[&str] = &[
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE,
    ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
    ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE,
    ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO,
];

const OUTDOOR_AIR_HEAT_RECOVERY_RATE_CONFORMANCE_VARIABLES: &[&str] = &[
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_HEATING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_HEATING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_COOLING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_COOLING_RATE,
];

const OUTDOOR_AIR_ACTIVE_TIME_CONFORMANCE_VARIABLES: &[&str] = &[
    ZONE_IDEAL_LOADS_ECONOMIZER_ACTIVE_TIME,
    ZONE_IDEAL_LOADS_HEAT_RECOVERY_ACTIVE_TIME,
];

fn manifest_allows_outdoor_air_flow_zone_conformance_manifest(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_FLOW_ZONE_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_flow_person_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_FLOW_PERSON_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_flow_area_conformance_manifest(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_FLOW_AREA_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_air_changes_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_AIR_CHANGES_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_sum_conformance_manifest(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_SUM_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_maximum_conformance_manifest(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_MAXIMUM_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_differential_dry_bulb_economizer_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_DIFFERENTIAL_DRY_BULB_ECONOMIZER_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_differential_enthalpy_economizer_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_DIFFERENTIAL_ENTHALPY_ECONOMIZER_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_sensible_heat_recovery_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEAT_RECOVERY_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_enthalpy_heat_recovery_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_ENTHALPY_HEAT_RECOVERY_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_occupancy_dcv_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_OCCUPANCY_DCV_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_co2_dcv_conformance_manifest(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_OUTDOOR_AIR_CO2_DCV_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_allows_outdoor_air_active_heat_recovery_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    manifest_allows_outdoor_air_sensible_heat_recovery_conformance_manifest(manifest)
        || manifest_allows_outdoor_air_enthalpy_heat_recovery_conformance_manifest(manifest)
}

fn manifest_is_no_oa_facility_meter_conformance_candidate(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_NO_OA_FACILITY_METER_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_is_no_oa_facility_meter_monthly_run_period_conformance_candidate(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_NO_OA_FACILITY_METER_MONTHLY_RUN_PERIOD_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_is_declared_no_oa_facility_meter_conformance_candidate(
    manifest: &ConformanceCase,
) -> bool {
    manifest_is_no_oa_facility_meter_conformance_candidate(manifest)
        || manifest_is_no_oa_facility_meter_monthly_run_period_conformance_candidate(manifest)
}

fn manifest_is_no_oa_report_energy_conformance_candidate(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_NO_OA_REPORT_ENERGY_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_is_humidity_report_purchased_air_conformance_candidate(
    manifest: &ConformanceCase,
) -> bool {
    IDEAL_LOADS_HUMIDITY_REPORT_PURCHASED_AIR_CONFORMANCE_CASE_IDS
        .iter()
        .any(|case_id| manifest.id == *case_id)
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_is_humidity_annual_facility_meter_conformance_candidate(
    manifest: &ConformanceCase,
) -> bool {
    IDEAL_LOADS_HUMIDITY_ANNUAL_FACILITY_METER_CONFORMANCE_CASE_IDS
        .iter()
        .any(|case_id| manifest.id == *case_id)
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_is_blank_fuel_efficiency_conformance_candidate(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_is_non_constant_fuel_efficiency_conformance_candidate(
    manifest: &ConformanceCase,
) -> bool {
    manifest.id == IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_is_constant_fuel_efficiency_conformance_candidate(manifest: &ConformanceCase) -> bool {
    manifest.id == IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_CASE_ID
        && manifest.comparison_class == ComparisonClass::Conformance
        && manifest.conformance_claim
}

fn manifest_has_conformance_output(manifest: &ConformanceCase) -> bool {
    manifest
        .outputs
        .iter()
        .any(|output| output.level == Some(OutputLevel::Conformance))
}

fn manifest_has_conformance_meter(manifest: &ConformanceCase) -> bool {
    manifest
        .meters
        .iter()
        .any(|meter| meter.level == OutputLevel::Conformance)
}

fn is_declared_no_oa_facility_meter_conformance_meter(name: &str) -> bool {
    IDEAL_LOADS_NO_OA_FACILITY_METER_CONFORMANCE_METERS
        .iter()
        .any(|expected| name.eq_ignore_ascii_case(expected))
}

fn is_supported_ideal_loads_meter_frequency(frequency: OutputFrequency) -> bool {
    matches!(
        frequency,
        OutputFrequency::Hourly
            | OutputFrequency::Monthly
            | OutputFrequency::Annual
            | OutputFrequency::RunPeriod
    )
}

fn required_facility_meter_conformance_frequencies(
    manifest: &ConformanceCase,
) -> &'static [OutputFrequency] {
    if manifest_is_no_oa_facility_meter_monthly_run_period_conformance_candidate(manifest) {
        &[
            OutputFrequency::Monthly,
            OutputFrequency::Annual,
            OutputFrequency::RunPeriod,
        ]
    } else if manifest_is_humidity_annual_facility_meter_conformance_candidate(manifest) {
        &[OutputFrequency::Annual]
    } else if manifest_is_humidity_report_purchased_air_conformance_candidate(manifest) {
        IDEAL_LOADS_HUMIDITY_FACILITY_METER_CONFORMANCE_FREQUENCIES
    } else {
        &[OutputFrequency::Hourly]
    }
}

fn facility_meter_frequency_allowed_for_manifest(
    manifest: &ConformanceCase,
    frequency: OutputFrequency,
) -> bool {
    required_facility_meter_conformance_frequencies(manifest).contains(&frequency)
}

fn is_declared_no_oa_report_energy_conformance_output(variable: &str) -> bool {
    IDEAL_LOADS_NO_OA_REPORT_ENERGY_CONFORMANCE_OUTPUTS
        .iter()
        .any(|expected| variable.eq_ignore_ascii_case(expected))
}

fn is_declared_blank_fuel_efficiency_conformance_output(variable: &str) -> bool {
    IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS
        .iter()
        .any(|expected| variable.eq_ignore_ascii_case(expected))
}

fn is_declared_non_constant_fuel_efficiency_conformance_output(variable: &str) -> bool {
    IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS
        .iter()
        .any(|expected| variable.eq_ignore_ascii_case(expected))
}

fn is_declared_constant_fuel_efficiency_conformance_output(variable: &str) -> bool {
    IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_OUTPUTS
        .iter()
        .any(|expected| variable.eq_ignore_ascii_case(expected))
}

fn outdoor_air_conformance_expectations_for_manifest(
    manifest: &ConformanceCase,
) -> Option<(
    DesignSpecificationOutdoorAirMethod,
    OutdoorAirEconomizerType,
    HeatRecoveryType,
    DemandControlledVentilationType,
)> {
    if manifest_allows_outdoor_air_flow_zone_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::FlowPerZone,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::None,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_flow_person_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::FlowPerPerson,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::None,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_flow_area_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::FlowPerArea,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::None,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_air_changes_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::AirChangesPerHour,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::None,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_sum_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::Sum,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::None,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_maximum_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::Maximum,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::None,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_differential_dry_bulb_economizer_conformance_manifest(
        manifest,
    ) {
        Some((
            DesignSpecificationOutdoorAirMethod::FlowPerZone,
            OutdoorAirEconomizerType::DifferentialDryBulb,
            HeatRecoveryType::None,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_differential_enthalpy_economizer_conformance_manifest(
        manifest,
    ) {
        Some((
            DesignSpecificationOutdoorAirMethod::FlowPerZone,
            OutdoorAirEconomizerType::DifferentialEnthalpy,
            HeatRecoveryType::None,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_sensible_heat_recovery_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::FlowPerZone,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::Sensible,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_enthalpy_heat_recovery_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::FlowPerZone,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::Enthalpy,
            DemandControlledVentilationType::None,
        ))
    } else if manifest_allows_outdoor_air_occupancy_dcv_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::FlowPerPerson,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::None,
            DemandControlledVentilationType::OccupancySchedule,
        ))
    } else if manifest_allows_outdoor_air_co2_dcv_conformance_manifest(manifest) {
        Some((
            DesignSpecificationOutdoorAirMethod::FlowPerPerson,
            OutdoorAirEconomizerType::NoEconomizer,
            HeatRecoveryType::None,
            DemandControlledVentilationType::Co2Setpoint,
        ))
    } else {
        None
    }
}

fn outdoor_air_conformance_method_for_manifest(
    manifest: &ConformanceCase,
) -> Option<DesignSpecificationOutdoorAirMethod> {
    outdoor_air_conformance_expectations_for_manifest(manifest)
        .map(|(method, _economizer, _heat_recovery, _dcv)| method)
}

fn manifest_allows_outdoor_air_inactive_heat_recovery_rate_conformance_manifest(
    manifest: &ConformanceCase,
) -> bool {
    matches!(
        outdoor_air_conformance_expectations_for_manifest(manifest),
        Some((_method, _economizer, HeatRecoveryType::None, _dcv))
    )
}

fn outdoor_air_conformance_variable_for_manifest(
    manifest: &ConformanceCase,
    variable: &str,
) -> bool {
    OUTDOOR_AIR_CONFORMANCE_VARIABLES.contains(&variable)
        || (outdoor_air_conformance_expectations_for_manifest(manifest).is_some()
            && OUTDOOR_AIR_ACTIVE_TIME_CONFORMANCE_VARIABLES.contains(&variable))
        || (manifest_allows_outdoor_air_active_heat_recovery_conformance_manifest(manifest)
            && OUTDOOR_AIR_HEAT_RECOVERY_RATE_CONFORMANCE_VARIABLES.contains(&variable))
        || (manifest_allows_outdoor_air_inactive_heat_recovery_rate_conformance_manifest(manifest)
            && OUTDOOR_AIR_HEAT_RECOVERY_RATE_CONFORMANCE_VARIABLES.contains(&variable))
}

fn build_outdoor_air_design_flow_context<'a>(
    manifest: &'a ConformanceCase,
    baseline: &'a BaselineSummary,
) -> Result<IdealLoadsOutdoorAirDiagnosticContext<'a>, String> {
    let raw_model = baseline.load_raw_model()?;
    let compile_result = compile_raw_model(&raw_model);
    let typed = compile_result.model.ok_or_else(|| {
        compile_result
            .report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let model = SimulationModel::from_typed(typed);
    let timestep = ideal_loads_timestep_context(&model.typed)?;
    if model.typed.zones.len() != 1 {
        return Err(format!(
            "IdealLoads outdoor-air design-flow report requires one zone, got {}",
            model.typed.zones.len()
        ));
    }
    if model.typed.ideal_loads_air_systems.len() != 1 {
        return Err(format!(
            "IdealLoads outdoor-air design-flow report requires one IdealLoads system, got {}",
            model.typed.ideal_loads_air_systems.len()
        ));
    }

    let edge = model
        .graph
        .zone_ideal_loads
        .first()
        .ok_or_else(|| "missing zone to IdealLoads graph edge".to_string())?;
    let zone = model
        .typed
        .zones
        .iter()
        .find(|zone| zone.id == edge.zone)
        .ok_or_else(|| "missing controlled zone for IdealLoads edge".to_string())?;
    let zone_air_node_edge = model
        .graph
        .zone_air_nodes
        .iter()
        .find(|candidate| candidate.zone == zone.id)
        .ok_or_else(|| "missing zone air-node edge".to_string())?;
    let zone_air_node = model
        .typed
        .nodes
        .iter()
        .find(|node| node.id == zone_air_node_edge.node)
        .ok_or_else(|| "missing zone air node".to_string())?;
    let system = model
        .typed
        .ideal_loads_air_systems
        .iter()
        .find(|system| system.id == edge.ideal_loads_air_system)
        .ok_or_else(|| "missing IdealLoads system for graph edge".to_string())?;
    let supply_edge = model
        .graph
        .ideal_loads_supply_nodes
        .iter()
        .find(|candidate| candidate.ideal_loads_air_system == system.id)
        .ok_or_else(|| "missing IdealLoads supply-node edge".to_string())?;
    let supply_node = model
        .typed
        .nodes
        .iter()
        .find(|node| node.id == supply_edge.node)
        .ok_or_else(|| "missing IdealLoads supply node".to_string())?;
    let zone_equipment_dispatch = validate_ideal_loads_zone_equipment_dispatch(&model, system.id);
    if !zone_equipment_dispatch.is_dispatchable() {
        return Err(format!(
            "IdealLoads outdoor-air zone equipment dispatch prerequisites failed: {}",
            label_list_or_none(&zone_equipment_dispatch.issue_codes())
        ));
    }
    if manifest.conformance_claim && !zone_equipment_dispatch.is_conformance_candidate() {
        return Err(format!(
            "IdealLoads outdoor-air conformance candidate requires single-zone/single-equipment dispatch scope: {}",
            label_list_or_none(&zone_equipment_dispatch.warning_codes())
        ));
    }
    let outdoor_air_edge = model
        .graph
        .ideal_loads_outdoor_air_specs
        .iter()
        .find(|candidate| candidate.ideal_loads_air_system == system.id)
        .ok_or_else(|| "missing IdealLoads outdoor-air design specification edge".to_string())?;
    let outdoor_air_specification = model
        .typed
        .design_specification_outdoor_air
        .iter()
        .find(|specification| specification.id == outdoor_air_edge.design_specification_outdoor_air)
        .ok_or_else(|| "missing IdealLoads outdoor-air design specification".to_string())?;
    let outdoor_air_node_name = system
        .outdoor_air_inlet_node_name
        .as_ref()
        .ok_or_else(|| "IdealLoads outdoor-air diagnostic requires an OA inlet node".to_string())?;
    if outdoor_air_specification.outdoor_air_schedule.is_some() {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic currently requires a blank OA schedule"
                .to_string(),
        );
    }

    validate_outdoor_air_design_flow_boundary(system, outdoor_air_specification.method, manifest)?;
    if let Some((
        conformance_method,
        conformance_economizer,
        conformance_heat_recovery,
        conformance_dcv,
    )) = outdoor_air_conformance_expectations_for_manifest(manifest)
    {
        validate_outdoor_air_conformance_boundary(
            system,
            outdoor_air_specification.method,
            conformance_method,
            conformance_economizer,
            conformance_heat_recovery,
            conformance_dcv,
        )?;
    }

    let site =
        model.typed.site.as_ref().ok_or_else(|| {
            "IdealLoads outdoor-air diagnostics require Site:Location".to_string()
        })?;
    let limit_context = IdealLoadsSensibleLimitContext::from_site_elevation_m(site.elevation_m)
        .ok_or_else(|| {
            format!(
                "failed to derive EnergyPlus StdRhoAir from site elevation {}",
                site.elevation_m
            )
        })?;
    let standard_air_density_kg_per_m3 = limit_context.standard_air_density_kg_per_m3;
    let outdoor_air_context = ideal_loads_outdoor_air_context(&model.typed, zone);

    let mut expected_series = Vec::with_capacity(manifest.outputs.len());
    for output in &manifest.outputs {
        expected_series.push(load_series(&baseline.eso, &output.key, &output.variable)?);
    }
    let zone_air_humidity_ratio =
        load_series(&baseline.eso, &zone.name.0, ZONE_AIR_HUMIDITY_RATIO)?;
    let zone_node_temperature = load_series(
        &baseline.eso,
        &zone_air_node.name.0,
        SYSTEM_NODE_TEMPERATURE,
    )?;
    let zone_node_humidity_ratio = load_series(
        &baseline.eso,
        &zone_air_node.name.0,
        SYSTEM_NODE_HUMIDITY_RATIO,
    )?;
    let outdoor_air_node_temperature = load_series(
        &baseline.eso,
        &outdoor_air_node_name.0,
        SYSTEM_NODE_TEMPERATURE,
    )?;
    let outdoor_air_node_humidity_ratio = load_series(
        &baseline.eso,
        &outdoor_air_node_name.0,
        SYSTEM_NODE_HUMIDITY_RATIO,
    )?;
    let (
        recirculation_node_temperature,
        recirculation_node_humidity_ratio,
        recirculation_node_name,
    ) = match ideal_loads_recirculation_node_name(&model, zone.id, system)
        .ok()
        .and_then(|node_name| {
            let node_temperature =
                load_series(&baseline.eso, &node_name, SYSTEM_NODE_TEMPERATURE).ok()?;
            let node_humidity_ratio =
                load_series(&baseline.eso, &node_name, SYSTEM_NODE_HUMIDITY_RATIO).ok()?;
            Some((node_temperature, node_humidity_ratio, Some(node_name)))
        }) {
        Some(recirculation_trace) => recirculation_trace,
        None => (
            zone_node_temperature.clone(),
            zone_node_humidity_ratio.clone(),
            None,
        ),
    };
    let heating_demand = load_series(
        &baseline.eso,
        &zone.name.0,
        ZONE_SYSTEM_PREDICTED_HEATING_LOAD,
    )?;
    let cooling_demand = load_series(
        &baseline.eso,
        &zone.name.0,
        ZONE_SYSTEM_PREDICTED_COOLING_LOAD,
    )?;
    let co2_setpoint_required_mass_flow_rate = if system.demand_controlled_ventilation_type
        == DemandControlledVentilationType::Co2Setpoint
    {
        Some(load_series(
            &baseline.eso,
            &zone.name.0,
            ZONE_AIR_CO2_PREDICTED_LOAD_TO_SETPOINT_MASS_FLOW_RATE,
        )?)
    } else {
        None
    };
    let sample_count = expected_series
        .iter()
        .map(|series| series.samples.len())
        .chain([
            zone_air_humidity_ratio.samples.len(),
            zone_node_temperature.samples.len(),
            zone_node_humidity_ratio.samples.len(),
            recirculation_node_temperature.samples.len(),
            recirculation_node_humidity_ratio.samples.len(),
            outdoor_air_node_temperature.samples.len(),
            outdoor_air_node_humidity_ratio.samples.len(),
            heating_demand.samples.len(),
            cooling_demand.samples.len(),
        ])
        .chain(
            co2_setpoint_required_mass_flow_rate
                .iter()
                .map(|series| series.samples.len()),
        )
        .min()
        .unwrap_or(0);
    if sample_count == 0 {
        return Err("IdealLoads outdoor-air diagnostic has no samples".to_string());
    }
    let timestamps = expected_series
        .first()
        .map(|series| {
            series
                .samples
                .iter()
                .take(sample_count)
                .map(|sample| sample.timestamp.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let current_people_counts = if system.demand_controlled_ventilation_type
        == DemandControlledVentilationType::OccupancySchedule
    {
        ideal_loads_zone_current_people_counts(&model, zone, sample_count, &timestamps)?
    } else {
        vec![outdoor_air_context.design_people_count; sample_count]
    };
    let (current_people_count_min, current_people_count_max) =
        finite_min_max(&current_people_counts);
    let co2_setpoint_required_mass_flow_rates = co2_setpoint_required_mass_flow_rate
        .as_ref()
        .map(|series| {
            series
                .samples
                .iter()
                .take(sample_count)
                .map(|sample| sample.value)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![0.0; sample_count]);
    let (
        co2_setpoint_required_mass_flow_rate_min_kg_per_s,
        co2_setpoint_required_mass_flow_rate_max_kg_per_s,
    ) = finite_min_max(&co2_setpoint_required_mass_flow_rates);
    let zone_timestep_hours = timestep.zone_timestep_seconds / 3600.0;
    let sample_timestep_hours = expected_series
        .first()
        .map(|series| {
            series
                .samples
                .iter()
                .take(sample_count)
                .map(|sample| {
                    ideal_loads_sample_timestep_hours(
                        sample.timestamp.as_deref(),
                        zone_timestep_hours,
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![zone_timestep_hours; sample_count]);
    let barometric_pressure_trace = ideal_loads_barometric_pressure_samples(
        &model,
        baseline.weather.as_deref(),
        expected_series
            .first()
            .map(|series| series.samples.as_slice())
            .unwrap_or(&[]),
        sample_count,
        limit_context,
    )?;

    let mut sensible_results = Vec::with_capacity(sample_count);
    let mut minimum_outdoor_air_results = Vec::with_capacity(sample_count);
    for index in 0..sample_count {
        let calc_zone_state_index = index.saturating_sub(1);
        let zone_state = IdealLoadsOutdoorAirNodeState {
            air_temperature_c: zone_node_temperature.samples[calc_zone_state_index].value,
            air_humidity_ratio: zone_air_humidity_ratio.samples[index].value,
        };
        let recirculation_state_index = if recirculation_node_name.is_some() {
            index
        } else {
            calc_zone_state_index
        };
        let recirculation_state = IdealLoadsOutdoorAirNodeState {
            air_temperature_c: recirculation_node_temperature.samples[recirculation_state_index]
                .value,
            air_humidity_ratio: recirculation_node_humidity_ratio.samples
                [recirculation_state_index]
                .value,
        };
        let outdoor_air_state = IdealLoadsOutdoorAirNodeState {
            air_temperature_c: outdoor_air_node_temperature.samples[index].value,
            air_humidity_ratio: outdoor_air_node_humidity_ratio.samples[index].value,
        };
        let demand = ZoneSysEnergyDemand::sensible_only(
            zone.id,
            heating_demand.samples[index].value,
            cooling_demand.samples[index].value,
        );
        let current_people_count = (system.demand_controlled_ventilation_type
            == DemandControlledVentilationType::OccupancySchedule)
            .then_some(current_people_counts[index]);
        let co2_setpoint_required_mass_flow_rate_kg_per_s = (system
            .demand_controlled_ventilation_type
            == DemandControlledVentilationType::Co2Setpoint)
            .then_some(co2_setpoint_required_mass_flow_rates[index]);
        let purchased_air =
            sim_purchased_air_outdoor_air_compat(SimPurchasedAirOutdoorAirCompatInput {
                system,
                supply_node: supply_node.id,
                zone_state,
                recirculation_state,
                outdoor_air_state,
                demand,
                minimum_outdoor_air: IdealLoadsMinimumOutdoorAirCompatInput {
                    specification: outdoor_air_specification,
                    context: outdoor_air_context,
                    outdoor_air_schedule_value: None,
                    current_people_count,
                    co2_setpoint_required_mass_flow_rate_kg_per_s,
                },
                system_timestep_hours: sample_timestep_hours[index],
                limit_context: limit_context
                    .with_barometric_pressure_pa(barometric_pressure_trace[index]),
                unit_available: true,
            })
            .map_err(|error| {
                format!(
                    "failed to resolve IdealLoads source-order outdoor-air minimum flow: {error:?}"
                )
            })?;
        minimum_outdoor_air_results.push(purchased_air.minimum_outdoor_air.ok_or_else(|| {
            "IdealLoads outdoor-air diagnostic unexpectedly resolved an unavailable unit"
                .to_string()
        })?);
        sensible_results.push(purchased_air.calculation);
    }
    let first_minimum_outdoor_air = minimum_outdoor_air_results
        .first()
        .copied()
        .ok_or_else(|| "IdealLoads outdoor-air diagnostic has no resolved samples".to_string())?;
    let design_flow_components = first_minimum_outdoor_air.design_flow_components;
    let design_volume_flow_rate_m3_per_s =
        design_flow_components.final_design_volume_flow_rate_m3_per_s;
    let outdoor_air_design_mass_flow_rate_kg_per_s =
        first_minimum_outdoor_air.scheduled_design_mass_flow_rate_kg_per_s;
    let outdoor_air_mass_flow_rates = minimum_outdoor_air_results
        .iter()
        .map(|result| result.final_minimum_mass_flow_rate_kg_per_s)
        .collect::<Vec<_>>();
    let (outdoor_air_mass_flow_rate_min_kg_per_s, outdoor_air_mass_flow_rate_max_kg_per_s) =
        finite_min_max(&outdoor_air_mass_flow_rates);

    let output_handles = resolve_ideal_loads_output_handles(manifest)?;
    let mut rows = Vec::new();
    let mut result_store = ResultStore::new();
    for (output, expected) in manifest.outputs.iter().zip(expected_series.iter()) {
        let output_handle = ideal_loads_output_handle(&output_handles, output)?;
        let (rust_source, units, observed_values) = outdoor_air_observed_values(
            output,
            system.demand_controlled_ventilation_type,
            system.outdoor_air_economizer_type,
            system.heat_recovery_type,
            standard_air_density_kg_per_m3,
            &sensible_results,
            expected.samples.len(),
        )?;
        let timestamps = expected
            .samples
            .iter()
            .map(|sample| sample.timestamp.clone())
            .collect::<Vec<_>>();
        let observed_samples = samples_with_timestamps(&observed_values, &timestamps);
        let tolerance = tolerance_for_output(manifest, output)?;
        let max_rmse_tolerance = max_rmse_tolerance_for_output(manifest, output)?;
        let comparison = compare_series_samples_v2(&expected.samples, &observed_samples, tolerance);
        let mean_abs_delta = mean_abs_delta(&expected.samples, &observed_samples);
        let status = if comparison.status == SeriesComparisonStatus::Pass
            && max_rmse_tolerance.is_none_or(|max_rmse| comparison.rmse_delta <= max_rmse)
        {
            SeriesComparisonStatus::Pass
        } else {
            SeriesComparisonStatus::Fail
        };

        result_store.add_series(OutputSeries {
            handle: output_handle,
            key: output.key.clone(),
            variable_name: output.variable.clone(),
            units: units.to_string(),
            values: observed_values,
        });
        rows.push(IdealLoadsDiagnosticRow {
            handle: output_handle,
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output.frequency,
            variable_class: output.class,
            source: output.source,
            domain: output.domain,
            level: output.level,
            units: units.to_string(),
            oracle_units: expected.units.clone(),
            rust_source,
            tolerance,
            max_rmse_tolerance,
            expected_samples: comparison.expected_samples,
            observed_samples: comparison.observed_samples,
            compared_samples: comparison.compared_samples,
            max_abs_delta: comparison.max_abs_delta,
            mean_abs_delta,
            rmse_delta: comparison.rmse_delta,
            max_rel_delta: comparison.max_rel_delta,
            alignment: comparison.alignment,
            first_divergence: comparison.first_divergence,
            status,
        });
    }

    Ok(IdealLoadsOutdoorAirDiagnosticContext {
        manifest,
        baseline,
        branch: "outdoor-air-design-flow",
        feature_flags: IdealLoadsFeatureFlags::from_system(system),
        zone_equipment_dispatch,
        zone_name: zone.name.0.clone(),
        system_name: system.name.0.clone(),
        outdoor_air_spec_name: outdoor_air_specification.name.0.clone(),
        outdoor_air_method: outdoor_air_specification.method,
        outdoor_air_node_name: outdoor_air_node_name.0.clone(),
        recirculation_node_name,
        demand_controlled_ventilation_type: system.demand_controlled_ventilation_type,
        outdoor_air_economizer_type: system.outdoor_air_economizer_type,
        heat_recovery_type: system.heat_recovery_type,
        standard_air_density_kg_per_m3,
        design_people_count: outdoor_air_context.design_people_count,
        current_people_count_min,
        current_people_count_max,
        co2_setpoint_required_mass_flow_rate_min_kg_per_s,
        co2_setpoint_required_mass_flow_rate_max_kg_per_s,
        zone_floor_area_m2: outdoor_air_context.zone_floor_area_m2,
        zone_volume_m3: outdoor_air_context.zone_volume_m3,
        flow_per_person_m3_per_s: design_flow_components.flow_per_person_m3_per_s,
        flow_per_area_m3_per_s: design_flow_components.flow_per_area_m3_per_s,
        flow_per_zone_m3_per_s: design_flow_components.flow_per_zone_m3_per_s,
        air_changes_m3_per_s: design_flow_components.air_changes_m3_per_s,
        design_volume_flow_rate_m3_per_s,
        outdoor_air_mass_flow_rate_kg_per_s: outdoor_air_design_mass_flow_rate_kg_per_s,
        outdoor_air_mass_flow_rate_min_kg_per_s,
        outdoor_air_mass_flow_rate_max_kg_per_s,
        timestep,
        sample_count,
        rows,
        result_store,
    })
}

fn validate_outdoor_air_design_flow_boundary(
    system: &IdealLoadsAirSystem,
    method: DesignSpecificationOutdoorAirMethod,
    manifest: &ConformanceCase,
) -> Result<(), String> {
    if !matches!(
        method,
        DesignSpecificationOutdoorAirMethod::FlowPerPerson
            | DesignSpecificationOutdoorAirMethod::FlowPerZone
            | DesignSpecificationOutdoorAirMethod::FlowPerArea
            | DesignSpecificationOutdoorAirMethod::AirChangesPerHour
            | DesignSpecificationOutdoorAirMethod::Sum
            | DesignSpecificationOutdoorAirMethod::Maximum
    ) {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic currently requires Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum, or Maximum"
                .to_string(),
        );
    }
    match system.demand_controlled_ventilation_type {
        DemandControlledVentilationType::None => {}
        DemandControlledVentilationType::OccupancySchedule
            if manifest_allows_outdoor_air_occupancy_dcv_conformance_manifest(manifest)
                && method == DesignSpecificationOutdoorAirMethod::FlowPerPerson => {}
        DemandControlledVentilationType::OccupancySchedule => {
            return Err(
                "IdealLoads outdoor-air design-flow diagnostic supports OccupancySchedule DCV only for the declared Flow/Person DCV conformance candidate"
                    .to_string(),
            );
        }
        DemandControlledVentilationType::Co2Setpoint
            if manifest_allows_outdoor_air_co2_dcv_conformance_manifest(manifest)
                && method == DesignSpecificationOutdoorAirMethod::FlowPerPerson => {}
        DemandControlledVentilationType::Co2Setpoint => {
            return Err(
                "IdealLoads outdoor-air design-flow diagnostic supports CO2Setpoint DCV only for the declared Flow/Person CO2 DCV conformance candidate"
                    .to_string(),
            );
        }
    }
    if !matches!(
        system.outdoor_air_economizer_type,
        OutdoorAirEconomizerType::NoEconomizer
            | OutdoorAirEconomizerType::DifferentialDryBulb
            | OutdoorAirEconomizerType::DifferentialEnthalpy
    ) {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic currently supports NoEconomizer, DifferentialDryBulb, or DifferentialEnthalpy economizer".to_string(),
        );
    }
    if !matches!(
        system.heat_recovery_type,
        HeatRecoveryType::None | HeatRecoveryType::Sensible | HeatRecoveryType::Enthalpy
    ) {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic currently supports no heat recovery, Sensible heat recovery, or Enthalpy heat recovery".to_string(),
        );
    }
    if system.heat_recovery_type != HeatRecoveryType::None
        && system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer
    {
        return Err(
            "IdealLoads outdoor-air heat-recovery diagnostic currently requires NoEconomizer"
                .to_string(),
        );
    }
    if system.heating_limit != IdealLoadsLimit::NoLimit
        || system.cooling_limit != IdealLoadsLimit::NoLimit
    {
        return Err(
            "IdealLoads outdoor-air design-flow diagnostic excludes finite flow/capacity limits"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_outdoor_air_conformance_boundary(
    system: &IdealLoadsAirSystem,
    method: DesignSpecificationOutdoorAirMethod,
    expected_method: DesignSpecificationOutdoorAirMethod,
    expected_economizer: OutdoorAirEconomizerType,
    expected_heat_recovery: HeatRecoveryType,
    expected_dcv: DemandControlledVentilationType,
) -> Result<(), String> {
    if method != expected_method {
        return Err(format!(
            "IdealLoads outdoor-air {} conformance candidate requires {}",
            outdoor_air_method_label(expected_method),
            outdoor_air_method_label(expected_method)
        ));
    }
    if system.outdoor_air_economizer_type != expected_economizer {
        return Err(format!(
            "IdealLoads outdoor-air conformance candidate requires {} economizer",
            outdoor_air_economizer_label(expected_economizer)
        ));
    }
    if system.heat_recovery_type != expected_heat_recovery {
        return Err(format!(
            "IdealLoads outdoor-air conformance candidate requires {} heat recovery",
            heat_recovery_label(expected_heat_recovery)
        ));
    }
    if system.demand_controlled_ventilation_type != expected_dcv {
        return Err(format!(
            "IdealLoads outdoor-air conformance candidate requires {} demand controlled ventilation",
            demand_controlled_ventilation_label(expected_dcv)
        ));
    }
    if system.dehumidification_control_type
        != DehumidificationControlType::ConstantSensibleHeatRatio
        || system.humidification_control_type != HumidificationControlType::None
    {
        return Err(
            "IdealLoads outdoor-air conformance candidate requires default ConstantSensibleHeatRatio dehumidification and no humidification control"
                .to_string(),
        );
    }
    Ok(())
}

fn outdoor_air_method_label(method: DesignSpecificationOutdoorAirMethod) -> &'static str {
    match method {
        DesignSpecificationOutdoorAirMethod::FlowPerPerson => "Flow/Person",
        DesignSpecificationOutdoorAirMethod::FlowPerArea => "Flow/Area",
        DesignSpecificationOutdoorAirMethod::FlowPerZone => "Flow/Zone",
        DesignSpecificationOutdoorAirMethod::AirChangesPerHour => "AirChanges/Hour",
        DesignSpecificationOutdoorAirMethod::Sum => "Sum",
        DesignSpecificationOutdoorAirMethod::Maximum => "Maximum",
        DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure => {
            "IndoorAirQualityProcedure"
        }
        DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnDesignOccupancy => {
            "ProportionalControlBasedOnDesignOccupancy"
        }
        DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnOccupancySchedule => {
            "ProportionalControlBasedOnOccupancySchedule"
        }
    }
}

fn outdoor_air_economizer_label(economizer: OutdoorAirEconomizerType) -> &'static str {
    match economizer {
        OutdoorAirEconomizerType::NoEconomizer => "NoEconomizer",
        OutdoorAirEconomizerType::DifferentialDryBulb => "DifferentialDryBulb",
        OutdoorAirEconomizerType::DifferentialEnthalpy => "DifferentialEnthalpy",
    }
}

fn heat_recovery_label(heat_recovery: HeatRecoveryType) -> &'static str {
    match heat_recovery {
        HeatRecoveryType::None => "None",
        HeatRecoveryType::Sensible => "Sensible",
        HeatRecoveryType::Enthalpy => "Enthalpy",
    }
}

fn demand_controlled_ventilation_label(dcv: DemandControlledVentilationType) -> &'static str {
    match dcv {
        DemandControlledVentilationType::None => "None",
        DemandControlledVentilationType::OccupancySchedule => "OccupancySchedule",
        DemandControlledVentilationType::Co2Setpoint => "CO2Setpoint",
    }
}

fn outdoor_air_selected_purchased_air_branch() -> &'static str {
    "outdoor_air"
}

fn outdoor_air_declared_ideal_loads_branch(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> &'static str {
    if manifest_allows_outdoor_air_flow_zone_conformance_manifest(context.manifest) {
        "outdoor_air_flow_zone"
    } else if manifest_allows_outdoor_air_flow_person_conformance_manifest(context.manifest) {
        "outdoor_air_flow_person"
    } else if manifest_allows_outdoor_air_flow_area_conformance_manifest(context.manifest) {
        "outdoor_air_flow_area"
    } else if manifest_allows_outdoor_air_air_changes_conformance_manifest(context.manifest) {
        "outdoor_air_air_changes"
    } else if manifest_allows_outdoor_air_sum_conformance_manifest(context.manifest) {
        "outdoor_air_sum"
    } else if manifest_allows_outdoor_air_maximum_conformance_manifest(context.manifest) {
        "outdoor_air_maximum"
    } else if manifest_allows_outdoor_air_differential_dry_bulb_economizer_conformance_manifest(
        context.manifest,
    ) {
        "outdoor_air_differential_dry_bulb_economizer"
    } else if manifest_allows_outdoor_air_differential_enthalpy_economizer_conformance_manifest(
        context.manifest,
    ) {
        "outdoor_air_differential_enthalpy_economizer"
    } else if manifest_allows_outdoor_air_sensible_heat_recovery_conformance_manifest(
        context.manifest,
    ) {
        "outdoor_air_sensible_heat_recovery"
    } else if manifest_allows_outdoor_air_enthalpy_heat_recovery_conformance_manifest(
        context.manifest,
    ) {
        "outdoor_air_enthalpy_heat_recovery"
    } else if manifest_allows_outdoor_air_occupancy_dcv_conformance_manifest(context.manifest) {
        "outdoor_air_occupancy_dcv"
    } else if manifest_allows_outdoor_air_co2_dcv_conformance_manifest(context.manifest) {
        "outdoor_air_co2_dcv"
    } else {
        "outdoor_air_diagnostic"
    }
}

fn outdoor_air_inactive_branches(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> Vec<&'static str> {
    let mut branches = Vec::new();
    if context.outdoor_air_economizer_type == OutdoorAirEconomizerType::NoEconomizer {
        branches.push("economizer");
    }
    if context.heat_recovery_type == HeatRecoveryType::None {
        branches.push("heat_recovery");
    }
    if context.demand_controlled_ventilation_type == DemandControlledVentilationType::None {
        branches.push("dcv");
    }
    branches.push("humidistat");
    branches.push("autosizing");
    branches.push("saturation_limit");
    branches
}

fn outdoor_air_claim_boundary(context: &IdealLoadsOutdoorAirDiagnosticContext<'_>) -> &'static str {
    if manifest_allows_outdoor_air_flow_zone_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Flow/Zone branch for declared variables only";
    }
    if manifest_allows_outdoor_air_flow_person_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Flow/Person branch for declared variables only";
    }
    if manifest_allows_outdoor_air_flow_area_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Flow/Area branch for declared variables only";
    }
    if manifest_allows_outdoor_air_air_changes_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air AirChanges/Hour branch for declared variables only";
    }
    if manifest_allows_outdoor_air_sum_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Sum branch for declared variables only";
    }
    if manifest_allows_outdoor_air_maximum_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Maximum branch for declared variables only";
    }
    if manifest_allows_outdoor_air_differential_dry_bulb_economizer_conformance_manifest(
        context.manifest,
    ) {
        return "conformance IdealLoads outdoor-air DifferentialDryBulb economizer branch for declared variables only";
    }
    if manifest_allows_outdoor_air_differential_enthalpy_economizer_conformance_manifest(
        context.manifest,
    ) {
        return "conformance IdealLoads outdoor-air DifferentialEnthalpy economizer branch for declared variables only";
    }
    if manifest_allows_outdoor_air_sensible_heat_recovery_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Sensible heat recovery branch for declared variables only";
    }
    if manifest_allows_outdoor_air_enthalpy_heat_recovery_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Enthalpy heat recovery branch for declared variables only; general heat-recovery saturation-limit branch parity remains outside the claim";
    }
    if manifest_allows_outdoor_air_occupancy_dcv_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Flow/Person OccupancySchedule DCV branch for declared variables only; CO2Setpoint DCV and broader DCV methods remain outside the claim";
    }
    if manifest_allows_outdoor_air_co2_dcv_conformance_manifest(context.manifest) {
        return "conformance IdealLoads outdoor-air Flow/Person CO2Setpoint DCV branch for declared variables only; broader DCV method combinations remain outside the claim";
    }
    if context.heat_recovery_type == HeatRecoveryType::Sensible {
        return "diagnostic-only IdealLoads outdoor-air Flow/Zone mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and Sensible heat recovery active-time/rate parity; DCV, economizer, Enthalpy heat recovery, humidity controls, saturation-limit branches, and broad OA conformance remain outside the claim";
    }
    if context.heat_recovery_type == HeatRecoveryType::Enthalpy {
        return "diagnostic-only IdealLoads outdoor-air Flow/Zone mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and Enthalpy heat recovery active-time/rate parity; DCV, economizer, humidity controls, saturation-limit branches, and broad OA conformance remain outside the claim";
    }
    match context.outdoor_air_economizer_type {
        OutdoorAirEconomizerType::DifferentialDryBulb => {
            "diagnostic-only IdealLoads outdoor-air Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum, and Maximum mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and DifferentialDryBulb economizer active-time/flow parity; DCV, DifferentialEnthalpy economizer, heat recovery, humidity controls, saturation-limit branches, and broad OA conformance remain outside the claim"
        }
        OutdoorAirEconomizerType::DifferentialEnthalpy => {
            "diagnostic-only IdealLoads outdoor-air Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum, and Maximum mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and DifferentialEnthalpy economizer active-time/flow parity; DCV, heat recovery, humidity controls, saturation-limit branches, and broad OA conformance remain outside the claim"
        }
        OutdoorAirEconomizerType::NoEconomizer => {
            "diagnostic-only IdealLoads outdoor-air Flow/Person, Flow/Zone, Flow/Area, AirChanges/Hour, Sum, and Maximum mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and inactive economizer/heat recovery"
        }
    }
}

fn outdoor_air_source_description(context: &IdealLoadsOutdoorAirDiagnosticContext<'_>) -> String {
    let dcv_source = match context.demand_controlled_ventilation_type {
        DemandControlledVentilationType::OccupancySchedule => {
            " plus EnergyPlus OccupancySchedule DCV current People schedule occupancy through DataSizing::calcDesignSpecificationOutdoorAir(UseOccSchFlag=true)"
        }
        DemandControlledVentilationType::Co2Setpoint => {
            " plus EnergyPlus CO2Setpoint DCV ZoneSysContDemand(ZoneNum).OutputRequiredToCO2SP through the Zone Air CO2 Predicted Load to Setpoint Mass Flow Rate proof input and CalcPurchAirMinOAMassFlow max(minimum OA, CO2 demand)"
        }
        DemandControlledVentilationType::None => "",
    };
    let economizer_source = match context.outdoor_air_economizer_type {
        OutdoorAirEconomizerType::DifferentialDryBulb => {
            " plus EnergyPlus DifferentialDryBulb economizer OA flow reset when outdoor dry-bulb is below recirculation dry-bulb"
        }
        OutdoorAirEconomizerType::DifferentialEnthalpy => {
            " plus EnergyPlus DifferentialEnthalpy economizer OA flow reset when outdoor enthalpy is below recirculation enthalpy"
        }
        OutdoorAirEconomizerType::NoEconomizer => "",
    };
    let heat_recovery_source = match context.heat_recovery_type {
        HeatRecoveryType::Sensible => {
            " plus EnergyPlus Sensible heat recovery OA tempering when recirculation air can beneficially warm or cool outdoor air"
        }
        HeatRecoveryType::Enthalpy => {
            " plus EnergyPlus Enthalpy heat recovery OA tempering when recirculation enthalpy can beneficially warm or cool outdoor air"
        }
        HeatRecoveryType::None => "",
    };
    format!(
        "DesignSpecification:OutdoorAir {} with blank OA schedule, EnergyPlus StdRhoAir from Site:Location, and source-order zone/OA/mixed-air state proof rows{}{}{}",
        outdoor_air_method_label(context.outdoor_air_method),
        dcv_source,
        economizer_source,
        heat_recovery_source
    )
}

fn ideal_loads_outdoor_air_context(model: &TypedModel, zone: &Zone) -> IdealLoadsOutdoorAirContext {
    IdealLoadsOutdoorAirContext {
        design_people_count: ideal_loads_zone_design_people_count(model, zone),
        zone_floor_area_m2: ideal_loads_zone_floor_area_m2(model, zone),
        zone_volume_m3: ideal_loads_zone_volume_m3(model, zone).unwrap_or(0.0),
    }
}

fn ideal_loads_zone_design_people_count(model: &TypedModel, zone: &Zone) -> f64 {
    let zone_floor_area_m2 = ideal_loads_zone_floor_area_m2(model, zone);
    model
        .people
        .iter()
        .filter(|people| people.zone == zone.id)
        .map(|people| ideal_loads_people_design_count(people, zone_floor_area_m2))
        .filter(|value| value.is_finite() && *value > 0.0)
        .sum()
}

fn ideal_loads_zone_current_people_counts(
    model: &SimulationModel,
    zone: &Zone,
    sample_count: usize,
    timestamps: &[Option<String>],
) -> Result<Vec<f64>, String> {
    let zone_floor_area_m2 = ideal_loads_zone_floor_area_m2(&model.typed, zone);
    let mut values = vec![0.0; sample_count];
    for people in model
        .typed
        .people
        .iter()
        .filter(|people| people.zone == zone.id)
    {
        let design_count = ideal_loads_people_design_count(people, zone_floor_area_m2);
        if !design_count.is_finite() || design_count <= 0.0 {
            continue;
        }
        let schedule_values = ideal_loads_optional_schedule_values(
            model,
            people.number_of_people_schedule,
            &format!("People/{} number-of-people", people.name.0),
            sample_count,
            timestamps,
        )?;
        for (value, schedule_value) in values.iter_mut().zip(schedule_values.iter()) {
            if !schedule_value.is_finite() || *schedule_value < 0.0 {
                return Err(format!(
                    "IdealLoads OccupancySchedule DCV requires nonnegative finite People schedule values, got {} for {}",
                    schedule_value, people.name.0
                ));
            }
            *value += design_count * schedule_value;
        }
    }
    Ok(values)
}

fn ideal_loads_people_design_count(people: &ep_model::People, zone_floor_area_m2: f64) -> f64 {
    match people.number_of_people_calculation_method {
        PeopleNumberCalculationMethod::People => people.number_of_people,
        PeopleNumberCalculationMethod::PeoplePerArea => {
            people.people_per_floor_area * zone_floor_area_m2
        }
        PeopleNumberCalculationMethod::AreaPerPerson => {
            if people.floor_area_per_person > 0.0 {
                zone_floor_area_m2 / people.floor_area_per_person
            } else {
                0.0
            }
        }
    }
}

fn ideal_loads_optional_schedule_values(
    model: &SimulationModel,
    schedule_id: Option<ScheduleId>,
    label: &str,
    sample_count: usize,
    timestamps: &[Option<String>],
) -> Result<Vec<f64>, String> {
    let Some(schedule_id) = schedule_id else {
        return Ok(vec![1.0; sample_count]);
    };
    if let Some(schedule) = model
        .typed
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
    {
        return Ok(vec![schedule.hourly_value; sample_count]);
    }
    let schedule = model
        .typed
        .compact_schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .ok_or_else(|| {
            format!(
                "IdealLoads {label} supports blank, Schedule:Constant, or calendar-invariant Schedule:Compact schedules"
            )
        })?;
    let segments =
        hour_only_single_period_compact_schedule_segments(schedule).map_err(|reason| {
            format!(
                "IdealLoads {label} rejects calendar-varying Schedule:Compact {}: {reason}",
                schedule.name.0
            )
        })?;
    let mut values = Vec::with_capacity(sample_count);
    for index in 0..sample_count {
        let timestamp = timestamps
            .get(index)
            .and_then(|timestamp| timestamp.as_deref());
        let minute_of_day = minute_of_day_from_timestamp(timestamp).ok_or_else(|| {
            format!(
                "IdealLoads {label} Schedule:Compact requires timestamped detailed sample {index}"
            )
        })?;
        let value = compact_schedule_value(segments, minute_of_day).ok_or_else(|| {
            format!(
                "IdealLoads {label} schedule {} has no value for minute {}",
                schedule.name.0, minute_of_day
            )
        })?;
        values.push(value);
    }
    Ok(values)
}

fn finite_min_max(values: &[f64]) -> (f64, f64) {
    let mut min_value = f64::INFINITY;
    let mut max_value = f64::NEG_INFINITY;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        min_value = min_value.min(value);
        max_value = max_value.max(value);
    }
    if min_value == f64::INFINITY {
        (0.0, 0.0)
    } else {
        (min_value, max_value)
    }
}

fn ideal_loads_zone_floor_area_m2(model: &TypedModel, zone: &Zone) -> f64 {
    if let AutoOrNumber::Value(floor_area_m2) = zone.floor_area
        && floor_area_m2 > 0.0
    {
        return floor_area_m2;
    }

    model
        .surfaces
        .iter()
        .filter(|surface| surface.zone == zone.id && surface.surface_type == SurfaceType::Floor)
        .map(|surface| surface_area_m2(&surface.vertices))
        .sum()
}

fn ideal_loads_zone_volume_m3(model: &TypedModel, zone: &Zone) -> Option<f64> {
    if let AutoOrNumber::Value(volume_m3) = zone.volume
        && volume_m3 > 0.0
    {
        return Some(volume_m3);
    }
    if let Some(volume_m3) = ideal_loads_bounding_box_volume_m3(model, zone)
        && volume_m3 > 0.0
    {
        return Some(volume_m3);
    }
    let AutoOrNumber::Value(ceiling_height_m) = zone.ceiling_height else {
        return None;
    };
    if ceiling_height_m <= 0.0 {
        return None;
    }
    let floor_area_m2 = ideal_loads_zone_floor_area_m2(model, zone);
    if floor_area_m2 > 0.0 {
        Some(floor_area_m2 * ceiling_height_m)
    } else {
        None
    }
}

fn ideal_loads_bounding_box_volume_m3(model: &TypedModel, zone: &Zone) -> Option<f64> {
    let mut bounds: Option<(f64, f64, f64, f64, f64, f64)> = None;
    for surface in model
        .surfaces
        .iter()
        .filter(|surface| surface.zone == zone.id)
    {
        for vertex in &surface.vertices {
            let x = vertex.x_m + zone.origin.x_m;
            let y = vertex.y_m + zone.origin.y_m;
            let z = vertex.z_m + zone.origin.z_m;
            bounds = Some(match bounds {
                Some((min_x, max_x, min_y, max_y, min_z, max_z)) => (
                    min_x.min(x),
                    max_x.max(x),
                    min_y.min(y),
                    max_y.max(y),
                    min_z.min(z),
                    max_z.max(z),
                ),
                None => (x, x, y, y, z, z),
            });
        }
    }
    let (min_x, max_x, min_y, max_y, min_z, max_z) = bounds?;
    let volume_m3 = (max_x - min_x) * (max_y - min_y) * (max_z - min_z);
    if volume_m3 > 0.0 {
        Some(volume_m3)
    } else {
        None
    }
}

fn outdoor_air_observed_values(
    output: &OutputRequest,
    demand_controlled_ventilation_type: DemandControlledVentilationType,
    outdoor_air_economizer_type: OutdoorAirEconomizerType,
    heat_recovery_type: HeatRecoveryType,
    standard_air_density_kg_per_m3: f64,
    sensible_results: &[IdealLoadsOutdoorAirSensibleResult],
    expected_samples: usize,
) -> Result<(&'static str, &'static str, Vec<f64>), String> {
    let outdoor_air_flow_source = if sensible_results
        .iter()
        .any(|result| result.economizer_active_time_hr > 0.0)
    {
        outdoor_air_economizer_source(outdoor_air_economizer_type)
    } else if demand_controlled_ventilation_type
        == DemandControlledVentilationType::OccupancySchedule
    {
        "rust-ideal-loads-outdoor-air-occupancy-schedule-dcv"
    } else if demand_controlled_ventilation_type == DemandControlledVentilationType::Co2Setpoint {
        "rust-ideal-loads-outdoor-air-co2-setpoint-dcv"
    } else {
        "rust-ideal-loads-outdoor-air-design-flow"
    };
    match output.variable.as_str() {
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE => Ok((
            outdoor_air_flow_source,
            "kg/s",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_mass_flow_rate_kg_per_s)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE => Ok((
            outdoor_air_flow_source,
            "m3/s",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| {
                    result.outdoor_air_mass_flow_rate_kg_per_s / standard_air_density_kg_per_m3
                })
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-sensible-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_sensible_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-sensible-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_sensible_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-latent-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_latent_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-latent-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_latent_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-total-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_total_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE => Ok((
            "rust-ideal-loads-outdoor-air-total-report",
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.outdoor_air_total_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE => Ok((
            "rust-ideal-loads-outdoor-air-supply-state",
            "kg/s",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.supply_mass_flow_rate_kg_per_s)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE => Ok((
            "rust-ideal-loads-outdoor-air-supply-state",
            "m3/s",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| {
                    result.supply_mass_flow_rate_kg_per_s / standard_air_density_kg_per_m3
                })
                .collect(),
        )),
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE => Ok((
            "rust-ideal-loads-outdoor-air-supply-state",
            "C",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.supply_air_temperature_c)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO => Ok((
            "rust-ideal-loads-outdoor-air-supply-state",
            "kgWater/kgDryAir",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.supply_air_humidity_ratio)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE => Ok((
            "rust-ideal-loads-outdoor-air-mixed-air",
            "C",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.mixed_air_temperature_c)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO => Ok((
            "rust-ideal-loads-outdoor-air-mixed-air",
            "kgWater/kgDryAir",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.mixed_air_humidity_ratio)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_HEATING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_sensible_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_HEATING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_latent_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_HEATING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_total_heating_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_COOLING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_sensible_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_COOLING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_latent_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_COOLING_RATE => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "W",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_total_cooling_rate_w)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_ECONOMIZER_ACTIVE_TIME => Ok((
            if sensible_results
                .iter()
                .any(|result| result.economizer_active_time_hr > 0.0)
            {
                outdoor_air_economizer_source(outdoor_air_economizer_type)
            } else {
                "rust-ideal-loads-outdoor-air-inactive-economizer"
            },
            "hr",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.economizer_active_time_hr)
                .collect(),
        )),
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_ACTIVE_TIME => Ok((
            outdoor_air_heat_recovery_source(heat_recovery_type),
            "hr",
            sensible_results
                .iter()
                .take(expected_samples)
                .map(|result| result.heat_recovery_active_time_hr)
                .collect(),
        )),
        _ => Err(format!(
            "IdealLoads outdoor-air design-flow report cannot produce Rust series for {} / {}",
            output.key, output.variable
        )),
    }
}

fn outdoor_air_economizer_source(
    outdoor_air_economizer_type: OutdoorAirEconomizerType,
) -> &'static str {
    match outdoor_air_economizer_type {
        OutdoorAirEconomizerType::DifferentialDryBulb => {
            "rust-ideal-loads-outdoor-air-differential-dry-bulb-economizer"
        }
        OutdoorAirEconomizerType::DifferentialEnthalpy => {
            "rust-ideal-loads-outdoor-air-differential-enthalpy-economizer"
        }
        OutdoorAirEconomizerType::NoEconomizer => "rust-ideal-loads-outdoor-air-design-flow",
    }
}

fn outdoor_air_heat_recovery_source(heat_recovery_type: HeatRecoveryType) -> &'static str {
    match heat_recovery_type {
        HeatRecoveryType::None => "rust-ideal-loads-outdoor-air-inactive-heat-recovery",
        HeatRecoveryType::Sensible => "rust-ideal-loads-outdoor-air-sensible-heat-recovery",
        HeatRecoveryType::Enthalpy => "rust-ideal-loads-outdoor-air-enthalpy-heat-recovery",
    }
}

fn build_context<'a>(
    manifest: &'a ConformanceCase,
    baseline: &'a BaselineSummary,
) -> Result<IdealLoadsDiagnosticContext<'a>, String> {
    let raw_model = baseline.load_raw_model()?;
    let compile_result = compile_raw_model(&raw_model);
    let typed = compile_result.model.ok_or_else(|| {
        compile_result
            .report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let model = SimulationModel::from_typed(typed);
    let timestep = ideal_loads_timestep_context(&model.typed)?;
    if model.typed.zones.len() != 1 {
        return Err(format!(
            "IdealLoads no-OA report requires one zone, got {}",
            model.typed.zones.len()
        ));
    }
    if model.typed.ideal_loads_air_systems.len() != 1 {
        return Err(format!(
            "IdealLoads no-OA report requires one IdealLoads system, got {}",
            model.typed.ideal_loads_air_systems.len()
        ));
    }

    let edge = model
        .graph
        .zone_ideal_loads
        .first()
        .ok_or_else(|| "missing zone to IdealLoads graph edge".to_string())?;
    let zone = model
        .typed
        .zones
        .iter()
        .find(|zone| zone.id == edge.zone)
        .ok_or_else(|| "missing controlled zone for IdealLoads edge".to_string())?;
    let system = model
        .typed
        .ideal_loads_air_systems
        .iter()
        .find(|system| system.id == edge.ideal_loads_air_system)
        .ok_or_else(|| "missing IdealLoads system for graph edge".to_string())?;
    let zone_equipment_dispatch = validate_ideal_loads_zone_equipment_dispatch(&model, system.id);
    if !zone_equipment_dispatch.is_dispatchable() {
        return Err(format!(
            "IdealLoads zone equipment dispatch prerequisites failed: {}",
            label_list_or_none(&zone_equipment_dispatch.issue_codes())
        ));
    }
    if manifest.conformance_claim && !zone_equipment_dispatch.is_conformance_candidate() {
        return Err(format!(
            "IdealLoads conformance candidate requires single-zone/single-equipment dispatch scope: {}",
            label_list_or_none(&zone_equipment_dispatch.warning_codes())
        ));
    }
    let supply_edge = model
        .graph
        .ideal_loads_supply_nodes
        .iter()
        .find(|candidate| candidate.ideal_loads_air_system == system.id)
        .ok_or_else(|| "missing IdealLoads supply-node edge".to_string())?;
    let supply_node = model
        .typed
        .nodes
        .iter()
        .find(|node| node.id == supply_edge.node)
        .ok_or_else(|| "missing IdealLoads supply node".to_string())?;
    let zone_air_node_edge = model
        .graph
        .zone_air_nodes
        .iter()
        .find(|candidate| candidate.zone == zone.id)
        .ok_or_else(|| "missing zone air-node edge".to_string())?;
    let zone_air_node = model
        .typed
        .nodes
        .iter()
        .find(|node| node.id == zone_air_node_edge.node)
        .ok_or_else(|| "missing zone air node".to_string())?;

    let mut boundary = if manifest_allows_finite_limit_conformance(manifest, system) {
        classify_no_oa_sensible_subset(system)
    } else if manifest.conformance_claim {
        classify_no_oa_no_limit_sensible_subset(system)
    } else {
        classify_no_oa_sensible_subset(system)
    };
    if manifest_allows_constant_supply_humidity_diagnostic(manifest, system)
        || manifest_allows_constant_supply_humidity_cooling_conformance(manifest, system)
        || manifest_allows_constant_supply_humidity_cooling_annual_meter_conformance(
            manifest, system,
        )
        || manifest_allows_humidistat_dehumidification_diagnostic(manifest, system)
        || manifest_allows_humidistat_dehumidification_conformance(manifest, system)
        || manifest_allows_humidistat_dehumidification_annual_meter_conformance(manifest, system)
    {
        boundary
            .unsupported_features
            .retain(|feature| *feature != IdealLoadsUnsupportedFeature::Dehumidification);
    }
    if manifest_allows_constant_supply_humidity_humidification_diagnostic(manifest, system)
        || manifest_allows_constant_supply_humidity_heating_conformance(manifest, system)
        || manifest_allows_constant_supply_humidity_heating_annual_meter_conformance(
            manifest, system,
        )
        || manifest_allows_humidistat_humidification_diagnostic(manifest, system)
        || manifest_allows_humidistat_humidification_conformance(manifest, system)
        || manifest_allows_humidistat_humidification_annual_meter_conformance(manifest, system)
    {
        boundary
            .unsupported_features
            .retain(|feature| *feature != IdealLoadsUnsupportedFeature::Humidification);
    }
    if !boundary.is_supported() {
        return Err(format!(
            "IdealLoads system is outside no-OA sensible subset: {}",
            unsupported_features_label(&boundary.unsupported_features)
        ));
    }

    let recirculation_node_name = if uses_finite_limits(system)
        || manifest_requests_ideal_loads_recirculation_node(manifest, &model, zone.id, system)?
    {
        Some(ideal_loads_recirculation_node_name(
            &model, zone.id, system,
        )?)
    } else {
        None
    };
    let input_trace = load_input_trace(
        &baseline.eso,
        &zone.name.0,
        &zone_air_node.name.0,
        recirculation_node_name.as_deref(),
    )?;
    let fuel_efficiency = ideal_loads_fuel_efficiency_context(&model, system, &input_trace)?;
    let mtr = baseline.output_dir.join("eplusout.mtr");
    let (rows, meter_rows, result_store, mode_counts, moisture_predictor) = evaluate_rows(
        manifest,
        &model,
        &baseline.eso,
        &mtr,
        baseline.weather.as_deref(),
        &input_trace,
        &zone.name.0,
        &zone_air_node.name.0,
        recirculation_node_name.as_deref(),
        &system.name.0,
        &supply_node.name.0,
        timestep.zone_timestep_seconds,
        fuel_efficiency.clone(),
    )?;

    let zone_name = zone.name.0.clone();
    let zone_air_node_name = zone_air_node.name.0.clone();
    let system_name = system.name.0.clone();
    let supply_node_name = supply_node.name.0.clone();
    let branch = ideal_loads_sensible_branch(system);
    let selected_purchased_air_branch = select_purchased_air_branch(system).label();
    let declared_ideal_loads_branch = declared_ideal_loads_branch(manifest, system);
    let inactive_branches = inactive_ideal_loads_branches(system);
    let feature_flags = IdealLoadsFeatureFlags::from_system(system);
    let constant_shr_conformance_claim = manifest_allows_constant_shr_conformance(manifest, system);
    let constant_supply_humidity_cooling_conformance_claim =
        manifest_allows_constant_supply_humidity_cooling_conformance(manifest, system);
    let constant_supply_humidity_heating_conformance_claim =
        manifest_allows_constant_supply_humidity_heating_conformance(manifest, system);
    let humidistat_dehumidification_conformance_claim =
        manifest_allows_humidistat_dehumidification_conformance(manifest, system);
    let humidistat_humidification_conformance_claim =
        manifest_allows_humidistat_humidification_conformance(manifest, system);
    let humidity_annual_facility_meter_conformance_claim =
        manifest_allows_humidity_annual_facility_meter_conformance(manifest, system);

    Ok(IdealLoadsDiagnosticContext {
        manifest,
        baseline,
        branch,
        selected_purchased_air_branch,
        declared_ideal_loads_branch,
        inactive_branches,
        feature_flags,
        zone_equipment_dispatch,
        constant_shr_conformance_claim,
        constant_supply_humidity_cooling_conformance_claim,
        constant_supply_humidity_heating_conformance_claim,
        humidistat_dehumidification_conformance_claim,
        humidistat_humidification_conformance_claim,
        humidity_annual_facility_meter_conformance_claim,
        zone_name,
        zone_air_node_name,
        recirculation_node_name,
        system_name,
        supply_node_name,
        timestep,
        fuel_efficiency,
        rows,
        meter_rows,
        result_store,
        input_trace,
        mode_counts,
        moisture_predictor,
    })
}

fn load_input_trace(
    eso: &Path,
    zone_name: &str,
    zone_air_node_name: &str,
    recirculation_node_name: Option<&str>,
) -> Result<IdealLoadsInputTrace, String> {
    let zone_node_temperature = load_series(eso, zone_air_node_name, SYSTEM_NODE_TEMPERATURE)?;
    let zone_air_temperature = load_optional_series_or_reference(
        eso,
        zone_name,
        ZONE_AIR_TEMPERATURE,
        &zone_node_temperature,
    );
    let zone_air_temperature_warmup_tail = load_warmup_tail(eso, zone_name, ZONE_AIR_TEMPERATURE);
    let zone_node_humidity_ratio =
        load_series(eso, zone_air_node_name, SYSTEM_NODE_HUMIDITY_RATIO)?;
    let zone_air_humidity_ratio = load_optional_series_or_reference(
        eso,
        zone_name,
        ZONE_AIR_HUMIDITY_RATIO,
        &zone_node_humidity_ratio,
    );
    let zone_air_humidity_ratio_warmup_tail =
        load_warmup_tail(eso, zone_name, ZONE_AIR_HUMIDITY_RATIO);
    let zone_mean_air_humidity_ratio = load_optional_series_or_reference(
        eso,
        zone_name,
        ZONE_MEAN_AIR_HUMIDITY_RATIO,
        &zone_air_humidity_ratio,
    );
    let zone_mean_air_humidity_ratio_warmup_tail =
        load_warmup_tail(eso, zone_name, ZONE_MEAN_AIR_HUMIDITY_RATIO);
    let site_barometric_pressure =
        load_optional_series(eso, ENVIRONMENT_KEY, SITE_OUTDOOR_AIR_BAROMETRIC_PRESSURE);
    let (recirculation_node_temperature, recirculation_node_humidity_ratio) =
        match recirculation_node_name {
            Some(recirculation_node_name) => (
                load_series(eso, recirculation_node_name, SYSTEM_NODE_TEMPERATURE)?,
                load_series(eso, recirculation_node_name, SYSTEM_NODE_HUMIDITY_RATIO)?,
            ),
            None => (
                zone_node_temperature.clone(),
                zone_node_humidity_ratio.clone(),
            ),
        };
    let active_demand = load_series(eso, zone_name, ZONE_SYSTEM_PREDICTED_SETPOINT_LOAD)?;
    let heating_demand = load_series(eso, zone_name, ZONE_SYSTEM_PREDICTED_HEATING_LOAD)?;
    let cooling_demand = load_series(eso, zone_name, ZONE_SYSTEM_PREDICTED_COOLING_LOAD)?;
    let humidifying_moisture_demand = load_optional_series_or_zero(
        eso,
        zone_name,
        ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD,
        &active_demand,
        "kgWater/s",
    )?;
    let dehumidifying_moisture_demand = load_optional_series_or_zero(
        eso,
        zone_name,
        ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD,
        &active_demand,
        "kgWater/s",
    )?;
    let sample_count = [
        zone_node_temperature.samples.len(),
        zone_air_temperature.samples.len(),
        zone_air_humidity_ratio.samples.len(),
        zone_mean_air_humidity_ratio.samples.len(),
        zone_node_humidity_ratio.samples.len(),
        site_barometric_pressure
            .as_ref()
            .map_or(usize::MAX, |series| series.samples.len()),
        recirculation_node_temperature.samples.len(),
        recirculation_node_humidity_ratio.samples.len(),
        active_demand.samples.len(),
        heating_demand.samples.len(),
        cooling_demand.samples.len(),
        humidifying_moisture_demand.samples.len(),
        dehumidifying_moisture_demand.samples.len(),
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    if sample_count == 0 {
        return Err("IdealLoads diagnostic input trace has no samples".to_string());
    }

    Ok(IdealLoadsInputTrace {
        sample_count,
        zone_air_temperature,
        zone_air_temperature_warmup_tail,
        zone_node_temperature,
        zone_air_humidity_ratio,
        zone_air_humidity_ratio_warmup_tail,
        zone_mean_air_humidity_ratio,
        zone_mean_air_humidity_ratio_warmup_tail,
        zone_node_humidity_ratio,
        site_barometric_pressure,
        recirculation_node_temperature,
        recirculation_node_humidity_ratio,
        active_demand,
        heating_demand,
        cooling_demand,
        humidifying_moisture_demand,
        dehumidifying_moisture_demand,
    })
}

fn timestamp_numeric_field(timestamp: &str, field_name: &str) -> Option<f64> {
    let prefix = format!("{field_name}=");
    timestamp
        .split(';')
        .find_map(|part| part.strip_prefix(&prefix))
        .and_then(|value| value.parse::<f64>().ok())
}

fn load_series(eso: &Path, key: &str, variable: &str) -> Result<LoadedSeries, String> {
    let series = load_eso_time_series(eso, key, variable)
        .map_err(|error| format!("failed to load ESO series {key}/{variable}: {error}"))?;
    Ok(LoadedSeries {
        units: series.metadata.units,
        samples: run_period_samples(series.samples),
    })
}

fn load_optional_series_or_zero(
    eso: &Path,
    key: &str,
    variable: &str,
    reference: &LoadedSeries,
    units: &str,
) -> Result<LoadedSeries, String> {
    match load_eso_time_series(eso, key, variable) {
        Ok(series) => Ok(LoadedSeries {
            units: series.metadata.units,
            samples: run_period_samples(series.samples),
        }),
        Err(_) => Ok(LoadedSeries {
            units: Some(units.to_string()),
            samples: reference
                .samples
                .iter()
                .enumerate()
                .map(|(index, sample)| SeriesSample {
                    index,
                    timestamp: sample.timestamp.clone(),
                    value: 0.0,
                })
                .collect(),
        }),
    }
}

fn load_optional_series_or_reference(
    eso: &Path,
    key: &str,
    variable: &str,
    reference: &LoadedSeries,
) -> LoadedSeries {
    match load_eso_time_series(eso, key, variable) {
        Ok(series) => LoadedSeries {
            units: series.metadata.units,
            samples: run_period_samples(series.samples),
        },
        Err(_) => reference.clone(),
    }
}

fn load_optional_series(eso: &Path, key: &str, variable: &str) -> Option<LoadedSeries> {
    load_eso_time_series(eso, key, variable)
        .ok()
        .map(|series| LoadedSeries {
            units: series.metadata.units,
            samples: run_period_samples(series.samples),
        })
}

fn load_warmup_tail(eso: &Path, key: &str, variable: &str) -> Option<[f64; 3]> {
    let series = load_eso_time_series(eso, key, variable).ok()?;
    let mut warmup_values = Vec::new();
    for sample in series.samples {
        let timestamp = sample.timestamp.as_deref();
        if timestamp.is_some_and(is_run_period_timestamp) {
            break;
        }
        warmup_values.push(sample.value);
    }
    let count = warmup_values.len();
    (count >= 3).then(|| {
        [
            warmup_values[count - 1],
            warmup_values[count - 2],
            warmup_values[count - 3],
        ]
    })
}

fn run_period_samples(samples: Vec<SeriesSample>) -> Vec<SeriesSample> {
    let run_period = samples
        .iter()
        .filter(|sample| {
            sample
                .timestamp
                .as_deref()
                .is_some_and(is_run_period_timestamp)
        })
        .cloned()
        .collect::<Vec<_>>();
    if run_period.is_empty() {
        samples
    } else {
        run_period
    }
}

fn is_run_period_timestamp(timestamp: &str) -> bool {
    timestamp.to_ascii_uppercase().contains("ENV=RUN PERIOD")
}

fn evaluate_rows(
    manifest: &ConformanceCase,
    model: &SimulationModel,
    eso: &Path,
    mtr: &Path,
    weather: Option<&Path>,
    input_trace: &IdealLoadsInputTrace,
    zone_name: &str,
    zone_air_node_name: &str,
    recirculation_node_name: Option<&str>,
    system_name: &str,
    supply_node_name: &str,
    nominal_zone_timestep_seconds: f64,
    fuel_efficiency: IdealLoadsFuelEfficiencyContext,
) -> Result<
    (
        Vec<IdealLoadsDiagnosticRow>,
        Vec<IdealLoadsMeterDiagnosticRow>,
        ResultStore,
        IdealLoadsModeCounts,
        Option<IdealLoadsMoisturePredictorSummary>,
    ),
    String,
> {
    let system = model
        .typed
        .ideal_loads_air_systems
        .first()
        .ok_or_else(|| "missing IdealLoads system".to_string())?;
    let zone = model
        .typed
        .zones
        .first()
        .ok_or_else(|| "missing controlled zone".to_string())?;
    let supply_node = model
        .typed
        .nodes
        .iter()
        .find(|node| node.name.0.eq_ignore_ascii_case(supply_node_name))
        .ok_or_else(|| "missing supply node".to_string())?;

    let heating_setpoint =
        thermostat_setpoint_values(model, zone.id, true, input_trace.sample_count)?;
    let cooling_setpoint =
        thermostat_setpoint_values(model, zone.id, false, input_trace.sample_count)?;
    let limit_context = ideal_loads_limit_context(model, system)?;
    let barometric_pressure_trace =
        ideal_loads_barometric_pressure_trace(model, weather, input_trace, limit_context)?;
    let source_order_trace_uses_recirculation = recirculation_node_name.is_some()
        && (uses_finite_limits(system) || uses_ideal_loads_humidity_control(system));
    let moisture_predictor = moisture_predictor_summary(
        model,
        system,
        zone,
        eso,
        input_trace,
        supply_node.id,
        system_name,
        supply_node_name,
        limit_context,
        &barometric_pressure_trace,
        source_order_trace_uses_recirculation,
        nominal_zone_timestep_seconds,
    )?;
    let promote_moisture_predictor = moisture_predictor.is_some()
        && manifest_promotes_humidistat_moisture_predictor(manifest, system);
    let mut calc_results = Vec::with_capacity(input_trace.sample_count);
    let mut mode_counts = IdealLoadsModeCounts::default();
    if promote_moisture_predictor {
        let summary = moisture_predictor
            .as_ref()
            .ok_or_else(|| "promoted moisture predictor requires summary".to_string())?;
        if summary.closed_loop_results.len() < input_trace.sample_count {
            return Err(format!(
                "IdealLoads Humidistat closed-loop result count {} is shorter than sample count {}",
                summary.closed_loop_results.len(),
                input_trace.sample_count
            ));
        }
        calc_results.extend(
            summary
                .closed_loop_results
                .iter()
                .copied()
                .take(input_trace.sample_count),
        );
        for result in &calc_results {
            record_mode(&mut mode_counts, result.mode);
        }
    } else {
        for index in 0..input_trace.sample_count {
            let (zone_temperature, zone_humidity_ratio) = if source_order_trace_uses_recirculation {
                (
                    input_trace.recirculation_node_temperature.samples[index].value,
                    input_trace.recirculation_node_humidity_ratio.samples[index].value,
                )
            } else {
                // CalcPurchAirLoads sees the zone node before the same-timestamp node
                // output row is updated, so no-limit transition samples use the previous row.
                let calc_zone_state_index = index.saturating_sub(1);
                (
                    input_trace.zone_node_temperature.samples[calc_zone_state_index].value,
                    input_trace.zone_node_humidity_ratio.samples[calc_zone_state_index].value,
                )
            };
            let active_demand = input_trace.active_demand.samples[index].value;
            let heating_demand = active_demand.max(0.0);
            let cooling_demand = active_demand.min(0.0);
            let zone_state = IdealLoadsZoneState {
                air_temperature_c: zone_temperature,
                air_humidity_ratio: zone_humidity_ratio,
            };
            let recirculation_state = if recirculation_node_name.is_some() {
                IdealLoadsZoneState {
                    air_temperature_c: input_trace.recirculation_node_temperature.samples[index]
                        .value,
                    air_humidity_ratio: input_trace.recirculation_node_humidity_ratio.samples
                        [index]
                        .value,
                }
            } else {
                zone_state
            };
            let mut demand =
                ZoneSysEnergyDemand::sensible_only(zone.id, heating_demand, cooling_demand);
            demand.remaining_output_req_to_humid_sp_kg_per_s =
                input_trace.humidifying_moisture_demand.samples[index].value;
            demand.remaining_output_req_to_dehumid_sp_kg_per_s =
                input_trace.dehumidifying_moisture_demand.samples[index].value;
            let purchased_air = sim_purchased_air_compat(SimPurchasedAirCompatInput {
                system,
                supply_node: supply_node.id,
                zone_state,
                recirculation_state,
                demand,
                unit_available: true,
                limit_context: limit_context
                    .with_barometric_pressure_pa(barometric_pressure_trace[index]),
            })
            .map_err(|error| {
                format!(
                    "IdealLoads SimPurchasedAir compatibility path rejected system {:?}: {:?}",
                    error.system_id, error.unsupported_features
                )
            })?;
            let result = purchased_air.report;
            record_mode(&mut mode_counts, result.mode);
            calc_results.push(result);
        }
    }

    let result_source = rust_result_source(system);
    let timestamps = input_trace
        .active_demand
        .samples
        .iter()
        .take(input_trace.sample_count)
        .map(|sample| sample.timestamp.clone())
        .collect::<Vec<_>>();

    let mut observed_by_variable = BTreeMap::new();
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_THERMOSTAT_HEATING_SETPOINT_TEMPERATURE.to_string(),
        ),
        ObservedSeries::new("rust-thermostat-schedule", "C", heating_setpoint),
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_THERMOSTAT_COOLING_SETPOINT_TEMPERATURE.to_string(),
        ),
        ObservedSeries::new("rust-thermostat-schedule", "C", cooling_setpoint),
    );
    observed_by_variable.insert(
        (
            zone_air_node_name.to_string(),
            SYSTEM_NODE_TEMPERATURE.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-air-node-input",
            "C",
            values_from_samples(
                &input_trace.zone_node_temperature.samples,
                input_trace.sample_count,
            ),
        ),
    );
    observed_by_variable.insert(
        (
            zone_air_node_name.to_string(),
            SYSTEM_NODE_HUMIDITY_RATIO.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-air-node-input",
            "kgWater/kgDryAir",
            values_from_samples(
                &input_trace.zone_node_humidity_ratio.samples,
                input_trace.sample_count,
            ),
        ),
    );
    if let Some(recirculation_node_name) = recirculation_node_name {
        observed_by_variable.insert(
            (
                recirculation_node_name.to_string(),
                SYSTEM_NODE_TEMPERATURE.to_string(),
            ),
            ObservedSeries::new(
                "oracle-recirculation-node-input",
                "C",
                values_from_samples(
                    &input_trace.recirculation_node_temperature.samples,
                    input_trace.sample_count,
                ),
            ),
        );
        observed_by_variable.insert(
            (
                recirculation_node_name.to_string(),
                SYSTEM_NODE_HUMIDITY_RATIO.to_string(),
            ),
            ObservedSeries::new(
                "oracle-recirculation-node-input",
                "kgWater/kgDryAir",
                values_from_samples(
                    &input_trace.recirculation_node_humidity_ratio.samples,
                    input_trace.sample_count,
                ),
            ),
        );
    }
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_SETPOINT_LOAD.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-system-active-demand-input",
            "W",
            values_from_samples(&input_trace.active_demand.samples, input_trace.sample_count),
        ),
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_HEATING_LOAD.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-system-demand-input",
            "W",
            values_from_samples(
                &input_trace.heating_demand.samples,
                input_trace.sample_count,
            ),
        ),
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_COOLING_LOAD.to_string(),
        ),
        ObservedSeries::new(
            "oracle-zone-system-demand-input",
            "W",
            values_from_samples(
                &input_trace.cooling_demand.samples,
                input_trace.sample_count,
            ),
        ),
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD.to_string(),
        ),
        if promote_moisture_predictor {
            let summary = moisture_predictor
                .as_ref()
                .ok_or_else(|| "promoted moisture predictor requires summary".to_string())?;
            ObservedSeries::new(
                "rust-zone-system-moisture-demand-closed-loop-predictor",
                "kgWater/s",
                summary.closed_loop_humidifying_values.clone(),
            )
        } else {
            ObservedSeries::new(
                "oracle-zone-system-moisture-demand-input",
                "kgWater/s",
                values_from_samples(
                    &input_trace.humidifying_moisture_demand.samples,
                    input_trace.sample_count,
                ),
            )
        },
    );
    observed_by_variable.insert(
        (
            zone_name.to_string(),
            ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD.to_string(),
        ),
        if promote_moisture_predictor {
            let summary = moisture_predictor
                .as_ref()
                .ok_or_else(|| "promoted moisture predictor requires summary".to_string())?;
            ObservedSeries::new(
                "rust-zone-system-moisture-demand-closed-loop-predictor",
                "kgWater/s",
                summary.closed_loop_dehumidifying_values.clone(),
            )
        } else {
            ObservedSeries::new(
                "oracle-zone-system-moisture-demand-input",
                "kgWater/s",
                values_from_samples(
                    &input_trace.dehumidifying_moisture_demand.samples,
                    input_trace.sample_count,
                ),
            )
        },
    );

    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
        "W",
        result_source,
        |result| result.zone_total_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE,
        "W",
        result_source,
        |result| result.zone_total_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE,
        "W",
        result_source,
        |result| result.zone_sensible_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE,
        "W",
        result_source,
        |result| result.zone_sensible_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE,
        "W",
        result_source,
        |result| result.zone_latent_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
        "W",
        result_source,
        |result| result.zone_latent_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE,
        "W",
        result_source,
        |result| result.supply_air_sensible_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE,
        "W",
        result_source,
        |result| result.supply_air_sensible_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE,
        "W",
        result_source,
        |result| result.supply_air_latent_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
        "W",
        result_source,
        |result| result.supply_air_latent_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE,
        "W",
        result_source,
        |result| result.supply_air_total_heating_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE,
        "W",
        result_source,
        |result| result.supply_air_total_cooling_rate_w,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
        "kg/s",
        result_source,
        |result| result.supply_mass_flow_rate_kg_per_s,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE,
        "C",
        result_source,
        |result| result.supply_temperature_c,
    );
    add_result_series(
        &mut observed_by_variable,
        system_name,
        &calc_results,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
        "kgWater/kgDryAir",
        result_source,
        |result| result.supply_humidity_ratio,
    );
    if manifest_requests_report_energies(manifest) {
        let energy_source = "rust-ideal-loads-report-time-step-energy";
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY,
            energy_source,
            &timestamps,
            nominal_zone_timestep_seconds,
            |result| result.supply_air_total_heating_rate_w,
        );
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY,
            energy_source,
            &timestamps,
            nominal_zone_timestep_seconds,
            |result| result.supply_air_total_cooling_rate_w,
        );
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
            energy_source,
            &timestamps,
            nominal_zone_timestep_seconds,
            |result| result.zone_total_heating_rate_w,
        );
        add_result_energy_series(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY,
            energy_source,
            &timestamps,
            nominal_zone_timestep_seconds,
            |result| result.zone_total_cooling_rate_w,
        );
    }
    if manifest_requests_fuel_energy_outputs(manifest) || !manifest.meters.is_empty() {
        let fuel_source = fuel_efficiency.rate_rust_source;
        add_result_series_indexed(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY_RATE,
            "W",
            fuel_source,
            |index, result| {
                result.supply_air_total_heating_rate_w / fuel_efficiency.heating_at(index)
            },
        );
        add_result_series_indexed(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY_RATE,
            "W",
            fuel_source,
            |index, result| {
                result.supply_air_total_cooling_rate_w / fuel_efficiency.cooling_at(index)
            },
        );
        add_result_series_indexed(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY_RATE,
            "W",
            fuel_source,
            |index, result| result.zone_total_heating_rate_w / fuel_efficiency.heating_at(index),
        );
        add_result_series_indexed(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY_RATE,
            "W",
            fuel_source,
            |index, result| result.zone_total_cooling_rate_w / fuel_efficiency.cooling_at(index),
        );
        let fuel_energy_source = fuel_efficiency.energy_rust_source;
        add_result_energy_series_indexed(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY,
            fuel_energy_source,
            &timestamps,
            nominal_zone_timestep_seconds,
            |index, result| {
                result.supply_air_total_heating_rate_w / fuel_efficiency.heating_at(index)
            },
        );
        add_result_energy_series_indexed(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY,
            fuel_energy_source,
            &timestamps,
            nominal_zone_timestep_seconds,
            |index, result| {
                result.supply_air_total_cooling_rate_w / fuel_efficiency.cooling_at(index)
            },
        );
        add_result_energy_series_indexed(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY,
            fuel_energy_source,
            &timestamps,
            nominal_zone_timestep_seconds,
            |index, result| result.zone_total_heating_rate_w / fuel_efficiency.heating_at(index),
        );
        add_result_energy_series_indexed(
            &mut observed_by_variable,
            system_name,
            &calc_results,
            ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY,
            fuel_energy_source,
            &timestamps,
            nominal_zone_timestep_seconds,
            |index, result| result.zone_total_cooling_rate_w / fuel_efficiency.cooling_at(index),
        );
    }
    add_result_series(
        &mut observed_by_variable,
        supply_node_name,
        &calc_results,
        SYSTEM_NODE_TEMPERATURE,
        "C",
        result_source,
        |result| result.supply_temperature_c,
    );
    add_result_series(
        &mut observed_by_variable,
        supply_node_name,
        &calc_results,
        SYSTEM_NODE_HUMIDITY_RATIO,
        "kgWater/kgDryAir",
        result_source,
        |result| result.supply_humidity_ratio,
    );
    add_result_series(
        &mut observed_by_variable,
        supply_node_name,
        &calc_results,
        SYSTEM_NODE_MASS_FLOW_RATE,
        "kg/s",
        result_source,
        |result| result.supply_mass_flow_rate_kg_per_s,
    );

    let meter_rows = evaluate_meter_rows(
        manifest,
        model,
        mtr,
        system_name,
        &observed_by_variable,
        &timestamps,
    )?;

    let output_handles = resolve_ideal_loads_output_handles(manifest)?;
    let mut rows = Vec::new();
    let mut result_store = ResultStore::new();
    for output in &manifest.outputs {
        let output_handle = ideal_loads_output_handle(&output_handles, output)?;
        let expected = load_series(eso, &output.key, &output.variable)?;
        let Some(observed) =
            observed_by_variable.get(&(output.key.clone(), output.variable.clone()))
        else {
            return Err(format!(
                "IdealLoads diagnostic report cannot produce Rust series for {} / {}",
                output.key, output.variable
            ));
        };
        let observed_samples = samples_with_timestamps(&observed.values, &timestamps);
        let tolerance = tolerance_for_output(manifest, output)?;
        let max_rmse_tolerance = max_rmse_tolerance_for_output(manifest, output)?;
        let comparison = compare_series_samples_v2(&expected.samples, &observed_samples, tolerance);
        let mean_abs_delta = mean_abs_delta(&expected.samples, &observed_samples);
        let status = if comparison.status == SeriesComparisonStatus::Pass
            && max_rmse_tolerance.is_none_or(|max_rmse| comparison.rmse_delta <= max_rmse)
        {
            SeriesComparisonStatus::Pass
        } else {
            SeriesComparisonStatus::Fail
        };

        result_store.add_series(OutputSeries {
            handle: output_handle,
            key: output.key.clone(),
            variable_name: output.variable.clone(),
            units: observed.units.to_string(),
            values: observed.values.clone(),
        });
        rows.push(IdealLoadsDiagnosticRow {
            handle: output_handle,
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output.frequency,
            variable_class: output.class,
            source: output.source,
            domain: output.domain,
            level: output.level,
            units: observed.units.to_string(),
            oracle_units: expected.units.clone(),
            rust_source: observed.source,
            tolerance,
            max_rmse_tolerance,
            expected_samples: comparison.expected_samples,
            observed_samples: comparison.observed_samples,
            compared_samples: comparison.compared_samples,
            max_abs_delta: comparison.max_abs_delta,
            mean_abs_delta,
            rmse_delta: comparison.rmse_delta,
            max_rel_delta: comparison.max_rel_delta,
            alignment: comparison.alignment,
            first_divergence: comparison.first_divergence,
            status,
        });
    }

    Ok((
        rows,
        meter_rows,
        result_store,
        mode_counts,
        moisture_predictor.map(|mut summary| {
            summary.promoted_input = promote_moisture_predictor;
            summary
        }),
    ))
}

fn manifest_requests_report_energies(manifest: &ConformanceCase) -> bool {
    manifest
        .outputs
        .iter()
        .any(|output| ideal_loads_report_energy_variable(&output.variable))
}

fn ideal_loads_report_energy_variable(variable: &str) -> bool {
    matches!(
        variable,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY
            | ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY
    )
}

fn manifest_requests_fuel_energy_outputs(manifest: &ConformanceCase) -> bool {
    manifest
        .outputs
        .iter()
        .any(|output| ideal_loads_fuel_energy_variable(&output.variable))
}

fn ideal_loads_fuel_energy_variable(variable: &str) -> bool {
    matches!(
        variable,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY_RATE
            | ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY_RATE
            | ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY
            | ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY
            | ZONE_IDEAL_LOADS_ZONE_HEATING_FUEL_ENERGY_RATE
            | ZONE_IDEAL_LOADS_ZONE_COOLING_FUEL_ENERGY_RATE
    )
}

fn evaluate_meter_rows(
    manifest: &ConformanceCase,
    model: &SimulationModel,
    mtr: &Path,
    system_name: &str,
    observed_by_variable: &BTreeMap<(String, String), ObservedSeries>,
    timestamps: &[Option<String>],
) -> Result<Vec<IdealLoadsMeterDiagnosticRow>, String> {
    let meter_requests = manifest
        .meters
        .iter()
        .map(runtime_meter_request_for_manifest_meter)
        .collect::<Result<Vec<_>, String>>()?;
    let meter_registry = RuntimeOutputRegistry::from_model(model);
    let meter_resolution = meter_registry
        .meter_registry()
        .resolve_meter_requests(&meter_requests);
    if meter_resolution.diagnostics.has_errors() {
        let diagnostics = meter_resolution
            .diagnostics
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        return Err(format!(
            "IdealLoads diagnostic meter aggregation requires MeterRegistry-resolved meters: {diagnostics}"
        ));
    }
    let fuel_energy_bindings = ideal_loads_meter_fuel_energy_bindings(model);
    let mut rows = Vec::new();
    for meter in &manifest.meters {
        let resolved_meter = meter_resolution
            .resolved
            .iter()
            .find(|resolved| resolved.request.name.eq_ignore_ascii_case(&meter.name))
            .ok_or_else(|| {
                format!(
                    "IdealLoads diagnostic meter aggregation did not resolve {} through MeterRegistry",
                    meter.name
                )
            })?;
        let expected = load_meter_series(mtr, &meter.name, meter.frequency)?;
        let resolved_meter_key = NormalizedName::new(&resolved_meter.definition.name).0;
        let fuel_energy_variable = fuel_energy_bindings
            .get(&resolved_meter_key)
            .copied()
            .ok_or_else(|| {
                format!(
                    "IdealLoads diagnostic meter aggregation has no fuel-energy binding for MeterRegistry meter {}",
                    resolved_meter.definition.name
                )
            })?;
        let Some(observed) =
            observed_by_variable.get(&(system_name.to_string(), fuel_energy_variable.to_string()))
        else {
            return Err(format!(
                "IdealLoads diagnostic report cannot produce Rust meter source series for {} from {}",
                meter.name, fuel_energy_variable
            ));
        };
        let observed_samples =
            meter_samples_from_detailed_energy(&observed.values, timestamps, meter.frequency)?;
        let tolerance = tolerance_for_meter(meter);
        let max_rmse_tolerance = meter.rmse_tol;
        let comparison = compare_series_samples_v2(&expected.samples, &observed_samples, tolerance);
        let mean_abs_delta = mean_abs_delta(&expected.samples, &observed_samples);
        let status = if comparison.status == SeriesComparisonStatus::Pass
            && max_rmse_tolerance.is_none_or(|max_rmse| comparison.rmse_delta <= max_rmse)
        {
            SeriesComparisonStatus::Pass
        } else {
            SeriesComparisonStatus::Fail
        };

        rows.push(IdealLoadsMeterDiagnosticRow {
            name: meter.name.clone(),
            frequency: meter.frequency,
            source: meter.source,
            domain: meter.domain,
            level: meter.level,
            units: observed.units.to_string(),
            oracle_units: expected.units.clone(),
            rust_source: facility_meter_rust_source(meter.frequency),
            tolerance,
            max_rmse_tolerance,
            expected_samples: comparison.expected_samples,
            observed_samples: comparison.observed_samples,
            compared_samples: comparison.compared_samples,
            max_abs_delta: comparison.max_abs_delta,
            mean_abs_delta,
            rmse_delta: comparison.rmse_delta,
            max_rel_delta: comparison.max_rel_delta,
            alignment: comparison.alignment,
            first_divergence: comparison.first_divergence,
            status,
        });
    }

    Ok(rows)
}

fn runtime_meter_request_for_manifest_meter(
    meter: &MeterRequest,
) -> Result<RuntimeMeterRequest, String> {
    let frequency = runtime_meter_frequency(meter.frequency).ok_or_else(|| {
        format!(
            "IdealLoads diagnostic meter aggregation supports hourly, monthly, annual, and run-period meters, got {} for {}",
            output_frequency_label(meter.frequency),
            meter.name
        )
    })?;
    Ok(RuntimeMeterRequest::new(meter.name.clone(), frequency))
}

fn runtime_meter_frequency(frequency: OutputFrequency) -> Option<RuntimeOutputFrequency> {
    match frequency {
        OutputFrequency::Hourly => Some(RuntimeOutputFrequency::Hourly),
        OutputFrequency::Monthly => Some(RuntimeOutputFrequency::Monthly),
        OutputFrequency::Annual => Some(RuntimeOutputFrequency::Annual),
        OutputFrequency::RunPeriod => Some(RuntimeOutputFrequency::RunPeriod),
        OutputFrequency::Static
        | OutputFrequency::Detailed
        | OutputFrequency::Timestep
        | OutputFrequency::Daily => None,
    }
}

fn load_meter_series(
    mtr: &Path,
    meter: &str,
    frequency: OutputFrequency,
) -> Result<LoadedSeries, String> {
    let frequency_label = output_frequency_idf_label_for_mtr(frequency)?;
    let series =
        load_mtr_time_series_for_frequency(mtr, meter, frequency_label).map_err(|error| {
            format!(
                "failed to load MTR meter {} ({}): {}",
                meter,
                output_frequency_label(frequency),
                error
            )
        })?;
    Ok(LoadedSeries {
        units: series.metadata.units,
        samples: run_period_samples(series.samples),
    })
}

fn output_frequency_idf_label_for_mtr(frequency: OutputFrequency) -> Result<&'static str, String> {
    match frequency {
        OutputFrequency::Hourly => Ok("Hourly"),
        OutputFrequency::Monthly => Ok("Monthly"),
        OutputFrequency::Annual => Ok("Annual"),
        OutputFrequency::RunPeriod => Ok("RunPeriod"),
        _ => Err(format!(
            "IdealLoads MTR meter loading does not support {} frequency",
            output_frequency_label(frequency)
        )),
    }
}

fn ideal_loads_meter_fuel_energy_bindings(
    model: &SimulationModel,
) -> BTreeMap<String, &'static str> {
    let mut bindings = BTreeMap::new();
    for system in &model.typed.ideal_loads_air_systems {
        for fuel_type in [system.heating_fuel_type, system.cooling_fuel_type] {
            if let Some(binding) = ideal_loads_facility_meter_binding(fuel_type) {
                bindings.insert(
                    NormalizedName::new(binding.meter_name).0,
                    binding.fuel_energy_variable,
                );
            }
        }
    }
    bindings
}

fn meter_samples_from_detailed_energy(
    values: &[f64],
    timestamps: &[Option<String>],
    frequency: OutputFrequency,
) -> Result<Vec<SeriesSample>, String> {
    match frequency {
        OutputFrequency::Hourly => hourly_meter_samples_from_detailed_energy(values, timestamps),
        OutputFrequency::Monthly => monthly_meter_samples_from_detailed_energy(values, timestamps),
        OutputFrequency::Annual => Ok(vec![SeriesSample {
            index: 0,
            timestamp: None,
            value: values.iter().sum(),
        }]),
        OutputFrequency::RunPeriod => Ok(vec![SeriesSample {
            index: 0,
            timestamp: None,
            value: values.iter().sum(),
        }]),
        _ => Err(format!(
            "IdealLoads meter aggregation does not support {} frequency",
            output_frequency_label(frequency)
        )),
    }
}

fn hourly_meter_samples_from_detailed_energy(
    values: &[f64],
    timestamps: &[Option<String>],
) -> Result<Vec<SeriesSample>, String> {
    let mut hourly_values = Vec::<(String, f64)>::new();
    for (index, value) in values.iter().copied().enumerate() {
        let timestamp = timestamps
            .get(index)
            .and_then(|timestamp| timestamp.as_deref())
            .ok_or_else(|| {
                format!(
                    "IdealLoads meter diagnostic requires timestamped detailed fuel energy sample {index}"
                )
            })?;
        let hourly_timestamp = hourly_meter_timestamp_label(timestamp).ok_or_else(|| {
            format!("IdealLoads meter diagnostic cannot derive hourly timestamp from {timestamp}")
        })?;
        if let Some((_, total)) = hourly_values
            .iter_mut()
            .find(|(candidate, _)| candidate == &hourly_timestamp)
        {
            *total += value;
        } else {
            hourly_values.push((hourly_timestamp, value));
        }
    }

    Ok(hourly_values
        .into_iter()
        .enumerate()
        .map(|(index, (timestamp, value))| SeriesSample::timestamped(index, timestamp, value))
        .collect())
}

fn monthly_meter_samples_from_detailed_energy(
    values: &[f64],
    timestamps: &[Option<String>],
) -> Result<Vec<SeriesSample>, String> {
    let mut monthly_values = Vec::<(String, f64)>::new();
    for (index, value) in values.iter().copied().enumerate() {
        let timestamp = timestamps
            .get(index)
            .and_then(|timestamp| timestamp.as_deref())
            .ok_or_else(|| {
                format!(
                    "IdealLoads meter diagnostic requires timestamped detailed fuel energy sample {index}"
                )
            })?;
        let monthly_timestamp = monthly_meter_timestamp_label(timestamp).ok_or_else(|| {
            format!("IdealLoads meter diagnostic cannot derive monthly timestamp from {timestamp}")
        })?;
        if let Some((_, total)) = monthly_values
            .iter_mut()
            .find(|(candidate, _)| candidate == &monthly_timestamp)
        {
            *total += value;
        } else {
            monthly_values.push((monthly_timestamp, value));
        }
    }

    Ok(monthly_values
        .into_iter()
        .enumerate()
        .map(|(index, (_, value))| SeriesSample {
            index,
            timestamp: None,
            value,
        })
        .collect())
}

fn hourly_meter_timestamp_label(timestamp: &str) -> Option<String> {
    Some(format!(
        "env={};day={};month={};date={};dst={};hour={};start=0.00;end=60.00;day_type={}",
        timestamp_field(timestamp, "env")?,
        timestamp_field(timestamp, "day")?,
        timestamp_field(timestamp, "month")?,
        timestamp_field(timestamp, "date")?,
        timestamp_field(timestamp, "dst")?,
        timestamp_field(timestamp, "hour")?,
        timestamp_field(timestamp, "day_type")?
    ))
}

fn monthly_meter_timestamp_label(timestamp: &str) -> Option<String> {
    Some(format!(
        "env={};month={}",
        timestamp_field(timestamp, "env")?,
        timestamp_field(timestamp, "month")?
    ))
}

fn facility_meter_rust_source(frequency: OutputFrequency) -> &'static str {
    match frequency {
        OutputFrequency::Hourly => IDEAL_LOADS_FACILITY_METER_HOURLY_RUST_SOURCE,
        OutputFrequency::Monthly => IDEAL_LOADS_FACILITY_METER_MONTHLY_RUST_SOURCE,
        OutputFrequency::Annual => IDEAL_LOADS_FACILITY_METER_ANNUAL_RUST_SOURCE,
        OutputFrequency::RunPeriod => IDEAL_LOADS_FACILITY_METER_RUN_PERIOD_RUST_SOURCE,
        _ => IDEAL_LOADS_FACILITY_METER_HOURLY_RUST_SOURCE,
    }
}

fn timestamp_field<'a>(timestamp: &'a str, name: &str) -> Option<&'a str> {
    for field in timestamp.split(';') {
        let Some((key, value)) = field.split_once('=') else {
            continue;
        };
        if key.trim().eq_ignore_ascii_case(name) {
            return Some(value.trim());
        }
    }
    None
}

fn tolerance_for_meter(meter: &MeterRequest) -> Tolerance {
    Tolerance {
        absolute: meter.abs_tol.unwrap_or(0.0),
        relative: meter.rel_tol.unwrap_or(0.0),
    }
}

fn ideal_loads_fuel_efficiency_context(
    model: &SimulationModel,
    system: &IdealLoadsAirSystem,
    input_trace: &IdealLoadsInputTrace,
) -> Result<IdealLoadsFuelEfficiencyContext, String> {
    let timestamps = input_trace
        .zone_node_temperature
        .samples
        .iter()
        .take(input_trace.sample_count)
        .map(|sample| sample.timestamp.clone())
        .collect::<Vec<_>>();
    let heating = ideal_loads_fuel_efficiency_values(
        model,
        system.heating_fuel_efficiency_schedule,
        "heating",
        input_trace.sample_count,
        &timestamps,
    )?;
    let cooling = ideal_loads_fuel_efficiency_values(
        model,
        system.cooling_fuel_efficiency_schedule,
        "cooling",
        input_trace.sample_count,
        &timestamps,
    )?;
    if system.heating_fuel_efficiency_schedule.is_none()
        && system.cooling_fuel_efficiency_schedule.is_none()
    {
        Ok(IdealLoadsFuelEfficiencyContext::blank(
            input_trace.sample_count,
        ))
    } else if heating.is_constant && cooling.is_constant {
        Ok(IdealLoadsFuelEfficiencyContext::constant(
            heating.representative,
            cooling.representative,
            input_trace.sample_count,
        ))
    } else {
        Ok(IdealLoadsFuelEfficiencyContext::non_constant(
            heating.values,
            cooling.values,
        ))
    }
}

struct IdealLoadsFuelEfficiencyValues {
    values: Vec<f64>,
    representative: f64,
    is_constant: bool,
}

fn ideal_loads_fuel_efficiency_values(
    model: &SimulationModel,
    schedule_id: Option<ScheduleId>,
    label: &str,
    sample_count: usize,
    timestamps: &[Option<String>],
) -> Result<IdealLoadsFuelEfficiencyValues, String> {
    let Some(schedule_id) = schedule_id else {
        return Ok(IdealLoadsFuelEfficiencyValues {
            values: vec![1.0; sample_count],
            representative: 1.0,
            is_constant: true,
        });
    };
    if let Some(schedule) = model
        .typed
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
    {
        validate_fuel_efficiency_value(schedule.hourly_value, label, &schedule.name.0)?;
        return Ok(IdealLoadsFuelEfficiencyValues {
            values: vec![schedule.hourly_value; sample_count],
            representative: schedule.hourly_value,
            is_constant: true,
        });
    }
    let schedule = model
        .typed
        .compact_schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .ok_or_else(|| {
            format!(
                "IdealLoads {label} fuel energy diagnostic supports blank, Schedule:Constant, or calendar-invariant Schedule:Compact fuel efficiency schedules"
            )
        })?;
    let segments = hour_only_single_period_compact_schedule_segments(schedule).map_err(|reason| {
        format!(
            "IdealLoads {label} fuel energy diagnostic rejects calendar-varying Schedule:Compact {}: {reason}",
            schedule.name.0
        )
    })?;
    let mut values = Vec::with_capacity(sample_count);
    for index in 0..sample_count {
        let timestamp = timestamps
            .get(index)
            .and_then(|timestamp| timestamp.as_deref());
        let minute_of_day = minute_of_day_from_timestamp(timestamp).ok_or_else(|| {
            format!(
                "IdealLoads {label} Schedule:Compact fuel efficiency requires timestamped detailed sample {index}"
            )
        })?;
        let value = compact_schedule_value(segments, minute_of_day).ok_or_else(|| {
            format!(
                "IdealLoads {label} fuel efficiency schedule {} has no value for minute {}",
                schedule.name.0, minute_of_day
            )
        })?;
        validate_fuel_efficiency_value(value, label, &schedule.name.0)?;
        values.push(value);
    }
    let representative = values.first().copied().unwrap_or(1.0);
    let is_constant = values
        .iter()
        .all(|value| (*value - representative).abs() <= f64::EPSILON);
    Ok(IdealLoadsFuelEfficiencyValues {
        values,
        representative,
        is_constant,
    })
}

fn validate_fuel_efficiency_value(
    value: f64,
    label: &str,
    schedule_name: &str,
) -> Result<(), String> {
    if !value.is_finite() || value <= 0.0 {
        return Err(format!(
            "IdealLoads {label} fuel efficiency schedule {} must have a positive finite value, got {}",
            schedule_name, value
        ));
    }
    Ok(())
}

fn minute_of_day_from_timestamp(timestamp: Option<&str>) -> Option<u32> {
    let timestamp = timestamp?;
    let hour = timestamp_numeric_field(timestamp, "hour")?;
    let end_minute = timestamp_numeric_field(timestamp, "end")?;
    if !hour.is_finite() || !end_minute.is_finite() {
        return None;
    }
    let hour = hour.round().clamp(1.0, 24.0);
    let minute = ((hour - 1.0) * 60.0 + end_minute)
        .round()
        .clamp(1.0, 1440.0);
    Some(minute as u32)
}

fn compact_schedule_value(
    segments: &[ep_model::ScheduleCompactSegment],
    minute_of_day: u32,
) -> Option<f64> {
    segments
        .iter()
        .find(|segment| minute_of_day <= segment.until_minute_of_day)
        .map(|segment| segment.value)
        .or_else(|| segments.last().map(|segment| segment.value))
}

fn ideal_loads_limit_context(
    model: &SimulationModel,
    system: &IdealLoadsAirSystem,
) -> Result<IdealLoadsSensibleLimitContext, String> {
    if !uses_finite_limits(system) {
        return Ok(model
            .typed
            .site
            .as_ref()
            .and_then(|site| {
                IdealLoadsSensibleLimitContext::from_site_elevation_m(site.elevation_m)
            })
            .unwrap_or_default());
    }

    let site =
        model.typed.site.as_ref().ok_or_else(|| {
            "IdealLoads finite-limit diagnostics require Site:Location".to_string()
        })?;
    IdealLoadsSensibleLimitContext::from_site_elevation_m(site.elevation_m).ok_or_else(|| {
        format!(
            "failed to derive EnergyPlus StdRhoAir from site elevation {}",
            site.elevation_m
        )
    })
}

fn ideal_loads_barometric_pressure_trace(
    model: &SimulationModel,
    weather: Option<&Path>,
    input_trace: &IdealLoadsInputTrace,
    limit_context: IdealLoadsSensibleLimitContext,
) -> Result<Vec<f64>, String> {
    if let Some(series) = input_trace.site_barometric_pressure.as_ref() {
        return Ok(series
            .samples
            .iter()
            .take(input_trace.sample_count)
            .map(|sample| sample.value)
            .collect());
    }
    ideal_loads_barometric_pressure_samples(
        model,
        weather,
        &input_trace.zone_node_temperature.samples,
        input_trace.sample_count,
        limit_context,
    )
}

fn ideal_loads_barometric_pressure_samples(
    model: &SimulationModel,
    weather: Option<&Path>,
    samples: &[SeriesSample],
    sample_count: usize,
    limit_context: IdealLoadsSensibleLimitContext,
) -> Result<Vec<f64>, String> {
    let Some(weather) = weather else {
        return Ok(vec![limit_context.barometric_pressure_pa; sample_count]);
    };
    let weather_records =
        load_epw_records(weather).map_err(|error| format!("failed to load EPW: {error}"))?;
    if weather_records.is_empty() {
        return Ok(vec![limit_context.barometric_pressure_pa; sample_count]);
    }

    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    let first_hour_interpolation_starting_values = model
        .typed
        .run_periods
        .first()
        .map(|run_period| run_period.first_hour_interpolation_starting_values)
        .unwrap_or_default();
    Ok(samples
        .iter()
        .take(sample_count)
        .map(|sample| {
            sample
                .timestamp
                .as_deref()
                .and_then(parse_ideal_loads_timestamp)
                .and_then(|timestamp| {
                    ideal_loads_weather_pressure_for_timestamp(
                        &weather_records,
                        timestamp,
                        zone_steps_per_hour,
                        first_hour_interpolation_starting_values,
                    )
                })
                .unwrap_or(limit_context.barometric_pressure_pa)
        })
        .collect())
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct IdealLoadsTimestampFields {
    month: u32,
    day_of_month: u32,
    hour: u32,
    end_minute: f64,
}

fn parse_ideal_loads_timestamp(timestamp: &str) -> Option<IdealLoadsTimestampFields> {
    let mut month = None;
    let mut day_of_month = None;
    let mut hour = None;
    let mut end_minute = None;
    for field in timestamp.split(';') {
        let (key, value) = field.split_once('=')?;
        match key.trim() {
            "month" => month = value.trim().parse::<u32>().ok(),
            "date" => day_of_month = value.trim().parse::<u32>().ok(),
            "hour" => hour = value.trim().parse::<u32>().ok(),
            "end" => end_minute = value.trim().parse::<f64>().ok(),
            _ => {}
        }
    }
    Some(IdealLoadsTimestampFields {
        month: month?,
        day_of_month: day_of_month?,
        hour: hour?,
        end_minute: end_minute?,
    })
}

fn ideal_loads_weather_pressure_for_timestamp(
    weather_records: &[EpwRecord],
    timestamp: IdealLoadsTimestampFields,
    zone_steps_per_hour: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> Option<f64> {
    let record_index = weather_records.iter().position(|record| {
        record.month == timestamp.month
            && record.day == timestamp.day_of_month
            && record.hour == timestamp.hour
    })?;
    let record = weather_records.get(record_index)?;
    let previous_record = previous_ideal_loads_weather_record(
        weather_records,
        record_index,
        first_hour_interpolation_starting_values,
    )?;
    let weight = ideal_loads_weather_interpolation_weight(
        zone_steps_per_hour,
        ideal_loads_zone_timestep(timestamp.end_minute, zone_steps_per_hour),
    );
    Some(
        previous_record.atmospheric_pressure_pa * (1.0 - weight)
            + record.atmospheric_pressure_pa * weight,
    )
}

fn previous_ideal_loads_weather_record(
    weather_records: &[EpwRecord],
    record_index: usize,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> Option<&EpwRecord> {
    if record_index == 0 {
        let first_day_record_index = match first_hour_interpolation_starting_values {
            FirstHourInterpolationStartingValues::Hour1 => 0,
            FirstHourInterpolationStartingValues::Hour24 => weather_records.len().min(24) - 1,
        };
        weather_records.get(first_day_record_index)
    } else {
        weather_records.get(record_index - 1)
    }
}

fn ideal_loads_zone_timestep(end_minute: f64, zone_steps_per_hour: u32) -> u32 {
    let steps = zone_steps_per_hour.max(1);
    let minutes_per_step = 60.0 / f64::from(steps);
    (end_minute / minutes_per_step)
        .round()
        .clamp(1.0, f64::from(steps)) as u32
}

fn ideal_loads_weather_interpolation_weight(zone_steps_per_hour: u32, zone_timestep: u32) -> f64 {
    let steps = zone_steps_per_hour.max(1);
    if steps == 1 {
        return 1.0;
    }
    (f64::from(zone_timestep.clamp(1, steps)) / f64::from(steps)).min(1.0)
}

fn ideal_loads_recirculation_node_name(
    model: &SimulationModel,
    zone_id: ep_model::ZoneId,
    system: &IdealLoadsAirSystem,
) -> Result<String, String> {
    if let Some(exhaust_node_name) = system.zone_exhaust_air_node_name.as_ref() {
        return resolve_first_node_or_list_name(model, &exhaust_node_name.0).ok_or_else(|| {
            format!(
                "failed to resolve IdealLoads exhaust/recirculation node {}",
                exhaust_node_name.0
            )
        });
    }

    let connection = model
        .typed
        .zone_equipment_connections
        .iter()
        .find(|connection| connection.zone == zone_id)
        .ok_or_else(|| {
            "missing ZoneHVAC:EquipmentConnections for finite-limit recirculation node".to_string()
        })?;
    let Some(return_node_name) = connection.zone_return_air_node_or_nodelist_name.as_ref() else {
        return Err(
            "finite-limit IdealLoads diagnostic requires a zone return air node or node list"
                .to_string(),
        );
    };
    resolve_first_node_or_list_name(model, &return_node_name.0).ok_or_else(|| {
        format!(
            "failed to resolve IdealLoads return/recirculation node {}",
            return_node_name.0
        )
    })
}

fn manifest_requests_ideal_loads_recirculation_node(
    manifest: &ConformanceCase,
    model: &SimulationModel,
    zone_id: ep_model::ZoneId,
    system: &IdealLoadsAirSystem,
) -> Result<bool, String> {
    if !uses_ideal_loads_humidity_control(system) {
        return Ok(false);
    }
    let recirculation_node_name = ideal_loads_recirculation_node_name(model, zone_id, system)?;
    Ok(manifest
        .outputs
        .iter()
        .any(|output| output.key.eq_ignore_ascii_case(&recirculation_node_name)))
}

fn manifest_allows_constant_supply_humidity_diagnostic(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    !manifest.conformance_claim
        && system.dehumidification_control_type
            == DehumidificationControlType::ConstantSupplyHumidityRatio
        && system.humidification_control_type == HumidificationControlType::None
        && manifest.outputs.iter().any(|output| {
            output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE
                || output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE
        })
}

fn manifest_allows_constant_supply_humidity_cooling_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001"
        && system.dehumidification_control_type
            == DehumidificationControlType::ConstantSupplyHumidityRatio
        && system.humidification_control_type == HumidificationControlType::None
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE
        })
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE
        })
}

fn manifest_allows_constant_supply_humidity_cooling_annual_meter_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest_is_humidity_annual_facility_meter_conformance_candidate(manifest)
        && manifest.id
            == IDEAL_LOADS_CONSTANT_SUPPLY_HUMIDITY_COOLING_ANNUAL_METER_CONFORMANCE_CASE_ID
        && system.dehumidification_control_type
            == DehumidificationControlType::ConstantSupplyHumidityRatio
        && system.humidification_control_type == HumidificationControlType::None
}

fn manifest_allows_humidistat_dehumidification_diagnostic(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    !manifest.conformance_claim
        && system.dehumidification_control_type == DehumidificationControlType::Humidistat
        && system.humidification_control_type == HumidificationControlType::None
        && manifest.outputs.iter().any(|output| {
            output.variable == ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD
                || output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE
                || output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE
        })
}

fn manifest_allows_humidistat_dehumidification_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_humidistat_dehumidification_conformance_candidate_001"
        && system.dehumidification_control_type == DehumidificationControlType::Humidistat
        && system.humidification_control_type == HumidificationControlType::None
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE
        })
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE
        })
}

fn manifest_promotes_humidistat_moisture_predictor(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest_allows_humidistat_dehumidification_conformance(manifest, system)
        || manifest_allows_humidistat_humidification_conformance(manifest, system)
}

fn manifest_allows_humidistat_dehumidification_annual_meter_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest_is_humidity_annual_facility_meter_conformance_candidate(manifest)
        && manifest.id == IDEAL_LOADS_HUMIDISTAT_DEHUMIDIFICATION_ANNUAL_METER_CONFORMANCE_CASE_ID
        && system.dehumidification_control_type == DehumidificationControlType::Humidistat
        && system.humidification_control_type == HumidificationControlType::None
}

fn manifest_allows_humidistat_humidification_diagnostic(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    !manifest.conformance_claim
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::Humidistat
        && manifest.outputs.iter().any(|output| {
            output.variable == ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD
                || output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE
                || output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE
        })
}

fn manifest_allows_humidistat_humidification_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_humidistat_humidification_conformance_candidate_001"
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::Humidistat
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE
        })
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE
        })
}

fn manifest_allows_humidistat_humidification_annual_meter_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest_is_humidity_annual_facility_meter_conformance_candidate(manifest)
        && manifest.id == IDEAL_LOADS_HUMIDISTAT_HUMIDIFICATION_ANNUAL_METER_CONFORMANCE_CASE_ID
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::Humidistat
}

fn manifest_allows_constant_supply_humidity_humidification_diagnostic(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    !manifest.conformance_claim
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type
            == HumidificationControlType::ConstantSupplyHumidityRatio
        && manifest.outputs.iter().any(|output| {
            output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE
                || output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE
        })
}

fn manifest_allows_constant_supply_humidity_heating_annual_meter_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest_is_humidity_annual_facility_meter_conformance_candidate(manifest)
        && manifest.id
            == IDEAL_LOADS_CONSTANT_SUPPLY_HUMIDITY_HEATING_ANNUAL_METER_CONFORMANCE_CASE_ID
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type
            == HumidificationControlType::ConstantSupplyHumidityRatio
}

fn manifest_allows_humidity_annual_facility_meter_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest_allows_constant_supply_humidity_cooling_annual_meter_conformance(manifest, system)
        || manifest_allows_constant_supply_humidity_heating_annual_meter_conformance(
            manifest, system,
        )
        || manifest_allows_humidistat_dehumidification_annual_meter_conformance(manifest, system)
        || manifest_allows_humidistat_humidification_annual_meter_conformance(manifest, system)
}

fn manifest_allows_constant_supply_humidity_heating_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_constant_supply_humidity_heating_conformance_candidate_001"
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type
            == HumidificationControlType::ConstantSupplyHumidityRatio
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE
        })
        && manifest.outputs.iter().any(|output| {
            output.level == Some(OutputLevel::Conformance)
                && output.variable == ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE
        })
}

fn manifest_allows_finite_limit_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    if !manifest.conformance_claim {
        return false;
    }

    match manifest.id.as_str() {
        "ideal_loads_capacity_limit_conformance_001" => {
            system.heating_limit == IdealLoadsLimit::LimitCapacity
                && system.cooling_limit == IdealLoadsLimit::LimitCapacity
        }
        "ideal_loads_flow_limit_conformance_001" => {
            system.heating_limit == IdealLoadsLimit::LimitFlowRate
                && system.cooling_limit == IdealLoadsLimit::LimitFlowRate
        }
        "ideal_loads_flow_capacity_limit_conformance_001" => {
            system.heating_limit == IdealLoadsLimit::LimitFlowRateAndCapacity
                && system.cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity
        }
        _ => false,
    }
}

fn manifest_allows_constant_shr_conformance(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> bool {
    manifest.conformance_claim
        && manifest.id == "ideal_loads_constant_shr_conformance_001"
        && system.dehumidification_control_type
            == DehumidificationControlType::ConstantSensibleHeatRatio
        && system.humidification_control_type == HumidificationControlType::None
}

fn resolve_first_node_or_list_name(model: &SimulationModel, name: &str) -> Option<String> {
    if let Some(node_id) = model.typed.node_names.resolve(name) {
        return model
            .typed
            .nodes
            .iter()
            .find(|node| node.id == node_id)
            .map(|node| node.name.0.clone());
    }
    let node_list_id = model.typed.node_list_names.resolve(name)?;
    let node_list = model
        .typed
        .node_lists
        .iter()
        .find(|node_list| node_list.id == node_list_id)?;
    let node_id = node_list.nodes.first()?;
    model
        .typed
        .nodes
        .iter()
        .find(|node| node.id == *node_id)
        .map(|node| node.name.0.clone())
}

fn ideal_loads_sensible_branch(system: &IdealLoadsAirSystem) -> &'static str {
    if uses_finite_limits(system) {
        "no-oa-finite-limit-sensible"
    } else {
        "no-oa-no-limit-sensible"
    }
}

fn inactive_ideal_loads_branches(system: &IdealLoadsAirSystem) -> Vec<&'static str> {
    let mut branches = Vec::new();
    if !uses_outdoor_air(system) {
        branches.push("outdoor_air");
    }
    if system.outdoor_air_economizer_type == OutdoorAirEconomizerType::NoEconomizer {
        branches.push("economizer");
    }
    if system.heat_recovery_type == HeatRecoveryType::None {
        branches.push("heat_recovery");
    }
    if system.dehumidification_control_type != DehumidificationControlType::Humidistat
        && system.humidification_control_type != HumidificationControlType::Humidistat
    {
        branches.push("humidistat");
    }
    if system.demand_controlled_ventilation_type == DemandControlledVentilationType::None {
        branches.push("dcv");
    }
    if !uses_autosizing(system) {
        branches.push("autosizing");
    }
    branches.push("saturation_limit");
    branches
}

fn declared_ideal_loads_branch(
    manifest: &ConformanceCase,
    system: &IdealLoadsAirSystem,
) -> &'static str {
    if manifest.id.contains("constant_supply_humidity_heating") {
        "constant_supply_humidity_heating"
    } else if manifest.id.contains("constant_supply_humidity") {
        "constant_supply_humidity_cooling"
    } else if manifest.id.contains("humidistat_dehumidification") {
        "humidistat_dehumidification"
    } else if manifest.id.contains("humidistat_humidification") {
        "humidistat_humidification"
    } else if manifest.id.contains("constant_shr") {
        "constant_shr"
    } else if manifest.id.contains("flow_capacity_limit") {
        "flow_and_capacity"
    } else if manifest.id.contains("flow_limit") {
        "finite_flow"
    } else if manifest.id.contains("capacity_limit") {
        "finite_capacity"
    } else if uses_finite_limits(system) {
        select_purchased_air_branch(system).label()
    } else {
        "no_oa_sensible"
    }
}

fn ideal_loads_recirculation_state_source(branch: &str) -> &'static str {
    if branch == "no-oa-finite-limit-sensible" {
        IDEAL_LOADS_FINITE_LIMIT_RECIRCULATION_STATE_SOURCE
    } else {
        IDEAL_LOADS_HUMIDITY_CONTROL_RECIRCULATION_STATE_SOURCE
    }
}

fn rust_result_source(system: &IdealLoadsAirSystem) -> &'static str {
    if uses_finite_limits(system) {
        "rust-ideal-loads-no-oa-sensible-limited-calc"
    } else {
        "rust-ideal-loads-no-oa-sensible-calc"
    }
}

fn uses_finite_limits(system: &IdealLoadsAirSystem) -> bool {
    system.heating_limit != IdealLoadsLimit::NoLimit
        || system.cooling_limit != IdealLoadsLimit::NoLimit
}

fn uses_outdoor_air(system: &IdealLoadsAirSystem) -> bool {
    system
        .design_specification_outdoor_air_object_name
        .is_some()
        || system.outdoor_air_inlet_node_name.is_some()
}

fn uses_autosizing(system: &IdealLoadsAirSystem) -> bool {
    system
        .design_specification_zonehvac_sizing_object_name
        .is_some()
        || matches!(
            system.maximum_heating_air_flow_rate_m3_per_s,
            Some(AutosizeOrNumber::Autosize)
        )
        || matches!(
            system.maximum_sensible_heating_capacity_w,
            Some(AutosizeOrNumber::Autosize)
        )
        || matches!(
            system.maximum_cooling_air_flow_rate_m3_per_s,
            Some(AutosizeOrNumber::Autosize)
        )
        || matches!(
            system.maximum_total_cooling_capacity_w,
            Some(AutosizeOrNumber::Autosize)
        )
}

fn uses_ideal_loads_humidity_control(system: &IdealLoadsAirSystem) -> bool {
    system.dehumidification_control_type != DehumidificationControlType::None
        || system.humidification_control_type != HumidificationControlType::None
}

fn uses_humidistat_control(system: &IdealLoadsAirSystem) -> bool {
    matches!(
        system.dehumidification_control_type,
        DehumidificationControlType::Humidistat
    ) || matches!(
        system.humidification_control_type,
        HumidificationControlType::Humidistat
    )
}

fn moisture_predictor_summary(
    model: &SimulationModel,
    system: &IdealLoadsAirSystem,
    zone: &Zone,
    eso: &Path,
    input_trace: &IdealLoadsInputTrace,
    supply_node: ep_model::NodeId,
    system_name: &str,
    supply_node_name: &str,
    limit_context: IdealLoadsSensibleLimitContext,
    barometric_pressure_trace: &[f64],
    source_order_trace_uses_recirculation: bool,
    timestep_seconds: f64,
) -> Result<Option<IdealLoadsMoisturePredictorSummary>, String> {
    if !uses_humidistat_control(system) {
        return Ok(None);
    }
    let Some(humidistat) = model
        .typed
        .zone_humidistats
        .iter()
        .find(|humidistat| humidistat.zone == zone.id)
    else {
        return Ok(None);
    };

    let timestamps = input_trace
        .zone_node_temperature
        .samples
        .iter()
        .take(input_trace.sample_count)
        .map(|sample| sample.timestamp.clone())
        .collect::<Vec<_>>();
    let humidifying_rh = ideal_loads_optional_schedule_values(
        model,
        Some(humidistat.humidifying_relative_humidity_setpoint_schedule),
        "Humidistat humidifying relative-humidity",
        input_trace.sample_count,
        &timestamps,
    )?;
    let dehumidifying_rh = ideal_loads_optional_schedule_values(
        model,
        Some(humidistat.dehumidifying_relative_humidity_setpoint_schedule),
        "Humidistat dehumidifying relative-humidity",
        input_trace.sample_count,
        &timestamps,
    )?;
    let latent_gain = ideal_loads_other_equipment_latent_gain_values(
        model,
        zone,
        input_trace.sample_count,
        &timestamps,
    )?;
    let latent_gain_comparisons = [
        ZONE_OTHER_EQUIPMENT_LATENT_GAIN_RATE,
        ZONE_TOTAL_INTERNAL_LATENT_GAIN_RATE,
    ]
    .into_iter()
    .filter_map(|variable| {
        load_optional_series(eso, &zone.name.0, variable).map(|expected| {
            moisture_predictor_comparison(
                variable,
                &expected,
                &latent_gain,
                input_trace.sample_count,
            )
        })
    })
    .collect::<Vec<_>>();
    let zone_volume_m3 = ideal_loads_zone_volume_m3(&model.typed, zone).ok_or_else(|| {
        format!(
            "IdealLoads Humidistat moisture predictor requires explicit zone volume for {}",
            zone.name.0
        )
    })?;
    let zone_moisture_capacity_multiplier = 1.0;
    let zone_multiplier = f64::from(zone.multiplier.max(1));
    let mut humidifying = Vec::with_capacity(input_trace.sample_count);
    let mut dehumidifying = Vec::with_capacity(input_trace.sample_count);
    for index in 0..input_trace.sample_count {
        let pressure = *barometric_pressure_trace.get(index).ok_or_else(|| {
            format!("IdealLoads Humidistat moisture predictor missing pressure sample {index}")
        })?;
        let zone_state = oracle_zone_predictor_state(input_trace, index);
        let demand =
            calc_no_oa_third_order_moisture_demand_compat(NoOaThirdOrderMoistureDemandInput {
                zone_state,
                previous_zone_timestep_humidity_ratios: oracle_zone_air_humidity_history(
                    input_trace,
                    index,
                ),
                zone_volume_m3,
                zone_moisture_capacity_multiplier,
                timestep_seconds,
                barometric_pressure_pa: pressure,
                latent_gain_w: latent_gain[index],
                humidifying_relative_humidity_percent: humidifying_rh[index],
                dehumidifying_relative_humidity_percent: dehumidifying_rh[index],
                zone_multiplier,
            })
            .ok_or_else(|| {
                format!("IdealLoads Humidistat moisture predictor rejected sample {index}")
            })?;
        humidifying.push(demand.humidifying_setpoint_load_kg_per_s);
        dehumidifying.push(demand.dehumidifying_setpoint_load_kg_per_s);
    }
    let closed_loop = humidistat_closed_loop_comparisons(IdealLoadsHumidistatClosedLoopInput {
        system,
        zone,
        eso,
        input_trace,
        supply_node,
        system_name,
        supply_node_name,
        limit_context,
        barometric_pressure_trace,
        latent_gain: &latent_gain,
        humidifying_rh: &humidifying_rh,
        dehumidifying_rh: &dehumidifying_rh,
        zone_volume_m3,
        zone_moisture_capacity_multiplier,
        zone_multiplier,
        timestep_seconds,
        source_order_trace_uses_recirculation,
    })?;
    let humidifying_history_term = moisture_history_term_comparison(
        MoistureEquivalentHistoryDeltaInput {
            input_trace,
            expected: &input_trace.humidifying_moisture_demand,
            observed: &humidifying,
            barometric_pressure_trace,
            zone_volume_m3,
            zone_moisture_capacity_multiplier,
            timestep_seconds,
            zone_multiplier,
        },
        ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD,
    )?;
    let dehumidifying_history_term = moisture_history_term_comparison(
        MoistureEquivalentHistoryDeltaInput {
            input_trace,
            expected: &input_trace.dehumidifying_moisture_demand,
            observed: &dehumidifying,
            barometric_pressure_trace,
            zone_volume_m3,
            zone_moisture_capacity_multiplier,
            timestep_seconds,
            zone_multiplier,
        },
        ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD,
    )?;

    Ok(Some(IdealLoadsMoisturePredictorSummary {
        promoted_input: false,
        history_source: "EnergyPlus Zone Mean Air Humidity Ratio row lag plus warmup tail seeds WPrevZoneTS; promoted Humidistat branch values then use Rust closed-loop humidity history",
        latent_gain_source: "typed OtherEquipment design_level * fraction_latent * schedule; radiant-system, pool, people, infiltration, mixing, EMS, and fault latent terms are outside this diagnostic",
        closed_loop_state_source: "ep_runtime::advance_no_oa_humidistat_zone_timestep_compat atomically owns each seeded fixed zone-timestep predictor, source-order SimPurchasedAir, correctHumRat, and humidity-history push transition; adaptive or multiple system substeps remain outside this boundary, and EnergyPlus still supplies warmup seed history, sensible demand, temperatures, latent gain schedule, relative-humidity schedules, and pressure",
        history_residual_source: "predictor residual divided by EnergyPlus ThirdOrder C coefficient and zone multiplier; includes any latent-gain, schedule, psychrometric, or WPrevZoneTSTemp mismatch as an equivalent humidity-history delta",
        humidifying_equivalent_history_delta_max: humidifying_history_term
            .row_lag_minus_inferred_max_abs_delta,
        dehumidifying_equivalent_history_delta_max: dehumidifying_history_term
            .row_lag_minus_inferred_max_abs_delta,
        humidifying_history_term,
        dehumidifying_history_term,
        zone_moisture_capacity_multiplier,
        zone_multiplier,
        closed_loop_humidifying_values: closed_loop.humidifying_values,
        closed_loop_dehumidifying_values: closed_loop.dehumidifying_values,
        closed_loop_results: closed_loop.results,
        latent_gain: latent_gain_comparisons,
        humidifying: moisture_predictor_comparison(
            ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD,
            &input_trace.humidifying_moisture_demand,
            &humidifying,
            input_trace.sample_count,
        ),
        dehumidifying: moisture_predictor_comparison(
            ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD,
            &input_trace.dehumidifying_moisture_demand,
            &dehumidifying,
            input_trace.sample_count,
        ),
        closed_loop: closed_loop.comparisons,
    }))
}

struct IdealLoadsHumidistatClosedLoopInput<'a> {
    system: &'a IdealLoadsAirSystem,
    zone: &'a Zone,
    eso: &'a Path,
    input_trace: &'a IdealLoadsInputTrace,
    supply_node: ep_model::NodeId,
    system_name: &'a str,
    supply_node_name: &'a str,
    limit_context: IdealLoadsSensibleLimitContext,
    barometric_pressure_trace: &'a [f64],
    latent_gain: &'a [f64],
    humidifying_rh: &'a [f64],
    dehumidifying_rh: &'a [f64],
    zone_volume_m3: f64,
    zone_moisture_capacity_multiplier: f64,
    zone_multiplier: f64,
    timestep_seconds: f64,
    source_order_trace_uses_recirculation: bool,
}

struct MoistureEquivalentHistoryDeltaInput<'a> {
    input_trace: &'a IdealLoadsInputTrace,
    expected: &'a LoadedSeries,
    observed: &'a [f64],
    barometric_pressure_trace: &'a [f64],
    zone_volume_m3: f64,
    zone_moisture_capacity_multiplier: f64,
    timestep_seconds: f64,
    zone_multiplier: f64,
}

fn moisture_history_term_comparison(
    input: MoistureEquivalentHistoryDeltaInput<'_>,
    demand: impl Into<String>,
) -> Result<IdealLoadsMoistureHistoryTermComparison, String> {
    let mut max_abs_delta = 0.0_f64;
    let mut sum_squared_delta = 0.0_f64;
    let mut sum_delta = 0.0_f64;
    let mut max_abs_row_lag_history_term = 0.0_f64;
    let mut max_abs_inferred_history_term = 0.0_f64;
    let mut largest_delta_sample = None;
    let zone_multiplier = input.zone_multiplier.max(1.0);
    for index in 0..input.input_trace.sample_count {
        let pressure = *input.barometric_pressure_trace.get(index).ok_or_else(|| {
            format!("IdealLoads Humidistat history residual missing pressure sample {index}")
        })?;
        let zone_state = oracle_zone_predictor_state(input.input_trace, index);
        let density = energyplus_moist_air_density_kg_per_m3(
            pressure,
            zone_state.air_temperature_c,
            zone_state.air_humidity_ratio,
        )
        .ok_or_else(|| {
            format!("IdealLoads Humidistat history residual rejected density sample {index}")
        })?;
        let c = density * input.zone_volume_m3 * input.zone_moisture_capacity_multiplier
            / input.timestep_seconds;
        let denominator = c * zone_multiplier;
        if denominator == 0.0 || !denominator.is_finite() {
            return Err(format!(
                "IdealLoads Humidistat history residual has invalid C coefficient at sample {index}"
            ));
        }
        let expected = input.expected.samples[index].value;
        let observed = *input.observed.get(index).ok_or_else(|| {
            format!("IdealLoads Humidistat history residual missing observed sample {index}")
        })?;
        let equivalent_history_delta = -(observed - expected) / denominator;
        if !equivalent_history_delta.is_finite() {
            return Err(format!(
                "IdealLoads Humidistat history residual produced non-finite sample {index}"
            ));
        }
        let row_lag_history_term = third_order_humidity_history_term(
            oracle_zone_air_humidity_history(input.input_trace, index),
        );
        let inferred_history_term = row_lag_history_term - equivalent_history_delta;
        let abs_delta = equivalent_history_delta.abs();
        sum_squared_delta += equivalent_history_delta * equivalent_history_delta;
        sum_delta += equivalent_history_delta;
        max_abs_row_lag_history_term = max_abs_row_lag_history_term.max(row_lag_history_term.abs());
        max_abs_inferred_history_term =
            max_abs_inferred_history_term.max(inferred_history_term.abs());
        if abs_delta > max_abs_delta {
            max_abs_delta = abs_delta;
            largest_delta_sample = Some(IdealLoadsMoistureHistoryTermSample {
                index,
                timestamp: input.expected.samples[index].timestamp.clone(),
                row_lag_history_term,
                inferred_history_term,
                row_lag_minus_inferred_delta: equivalent_history_delta,
            });
        }
    }
    let samples = input.input_trace.sample_count;
    let divisor = samples.max(1) as f64;
    Ok(IdealLoadsMoistureHistoryTermComparison {
        demand: demand.into(),
        samples,
        row_lag_minus_inferred_max_abs_delta: max_abs_delta,
        row_lag_minus_inferred_rmse_delta: (sum_squared_delta / divisor).sqrt(),
        row_lag_minus_inferred_mean_delta: sum_delta / divisor,
        max_abs_row_lag_history_term,
        max_abs_inferred_history_term,
        largest_delta_sample,
    })
}

fn humidistat_closed_loop_comparisons(
    input: IdealLoadsHumidistatClosedLoopInput<'_>,
) -> Result<IdealLoadsHumidistatClosedLoopSummary, String> {
    let sample_count = input.input_trace.sample_count;
    let mut humidifying = Vec::with_capacity(sample_count);
    let mut dehumidifying = Vec::with_capacity(sample_count);
    let mut results = Vec::with_capacity(sample_count);
    let mut corrected_zone_humidity = Vec::with_capacity(sample_count);
    let mut supply_mass_flow = Vec::with_capacity(sample_count);
    let mut supply_humidity = Vec::with_capacity(sample_count);
    let mut zone_latent_heating = Vec::with_capacity(sample_count);
    let mut zone_latent_cooling = Vec::with_capacity(sample_count);
    let mut supply_air_latent_heating = Vec::with_capacity(sample_count);
    let mut supply_air_latent_cooling = Vec::with_capacity(sample_count);
    let mut closed_loop_state = NoOaHumidistatClosedLoopState::from_seed_histories(
        oracle_zone_air_humidity_history(input.input_trace, 0),
        oracle_zone_air_humidity_ratio_history(input.input_trace, 0),
    );

    for index in 0..sample_count {
        let pressure = *input.barometric_pressure_trace.get(index).ok_or_else(|| {
            format!("IdealLoads Humidistat closed-loop missing pressure sample {index}")
        })?;
        let predictor_state = oracle_zone_predictor_state(input.input_trace, index);
        let active_demand = input.input_trace.active_demand.samples[index].value;
        let sensible_demand = ZoneSysEnergyDemand::sensible_only(
            input.zone.id,
            active_demand.max(0.0),
            active_demand.min(0.0),
        );
        let calc_temperature = if input.source_order_trace_uses_recirculation {
            input.input_trace.recirculation_node_temperature.samples[index].value
        } else {
            input.input_trace.zone_node_temperature.samples[index.saturating_sub(1)].value
        };
        let recirculation_temperature = if input.source_order_trace_uses_recirculation {
            input.input_trace.recirculation_node_temperature.samples[index].value
        } else {
            calc_temperature
        };

        let step = advance_no_oa_humidistat_zone_timestep_compat(
            &mut closed_loop_state,
            NoOaHumidistatZoneTimestepInput {
                system: input.system,
                supply_node: input.supply_node,
                sensible_demand,
                predictor_zone_air_temperature_c: predictor_state.air_temperature_c,
                purchased_air_zone_temperature_c: calc_temperature,
                recirculation_air_temperature_c: recirculation_temperature,
                corrector_zone_air_temperature_c: input.input_trace.zone_air_temperature.samples
                    [index]
                    .value,
                zone_volume_m3: input.zone_volume_m3,
                zone_moisture_capacity_multiplier: input.zone_moisture_capacity_multiplier,
                zone_timestep_seconds: input.timestep_seconds,
                barometric_pressure_pa: pressure,
                latent_gain_w: input.latent_gain[index],
                humidifying_relative_humidity_percent: input.humidifying_rh[index],
                dehumidifying_relative_humidity_percent: input.dehumidifying_rh[index],
                zone_multiplier: input.zone_multiplier,
                unit_available: true,
                limit_context: input.limit_context,
            },
        )
        .map_err(|error| match error {
            NoOaHumidistatZoneTimestepError::UnsupportedBranch(branch) => format!(
                "IdealLoads Humidistat closed-loop rejected branch {branch:?} at sample {index}"
            ),
            NoOaHumidistatZoneTimestepError::MoisturePredictorRejected => {
                format!("IdealLoads Humidistat closed-loop predictor rejected sample {index}")
            }
            NoOaHumidistatZoneTimestepError::PurchasedAir(error) => format!(
                "IdealLoads Humidistat closed-loop rejected system {:?}: {:?}",
                error.system_id, error.unsupported_features
            ),
            NoOaHumidistatZoneTimestepError::HumidityCorrectorRejected => {
                format!("IdealLoads Humidistat closed-loop correctHumRat rejected sample {index}")
            }
        })?;
        let predicted = step.moisture_demand;
        humidifying.push(predicted.humidifying_setpoint_load_kg_per_s);
        dehumidifying.push(predicted.dehumidifying_setpoint_load_kg_per_s);

        let result = step.purchased_air.report;
        results.push(result);
        supply_mass_flow.push(result.supply_mass_flow_rate_kg_per_s);
        supply_humidity.push(result.supply_humidity_ratio);
        zone_latent_heating.push(result.zone_latent_heating_rate_w);
        zone_latent_cooling.push(result.zone_latent_cooling_rate_w);
        supply_air_latent_heating.push(result.supply_air_latent_heating_rate_w);
        supply_air_latent_cooling.push(result.supply_air_latent_cooling_rate_w);
        corrected_zone_humidity.push(step.humidity_correction.zone_air_humidity_ratio);
    }

    let mut comparisons = Vec::new();
    comparisons.push(moisture_predictor_comparison(
        format!(
            "{} :: {} (closed loop)",
            input.zone.name.0, ZONE_SYSTEM_PREDICTED_HUMIDIFYING_MOISTURE_LOAD
        ),
        &input.input_trace.humidifying_moisture_demand,
        &humidifying,
        sample_count,
    ));
    comparisons.push(moisture_predictor_comparison(
        format!(
            "{} :: {} (closed loop)",
            input.zone.name.0, ZONE_SYSTEM_PREDICTED_DEHUMIDIFYING_MOISTURE_LOAD
        ),
        &input.input_trace.dehumidifying_moisture_demand,
        &dehumidifying,
        sample_count,
    ));
    comparisons.push(moisture_predictor_comparison(
        format!(
            "{} :: {} (closed-loop corrected)",
            input.zone.name.0, ZONE_AIR_HUMIDITY_RATIO
        ),
        &input.input_trace.zone_air_humidity_ratio,
        &corrected_zone_humidity,
        sample_count,
    ));
    push_optional_closed_loop_comparison(
        &mut comparisons,
        input.eso,
        input.system_name,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
        &supply_mass_flow,
        sample_count,
    );
    push_optional_closed_loop_comparison(
        &mut comparisons,
        input.eso,
        input.system_name,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
        &supply_humidity,
        sample_count,
    );
    push_optional_closed_loop_comparison(
        &mut comparisons,
        input.eso,
        input.system_name,
        ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE,
        &zone_latent_heating,
        sample_count,
    );
    push_optional_closed_loop_comparison(
        &mut comparisons,
        input.eso,
        input.system_name,
        ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
        &zone_latent_cooling,
        sample_count,
    );
    push_optional_closed_loop_comparison(
        &mut comparisons,
        input.eso,
        input.system_name,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE,
        &supply_air_latent_heating,
        sample_count,
    );
    push_optional_closed_loop_comparison(
        &mut comparisons,
        input.eso,
        input.system_name,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
        &supply_air_latent_cooling,
        sample_count,
    );
    push_optional_closed_loop_comparison(
        &mut comparisons,
        input.eso,
        input.supply_node_name,
        SYSTEM_NODE_MASS_FLOW_RATE,
        &supply_mass_flow,
        sample_count,
    );
    push_optional_closed_loop_comparison(
        &mut comparisons,
        input.eso,
        input.supply_node_name,
        SYSTEM_NODE_HUMIDITY_RATIO,
        &supply_humidity,
        sample_count,
    );

    Ok(IdealLoadsHumidistatClosedLoopSummary {
        comparisons,
        humidifying_values: humidifying,
        dehumidifying_values: dehumidifying,
        results,
    })
}

fn push_optional_closed_loop_comparison(
    comparisons: &mut Vec<IdealLoadsMoisturePredictorComparison>,
    eso: &Path,
    key: &str,
    variable: &'static str,
    observed_values: &[f64],
    sample_count: usize,
) {
    if let Ok(expected) = load_series(eso, key, variable) {
        comparisons.push(moisture_predictor_comparison(
            format!("{key} :: {variable} (closed loop)"),
            &expected,
            observed_values,
            sample_count,
        ));
    }
}

fn ideal_loads_other_equipment_latent_gain_values(
    model: &SimulationModel,
    zone: &Zone,
    sample_count: usize,
    timestamps: &[Option<String>],
) -> Result<Vec<f64>, String> {
    let mut values = vec![0.0; sample_count];
    for equipment in model
        .typed
        .other_equipment
        .iter()
        .filter(|equipment| equipment.zone == zone.id)
    {
        let schedule_values = ideal_loads_optional_schedule_values(
            model,
            equipment.schedule,
            &format!("OtherEquipment/{} latent gain", equipment.name.0),
            sample_count,
            timestamps,
        )?;
        for (value, schedule_value) in values.iter_mut().zip(schedule_values.iter()) {
            *value += equipment.design_level_w * equipment.fraction_latent * schedule_value;
        }
    }
    Ok(values)
}

fn oracle_zone_air_humidity_history(input_trace: &IdealLoadsInputTrace, index: usize) -> [f64; 3] {
    [
        oracle_zone_air_humidity_history_value(input_trace, index, 1),
        oracle_zone_air_humidity_history_value(input_trace, index, 2),
        oracle_zone_air_humidity_history_value(input_trace, index, 3),
    ]
}

fn oracle_zone_air_humidity_ratio_history(
    input_trace: &IdealLoadsInputTrace,
    index: usize,
) -> [f64; 3] {
    [
        lagged_trace_value(
            &input_trace.zone_air_humidity_ratio,
            input_trace.zone_air_humidity_ratio_warmup_tail,
            index,
            1,
        ),
        lagged_trace_value(
            &input_trace.zone_air_humidity_ratio,
            input_trace.zone_air_humidity_ratio_warmup_tail,
            index,
            2,
        ),
        lagged_trace_value(
            &input_trace.zone_air_humidity_ratio,
            input_trace.zone_air_humidity_ratio_warmup_tail,
            index,
            3,
        ),
    ]
}

fn oracle_zone_predictor_state(
    input_trace: &IdealLoadsInputTrace,
    index: usize,
) -> IdealLoadsZoneState {
    IdealLoadsZoneState {
        air_temperature_c: lagged_trace_value(
            &input_trace.zone_air_temperature,
            input_trace.zone_air_temperature_warmup_tail,
            index,
            1,
        ),
        air_humidity_ratio: lagged_trace_value(
            &input_trace.zone_air_humidity_ratio,
            input_trace.zone_air_humidity_ratio_warmup_tail,
            index,
            1,
        ),
    }
}

fn oracle_zone_air_humidity_history_value(
    input_trace: &IdealLoadsInputTrace,
    index: usize,
    lag: usize,
) -> f64 {
    lagged_trace_value(
        &input_trace.zone_mean_air_humidity_ratio,
        input_trace.zone_mean_air_humidity_ratio_warmup_tail,
        index,
        lag,
    )
}

fn lagged_trace_value(
    series: &LoadedSeries,
    warmup_tail: Option<[f64; 3]>,
    index: usize,
    lag: usize,
) -> f64 {
    if index < lag {
        let warmup_index = lag - index - 1;
        if let Some(value) = warmup_tail.and_then(|tail| tail.get(warmup_index).copied()) {
            return value;
        }
    }
    series
        .samples
        .get(index.saturating_sub(lag))
        .or_else(|| series.samples.first())
        .map_or(0.0, |sample| sample.value)
}

fn moisture_predictor_comparison(
    variable: impl Into<String>,
    expected: &LoadedSeries,
    observed_values: &[f64],
    sample_count: usize,
) -> IdealLoadsMoisturePredictorComparison {
    let expected_samples = expected
        .samples
        .iter()
        .take(sample_count)
        .cloned()
        .collect::<Vec<_>>();
    let timestamps = expected_samples
        .iter()
        .map(|sample| sample.timestamp.clone())
        .collect::<Vec<_>>();
    let observed_samples = samples_with_timestamps(observed_values, &timestamps);
    let tolerance = Tolerance::default();
    let comparison = compare_series_samples_v2(&expected_samples, &observed_samples, tolerance);
    let status = if comparison.status == SeriesComparisonStatus::Pass
        && comparison.rmse_delta <= tolerance.absolute
    {
        SeriesComparisonStatus::Pass
    } else {
        SeriesComparisonStatus::Fail
    };
    IdealLoadsMoisturePredictorComparison {
        variable: variable.into(),
        samples: comparison.compared_samples,
        max_abs_delta: comparison.max_abs_delta,
        rmse_delta: comparison.rmse_delta,
        max_rel_delta: comparison.max_rel_delta,
        status,
        first_divergence: comparison.first_divergence,
    }
}

struct ObservedSeries {
    source: &'static str,
    units: &'static str,
    values: Vec<f64>,
}

impl ObservedSeries {
    fn new(source: &'static str, units: &'static str, values: Vec<f64>) -> Self {
        Self {
            source,
            units,
            values,
        }
    }
}

fn thermostat_setpoint_values(
    model: &SimulationModel,
    zone: ep_model::ZoneId,
    heating: bool,
    sample_count: usize,
) -> Result<Vec<f64>, String> {
    let thermostat_edge = model
        .graph
        .zone_thermostats
        .iter()
        .find(|edge| edge.zone == zone)
        .ok_or_else(|| "missing zone thermostat edge".to_string())?;
    let thermostat = model
        .typed
        .zone_thermostats
        .iter()
        .find(|thermostat| thermostat.id == thermostat_edge.thermostat)
        .ok_or_else(|| "missing zone thermostat".to_string())?;
    let control = thermostat
        .controls
        .first()
        .ok_or_else(|| "zone thermostat has no controls".to_string())?;
    let dual_setpoint = model
        .typed
        .thermostat_dual_setpoints
        .iter()
        .find(|setpoint| setpoint.id == control.dual_setpoint)
        .ok_or_else(|| "missing ThermostatSetpoint:DualSetpoint".to_string())?;
    let schedule_id = if heating {
        dual_setpoint.heating_setpoint_schedule
    } else {
        dual_setpoint.cooling_setpoint_schedule
    };
    let schedule = model
        .typed
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .ok_or_else(|| {
            "IdealLoads diagnostic currently requires constant thermostat setpoint schedules"
                .to_string()
        })?;
    Ok(vec![schedule.hourly_value; sample_count])
}

fn add_result_series(
    observed_by_variable: &mut BTreeMap<(String, String), ObservedSeries>,
    key: &str,
    results: &[IdealLoadsReportSnapshot],
    variable: &str,
    units: &'static str,
    source: &'static str,
    value: impl Fn(IdealLoadsReportSnapshot) -> f64,
) {
    observed_by_variable.insert(
        (key.to_string(), variable.to_string()),
        ObservedSeries::new(source, units, results.iter().copied().map(value).collect()),
    );
}

fn add_result_series_indexed(
    observed_by_variable: &mut BTreeMap<(String, String), ObservedSeries>,
    key: &str,
    results: &[IdealLoadsReportSnapshot],
    variable: &str,
    units: &'static str,
    source: &'static str,
    value: impl Fn(usize, IdealLoadsReportSnapshot) -> f64,
) {
    observed_by_variable.insert(
        (key.to_string(), variable.to_string()),
        ObservedSeries::new(
            source,
            units,
            results
                .iter()
                .copied()
                .enumerate()
                .map(|(index, result)| value(index, result))
                .collect(),
        ),
    );
}

fn add_result_energy_series(
    observed_by_variable: &mut BTreeMap<(String, String), ObservedSeries>,
    key: &str,
    results: &[IdealLoadsReportSnapshot],
    variable: &str,
    source: &'static str,
    timestamps: &[Option<String>],
    default_report_interval_seconds: f64,
    rate: impl Fn(IdealLoadsReportSnapshot) -> f64,
) {
    let values = results
        .iter()
        .copied()
        .enumerate()
        .map(|(index, result)| {
            let interval_seconds = ideal_loads_sample_timestep_seconds(
                timestamps
                    .get(index)
                    .and_then(|timestamp| timestamp.as_deref()),
                default_report_interval_seconds,
            );
            meter_rate_to_energy_j(rate(result), interval_seconds)
        })
        .collect();
    observed_by_variable.insert(
        (key.to_string(), variable.to_string()),
        ObservedSeries::new(source, "J", values),
    );
}

fn add_result_energy_series_indexed(
    observed_by_variable: &mut BTreeMap<(String, String), ObservedSeries>,
    key: &str,
    results: &[IdealLoadsReportSnapshot],
    variable: &str,
    source: &'static str,
    timestamps: &[Option<String>],
    default_report_interval_seconds: f64,
    rate: impl Fn(usize, IdealLoadsReportSnapshot) -> f64,
) {
    let values = results
        .iter()
        .copied()
        .enumerate()
        .map(|(index, result)| {
            let interval_seconds = ideal_loads_sample_timestep_seconds(
                timestamps
                    .get(index)
                    .and_then(|timestamp| timestamp.as_deref()),
                default_report_interval_seconds,
            );
            meter_rate_to_energy_j(rate(index, result), interval_seconds)
        })
        .collect();
    observed_by_variable.insert(
        (key.to_string(), variable.to_string()),
        ObservedSeries::new(source, "J", values),
    );
}

fn values_from_samples(samples: &[SeriesSample], sample_count: usize) -> Vec<f64> {
    samples
        .iter()
        .take(sample_count)
        .map(|sample| sample.value)
        .collect()
}

fn samples_with_timestamps(values: &[f64], timestamps: &[Option<String>]) -> Vec<SeriesSample> {
    values
        .iter()
        .copied()
        .enumerate()
        .map(
            |(index, value)| match timestamps.get(index).cloned().flatten() {
                Some(timestamp) => SeriesSample::timestamped(index, timestamp, value),
                None => SeriesSample::indexed(index, value),
            },
        )
        .collect()
}

fn resolve_ideal_loads_output_handles(
    manifest: &ConformanceCase,
) -> Result<IdealLoadsOutputHandleMap, String> {
    let mut seen = BTreeSet::new();
    let mut handles = BTreeMap::new();
    for output in &manifest.outputs {
        let identity = ideal_loads_output_identity(output);
        if !seen.insert(identity.clone()) {
            return Err(format!(
                "IdealLoads output handle setup rejected duplicate output request {} / {} ({})",
                output.key,
                output.variable,
                output_frequency_label(output.frequency)
            ));
        }
        handles.insert(identity, OutputHandle(handles.len() as u32));
    }
    Ok(handles)
}

fn ideal_loads_output_handle(
    handles: &IdealLoadsOutputHandleMap,
    output: &OutputRequest,
) -> Result<OutputHandle, String> {
    handles
        .get(&ideal_loads_output_identity(output))
        .copied()
        .ok_or_else(|| {
            format!(
                "IdealLoads output handle setup is missing {} / {} ({})",
                output.key,
                output.variable,
                output_frequency_label(output.frequency)
            )
        })
}

fn ideal_loads_output_identity(output: &OutputRequest) -> (String, String, OutputFrequency) {
    (
        NormalizedName::new(&output.key).0,
        output.variable.trim().to_ascii_lowercase(),
        output.frequency,
    )
}

fn tolerance_for_output(
    manifest: &ConformanceCase,
    output: &OutputRequest,
) -> Result<Tolerance, String> {
    let rule = manifest
        .tolerances
        .iter()
        .find(|rule| rule.variable_class == output.class)
        .ok_or_else(|| {
            format!(
                "missing tolerance rule for {} output {}",
                variable_class_label(output.class),
                output.variable
            )
        })?;
    Ok(Tolerance {
        absolute: output.abs_tol.or(rule.max_abs).unwrap_or(0.0),
        relative: output.rel_tol.or(rule.max_rel).unwrap_or(0.0),
    })
}

fn max_rmse_tolerance_for_output(
    manifest: &ConformanceCase,
    output: &OutputRequest,
) -> Result<Option<f64>, String> {
    let rule = manifest
        .tolerances
        .iter()
        .find(|rule| rule.variable_class == output.class)
        .ok_or_else(|| {
            format!(
                "missing tolerance rule for {} output {}",
                variable_class_label(output.class),
                output.variable
            )
        })?;
    Ok(output.rmse_tol.or(rule.max_rmse))
}

fn mean_abs_delta(expected: &[SeriesSample], observed: &[SeriesSample]) -> f64 {
    let compared_samples = expected.len().min(observed.len());
    if compared_samples == 0 {
        return 0.0;
    }
    expected
        .iter()
        .zip(observed)
        .take(compared_samples)
        .map(|(left, right)| (left.value - right.value).abs())
        .sum::<f64>()
        / compared_samples as f64
}

fn write_artifacts(
    compare_dir: &Path,
    context: &IdealLoadsDiagnosticContext<'_>,
    timing: &ReportTimingSummary,
) -> Result<(), String> {
    std::fs::create_dir_all(compare_dir)
        .map_err(|error| format!("failed to create IdealLoads report directory: {error}"))?;
    std::fs::write(
        compare_dir.join("compare-report.md"),
        render_markdown(context),
    )
    .map_err(|error| format!("failed to write IdealLoads compare report: {error}"))?;
    std::fs::write(
        compare_dir.join("compare-summary.json"),
        append_timing_to_json_object(render_summary_json(context), timing),
    )
    .map_err(|error| format!("failed to write IdealLoads compare summary: {error}"))?;
    std::fs::write(
        compare_dir.join("selected_outputs.json"),
        render_selected_outputs_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads selected outputs: {error}"))?;
    std::fs::write(
        compare_dir.join("rust-result-store.json"),
        render_result_store_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads Rust result store: {error}"))?;
    std::fs::write(
        compare_dir.join("variable-deltas.csv"),
        render_variable_deltas_csv(context),
    )
    .map_err(|error| format!("failed to write IdealLoads variable deltas: {error}"))?;
    std::fs::write(
        compare_dir.join("first-divergence.csv"),
        render_first_divergence_csv(context),
    )
    .map_err(|error| format!("failed to write IdealLoads first divergence CSV: {error}"))?;
    std::fs::write(
        compare_dir.join("tolerance-failures.csv"),
        render_tolerance_failures_csv(context),
    )
    .map_err(|error| format!("failed to write IdealLoads tolerance failures CSV: {error}"))?;
    std::fs::write(
        compare_dir.join("stage-summary.json"),
        render_stage_summary_json(context),
    )
    .map_err(|error| format!("failed to write IdealLoads stage summary: {error}"))?;
    Ok(())
}

fn render_markdown(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let manifest = context.manifest;
    let mut report = String::new();
    report.push_str("# IdealLoads No-OA Sensible Report\n\n");
    report.push_str("## Manifest\n\n");
    report.push_str(&format!("case_id: {}\n", manifest.id));
    report.push_str(&format!(
        "comparison_class: {}\n",
        comparison_class_label(manifest.comparison_class)
    ));
    report.push_str(&format!(
        "conformance_claim: {}\n",
        manifest.conformance_claim
    ));
    report.push_str(&format!("claim_boundary: {}\n", claim_boundary(context)));
    report.push_str(&format!(
        "tolerance_policy: {}\n",
        tolerance_policy(context)
    ));
    report.push_str("timestamp_rule: EnergyPlus timestep ESO timestamps; Rust samples inherit oracle timestep labels\n");
    report.push_str(&format!(
        "zone_demand_source: {}\n",
        ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE
    ));
    report.push_str(&format!(
        "zone_demand_struct_source: {}::{}\n",
        ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT
    ));
    report.push_str(&format!(
        "zone_demand_heating_field: {}\n",
        ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD
    ));
    report.push_str(&format!(
        "zone_demand_heating_sign_convention: {}\n",
        ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION
    ));
    report.push_str(&format!(
        "zone_demand_cooling_field: {}\n",
        ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD
    ));
    report.push_str(&format!(
        "zone_demand_cooling_sign_convention: {}\n",
        ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION
    ));
    report.push_str(&format!(
        "zone_demand_mismatch_classification: {}\n",
        ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION
    ));
    report.push_str(&format!(
        "zone_demand_fixture_mode: {}\n",
        ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE
    ));
    report.push_str("zone_state_source: source-order pre-update zone air node state; same-timestamp zone air node outputs are diagnostic proof rows\n");
    report.push_str(&format!(
        "fuel_energy_rate_source: {}\n",
        fuel_energy_report_source(context)
    ));
    report.push_str(&format!(
        "fuel_efficiency: heating={:.12} cooling={:.12}\n",
        context.fuel_efficiency.heating, context.fuel_efficiency.cooling
    ));
    report.push_str(&format!(
        "energy_source: EnergyPlus ReportPurchasedAir raw rate * TimeStepSysSec summed by OutputProcessor; {}\n",
        report_energy_source_policy(context)
    ));
    report.push_str(&format!("timestep_source: {}\n", context.timestep.source));
    report.push_str(&format!(
        "nominal_system_timestep_substeps: {:.0}\n",
        context.timestep.nominal_system_timestep_substeps
    ));
    report.push_str(&format!(
        "nominal_system_timestep_seconds: {:.12}\n",
        context.timestep.nominal_system_timestep_seconds
    ));
    report.push_str(&format!(
        "zone_timestep_seconds: {:.12}\n",
        context.timestep.zone_timestep_seconds
    ));
    report.push_str(&format!(
        "adaptive_system_timestep_claim: {}\n",
        context.timestep.adaptive_system_timestep_claim
    ));
    report.push_str("sample_timestep_source: ESO timestamp duration with ep_runtime::TimeAxis integer-substep normalization and nominal fallback\n");
    report.push_str(&format!(
        "rate_output_source: {}\n",
        IDEAL_LOADS_RATE_OUTPUT_SOURCE
    ));
    report.push_str(&format!(
        "rate_output_timestep_source: {}\n",
        IDEAL_LOADS_RATE_OUTPUT_TIMESTEP_SOURCE
    ));
    report.push_str(&format!(
        "energy_output_timestep_source: {}\n",
        IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE
    ));
    report.push_str(&format!(
        "energy_output_level_policy: {}\n",
        report_energy_output_level_policy(context)
    ));
    report.push_str(&format!(
        "fuel_energy_output_level_policy: {}\n",
        fuel_energy_output_level_policy(context)
    ));
    report.push_str(&format!(
        "meter_source: {}; rust_meter_time_series_comparison=true requested_meters={}\n",
        facility_meter_report_source(context),
        manifest.meters.len()
    ));
    report.push_str(&format!(
        "meter_aggregation_source: {}\n",
        IDEAL_LOADS_METER_AGGREGATION_SOURCE
    ));
    report.push_str(&format!(
        "meter_fuel_energy_binding_source: {}\n",
        IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE
    ));
    if !manifest.meters.is_empty() {
        let meter_names = manifest
            .meters
            .iter()
            .map(|meter| meter.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        report.push_str(&format!(
            "meter_requests: {}\n",
            markdown_cell(&meter_names)
        ));
    }
    report.push_str("zone_demand_synthetic_rc_model: false\n");
    report.push_str(&format!("oracle_version: {}\n", manifest.oracle_version));
    report.push_str(&format!("zone: {}\n", markdown_cell(&context.zone_name)));
    report.push_str(&format!(
        "zone_air_node: {}\n",
        markdown_cell(&context.zone_air_node_name)
    ));
    if let Some(recirculation_node_name) = context.recirculation_node_name.as_ref() {
        report.push_str(&format!(
            "recirculation_node: {}\n",
            markdown_cell(recirculation_node_name)
        ));
        report.push_str(&format!(
            "recirculation_state_source: {}\n",
            ideal_loads_recirculation_state_source(context.branch)
        ));
    }
    report.push_str(&format!(
        "ideal_loads_system: {}\n",
        markdown_cell(&context.system_name)
    ));
    let purchased_air_source_order = purchased_air_source_order_stages()
        .iter()
        .map(|stage| stage.source_routine)
        .collect::<Vec<_>>()
        .join(" -> ");
    let zone_equipment_dispatch_issues = context.zone_equipment_dispatch.issue_codes();
    let zone_equipment_dispatch_warnings = context.zone_equipment_dispatch.warning_codes();
    report.push_str(&format!(
        "source_order_wrapper: {}\n",
        IDEAL_LOADS_NO_OA_SOURCE_ORDER_WRAPPER
    ));
    report.push_str(&format!(
        "ideal_loads_invocation_path: {}\n",
        IDEAL_LOADS_INVOCATION_PATH
    ));
    report.push_str(&format!(
        "direct_calc_helper_invocation: {}\n",
        IDEAL_LOADS_DIRECT_CALC_HELPER_INVOCATION
    ));
    report.push_str(&format!(
        "zone_equipment_dispatch_execution_boundary: {}\n",
        IDEAL_LOADS_ZONE_EQUIPMENT_EXECUTION_BOUNDARY
    ));
    report.push_str(&format!(
        "ideal_loads_runtime_binding_source: {}\n",
        IDEAL_LOADS_RUNTIME_BINDING_SOURCE
    ));
    report.push_str(&format!(
        "purchased_air_name_lookup_policy: {}\n",
        IDEAL_LOADS_RUNTIME_STRING_LOOKUP_POLICY
    ));
    report.push_str(&format!(
        "zone_equipment_dispatch_path: {}\n",
        IDEAL_LOADS_ZONE_EQUIPMENT_DISPATCH_PATH
    ));
    report.push_str(&format!(
        "zone_equipment_dispatch_validation: {}\n",
        context.zone_equipment_dispatch.dispatch_status_label()
    ));
    report.push_str(&format!(
        "zone_equipment_conformance_candidate: {}\n",
        context
            .zone_equipment_dispatch
            .conformance_candidate_status_label()
    ));
    report.push_str(&format!(
        "zone_equipment_scope: {}\n",
        context.zone_equipment_dispatch.scope_label()
    ));
    report.push_str(&format!(
        "zone_equipment_dispatch_issues: {}\n",
        label_list_or_none(&zone_equipment_dispatch_issues)
    ));
    report.push_str(&format!(
        "zone_equipment_dispatch_warnings: {}\n",
        label_list_or_none(&zone_equipment_dispatch_warnings)
    ));
    report.push_str(&format!(
        "selected_purchased_air_branch: {}\n",
        context.selected_purchased_air_branch
    ));
    report.push_str(&format!(
        "declared_ideal_loads_branch: {}\n",
        context.declared_ideal_loads_branch
    ));
    report.push_str(&format!(
        "inactive_branches: {}\n",
        context.inactive_branches.join(", ")
    ));
    report.push_str(&format!(
        "ideal_loads_feature_flags: {}\n",
        ideal_loads_feature_flags_label(context.feature_flags)
    ));
    report.push_str(&format!(
        "ideal_loads_feature_dispatch_policy: {}\n",
        IDEAL_LOADS_FEATURE_DISPATCH_POLICY
    ));
    report.push_str(&format!(
        "ideal_loads_prebound_id_contract: {}\n",
        IDEAL_LOADS_PREBOUND_ID_CONTRACT
    ));
    report.push_str(&format!(
        "ideal_loads_psychrometric_evaluation_policy: {}\n",
        IDEAL_LOADS_PSYCHROMETRIC_EVALUATION_POLICY
    ));
    report.push_str(&format!(
        "ideal_loads_psychrometric_cache_policy: {}\n",
        IDEAL_LOADS_PSYCHROMETRIC_CACHE_POLICY
    ));
    push_ideal_loads_output_handle_policy_markdown(&mut report);
    report.push_str(&format!(
        "trace_level: {}\n",
        ideal_loads_trace_level(context.manifest)
    ));
    report.push_str(&format!(
        "trace_level_source: {}\n",
        ideal_loads_trace_level_source(context.manifest)
    ));
    report.push_str(&format!(
        "trace_payload: {}\n",
        IDEAL_LOADS_NO_OA_TRACE_PAYLOAD
    ));
    report.push_str(&format!(
        "trace_side_effect_policy: {}\n",
        IDEAL_LOADS_TRACE_SIDE_EFFECT_POLICY
    ));
    report.push_str(&format!(
        "trace_result_invariance_policy: {}\n",
        IDEAL_LOADS_TRACE_RESULT_INVARIANCE_POLICY
    ));
    report.push_str(&format!(
        "trace_overhead_accounting: {}\n",
        IDEAL_LOADS_TRACE_OVERHEAD_ACCOUNTING
    ));
    report.push_str(&format!(
        "source_map_anchor: {}\n",
        IDEAL_LOADS_SOURCE_MAP_ANCHOR
    ));
    report.push_str(&format!(
        "node_output_timestamp_alignment: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_TIMESTAMP_ALIGNMENT
    ));
    report.push_str(&format!(
        "node_output_store_type: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE
    ));
    report.push_str(&format!(
        "node_output_state_struct: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT
    ));
    report.push_str(&format!(
        "node_output_update_source: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE
    ));
    report.push_str(&format!(
        "node_output_report_source: {}\n",
        IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE
    ));
    report.push_str(&format!(
        "purchased_air_source_order: {}\n",
        purchased_air_source_order
    ));
    report.push_str(&format!(
        "supply_node: {}\n\n",
        markdown_cell(&context.supply_node_name)
    ));

    report.push_str("## Result\n\n");
    report.push_str(&format!("status: {}\n", overall_status(context)));
    report.push_str(&format!("series: {}\n", context.rows.len()));
    report.push_str(&format!("samples: {}\n", context.input_trace.sample_count));
    report.push_str(&format!(
        "tolerance_failures: {}\n",
        tolerance_failures_count(context)
    ));
    report.push_str(&format!("meter_series: {}\n", context.meter_rows.len()));
    report.push_str(&format!(
        "meter_tolerance_failures: {}\n",
        context
            .meter_rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count()
    ));
    report.push_str(&format!(
        "mode_counts: off={} deadband={} cooling={} heating={}\n\n",
        context.mode_counts.off,
        context.mode_counts.deadband,
        context.mode_counts.cooling,
        context.mode_counts.heating
    ));
    if let Some(summary) = context.moisture_predictor.as_ref() {
        report.push_str("## Humidistat Moisture Predictor Diagnostic\n\n");
        report.push_str(&format!(
            "claim_status: {}\n",
            moisture_predictor_claim_status(summary)
        ));
        report.push_str(&format!(
            "history_source: {}\n",
            markdown_cell(summary.history_source)
        ));
        report.push_str(&format!(
            "latent_gain_source: {}\n",
            markdown_cell(summary.latent_gain_source)
        ));
        report.push_str(&format!(
            "closed_loop_state_source: {}\n",
            markdown_cell(summary.closed_loop_state_source)
        ));
        report.push_str(&format!(
            "history_residual_source: {}\n",
            markdown_cell(summary.history_residual_source)
        ));
        report.push_str(&format!(
            "humidifying_equivalent_history_delta_max: {:.12}\n",
            summary.humidifying_equivalent_history_delta_max
        ));
        report.push_str(&format!(
            "dehumidifying_equivalent_history_delta_max: {:.12}\n",
            summary.dehumidifying_equivalent_history_delta_max
        ));
        report.push_str(&format!(
            "zone_moisture_capacity_multiplier: {:.12}\n",
            summary.zone_moisture_capacity_multiplier
        ));
        report.push_str(&format!(
            "zone_multiplier: {:.12}\n\n",
            summary.zone_multiplier
        ));
        report.push_str("### Inferred Third-Order History Term Check\n\n");
        report.push_str("| demand | samples | row_lag_minus_inferred_max_abs_delta | rmse_delta | mean_delta | max_abs_row_lag_term | max_abs_inferred_term | largest_delta_sample |\n");
        report.push_str("|---|---:|---:|---:|---:|---:|---:|---|\n");
        for comparison in [
            &summary.humidifying_history_term,
            &summary.dehumidifying_history_term,
        ] {
            report.push_str(&format!(
                "| {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {} |\n",
                markdown_cell(&comparison.demand),
                comparison.samples,
                comparison.row_lag_minus_inferred_max_abs_delta,
                comparison.row_lag_minus_inferred_rmse_delta,
                comparison.row_lag_minus_inferred_mean_delta,
                comparison.max_abs_row_lag_history_term,
                comparison.max_abs_inferred_history_term,
                moisture_history_term_sample_label(comparison.largest_delta_sample.as_ref())
            ));
        }
        report.push('\n');
        if !summary.latent_gain.is_empty() {
            report.push_str("### Latent Gain Input Check\n\n");
            report.push_str("| variable | samples | max_abs_delta | rmse_delta | max_rel_delta | status | first_divergence |\n");
            report.push_str("|---|---:|---:|---:|---:|---|---|\n");
            for comparison in &summary.latent_gain {
                report.push_str(&format!(
                    "| {} | {} | {:.12} | {:.12} | {:.12} | {} | {} |\n",
                    markdown_cell(&comparison.variable),
                    comparison.samples,
                    comparison.max_abs_delta,
                    comparison.rmse_delta,
                    comparison.max_rel_delta,
                    status_label(comparison.status),
                    first_divergence_label(comparison.first_divergence.as_ref())
                ));
            }
            report.push('\n');
        }
        report.push_str("### Moisture Demand Predictor Check\n\n");
        report.push_str("| variable | samples | max_abs_delta | rmse_delta | max_rel_delta | status | first_divergence |\n");
        report.push_str("|---|---:|---:|---:|---:|---|---|\n");
        for comparison in [&summary.humidifying, &summary.dehumidifying] {
            report.push_str(&format!(
                "| {} | {} | {:.12} | {:.12} | {:.12} | {} | {} |\n",
                markdown_cell(&comparison.variable),
                comparison.samples,
                comparison.max_abs_delta,
                comparison.rmse_delta,
                comparison.max_rel_delta,
                status_label(comparison.status),
                first_divergence_label(comparison.first_divergence.as_ref())
            ));
        }
        report.push('\n');
        if !summary.closed_loop.is_empty() {
            report.push_str("### Rust Closed-Loop Humidity Diagnostic\n\n");
            report.push_str("| variable | samples | max_abs_delta | rmse_delta | max_rel_delta | status | first_divergence |\n");
            report.push_str("|---|---:|---:|---:|---:|---|---|\n");
            for comparison in &summary.closed_loop {
                report.push_str(&format!(
                    "| {} | {} | {:.12} | {:.12} | {:.12} | {} | {} |\n",
                    markdown_cell(&comparison.variable),
                    comparison.samples,
                    comparison.max_abs_delta,
                    comparison.rmse_delta,
                    comparison.max_rel_delta,
                    status_label(comparison.status),
                    first_divergence_label(comparison.first_divergence.as_ref())
                ));
            }
            report.push('\n');
        }
    }

    report.push_str("## Artifacts\n\n");
    report.push_str("- selected_outputs.json\n");
    report.push_str("- rust-result-store.json\n");
    report.push_str("- compare-summary.json\n");
    report.push_str("- compare-report.md\n");
    report.push_str("- variable-deltas.csv\n");
    report.push_str("- first-divergence.csv\n");
    report.push_str("- tolerance-failures.csv\n");
    report.push_str("- stage-summary.json\n\n");

    report.push_str("## Series\n\n");
    report.push_str("| key | variable | level | domain | class | frequency | rust_source | units | unit_match | alignment | expected | observed | compared | max_abs_delta | mean_abs_delta | rmse_delta | max_rel_delta | tolerance | status | first_divergence |\n");
    report.push_str("|---|---|---|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|---|---|\n");
    for row in &context.rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {} | {} | {} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.variable),
            optional_output_level_label(row.level),
            row.domain.map_or("unspecified", evidence_domain_label),
            variable_class_label(row.variable_class),
            output_frequency_label(row.frequency),
            row.rust_source,
            markdown_cell(&row.units),
            row.unit_match(),
            alignment_label(row.alignment),
            row.expected_samples,
            row.observed_samples,
            row.compared_samples,
            row.max_abs_delta,
            row.mean_abs_delta,
            row.rmse_delta,
            row.max_rel_delta,
            tolerance_label(row.tolerance, row.max_rmse_tolerance),
            status_label(row.status),
            first_divergence_label(row.first_divergence.as_ref())
        ));
    }
    if !context.meter_rows.is_empty() {
        report.push_str("\n## Meters\n\n");
        report.push_str("| meter | level | domain | frequency | source | rust_source | units | unit_match | alignment | expected | observed | compared | max_abs_delta | mean_abs_delta | rmse_delta | max_rel_delta | tolerance | status | first_divergence |\n");
        report.push_str("|---|---|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|---|---|\n");
        for row in &context.meter_rows {
            report.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {} | {} | {} |\n",
                markdown_cell(&row.name),
                output_level_label(row.level),
                evidence_domain_label(row.domain),
                output_frequency_label(row.frequency),
                source_artifact_label(row.source),
                row.rust_source,
                markdown_cell(&row.units),
                row.unit_match(),
                alignment_label(row.alignment),
                row.expected_samples,
                row.observed_samples,
                row.compared_samples,
                row.max_abs_delta,
                row.mean_abs_delta,
                row.rmse_delta,
                row.max_rel_delta,
                tolerance_label(row.tolerance, row.max_rmse_tolerance),
                status_label(row.status),
                first_divergence_label(row.first_divergence.as_ref())
            ));
        }
    }
    report
}

fn render_summary_json(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let manifest = context.manifest;
    let zone_equipment_dispatch_issues = context.zone_equipment_dispatch.issue_codes();
    let zone_equipment_dispatch_warnings = context.zone_equipment_dispatch.warning_codes();
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!("  \"case_id\": {},\n", json_string(&manifest.id)));
    json.push_str(&format!(
        "  \"oracle_version\": {},\n",
        json_string(&manifest.oracle_version)
    ));
    json.push_str(&format!(
        "  \"comparison_class\": {},\n",
        json_string(comparison_class_label(manifest.comparison_class))
    ));
    json.push_str(&format!(
        "  \"conformance_claim\": {},\n",
        manifest.conformance_claim
    ));
    json.push_str(&format!(
        "  \"status\": {},\n",
        json_string(overall_status(context))
    ));
    json.push_str(&format!(
        "  \"tolerance_policy\": {},\n",
        json_string(tolerance_policy(context))
    ));
    json.push_str("  \"timestamp_rule\": \"EnergyPlus timestep ESO timestamps; Rust samples inherit oracle timestep labels\",\n");
    json.push_str(&format!(
        "  \"source_order_wrapper\": {},\n",
        json_string(IDEAL_LOADS_NO_OA_SOURCE_ORDER_WRAPPER)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_invocation_path\": {},\n",
        json_string(IDEAL_LOADS_INVOCATION_PATH)
    ));
    json.push_str(&format!(
        "  \"direct_calc_helper_invocation\": {},\n",
        IDEAL_LOADS_DIRECT_CALC_HELPER_INVOCATION
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_execution_boundary\": {},\n",
        json_string(IDEAL_LOADS_ZONE_EQUIPMENT_EXECUTION_BOUNDARY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_runtime_binding_source\": {},\n",
        json_string(IDEAL_LOADS_RUNTIME_BINDING_SOURCE)
    ));
    json.push_str(&format!(
        "  \"purchased_air_name_lookup_policy\": {},\n",
        json_string(IDEAL_LOADS_RUNTIME_STRING_LOOKUP_POLICY)
    ));
    json.push_str(&format!(
        "  \"zone_demand_source\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"zone_demand_struct_source\": {},\n",
        json_string(&format!(
            "{}::{}",
            ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT
        ))
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_field\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD)
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_field\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_mismatch_classification\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_fixture_mode\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE)
    ));
    json.push_str("  \"zone_state_source\": \"source-order pre-update zone air node state; same-timestamp zone air node outputs are diagnostic proof rows\",\n");
    json.push_str(&format!(
        "  \"source_map_anchor\": {},\n",
        json_string(IDEAL_LOADS_SOURCE_MAP_ANCHOR)
    ));
    json.push_str(&format!(
        "  \"node_output_timestamp_alignment\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_TIMESTAMP_ALIGNMENT)
    ));
    json.push_str(&format!(
        "  \"node_output_store_type\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE)
    ));
    json.push_str(&format!(
        "  \"node_output_state_struct\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT)
    ));
    json.push_str(&format!(
        "  \"node_output_update_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE)
    ));
    json.push_str(&format!(
        "  \"node_output_report_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"selected_purchased_air_branch\": {},\n",
        json_string(context.selected_purchased_air_branch)
    ));
    json.push_str(&format!(
        "  \"declared_ideal_loads_branch\": {},\n",
        json_string(context.declared_ideal_loads_branch)
    ));
    json.push_str(&format!(
        "  \"inactive_branches\": {},\n",
        json_string_array(&context.inactive_branches)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_feature_flags\": {},\n",
        ideal_loads_feature_flags_json(context.feature_flags)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_feature_dispatch_policy\": {},\n",
        json_string(IDEAL_LOADS_FEATURE_DISPATCH_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_prebound_id_contract\": {},\n",
        json_string(IDEAL_LOADS_PREBOUND_ID_CONTRACT)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_psychrometric_evaluation_policy\": {},\n",
        json_string(IDEAL_LOADS_PSYCHROMETRIC_EVALUATION_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_psychrometric_cache_policy\": {},\n",
        json_string(IDEAL_LOADS_PSYCHROMETRIC_CACHE_POLICY)
    ));
    push_ideal_loads_output_handle_policy_json(&mut json);
    json.push_str(&format!(
        "  \"trace_level\": {},\n",
        json_string(ideal_loads_trace_level(manifest))
    ));
    json.push_str(&format!(
        "  \"trace_level_source\": {},\n",
        json_string(ideal_loads_trace_level_source(manifest))
    ));
    json.push_str(&format!(
        "  \"trace_payload\": {},\n",
        json_string(IDEAL_LOADS_NO_OA_TRACE_PAYLOAD)
    ));
    json.push_str(&format!(
        "  \"trace_side_effect_policy\": {},\n",
        json_string(IDEAL_LOADS_TRACE_SIDE_EFFECT_POLICY)
    ));
    json.push_str(&format!(
        "  \"trace_result_invariance_policy\": {},\n",
        json_string(IDEAL_LOADS_TRACE_RESULT_INVARIANCE_POLICY)
    ));
    json.push_str(&format!(
        "  \"trace_overhead_accounting\": {},\n",
        json_string(IDEAL_LOADS_TRACE_OVERHEAD_ACCOUNTING)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_path\": {},\n",
        json_string(IDEAL_LOADS_ZONE_EQUIPMENT_DISPATCH_PATH)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_validation\": {},\n",
        json_string(context.zone_equipment_dispatch.dispatch_status_label())
    ));
    json.push_str(&format!(
        "  \"zone_equipment_conformance_candidate\": {},\n",
        json_string(
            context
                .zone_equipment_dispatch
                .conformance_candidate_status_label()
        )
    ));
    json.push_str(&format!(
        "  \"zone_equipment_scope\": {},\n",
        json_string(context.zone_equipment_dispatch.scope_label())
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_issues\": {},\n",
        json_string_array(&zone_equipment_dispatch_issues)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_warnings\": {},\n",
        json_string_array(&zone_equipment_dispatch_warnings)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_rate_source\": {},\n",
        json_string(fuel_energy_report_source(context))
    ));
    json.push_str(&format!(
        "  \"heating_fuel_efficiency\": {},\n",
        json_number(context.fuel_efficiency.heating)
    ));
    json.push_str(&format!(
        "  \"cooling_fuel_efficiency\": {},\n",
        json_number(context.fuel_efficiency.cooling)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_rate_rust_source\": {},\n",
        json_string(context.fuel_efficiency.rate_rust_source)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_rust_source\": {},\n",
        json_string(context.fuel_efficiency.energy_rust_source)
    ));
    json.push_str(&format!(
        "  \"energy_source\": {},\n",
        json_string(&format!(
            "EnergyPlus ReportPurchasedAir raw rate * TimeStepSysSec summed by OutputProcessor; {}",
            report_energy_source_policy(context)
        ))
    ));
    json.push_str(&format!(
        "  \"rate_output_source\": {},\n",
        json_string(IDEAL_LOADS_RATE_OUTPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"rate_output_timestep_source\": {},\n",
        json_string(IDEAL_LOADS_RATE_OUTPUT_TIMESTEP_SOURCE)
    ));
    json.push_str(&format!(
        "  \"energy_output_timestep_source\": {},\n",
        json_string(IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE)
    ));
    json.push_str(&format!(
        "  \"energy_output_level_policy\": {},\n",
        json_string(report_energy_output_level_policy(context))
    ));
    json.push_str(&format!(
        "  \"fuel_energy_output_level_policy\": {},\n",
        json_string(fuel_energy_output_level_policy(context))
    ));
    json.push_str(&format!(
        "  \"meter_source\": {},\n",
        json_string(facility_meter_report_source(context))
    ));
    json.push_str(&format!(
        "  \"meter_aggregation_source\": {},\n",
        json_string(IDEAL_LOADS_METER_AGGREGATION_SOURCE)
    ));
    json.push_str(&format!(
        "  \"meter_fuel_energy_binding_source\": {},\n",
        json_string(IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE)
    ));
    json.push_str("  \"rust_meter_time_series_comparison\": true,\n");
    json.push_str(&format!(
        "  \"requested_meter_count\": {},\n",
        manifest.meters.len()
    ));
    json.push_str("  \"requested_meters\": [\n");
    for (index, meter) in manifest.meters.iter().enumerate() {
        json.push_str(&format!(
            "    {{\"name\": {}, \"frequency\": {}, \"source\": {}, \"domain\": {}, \"level\": {}}}",
            json_string(&meter.name),
            json_string(output_frequency_label(meter.frequency)),
            json_string(source_artifact_label(meter.source)),
            json_string(evidence_domain_label(meter.domain)),
            json_string(optional_output_level_label(Some(meter.level)))
        ));
        if index + 1 < manifest.meters.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ],\n");
    json.push_str(&format!(
        "  \"meter_series_count\": {},\n",
        context.meter_rows.len()
    ));
    json.push_str(&format!(
        "  \"meter_tolerance_failures\": {},\n",
        context
            .meter_rows
            .iter()
            .filter(|row| row.status == SeriesComparisonStatus::Fail)
            .count()
    ));
    json.push_str("  \"meter_series\": [\n");
    for (index, row) in context.meter_rows.iter().enumerate() {
        json.push_str("    ");
        json.push_str(&meter_row_json(row));
        if index + 1 < context.meter_rows.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ],\n");
    json.push_str(&format!(
        "  \"timestep_source\": {},\n",
        json_string(context.timestep.source)
    ));
    json.push_str(&format!(
        "  \"nominal_system_timestep_substeps\": {},\n",
        json_number(context.timestep.nominal_system_timestep_substeps)
    ));
    json.push_str(&format!(
        "  \"nominal_system_timestep_seconds\": {},\n",
        json_number(context.timestep.nominal_system_timestep_seconds)
    ));
    json.push_str(&format!(
        "  \"zone_timestep_seconds\": {},\n",
        json_number(context.timestep.zone_timestep_seconds)
    ));
    json.push_str(&format!(
        "  \"adaptive_system_timestep_claim\": {},\n",
        context.timestep.adaptive_system_timestep_claim
    ));
    json.push_str("  \"sample_timestep_source\": \"ESO timestamp duration with ep_runtime::TimeAxis integer-substep normalization and nominal fallback\",\n");
    json.push_str("  \"zone_demand_synthetic_rc_model\": false,\n");
    json.push_str(&format!(
        "  \"zone\": {},\n",
        json_string(&context.zone_name)
    ));
    json.push_str(&format!(
        "  \"zone_air_node\": {},\n",
        json_string(&context.zone_air_node_name)
    ));
    json.push_str(&format!(
        "  \"recirculation_node\": {},\n",
        context
            .recirculation_node_name
            .as_ref()
            .map_or_else(|| "null".to_string(), |name| json_string(name))
    ));
    if context.recirculation_node_name.is_some() {
        json.push_str(&format!(
            "  \"recirculation_state_source\": {},\n",
            json_string(ideal_loads_recirculation_state_source(context.branch))
        ));
    }
    json.push_str(&format!(
        "  \"ideal_loads_system\": {},\n",
        json_string(&context.system_name)
    ));
    json.push_str(&format!(
        "  \"supply_node\": {},\n",
        json_string(&context.supply_node_name)
    ));
    json.push_str(&format!(
        "  \"samples\": {},\n",
        context.input_trace.sample_count
    ));
    json.push_str(&format!("  \"series_count\": {},\n", context.rows.len()));
    json.push_str(&format!(
        "  \"tolerance_failures\": {},\n",
        tolerance_failures_count(context)
    ));
    json.push_str(&format!(
        "  \"mode_counts\": {{\"off\": {}, \"deadband\": {}, \"cooling\": {}, \"heating\": {}}},\n",
        context.mode_counts.off,
        context.mode_counts.deadband,
        context.mode_counts.cooling,
        context.mode_counts.heating
    ));
    json.push_str(&format!(
        "  \"moisture_predictor\": {},\n",
        moisture_predictor_json(context.moisture_predictor.as_ref())
    ));
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"oracle_selected_outputs_json\": \"selected_outputs.json\",\n");
    json.push_str("    \"rust_result_store_json\": \"rust-result-store.json\",\n");
    json.push_str("    \"compare_summary_json\": \"compare-summary.json\",\n");
    json.push_str("    \"compare_report_md\": \"compare-report.md\",\n");
    json.push_str("    \"variable_deltas_csv\": \"variable-deltas.csv\",\n");
    json.push_str("    \"first_divergence_csv\": \"first-divergence.csv\",\n");
    json.push_str("    \"tolerance_failures_csv\": \"tolerance-failures.csv\",\n");
    json.push_str("    \"stage_summary_json\": \"stage-summary.json\"\n");
    json.push_str("  },\n");
    json.push_str(&format!(
        "  \"domains\": {},\n",
        domain_status_json(&context.rows)
    ));
    json.push_str("  \"series\": [\n");
    for (index, row) in context.rows.iter().enumerate() {
        json.push_str("    ");
        json.push_str(&row_json(row));
        if index + 1 < context.rows.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn ideal_loads_trace_level(manifest: &ConformanceCase) -> &str {
    manifest
        .trace
        .as_ref()
        .map_or(IDEAL_LOADS_TRACE_LEVEL_DEFAULT, |trace| {
            trace.level.as_str()
        })
}

fn ideal_loads_trace_level_source(manifest: &ConformanceCase) -> &'static str {
    if manifest.trace.is_some() {
        IDEAL_LOADS_TRACE_LEVEL_SOURCE_MANIFEST
    } else {
        IDEAL_LOADS_TRACE_LEVEL_SOURCE_DEFAULT
    }
}

fn push_ideal_loads_output_handle_policy_markdown(report: &mut String) {
    report.push_str(&format!(
        "ideal_loads_output_handle_registration_policy: {}\n",
        IDEAL_LOADS_OUTPUT_HANDLE_REGISTRATION_POLICY
    ));
    report.push_str(&format!(
        "ideal_loads_output_handle_write_policy: {}\n",
        IDEAL_LOADS_OUTPUT_HANDLE_WRITE_POLICY
    ));
    report.push_str(&format!(
        "ideal_loads_diagnostic_output_request_policy: {}\n",
        IDEAL_LOADS_DIAGNOSTIC_OUTPUT_REQUEST_POLICY
    ));
    report.push_str(&format!(
        "ideal_loads_report_export_order_policy: {}\n",
        IDEAL_LOADS_REPORT_EXPORT_ORDER_POLICY
    ));
    report.push_str(&format!(
        "ideal_loads_detailed_output_lookup_policy: {}\n",
        IDEAL_LOADS_DETAILED_OUTPUT_LOOKUP_POLICY
    ));
    report.push_str(&format!(
        "ideal_loads_duplicate_output_handle_policy: {}\n",
        IDEAL_LOADS_DUPLICATE_OUTPUT_HANDLE_POLICY
    ));
}

fn push_ideal_loads_output_handle_policy_json(json: &mut String) {
    json.push_str(&format!(
        "  \"ideal_loads_output_handle_registration_policy\": {},\n",
        json_string(IDEAL_LOADS_OUTPUT_HANDLE_REGISTRATION_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_output_handle_write_policy\": {},\n",
        json_string(IDEAL_LOADS_OUTPUT_HANDLE_WRITE_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_diagnostic_output_request_policy\": {},\n",
        json_string(IDEAL_LOADS_DIAGNOSTIC_OUTPUT_REQUEST_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_report_export_order_policy\": {},\n",
        json_string(IDEAL_LOADS_REPORT_EXPORT_ORDER_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_detailed_output_lookup_policy\": {},\n",
        json_string(IDEAL_LOADS_DETAILED_OUTPUT_LOOKUP_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_duplicate_output_handle_policy\": {},\n",
        json_string(IDEAL_LOADS_DUPLICATE_OUTPUT_HANDLE_POLICY)
    ));
}

fn ideal_loads_feature_flags_label(flags: IdealLoadsFeatureFlags) -> String {
    format!(
        concat!(
            "has_outdoor_air={}, has_economizer={}, has_heat_recovery={}, ",
            "has_dcv={}, has_humidistat={}, has_constant_shr={}, ",
            "has_constant_supply_humidity={}, has_flow_limit={}, ",
            "has_capacity_limit={}, has_autosize={}"
        ),
        flags.has_outdoor_air,
        flags.has_economizer,
        flags.has_heat_recovery,
        flags.has_dcv,
        flags.has_humidistat,
        flags.has_constant_shr,
        flags.has_constant_supply_humidity,
        flags.has_flow_limit,
        flags.has_capacity_limit,
        flags.has_autosize
    )
}

fn ideal_loads_feature_flags_json(flags: IdealLoadsFeatureFlags) -> String {
    format!(
        concat!(
            "{{\"has_outdoor_air\": {}, \"has_economizer\": {}, ",
            "\"has_heat_recovery\": {}, \"has_dcv\": {}, ",
            "\"has_humidistat\": {}, \"has_constant_shr\": {}, ",
            "\"has_constant_supply_humidity\": {}, \"has_flow_limit\": {}, ",
            "\"has_capacity_limit\": {}, \"has_autosize\": {}}}"
        ),
        flags.has_outdoor_air,
        flags.has_economizer,
        flags.has_heat_recovery,
        flags.has_dcv,
        flags.has_humidistat,
        flags.has_constant_shr,
        flags.has_constant_supply_humidity,
        flags.has_flow_limit,
        flags.has_capacity_limit,
        flags.has_autosize
    )
}

fn json_string_array(values: &[&str]) -> String {
    let mut json = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&json_string(value));
    }
    json.push(']');
    json
}

fn label_list_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

fn moisture_predictor_json(summary: Option<&IdealLoadsMoisturePredictorSummary>) -> String {
    let Some(summary) = summary else {
        return "null".to_string();
    };
    format!(
        concat!(
            "{{\"claim_status\": {}, ",
            "\"history_source\": {}, \"latent_gain_source\": {}, ",
            "\"closed_loop_state_source\": {}, \"history_residual_source\": {}, ",
            "\"humidifying_equivalent_history_delta_max\": {}, ",
            "\"dehumidifying_equivalent_history_delta_max\": {}, ",
            "\"humidifying_history_term\": {}, ",
            "\"dehumidifying_history_term\": {}, ",
            "\"zone_moisture_capacity_multiplier\": {}, \"zone_multiplier\": {}, ",
            "\"latent_gain\": [{}], \"humidifying\": {}, \"dehumidifying\": {}, ",
            "\"closed_loop\": [{}]}}"
        ),
        json_string(moisture_predictor_claim_status(summary)),
        json_string(summary.history_source),
        json_string(summary.latent_gain_source),
        json_string(summary.closed_loop_state_source),
        json_string(summary.history_residual_source),
        json_number(summary.humidifying_equivalent_history_delta_max),
        json_number(summary.dehumidifying_equivalent_history_delta_max),
        moisture_history_term_comparison_json(&summary.humidifying_history_term),
        moisture_history_term_comparison_json(&summary.dehumidifying_history_term),
        json_number(summary.zone_moisture_capacity_multiplier),
        json_number(summary.zone_multiplier),
        summary
            .latent_gain
            .iter()
            .map(moisture_predictor_comparison_json)
            .collect::<Vec<_>>()
            .join(", "),
        moisture_predictor_comparison_json(&summary.humidifying),
        moisture_predictor_comparison_json(&summary.dehumidifying),
        summary
            .closed_loop
            .iter()
            .map(moisture_predictor_comparison_json)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn moisture_predictor_claim_status(summary: &IdealLoadsMoisturePredictorSummary) -> &'static str {
    if summary.promoted_input {
        "conformance-supporting closed-loop no-OA ThirdOrder humidity predictor/corrector; values feed the promoted Humidistat no-OA comparison branch for the declared candidate rows after seeded history"
    } else {
        "diagnostic trace-driven predictor; this manifest keeps EnergyPlus moisture-demand proof rows as the promoted branch input"
    }
}

fn moisture_predictor_comparison_json(
    comparison: &IdealLoadsMoisturePredictorComparison,
) -> String {
    format!(
        concat!(
            "{{\"variable\": {}, \"samples\": {}, \"max_abs_delta\": {}, ",
            "\"rmse_delta\": {}, \"max_rel_delta\": {}, \"status\": {}, ",
            "\"first_divergence\": {}}}"
        ),
        json_string(&comparison.variable),
        comparison.samples,
        json_number(comparison.max_abs_delta),
        json_number(comparison.rmse_delta),
        json_number(comparison.max_rel_delta),
        json_string(status_label(comparison.status)),
        first_divergence_json(comparison.first_divergence.as_ref())
    )
}

fn moisture_history_term_comparison_json(
    comparison: &IdealLoadsMoistureHistoryTermComparison,
) -> String {
    format!(
        concat!(
            "{{\"demand\": {}, \"samples\": {}, ",
            "\"row_lag_minus_inferred_max_abs_delta\": {}, ",
            "\"row_lag_minus_inferred_rmse_delta\": {}, ",
            "\"row_lag_minus_inferred_mean_delta\": {}, ",
            "\"max_abs_row_lag_history_term\": {}, ",
            "\"max_abs_inferred_history_term\": {}, ",
            "\"largest_delta_sample\": {}}}"
        ),
        json_string(&comparison.demand),
        comparison.samples,
        json_number(comparison.row_lag_minus_inferred_max_abs_delta),
        json_number(comparison.row_lag_minus_inferred_rmse_delta),
        json_number(comparison.row_lag_minus_inferred_mean_delta),
        json_number(comparison.max_abs_row_lag_history_term),
        json_number(comparison.max_abs_inferred_history_term),
        moisture_history_term_sample_json(comparison.largest_delta_sample.as_ref())
    )
}

fn moisture_history_term_sample_json(
    sample: Option<&IdealLoadsMoistureHistoryTermSample>,
) -> String {
    let Some(sample) = sample else {
        return "null".to_string();
    };
    format!(
        concat!(
            "{{\"index\": {}, \"timestamp\": {}, ",
            "\"row_lag_history_term\": {}, \"inferred_history_term\": {}, ",
            "\"row_lag_minus_inferred_delta\": {}}}"
        ),
        sample.index,
        sample
            .timestamp
            .as_ref()
            .map_or_else(|| "null".to_string(), |timestamp| json_string(timestamp)),
        json_number(sample.row_lag_history_term),
        json_number(sample.inferred_history_term),
        json_number(sample.row_lag_minus_inferred_delta)
    )
}

fn row_json(row: &IdealLoadsDiagnosticRow) -> String {
    format!(
        concat!(
            "{{\"handle\": {}, \"key\": {}, \"variable\": {}, \"level\": {}, \"domain\": {}, ",
            "\"class\": {}, \"frequency\": {}, \"source\": {}, \"rust_source\": {}, ",
            "\"units\": {}, \"oracle_units\": {}, \"unit_match\": {}, ",
            "\"alignment\": {}, \"expected_samples\": {}, \"observed_samples\": {}, ",
            "\"compared_samples\": {}, \"max_abs_delta\": {}, \"mean_abs_delta\": {}, ",
            "\"rmse_delta\": {}, \"max_rel_delta\": {}, \"max_abs_tolerance\": {}, ",
            "\"max_rel_tolerance\": {}, \"max_rmse_tolerance\": {}, \"status\": {}, ",
            "\"first_divergence\": {}}}"
        ),
        row.handle.0,
        json_string(&row.key),
        json_string(&row.variable),
        json_string(optional_output_level_label(row.level)),
        json_string(row.domain.map_or("unspecified", evidence_domain_label)),
        json_string(variable_class_label(row.variable_class)),
        json_string(output_frequency_label(row.frequency)),
        json_string(source_artifact_label(row.source)),
        json_string(row.rust_source),
        json_string(&row.units),
        row.oracle_units
            .as_ref()
            .map_or_else(|| "null".to_string(), |units| json_string(units)),
        row.unit_match(),
        json_string(alignment_label(row.alignment)),
        row.expected_samples,
        row.observed_samples,
        row.compared_samples,
        json_number(row.max_abs_delta),
        json_number(row.mean_abs_delta),
        json_number(row.rmse_delta),
        json_number(row.max_rel_delta),
        json_number(row.tolerance.absolute),
        json_number(row.tolerance.relative),
        row.max_rmse_tolerance
            .map_or_else(|| "null".to_string(), json_number),
        json_string(status_label(row.status)),
        first_divergence_json(row.first_divergence.as_ref())
    )
}

fn meter_row_json(row: &IdealLoadsMeterDiagnosticRow) -> String {
    format!(
        concat!(
            "{{\"name\": {}, \"level\": {}, \"domain\": {}, \"frequency\": {}, ",
            "\"source\": {}, \"rust_source\": {}, \"units\": {}, \"oracle_units\": {}, ",
            "\"unit_match\": {}, \"alignment\": {}, \"expected_samples\": {}, ",
            "\"observed_samples\": {}, \"compared_samples\": {}, \"max_abs_delta\": {}, ",
            "\"mean_abs_delta\": {}, \"rmse_delta\": {}, \"max_rel_delta\": {}, ",
            "\"max_abs_tolerance\": {}, \"max_rel_tolerance\": {}, ",
            "\"max_rmse_tolerance\": {}, \"status\": {}, \"first_divergence\": {}}}"
        ),
        json_string(&row.name),
        json_string(output_level_label(row.level)),
        json_string(evidence_domain_label(row.domain)),
        json_string(output_frequency_label(row.frequency)),
        json_string(source_artifact_label(row.source)),
        json_string(row.rust_source),
        json_string(&row.units),
        row.oracle_units
            .as_ref()
            .map_or_else(|| "null".to_string(), |units| json_string(units)),
        row.unit_match(),
        json_string(alignment_label(row.alignment)),
        row.expected_samples,
        row.observed_samples,
        row.compared_samples,
        json_number(row.max_abs_delta),
        json_number(row.mean_abs_delta),
        json_number(row.rmse_delta),
        json_number(row.max_rel_delta),
        json_number(row.tolerance.absolute),
        json_number(row.tolerance.relative),
        row.max_rmse_tolerance
            .map_or_else(|| "null".to_string(), json_number),
        json_string(status_label(row.status)),
        first_divergence_json(row.first_divergence.as_ref())
    )
}

fn render_selected_outputs_json(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!(
        "  \"eso\": {},\n",
        json_string(&context.baseline.eso.display().to_string())
    ));
    json.push_str("  \"series\": [\n");
    for (index, row) in context.rows.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"key\": {},\n", json_string(&row.key)));
        json.push_str(&format!(
            "      \"variable\": {},\n",
            json_string(&row.variable)
        ));
        json.push_str(&format!(
            "      \"frequency\": {},\n",
            json_string(output_frequency_label(row.frequency))
        ));
        json.push_str(&format!(
            "      \"units\": {},\n",
            row.oracle_units
                .as_ref()
                .map_or_else(|| "null".to_string(), |units| json_string(units))
        ));
        json.push_str(&format!("      \"samples\": {}\n", row.expected_samples));
        json.push_str("    }");
        if index + 1 < context.rows.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn render_result_store_json(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let diagnostics = context.result_store.diagnostics();
    let profile = context.result_store.profile();
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!(
        "  \"series_count\": {},\n",
        context.result_store.series.len()
    ));
    json.push_str(&format!(
        "  \"sample_count\": {},\n",
        context.result_store.sample_count()
    ));
    json.push_str("  \"profile\": {\n");
    json.push_str(&format!(
        "    \"series_count\": {},\n",
        profile.series_count
    ));
    json.push_str(&format!(
        "    \"sample_count\": {},\n",
        profile.sample_count
    ));
    json.push_str(&format!(
        "    \"empty_series_count\": {}\n",
        profile.empty_series_count
    ));
    json.push_str("  },\n");
    json.push_str("  \"duplicate_guard\": \"ep_runtime::ResultStore::diagnostics\",\n");
    json.push_str(&format!(
        "  \"diagnostic_count\": {},\n",
        diagnostics.diagnostics.len()
    ));
    json.push_str("  \"diagnostics\": [\n");
    for (index, diagnostic) in diagnostics.diagnostics.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"code\": {},\n",
            json_string(&format!("{:?}", diagnostic.code))
        ));
        json.push_str(&format!(
            "      \"message\": {},\n",
            json_string(&diagnostic.message)
        ));
        json.push_str(&format!(
            "      \"handle\": {}\n",
            diagnostic
                .handle
                .map_or_else(|| "null".to_string(), |handle| handle.0.to_string())
        ));
        json.push_str("    }");
        if index + 1 < diagnostics.diagnostics.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ],\n");
    json.push_str("  \"series\": [\n");
    for (index, series) in context.result_store.series.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"handle\": {},\n", series.handle.0));
        json.push_str(&format!("      \"key\": {},\n", json_string(&series.key)));
        json.push_str(&format!(
            "      \"variable_name\": {},\n",
            json_string(&series.variable_name)
        ));
        json.push_str(&format!(
            "      \"units\": {},\n",
            json_string(&series.units)
        ));
        json.push_str("      \"values\": [");
        for (value_index, value) in series.values.iter().enumerate() {
            if value_index > 0 {
                json.push_str(", ");
            }
            json.push_str(&json_number(*value));
        }
        json.push_str("]\n");
        json.push_str("    }");
        if index + 1 < context.result_store.series.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn render_variable_deltas_csv(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let mut csv = String::from(
        "key,variable,domain,class,level,expected_samples,observed_samples,compared_samples,max_abs_delta,mean_abs_delta,rmse_delta,max_rel_delta,status\n",
    );
    for row in &context.rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&row.key),
            csv_cell(&row.variable),
            row.domain.map_or("unspecified", evidence_domain_label),
            variable_class_label(row.variable_class),
            optional_output_level_label(row.level),
            row.expected_samples,
            row.observed_samples,
            row.compared_samples,
            json_number(row.max_abs_delta),
            json_number(row.mean_abs_delta),
            json_number(row.rmse_delta),
            json_number(row.max_rel_delta),
            status_label(row.status)
        ));
    }
    csv
}

fn render_first_divergence_csv(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let mut csv =
        String::from("key,variable,index,timestamp,kind,expected,observed,abs_delta,rel_delta\n");
    for row in &context.rows {
        let Some(divergence) = row.first_divergence.as_ref() else {
            continue;
        };
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&row.key),
            csv_cell(&row.variable),
            divergence.index,
            csv_cell(divergence.timestamp.as_deref().unwrap_or("")),
            divergence_kind_label(divergence.kind),
            optional_number_csv(divergence.expected),
            optional_number_csv(divergence.observed),
            optional_number_csv(divergence.abs_delta),
            optional_number_csv(divergence.rel_delta)
        ));
    }
    csv
}

fn render_tolerance_failures_csv(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let mut csv = String::from(
        "key,variable,domain,class,level,max_abs_delta,rmse_delta,max_abs_tolerance,max_rmse_tolerance,status\n",
    );
    for row in &context.rows {
        if row.status == SeriesComparisonStatus::Pass {
            continue;
        }
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&row.key),
            csv_cell(&row.variable),
            row.domain.map_or("unspecified", evidence_domain_label),
            variable_class_label(row.variable_class),
            optional_output_level_label(row.level),
            json_number(row.max_abs_delta),
            json_number(row.rmse_delta),
            json_number(row.tolerance.absolute),
            row.max_rmse_tolerance
                .map_or_else(|| "null".to_string(), json_number),
            status_label(row.status)
        ));
    }
    for row in &context.meter_rows {
        if row.status == SeriesComparisonStatus::Pass {
            continue;
        }
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            "",
            csv_cell(&row.name),
            evidence_domain_label(row.domain),
            "meter",
            output_level_label(row.level),
            json_number(row.max_abs_delta),
            json_number(row.rmse_delta),
            json_number(row.tolerance.absolute),
            row.max_rmse_tolerance
                .map_or_else(|| "null".to_string(), json_number),
            status_label(row.status)
        ));
    }
    csv
}

fn render_stage_summary_json(context: &IdealLoadsDiagnosticContext<'_>) -> String {
    let zone_equipment_dispatch_issues = context.zone_equipment_dispatch.issue_codes();
    let zone_equipment_dispatch_warnings = context.zone_equipment_dispatch.warning_codes();
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.manifest.id)
    ));
    json.push_str(&format!("  \"branch\": {},\n", json_string(context.branch)));
    json.push_str(&format!(
        "  \"source_order_wrapper\": {},\n",
        json_string(IDEAL_LOADS_NO_OA_SOURCE_ORDER_WRAPPER)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_invocation_path\": {},\n",
        json_string(IDEAL_LOADS_INVOCATION_PATH)
    ));
    json.push_str(&format!(
        "  \"direct_calc_helper_invocation\": {},\n",
        IDEAL_LOADS_DIRECT_CALC_HELPER_INVOCATION
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_execution_boundary\": {},\n",
        json_string(IDEAL_LOADS_ZONE_EQUIPMENT_EXECUTION_BOUNDARY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_runtime_binding_source\": {},\n",
        json_string(IDEAL_LOADS_RUNTIME_BINDING_SOURCE)
    ));
    json.push_str(&format!(
        "  \"purchased_air_name_lookup_policy\": {},\n",
        json_string(IDEAL_LOADS_RUNTIME_STRING_LOOKUP_POLICY)
    ));
    json.push_str(&format!(
        "  \"selected_purchased_air_branch\": {},\n",
        json_string(context.selected_purchased_air_branch)
    ));
    json.push_str(&format!(
        "  \"declared_ideal_loads_branch\": {},\n",
        json_string(context.declared_ideal_loads_branch)
    ));
    json.push_str(&format!(
        "  \"inactive_branches\": {},\n",
        json_string_array(&context.inactive_branches)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_feature_flags\": {},\n",
        ideal_loads_feature_flags_json(context.feature_flags)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_feature_dispatch_policy\": {},\n",
        json_string(IDEAL_LOADS_FEATURE_DISPATCH_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_prebound_id_contract\": {},\n",
        json_string(IDEAL_LOADS_PREBOUND_ID_CONTRACT)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_psychrometric_evaluation_policy\": {},\n",
        json_string(IDEAL_LOADS_PSYCHROMETRIC_EVALUATION_POLICY)
    ));
    json.push_str(&format!(
        "  \"ideal_loads_psychrometric_cache_policy\": {},\n",
        json_string(IDEAL_LOADS_PSYCHROMETRIC_CACHE_POLICY)
    ));
    push_ideal_loads_output_handle_policy_json(&mut json);
    json.push_str(&format!(
        "  \"trace_level\": {},\n",
        json_string(ideal_loads_trace_level(context.manifest))
    ));
    json.push_str(&format!(
        "  \"trace_level_source\": {},\n",
        json_string(ideal_loads_trace_level_source(context.manifest))
    ));
    json.push_str(&format!(
        "  \"trace_payload\": {},\n",
        json_string(IDEAL_LOADS_NO_OA_TRACE_PAYLOAD)
    ));
    json.push_str(&format!(
        "  \"trace_side_effect_policy\": {},\n",
        json_string(IDEAL_LOADS_TRACE_SIDE_EFFECT_POLICY)
    ));
    json.push_str(&format!(
        "  \"trace_result_invariance_policy\": {},\n",
        json_string(IDEAL_LOADS_TRACE_RESULT_INVARIANCE_POLICY)
    ));
    json.push_str(&format!(
        "  \"trace_overhead_accounting\": {},\n",
        json_string(IDEAL_LOADS_TRACE_OVERHEAD_ACCOUNTING)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_path\": {},\n",
        json_string(IDEAL_LOADS_ZONE_EQUIPMENT_DISPATCH_PATH)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_validation\": {},\n",
        json_string(context.zone_equipment_dispatch.dispatch_status_label())
    ));
    json.push_str(&format!(
        "  \"zone_equipment_conformance_candidate\": {},\n",
        json_string(
            context
                .zone_equipment_dispatch
                .conformance_candidate_status_label()
        )
    ));
    json.push_str(&format!(
        "  \"zone_equipment_scope\": {},\n",
        json_string(context.zone_equipment_dispatch.scope_label())
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_issues\": {},\n",
        json_string_array(&zone_equipment_dispatch_issues)
    ));
    json.push_str(&format!(
        "  \"zone_equipment_dispatch_warnings\": {},\n",
        json_string_array(&zone_equipment_dispatch_warnings)
    ));
    json.push_str(&format!(
        "  \"source_map_anchor\": {},\n",
        json_string(IDEAL_LOADS_SOURCE_MAP_ANCHOR)
    ));
    json.push_str(&format!(
        "  \"node_output_timestamp_alignment\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_TIMESTAMP_ALIGNMENT)
    ));
    json.push_str(&format!(
        "  \"node_output_store_type\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE)
    ));
    json.push_str(&format!(
        "  \"node_output_state_struct\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_STATE_STRUCT)
    ));
    json.push_str(&format!(
        "  \"node_output_update_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_UPDATE_SOURCE)
    ));
    json.push_str(&format!(
        "  \"node_output_report_source\": {},\n",
        json_string(IDEAL_LOADS_NODE_OUTPUT_REPORT_SOURCE)
    ));
    json.push_str("  \"outdoor_air\": false,\n");
    json.push_str("  \"economizer\": \"NoEconomizer\",\n");
    json.push_str("  \"heat_recovery\": \"None\",\n");
    json.push_str("  \"humidity_control_conformance\": false,\n");
    json.push_str(&format!(
        "  \"finite_limit_conformance\": {},\n",
        context.manifest.conformance_claim && context.branch == "no-oa-finite-limit-sensible"
    ));
    json.push_str(&format!(
        "  \"heating_fuel_efficiency\": {},\n",
        json_number(context.fuel_efficiency.heating)
    ));
    json.push_str(&format!(
        "  \"cooling_fuel_efficiency\": {},\n",
        json_number(context.fuel_efficiency.cooling)
    ));
    json.push_str(&format!(
        "  \"fuel_energy_rate_source\": {},\n",
        json_string(fuel_energy_report_source(context))
    ));
    json.push_str(&format!(
        "  \"rate_output_source\": {},\n",
        json_string(IDEAL_LOADS_RATE_OUTPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"rate_output_timestep_source\": {},\n",
        json_string(IDEAL_LOADS_RATE_OUTPUT_TIMESTEP_SOURCE)
    ));
    json.push_str(&format!(
        "  \"energy_output_timestep_source\": {},\n",
        json_string(IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE)
    ));
    json.push_str(&format!(
        "  \"energy_output_level_policy\": {},\n",
        json_string(report_energy_output_level_policy(context))
    ));
    json.push_str(&format!(
        "  \"fuel_energy_output_level_policy\": {},\n",
        json_string(fuel_energy_output_level_policy(context))
    ));
    json.push_str(&format!(
        "  \"meter_aggregation_source\": {},\n",
        json_string(IDEAL_LOADS_METER_AGGREGATION_SOURCE)
    ));
    json.push_str(&format!(
        "  \"meter_fuel_energy_binding_source\": {},\n",
        json_string(IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE)
    ));
    json.push_str("  \"meter_time_series_comparison\": true,\n");
    json.push_str(&format!(
        "  \"meter_series_count\": {},\n",
        context.meter_rows.len()
    ));
    json.push_str(&format!(
        "  \"zone_demand_source\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE)
    ));
    json.push_str(&format!(
        "  \"zone_demand_struct_source\": {},\n",
        json_string(&format!(
            "{}::{}",
            ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE, ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT
        ))
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_field\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD)
    ));
    json.push_str(&format!(
        "  \"zone_demand_heating_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_field\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD)
    ));
    json.push_str(&format!(
        "  \"zone_demand_cooling_sign_convention\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_mismatch_classification\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION)
    ));
    json.push_str(&format!(
        "  \"zone_demand_fixture_mode\": {},\n",
        json_string(ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE)
    ));
    json.push_str("  \"zone_state_source\": \"source-order pre-update zone air node state\",\n");
    json.push_str(&format!(
        "  \"zone_air_node\": {},\n",
        json_string(&context.zone_air_node_name)
    ));
    json.push_str(&format!(
        "  \"recirculation_node\": {},\n",
        context
            .recirculation_node_name
            .as_ref()
            .map_or_else(|| "null".to_string(), |name| json_string(name))
    ));
    if context.recirculation_node_name.is_some() {
        json.push_str(&format!(
            "  \"recirculation_state_source\": {},\n",
            json_string(ideal_loads_recirculation_state_source(context.branch))
        ));
    }
    json.push_str("  \"zone_demand_synthetic_rc_model\": false,\n");
    json.push_str("  \"stages\": [\n");
    let stages = ideal_loads_zone_equipment_stages();
    for (index, stage) in stages.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"stage_name\": {},\n",
            json_string(stage.stage_name)
        ));
        json.push_str(&format!(
            "      \"source_file\": {},\n",
            json_string(stage.source_file)
        ));
        json.push_str(&format!(
            "      \"source_routine\": {}\n",
            json_string(stage.source_routine)
        ));
        json.push_str("    }");
        if index + 1 < stages.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ],\n");
    json.push_str("  \"purchased_air_stages\": [\n");
    let purchased_air_stages = purchased_air_source_order_stages();
    for (index, stage) in purchased_air_stages.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"stage_name\": {},\n",
            json_string(stage.stage_name)
        ));
        json.push_str(&format!(
            "      \"source_file\": {},\n",
            json_string(stage.source_file)
        ));
        json.push_str(&format!(
            "      \"source_routine\": {},\n",
            json_string(stage.source_routine)
        ));
        json.push_str(&format!(
            "      \"rust_equivalent\": {}\n",
            json_string(stage.rust_equivalent)
        ));
        json.push_str("    }");
        if index + 1 < purchased_air_stages.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn domain_status_json(rows: &[IdealLoadsDiagnosticRow]) -> String {
    let mut domains: BTreeMap<&'static str, (usize, usize)> = BTreeMap::new();
    for row in rows {
        let entry = domains
            .entry(row.domain.map_or("unspecified", evidence_domain_label))
            .or_insert((0, 0));
        entry.0 += 1;
        if row.status == SeriesComparisonStatus::Fail {
            entry.1 += 1;
        }
    }
    let mut json = String::from("{");
    for (index, (domain, (series, failures))) in domains.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            "{}: {{\"series\": {}, \"failures\": {}, \"status\": {}}}",
            json_string(domain),
            series,
            failures,
            json_string(if *failures == 0 { "pass" } else { "fail" })
        ));
    }
    json.push('}');
    json
}

impl IdealLoadsDiagnosticRow {
    fn unit_match(&self) -> bool {
        self.oracle_units
            .as_ref()
            .is_some_and(|oracle_units| oracle_units == &self.units)
    }
}

impl IdealLoadsMeterDiagnosticRow {
    fn unit_match(&self) -> bool {
        self.oracle_units
            .as_ref()
            .is_some_and(|oracle_units| oracle_units == &self.units)
    }
}

fn overall_status(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    let has_conformance_output = context
        .rows
        .iter()
        .any(|row| row.level == Some(OutputLevel::Conformance));
    let has_conformance_meter = context
        .meter_rows
        .iter()
        .any(|row| row.level == OutputLevel::Conformance);
    if !has_conformance_output && !has_conformance_meter && !context.manifest.conformance_claim {
        "diagnostic"
    } else if !has_conformance_output && !has_conformance_meter {
        "fail"
    } else if context
        .rows
        .iter()
        .filter(|row| row.level == Some(OutputLevel::Conformance))
        .all(|row| row.status == SeriesComparisonStatus::Pass)
        && context
            .meter_rows
            .iter()
            .filter(|row| row.level == OutputLevel::Conformance)
            .all(|row| row.status == SeriesComparisonStatus::Pass)
    {
        "pass"
    } else {
        "fail"
    }
}

fn outdoor_air_overall_status(context: &IdealLoadsOutdoorAirDiagnosticContext<'_>) -> &'static str {
    let conformance_rows = context
        .rows
        .iter()
        .filter(|row| row.level == Some(OutputLevel::Conformance))
        .collect::<Vec<_>>();
    if conformance_rows.is_empty() && !context.manifest.conformance_claim {
        "diagnostic"
    } else if conformance_rows.is_empty() {
        "fail"
    } else if conformance_rows
        .iter()
        .all(|row| row.status == SeriesComparisonStatus::Pass)
    {
        "pass"
    } else {
        "fail"
    }
}

fn tolerance_policy(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    if context.manifest.conformance_claim {
        "conformance-gate"
    } else {
        "diagnostic-draft"
    }
}

fn outdoor_air_tolerance_policy(
    context: &IdealLoadsOutdoorAirDiagnosticContext<'_>,
) -> &'static str {
    if context.manifest.conformance_claim {
        "conformance-gate"
    } else {
        "diagnostic-draft"
    }
}

fn tolerance_failures_count(context: &IdealLoadsDiagnosticContext<'_>) -> usize {
    let output_failures = context
        .rows
        .iter()
        .filter(|row| row.status == SeriesComparisonStatus::Fail)
        .count();
    let conformance_meter_failures = context
        .meter_rows
        .iter()
        .filter(|row| {
            row.level == OutputLevel::Conformance && row.status == SeriesComparisonStatus::Fail
        })
        .count();
    output_failures + conformance_meter_failures
}

fn facility_meter_report_source(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    if manifest_is_humidity_annual_facility_meter_conformance_candidate(context.manifest) {
        IDEAL_LOADS_HUMIDITY_ANNUAL_FACILITY_METER_CONFORMANCE_REPORT_SOURCE
    } else if manifest_is_humidity_report_purchased_air_conformance_candidate(context.manifest)
        && context.meter_rows.iter().any(|row| {
            row.level == OutputLevel::Conformance && row.frequency != OutputFrequency::Hourly
        })
    {
        IDEAL_LOADS_HUMIDITY_FACILITY_METER_CONFORMANCE_REPORT_SOURCE
    } else if context.meter_rows.iter().any(|row| {
        row.level == OutputLevel::Conformance && row.frequency != OutputFrequency::Hourly
    }) {
        IDEAL_LOADS_FACILITY_METER_MONTHLY_RUN_PERIOD_CONFORMANCE_REPORT_SOURCE
    } else if context
        .meter_rows
        .iter()
        .any(|row| row.level == OutputLevel::Conformance)
    {
        IDEAL_LOADS_FACILITY_METER_CONFORMANCE_REPORT_SOURCE
    } else {
        IDEAL_LOADS_FACILITY_METER_DIAGNOSTIC_REPORT_SOURCE
    }
}

fn report_energy_output_level_policy(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    if manifest_is_no_oa_report_energy_conformance_candidate(context.manifest) {
        IDEAL_LOADS_NO_OA_REPORT_ENERGY_CONFORMANCE_POLICY
    } else if manifest_is_humidity_report_purchased_air_conformance_candidate(context.manifest) {
        IDEAL_LOADS_HUMIDITY_REPORT_ENERGY_CONFORMANCE_POLICY
    } else {
        IDEAL_LOADS_ENERGY_OUTPUT_LEVEL_POLICY
    }
}

fn report_energy_source_policy(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    if manifest_is_no_oa_report_energy_conformance_candidate(context.manifest) {
        "declared non-fuel energy conformance"
    } else if manifest_is_humidity_report_purchased_air_conformance_candidate(context.manifest) {
        "declared humidity-control non-fuel energy conformance"
    } else {
        "diagnostic-only"
    }
}

fn fuel_energy_output_level_policy(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    if manifest_is_non_constant_fuel_efficiency_conformance_candidate(context.manifest) {
        IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_POLICY
    } else if manifest_is_blank_fuel_efficiency_conformance_candidate(context.manifest) {
        IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_POLICY
    } else if manifest_is_constant_fuel_efficiency_conformance_candidate(context.manifest) {
        IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_POLICY
    } else if manifest_is_humidity_report_purchased_air_conformance_candidate(context.manifest) {
        IDEAL_LOADS_HUMIDITY_FUEL_EFFICIENCY_CONFORMANCE_POLICY
    } else {
        IDEAL_LOADS_FUEL_ENERGY_OUTPUT_LEVEL_POLICY
    }
}

fn fuel_energy_report_source(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    if manifest_is_non_constant_fuel_efficiency_conformance_candidate(context.manifest) {
        IDEAL_LOADS_NON_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_REPORT_SOURCE
    } else if manifest_is_blank_fuel_efficiency_conformance_candidate(context.manifest) {
        IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_REPORT_SOURCE
    } else if manifest_is_constant_fuel_efficiency_conformance_candidate(context.manifest) {
        IDEAL_LOADS_CONSTANT_FUEL_EFFICIENCY_CONFORMANCE_REPORT_SOURCE
    } else if manifest_is_humidity_report_purchased_air_conformance_candidate(context.manifest) {
        IDEAL_LOADS_BLANK_FUEL_EFFICIENCY_CONFORMANCE_REPORT_SOURCE
    } else {
        context.fuel_efficiency.report_source
    }
}

fn claim_boundary(context: &IdealLoadsDiagnosticContext<'_>) -> &'static str {
    if context.manifest.conformance_claim && context.branch == "no-oa-finite-limit-sensible" {
        "conformance no-OA finite-limit sensible IdealLoads branch for declared variables only"
    } else if context.constant_shr_conformance_claim {
        "conformance no-OA ConstantSensibleHeatRatio cooling IdealLoads branch for declared variables only"
    } else if context.constant_supply_humidity_cooling_conformance_claim {
        "conformance no-OA ConstantSupplyHumidityRatio cooling IdealLoads branch for declared heating/cooling rate rows, supply-node rows, ReportPurchasedAir energy rows, blank fuel-efficiency rows, and hourly/monthly/run-period facility meters only"
    } else if context.constant_supply_humidity_heating_conformance_claim {
        "conformance no-OA ConstantSupplyHumidityRatio heating IdealLoads branch for declared heating/cooling rate rows, supply-node rows, ReportPurchasedAir energy rows, blank fuel-efficiency rows, and hourly/monthly/run-period facility meters only"
    } else if context.humidistat_dehumidification_conformance_claim {
        "conformance no-OA Humidistat dehumidification IdealLoads branch for declared heating/cooling rate rows, supply-node rows, moisture-demand rows, ReportPurchasedAir energy rows, blank fuel-efficiency rows, and hourly/monthly/run-period facility meters only"
    } else if context.humidistat_humidification_conformance_claim {
        "conformance no-OA Humidistat humidification IdealLoads branch for declared heating/cooling rate rows, supply-node rows, moisture-demand rows, ReportPurchasedAir energy rows, blank fuel-efficiency rows, and hourly/monthly/run-period facility meters only"
    } else if context.humidity_annual_facility_meter_conformance_claim {
        "conformance no-OA full-year humidity-control annual IdealLoads facility meter aggregation for declared facility meters only"
    } else if manifest_is_no_oa_facility_meter_monthly_run_period_conformance_candidate(
        context.manifest,
    ) {
        "conformance no-OA monthly/annual/run-period IdealLoads facility meter aggregation for declared facility meters only"
    } else if manifest_is_no_oa_facility_meter_conformance_candidate(context.manifest) {
        "conformance no-OA hourly IdealLoads facility meter aggregation for declared facility meters only"
    } else if manifest_is_no_oa_report_energy_conformance_candidate(context.manifest) {
        "conformance no-OA ReportPurchasedAir rate-to-TimeStepSysSec energy for declared non-fuel energy rows only"
    } else if manifest_is_non_constant_fuel_efficiency_conformance_candidate(context.manifest) {
        "conformance no-OA non-constant Schedule:Compact IdealLoads fuel-efficiency for declared fuel-energy rows only"
    } else if manifest_is_blank_fuel_efficiency_conformance_candidate(context.manifest) {
        "conformance no-OA blank IdealLoads fuel-efficiency for declared fuel-energy rows only"
    } else if manifest_is_constant_fuel_efficiency_conformance_candidate(context.manifest) {
        "conformance no-OA constant Schedule:Constant IdealLoads fuel-efficiency for declared fuel-energy rows only"
    } else if context.manifest.conformance_claim {
        "conformance no-OA/no-limit sensible IdealLoads branch for declared variables only"
    } else if context.branch == "no-oa-finite-limit-sensible" {
        "diagnostic-only no-OA finite-limit sensible IdealLoads branch"
    } else {
        "diagnostic-only no-OA/no-limit sensible IdealLoads branch"
    }
}

fn record_mode(counts: &mut IdealLoadsModeCounts, mode: IdealLoadsSensibleMode) {
    match mode {
        IdealLoadsSensibleMode::Off => counts.off += 1,
        IdealLoadsSensibleMode::Deadband => counts.deadband += 1,
        IdealLoadsSensibleMode::Cooling => counts.cooling += 1,
        IdealLoadsSensibleMode::Heating => counts.heating += 1,
    }
}

fn optional_output_level_label(level: Option<OutputLevel>) -> &'static str {
    level.map_or("unspecified", output_level_label)
}

fn alignment_label(alignment: SeriesAlignment) -> &'static str {
    match alignment {
        SeriesAlignment::Index => "index",
        SeriesAlignment::Timestamp => "timestamp",
    }
}

fn status_label(status: SeriesComparisonStatus) -> &'static str {
    match status {
        SeriesComparisonStatus::Pass => "pass",
        SeriesComparisonStatus::Fail => "fail",
    }
}

fn tolerance_label(tolerance: Tolerance, max_rmse: Option<f64>) -> String {
    format!(
        "abs={} rel={} rmse={}",
        json_number(tolerance.absolute),
        json_number(tolerance.relative),
        max_rmse.map_or_else(|| "none".to_string(), json_number)
    )
}

fn first_divergence_label(divergence: Option<&ep_compare::SeriesDivergenceV2>) -> String {
    let Some(divergence) = divergence else {
        return "none".to_string();
    };
    format!(
        "{} index={} timestamp={} expected={} observed={} abs_delta={}",
        divergence_kind_label(divergence.kind),
        divergence.index,
        divergence.timestamp.as_deref().unwrap_or("none"),
        optional_number_csv(divergence.expected),
        optional_number_csv(divergence.observed),
        optional_number_csv(divergence.abs_delta)
    )
}

fn moisture_history_term_sample_label(
    sample: Option<&IdealLoadsMoistureHistoryTermSample>,
) -> String {
    let Some(sample) = sample else {
        return "none".to_string();
    };
    markdown_cell(&format!(
        "index={} timestamp={} row_lag={:.12} inferred={:.12} delta={:.12}",
        sample.index,
        sample.timestamp.as_deref().unwrap_or("none"),
        sample.row_lag_history_term,
        sample.inferred_history_term,
        sample.row_lag_minus_inferred_delta
    ))
}

fn first_divergence_json(divergence: Option<&ep_compare::SeriesDivergenceV2>) -> String {
    let Some(divergence) = divergence else {
        return "null".to_string();
    };
    format!(
        "{{\"index\": {}, \"timestamp\": {}, \"kind\": {}, \"expected\": {}, \"observed\": {}, \"abs_delta\": {}, \"rel_delta\": {}}}",
        divergence.index,
        divergence
            .timestamp
            .as_ref()
            .map_or_else(|| "null".to_string(), |value| json_string(value)),
        json_string(divergence_kind_label(divergence.kind)),
        optional_number_json(divergence.expected),
        optional_number_json(divergence.observed),
        optional_number_json(divergence.abs_delta),
        optional_number_json(divergence.rel_delta)
    )
}

fn divergence_kind_label(kind: SeriesDivergenceKind) -> &'static str {
    match kind {
        SeriesDivergenceKind::Tolerance => "tolerance",
        SeriesDivergenceKind::MissingExpectedSample => "missing-expected-sample",
        SeriesDivergenceKind::MissingObservedSample => "missing-observed-sample",
    }
}

fn optional_number_json(value: Option<f64>) -> String {
    value.map_or_else(|| "null".to_string(), json_number)
}

fn optional_number_csv(value: Option<f64>) -> String {
    value.map_or_else(|| "".to_string(), json_number)
}

fn csv_cell(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn unsupported_features_label(features: &[IdealLoadsUnsupportedFeature]) -> String {
    if features.is_empty() {
        return "none".to_string();
    }
    features
        .iter()
        .map(|feature| match feature {
            IdealLoadsUnsupportedFeature::OutdoorAir => "outdoor-air",
            IdealLoadsUnsupportedFeature::DemandControlledVentilation => "dcv",
            IdealLoadsUnsupportedFeature::Economizer => "economizer",
            IdealLoadsUnsupportedFeature::HeatRecovery => "heat-recovery",
            IdealLoadsUnsupportedFeature::HeatingLimit => "heating-limit",
            IdealLoadsUnsupportedFeature::CoolingLimit => "cooling-limit",
            IdealLoadsUnsupportedFeature::Humidification => "humidification",
            IdealLoadsUnsupportedFeature::Dehumidification => "dehumidification",
            IdealLoadsUnsupportedFeature::UnresolvedHeatingLimit => "unresolved-heating-limit",
            IdealLoadsUnsupportedFeature::UnresolvedCoolingLimit => "unresolved-cooling-limit",
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod schedule_boundary_tests {
    use super::*;
    use ep_model::{
        ScheduleCompact, ScheduleCompactDayProfile, ScheduleCompactPeriod, ScheduleCompactSegment,
        ScheduleDayType, ScheduleInterpolation,
    };

    fn all_schedule_day_types() -> Vec<ScheduleDayType> {
        vec![
            ScheduleDayType::Sunday,
            ScheduleDayType::Monday,
            ScheduleDayType::Tuesday,
            ScheduleDayType::Wednesday,
            ScheduleDayType::Thursday,
            ScheduleDayType::Friday,
            ScheduleDayType::Saturday,
            ScheduleDayType::Holiday,
            ScheduleDayType::SummerDesignDay,
            ScheduleDayType::WinterDesignDay,
            ScheduleDayType::CustomDay1,
            ScheduleDayType::CustomDay2,
        ]
    }

    fn calendar_varying_schedule(id: ScheduleId) -> ScheduleCompact {
        let other_days = all_schedule_day_types()
            .into_iter()
            .filter(|day_type| *day_type != ScheduleDayType::Tuesday)
            .collect();
        ScheduleCompact {
            id,
            name: NormalizedName::new("Calendar Varying"),
            schedule_type_limits: None,
            periods: vec![ScheduleCompactPeriod {
                through_schedule_day_of_year: 366,
                day_profiles: vec![
                    ScheduleCompactDayProfile {
                        day_types: vec![ScheduleDayType::Tuesday],
                        interpolation: ScheduleInterpolation::No,
                        segments: vec![ScheduleCompactSegment {
                            until_minute_of_day: 24 * 60,
                            value: 1.0,
                        }],
                    },
                    ScheduleCompactDayProfile {
                        day_types: other_days,
                        interpolation: ScheduleInterpolation::No,
                        segments: vec![ScheduleCompactSegment {
                            until_minute_of_day: 24 * 60,
                            value: 2.0,
                        }],
                    },
                ],
            }],
        }
    }

    #[test]
    fn ideal_loads_hour_only_paths_reject_calendar_varying_compact_schedules()
    -> Result<(), Box<dyn std::error::Error>> {
        let schedule_id = ScheduleId(41);
        let model = SimulationModel::from_typed(TypedModel {
            compact_schedules: vec![calendar_varying_schedule(schedule_id)],
            ..TypedModel::default()
        });
        let timestamps = vec![Some("hour=1;end=60".to_string())];

        let optional_error = match ideal_loads_optional_schedule_values(
            &model,
            Some(schedule_id),
            "availability",
            1,
            &timestamps,
        ) {
            Ok(_) => {
                return Err(std::io::Error::other(
                    "IdealLoads optional schedule accepted calendar variation",
                )
                .into());
            }
            Err(error) => error,
        };
        assert!(optional_error.contains("rejects calendar-varying Schedule:Compact"));

        let fuel_error = match ideal_loads_fuel_efficiency_values(
            &model,
            Some(schedule_id),
            "heating",
            1,
            &timestamps,
        ) {
            Ok(_) => {
                return Err(std::io::Error::other(
                    "IdealLoads fuel schedule accepted calendar variation",
                )
                .into());
            }
            Err(error) => error,
        };
        assert!(fuel_error.contains("rejects calendar-varying Schedule:Compact"));

        Ok(())
    }
}
