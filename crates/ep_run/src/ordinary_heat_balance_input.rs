//! Raw operands for the consumed selected HB getters; no second query service.
use ep_raw_model::{FieldName, RawModel, RawValue};
use ep_runtime::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInput;
use ep_runtime::heat_balance::selected_input_processor::SelectedInputProcessor;
use ep_runtime::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
use std::collections::BTreeMap;
mod surface;

pub(crate) fn resolve(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedHeatBalanceInput, String> {
    resolve_ordered(raw, input, false)
}

pub(crate) fn resolve_source(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedHeatBalanceInput, String> {
    resolve_ordered(raw, input, true)
}

fn resolve_ordered(
    raw: &RawModel,
    input: &SelectedInputProcessor,
    source_order: bool,
) -> Result<SelectedHeatBalanceInput, String> {
    // Actual SourceInputFormat custody comes from the loader, not field keys.
    // Existing ordinary admission has already checked this same raw model.
    let mut result = SelectedHeatBalanceInput {
        source_is_epjson: raw.is_epjson(),
        surface: surface::resolve(raw, input, source_order)?,
        objects: BTreeMap::new(),
    };
    for family in [
        "Building",
        "SurfaceConvectionAlgorithm:Inside",
        "SurfaceConvectionAlgorithm:Outside",
        "HeatBalanceAlgorithm",
        "ZoneAirHeatBalanceAlgorithm",
        "Construction",
        "Zone",
        "OtherEquipment",
        "ElectricEquipment",
        "People",
        "Exterior:Lights",
        "Site:Location",
        "RunPeriod",
        "RunPeriodControl:SpecialDays",
        "RunPeriodControl:DaylightSavingTime",
        "SizingPeriod:DesignDay",
        "Output:Meter",
        "Output:Meter:MeterFileOnly",
        "Output:Meter:Cumulative",
        "Output:Meter:Cumulative:MeterFileOnly",
        "Site:GroundTemperature:BuildingSurface",
        "Site:GroundTemperature:FCfactorMethod",
        "Site:GroundTemperature:Shallow",
        "Site:GroundTemperature:Deep",
    ] {
        // No new HB map key/state on legacy or Source B's unselected family.
        // Positive raw statements include genuine zero-member Source A inputs.
        if family == "ElectricEquipment"
            && (!source_order || input.get_num_objects_found(family) == 0)
        {
            continue;
        }
        let ordered = if source_order {
            raw.source_ordered_instances(family)?
        } else {
            raw.ordered_instances(family).map_err(|e| e.to_string())?
        };
        if input.get_num_objects_found(family) != ordered.len() {
            return Err(format!("HB raw/IP count: {family}"));
        }
        let mut projected = Vec::new();
        for (name, object) in ordered {
            if !input.has_object(family, &name.0) {
                return Err(format!("HB raw/IP identity: {family}/{}", name.0));
            }
            let idf_max_fields = object
                .fields
                .get(&FieldName("idf_max_fields".into()))
                .map(|value| {
                    let RawValue::Number(text) = value else {
                        return Err("HB idf_max_fields is not an actual integer".to_string());
                    };
                    text.parse::<usize>()
                        .map_err(|_| "HB idf_max_fields integer domain".to_string())
                })
                .transpose()?;
            if !result.source_is_epjson && idf_max_fields.is_none() {
                return Err(format!(
                    "HB requires real staged IDF max_fields for {family}"
                ));
            }
            let mut fields = BTreeMap::new();
            for (field, value) in &object.fields {
                if matches!(
                    field.0.as_str(),
                    "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
                ) {
                    continue;
                }
                // Native IP walks only genuine OtherEquipment schema fields.
                // Unknown instance metadata remains in the original raw/IP
                // owners, rather than becoming a heat-balance operand.
                if family == "People"
                    && !matches!(
                        field.0.as_str(),
                        "zone_or_zonelist_or_space_or_spacelist_name"
                            | "number_of_people_schedule_name"
                            | "number_of_people_calculation_method"
                            | "number_of_people"
                            | "people_per_floor_area"
                            | "floor_area_per_person"
                            | "fraction_radiant"
                            | "sensible_heat_fraction"
                            | "activity_level_schedule_name"
                            | "carbon_dioxide_generation_rate"
                            | "enable_ashrae_55_comfort_warnings"
                            | "mean_radiant_temperature_calculation_type"
                            | "surface_name_angle_factor_list_name"
                            | "work_efficiency_schedule_name"
                            | "clothing_insulation_calculation_method"
                            | "clothing_insulation_calculation_method_schedule_name"
                            | "clothing_insulation_schedule_name"
                            | "air_velocity_schedule_name"
                            | "thermal_comfort_model_1_type"
                            | "thermal_comfort_model_2_type"
                            | "thermal_comfort_model_3_type"
                            | "thermal_comfort_model_4_type"
                            | "thermal_comfort_model_5_type"
                            | "thermal_comfort_model_6_type"
                            | "thermal_comfort_model_7_type"
                            | "ankle_level_air_velocity_schedule_name"
                            | "cold_stress_temperature_threshold"
                            | "heat_stress_temperature_threshold"
                    )
                {
                    continue;
                }
                if family == "ElectricEquipment"
                    && !matches!(
                        field.0.as_str(),
                        "zone_or_zonelist_or_space_or_spacelist_name"
                            | "schedule_name"
                            | "design_level_calculation_method"
                            | "design_level"
                            | "watts_per_floor_area"
                            | "watts_per_person"
                            | "fraction_latent"
                            | "fraction_radiant"
                            | "fraction_lost"
                            | "end_use_subcategory"
                    )
                {
                    continue;
                }
                if family == "OtherEquipment"
                    && !matches!(
                        field.0.as_str(),
                        "fuel_type"
                            | "zone_or_zonelist_or_space_or_spacelist_name"
                            | "schedule_name"
                            | "design_level_calculation_method"
                            | "design_level"
                            | "power_per_floor_area"
                            | "power_per_person"
                            | "fraction_latent"
                            | "fraction_radiant"
                            | "fraction_lost"
                            | "carbon_dioxide_generation_rate"
                            | "end_use_subcategory"
                    )
                {
                    continue;
                }
                let scalar = match value {
                    RawValue::String(value) => ProjectInputValue::String(value.clone()),
                    RawValue::Number(value) => {
                        let value = if matches!(family, "OtherEquipment" | "People")
                            || (source_order && family == "ElectricEquipment")
                        {
                            ep_model::classify_source_json_number(value)
                                .map(ep_model::SourceJsonNumber::as_f64)
                                .map_err(|_| format!("HB numeric parse: {family}.{}", field.0))?
                        } else {
                            value
                                .parse::<f64>()
                                .map_err(|_| format!("HB numeric parse: {family}.{}", field.0))?
                        };
                        if !value.is_finite() {
                            return Err(format!("HB nonfinite JSON: {family}.{}", field.0));
                        }
                        ProjectInputValue::Number(value)
                    }
                    _ => {
                        return Err(format!(
                            "HB actual JSON representation: {family}.{}",
                            field.0
                        ));
                    }
                };
                fields.insert(field.0.clone(), scalar);
            }
            projected.push(ProjectInputObject {
                object_name: name.0.clone(),
                fields,
                idf_max_fields,
            });
        }
        result.objects.insert(family.into(), projected);
    }
    Ok(result)
}
