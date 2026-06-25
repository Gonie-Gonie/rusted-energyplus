//! Source-order PurchasedAir/IdealLoads compatibility wrapper.

use crate::{
    ideal_loads::{
        IdealLoadsFeatureFlags, IdealLoadsInitFlags, IdealLoadsReportSnapshot,
        IdealLoadsSensibleLimitContext, IdealLoadsSensibleResult, IdealLoadsUnsupportedFeature,
        IdealLoadsZoneState, calc_no_oa_no_limit_sensible_with_recirculation_context_compat,
        calc_no_oa_sensible_with_limits_and_recirculation_compat, classify_no_oa_sensible_subset,
        supply_node_update_from_result,
    },
    node::IdealLoadsSupplyNodeUpdate,
    zone_equipment::ZoneSysEnergyDemand,
};
use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, NodeId};

/// Rust-visible branch selected inside `CalcPurchAirLoads`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsPurchasedAirBranch {
    /// No outdoor air, no numeric limits, sensible-only branch.
    NoOaNoLimitSensible,
    /// No outdoor air with a numeric capacity limit.
    NoOaFiniteCapacity,
    /// No outdoor air with a numeric flow limit.
    NoOaFiniteFlow,
    /// No outdoor air with numeric flow and capacity limits.
    NoOaFiniteFlowAndCapacity,
    /// No outdoor air ConstantSensibleHeatRatio cooling branch.
    NoOaConstantSensibleHeatRatioCooling,
    /// No outdoor air ConstantSupplyHumidityRatio cooling branch.
    NoOaConstantSupplyHumidityCooling,
    /// No outdoor air ConstantSupplyHumidityRatio heating branch.
    NoOaConstantSupplyHumidityHeating,
    /// No outdoor air humidistat dehumidification branch.
    NoOaHumidistatDehumidification,
    /// No outdoor air humidistat humidification branch.
    NoOaHumidistatHumidification,
}

impl IdealLoadsPurchasedAirBranch {
    /// Stable stage-summary label for the branch.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoOaNoLimitSensible => "no_oa_sensible",
            Self::NoOaFiniteCapacity => "finite_capacity",
            Self::NoOaFiniteFlow => "finite_flow",
            Self::NoOaFiniteFlowAndCapacity => "flow_and_capacity",
            Self::NoOaConstantSensibleHeatRatioCooling => "constant_shr",
            Self::NoOaConstantSupplyHumidityCooling => "constant_supply_humidity_cooling",
            Self::NoOaConstantSupplyHumidityHeating => "constant_supply_humidity_heating",
            Self::NoOaHumidistatDehumidification => "humidistat_dehumidification",
            Self::NoOaHumidistatHumidification => "humidistat_humidification",
        }
    }

    const fn uses_finite_limit_calc(self) -> bool {
        matches!(
            self,
            Self::NoOaFiniteCapacity | Self::NoOaFiniteFlow | Self::NoOaFiniteFlowAndCapacity
        )
    }
}

/// Source-order stage metadata for PurchasedAir compatibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdealLoadsPurchasedAirStage {
    /// Stable Rust stage name.
    pub stage_name: &'static str,
    /// EnergyPlus source file.
    pub source_file: &'static str,
    /// EnergyPlus source routine.
    pub source_routine: &'static str,
    /// Rust-side equivalent boundary.
    pub rust_equivalent: &'static str,
}

/// PurchasedAir source-order stages preserved by `sim_purchased_air_compat`.
#[must_use]
pub const fn purchased_air_source_order_stages() -> [IdealLoadsPurchasedAirStage; 5] {
    [
        IdealLoadsPurchasedAirStage {
            stage_name: "get-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "GetPurchasedAir",
            rust_equivalent: "compile-stage typed IdealLoadsAirSystemId binding",
        },
        IdealLoadsPurchasedAirStage {
            stage_name: "init-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "InitPurchasedAir",
            rust_equivalent: "IdealLoadsInitFlags",
        },
        IdealLoadsPurchasedAirStage {
            stage_name: "calc-purch-air-loads",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "CalcPurchAirLoads",
            rust_equivalent: "branch-specific IdealLoads calc compatibility function",
        },
        IdealLoadsPurchasedAirStage {
            stage_name: "update-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "UpdatePurchasedAir",
            rust_equivalent: "IdealLoadsSupplyNodeUpdate",
        },
        IdealLoadsPurchasedAirStage {
            stage_name: "report-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "ReportPurchasedAir",
            rust_equivalent: "IdealLoadsReportSnapshot",
        },
    ]
}

/// Runtime binding evidence for the IdealLoads compatibility path.
pub const IDEAL_LOADS_RUNTIME_BINDING_SOURCE: &str =
    "compile-stage typed IdealLoadsAirSystemId, ZoneId, and NodeId binding";

/// Runtime string lookup policy for PurchasedAir compatibility.
pub const IDEAL_LOADS_RUNTIME_STRING_LOOKUP_POLICY: &str =
    "PurchAirName string lookup is compile/report only; simulation loop uses prebound typed IDs";

/// Inputs consumed by the source-order PurchasedAir wrapper.
#[derive(Clone, Copy, Debug)]
pub struct SimPurchasedAirCompatInput<'a> {
    /// Prebound typed IdealLoads system.
    pub system: &'a IdealLoadsAirSystem,
    /// Resolved supply node to update.
    pub supply_node: NodeId,
    /// Zone state visible to `CalcPurchAirLoads`.
    pub zone_state: IdealLoadsZoneState,
    /// Recirculation/mixed-air state for no-OA finite-limit and humidity branches.
    pub recirculation_state: IdealLoadsZoneState,
    /// Source-order zone demand snapshot.
    pub demand: ZoneSysEnergyDemand,
    /// Availability-schedule result for this timestep.
    pub unit_available: bool,
    /// Psychrometric and standard-density context.
    pub limit_context: IdealLoadsSensibleLimitContext,
}

/// PurchasedAir wrapper result in source order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimPurchasedAirCompatOutput {
    /// Typed IdealLoads system ID used instead of a runtime string lookup.
    pub system_id: IdealLoadsAirSystemId,
    /// Selected compatibility branch.
    pub branch: IdealLoadsPurchasedAirBranch,
    /// `InitPurchasedAir` equivalent flags.
    pub init_flags: IdealLoadsInitFlags,
    /// `CalcPurchAirLoads` equivalent result.
    pub calculation: IdealLoadsSensibleResult,
    /// `UpdatePurchasedAir` equivalent node write.
    pub supply_node_update: IdealLoadsSupplyNodeUpdate,
    /// `ReportPurchasedAir` equivalent rate snapshot.
    pub report: IdealLoadsReportSnapshot,
    /// Optional diagnostic trace payload for source-order auditing.
    pub trace: IdealLoadsPurchasedAirTrace,
}

/// Diagnostic trace payload for the PurchasedAir wrapper.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsPurchasedAirTrace {
    /// Zone state consumed by the calc stage.
    pub zone_state: IdealLoadsZoneState,
    /// Recirculation state consumed by the calc stage.
    pub recirculation_state: IdealLoadsZoneState,
    /// Zone demand consumed by the calc stage.
    pub demand: ZoneSysEnergyDemand,
    /// Selected compatibility branch.
    pub branch: IdealLoadsPurchasedAirBranch,
}

/// Unsupported PurchasedAir branch for the current compatibility wrapper.
#[derive(Clone, Debug, PartialEq)]
pub struct SimPurchasedAirCompatError {
    /// Typed IdealLoads system ID.
    pub system_id: IdealLoadsAirSystemId,
    /// Unsupported features discovered before calculation.
    pub unsupported_features: Vec<IdealLoadsUnsupportedFeature>,
}

/// Executes the source-order `SimPurchasedAir` equivalent for supported branches.
pub fn sim_purchased_air_compat(
    input: SimPurchasedAirCompatInput<'_>,
) -> Result<SimPurchasedAirCompatOutput, SimPurchasedAirCompatError> {
    let boundary = classify_purchased_air_compat_subset(input.system);
    if !boundary.is_supported() {
        return Err(SimPurchasedAirCompatError {
            system_id: input.system.id,
            unsupported_features: boundary.unsupported_features,
        });
    }

    let branch = select_purchased_air_branch(input.system);
    let init_flags = IdealLoadsInitFlags::no_oa_no_limit_candidate();
    let calculation = if branch.uses_finite_limit_calc() {
        calc_no_oa_sensible_with_limits_and_recirculation_compat(
            input.system,
            input.zone_state,
            input.recirculation_state,
            input.demand,
            input.unit_available,
            input.limit_context,
        )
    } else {
        calc_no_oa_no_limit_sensible_with_recirculation_context_compat(
            input.system,
            input.zone_state,
            input.recirculation_state,
            input.demand,
            input.unit_available,
            input.limit_context,
        )
    };
    let supply_node_update = supply_node_update_from_result(input.supply_node, calculation);
    let report = IdealLoadsReportSnapshot::from(calculation);
    let trace = IdealLoadsPurchasedAirTrace {
        zone_state: input.zone_state,
        recirculation_state: input.recirculation_state,
        demand: input.demand,
        branch,
    };

    Ok(SimPurchasedAirCompatOutput {
        system_id: input.system.id,
        branch,
        init_flags,
        calculation,
        supply_node_update,
        report,
        trace,
    })
}

/// Selects the Rust-visible PurchasedAir branch for a supported no-OA compatibility system.
#[must_use]
pub fn select_purchased_air_branch(system: &IdealLoadsAirSystem) -> IdealLoadsPurchasedAirBranch {
    let feature_flags = IdealLoadsFeatureFlags::from_system(system);

    match (
        feature_flags.has_flow_limit,
        feature_flags.has_capacity_limit,
    ) {
        (true, true) => IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity,
        (true, false) => IdealLoadsPurchasedAirBranch::NoOaFiniteFlow,
        (false, true) => IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity,
        (false, false)
            if system.dehumidification_control_type
                == DehumidificationControlType::ConstantSensibleHeatRatio =>
        {
            IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling
        }
        (false, false)
            if system.dehumidification_control_type
                == DehumidificationControlType::ConstantSupplyHumidityRatio =>
        {
            IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityCooling
        }
        (false, false)
            if system.dehumidification_control_type == DehumidificationControlType::Humidistat =>
        {
            IdealLoadsPurchasedAirBranch::NoOaHumidistatDehumidification
        }
        (false, false)
            if system.humidification_control_type
                == ep_model::HumidificationControlType::ConstantSupplyHumidityRatio =>
        {
            IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityHeating
        }
        (false, false)
            if system.humidification_control_type
                == ep_model::HumidificationControlType::Humidistat =>
        {
            IdealLoadsPurchasedAirBranch::NoOaHumidistatHumidification
        }
        (false, false) => IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible,
    }
}

fn classify_purchased_air_compat_subset(
    system: &IdealLoadsAirSystem,
) -> crate::ideal_loads::IdealLoadsSubsetBoundary {
    let mut boundary = classify_no_oa_sensible_subset(system);
    if supports_no_oa_humidity_selected_branch(system) {
        boundary.unsupported_features.retain(|feature| {
            !matches!(
                feature,
                IdealLoadsUnsupportedFeature::Dehumidification
                    | IdealLoadsUnsupportedFeature::Humidification
            )
        });
    }
    boundary
}

fn supports_no_oa_humidity_selected_branch(system: &IdealLoadsAirSystem) -> bool {
    let feature_flags = IdealLoadsFeatureFlags::from_system(system);
    if feature_flags.has_flow_limit || feature_flags.has_capacity_limit {
        return false;
    }

    matches!(
        (
            system.dehumidification_control_type,
            system.humidification_control_type
        ),
        (
            DehumidificationControlType::ConstantSupplyHumidityRatio
                | DehumidificationControlType::Humidistat,
            ep_model::HumidificationControlType::None
        ) | (
            DehumidificationControlType::None,
            ep_model::HumidificationControlType::ConstantSupplyHumidityRatio
                | ep_model::HumidificationControlType::Humidistat
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::{
        AutosizeOrNumber, DemandControlledVentilationType, HeatRecoveryType, IdealLoadsFuelType,
        IdealLoadsLimit, NormalizedName, OutdoorAirEconomizerType, ZoneId,
    };

    #[test]
    fn purchased_air_source_order_stages_preserve_energyplus_order() {
        let stages = purchased_air_source_order_stages();
        assert_eq!(stages[0].source_routine, "GetPurchasedAir");
        assert_eq!(stages[1].source_routine, "InitPurchasedAir");
        assert_eq!(stages[2].source_routine, "CalcPurchAirLoads");
        assert_eq!(stages[3].source_routine, "UpdatePurchasedAir");
        assert_eq!(stages[4].source_routine, "ReportPurchasedAir");
    }

    #[test]
    fn ideal_loads_feature_flags_capture_compile_specialization_inputs() {
        let mut system = test_system();
        system.design_specification_outdoor_air_object_name = Some(NormalizedName::new("OA SPEC"));
        system.outdoor_air_inlet_node_name = Some(NormalizedName::new("OA NODE"));
        system.demand_controlled_ventilation_type =
            DemandControlledVentilationType::OccupancySchedule;
        system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialDryBulb;
        system.heat_recovery_type = HeatRecoveryType::Sensible;
        system.dehumidification_control_type =
            DehumidificationControlType::ConstantSupplyHumidityRatio;
        system.humidification_control_type = ep_model::HumidificationControlType::Humidistat;
        system.heating_limit = IdealLoadsLimit::LimitFlowRateAndCapacity;
        system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Autosize);
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(500.0));
        system.cooling_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(300.0));

        let flags = IdealLoadsFeatureFlags::from_system(&system);

        assert!(flags.has_outdoor_air);
        assert!(flags.has_economizer);
        assert!(flags.has_heat_recovery);
        assert!(flags.has_dcv);
        assert!(flags.has_humidistat);
        assert!(!flags.has_constant_shr);
        assert!(flags.has_constant_supply_humidity);
        assert!(flags.has_flow_limit);
        assert!(flags.has_capacity_limit);
        assert!(flags.has_autosize);
    }

    #[test]
    fn sim_purchased_air_wrapper_matches_no_limit_calc_and_update() {
        let system = test_system();
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 20.0,
            air_humidity_ratio: 0.008,
        };
        let demand = ZoneSysEnergyDemand::sensible_only(ZoneId(0), 1000.0, 0.0);
        let output = sim_purchased_air_compat(SimPurchasedAirCompatInput {
            system: &system,
            supply_node: NodeId(3),
            zone_state,
            recirculation_state: zone_state,
            demand,
            unit_available: true,
            limit_context: IdealLoadsSensibleLimitContext::default(),
        })
        .expect("supported no-OA/no-limit branch");

        assert_eq!(output.system_id, system.id);
        assert_eq!(
            output.branch,
            IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible
        );
        assert!(output.init_flags.one_time_checked);
        assert_eq!(output.supply_node_update.node, NodeId(3));
        assert_eq!(
            output.supply_node_update.temperature_c,
            output.calculation.supply_temperature_c
        );
        assert_eq!(
            output.report.zone_total_heating_rate_w,
            output.calculation.zone_total_heating_rate_w
        );
    }

    #[test]
    fn sim_purchased_air_wrapper_labels_finite_limit_branch() {
        let mut system = test_system();
        system.heating_limit = IdealLoadsLimit::LimitFlowRateAndCapacity;
        system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.05));
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(500.0));
        system.cooling_limit = IdealLoadsLimit::LimitFlowRateAndCapacity;
        system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.05));
        system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(500.0));

        let state = IdealLoadsZoneState {
            air_temperature_c: 20.0,
            air_humidity_ratio: 0.008,
        };
        let output = sim_purchased_air_compat(SimPurchasedAirCompatInput {
            system: &system,
            supply_node: NodeId(1),
            zone_state: state,
            recirculation_state: state,
            demand: ZoneSysEnergyDemand::sensible_only(ZoneId(0), 1000.0, 0.0),
            unit_available: true,
            limit_context: IdealLoadsSensibleLimitContext::default(),
        })
        .expect("supported numeric finite-limit branch");

        assert_eq!(
            output.branch,
            IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity
        );
        assert_eq!(output.branch.label(), "flow_and_capacity");
    }

    #[test]
    fn sim_purchased_air_wrapper_rejects_unsupported_oa_branch() {
        let mut system = test_system();
        system.outdoor_air_inlet_node_name = Some(NormalizedName::new("OA NODE"));

        let state = IdealLoadsZoneState {
            air_temperature_c: 20.0,
            air_humidity_ratio: 0.008,
        };
        let error = sim_purchased_air_compat(SimPurchasedAirCompatInput {
            system: &system,
            supply_node: NodeId(1),
            zone_state: state,
            recirculation_state: state,
            demand: ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, 0.0),
            unit_available: true,
            limit_context: IdealLoadsSensibleLimitContext::default(),
        })
        .expect_err("OA branch is not in the no-OA wrapper");

        assert_eq!(error.system_id, system.id);
        assert!(
            error
                .unsupported_features
                .contains(&IdealLoadsUnsupportedFeature::OutdoorAir)
        );
    }

    #[test]
    fn sim_purchased_air_wrapper_labels_constant_supply_humidity_selected_branch() {
        let mut system = test_system();
        system.dehumidification_control_type =
            DehumidificationControlType::ConstantSupplyHumidityRatio;

        let state = IdealLoadsZoneState {
            air_temperature_c: 26.0,
            air_humidity_ratio: 0.012,
        };
        let output = sim_purchased_air_compat(SimPurchasedAirCompatInput {
            system: &system,
            supply_node: NodeId(1),
            zone_state: state,
            recirculation_state: state,
            demand: ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -1000.0),
            unit_available: true,
            limit_context: IdealLoadsSensibleLimitContext::default(),
        })
        .expect("supported no-OA ConstantSupplyHumidityRatio selected branch");

        assert_eq!(
            output.branch,
            IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityCooling
        );
        assert_eq!(output.branch.label(), "constant_supply_humidity_cooling");
    }

    fn test_system() -> IdealLoadsAirSystem {
        IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("ZONE ONE IDEAL LOADS"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("ZONE ONE INLETS"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: None,
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::None,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: ep_model::HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        }
    }
}
