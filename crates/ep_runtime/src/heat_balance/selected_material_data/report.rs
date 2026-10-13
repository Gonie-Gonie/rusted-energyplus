//! Material.cc2767-2798 writes to the real EIO using existing Source R formatting.
use super::{SelectedMaterialData, unavailable};
use crate::error::{RuntimeError, WarmupOutputError};
use ep_model::{MaterialDefinition, MaterialSurfaceRoughness, TypedModel};
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
