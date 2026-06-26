//! HeatBalanceSurfaceManager source-order stage contract.

use crate::RuntimeError;
use crate::execution_plan::{EnergyPlusCompatibilityStage, ExecutionStageKind};
use ep_model::{ConstructionId, MaterialId, MaterialSurfaceRoughness, Surface, TypedModel};

const SOURCE_FILE: &str = "src/EnergyPlus/HeatBalanceSurfaceManager.cc";
const DEFAULT_MATERIAL_THERMAL_ABSORPTANCE: f64 = 0.9;
const DEFAULT_MATERIAL_SOLAR_ABSORPTANCE: f64 = 0.7;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceThermalProperties {
    pub(crate) construction_id: ConstructionId,
    pub(crate) construction_name: String,
    pub(crate) outside_layer_material_id: MaterialId,
    pub(crate) outside_layer_material_name: String,
    pub(crate) outside_layer_roughness: MaterialSurfaceRoughness,
    pub(crate) thermal_resistance_m2_k_per_w: f64,
    pub(crate) heat_capacity_j_per_m2_k: Option<f64>,
    pub(crate) thermal_absorptance: f64,
    pub(crate) inside_thermal_absorptance: f64,
    pub(crate) solar_absorptance: f64,
}

pub(crate) type ConstructionThermalData = SurfaceThermalProperties;

pub(crate) fn surface_thermal_properties(
    model: &TypedModel,
    surface: &Surface,
) -> Result<ConstructionThermalData, RuntimeError> {
    let construction = model
        .constructions
        .iter()
        .find(|construction| construction.id == surface.construction)
        .ok_or_else(|| RuntimeError::MissingConstruction {
            surface_name: surface.name.0.clone(),
        })?;
    let layer_ids = if construction.layers.is_empty() {
        std::slice::from_ref(&construction.outside_layer)
    } else {
        construction.layers.as_slice()
    };
    let mut layer_materials = Vec::with_capacity(layer_ids.len());
    for layer_id in layer_ids {
        let material = model
            .materials
            .iter()
            .find(|material| material.id == *layer_id)
            .ok_or_else(|| RuntimeError::MissingMaterial {
                construction_name: construction.name.0.clone(),
            })?;
        layer_materials.push(material);
    }
    let outside_material =
        layer_materials
            .first()
            .ok_or_else(|| RuntimeError::MissingMaterial {
                construction_name: construction.name.0.clone(),
            })?;
    let inside_material = layer_materials
        .last()
        .ok_or_else(|| RuntimeError::MissingMaterial {
            construction_name: construction.name.0.clone(),
        })?;
    let mut thermal_resistance_m2_k_per_w = 0.0;
    for material in &layer_materials {
        thermal_resistance_m2_k_per_w += material.thermal_resistance().ok_or_else(|| {
            RuntimeError::MissingThermalResistance {
                material_name: material.name.0.clone(),
            }
        })?;
    }
    let heat_capacity_j_per_m2_k = layer_materials
        .iter()
        .filter_map(|material| material.heat_capacity_per_area())
        .sum::<f64>();
    let heat_capacity_j_per_m2_k = if heat_capacity_j_per_m2_k > 0.0 {
        Some(heat_capacity_j_per_m2_k)
    } else {
        None
    };

    Ok(SurfaceThermalProperties {
        construction_id: construction.id,
        construction_name: construction.name.0.clone(),
        outside_layer_material_id: outside_material.id,
        outside_layer_material_name: outside_material.name.0.clone(),
        outside_layer_roughness: outside_material
            .roughness
            .unwrap_or(MaterialSurfaceRoughness::MediumRough),
        thermal_resistance_m2_k_per_w,
        heat_capacity_j_per_m2_k,
        thermal_absorptance: outside_material
            .thermal_absorptance
            .unwrap_or(DEFAULT_MATERIAL_THERMAL_ABSORPTANCE),
        inside_thermal_absorptance: inside_material
            .thermal_absorptance
            .unwrap_or(DEFAULT_MATERIAL_THERMAL_ABSORPTANCE),
        solar_absorptance: outside_material
            .solar_absorptance
            .unwrap_or(DEFAULT_MATERIAL_SOLAR_ABSORPTANCE),
    })
}

const fn stage(
    kind: ExecutionStageKind,
    stage_name: &'static str,
    source_routine: &'static str,
) -> EnergyPlusCompatibilityStage {
    EnergyPlusCompatibilityStage {
        kind,
        stage_name,
        source_file: SOURCE_FILE,
        source_routine,
    }
}

/// EnergyPlus `HeatBalanceSurfaceManager::ManageSurfaceHeatBalance`.
#[must_use]
pub const fn manage_surface_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::ManageSurfaceHeatBalance,
        "manage-surface-heat-balance",
        "ManageSurfaceHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::ManageSurfaceHeatBalance` source-order wrapper.
pub(crate) fn manage_surface_heat_balance_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// EnergyPlus `HeatBalanceSurfaceManager::InitSurfaceHeatBalance`.
#[must_use]
pub const fn init_surface_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::InitSurfaceHeatBalance,
        "init-surface-heat-balance",
        "InitSurfaceHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::InitSurfaceHeatBalance` source-order wrapper.
pub(crate) fn init_surface_heat_balance_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// EnergyPlus `HeatBalanceSurfaceManager::CalcHeatBalanceOutsideSurf`.
#[must_use]
pub const fn calc_heat_balance_outside_surf_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::CalcHeatBalanceOutsideSurf,
        "calc-heat-balance-outside-surf",
        "CalcHeatBalanceOutsideSurf",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::CalcHeatBalanceOutsideSurf` source-order wrapper.
pub(crate) fn calc_heat_balance_outside_surf_source_order_path<T>(
    execute: impl FnOnce() -> T,
) -> T {
    execute()
}

/// EnergyPlus `HeatBalanceSurfaceManager::CalcHeatBalanceInsideSurf`.
#[must_use]
pub const fn calc_heat_balance_inside_surf_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::CalcHeatBalanceInsideSurf,
        "calc-heat-balance-inside-surf",
        "CalcHeatBalanceInsideSurf",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::CalcHeatBalanceInsideSurf` source-order wrapper.
pub(crate) fn calc_heat_balance_inside_surf_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// EnergyPlus `HeatBalanceSurfaceManager::UpdateFinalSurfaceHeatBalance`.
#[must_use]
pub const fn update_final_surface_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::UpdateFinalSurfaceHeatBalance,
        "update-final-surface-heat-balance",
        "UpdateFinalSurfaceHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::UpdateFinalSurfaceHeatBalance` source-order wrapper.
pub(crate) fn update_final_surface_heat_balance_source_order_path<T>(
    execute: impl FnOnce() -> T,
) -> T {
    execute()
}

/// EnergyPlus `HeatBalanceSurfaceManager::UpdateThermalHistories`.
#[must_use]
pub const fn update_thermal_histories_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::UpdateThermalHistories,
        "update-thermal-histories",
        "UpdateThermalHistories",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::UpdateThermalHistories` source-order wrapper.
pub(crate) fn update_thermal_histories_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// EnergyPlus `HeatBalanceSurfaceManager::ReportSurfaceHeatBalance`.
#[must_use]
pub const fn report_surface_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::ReportSurfaceHeatBalance,
        "report-surface-heat-balance",
        "ReportSurfaceHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::ReportSurfaceHeatBalance` source-order wrapper.
pub(crate) fn report_surface_heat_balance_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}
