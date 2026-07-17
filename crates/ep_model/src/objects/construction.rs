//! Typed construction records and source-projected metadata.

use std::sync::Arc;

use crate::{ConstructionId, MaterialId, NormalizedName, ScheduleId};

/// Consumer family for an ordered construction layer stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstructionKind {
    /// Opaque construction consumed by the existing surface heat-balance path.
    Opaque,
    /// Fenestration construction reserved for a dedicated window heat-balance path.
    Fenestration,
    /// BSDF complex-fenestration construction reserved for dedicated window paths.
    ComplexFenestration,
    /// Zero-layer air boundary reserved for enclosure and interzone mixing paths.
    AirBoundary,
}

impl ConstructionKind {
    /// Stable diagnostic identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::Fenestration => "fenestration",
            Self::ComplexFenestration => "complex_fenestration",
            Self::AirBoundary => "air_boundary",
        }
    }
}

/// Basis family selected by `Construction:ComplexFenestrationState`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComplexFenestrationBasisType {
    /// Basis emitted by LBNL WINDOW.
    LbnlWindow,
    /// User-defined basis input.
    UserDefined,
}

/// Symmetry applied to complex-fenestration optical matrices.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComplexFenestrationBasisSymmetry {
    /// Full nonsymmetric matrices.
    None,
    /// Axisymmetric matrix input.
    Axisymmetric,
}

/// Standard selected for complex-window thermal calculations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowThermalCalculationStandard {
    /// ISO 15099.
    Iso15099,
    /// EN 673 / ISO 10292 declared conditions.
    En673Declared,
    /// EN 673 / ISO 10292 design conditions.
    En673Design,
}

/// TARCOG thermal-model selector retained from `WindowThermalModel:Params`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowThermalCalculationModel {
    /// ISO 15099 model.
    Iso15099,
    /// Scaled-cavity-width model.
    ScaledCavityWidth,
    /// Convective scalar model without shading-device thickness.
    ConvectiveScalarNoSdThickness,
    /// Convective scalar model including shading-device thickness.
    ConvectiveScalarWithSdThickness,
}

/// Deflection state retained from `WindowThermalModel:Params`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowThermalDeflectionModel {
    /// No pane deflection.
    NoDeflection,
    /// Gap widths are supplied by measured-deflection state.
    MeasuredDeflection,
    /// Temperature-and-pressure deflection inputs.
    TemperatureAndPressureInput {
        /// Pressure threshold for the vacuum model in pascals.
        vacuum_pressure_limit_pa: f64,
        /// Window fabrication temperature in degrees Celsius.
        initial_temperature_c: f64,
        /// Window fabrication pressure in pascals.
        initial_pressure_pa: f64,
    },
}

/// Source-effective thermal-model descriptor used by one complex construction.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowThermalModelParameters {
    /// Normalized helper-object name.
    pub name: NormalizedName,
    /// Calculation standard.
    pub standard: WindowThermalCalculationStandard,
    /// TARCOG thermal model.
    pub thermal_model: WindowThermalCalculationModel,
    /// Shading-device scalar in the inclusive range `[0, 1]`.
    pub shading_device_scalar: f64,
    /// Deflection selection and active dependent inputs.
    pub deflection_model: WindowThermalDeflectionModel,
}

/// Immutable row-major `Matrix:TwoDimension` snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct ComplexFenestrationMatrix {
    /// Original source spelling of the matrix name before input-level normalization.
    pub source_name: String,
    /// Source row count.
    pub rows: u32,
    /// Source column count.
    pub columns: u32,
    /// Effective row-major prefix of `rows * columns` finite values.
    pub values: Arc<[f64]>,
}

impl ComplexFenestrationMatrix {
    /// Returns a zero-based row-major element.
    #[must_use]
    pub fn get(&self, row: usize, column: usize) -> Option<f64> {
        let columns = usize::try_from(self.columns).ok()?;
        let rows = usize::try_from(self.rows).ok()?;
        if row >= rows || column >= columns {
            return None;
        }
        self.values
            .get(row.checked_mul(columns)?.checked_add(column)?)
            .copied()
    }
}

/// Directional absorptance state for one solid complex-window layer.
#[derive(Clone, Debug, PartialEq)]
pub struct ComplexFenestrationOpticalLayer {
    /// Glazing or complex-shade material.
    pub material: MaterialId,
    /// One-row front directional absorptance matrix.
    pub front_absorptance: ComplexFenestrationMatrix,
    /// One-row back directional absorptance matrix.
    pub back_absorptance: ComplexFenestrationMatrix,
}

/// Source-projected metadata for `Construction:ComplexFenestrationState`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionComplexFenestrationState {
    /// Basis family.
    pub basis_type: ComplexFenestrationBasisType,
    /// Basis symmetry.
    pub basis_symmetry: ComplexFenestrationBasisSymmetry,
    /// Resolved thermal-model helper state.
    pub thermal_model: WindowThermalModelParameters,
    /// Basis matrix.
    pub basis_matrix: ComplexFenestrationMatrix,
    /// Source-derived number of basis directions.
    pub basis_length: u32,
    /// Solar front transmittance matrix.
    pub solar_front_transmittance: ComplexFenestrationMatrix,
    /// Solar back reflectance matrix.
    pub solar_back_reflectance: ComplexFenestrationMatrix,
    /// Visible front transmittance matrix.
    pub visible_front_transmittance: ComplexFenestrationMatrix,
    /// Visible back reflectance matrix (the schema field is named back transmittance).
    pub visible_back_reflectance: ComplexFenestrationMatrix,
    /// Solid-layer absorptance descriptors in outside-to-inside order.
    pub optical_layers: Vec<ComplexFenestrationOpticalLayer>,
}

/// Schedule source used by a simple-mixing air boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AirBoundaryMixingSchedule {
    /// Blank or omitted input selects EnergyPlus's built-in always-on schedule.
    AlwaysOn,
    /// Explicit user schedule resolved through the shared schedule namespace.
    User(ScheduleId),
}

/// Air-exchange behavior retained for a zero-layer air boundary construction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AirBoundaryAirExchange {
    /// No interzone air exchange is requested by this construction.
    None,
    /// Simple scheduled mixing at the source air-changes-per-hour input.
    SimpleMixing {
        /// Air changes per hour.
        air_changes_per_hour: f64,
        /// Built-in or explicitly named mixing schedule.
        schedule: AirBoundaryMixingSchedule,
    },
}

/// Source-projected metadata for `Construction:AirBoundary`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstructionAirBoundary {
    /// Requested air-exchange method and its dependent inputs.
    pub air_exchange: AirBoundaryAirExchange,
}

/// Source inputs and derived resistance state for a generated F/C-factor construction.
///
/// EnergyPlus projects both object families into opaque two-layer constructions. The
/// dedicated metadata keeps those generated stacks distinguishable from ordinary opaque
/// constructions while their surface binding and runtime behavior remain deferred.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstructionGroundFactor {
    /// `Construction:FfactorGroundFloor` source and derived values.
    FfactorGroundFloor {
        /// F-factor in W/m-K.
        f_factor_w_per_m_k: f64,
        /// Source floor area in m2.
        area_m2: f64,
        /// Source exposed perimeter in m.
        perimeter_exposed_m: f64,
        /// Effective construction resistance in m2-K/W.
        effective_thermal_resistance_m2_k_per_w: f64,
        /// Generated fictitious-insulation resistance in m2-K/W.
        insulation_thermal_resistance_m2_k_per_w: f64,
    },
    /// `Construction:CfactorUndergroundWall` source and derived values.
    CfactorUndergroundWall {
        /// C-factor in W/m2-K.
        c_factor_w_per_m2_k: f64,
        /// Source wall height in m.
        height_m: f64,
        /// Height-derived equivalent soil resistance in m2-K/W.
        equivalent_soil_thermal_resistance_m2_k_per_w: f64,
        /// Effective construction resistance in m2-K/W.
        effective_thermal_resistance_m2_k_per_w: f64,
        /// Generated fictitious-insulation resistance in m2-K/W.
        insulation_thermal_resistance_m2_k_per_w: f64,
    },
}

/// Thermochromic parent metadata retained on an effective construction stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstructionThermochromicMaster {
    /// Thermochromic glazing-group material replaced by its first typed state.
    pub parent_material: MaterialId,
    /// Zero-based construction layer index (EnergyPlus `TCLayerNum` is one-based).
    pub layer_index: u32,
    /// Zero-based source glass-layer ordinal (EnergyPlus `TCGlassNum` is one-based).
    pub glazing_layer_index: u32,
}

/// Construction resolved to an ordered, effective material layer stack.
#[derive(Clone, Debug, PartialEq)]
pub struct Construction {
    /// Typed ID.
    pub id: ConstructionId,
    /// Construction name.
    pub name: NormalizedName,
    /// Consumer family for this construction.
    pub kind: ConstructionKind,
    /// Effective outside layer material (including first-state TC substitution).
    ///
    /// Zero-layer air-boundary constructions have no outside material.
    pub outside_layer: Option<MaterialId>,
    /// Ordered material layers from outside to inside.
    pub layers: Vec<MaterialId>,
    /// Source-style thermochromic master metadata for the last group parent in the stack.
    ///
    /// The effective layer stack contains the parent's first glazing state. Generating
    /// thermochromic child constructions and selecting states at runtime remain deferred.
    pub thermochromic_master: Option<ConstructionThermochromicMaster>,
    /// F/C-factor source and derived state for generated ground constructions.
    pub ground_factor: Option<ConstructionGroundFactor>,
    /// Zero-layer air-boundary source state when this is an air boundary.
    pub air_boundary: Option<ConstructionAirBoundary>,
    /// BSDF source state when this is a complex-fenestration construction.
    pub complex_fenestration: Option<ConstructionComplexFenestrationState>,
}

impl Construction {
    /// Returns whether this is an ordinary opaque construction currently admitted by
    /// BuildingSurface, reporting, and runtime consumers.
    #[must_use]
    pub const fn is_ordinary_opaque(&self) -> bool {
        matches!(self.kind, ConstructionKind::Opaque) && self.ground_factor.is_none()
    }

    /// Returns whether this record is a zero-layer air boundary construction.
    #[must_use]
    pub const fn is_air_boundary(&self) -> bool {
        matches!(self.kind, ConstructionKind::AirBoundary)
    }

    /// Returns whether this record is a BSDF complex-fenestration construction.
    #[must_use]
    pub const fn is_complex_fenestration(&self) -> bool {
        matches!(self.kind, ConstructionKind::ComplexFenestration)
    }

    /// Returns the effective layer stack, retaining the legacy outside-only fallback.
    #[must_use]
    pub fn effective_layers(&self) -> &[MaterialId] {
        if self.layers.is_empty() {
            self.outside_layer.as_slice()
        } else {
            self.layers.as_slice()
        }
    }
}
