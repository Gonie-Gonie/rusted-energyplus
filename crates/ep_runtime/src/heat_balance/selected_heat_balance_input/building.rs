//! Building getter writes before terrain-dependent GetSiteAtmosphereData.
use super::SelectedSolarDistribution;
use super::{
    HeatBalanceInputError, HeatBalanceInputOwners, SelectedHeatBalanceInput,
    SelectedHeatBalanceInputState, SourceErrorBackend, rounded,
};
use std::io::Write;

impl SelectedHeatBalanceInputState {
    pub(super) fn get_building(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        owners: &mut HeatBalanceInputOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
        errors_found: &mut bool,
    ) -> Result<(), HeatBalanceInputError> {
        *owners.current_module_object = "Building".into();
        let (terrain, solar_label) = if owners.input.get_num_objects_found("Building") > 0 {
            let mut values =
                raw.get_object_item::<4, 5>("Building", 0, owners.input, owners.shortcut)?;
            self.building_name = values.alphas[0]
                .replace('\u{1}', ",")
                .replace('\u{2}', "!")
                .replace('\u{3}', "\\");
            self.building_azimuth = values.numbers[0] % 360.0;
            let (exponent, height, terrain) = match values.alphas[1].as_str() {
                "COUNTRY" | "1" => (0.14, 270.0, "Country"),
                "SUBURBS" | "SUBURB" | "2" => (0.22, 370.0, "Suburbs"),
                "CITY" | "3" => (0.33, 460.0, "City"),
                "OCEAN" => (0.10, 210.0, "Ocean"),
                "URBAN" => (0.22, 370.0, "Urban"),
                _ => {
                    return Err(HeatBalanceInputError::Unavailable(
                        "unadmitted terrain error branch",
                    ));
                }
            };
            owners
                .environment
                .outdoor_environment_parameters
                .site_wind_exponent = exponent;
            owners
                .environment
                .outdoor_environment_parameters
                .site_wind_boundary_layer_height_m = height;
            self.loads_convergence_tolerance = values.numbers[1];
            self.temperature_convergence_tolerance = values.numbers[2];
            // The same ordinary raw admission requires both tolerances positive.
            if self.loads_convergence_tolerance <= 0.0
                || self.temperature_convergence_tolerance <= 0.0
            {
                return Err(HeatBalanceInputError::Unavailable(
                    "unadmitted convergence error branch",
                ));
            }
            let solar_label = if values.alphas[2].starts_with("MIN")
                || values.alphas[2] == "-1"
                || owners.project.system.minimal_shadowing
            {
                self.solar_distribution = SelectedSolarDistribution::Minimal;
                self.calculate_solar_reflections = false;
                "MinimalShadowing"
            } else {
                match values.alphas[2].as_str() {
                    "FULLEXTERIOR" | "0" => {
                        self.solar_distribution = SelectedSolarDistribution::FullExterior;
                        self.calculate_solar_reflections = false;
                        "FullExterior"
                    }
                    "FULLINTERIORANDEXTERIOR" | "1" => {
                        self.solar_distribution = SelectedSolarDistribution::FullInteriorExterior;
                        self.calculate_solar_reflections = false;
                        "FullInteriorAndExterior"
                    }
                    "FULLEXTERIORWITHREFLECTIONS" => {
                        self.solar_distribution = SelectedSolarDistribution::FullExterior;
                        self.calculate_solar_reflections = true;
                        "FullExteriorWithReflectionsFromExteriorSurfaces"
                    }
                    "FULLINTERIORANDEXTERIORWITHREFLECTIONS" => {
                        self.solar_distribution = SelectedSolarDistribution::FullInteriorExterior;
                        self.calculate_solar_reflections = true;
                        "FullInteriorAndExteriorWithReflectionsFromExteriorSurfaces"
                    }
                    _ => {
                        return Err(HeatBalanceInputError::Unavailable(
                            "unadmitted solar-distribution error branch",
                        ));
                    }
                }
            };
            self.max_number_of_warmup_days = if !owners.shortcut.numeric_field_blanks[3] {
                source_int(values.numbers[3])?
            } else {
                25
            };
            owners.project.min_number_of_warmup_days = if !owners.shortcut.numeric_field_blanks[4] {
                source_int(values.numbers[4])?
            } else {
                1
            };
            if self.max_number_of_warmup_days <= 0 || owners.project.min_number_of_warmup_days <= 0
            {
                return Err(HeatBalanceInputError::Unavailable(
                    "unadmitted nonpositive warmup branch",
                ));
            }
            if owners.project.min_number_of_warmup_days > self.max_number_of_warmup_days {
                backend.show_warning_error(&format!("GetProjectControlData: Building: {} [{}]  is greater than {} [{}], {} will be used.",
                    owners.shortcut.numeric_field_names[4], owners.project.min_number_of_warmup_days,
                    owners.shortcut.numeric_field_names[3], self.max_number_of_warmup_days, owners.project.min_number_of_warmup_days))?;
                self.max_number_of_warmup_days = owners.project.min_number_of_warmup_days;
            }
            // These are locals, not the shared alpha_args array.
            values.alphas[1] = terrain.into();
            values.alphas[2] = solar_label.into();
            (values.alphas[1].clone(), values.alphas[2].clone())
        } else {
            backend
                .show_severe_error("GetProjectControlData:  A Building Object must be entered.")?;
            *errors_found = true;
            self.building_name = "NOT ENTERED".into();
            self.max_number_of_warmup_days = 25;
            owners.project.min_number_of_warmup_days = 1;
            ("NOT ENTERED".into(), "NOT ENTERED".into())
        };
        writeln!(
            eio,
            "! <Building Information>, Building Name,North Axis {{deg}},Terrain,  Loads Convergence Tolerance Value,Temperature Convergence Tolerance Value,  Solar Distribution,Maximum Number of Warmup Days,Minimum Number of Warmup Days"
        )?;
        writeln!(
            eio,
            " Building Information,{},{},{},{},{},{},{},{}",
            self.building_name,
            rounded(self.building_azimuth, 3)?,
            terrain,
            rounded(self.loads_convergence_tolerance, 5)?,
            rounded(self.temperature_convergence_tolerance, 5)?,
            solar_label,
            self.max_number_of_warmup_days,
            owners.project.min_number_of_warmup_days
        )?;
        Ok(())
    }
}
fn source_int(value: f64) -> Result<i32, HeatBalanceInputError> {
    if !value.is_finite() || value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
        Err(HeatBalanceInputError::Unavailable(
            "undefined original signed-int conversion",
        ))
    } else {
        Ok(value as i32)
    }
}
