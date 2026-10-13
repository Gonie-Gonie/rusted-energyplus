//! Literal selected getObjectItem operations, sharing IP identity/unused state.
use super::super::super::selected_project_data::{ProjectInputObject, ProjectInputValue};
use super::{
    Error, SelectedHeatBalanceInput, SelectedInputProcessor, SourceInputProcessorShortCut,
};

pub(super) const GEOMETRY_FIELDS: [(&str, &str); 5] = [
    ("starting_vertex_position", "Starting Vertex Position"),
    ("vertex_entry_direction", "Vertex Entry Direction"),
    ("coordinate_system", "Coordinate System"),
    (
        "daylighting_reference_point_coordinate_system",
        "Daylighting Reference Point Coordinate System",
    ),
    (
        "rectangular_surface_coordinate_system",
        "Rectangular Surface Coordinate System",
    ),
];
const ALPHAS: [(&str, &str); 9] = [
    ("name", "Name"),
    ("surface_type", "Surface Type"),
    ("construction_name", "Construction Name"),
    ("zone_name", "Zone Name"),
    ("space_name", "Space Name"),
    ("outside_boundary_condition", "Outside Boundary Condition"),
    (
        "outside_boundary_condition_object",
        "Outside Boundary Condition Object",
    ),
    ("sun_exposure", "Sun Exposure"),
    ("wind_exposure", "Wind Exposure"),
];
const NUMBERS: [(&str, &str); 2] = [
    ("view_factor_to_ground", "View Factor to Ground"),
    ("number_of_vertices", "Number of Vertices"),
];
pub(super) const COORDINATES: [(&str, &str); 3] = [
    ("vertex_x_coordinate", "Vertex X-coordinate"),
    ("vertex_y_coordinate", "Vertex Y-coordinate"),
    ("vertex_z_coordinate", "Vertex Z-coordinate"),
];
fn alpha(object: &ProjectInputObject, name: &str, default: &str) -> Result<(String, bool), Error> {
    match object.fields.get(name) {
        None => Ok((default.to_ascii_uppercase(), true)),
        Some(ProjectInputValue::String(value)) if value.is_empty() => {
            Ok((default.to_ascii_uppercase(), true))
        }
        Some(ProjectInputValue::String(value)) => Ok((value.to_ascii_uppercase(), false)),
        Some(ProjectInputValue::Number(_)) => {
            Err(Error::Unavailable("unadmitted surface alpha number"))
        }
    }
}
fn numeric(object: &ProjectInputObject, name: &str) -> Result<(f64, bool), Error> {
    match object.fields.get(name) {
        None => Ok((-99999.0, true)),
        Some(ProjectInputValue::Number(value)) => Ok((*value, false)),
        Some(ProjectInputValue::String(value)) if value.is_empty() => Ok((-99999.0, true)),
        Some(ProjectInputValue::String(value)) if value.eq_ignore_ascii_case("Autocalculate") => {
            Ok((-99999.0, false))
        }
        Some(ProjectInputValue::String(_)) => {
            Err(Error::Unavailable("unadmitted surface numeric string"))
        }
    }
}
fn maximum(
    raw: &SelectedHeatBalanceInput,
    object: &ProjectInputObject,
    names: &[(&str, &str)],
    minimum: usize,
) -> Result<usize, Error> {
    if raw.source_is_epjson {
        Ok(names
            .iter()
            .enumerate()
            .fold(minimum, |count, (i, (name, _))| {
                if object.fields.contains_key(*name) {
                    count.max(i + 1)
                } else {
                    count
                }
            }))
    } else {
        object.idf_max_fields.ok_or(Error::Unavailable(
            "missing physical IDF surface max_fields",
        ))
    }
}
impl SelectedHeatBalanceInput {
    pub(super) fn geometry_arguments(
        &self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<([String; 5], usize), Error> {
        let object = self
            .surface
            .geometry_rules
            .first()
            .ok_or(Error::Unavailable("actual GlobalGeometryRules raw object"))?;
        if !input.has_object("GlobalGeometryRules", &object.object_name) {
            return Err(Error::Unavailable("actual geometry raw/IP identity"));
        }
        if shortcut.alpha_field_names.len() < 5 || shortcut.alpha_field_blanks.len() < 5 {
            return Err(Error::Unavailable(
                "actual shared geometry field-name/blank allocation",
            ));
        }
        let max_fields = maximum(self, object, &GEOMETRY_FIELDS, 0)?;
        // LOCAL GAlphas(5)/GNum(1), not cAlphaArgs/rNumericArgs.
        let mut values = std::array::from_fn(|_| String::new());
        shortcut.alpha_field_names.fill(String::new());
        shortcut.numeric_field_names.fill(String::new());
        shortcut.alpha_field_blanks.fill(true);
        shortcut.numeric_field_blanks.fill(true);
        input.mark_object_as_used("GlobalGeometryRules", &object.object_name);
        let mut num_alphas = 0;
        for (i, &(json, legacy)) in GEOMETRY_FIELDS.iter().enumerate() {
            let default = if i >= 3 { "Relative" } else { "" };
            let (value, blank) = alpha(object, json, default)?;
            values[i] = value;
            shortcut.alpha_field_blanks[i] = blank;
            shortcut.alpha_field_names[i] =
                if self.source_is_epjson { json } else { legacy }.into();
            if i < max_fields {
                num_alphas = i + 1;
            }
        }
        Ok((values, num_alphas))
    }
    pub(super) fn surface_arguments(
        &self,
        index: usize,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(usize, usize), Error> {
        let raw = self
            .surface
            .building_surfaces
            .get(index)
            .ok_or(Error::Unavailable("actual surface raw index"))?;
        let object = &raw.object;
        if !input.has_object("BuildingSurface:Detailed", &object.object_name) {
            return Err(Error::Unavailable("actual surface raw/IP identity"));
        }
        let needed = raw
            .vertices
            .len()
            .checked_mul(3)
            .and_then(|n| n.checked_add(2))
            .ok_or(Error::Unavailable("surface argument count overflow"))?;
        if shortcut.alpha_args.len() < 9
            || shortcut.alpha_field_names.len() < 9
            || shortcut.alpha_field_blanks.len() < 9
            || shortcut.numeric_args.len() < needed
            || shortcut.numeric_field_names.len() < needed
            || shortcut.numeric_field_blanks.len() < needed
        {
            return Err(Error::Unavailable(
                "actual full shared surface argument allocation",
            ));
        }
        let max_fields = if self.source_is_epjson {
            20
        } else {
            object
                .idf_max_fields
                .ok_or(Error::Unavailable("physical surface IDF max_fields"))?
        };
        let max_extensible = if self.source_is_epjson {
            needed - 2
        } else {
            raw.idf_max_extensible_fields.ok_or(Error::Unavailable(
                "physical surface IDF max_extensible_fields",
            ))?
        };
        shortcut.alpha_args.fill(String::new());
        shortcut.numeric_args.fill(0.0);
        shortcut.alpha_field_names.fill(String::new());
        shortcut.numeric_field_names.fill(String::new());
        shortcut.alpha_field_blanks.fill(true);
        shortcut.numeric_field_blanks.fill(true);
        input.mark_object_as_used("BuildingSurface:Detailed", &object.object_name);
        let (mut num_alphas, mut num_numbers) = (0, 0);
        for (i, &(json, legacy)) in ALPHAS.iter().enumerate() {
            let (value, blank) = if i == 0 {
                (
                    object.object_name.to_ascii_uppercase(),
                    object.object_name.is_empty(),
                )
            } else {
                alpha(
                    object,
                    json,
                    match i {
                        7 => "SunExposed",
                        8 => "WindExposed",
                        _ => "",
                    },
                )?
            };
            shortcut.alpha_args[i] = value;
            shortcut.alpha_field_blanks[i] = blank;
            shortcut.alpha_field_names[i] =
                if self.source_is_epjson { json } else { legacy }.into();
            if i == 0 || i < max_fields {
                num_alphas = i + 1;
            }
        }
        for (i, &(json, legacy)) in NUMBERS.iter().enumerate() {
            let (value, blank) = numeric(object, json)?;
            shortcut.numeric_args[i] = value;
            shortcut.numeric_field_blanks[i] = blank;
            shortcut.numeric_field_names[i] =
                if self.source_is_epjson { json } else { legacy }.into();
            if i + 9 < max_fields {
                num_numbers = i + 1;
            }
        }
        for (row, vertex) in raw.vertices.iter().enumerate() {
            for (coordinate, &(json, legacy)) in COORDINATES.iter().enumerate() {
                let extensible = row * 3 + coordinate;
                let numeric = extensible + 2;
                shortcut.numeric_args[numeric] = vertex[coordinate];
                shortcut.numeric_field_blanks[numeric] = false;
                shortcut.numeric_field_names[numeric] =
                    if self.source_is_epjson { json } else { legacy }.into();
                if extensible < max_extensible {
                    num_numbers = numeric + 1;
                }
            }
        }
        Ok((num_alphas, num_numbers))
    }
}
