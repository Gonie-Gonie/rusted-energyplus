use ep_model::{ReportInputObject, ReportInputObjectType as Kind, TypedModel};
use ep_raw_model::{FieldName, ObjectType, RawValue};

use super::{Compiler, field_value};

type AlphaField = (&'static str, Option<&'static str>);
type ReportFamily = (Kind, &'static [AlphaField], u8);

impl Compiler<'_> {
    fn parse_predefined_table_sizing(&mut self, model: &mut TypedModel) {
        const OBJECT: &str = "SimulationControl";
        model.predefined_table_sizing = Default::default();
        let Some((name, object)) = self.single_object(OBJECT) else {
            return;
        };
        let mut alphas: [String; 2] = Default::default();
        let mut alpha_blank = [true; 2];
        let mut default_applied = [false; 2];
        let mut valid = true;
        for (index, field) in ["do_zone_sizing_calculation", "do_system_sizing_calculation"]
            .iter()
            .enumerate()
        {
            let text = match field_value(&object, field) {
                None => "",
                Some(RawValue::String(text)) => text,
                Some(_) => {
                    self.invalid_field_type(OBJECT, &name, field, "string");
                    valid = false;
                    ""
                }
            };
            alpha_blank[index] = text.is_empty();
            let actual = if text.is_empty() {
                self.record_default(OBJECT, &name, field, "No");
                default_applied[index] = true;
                "No"
            } else {
                text
            };
            if super::parse_yes_no(actual).is_none() {
                self.error(
                    "InvalidEnumValue",
                    OBJECT,
                    Some(&name),
                    Some(field),
                    "sizing calculation requires Yes or No".to_owned(),
                );
                valid = false;
            }
            alphas[index] = actual.to_ascii_uppercase();
        }
        if valid {
            model.predefined_table_sizing.first_object =
                Some(ep_model::SimulationControlSizingInput {
                    object_name: name,
                    alphas,
                    alpha_blank,
                    default_applied,
                });
        }
    }

    pub(super) fn parse_report_inputs(&mut self, model: &mut TypedModel) {
        self.parse_predefined_table_sizing(model);
        let families: [ReportFamily; 5] = [
            (
                Kind::SurfacesList,
                &[("report_type", None), ("report_specifications", None)],
                0,
            ),
            (
                Kind::SurfacesDrawing,
                &[
                    ("report_type", None),
                    ("report_specifications_1", Some("Triangulate3DFace")),
                    ("report_specifications_2", None),
                ],
                0,
            ),
            (
                Kind::VariableDictionary,
                &[("key_field", Some("regular")), ("sort_option", None)],
                1,
            ),
            (
                Kind::Constructions,
                &[("details_type_1", None), ("details_type_2", None)],
                0,
            ),
            (
                Kind::EnergyManagementSystem,
                &[
                    ("actuator_availability_dictionary_reporting", Some("None")),
                    (
                        "internal_variable_availability_dictionary_reporting",
                        Some("None"),
                    ),
                    ("ems_runtime_language_debug_output_level", Some("None")),
                ],
                0,
            ),
        ];
        model.report_inputs.legacy_variable_dictionary_sections = match self
            .raw_model
            .objects
            .get(&ObjectType("Report Variable Dictionary".to_string()))
        {
            None => -1,
            Some(objects) => match i32::try_from(objects.len()) {
                Ok(count) => count,
                Err(_) => {
                    self.error(
                        "TooManyReportObjects",
                        "Report Variable Dictionary",
                        None,
                        None,
                        "section count exceeds the source integer owner".to_string(),
                    );
                    0
                }
            },
        };
        for (kind, fields, min_fields) in families {
            let object_type = kind.source_name();
            let is_epjson = self.raw_model.is_epjson();
            for (name, object) in self.objects(object_type) {
                let mut alphas: [String; 3] = Default::default();
                let mut alpha_blank = [true; 3];
                let mut default_applied = [false; 3];
                let mut max_fields = min_fields;
                let mut valid = true;
                for (index, (field, default)) in fields.iter().enumerate() {
                    if object.fields.contains_key(&FieldName((*field).to_string())) {
                        max_fields = max_fields.max([1, 2, 3][index]);
                    }
                    let text = match field_value(&object, field) {
                        None => "",
                        Some(RawValue::String(text)) => text,
                        Some(_) => {
                            self.error(
                                "InvalidFieldType",
                                object_type,
                                Some(&name),
                                Some(field),
                                "report alpha field requires a string".to_string(),
                            );
                            valid = false;
                            ""
                        }
                    };
                    alpha_blank[index] = text.is_empty();
                    let actual = if text.is_empty() {
                        if let Some(default) = default {
                            self.record_default(object_type, &name, field, default);
                            default_applied[index] = true;
                            *default
                        } else {
                            ""
                        }
                    } else {
                        text
                    };
                    alphas[index] = actual.to_ascii_uppercase();
                }
                if !is_epjson {
                    match field_value(&object, "idf_max_fields") {
                        Some(RawValue::Number(value)) => match value.parse::<u8>() {
                            Ok(value) if usize::from(value) <= fields.len() => max_fields = value,
                            _ => {
                                self.error(
                                    "InvalidIdfReportFieldCount",
                                    object_type,
                                    Some(&name),
                                    None,
                                    "actual staged-IDF report field count is not representable"
                                        .to_string(),
                                );
                                valid = false;
                            }
                        },
                        _ => {
                            self.error("MissingIdfReportFieldCount", object_type, Some(&name), None, "validated IDF report order must retain its actual physical field count".to_string());
                            valid = false;
                        }
                    }
                }
                if valid {
                    model.report_inputs.objects.push(ReportInputObject {
                        object_name: name,
                        object_type: kind,
                        alphas,
                        alpha_blank,
                        default_applied,
                        num_alphas: max_fields,
                        is_epjson,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod predefined_sizing_tests {
    use crate::compile_raw_model;
    use ep_raw_model::parse_epjson_str;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn absent_object_retains_literal_false_reset_without_getter_owner() -> TestResult {
        let model = compile_raw_model(&parse_epjson_str("{}")?)
            .model
            .ok_or("compiled model missing")?;
        assert_eq!(model.predefined_table_sizing.actual_flags(), (false, false));
        assert!(model.predefined_table_sizing.first_object.is_none());
        Ok(())
    }

    #[test]
    fn missing_and_empty_fields_retain_genuine_no_defaults() -> TestResult {
        let raw = parse_epjson_str(
            r#"{"SimulationControl":{"control":{"do_system_sizing_calculation":""}}}"#,
        )?;
        let model = compile_raw_model(&raw)
            .model
            .ok_or("compiled model missing")?;
        assert_eq!(model.predefined_table_sizing.actual_flags(), (false, false));
        let input = model
            .predefined_table_sizing
            .first_object
            .ok_or("getter owner missing")?;
        assert_eq!(input.object_name, "control");
        assert_eq!(input.alphas, ["NO", "NO"]);
        assert_eq!(input.alpha_blank, [true, true]);
        assert_eq!(input.default_applied, [true, true]);
        Ok(())
    }

    #[test]
    fn each_actual_yes_flag_is_preserved_independently() -> TestResult {
        for (zone, system, flags) in [
            ("yEs", "No", (true, false)),
            ("No", "YES", (false, true)),
            ("Yes", "Yes", (true, true)),
        ] {
            let text = format!(
                r#"{{"SimulationControl":{{"control":{{"do_zone_sizing_calculation":"{zone}","do_system_sizing_calculation":"{system}"}}}}}}"#
            );
            let model = compile_raw_model(&parse_epjson_str(&text)?)
                .model
                .ok_or("compiled model missing")?;
            assert_eq!(model.predefined_table_sizing.actual_flags(), flags);
            let input = model
                .predefined_table_sizing
                .first_object
                .ok_or("getter owner missing")?;
            assert_eq!(input.alpha_blank, [false, false]);
            assert_eq!(input.default_applied, [false, false]);
        }
        Ok(())
    }

    #[test]
    fn malformed_choice_or_type_does_not_release_a_defaulted_success() -> TestResult {
        for value in [r#""Sometimes""#, "42"] {
            let text = format!(
                r#"{{"SimulationControl":{{"control":{{"do_zone_sizing_calculation":{value}}}}}}}"#
            );
            let compiled = compile_raw_model(&parse_epjson_str(&text)?);
            assert!(compiled.has_errors());
            assert!(compiled.model.is_none());
            assert!(
                compiled
                    .report
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.object_type == "SimulationControl"
                        && diagnostic.field.as_deref() == Some("do_zone_sizing_calculation"))
            );
        }
        Ok(())
    }

    #[test]
    fn multiple_unique_objects_are_not_silently_admitted() -> TestResult {
        let compiled = compile_raw_model(&parse_epjson_str(
            r#"{"SimulationControl":{"a":{},"b":{}}}"#,
        )?);
        assert!(compiled.has_errors());
        assert!(compiled.model.is_none());
        assert!(
            compiled
                .report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "TooManyObjects"
                    && diagnostic.object_type == "SimulationControl")
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile_raw_model;
    use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

    #[test]
    fn actual_a_report_requests_keep_native_default_and_blank_owners()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw = parse_epjson_str(
            r#"{"Output:VariableDictionary":{"dictionary":{"key_field":"IDF"}},"Output:Surfaces:Drawing":{"drawing":{"report_type":"dxf:wireframe"}},"Output:Constructions":{"ctf":{"details_type_1":"Constructions"}}}"#,
        )?;
        let result = compile_raw_model(&raw);
        let model = result.model.ok_or("typed report input unavailable")?;
        let drawing = &model.report_inputs.objects[0];
        assert_eq!(drawing.object_type, Kind::SurfacesDrawing);
        assert_eq!(drawing.alphas, ["DXF:WIREFRAME", "TRIANGULATE3DFACE", ""]);
        assert_eq!(drawing.alpha_blank, [false, true, true]);
        assert_eq!(drawing.default_applied, [false, true, false]);
        assert_eq!(drawing.num_alphas, 1);
        assert_eq!(model.report_inputs.objects[1].alphas[0], "IDF");
        assert_eq!(model.report_inputs.objects[2].alphas[0], "CONSTRUCTIONS");
        Ok(())
    }

    #[test]
    fn idf_unnamed_order_and_explicit_blank_second_alpha_survive()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw = parse_epjson_str_with_idf_order(
            r#"{"Output:Constructions":{"Output:Constructions 1":{"details_type_1":"Materials"},"Output:Constructions 2":{"details_type_1":"Constructions"}}}"#,
            "Output:Constructions, Materials,; Output:Constructions, Constructions;",
        )?;
        let model = compile_raw_model(&raw)
            .model
            .ok_or("typed report input unavailable")?;
        let rows = &model.report_inputs.objects;
        assert_eq!(rows[0].object_name, "Output:Constructions 1");
        assert_eq!(rows[0].num_alphas, 2);
        assert_eq!(rows[1].num_alphas, 1);
        assert!(!rows[0].is_epjson);
        assert_eq!(rows[0].alphas[1], "");
        assert!(rows[0].alpha_blank[1]);

        // Source findMaxFields applies schema min_fields only to epJSON.
        let empty_idf = parse_epjson_str_with_idf_order(
            r#"{"Output:VariableDictionary":{"Output:VariableDictionary 1":{}}}"#,
            "Output:VariableDictionary;",
        )?;
        let empty_idf_model = compile_raw_model(&empty_idf)
            .model
            .ok_or("IDF report unavailable")?;
        assert_eq!(empty_idf_model.report_inputs.objects[0].num_alphas, 0);
        assert_eq!(
            empty_idf_model.report_inputs.objects[0].alphas[0],
            "REGULAR"
        );
        assert!(!empty_idf_model.report_inputs.objects[0].is_epjson);
        let mut empty_json = parse_epjson_str(
            r#"{"Output:VariableDictionary":{"Output:VariableDictionary 1":{}}}"#,
        )?;
        let empty_json_model = compile_raw_model(&empty_json)
            .model
            .ok_or("epJSON report unavailable")?;
        assert_eq!(empty_json_model.report_inputs.objects[0].num_alphas, 1);
        assert!(empty_json_model.report_inputs.objects[0].is_epjson);
        // A real IDF source with no per-family overlay is still IDF; missing
        // physical field metadata keeps the existing explicit compiler stop.
        ep_raw_model::apply_idf_declaration_order(
            &mut empty_json,
            "Output:VariableDictionary;",
            &[],
        )?;
        assert!(!empty_json.has_idf_declaration_order("Output:VariableDictionary"));
        assert!(!empty_json.is_epjson());
        let missing = compile_raw_model(&empty_json);
        assert!(missing.has_errors());
        assert!(
            missing
                .report
                .diagnostics
                .iter()
                .any(|d| d.code == "MissingIdfReportFieldCount")
        );
        Ok(())
    }
}

#[cfg(test)]
mod ctf_report_name_tests {
    use crate::compile_raw_model;
    use ep_raw_model::parse_epjson_str;

    #[test]
    fn actual_selected_loaders_keep_distinct_report_name_rules()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw = parse_epjson_str(
            r#"{
            "Material":{"MiXeD regular":{"roughness":"Smooth","thickness":0.1,"conductivity":0.5,"density":1000.0,"specific_heat":1000.0}},
            "Material:NoMass":{"MiXeD noMass":{"roughness":"Rough","thermal_resistance":0.25}},
            "Material:AirGap":{"MiXeD gap":{"thermal_resistance":0.15}},
            "Construction":{"MiXeD construction":{"outside_layer":"MiXeD regular","layer_2":"MiXeD gap","layer_3":"MiXeD noMass"}}
        }"#,
        )?;
        let model = compile_raw_model(&raw)
            .model
            .ok_or("selected compiled report owners unavailable")?;
        assert_eq!(
            model
                .ctf_report_names
                .materials
                .iter()
                .map(|(_, name)| name.as_str())
                .collect::<Vec<_>>(),
            ["MiXeD regular", "MIXED NOMASS", "MIXED GAP"]
        );
        assert_eq!(
            model
                .ctf_report_names
                .constructions
                .iter()
                .map(|(_, name)| name.as_str())
                .collect::<Vec<_>>(),
            ["MIXED CONSTRUCTION"]
        );
        for (id, name) in &model.ctf_report_names.materials {
            let material = model
                .materials
                .iter()
                .find(|material| material.id == *id)
                .ok_or("original material ID missing")?;
            assert_eq!(material.name, ep_model::NormalizedName::new(name));
        }
        assert_eq!(
            model.constructions[0].id,
            model.ctf_report_names.constructions[0].0
        );
        Ok(())
    }
}
