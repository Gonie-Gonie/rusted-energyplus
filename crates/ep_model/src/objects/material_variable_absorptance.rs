use crate::{MaterialId, MaterialVariableAbsorptanceId, NormalizedName, ScheduleId, SurfaceId};

/// A curve or table identity whose payload remains outside the typed model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredVariableAbsorptanceFunction {
    /// EnergyPlus object type that owns the referenced name.
    pub object_type: String,
    /// Case-insensitive curve or table name.
    pub name: NormalizedName,
}

/// Schedule identity accepted by `MaterialProperty:VariableAbsorptance`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VariableAbsorptanceSchedule {
    /// User-declared schedule in the shared typed schedule namespace.
    User(ScheduleId),
    /// EnergyPlus built-in `Constant-0.0` schedule.
    ConstantZero,
    /// EnergyPlus built-in `Constant-1.0` schedule.
    ConstantOne,
}

/// Non-scheduled signal supplied to a variable-absorptance function.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VariableAbsorptanceFunctionSignal {
    /// Exterior surface temperature.
    SurfaceTemperature,
    /// Exterior received short-wave solar radiation.
    SurfaceReceivedSolarRadiation,
    /// Zone heating/cooling mode represented by a zero-or-one signal.
    SpaceHeatingCoolingMode,
}

/// Source-resolved control payload for one variable-absorptance overlay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VariableAbsorptanceControl {
    /// Thermal and solar absorptance are driven by schedules.
    Scheduled {
        /// Optional resolved thermal-absorptance schedule.
        thermal: Option<VariableAbsorptanceSchedule>,
        /// Optional resolved solar-absorptance schedule.
        solar: Option<VariableAbsorptanceSchedule>,
    },
    /// Thermal and solar absorptance are driven by curves or lookup tables.
    Function {
        /// Runtime signal passed to each resolved function.
        signal: VariableAbsorptanceFunctionSignal,
        /// Optional resolved thermal-absorptance function identity.
        thermal: Option<DeferredVariableAbsorptanceFunction>,
        /// Optional resolved solar-absorptance function identity.
        solar: Option<DeferredVariableAbsorptanceFunction>,
    },
}

/// Typed `MaterialProperty:VariableAbsorptance` overlay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterialVariableAbsorptance {
    /// Stable typed object ID.
    pub id: MaterialVariableAbsorptanceId,
    /// Normalized overlay-object name in its separate namespace.
    pub name: NormalizedName,
    /// Referenced regular or no-mass material.
    pub reference_material: MaterialId,
    /// Source-resolved control and dependency state.
    pub control: VariableAbsorptanceControl,
}

/// One exterior opaque surface selected for variable-absorptance evaluation.
///
/// The binding records only the source `GetVariableAbsorptanceSurfaceList` selection. Runtime
/// evaluation and mutation of surface absorptances remain outside this typed subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VariableAbsorptanceSurfaceBinding {
    /// Typed detailed-surface ID in deterministic surface order.
    pub surface: SurfaceId,
    /// Existing variable-absorptance overlay applied through the construction outside layer.
    pub variable_absorptance: MaterialVariableAbsorptanceId,
}
