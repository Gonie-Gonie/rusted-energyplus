//! Zone's real shared-array getObjectItem caller; no second query/schema owner.
use super::super::super::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use super::super::super::selected_project_data::ProjectInputValue;
use super::super::{HeatBalanceInputError, SelectedHeatBalanceInput};

const FIELDS: [(&str, &str, bool, f64); 13] = [
    ("name", "Name", true, 0.0),
    (
        "direction_of_relative_north",
        "Direction of Relative North",
        false,
        0.0,
    ),
    ("x_origin", "X Origin", false, 0.0),
    ("y_origin", "Y Origin", false, 0.0),
    ("z_origin", "Z Origin", false, 0.0),
    ("type", "Type", false, 1.0),
    ("multiplier", "Multiplier", false, 1.0),
    ("ceiling_height", "Ceiling Height", false, -99999.0),
    ("volume", "Volume", false, -99999.0),
    ("floor_area", "Floor Area", false, -99999.0),
    (
        "zone_inside_convection_algorithm",
        "Zone Inside Convection Algorithm",
        true,
        0.0,
    ),
    (
        "zone_outside_convection_algorithm",
        "Zone Outside Convection Algorithm",
        true,
        0.0,
    ),
    (
        "part_of_total_floor_area",
        "Part of Total Floor Area",
        true,
        0.0,
    ),
];

impl SelectedHeatBalanceInput {
    pub(super) fn get_zone_object_item(
        &self,
        index: usize,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(usize, usize), HeatBalanceInputError> {
        let objects = self
            .objects
            .get("Zone")
            .ok_or(HeatBalanceInputError::Unavailable(
                "missing genuine Zone raw values",
            ))?;
        if input.get_num_objects_found("Zone") != objects.len() {
            return Err(HeatBalanceInputError::Unavailable("Zone raw/IP count"));
        }
        let object = objects
            .get(index)
            .ok_or(HeatBalanceInputError::Unavailable(
                "Zone getObjectItem index",
            ))?;
        if !input.has_object("Zone", &object.object_name) {
            return Err(HeatBalanceInputError::Unavailable("Zone raw/IP identity"));
        }
        if shortcut.alpha_args.len() < 4
            || shortcut.numeric_args.len() < 9
            || shortcut.alpha_field_names.len() < 4
            || shortcut.alpha_field_blanks.len() < 4
            || shortcut.numeric_field_names.len() < 9
            || shortcut.numeric_field_blanks.len() < 9
        {
            return Err(HeatBalanceInputError::Unavailable(
                "sole actual shared Zone argument allocation",
            ));
        }
        let max_fields = if self.source_is_epjson {
            FIELDS.iter().enumerate().fold(0, |n, (i, field)| {
                if object.fields.contains_key(field.0) {
                    n.max(i + 1)
                } else {
                    n
                }
            })
        } else {
            object
                .idf_max_fields
                .ok_or(HeatBalanceInputError::Unavailable(
                    "missing physical IDF Zone field count",
                ))?
        };
        shortcut.alpha_args.fill(String::new());
        shortcut.numeric_args.fill(0.0);
        shortcut.alpha_field_names.fill(String::new());
        shortcut.numeric_field_names.fill(String::new());
        shortcut.alpha_field_blanks.fill(true);
        shortcut.numeric_field_blanks.fill(true);
        input.mark_object_as_used("Zone", &object.object_name);
        let (mut alpha, mut numeric, mut num_alpha, mut num_numeric) = (0, 0, 0, 0);
        for (position, &(json, legacy, is_alpha, default)) in FIELDS.iter().enumerate() {
            let label = if self.source_is_epjson { json } else { legacy };
            if is_alpha {
                let default = if json == "part_of_total_floor_area" {
                    "Yes"
                } else {
                    ""
                };
                let (value, blank) = if json == "name" {
                    (object.object_name.as_str(), false)
                } else {
                    match object.fields.get(json) {
                        None => (default, true),
                        Some(ProjectInputValue::String(value)) if value.is_empty() => {
                            (default, true)
                        }
                        Some(ProjectInputValue::String(value)) => (value.as_str(), false),
                        _ => {
                            return Err(HeatBalanceInputError::Unavailable(
                                "Zone alpha JSON representation",
                            ));
                        }
                    }
                };
                shortcut.alpha_args[alpha] = value.to_ascii_uppercase();
                shortcut.alpha_field_names[alpha] = label.into();
                shortcut.alpha_field_blanks[alpha] = blank;
                alpha += 1;
                if json == "name" || position < max_fields {
                    num_alpha = alpha;
                }
            } else {
                let (value, blank) = match object.fields.get(json) {
                    None => (default, true),
                    Some(ProjectInputValue::String(value)) if value.is_empty() => (default, true),
                    Some(ProjectInputValue::Number(value)) => (*value, false),
                    Some(ProjectInputValue::String(value))
                        if (7..=9).contains(&position)
                            && value.eq_ignore_ascii_case("Autocalculate") =>
                    {
                        (-99999.0, false)
                    }
                    _ => {
                        return Err(HeatBalanceInputError::Unavailable(
                            "Zone numeric JSON representation",
                        ));
                    }
                };
                shortcut.numeric_args[numeric] = value;
                shortcut.numeric_field_names[numeric] = label.into();
                shortcut.numeric_field_blanks[numeric] = blank;
                numeric += 1;
                if position < max_fields {
                    num_numeric = numeric;
                }
            }
        }
        Ok((num_alpha, num_numeric))
    }
}
