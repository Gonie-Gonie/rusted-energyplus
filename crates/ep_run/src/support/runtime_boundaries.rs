use std::collections::BTreeSet;

use ep_model::{
    DehumidificationControlType, HumidificationControlType, MaterialKind, SimulationModel,
    TypedModel,
};
use ep_raw_model::RawModel;
use ep_runtime::{
    IdealLoadsPurchasedAirBranch, IdealLoadsUnsupportedFeature, classify_no_oa_sensible_subset,
    select_purchased_air_branch, validate_ideal_loads_zone_equipment_dispatch,
};

use crate::{
    RunDiagnostic, RunDiagnosticSeverity, RunDiagnostics, RunMode,
    support_registry::{
        CapabilityRegistrySpec, registry_capability, registry_capability_ids_and_missing,
        unsupported_rule_for_object,
    },
};

use super::{MatchedCapabilityEntry, RuntimeClass, SupportObjectEntry, SupportStatus};

pub(super) fn matched_capabilities(
    ids: &[String],
    registry: &CapabilityRegistrySpec,
) -> Vec<MatchedCapabilityEntry> {
    ids.iter()
        .filter_map(|id| registry_capability(registry, id))
        .map(|capability| MatchedCapabilityEntry {
            id: capability.id.clone(),
            domain: capability.domain.clone(),
            support_level: capability.support_level.clone(),
            run_state: capability.run_state.clone(),
            required_objects: capability.required_objects.clone(),
            forbidden_active_features: capability.forbidden_active_features.clone(),
            algorithms: capability.algorithms.clone(),
            evidence_cases: capability.evidence_cases.clone(),
            claim_boundary: capability.claim_boundary.clone(),
        })
        .collect()
}

pub(super) fn runtime_status_for_typed_model(
    typed_model: Option<&TypedModel>,
    registry: &CapabilityRegistrySpec,
) -> (SupportStatus, RuntimeClass, Vec<String>, Vec<String>) {
    let Some(typed_model) = typed_model else {
        return (
            SupportStatus::Unsupported,
            RuntimeClass::None,
            Vec::new(),
            Vec::new(),
        );
    };

    if !typed_model.ideal_loads_air_systems.is_empty() {
        let mut capability_ids = BTreeSet::new();
        let mut selected_runtime_class = None;
        for system in &typed_model.ideal_loads_air_systems {
            let branch = select_purchased_air_branch(system);
            capability_ids.insert(ideal_loads_capability_id_for_branch(branch).to_string());
            let branch_runtime_class = ideal_loads_runtime_class_for_branch(branch);
            selected_runtime_class = Some(match selected_runtime_class {
                Some(existing) => merge_ideal_loads_runtime_class(existing, branch_runtime_class),
                None => branch_runtime_class,
            });
        }

        let runtime_class =
            selected_runtime_class.unwrap_or(RuntimeClass::IdealLoadsNodeStateProjection);
        let status = if runtime_class == RuntimeClass::IdealLoadsNodeStateProjection {
            SupportStatus::SupportedDiagnosticOnly
        } else {
            SupportStatus::SupportedCompatibility
        };
        return runtime_selection_from_registry(
            status,
            runtime_class,
            registry,
            capability_ids.into_iter().collect::<Vec<_>>(),
        );
    }

    runtime_selection_from_registry(
        SupportStatus::SupportedCompatibility,
        RuntimeClass::OneZoneHeatBalanceCompatibility,
        registry,
        vec!["official_1zone_uncontrolled_declared_heat_balance".to_string()],
    )
}

fn runtime_selection_from_registry(
    status: SupportStatus,
    runtime_class: RuntimeClass,
    registry: &CapabilityRegistrySpec,
    capability_ids: Vec<String>,
) -> (SupportStatus, RuntimeClass, Vec<String>, Vec<String>) {
    let (matched_capability_ids, missing_capability_ids) =
        registry_capability_ids_and_missing(registry, capability_ids);
    if missing_capability_ids.is_empty() {
        (
            status,
            runtime_class,
            matched_capability_ids,
            missing_capability_ids,
        )
    } else {
        (
            SupportStatus::Unsupported,
            RuntimeClass::None,
            matched_capability_ids,
            missing_capability_ids,
        )
    }
}
const fn ideal_loads_capability_id_for_branch(
    branch: IdealLoadsPurchasedAirBranch,
) -> &'static str {
    match branch {
        IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible => "ideal_loads_no_oa_sensible",
        IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlow
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity => "ideal_loads_finite_limits",
        IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling => {
            "ideal_loads_constant_shr"
        }
        IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityCooling
        | IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityHeating
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatDehumidification
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatHumidification => {
            "ideal_loads_humidity_selected_branches"
        }
        IdealLoadsPurchasedAirBranch::OutdoorAirSelected => {
            "ideal_loads_outdoor_air_selected_branches"
        }
    }
}

const fn ideal_loads_runtime_class_for_branch(
    branch: IdealLoadsPurchasedAirBranch,
) -> RuntimeClass {
    match branch {
        IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible => {
            RuntimeClass::IdealLoadsNoOaSensibleCompatibility
        }
        IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlow
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity => {
            RuntimeClass::IdealLoadsFiniteLimitCompatibility
        }
        IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling => {
            RuntimeClass::IdealLoadsConstantShrCompatibility
        }
        IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityCooling
        | IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityHeating
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatDehumidification
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatHumidification => {
            RuntimeClass::IdealLoadsHumiditySelectedBranchesCompatibility
        }
        IdealLoadsPurchasedAirBranch::OutdoorAirSelected => {
            RuntimeClass::IdealLoadsOutdoorAirSelectedBranchesCompatibility
        }
    }
}

fn merge_ideal_loads_runtime_class(existing: RuntimeClass, next: RuntimeClass) -> RuntimeClass {
    if existing == next {
        return existing;
    }
    if is_declared_ideal_loads_compatibility(existing)
        && is_declared_ideal_loads_compatibility(next)
    {
        RuntimeClass::IdealLoadsMixedDeclaredCompatibility
    } else {
        RuntimeClass::IdealLoadsNodeStateProjection
    }
}

fn is_declared_ideal_loads_compatibility(runtime_class: RuntimeClass) -> bool {
    matches!(
        runtime_class,
        RuntimeClass::IdealLoadsNoOaSensibleCompatibility
            | RuntimeClass::IdealLoadsFiniteLimitCompatibility
            | RuntimeClass::IdealLoadsConstantShrCompatibility
            | RuntimeClass::IdealLoadsHumiditySelectedBranchesCompatibility
            | RuntimeClass::IdealLoadsOutdoorAirSelectedBranchesCompatibility
            | RuntimeClass::IdealLoadsMixedDeclaredCompatibility
    )
}

fn unsupported_features_for_selected_branch(
    system: &ep_model::IdealLoadsAirSystem,
) -> Vec<IdealLoadsUnsupportedFeature> {
    let branch = select_purchased_air_branch(system);
    let mut unsupported_features = classify_no_oa_sensible_subset(system).unsupported_features;
    if supports_no_oa_humidity_selected_branch(system, branch) {
        unsupported_features.retain(|feature| {
            !matches!(
                feature,
                IdealLoadsUnsupportedFeature::Dehumidification
                    | IdealLoadsUnsupportedFeature::Humidification
            )
        });
    }
    if supports_outdoor_air_selected_branch(system, branch) {
        unsupported_features
            .retain(|feature| !matches!(feature, IdealLoadsUnsupportedFeature::OutdoorAir));
    }
    unsupported_features
}

fn supports_no_oa_humidity_selected_branch(
    system: &ep_model::IdealLoadsAirSystem,
    branch: IdealLoadsPurchasedAirBranch,
) -> bool {
    match branch {
        IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityCooling
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatDehumidification => {
            system.humidification_control_type == HumidificationControlType::None
        }
        IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityHeating
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatHumidification => {
            system.dehumidification_control_type == DehumidificationControlType::None
        }
        IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible
        | IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlow
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity
        | IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling
        | IdealLoadsPurchasedAirBranch::OutdoorAirSelected => false,
    }
}

fn supports_outdoor_air_selected_branch(
    system: &ep_model::IdealLoadsAirSystem,
    branch: IdealLoadsPurchasedAirBranch,
) -> bool {
    matches!(branch, IdealLoadsPurchasedAirBranch::OutdoorAirSelected)
        && system
            .design_specification_outdoor_air_object_name
            .is_some()
}
pub(super) fn assess_typed_runtime_boundaries(
    typed_model: Option<&TypedModel>,
    raw_model: &RawModel,
    registry: &CapabilityRegistrySpec,
    mode: RunMode,
    unsupported_objects: &mut Vec<SupportObjectEntry>,
    diagnostics: &mut RunDiagnostics,
) {
    let Some(typed_model) = typed_model else {
        return;
    };

    if typed_model.zones.is_empty() {
        diagnostics.error(
            "UnsupportedTopology",
            "support",
            "no Zone objects are available for the current runtime",
        );
    }
    if typed_model.zones.len() > 1 {
        diagnostics.error(
            "UnsupportedTopology",
            "support",
            format!(
                "the arbitrary runtime currently supports one-zone cases; found {} zones",
                typed_model.zones.len()
            ),
        );
    }

    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Glazing:RefractionExtinctionMethod",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowGlazingRefractionExtinction)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Glazing:EquivalentLayer",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowGlazingEquivalentLayer)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Gas",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowGas)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Gap:EquivalentLayer",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowGapEquivalentLayer)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:GasMixture",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowGasMixture)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Shade",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowShade)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Shade:EquivalentLayer",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowShadeEquivalentLayer)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Drape:EquivalentLayer",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowDrapeEquivalentLayer)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Screen",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowScreen)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Screen:EquivalentLayer",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowScreenEquivalentLayer)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Blind",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowBlind)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Blind:EquivalentLayer",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowBlindEquivalentLayer)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "Material:RoofVegetation",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::RoofVegetation)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:GlazingGroup:Thermochromic",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowGlazingThermochromicGroup)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:SimpleGlazingSystem",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowSimpleGlazing)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:Gap",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowComplexGap)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "WindowMaterial:ComplexShade",
        typed_model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowComplexShade)
            .count(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "MaterialProperty:VariableAbsorptance",
        typed_model.material_variable_absorptances.len(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "MaterialProperty:PhaseChangeHysteresis",
        typed_model.material_phase_change_hystereses.len(),
    );
    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "MaterialProperty:PhaseChange",
        typed_model.material_phase_changes.len(),
    );

    push_typed_unsupported_object(
        registry,
        unsupported_objects,
        diagnostics,
        "AirLoopHVAC",
        typed_model.air_loops.len(),
    );
    for fan in &typed_model.fans {
        push_typed_unsupported_object(
            registry,
            unsupported_objects,
            diagnostics,
            fan.kind.object_type(),
            1,
        );
    }
    for coil in &typed_model.coils {
        push_typed_unsupported_object(
            registry,
            unsupported_objects,
            diagnostics,
            coil.kind.object_type(),
            1,
        );
    }
    for manager in &typed_model.setpoint_managers {
        push_typed_unsupported_object(
            registry,
            unsupported_objects,
            diagnostics,
            manager.object_type.0.as_str(),
            1,
        );
    }

    if !typed_model.plant_loops.is_empty()
        || !typed_model.plant_branches.is_empty()
        || !typed_model.pumps_constant_speed.is_empty()
        || !typed_model.boilers_hot_water.is_empty()
        || !typed_model.chillers_electric_eir.is_empty()
    {
        push_typed_boundary(
            unsupported_objects,
            diagnostics,
            "PlantLoop/PlantEquipment",
            typed_model.plant_loops.len()
                + typed_model.plant_branches.len()
                + typed_model.pumps_constant_speed.len()
                + typed_model.boilers_hot_water.len()
                + typed_model.chillers_electric_eir.len(),
            "UnsupportedPlantObject",
            "Plant objects are typed for graph diagnostics but not executable in arbitrary-run compatibility mode",
        );
    }

    if !typed_model.ideal_loads_air_systems.is_empty() {
        let simulation_model = SimulationModel::from_typed(typed_model.clone());
        if simulation_model.graph.zone_ideal_loads.is_empty()
            || simulation_model.graph.ideal_loads_supply_nodes.is_empty()
        {
            diagnostics.error(
                "UnsupportedTopology",
                "support",
                "IdealLoads systems require zone equipment connections and resolvable supply nodes",
            );
        }

        for system in &typed_model.ideal_loads_air_systems {
            let validation =
                validate_ideal_loads_zone_equipment_dispatch(&simulation_model, system.id);
            if !validation.is_dispatchable() {
                diagnostics.push(
                    RunDiagnostic::new(
                        RunDiagnosticSeverity::Error,
                        "UnsupportedTopology",
                        "support",
                        format!(
                            "IdealLoads system '{}' is not dispatchable through ZoneEquipmentManager: {:?}",
                            system.name.0,
                            validation.issue_codes()
                        ),
                    )
                    .with_object("ZoneHVAC:IdealLoadsAirSystem", Some(system.name.0.clone())),
                );
            }

            let unsupported_features = unsupported_features_for_selected_branch(system);
            if !unsupported_features.is_empty() {
                diagnostics.push(
                    RunDiagnostic::new(
                        RunDiagnosticSeverity::Error,
                        "UnsupportedHeatBalanceBranch",
                        "support",
                        format!(
                            "IdealLoads system '{}' uses unsupported feature flags: {:?}",
                            system.name.0, unsupported_features
                        ),
                    )
                    .with_object("ZoneHVAC:IdealLoadsAirSystem", Some(system.name.0.clone())),
                );
            }
        }
    }

    if mode == RunMode::Fast || mode == RunMode::Experimental {
        diagnostics.warning(
            "UnsupportedAlgorithm",
            "support",
            format!(
                "{} mode is parsed but currently uses the same runtime boundary as compatibility/diagnostic mode",
                mode.id()
            ),
        );
    }

    warn_for_ignored_semantic_objects(raw_model, diagnostics);
}

fn push_typed_unsupported_object(
    registry: &CapabilityRegistrySpec,
    unsupported_objects: &mut Vec<SupportObjectEntry>,
    diagnostics: &mut RunDiagnostics,
    object_type: &str,
    count: usize,
) {
    if count == 0 || unsupported_rule_for_object(registry, object_type).is_none() {
        return;
    }

    let (code, note) = unsupported_object_reason(registry, object_type);
    unsupported_objects.push(SupportObjectEntry {
        object_type: object_type.to_string(),
        count,
        status: "unsupported".to_string(),
        note: note.clone(),
    });
    diagnostics.push(
        RunDiagnostic::new(
            RunDiagnosticSeverity::Error,
            code,
            "support",
            format!("{object_type} is typed for graph/source-map diagnostics but not executable in arbitrary-run compatibility mode"),
        )
        .with_object(object_type.to_string(), None),
    );
}

fn push_typed_boundary(
    unsupported_objects: &mut Vec<SupportObjectEntry>,
    diagnostics: &mut RunDiagnostics,
    label: &str,
    count: usize,
    code: &str,
    note: &str,
) {
    unsupported_objects.push(SupportObjectEntry {
        object_type: label.to_string(),
        count,
        status: "unsupported".to_string(),
        note: note.to_string(),
    });
    diagnostics.error(code, "support", note);
}

fn warn_for_ignored_semantic_objects(raw_model: &RawModel, diagnostics: &mut RunDiagnostics) {
    for object_type in raw_model.objects.keys() {
        if matches!(
            object_type.0.as_str(),
            "HeatBalanceAlgorithm"
                | "SimulationControl"
                | "SizingPeriod:DesignDay"
                | "Exterior:Lights"
        ) {
            diagnostics.push(
                RunDiagnostic::new(
                    RunDiagnosticSeverity::Warning,
                    "UnsupportedAlgorithmIgnored",
                    "support",
                    format!(
                        "{} is recorded but not currently evaluated by the Rust arbitrary runtime",
                        object_type.0
                    ),
                )
                .with_object(object_type.0.clone(), None),
            );
        }
    }
}

pub(super) fn unsupported_object_reason(
    registry: &CapabilityRegistrySpec,
    object_type: &str,
) -> (String, String) {
    if let Some(rule) = unsupported_rule_for_object(registry, object_type) {
        return (
            unsupported_rule_code(rule.id.as_str(), object_type),
            rule.reason.clone(),
        );
    }

    (
        "UnsupportedObject".to_string(),
        "object type is preserved in RawModel but lacks a runtime support declaration".to_string(),
    )
}

fn unsupported_rule_code(rule_id: &str, object_type: &str) -> String {
    if object_type.starts_with("EnergyManagementSystem:") {
        return "UnsupportedEMS".to_string();
    }
    if object_type.starts_with("PythonPlugin:") {
        return "UnsupportedPythonPlugin".to_string();
    }
    if object_type.starts_with("AirflowNetwork:") {
        return "UnsupportedAirflowNetwork".to_string();
    }

    match rule_id {
        "unsupported_hvac_air_loop" => "UnsupportedHVACObject",
        "unsupported_hvac_zone_equipment" => "UnsupportedHVACObject",
        "unsupported_plant" => "UnsupportedPlantObject",
        "unsupported_ems_python_airflow" => "UnsupportedRuntimeModifier",
        "unsupported_sizing" => "UnsupportedSizing",
        "unsupported_surface_boundary" => "UnsupportedSurfaceBoundary",
        _ => "UnsupportedObject",
    }
    .to_string()
}
