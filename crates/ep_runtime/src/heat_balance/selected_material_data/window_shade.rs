//! Material.cc1233-1310: genuine ordinary MaterialShade owner and diagnostic order.
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness};

// Actual MaterialBase/Fen/ShadingDevice/Shade ctor state, not typed derived data.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceShadeMaterial {
    pub roughness: Option<MaterialSurfaceRoughness>,
    pub transmittance: f64,
    pub reflectance: f64,
    pub visible_transmittance: f64,
    pub visible_reflectance: f64,
    pub thermal_absorptance: f64,
    pub thermal_absorptance_input: f64,
    pub infrared_transmittance: f64,
    pub thickness: f64,
    pub conductivity: f64,
    pub solar_absorptance: f64,
    pub solar_absorptance_input: f64,
    pub to_glass_distance: f64,
    pub top_opening: f64,
    pub bottom_opening: f64,
    pub left_opening: f64,
    pub right_opening: f64,
    pub airflow_permeability: f64,
    pub resistance: f64,
    pub density: f64,
    pub specific_heat: f64,
    pub visible_absorptance: f64,
}

fn severe(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual Shade diagnostic phase owners"))?;
    owners
        .backend
        .show_source_severe_error(message, flags)
        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))
}

fn continuation(
    owners: &mut MaterialDataOwners<'_, '_>,
    message: &str,
) -> Result<(), RuntimeError> {
    owners
        .backend
        .show_continue_error(message)
        .map_err(|error| {
            RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Backend(error)))
        })
}

impl SelectedMaterialData {
    pub(super) fn get_window_shade_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Shade".into();
        self.num_shades = i32::try_from(owners.input.get_num_objects_found("WindowMaterial:Shade"))
            .map_err(|_| unavailable("Shade count exceeds Native integer"))?;
        if usize::try_from(self.num_shades).ok() != Some(raw.window_shade.len()) {
            return Err(unavailable("Shade raw/IP count"));
        }
        for object in &raw.window_shade {
            let (alphas, numbers) = raw.get_window_shade_item(object, owners)?;
            let invalid_counts = match raw.source_original_idf_fixed_counts(
                "WindowMaterial:Shade",
                object,
                owners,
            )? {
                Some(expected) => (alphas, numbers) != expected,
                None => (alphas, numbers) != (1, 14),
            };
            if invalid_counts {
                return Err(unavailable("Shade1A14N primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                // Native ShowSevereDuplicateName precedes flag mutation/allocation.
                severe(
                    owners,
                    &format!("GetMaterialData: WindowMaterial:Shade = {name}, duplicate name."),
                )?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::WindowShade)?;
            let position = self.insert(&name, id, SourceMaterialGroup::Shade)?;
            let material = self.window_shade.entry(id).or_default();
            material.roughness = Some(MaterialSurfaceRoughness::MediumRough);
            material.transmittance = owners.shortcut.numeric_args[0];
            material.reflectance = owners.shortcut.numeric_args[1];
            material.visible_transmittance = owners.shortcut.numeric_args[2];
            material.visible_reflectance = owners.shortcut.numeric_args[3];
            material.thermal_absorptance = owners.shortcut.numeric_args[4];
            material.thermal_absorptance_input = owners.shortcut.numeric_args[4];
            material.infrared_transmittance = owners.shortcut.numeric_args[5];
            material.thickness = owners.shortcut.numeric_args[6];
            material.conductivity = owners.shortcut.numeric_args[7];
            material.solar_absorptance =
                (1.0 - material.transmittance - material.reflectance).max(0.0);
            material.solar_absorptance_input = material.solar_absorptance;
            material.to_glass_distance = owners.shortcut.numeric_args[8];
            material.top_opening = owners.shortcut.numeric_args[9];
            material.bottom_opening = owners.shortcut.numeric_args[10];
            material.left_opening = owners.shortcut.numeric_args[11];
            material.right_opening = owners.shortcut.numeric_args[12];
            material.airflow_permeability = owners.shortcut.numeric_args[13];
            self.resistance_only.insert(id, true);
            self.nominal_r[position] = if material.conductivity > 0.0 {
                material.thickness / material.conductivity
            } else {
                1.0
            };
            // All fields/ROnly/NominalR are written before the three checks.
            for (a, b) in [(0, 1), (2, 3), (4, 5)] {
                if owners.shortcut.numeric_args[a] + owners.shortcut.numeric_args[b] >= 1.0 {
                    *owners.errors_found = true;
                    severe(
                        owners,
                        &format!("WindowMaterial:Shade=\"{name}\", Illegal value combination."),
                    )?;
                    continuation(
                        owners,
                        &format!(
                            "{} + {} not < 1.0",
                            owners.shortcut.numeric_field_names[a],
                            owners.shortcut.numeric_field_names[b]
                        ),
                    )?;
                }
            }
        }
        Ok(())
    }
}
