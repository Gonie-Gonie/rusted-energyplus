//! Project actual ordered constant-schedule getter inputs after sole ordinary admission.
use ep_model::ScheduleUnitType;
use ep_raw_model::{FieldName, RawModel, RawObject, RawValue};
use ep_runtime::heat_balance::selected_schedule_manager::{
    SelectedScheduleInput, SourceConstantScheduleInput, SourceScheduleTypeInput,
};

fn field<'a>(object: &'a RawObject, name: &str) -> Option<&'a RawValue> {
    object.fields.get(&FieldName(name.into()))
}
fn alpha(object: &RawObject, name: &str) -> Result<Option<String>, String> {
    match field(object, name) {
        None | Some(RawValue::Null) => Ok(None),
        Some(RawValue::String(value)) if value.is_empty() => Ok(None),
        Some(RawValue::String(value)) => Ok(Some(value.to_ascii_uppercase())),
        _ => Err(format!("selected schedule alpha unavailable: {name}")),
    }
}
fn number(object: &RawObject, name: &str) -> Result<Option<f64>, String> {
    match field(object, name) {
        None | Some(RawValue::Null) => Ok(None),
        Some(RawValue::String(value)) if value.is_empty() => Ok(None),
        Some(RawValue::Number(value)) => value
            .parse::<f64>()
            .ok()
            .filter(|x| x.is_finite())
            .map(Some)
            .ok_or_else(|| format!("selected schedule number unavailable: {name}")),
        _ => Err(format!("selected schedule number unavailable: {name}")),
    }
}
fn unit(object: &RawObject) -> Result<ScheduleUnitType, String> {
    Ok(match alpha(object, "unit_type")?.as_deref() {
        None => ScheduleUnitType::Invalid,
        Some("DIMENSIONLESS") => ScheduleUnitType::Dimensionless,
        Some("TEMPERATURE") => ScheduleUnitType::Temperature,
        Some("DELTATEMPERATURE") => ScheduleUnitType::DeltaTemperature,
        Some("PRECIPITATIONRATE") => ScheduleUnitType::PrecipitationRate,
        Some("ANGLE") => ScheduleUnitType::Angle,
        Some("CONVECTIONCOEFFICIENT") => ScheduleUnitType::ConvectionCoefficient,
        Some("ACTIVITYLEVEL") => ScheduleUnitType::ActivityLevel,
        Some("VELOCITY") => ScheduleUnitType::Velocity,
        Some("CAPACITY") => ScheduleUnitType::Capacity,
        Some("POWER") => ScheduleUnitType::Power,
        Some("AVAILABILITY") => ScheduleUnitType::Availability,
        Some("PERCENT") => ScheduleUnitType::Percent,
        Some("CONTROL") => ScheduleUnitType::Control,
        Some("MODE") => ScheduleUnitType::Mode,
        Some(value) => return Err(format!("selected schedule unit unavailable: {value}")),
    })
}

pub(crate) fn resolve(raw: &RawModel) -> Result<SelectedScheduleInput, String> {
    // Caller already passed the same raw model through the sole ordinary admission.
    let mut result = SelectedScheduleInput::default();
    for (name, object) in raw
        .ordered_instances("ScheduleTypeLimits")
        .map_err(|e| e.to_string())?
    {
        result.types.push(SourceScheduleTypeInput {
            object_name: name.0.clone(),
            name: name.0.to_ascii_uppercase(),
            lower: number(object, "lower_limit_value")?,
            upper: number(object, "upper_limit_value")?,
            numeric_type: alpha(object, "numeric_type")?,
            unit: unit(object)?,
        });
    }
    for (name, object) in raw
        .ordered_instances("Schedule:Constant")
        .map_err(|e| e.to_string())?
    {
        result.constants.push(SourceConstantScheduleInput {
            object_name: name.0.clone(),
            name: name.0.to_ascii_uppercase(),
            type_name: alpha(object, "schedule_type_limits_name")?,
            value: number(object, "hourly_value")?.unwrap_or(0.0),
        });
    }
    Ok(result)
}
