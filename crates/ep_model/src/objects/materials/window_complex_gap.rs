use super::{WindowGasMixture, WindowGasProperties, WindowGasType};
use crate::MaterialId;

/// Advisory gas identity for a `WindowMaterial:Gap` object.
///
/// Legacy Single/Mixture retain their copied payload. Explicit Source may keep
/// Empty or an actual already-reserved Reference, including self/earlier Gap;
/// the genuine runtime Material owner performs the physical copy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowComplexGapGasComposition {
    /// No copied gas yet: a blank or deferred unresolved Source reference.
    Empty,
    /// Actual already-reserved Source material identity; runtime owns gas copy.
    Reference {
        /// Source prefix may include the newly allocated Gap itself.
        source_material_id: MaterialId,
    },
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
    /// Returns an actual source-prefix identity, or None for blank/missing gas.
    #[must_use]
    pub const fn source_material_id(&self) -> Option<MaterialId> {
        match self {
            Self::Empty => None,
            Self::Reference { source_material_id }
            | Self::Single {
                source_material_id, ..
            }
            | Self::Mixture {
                source_material_id, ..
            } => Some(*source_material_id),
        }
    }

    /// Returns advisory copied count, or None for a Reference whose count belongs to runtime.
    #[must_use]
    pub const fn active_gas_count(&self) -> Option<usize> {
        match self {
            Self::Empty => Some(0),
            Self::Reference { .. } => None,
            Self::Single { .. } => Some(1),
            Self::Mixture { gases, .. } => Some(gases.len()),
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

/// Advisory typed state for a complex-fenestration `WindowMaterial:Gap`.
///
/// Legacy copied gas and geometry stay available; Source Empty/Reference waits
/// for the actual runtime owner and must not seed physical copied properties. It
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
    /// Returns the actual optional Source-prefix identity, including self/earlier Gap.
    #[must_use]
    pub const fn source_material_id(&self) -> Option<MaterialId> {
        self.gas.source_material_id()
    }

    /// Returns advisory copied count, or None for a Reference whose count belongs to runtime.
    #[must_use]
    pub const fn active_gas_count(&self) -> Option<usize> {
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

            assert_eq!(gas.source_material_id(), Some(MaterialId(3)));
            assert_eq!(gas.active_gas_count(), Some(1));
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

        assert_eq!(payload.source_material_id(), Some(MaterialId(5)));
        assert_eq!(payload.active_gas_count(), Some(2));
        assert_eq!(material.kind(), MaterialKind::WindowComplexGap);
        assert_eq!(material.family(), MaterialFamily::ComplexFenestration);
        assert_eq!(material.family().id(), "complex-fenestration");
        assert_eq!(material.as_window_complex_gap(), Some(&payload));
        assert_eq!(material.as_opaque(), None);
    }
}
