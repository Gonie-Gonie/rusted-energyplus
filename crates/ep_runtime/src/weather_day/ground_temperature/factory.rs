//! Actual four factory branches; shared blank/name arrays are not passed.
use super::*;
pub(super) fn initialize(
    model: &mut SourceSiteGroundTemperatureModel,
    kind: SiteGroundModelKind,
    count: usize,
    owners: &mut SourceGroundTemperatureOwners<'_, '_>,
) -> Result<bool, Error> {
    if kind == SiteGroundModelKind::FCFactorMethod {
        let mut found = false;
        if count == 1 {
            let numbers = input::get_object_item(owners.raw, kind, owners.input, owners.shortcut)?;
            if numbers < 12 {
                owners.backend.show_severe_error(&format!(
                    "{}: Less than 12 values entered.",
                    kind.family()
                ))?;
            } else {
                for (i, value) in model.monthly_temperatures.iter_mut().enumerate() {
                    *value = owners.shortcut.numeric_args[i];
                }
                owners.environment.ground_temp_inputs[kind.index()] = true;
                found = true;
            }
        } else if count > 1 {
            owners.backend.show_severe_error(&format!(
                "{}: Too many objects entered. Only one allowed.",
                kind.family()
            ))?;
        } else if owners.header.weather_file_fc_ground_temps {
            for (i, value) in model.monthly_temperatures.iter_mut().enumerate() {
                *value = owners.header.ground_temps_fc_from_epw_header[i];
            }
            owners.environment.ground_temp_inputs[kind.index()] = true;
            found = true;
        } else {
            model.monthly_temperatures.fill(0.0);
            found = true;
        }
        if owners.environment.ground_temp_inputs[kind.index()] {
            output::write_ground_temperatures(
                owners.eio,
                kind.output_name(),
                &model.monthly_temperatures,
            )?;
        }
        return Ok(found);
    }
    let mut errors_found = false;
    if count == 1 {
        let mut general_warning = false;
        let numbers = input::get_object_item(owners.raw, kind, owners.input, owners.shortcut)?;
        if numbers < 12 {
            owners
                .backend
                .show_severe_error(&format!("{}: Less than 12 values entered.", kind.family()))?;
            errors_found = true;
        }
        for (i, value) in model.monthly_temperatures.iter_mut().enumerate() {
            *value = owners.shortcut.numeric_args[i];
            if kind == SiteGroundModelKind::BuildingSurface && (*value < 15.0 || *value > 25.0) {
                general_warning = true;
            }
        }
        owners.environment.ground_temp_inputs[kind.index()] = true;
        if kind == SiteGroundModelKind::BuildingSurface && general_warning {
            owners.backend.show_warning_error(&format!(
                "{}: Some values fall outside the range of 15-25C.",
                kind.family()
            ))?;
            owners.backend.show_continue_error("These values may be inappropriate.  Please consult the Input Output Reference for more details.")?;
        }
    } else if count > 1 {
        owners.backend.show_severe_error(&format!(
            "{}: Too many objects entered. Only one allowed.",
            kind.family()
        ))?;
        errors_found = true;
    } else {
        model.monthly_temperatures.fill(kind.default_temperature());
    }
    output::write_ground_temperatures(owners.eio, kind.output_name(), &model.monthly_temperatures)?;
    Ok(!errors_found)
}
