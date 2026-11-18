//! Material.cc2227-2310: actual MaterialEcoRoof/base owner and late moisture recovery.
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness};

// Reached MaterialBase and MaterialEcoRoof state; no active EcoRoof physics cache.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceRoofVegetationMaterial {
    pub roughness: Option<MaterialSurfaceRoughness>,
    pub resistance: f64,
    pub thickness: f64,
    pub conductivity: f64,
    pub density: f64,
    pub specific_heat: f64,
    pub thermal_absorptance: f64,
    pub solar_absorptance: f64,
    pub visible_absorptance: f64,
    pub thermal_absorptance_input: f64,
    pub thermal_absorptance_back: f64,
    pub thermal_absorptance_front: f64,
    pub solar_absorptance_input: f64,
    pub visible_absorptance_input: f64,
    pub porosity: f64,
    pub height_of_plants: f64,
    pub leaf_area_index: f64,
    pub leaf_reflectivity: f64,
    pub leaf_emissivity: f64,
    pub minimum_stomatal_resistance: f64,
    pub minimum_moisture: f64,
    pub initial_moisture: f64,
    pub calc_method: i32,
}
impl Default for SourceRoofVegetationMaterial {
    fn default() -> Self {
        Self {
            roughness: None,
            resistance: 0.0,
            thickness: 0.0,
            conductivity: 0.0,
            density: 0.0,
            specific_heat: 0.0,
            thermal_absorptance: 0.0,
            solar_absorptance: 0.0,
            visible_absorptance: 0.0,
            thermal_absorptance_input: 0.0,
            thermal_absorptance_back: 0.0,
            thermal_absorptance_front: 0.0,
            solar_absorptance_input: 0.0,
            visible_absorptance_input: 0.0,
            porosity: 0.0,
            height_of_plants: 0.0,
            leaf_area_index: 0.0,
            leaf_reflectivity: 0.0,
            leaf_emissivity: 0.0,
            minimum_stomatal_resistance: 0.0,
            minimum_moisture: 0.0,
            initial_moisture: 0.0,
            calc_method: -1,
        }
    }
}
fn severe(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual RoofVegetation severe phase owners"))?;
    owners
        .backend
        .show_source_severe_error(message, flags)
        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))
}
fn warning(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual RoofVegetation warning phase owners"))?;
    owners
        .backend
        .show_source_warning_error(message, flags)
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
fn truncated(value: f64) -> Result<String, RuntimeError> {
    super::super::source_round_format::truncate(value, 3).map_err(|error| {
        RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Format(format!(
            "RoofVegetation Native .3T formatting: {error:?}"
        ))))
    })
}
impl SelectedMaterialData {
    pub(super) fn get_roof_vegetation_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "Material:RoofVegetation".into();
        self.num_eco_roofs = i32::try_from(
            owners
                .input
                .get_num_objects_found("Material:RoofVegetation"),
        )
        .map_err(|_| unavailable("RoofVegetation count exceeds Native integer"))?;
        if usize::try_from(self.num_eco_roofs).ok() != Some(raw.roof_vegetation.len()) {
            return Err(unavailable("RoofVegetation raw/IP count"));
        }
        for object in &raw.roof_vegetation {
            if raw.get_roof_vegetation_item(object, owners)? != (4, 15) {
                return Err(unavailable("RoofVegetation4A15N primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                severe(
                    owners,
                    &format!("GetMaterialData: Material:RoofVegetation = {name}, duplicate name."),
                )?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::RoofVegetation)?;
            let position = self.insert(&name, id, SourceMaterialGroup::EcoRoof)?;
            let material = self.roof_vegetation.entry(id).or_default();
            material.height_of_plants = owners.shortcut.numeric_args[0];
            material.leaf_area_index = owners.shortcut.numeric_args[1];
            material.leaf_reflectivity = owners.shortcut.numeric_args[2];
            material.leaf_emissivity = owners.shortcut.numeric_args[3];
            material.minimum_stomatal_resistance = owners.shortcut.numeric_args[4];
            // A2 soil label is intentionally not a physical Material member.
            material.roughness = match owners.shortcut.alpha_args[2].as_str() {
                "VERYROUGH" => Some(MaterialSurfaceRoughness::VeryRough),
                "ROUGH" => Some(MaterialSurfaceRoughness::Rough),
                "MEDIUMROUGH" => Some(MaterialSurfaceRoughness::MediumRough),
                "MEDIUMSMOOTH" => Some(MaterialSurfaceRoughness::MediumSmooth),
                "SMOOTH" => Some(MaterialSurfaceRoughness::Smooth),
                "VERYSMOOTH" => Some(MaterialSurfaceRoughness::VerySmooth),
                _ => None,
            };
            material.calc_method = if owners.shortcut.alpha_field_blanks[3] {
                1 // Native SchaapGenuchten, irrespective of the retained default alpha.
            } else {
                match owners.shortcut.alpha_args[3].as_str() {
                    "SIMPLE" => 0,
                    "ADVANCED" => 1,
                    _ => -1,
                }
            };
            material.thickness = owners.shortcut.numeric_args[5];
            material.conductivity = owners.shortcut.numeric_args[6];
            material.density = owners.shortcut.numeric_args[7];
            material.specific_heat = owners.shortcut.numeric_args[8];
            material.thermal_absorptance = owners.shortcut.numeric_args[9];
            material.solar_absorptance = owners.shortcut.numeric_args[10];
            material.visible_absorptance = owners.shortcut.numeric_args[11];
            material.porosity = owners.shortcut.numeric_args[12];
            material.minimum_moisture = owners.shortcut.numeric_args[13];
            material.initial_moisture = owners.shortcut.numeric_args[14];
            // ROnly remains the actual MaterialBase constructor false. This branch
            // is unconditional on the shared flag left by earlier families/rows.
            if material.conductivity > 0.0 {
                let resistance = material.thickness / material.conductivity;
                material.resistance = resistance;
                self.nominal_r[position] = resistance;
            } else {
                severe(
                    owners,
                    &format!("Material:RoofVegetation=\"{name}\" is not defined correctly."),
                )?;
                let message = format!("{} is <=0.", owners.shortcut.numeric_field_names[6]);
                continuation(owners, &message)?;
                *owners.errors_found = true;
            }
            if material.initial_moisture > material.porosity {
                warning(
                    owners,
                    &format!("Material:RoofVegetation=\"{name}\", Illegal value combination."),
                )?;
                let message = format!(
                    "{} is greater than {}. It must be less or equal.",
                    owners.shortcut.numeric_field_names[14],
                    owners.shortcut.numeric_field_names[12]
                );
                continuation(owners, &message)?;
                let message = format!(
                    "{} = {}.",
                    owners.shortcut.numeric_field_names[12],
                    truncated(material.porosity)?
                );
                continuation(owners, &message)?;
                let message = format!(
                    "{} = {}.",
                    owners.shortcut.numeric_field_names[14],
                    truncated(material.initial_moisture)?
                );
                continuation(owners, &message)?;
                let message = format!(
                    "{} is reset to the maximum (saturation) value = {}.",
                    owners.shortcut.numeric_field_names[14],
                    truncated(material.porosity)?
                );
                continuation(owners, &message)?;
                continuation(owners, "Simulation continues.")?;
                material.initial_moisture = material.porosity;
            }
        }
        Ok(())
    }
}
