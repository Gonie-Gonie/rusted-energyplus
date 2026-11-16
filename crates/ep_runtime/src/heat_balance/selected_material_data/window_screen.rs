//! Material.cc1493-1670 actual ordinary MaterialScreen scalar owner.
//! Unused btars/active optical integrations are not represented or asserted.
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceScreenMaterial {
    pub roughness: Option<MaterialSurfaceRoughness>,
    pub beam_reflectance_model: i32,
    pub transmittance: f64,
    pub visible_transmittance: f64,
    pub thermal_transmittance: f64,
    pub shade_reflectance: f64,
    pub shade_visible_reflectance: f64,
    pub thermal_absorptance: f64,
    pub thermal_absorptance_input: f64,
    pub solar_absorptance: f64,
    pub solar_absorptance_input: f64,
    pub visible_absorptance: f64,
    pub visible_absorptance_input: f64,
    pub conductivity: f64,
    pub thickness: f64,
    pub to_glass_distance: f64,
    pub top_opening: f64,
    pub bottom_opening: f64,
    pub left_opening: f64,
    pub right_opening: f64,
    pub airflow_permeability: f64,
    pub resistance: f64,
    pub density: f64,
    pub specific_heat: f64,
    // These scalar members remain ctor state until a genuine used Screen.
    pub diameter_to_spacing_ratio: f64,
    pub diffuse_transmittance: f64,
    pub diffuse_visible_transmittance: f64,
    pub diffuse_reflectance: f64,
    pub diffuse_visible_reflectance: f64,
    pub diffuse_absorptance: f64,
    pub cylinder_reflectance: f64,
    pub cylinder_visible_reflectance: f64,
    pub map_degree_resolution: i32,
    pub d_phi: f64,
    pub d_theta: f64,
    pub is_used: bool,
}

impl Default for SourceScreenMaterial {
    fn default() -> Self {
        Self {
            roughness: None,
            beam_reflectance_model: -1,
            transmittance: 0.0,
            visible_transmittance: 0.0,
            thermal_transmittance: 0.0,
            shade_reflectance: 0.0,
            shade_visible_reflectance: 0.0,
            thermal_absorptance: 0.0,
            thermal_absorptance_input: 0.0,
            solar_absorptance: 0.0,
            solar_absorptance_input: 0.0,
            visible_absorptance: 0.0,
            visible_absorptance_input: 0.0,
            conductivity: 0.0,
            thickness: 0.0,
            to_glass_distance: 0.0,
            top_opening: 0.0,
            bottom_opening: 0.0,
            left_opening: 0.0,
            right_opening: 0.0,
            airflow_permeability: 0.0,
            resistance: 0.0,
            density: 0.0,
            specific_heat: 0.0,
            diameter_to_spacing_ratio: 0.0,
            diffuse_transmittance: 0.0,
            diffuse_visible_transmittance: 0.0,
            diffuse_reflectance: 0.0,
            diffuse_visible_reflectance: 0.0,
            diffuse_absorptance: 0.0,
            cylinder_reflectance: 0.0,
            cylinder_visible_reflectance: 0.0,
            map_degree_resolution: 0,
            d_phi: 5.0 * (std::f64::consts::PI / 180.0),
            d_theta: 5.0 * (std::f64::consts::PI / 180.0),
            is_used: false,
        }
    }
}

fn severe(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual Screen diagnostic phase owners"))?;
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
fn warning(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual Screen warning phase owners"))?;
    owners
        .backend
        .show_source_warning_error(message, flags)
        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))
}

impl SelectedMaterialData {
    pub(super) fn get_window_screen_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Screen".into();
        self.num_screens =
            i32::try_from(owners.input.get_num_objects_found("WindowMaterial:Screen"))
                .map_err(|_| unavailable("Screen count exceeds Native integer"))?;
        if usize::try_from(self.num_screens).ok() != Some(raw.window_screen.len()) {
            return Err(unavailable("Screen raw/IP count"));
        }
        for object in &raw.window_screen {
            let (alphas, numbers) = raw.get_window_screen_item(object, owners)?;
            if alphas != 2 || !(7..=12).contains(&numbers) {
                return Err(unavailable("Screen2A12N/min9 primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                severe(
                    owners,
                    &format!("GetMaterialData: WindowMaterial:Screen = {name}, duplicate name."),
                )?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::WindowScreen)?;
            let position = self.insert(&name, id, SourceMaterialGroup::Screen)?;
            let material = self.window_screen.entry(id).or_default();
            material.beam_reflectance_model = match owners.shortcut.alpha_args[1].as_str() {
                "DONOTMODEL" => 0,
                "MODELASDIRECTBEAM" => 1,
                "MODELASDIFFUSE" => 2,
                _ => -1,
            };
            if material.beam_reflectance_model == -1 {
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{}=\"{}\", must be one of DoNotModel, ModelAsDirectBeam or ModelAsDiffuse.",
                        owners.shortcut.alpha_field_names[1], owners.shortcut.alpha_args[1]
                    ),
                )?;
                *owners.errors_found = true;
            }
            material.roughness = Some(MaterialSurfaceRoughness::MediumRough);
            material.shade_reflectance = owners.shortcut.numeric_args[0];
            if material.shade_reflectance < 0.0 || material.shade_reflectance > 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be >= 0 and <= 1",
                        owners.shortcut.numeric_field_names[0]
                    ),
                )?;
            }
            material.shade_visible_reflectance = owners.shortcut.numeric_args[1];
            if material.shade_visible_reflectance < 0.0 || material.shade_visible_reflectance > 1.0
            {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be >= 0 and <= 1 for material {name}.",
                        owners.shortcut.numeric_field_names[1]
                    ),
                )?;
            }
            material.thermal_absorptance = owners.shortcut.numeric_args[2];
            material.thermal_absorptance_input = owners.shortcut.numeric_args[2];
            if material.thermal_absorptance < 0.0 || material.thermal_absorptance > 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be >= 0 and <= 1",
                        owners.shortcut.numeric_field_names[2]
                    ),
                )?;
            }
            material.conductivity = owners.shortcut.numeric_args[3];
            material.thickness = owners.shortcut.numeric_args[5];
            if owners.shortcut.numeric_args[4] > 0.0 {
                if owners.shortcut.numeric_args[5] / owners.shortcut.numeric_args[4] >= 1.0 {
                    *owners.errors_found = true;
                    severe(
                        owners,
                        &format!("WindowMaterial:Screen=\"{name}\", Illegal value combination."),
                    )?;
                    continuation(
                        owners,
                        &format!(
                            "{} must be less than {}",
                            owners.shortcut.numeric_field_names[5],
                            owners.shortcut.numeric_field_names[4]
                        ),
                    )?;
                } else {
                    let operand =
                        1.0 - owners.shortcut.numeric_args[5] / owners.shortcut.numeric_args[4];
                    material.transmittance = operand * operand;
                }
            } else {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!("{} must be > 0.", owners.shortcut.numeric_field_names[4]),
                )?;
                owners.shortcut.numeric_args[4] = 0.000000001;
            }
            if owners.shortcut.numeric_args[5] <= 0.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!("{} must be > 0.", owners.shortcut.numeric_field_names[5]),
                )?;
            }
            material.shade_reflectance *= 1.0 - material.transmittance;
            material.shade_visible_reflectance *= 1.0 - material.transmittance;
            material.to_glass_distance = owners.shortcut.numeric_args[6];
            if material.to_glass_distance < 0.001 || material.to_glass_distance > 1.0 {
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be greater than or equal to 0.001 and less than or equal to 1.",
                        owners.shortcut.numeric_field_names[6]
                    ),
                )?;
            }
            material.top_opening = owners.shortcut.numeric_args[7];
            if material.top_opening < 0.0 || material.top_opening > 1.0 {
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be greater than or equal to 0 and less than or equal to 1.",
                        owners.shortcut.numeric_field_names[7]
                    ),
                )?;
            }
            material.bottom_opening = owners.shortcut.numeric_args[8];
            if material.bottom_opening < 0.0 || material.bottom_opening > 1.0 {
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be greater than or equal to 0 and less than or equal to 1.",
                        owners.shortcut.numeric_field_names[8]
                    ),
                )?;
            }
            material.left_opening = owners.shortcut.numeric_args[9];
            if material.left_opening < 0.0 || material.left_opening > 1.0 {
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be greater than or equal to 0 and less than or equal to 1.",
                        owners.shortcut.numeric_field_names[9]
                    ),
                )?;
            }
            material.right_opening = owners.shortcut.numeric_args[10];
            if material.right_opening < 0.0 || material.right_opening > 1.0 {
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be greater than or equal to 0 and less than or equal to 1.",
                        owners.shortcut.numeric_field_names[10]
                    ),
                )?;
            }
            material.map_degree_resolution = owners.shortcut.numeric_args[11] as i32;
            if material.map_degree_resolution < 0
                || material.map_degree_resolution > 5
                || material.map_degree_resolution == 4
            {
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be 0, 1, 2, 3, or 5.",
                        owners.shortcut.numeric_field_names[11]
                    ),
                )?;
                *owners.errors_found = true;
            }
            material.airflow_permeability = material.transmittance;
            material.thermal_transmittance = material.transmittance;
            material.visible_transmittance = material.transmittance;
            self.resistance_only.insert(id, true);
            material.solar_absorptance =
                (1.0 - material.transmittance - material.shade_reflectance).max(0.0);
            material.solar_absorptance_input = material.solar_absorptance;
            material.visible_absorptance =
                (1.0 - material.visible_transmittance - material.shade_visible_reflectance)
                    .max(0.0);
            material.visible_absorptance_input = material.visible_absorptance;
            material.thermal_absorptance *= 1.0 - material.transmittance;
            material.thermal_absorptance_input = material.thermal_absorptance;
            if material.conductivity > 0.0 {
                self.nominal_r[position] =
                    (1.0 - material.transmittance) * material.thickness / material.conductivity;
            } else {
                self.nominal_r[position] = 1.0;
                warning(
                    owners,
                    &format!(
                        "Conductivity for material=\"{name}\" must be greater than 0 for calculating Nominal R-value, Nominal R is defaulted to 1 and the simulation continues."
                    ),
                )?;
            }
            if material.transmittance + material.shade_reflectance >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    "Calculated solar transmittance + solar reflectance not < 1.0",
                )?;
                continuation(
                    owners,
                    "See Engineering Reference for calculation procedure for solar transmittance.",
                )?;
            }
            if material.visible_transmittance + material.shade_visible_reflectance >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    "Calculated visible transmittance + visible reflectance not < 1.0",
                )?;
                continuation(
                    owners,
                    "See Engineering Reference for calculation procedure for visible solar transmittance.",
                )?;
            }
            if material.thermal_transmittance + material.thermal_absorptance >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen=\"{name}\", Illegal value combination."),
                )?;
                severe(
                    owners,
                    "Thermal hemispherical emissivity plus open area fraction (1-diameter/spacing)**2 not < 1.0",
                )?;
            }
        }
        Ok(())
    }
}
