//! Material.cc2767-2798 writes to the real EIO using existing Source R formatting.
use super::{SelectedMaterialData, SourceMaterialGroup, unavailable};
use crate::error::{RuntimeError, WarmupOutputError};
use ep_model::{MaterialDefinition, MaterialSurfaceRoughness, OpaqueSurfaceProperties, TypedModel};
use std::io::Write;

fn rounded(value: f64, precision: usize) -> Result<String, RuntimeError> {
    super::super::source_round_format::round(value, precision).map_err(|error| {
        RuntimeError::WarmupOutput(Box::new(WarmupOutputError::Format(format!(
            "Material Details .{precision}R: {error:?}"
        ))))
    })
}
fn io(error: std::io::Error) -> RuntimeError {
    RuntimeError::WarmupOutput(Box::new(WarmupOutputError::Io(error)))
}
fn roughness(value: MaterialSurfaceRoughness) -> &'static str {
    match value {
        MaterialSurfaceRoughness::VeryRough => "VeryRough",
        MaterialSurfaceRoughness::Rough => "Rough",
        MaterialSurfaceRoughness::MediumRough => "MediumRough",
        MaterialSurfaceRoughness::MediumSmooth => "MediumSmooth",
        MaterialSurfaceRoughness::Smooth => "Smooth",
        MaterialSurfaceRoughness::VerySmooth => "VerySmooth",
    }
}

impl SelectedMaterialData {
    pub(super) fn write_material_reports(
        &self,
        model: &TypedModel,
        eio: &mut dyn Write,
    ) -> Result<(), RuntimeError> {
        eio.write_all(b"! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible\n").map_err(io)?;
        eio.write_all(b"! <Material:Air>,Material Name,ThermalResistance {m2-K/w}\n")
            .map_err(io)?;
        for id in &self.materials {
            let material = model
                .materials
                .iter()
                .find(|row| row.id == *id)
                .ok_or_else(|| unavailable("reported MaterialId"))?;
            let mut names = model
                .ctf_report_names
                .materials
                .iter()
                .filter(|(key, _)| key == id);
            let name = &names
                .next()
                .ok_or_else(|| unavailable("reported material loader name"))?
                .1;
            if names.next().is_some() {
                return Err(unavailable("reported material name duplicate"));
            }
            if self.material_groups.get(id) == Some(&SourceMaterialGroup::AirGap) {
                let MaterialDefinition::AirGap(value) = material.definition else {
                    return Err(unavailable("reported AirGap group/physical owner"));
                };
                writeln!(
                    eio,
                    " Material:Air,{},{}",
                    name,
                    rounded(value.thermal_resistance_m2_k_per_w, 4)?
                )
                .map_err(io)?;
                continue;
            }
            let (resistance, rough, geometry, surface) = match material.definition {
                MaterialDefinition::Regular(value) => (
                    value.thermal_resistance_m2_k_per_w,
                    value.roughness,
                    [
                        value.thickness_m,
                        value.conductivity_w_per_m_k,
                        value.density_kg_per_m3,
                        value.specific_heat_j_per_kg_k,
                    ],
                    value.surface,
                ),
                MaterialDefinition::NoMass(value) => (
                    value.thermal_resistance_m2_k_per_w,
                    value.roughness,
                    [0.0; 4],
                    value.surface,
                ),
                MaterialDefinition::WindowGas(_) | MaterialDefinition::WindowGasMixture(_) => {
                    let expected = if matches!(
                        &material.definition,
                        MaterialDefinition::WindowGasMixture(_)
                    ) {
                        SourceMaterialGroup::GasMixture
                    } else {
                        SourceMaterialGroup::Gas
                    };
                    if self.material_groups.get(id) != Some(&expected) {
                        return Err(unavailable("reported Gas actual group"));
                    }
                    let value = self.window_gas.get(id).ok_or_else(|| {
                        unavailable("reported actual Gas runtime base properties")
                    })?;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("Gas getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowGlazingSpectralAndAngle(_)
                | MaterialDefinition::WindowGlazingSpectral(_)
                | MaterialDefinition::WindowGlazingSpectralAverage(_)
                | MaterialDefinition::WindowGlazingRefractionExtinction(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::Glass) {
                        return Err(unavailable("reported ordinary Glass actual group"));
                    }
                    let value = self
                        .simple_glazing
                        .get(id)
                        .ok_or_else(|| unavailable("reported actual Glass runtime properties"))?;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("Glass getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowShade(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::Shade) {
                        return Err(unavailable("reported Shade actual group"));
                    }
                    let value = self
                        .window_shade
                        .get(id)
                        .ok_or_else(|| unavailable("reported actual Shade runtime properties"))?;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("Shade getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowShadeEquivalentLayer(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::ShadeEql) {
                        return Err(unavailable("reported ShadeEQL actual group"));
                    }
                    let value = self.window_shade_equivalent_layer.get(id).ok_or_else(|| {
                        unavailable("reported actual ShadeEQL runtime properties")
                    })?;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("ShadeEQL getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowDrapeEquivalentLayer(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::DrapeEql) {
                        return Err(unavailable("reported DrapeEQL actual group"));
                    }
                    let value = self.window_drape_equivalent_layer.get(id).ok_or_else(|| {
                        unavailable("reported actual DrapeEQL runtime properties")
                    })?;
                    let value = &value.base;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("DrapeEQL getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowScreenEquivalentLayer(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::ScreenEql) {
                        return Err(unavailable("reported ScreenEQL actual group"));
                    }
                    let value = self.window_screen_equivalent_layer.get(id).ok_or_else(|| {
                        unavailable("reported actual ScreenEQL runtime properties")
                    })?;
                    let value = &value.base;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("ScreenEQL getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowBlindEquivalentLayer(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::BlindEql) {
                        return Err(unavailable("reported BlindEQL actual group"));
                    }
                    let value = self.window_blind_equivalent_layer.get(id).ok_or_else(|| {
                        unavailable("reported actual BlindEQL runtime properties")
                    })?;
                    let value = &value.base;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("BlindEQL getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowGlazingEquivalentLayer(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::GlassEql) {
                        return Err(unavailable("reported GlazingEQL actual group"));
                    }
                    let value = self
                        .window_glazing_equivalent_layer
                        .get(id)
                        .ok_or_else(|| {
                            unavailable("reported actual GlazingEQL runtime properties")
                        })?;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("GlazingEQL getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowGapEquivalentLayer(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::WindowGapEql) {
                        return Err(unavailable("reported GapEQL actual group"));
                    }
                    let value = self
                        .window_gas
                        .get(id)
                        .ok_or_else(|| unavailable("reported actual GapEQL runtime properties"))?;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("GapEQL getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowScreen(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::Screen) {
                        return Err(unavailable("reported Screen actual group"));
                    }
                    let value = self
                        .window_screen
                        .get(id)
                        .ok_or_else(|| unavailable("reported actual Screen runtime properties"))?;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("Screen getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowBlind(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::Blind) {
                        return Err(unavailable("reported Blind actual group"));
                    }
                    let value = self
                        .window_blind
                        .get(id)
                        .ok_or_else(|| unavailable("reported actual Blind runtime properties"))?;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("Blind getter roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                MaterialDefinition::WindowSimpleGlazing(_) => {
                    if self.material_groups.get(id) != Some(&SourceMaterialGroup::GlassSimple) {
                        return Err(unavailable("reported SimpleGlazing actual group"));
                    }
                    let value = self
                        .simple_glazing
                        .get(id)
                        .ok_or_else(|| unavailable("reported SimpleGlazing runtime properties"))?;
                    (
                        value.resistance,
                        value
                            .roughness
                            .ok_or_else(|| unavailable("SimpleGlazing Setup roughness"))?,
                        [
                            value.thickness,
                            value.conductivity,
                            value.density,
                            value.specific_heat,
                        ],
                        OpaqueSurfaceProperties {
                            thermal_absorptance: value.thermal_absorptance,
                            solar_absorptance: value.solar_absorptance,
                            visible_absorptance: value.visible_absorptance,
                        },
                    )
                }
                _ => return Err(unavailable("reported selected material family")),
            };
            let values = [
                rounded(resistance, 4)?,
                rounded(geometry[0], 4)?,
                rounded(geometry[1], 3)?,
                rounded(geometry[2], 3)?,
                rounded(geometry[3], 3)?,
                rounded(surface.thermal_absorptance, 4)?,
                rounded(surface.solar_absorptance, 4)?,
                rounded(surface.visible_absorptance, 4)?,
            ];
            writeln!(
                eio,
                " Material Details,{},{},{},{},{},{},{},{},{},{}",
                name,
                values[0],
                roughness(rough),
                values[1],
                values[2],
                values[3],
                values[4],
                values[5],
                values[6],
                values[7]
            )
            .map_err(io)?;
        }
        Ok(())
    }
}
