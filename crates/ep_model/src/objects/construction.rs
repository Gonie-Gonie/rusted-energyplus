//! Typed construction records and source-projected metadata.

use crate::{ConstructionId, MaterialId, NormalizedName};

/// Consumer family for an ordered construction layer stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstructionKind {
    /// Opaque construction consumed by the existing surface heat-balance path.
    Opaque,
    /// Fenestration construction reserved for a dedicated window heat-balance path.
    Fenestration,
}

impl ConstructionKind {
    /// Stable diagnostic identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::Fenestration => "fenestration",
        }
    }
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
    pub outside_layer: MaterialId,
    /// Ordered material layers from outside to inside.
    pub layers: Vec<MaterialId>,
    /// Source-style thermochromic master metadata for the last group parent in the stack.
    ///
    /// The effective layer stack contains the parent's first glazing state. Generating
    /// thermochromic child constructions and selecting states at runtime remain deferred.
    pub thermochromic_master: Option<ConstructionThermochromicMaster>,
    /// F/C-factor source and derived state for generated ground constructions.
    pub ground_factor: Option<ConstructionGroundFactor>,
}

impl Construction {
    /// Returns whether this is an ordinary opaque construction currently admitted by
    /// BuildingSurface, reporting, and runtime consumers.
    #[must_use]
    pub const fn is_ordinary_opaque(&self) -> bool {
        matches!(self.kind, ConstructionKind::Opaque) && self.ground_factor.is_none()
    }
}
