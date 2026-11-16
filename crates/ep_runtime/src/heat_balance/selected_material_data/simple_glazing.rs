//! Material.cc2389-2429 and3103-3276: actual GlassSimple row and ordered Setup.
use super::{
    MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, same, typed_id, unavailable,
};
use crate::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialDefinition, MaterialKind, MaterialSurfaceRoughness};

// Only this actual constructor-selected optical model is reached here.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum SourceSimpleGlazingOpticalModel {
    #[default]
    SpectralAverage,
}

/// Sole runtime GlassSimple properties. Typed derived values remain advisory.
// The unused material slice retains Native ctor/Setup writes that later optical
// consumers will read; those consumers are explicitly not admitted here.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceSimpleGlazingMaterial {
    pub u_factor: f64,
    pub shgc: f64,
    pub visible_input: f64,
    pub visible_input_by_user: bool,
    pub roughness: Option<MaterialSurfaceRoughness>,
    pub thickness: f64,
    pub conductivity: f64,
    pub resistance: f64,
    pub density: f64,
    pub specific_heat: f64,
    pub thermal_absorptance: f64,
    pub solar_absorptance: f64,
    pub visible_absorptance: f64,
    pub thermal_back: f64,
    pub thermal_front: f64,
    pub thermal_transmittance: f64,
    pub spectral_data_pointer: i32,
    pub solar_diffusing: bool,
    pub dirt_factor: f64,
    pub optical_model: SourceSimpleGlazingOpticalModel,
    pub transmittance: f64,
    pub solar_reflectance_back: f64,
    pub solar_reflectance_front: f64,
    pub visible_transmittance: f64,
    pub visible_reflectance_back: f64,
    pub visible_reflectance_front: f64,
    pub young_modulus: f64,
    pub poissons_ratio: f64,
}

impl Default for SourceSimpleGlazingMaterial {
    fn default() -> Self {
        Self {
            u_factor: 0.0,
            shgc: 0.0,
            visible_input: 0.0,
            visible_input_by_user: false,
            roughness: None,
            thickness: 0.0,
            conductivity: 0.0,
            resistance: 0.0,
            density: 0.0,
            specific_heat: 0.0,
            thermal_absorptance: 0.0,
            solar_absorptance: 0.0,
            visible_absorptance: 0.0,
            thermal_back: 0.0,
            thermal_front: 0.0,
            thermal_transmittance: 0.0,
            spectral_data_pointer: 0,
            solar_diffusing: false,
            dirt_factor: 1.0,
            optical_model: SourceSimpleGlazingOpticalModel::SpectralAverage,
            transmittance: 0.0,
            solar_reflectance_back: 0.0,
            solar_reflectance_front: 0.0,
            visible_transmittance: 0.0,
            visible_reflectance_back: 0.0,
            visible_reflectance_front: 0.0,
            young_modulus: 0.0,
            poissons_ratio: 0.0,
        }
    }
}

fn squared(value: f64) -> f64 {
    value * value
}
fn cubed(value: f64) -> f64 {
    value * value * value
}

impl SourceSimpleGlazingMaterial {
    fn setup(
        &mut self,
        nominal_r: &mut f64,
        name: &str,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        let mut errors_found = false;
        self.spectral_data_pointer = 0;
        self.solar_diffusing = false;
        self.roughness = Some(MaterialSurfaceRoughness::VerySmooth);
        self.thermal_transmittance = 0.0;
        self.thermal_back = 0.84;
        self.thermal_front = 0.84;
        self.thermal_absorptance = self.thermal_back;

        let riw = if self.u_factor < 5.85 {
            1.0 / (0.359073 * self.u_factor.ln() + 6.949915)
        } else {
            1.0 / (1.788041 * self.u_factor - 2.886625)
        };
        let row = 1.0 / (0.025342 * self.u_factor + 29.163853);
        let mut rlw = (1.0 / self.u_factor) - riw - row;
        if rlw <= 0.0 {
            rlw = rlw.max(0.001);
            let flags = owners
                .diagnostic_flags
                .as_ref()
                .ok_or_else(|| unavailable("actual SimpleGlazing warning phase owners"))?;
            owners.backend.show_source_warning_error(
                &format!("WindowMaterial:SimpleGlazingSystem: {name} has U-factor higher than that provided by surface film resistances, Check value of U-factor"),
                flags,
            ).map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
        }
        self.thickness = if (1.0 / rlw) > 7.0 {
            0.002
        } else {
            0.05914 - (0.00714 / rlw)
        };
        self.conductivity = self.thickness / rlw;
        if self.conductivity > 0.0 {
            self.resistance = rlw;
            *nominal_r = self.resistance;
        } else {
            errors_found = true;
            let flags = owners
                .diagnostic_flags
                .as_ref()
                .ok_or_else(|| unavailable("actual SimpleGlazing severe phase owners"))?;
            owners.backend.show_source_severe_error(
                &format!("WindowMaterial:SimpleGlazingSystem: {name} has Conductivity <= 0.0, must be >0.0, Check value of U-factor"),
                flags,
            ).map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
        }

        if self.u_factor > 4.5 {
            self.transmittance = if self.shgc < 0.7206 {
                0.939998 * squared(self.shgc) + 0.20332 * self.shgc
            } else {
                1.30415 * self.shgc - 0.30515
            };
        } else if self.u_factor < 3.4 {
            self.transmittance = if self.shgc <= 0.15 {
                0.41040 * self.shgc
            } else {
                0.085775 * squared(self.shgc) + 0.963954 * self.shgc - 0.084958
            };
        } else {
            let high = if self.shgc < 0.7206 {
                0.939998 * squared(self.shgc) + 0.20332 * self.shgc
            } else {
                1.30415 * self.shgc - 0.30515
            };
            let low = if self.shgc <= 0.15 {
                0.41040 * self.shgc
            } else {
                0.085775 * squared(self.shgc) + 0.963954 * self.shgc - 0.084958
            };
            self.transmittance = ((self.u_factor - 3.4) / (4.5 - 3.4)) * (high - low) + low;
        }
        if self.transmittance < 0.0 {
            self.transmittance = 0.0;
        }
        let delta = self.shgc - self.transmittance;
        let (ris, ros) = if self.u_factor > 4.5 {
            (
                1.0 / (29.436546 * cubed(delta) - 21.943415 * squared(delta)
                    + 9.945872 * delta
                    + 7.426151),
                1.0 / (2.225824 * delta + 20.577080),
            )
        } else if self.u_factor < 3.4 {
            (
                1.0 / (199.8208128 * cubed(delta) - 90.639733 * squared(delta)
                    + 19.737055 * delta
                    + 6.766575),
                1.0 / (5.763355 * delta + 20.541528),
            )
        } else {
            let low = 1.0
                / (199.8208128 * cubed(delta) - 90.639733 * squared(delta)
                    + 19.737055 * delta
                    + 6.766575);
            let high = 1.0
                / (29.436546 * cubed(delta) - 21.943415 * squared(delta)
                    + 9.945872 * delta
                    + 7.426151);
            let ris = ((self.u_factor - 3.4) / (4.5 - 3.4)) * (low - high) + low;
            let low = 1.0 / (5.763355 * delta + 20.541528);
            let high = 1.0 / (2.225824 * delta + 20.577080);
            let ros = ((self.u_factor - 3.4) / (4.5 - 3.4)) * (low - high) + low;
            (ris, ros)
        };
        let inflow_fraction = (ros + 0.5 * rlw) / (ros + rlw + ris);
        let solar_absorb = (self.shgc - self.transmittance) / inflow_fraction;
        self.solar_reflectance_back = 1.0 - self.transmittance - solar_absorb;
        self.solar_reflectance_front = self.solar_reflectance_back;
        if self.visible_input_by_user {
            self.visible_transmittance = self.visible_input;
            self.visible_reflectance_back = -0.7409 * cubed(self.visible_transmittance)
                + 1.6531 * squared(self.visible_transmittance)
                - 1.2299 * self.visible_transmittance
                + 0.4545;
            if self.visible_transmittance + self.visible_reflectance_back >= 1.0 {
                self.visible_reflectance_back = 0.999 - self.visible_transmittance;
            }
            self.visible_reflectance_front = -0.0622 * cubed(self.visible_transmittance)
                + 0.4277 * squared(self.visible_transmittance)
                - 0.4169 * self.visible_transmittance
                + 0.2399;
            if self.visible_transmittance + self.visible_reflectance_front >= 1.0 {
                self.visible_reflectance_front = 0.999 - self.visible_transmittance;
            }
        } else {
            self.visible_transmittance = self.transmittance;
            self.visible_reflectance_back = self.solar_reflectance_back;
            self.visible_reflectance_front = self.solar_reflectance_front;
        }
        if errors_found {
            match owners.backend.show_fatal_error(
                "Program halted because of input problem(s) in WindowMaterial:SimpleGlazingSystem",
            ) {
                Ok(never) => match never {},
                Err(stop) => {
                    return Err(RuntimeError::HeatBalanceInput(Box::new(
                        HeatBalanceInputError::Fatal(stop),
                    )));
                }
            }
        }
        Ok(())
    }
}

impl SelectedMaterialData {
    pub(super) fn get_simple_glazing_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:SimpleGlazingSystem".into();
        self.num_simple_windows = super::count(owners.input, "WindowMaterial:SimpleGlazingSystem")?;
        if usize::try_from(self.num_simple_windows).ok() != Some(raw.simple_glazing.len()) {
            return Err(unavailable("SimpleGlazing raw/IP count"));
        }
        for object in &raw.simple_glazing {
            raw.get_simple_glazing_item(object, owners)?;
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                let flags = owners
                    .diagnostic_flags
                    .as_ref()
                    .ok_or_else(|| unavailable("actual SimpleGlazing duplicate phase owners"))?;
                owners.backend.show_source_severe_error(
                    &format!("GetMaterialData: WindowMaterial:SimpleGlazingSystem = {name}, duplicate name."), flags,
                ).map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::WindowSimpleGlazing)?;
            let material = SourceSimpleGlazingMaterial::default();
            let index = self.insert(&name, id, SourceMaterialGroup::GlassSimple)?;
            self.simple_glazing.insert(id, material);
            let material = self
                .simple_glazing
                .get_mut(&id)
                .ok_or_else(|| unavailable("allocated SimpleGlazing property row"))?;
            material.u_factor = owners.shortcut.numeric_args[0];
            material.shgc = owners.shortcut.numeric_args[1];
            if !owners.shortcut.numeric_field_blanks[2] {
                material.visible_input = owners.shortcut.numeric_args[2];
                material.visible_input_by_user = true;
            }
            let typed = owners
                .model
                .materials
                .iter()
                .find(|row| row.id == id)
                .ok_or_else(|| unavailable("SimpleGlazing typed input ID"))?;
            let MaterialDefinition::WindowSimpleGlazing(typed) = typed.definition else {
                return Err(unavailable("SimpleGlazing typed input family"));
            };
            same(
                material.u_factor,
                typed.u_factor_with_film_coefficients_w_per_m2_k,
            )?;
            same(material.shgc, typed.solar_heat_gain_coefficient)?;
            match (
                material.visible_input_by_user,
                typed.input_visible_transmittance_at_normal_incidence,
            ) {
                (true, Some(value)) => same(material.visible_input, value)?,
                (false, None) => {}
                _ => return Err(unavailable("SimpleGlazing optional VT input association")),
            }
            material.setup(&mut self.nominal_r[index], &name, owners)?;
        }
        Ok(())
    }
}

fn glazing_custom(
    owners: &mut MaterialDataOwners<'_, '_>,
    name: &str,
    message: &str,
    warning: bool,
) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual Glazing diagnostic phase owners"))?;
    let header = format!("GetMaterialData: WindowMaterial:Glazing = {name}");
    if warning {
        owners.backend.show_source_warning_error(&header, flags)
    } else {
        owners.backend.show_source_severe_error(&header, flags)
    }
    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
    owners
        .backend
        .show_continue_error(message)
        .map_err(|error| {
            RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Backend(error)))
        })
}

impl SelectedMaterialData {
    // Native Material.cc417-612: ordinary SpectralAverage. SGS Setup is never called.
    pub(super) fn get_glazing_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Glazing".into();
        self.num_w5_glazings = super::count(owners.input, "WindowMaterial:Glazing")?;
        if usize::try_from(self.num_w5_glazings).ok() != Some(raw.glazing.len()) {
            return Err(unavailable("Glazing raw/IP count"));
        }
        for object in &raw.glazing {
            raw.get_glazing_item(object, owners)?;
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                let flags = owners
                    .diagnostic_flags
                    .as_ref()
                    .ok_or_else(|| unavailable("actual Glazing duplicate phase owners"))?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!(
                            "GetMaterialData: WindowMaterial:Glazing = {name}, duplicate name."
                        ),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::WindowGlazing)?;
            let position = self.insert(&name, id, SourceMaterialGroup::Glass)?;
            self.simple_glazing
                .insert(id, SourceSimpleGlazingMaterial::default());
            let material = self
                .simple_glazing
                .get_mut(&id)
                .ok_or_else(|| unavailable("allocated ordinary Glass row"))?;
            material.roughness = Some(MaterialSurfaceRoughness::VerySmooth);
            self.resistance_only.insert(id, true);
            material.thickness = owners.shortcut.numeric_args[0];
            if owners.shortcut.alpha_args[1] != "SPECTRALAVERAGE" {
                return Err(unavailable("ordinary Glazing optical owner beyond Average"));
            }
            material.optical_model = SourceSimpleGlazingOpticalModel::SpectralAverage;
            material.transmittance = owners.shortcut.numeric_args[1];
            material.solar_reflectance_front = owners.shortcut.numeric_args[2];
            material.solar_reflectance_back = owners.shortcut.numeric_args[3];
            material.visible_transmittance = owners.shortcut.numeric_args[4];
            material.visible_reflectance_front = owners.shortcut.numeric_args[5];
            material.visible_reflectance_back = owners.shortcut.numeric_args[6];
            material.thermal_transmittance = owners.shortcut.numeric_args[7];
            material.thermal_front = owners.shortcut.numeric_args[8];
            material.thermal_back = owners.shortcut.numeric_args[9];
            material.conductivity = owners.shortcut.numeric_args[10];
            material.dirt_factor = owners.shortcut.numeric_args[11];
            material.young_modulus = owners.shortcut.numeric_args[12];
            material.poissons_ratio = owners.shortcut.numeric_args[13];
            if owners.shortcut.numeric_args[11] == 0.0 {
                material.dirt_factor = 1.0;
            }
            material.thermal_absorptance = material.thermal_back;
            if material.conductivity > 0.0 {
                material.resistance = material.thickness / material.conductivity;
                self.nominal_r[position] = material.resistance;
            } else {
                *owners.errors_found = true;
                let flags = owners
                    .diagnostic_flags
                    .as_ref()
                    .ok_or_else(|| unavailable("actual Glazing conductivity phase owners"))?;
                owners.backend.show_source_severe_error(&format!("Window glass material {name} has Conductivity = 0.0, must be >0.0, default = .9"), flags)
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
            }
            // This second actual enum read follows the conductivity IO.
            if owners.shortcut.alpha_args[1] != "SPECTRALAVERAGE" {
                return Err(unavailable("ordinary Glazing optical owner beyond Average"));
            }
            material.optical_model = SourceSimpleGlazingOpticalModel::SpectralAverage;
            for (a, b) in [(1, 2), (1, 3), (4, 5), (4, 6), (7, 8), (7, 9)] {
                if owners.shortcut.numeric_args[a] + owners.shortcut.numeric_args[b] > 1.0 {
                    *owners.errors_found = true;
                    let message = format!(
                        "{} + {} not <= 1.0",
                        owners.shortcut.numeric_field_names[a],
                        owners.shortcut.numeric_field_names[b]
                    );
                    glazing_custom(owners, &name, &message, false)?;
                }
            }
            if owners.shortcut.numeric_args[1] < 0.0 {
                let message = format!("{} not >= 0.0", owners.shortcut.numeric_field_names[1]);
                glazing_custom(owners, &name, &message, false)?;
                *owners.errors_found = true;
            }
            if owners.shortcut.numeric_args[1] > 1.0 {
                *owners.errors_found = true;
                let message = format!("{} not <= 1.0", owners.shortcut.numeric_field_names[1]);
                glazing_custom(owners, &name, &message, false)?;
            }
            for index in [2, 3] {
                let below_lower = owners.shortcut.numeric_args[index] < 0.0;
                let above_upper = owners.shortcut.numeric_args[index] > 1.0;
                if below_lower || above_upper {
                    *owners.errors_found = true;
                    let message = format!(
                        "{} not >= 0.0 and <= 1.0",
                        owners.shortcut.numeric_field_names[index]
                    );
                    glazing_custom(owners, &name, &message, false)?;
                }
            }
            if owners.shortcut.numeric_args[4] < 0.0 {
                let message = format!("{} not >= 0.0", owners.shortcut.numeric_field_names[4]);
                glazing_custom(owners, &name, &message, true)?;
            }
            if owners.shortcut.numeric_args[4] > 1.0 {
                *owners.errors_found = true;
                let message = format!("{} not <= 1.0", owners.shortcut.numeric_field_names[4]);
                glazing_custom(owners, &name, &message, false)?;
            }
            for index in [5, 6] {
                let below_lower = owners.shortcut.numeric_args[index] < 0.0;
                let above_upper = owners.shortcut.numeric_args[index] > 1.0;
                if below_lower || above_upper {
                    *owners.errors_found = true;
                    let message = format!(
                        "{} not >= 0.0 and <= 1.0",
                        owners.shortcut.numeric_field_names[index]
                    );
                    glazing_custom(owners, &name, &message, false)?;
                }
            }
            if owners.shortcut.numeric_args[7] > 1.0 {
                *owners.errors_found = true;
                let message = format!("{} not <= 1.0", owners.shortcut.numeric_field_names[7]);
                glazing_custom(owners, &name, &message, false)?;
            }
            let below_lower = owners.shortcut.numeric_args[8] <= 0.0;
            let above_upper = owners.shortcut.numeric_args[8] >= 1.0;
            if below_lower || above_upper {
                *owners.errors_found = true;
                let message = format!(
                    "{} not > 0.0 and < 1.0",
                    owners.shortcut.numeric_field_names[8]
                );
                glazing_custom(owners, &name, &message, false)?;
            }
            let below_lower = owners.shortcut.numeric_args[9] <= 0.0;
            let above_upper = owners.shortcut.numeric_args[9] >= 1.0;
            if below_lower || above_upper {
                *owners.errors_found = true;
                let flags = owners
                    .diagnostic_flags
                    .as_ref()
                    .ok_or_else(|| unavailable("actual Glazing back emissivity phase owners"))?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!("WindowMaterial:Glazing=\"{name}\", Illegal value."),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                owners
                    .backend
                    .show_continue_error(&format!(
                        "{} not > 0.0 and < 1.0",
                        owners.shortcut.numeric_field_names[9]
                    ))
                    .map_err(|error| {
                        RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Backend(
                            error,
                        )))
                    })?;
            }
            if owners.shortcut.numeric_args[10] <= 0.0 {
                *owners.errors_found = true;
                let message = format!("{} not > 0.0", owners.shortcut.numeric_field_names[10]);
                glazing_custom(owners, &name, &message, false)?;
            }
            if owners.shortcut.numeric_args[12] < 0.0 {
                *owners.errors_found = true;
                let message = format!("{} not > 0.0", owners.shortcut.numeric_field_names[12]);
                glazing_custom(owners, &name, &message, false)?;
            }
            let below_lower = owners.shortcut.numeric_args[13] < 0.0;
            let above_upper = owners.shortcut.numeric_args[13] >= 1.0;
            if below_lower || above_upper {
                *owners.errors_found = true;
                let message = format!(
                    "{} not > 0.0 and < 1.0",
                    owners.shortcut.numeric_field_names[13]
                );
                glazing_custom(owners, &name, &message, false)?;
            }
            if owners.shortcut.alpha_args[3].is_empty() {
                material.solar_diffusing = false;
            } else if matches!(owners.shortcut.alpha_args[3].as_str(), "YES" | "NO") {
                material.solar_diffusing = owners.shortcut.alpha_args[3] == "YES";
            } else {
                *owners.errors_found = true;
                let flags = owners
                    .diagnostic_flags
                    .as_ref()
                    .ok_or_else(|| unavailable("actual Glazing diffusing phase owners"))?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!("WindowMaterial:Glazing=\"{name}\", Illegal value."),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                owners
                    .backend
                    .show_continue_error(&format!(
                        "{} must be Yes or No, entered value={}",
                        owners.shortcut.numeric_field_names[3], owners.shortcut.alpha_args[3]
                    ))
                    .map_err(|error| {
                        RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Backend(
                            error,
                        )))
                    })?;
            }
        }
        Ok(())
    }
}
