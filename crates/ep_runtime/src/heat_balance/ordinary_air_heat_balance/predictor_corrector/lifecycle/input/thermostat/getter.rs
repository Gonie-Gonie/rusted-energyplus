//! Selected real getObjectItem values on the sole IP/shared shortcut owners.
//! Thermostat consumes physical counts; Dual's dead count locals are unrepresented.
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut, SourceInputStatisticsFields,
    SourceInputStatisticsValue as V,
};

const THERMOSTAT: &str = "ZoneControl:Thermostat";
const DUAL: &str = "ThermostatSetpoint:DualSetpoint";
const THERMOSTAT_ALPHAS: [(&str, &str); 11] = [
    ("name", "Name"),
    ("zone_or_zonelist_name", "Zone or ZoneList Name"),
    ("control_type_schedule_name", "Control Type Schedule Name"),
    ("control_1_object_type", "Control 1 Object Type"),
    ("control_1_name", "Control 1 Name"),
    ("control_2_object_type", "Control 2 Object Type"),
    ("control_2_name", "Control 2 Name"),
    ("control_3_object_type", "Control 3 Object Type"),
    ("control_3_name", "Control 3 Name"),
    ("control_4_object_type", "Control 4 Object Type"),
    ("control_4_name", "Control 4 Name"),
];
const DUAL_ALPHAS: [(&str, &str); 3] = [
    ("name", "Name"),
    ("heating_setpoint_temperature_schedule_name", "Heating Setpoint Temperature Schedule Name"),
    ("cooling_setpoint_temperature_schedule_name", "Cooling Setpoint Temperature Schedule Name"),
];
const CUTOUT_FIELD: &str = "temperature_difference_between_cutout_and_setpoint";
const CUTOUT_LABEL: &str = "Temperature Difference Between Cutout And Setpoint";

fn object_name(
    input: &SelectedInputProcessor, family: &str, number: usize, source_is_epjson: bool,
) -> Result<String, Error> {
    let names = input.source_raw_object_names(family)
        .map_err(|_| Error::Unavailable("actual thermostat retained object order"))?;
    if family == DUAL && !source_is_epjson && names.len() != 1 {
        return Err(Error::Unavailable("actual multiple Dual IDF getJSONObjNum order not retained"));
    }
    number.checked_sub(1).and_then(|at| names.get(at)).cloned()
        .ok_or(Error::Unavailable("actual thermostat getter object ordinal"))
}

fn thermostat_max_fields(fields: &SourceInputStatisticsFields, source_is_epjson: bool) -> Result<usize, Error> {
    if source_is_epjson {
        // This actual schema has no min_fields or extensibles. Native747-787
        // scans its legacy field positions, starting from the genuine zero.
        let mut maximum = 0;
        for (at, &(field, _)) in THERMOSTAT_ALPHAS.iter().enumerate() {
            if fields.contains_key(field) { maximum = maximum.max(at + 1); }
        }
        if fields.contains_key(CUTOUT_FIELD) { maximum = maximum.max(12); }
        Ok(maximum)
    } else {
        match fields.get("idf_max_fields") {
            Some(V::Number(token)) => token.parse::<usize>()
                .map_err(|_| Error::Unavailable("actual thermostat staged IDF max-fields integer domain")),
            _ => Err(Error::Unavailable("actual thermostat consumed IDF max-fields metadata")),
        }
    }
}

fn uppercase(text: &str) -> Result<String, Error> {
    if !text.is_ascii() {
        return Err(Error::Unavailable("actual thermostat non-ASCII makeUPPER input"));
    }
    Ok(text.to_ascii_uppercase())
}

fn number(token: &str) -> Result<f64, Error> {
    if token.bytes().any(|byte| matches!(byte, b'.' | b'e' | b'E')) {
        token.parse::<f64>().map_err(|_| Error::Unavailable("actual thermostat JSON floating scalar"))
    } else {
        // Native839-845 converts the integer JSON branch through int64 first.
        // In particular integer -0 becomes +0.0; float -0.0 keeps its sign.
        token.parse::<i64>().map(|value| value as f64)
            .map_err(|_| Error::Unavailable("actual thermostat JSON signed integer domain"))
    }
}

fn fill(
    input: &mut SelectedInputProcessor, shortcut: &mut SourceInputProcessorShortCut,
    family: &str, object: &str, source_is_epjson: bool,
) -> Result<(), Error> {
    let alphas: &[(&str, &str)] = if family == THERMOSTAT { &THERMOSTAT_ALPHAS } else { &DUAL_ALPHAS };
    if shortcut.alpha_args.len() < alphas.len()
        || shortcut.alpha_field_names.len() < alphas.len()
        || shortcut.alpha_field_blanks.len() < alphas.len()
        || (family == THERMOSTAT && (shortcut.numeric_args.is_empty()
            || shortcut.numeric_field_names.is_empty() || shortcut.numeric_field_blanks.is_empty()))
    {
        return Err(Error::Unavailable("actual thermostat shared shortcut allocation"));
    }
    shortcut.alpha_args.fill(String::new()); shortcut.numeric_args.fill(0.0);
    shortcut.numeric_field_blanks.fill(true); shortcut.alpha_field_blanks.fill(true);
    shortcut.alpha_field_names.fill(String::new()); shortcut.numeric_field_names.fill(String::new());
    // Actual Native1026-1029 erase precedes filling the shared arguments.
    input.mark_object_as_used(family, object);
    let fields = input.source_raw_object_fields(family, object)
        .map_err(|_| Error::Unavailable("actual thermostat retained raw fields"))?;
    for (at, &(field, label)) in alphas.iter().enumerate() {
        if at == 0 {
            shortcut.alpha_args[at] = uppercase(object)?;
            shortcut.alpha_field_blanks[at] = object.is_empty();
        } else {
            match fields.get(field) {
                None => {},
                Some(V::String(value)) => {
                    shortcut.alpha_args[at] = uppercase(value)?;
                    shortcut.alpha_field_blanks[at] = value.is_empty();
                }
                _ => return Err(Error::Unavailable("actual thermostat alpha scalar conversion type")),
            }
        }
        shortcut.alpha_field_names[at] = if source_is_epjson { field } else { label }.into();
    }
    if family == THERMOSTAT {
        match fields.get(CUTOUT_FIELD) {
            None => {}, // Actual schema default0, blanktrue from shared resets.
            Some(V::String(value)) if value.is_empty() => {},
            Some(V::Number(token)) => {
                shortcut.numeric_args[0] = number(token)?;
                shortcut.numeric_field_blanks[0] = false;
            }
            _ => return Err(Error::Unavailable("actual thermostat numeric scalar conversion type")),
        }
        shortcut.numeric_field_names[0] = if source_is_epjson { CUTOUT_FIELD } else { CUTOUT_LABEL }.into();
    }
    Ok(())
}

pub(super) fn thermostat(
    input: &mut SelectedInputProcessor, shortcut: &mut SourceInputProcessorShortCut,
    number: usize, source_is_epjson: bool,
) -> Result<(i32, i32), Error> {
    let object = object_name(input, THERMOSTAT, number, source_is_epjson)?;
    let maximum = thermostat_max_fields(input.source_raw_object_fields(THERMOSTAT, &object)
        .map_err(|_| Error::Unavailable("actual thermostat max-fields raw association"))?, source_is_epjson)?;
    fill(input, shortcut, THERMOSTAT, &object, source_is_epjson)?;
    // Native name always increments NumAlphas; each remaining alpha/numeric
    // updates its count only inside the actual max-fields extent875-887.
    let num_alphas = i32::try_from(maximum.min(11).max(1))
        .map_err(|_| Error::Unavailable("actual thermostat NumAlphas integer domain"))?;
    let num_numbers = if maximum >= 12 { 1 } else { 0 };
    Ok((num_alphas, num_numbers))
}

pub(super) fn dual(
    input: &mut SelectedInputProcessor, shortcut: &mut SourceInputProcessorShortCut,
    number: usize, source_is_epjson: bool,
) -> Result<(), Error> {
    let object = object_name(input, DUAL, number, source_is_epjson)?;
    fill(input, shortcut, DUAL, &object, source_is_epjson)
}
