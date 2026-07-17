use super::{WindowGasMixture, WindowGasProperties, WindowGasType};
use crate::MaterialId;

/// Source-copied gas state for a `WindowMaterial:Gap` object.
///
/// EnergyPlus resolves the referenced ordinary gas or gas-mixture material
/// before storing this complex-fenestration gap. Both the source material ID
/// and the copied gas payload are retained here.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowComplexGapGasComposition {
    /// One gas copied from a `WindowMaterial:Gas` definition.
    Single {
        /// Referenced source material.
        source_material_id: MaterialId,
        /// Copied gas species.
        gas_type: WindowGasType,
        /// Copied thermophysical properties.
        properties: WindowGasProperties,
    },
    /// One-to-four gases copied from a `WindowMaterial:GasMixture` definition.
    Mixture {
        /// Referenced source material.
        source_material_id: MaterialId,
        /// Copied active gas components in source order.
        gases: WindowGasMixture,
    },
}

impl WindowComplexGapGasComposition {
    /// Returns the referenced ordinary gas or gas-mixture material.
    #[must_use]
    pub const fn source_material_id(&self) -> MaterialId {
        match self {
            Self::Single {
                source_material_id, ..
            }
            | Self::Mixture {
                source_material_id, ..
            } => *source_material_id,
        }
    }

    /// Returns the number of active gases in the copied state.
    #[must_use]
    pub const fn active_gas_count(&self) -> usize {
        match self {
            Self::Single { .. } => 1,
            Self::Mixture { gases, .. } => gases.len(),
        }
    }
}

/// Source-copied support-pillar geometry for a `WindowMaterial:Gap` object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowComplexGapSupportPillar {
    /// Center-to-center pillar spacing in meters.
    pub spacing_m: f64,
    /// Pillar radius in meters.
    pub radius_m: f64,
}

/// Fully resolved source state for a complex-fenestration `WindowMaterial:Gap`.
///
/// This payload preserves the copied gas definition and geometric inputs. It
/// does not claim an ordinary-gap nominal resistance or runtime consumer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowComplexGapMaterial {
    /// Nominal gap thickness in meters.
    pub thickness_m: f64,
    /// Initial gas pressure in pascals.
    pub pressure_pa: f64,
    /// Gas state copied from the referenced ordinary gas material.
    pub gas: WindowComplexGapGasComposition,
    /// Deflected gap thickness in meters.
    pub deflected_thickness_m: f64,
    /// Optional support-pillar geometry.
    pub support_pillar: Option<WindowComplexGapSupportPillar>,
}

impl WindowComplexGapMaterial {
    /// Returns the ordinary gas material from which this gap copied its state.
    #[must_use]
    pub const fn source_material_id(&self) -> MaterialId {
        self.gas.source_material_id()
    }

    /// Returns the number of active gases in the copied state.
    #[must_use]
    pub const fn active_gas_count(&self) -> usize {
        self.gas.active_gas_count()
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        Material, MaterialDefinition, MaterialFamily, MaterialKind, WindowGasMixtureComponent,
        WindowStandardGasType,
    };
    use super::*;
    use crate::NormalizedName;

    #[test]
    fn copied_single_gas_retains_source_identity_and_properties() {
        let properties = WindowGasType::Argon.standard_properties();
        assert!(properties.is_some());

        if let Some(properties) = properties {
            let gas = WindowComplexGapGasComposition::Single {
                source_material_id: MaterialId(3),
                gas_type: WindowGasType::Argon,
                properties,
            };

            assert_eq!(gas.source_material_id(), MaterialId(3));
            assert_eq!(gas.active_gas_count(), 1);
            assert_eq!(
                gas,
                WindowComplexGapGasComposition::Single {
                    source_material_id: MaterialId(3),
                    gas_type: WindowGasType::Argon,
                    properties,
                }
            );
        }
    }

    #[test]
    fn material_identity_uses_the_complex_fenestration_family() {
        let payload = WindowComplexGapMaterial {
            thickness_m: 0.012,
            pressure_pa: 101_325.0,
            gas: WindowComplexGapGasComposition::Mixture {
                source_material_id: MaterialId(5),
                gases: WindowGasMixture::Two([
                    WindowGasMixtureComponent {
                        gas_type: WindowStandardGasType::Air,
                        fraction: 0.1,
                    },
                    WindowGasMixtureComponent {
                        gas_type: WindowStandardGasType::Argon,
                        fraction: 0.9,
                    },
                ]),
            },
            deflected_thickness_m: 0.011,
            support_pillar: Some(WindowComplexGapSupportPillar {
                spacing_m: 0.04,
                radius_m: 0.00025,
            }),
        };
        let material = Material {
            id: MaterialId(6),
            name: NormalizedName::new("Complex Gap"),
            definition: MaterialDefinition::WindowComplexGap(payload),
        };

        assert_eq!(payload.source_material_id(), MaterialId(5));
        assert_eq!(payload.active_gas_count(), 2);
        assert_eq!(material.kind(), MaterialKind::WindowComplexGap);
        assert_eq!(material.family(), MaterialFamily::ComplexFenestration);
        assert_eq!(material.family().id(), "complex-fenestration");
        assert_eq!(material.as_window_complex_gap(), Some(&payload));
        assert_eq!(material.as_opaque(), None);
    }
}
