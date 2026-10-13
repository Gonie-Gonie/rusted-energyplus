use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};

use crate::{FieldName, RawModel, RawValue, SourceInputFormat};

/// One IDF object type whose declaration order must survive epJSON conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdfOrderTarget {
    /// Canonical EnergyPlus object type.
    pub object_type: &'static str,
    /// Zero-based data-field index containing the required object name.
    /// Source-defined unnamed report/project families use IdfParser-generated names.
    pub name_field_index: usize,
}

/// IDF declaration-order targets backed by current source and oracle evidence.
///
/// This default is deliberately limited to source-order-sensitive families with
/// explicit identity and converter edge-case evidence. Other position-sensitive
/// families require the same evidence before joining the fail-closed overlay.
pub const IDF_ORDER_TARGETS: &[IdfOrderTarget] = &[
    // Same four actual HB getter targets from the frozen adjacent packet.
    IdfOrderTarget {
        object_type: "Building",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "SurfaceConvectionAlgorithm:Inside",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "SurfaceConvectionAlgorithm:Outside",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "ZoneAirHeatBalanceAlgorithm",
        name_field_index: 0,
    },
    // Material.cc regular direct JSON and NoMass getter preserve IDF order.
    IdfOrderTarget {
        object_type: "Material",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Material:NoMass",
        name_field_index: 0,
    },
    // SimulationManager::GetProjectData uses real IDF max-field counts.
    IdfOrderTarget {
        object_type: "Version",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "HeatBalanceAlgorithm",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Timestep",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "SimulationControl",
        name_field_index: 0,
    },
    // OutputProcessor's actual getter requires the same unnamed-object order.
    IdfOrderTarget {
        object_type: "Output:Variable",
        name_field_index: 0,
    },
    // IdfParser.cc277-301 generates these unnamed keys from the per-type count.
    IdfOrderTarget {
        object_type: "Output:Surfaces:List",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Surfaces:Drawing",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:VariableDictionary",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Constructions",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:EnergyManagementSystem",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "RunPeriodControl:SpecialDays",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:FfactorGroundFloor",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:CfactorUndergroundWall",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:ComplexFenestrationState",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:WindowEquivalentLayer",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:WindowDataFile",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Zone",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "ZoneList",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "ZoneGroup",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "ZoneProperty:LocalEnvironment",
        name_field_index: 0,
    },
    // WeatherManager::GetDesignDayData calls getObjectItem in input order;
    // ReverseDD remaps the first two owners only after this effective ordering.
    IdfOrderTarget {
        object_type: "SizingPeriod:DesignDay",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "GlobalGeometryRules",
        name_field_index: 0,
    },
    // SurfaceGeometry::GetSurfaceData preserves IDF declaration order within
    // its Wall/Floor/Roof groups. Native epJSON keeps its own map order.
    IdfOrderTarget {
        object_type: "BuildingSurface:Detailed",
        name_field_index: 0,
    },
    // ScheduleManager uses getObjectItem's preserved IDF order within these
    // families. Cross-family registration order remains the compiler's owner.
    IdfOrderTarget {
        object_type: "ScheduleTypeLimits",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Constant",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Compact",
        name_field_index: 0,
    },
];

/// Error returned when staged IDF declaration order cannot be recovered safely.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdfOrderError {
    message: String,
}

impl IdfOrderError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for IdfOrderError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for IdfOrderError {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ScannedObject {
    idf_order: u32,
    max_fields: usize,
    name: String,
    line: usize,
    column: usize,
}

/// Applies declaration order recovered from the staged IDF to a converted RawModel.
///
/// The operation is strict and transactional: every configured target instance must
/// reconcile one-to-one with the converted epJSON map before any overlay is installed.
pub fn apply_idf_declaration_order(
    model: &mut RawModel,
    idf: &str,
    targets: &[IdfOrderTarget],
) -> Result<(), IdfOrderError> {
    validate_targets(targets)?;
    let scanned = scan_target_objects(idf, targets)?;
    let mut overlay = BTreeMap::new();
    let mut report_field_counts = Vec::new();

    for target in targets {
        let idf_objects = scanned.get(target.object_type).ok_or_else(|| {
            IdfOrderError::new(format!(
                "internal IDF order recovery error: missing scan bucket for {}",
                target.object_type
            ))
        })?;
        for (index, idf_object) in idf_objects.iter().enumerate() {
            if idf_objects[..index]
                .iter()
                .any(|earlier| earlier.name.eq_ignore_ascii_case(&idf_object.name))
            {
                return Err(IdfOrderError::new(format!(
                    "duplicate staged IDF {} name '{}' at line {}, column {}",
                    target.object_type, idf_object.name, idf_object.line, idf_object.column
                )));
            }
        }
        let matching_types = model
            .objects
            .keys()
            .filter(|object_type| object_type.0.eq_ignore_ascii_case(target.object_type))
            .collect::<Vec<_>>();

        if matching_types.len() > 1 {
            return Err(IdfOrderError::new(format!(
                "IDF declaration-order recovery for {} is ambiguous: converted epJSON contains multiple case-insensitive object-type matches",
                target.object_type
            )));
        }

        let Some(actual_type) = matching_types.first().copied() else {
            if idf_objects.is_empty() {
                continue;
            }
            return Err(IdfOrderError::new(format!(
                "IDF declaration-order recovery mismatch for {}: staged IDF has {} object(s), converted epJSON has none",
                target.object_type,
                idf_objects.len()
            )));
        };
        let instances = model.objects.get(actual_type).ok_or_else(|| {
            IdfOrderError::new(format!(
                "internal IDF order recovery error: converted epJSON type {} disappeared",
                actual_type.0
            ))
        })?;

        let actual_names = instances.keys().collect::<Vec<_>>();
        for (index, actual_name) in actual_names.iter().enumerate() {
            if actual_names[..index]
                .iter()
                .any(|earlier| earlier.0.eq_ignore_ascii_case(&actual_name.0))
            {
                return Err(IdfOrderError::new(format!(
                    "converted epJSON contains duplicate case-insensitive {} name '{}'",
                    actual_type.0, actual_name.0
                )));
            }
        }

        if idf_objects.len() != instances.len() {
            return Err(IdfOrderError::new(format!(
                "IDF declaration-order recovery count mismatch for {}: staged IDF has {}, converted epJSON has {}",
                actual_type.0,
                idf_objects.len(),
                instances.len()
            )));
        }
        if instances.is_empty() {
            continue;
        }

        let mut seen_actual_names = BTreeSet::new();
        let mut ordered_names = Vec::with_capacity(idf_objects.len());
        for idf_object in idf_objects {
            let matching_names = instances
                .keys()
                .filter(|name| name.0.eq_ignore_ascii_case(&idf_object.name))
                .collect::<Vec<_>>();
            if matching_names.is_empty() {
                return Err(IdfOrderError::new(format!(
                    "IDF declaration-order recovery name mismatch for {}: staged IDF name '{}' (idf_order {}) is absent from converted epJSON",
                    actual_type.0, idf_object.name, idf_object.idf_order
                )));
            }
            if matching_names.len() > 1 {
                return Err(IdfOrderError::new(format!(
                    "IDF declaration-order recovery for {}/{} is ambiguous: converted epJSON contains multiple case-insensitive name matches",
                    actual_type.0, idf_object.name
                )));
            }

            let actual_name = matching_names[0].clone();
            if !seen_actual_names.insert(actual_name.clone()) {
                return Err(IdfOrderError::new(format!(
                    "IDF declaration-order recovery mapped more than one staged object to {}/{}",
                    actual_type.0, actual_name.0
                )));
            }
            if unnamed_legacy_type(target.object_type)
                || matches!(
                    target.object_type,
                    "Material:NoMass"
                        | "Building"
                        | "Construction"
                        | "Zone"
                        | "BuildingSurface:Detailed"
                )
            {
                report_field_counts.push((
                    actual_type.clone(),
                    actual_name.clone(),
                    if target.object_type == "BuildingSurface:Detailed" {
                        idf_object.max_fields.max(20)
                    } else {
                        idf_object.max_fields
                    },
                    (target.object_type == "BuildingSurface:Detailed")
                        .then_some(idf_object.max_fields.saturating_sub(11)),
                ));
            }
            ordered_names.push(actual_name);
        }

        overlay.insert(actual_type.clone(), ordered_names);
    }

    // Apply only after every type/name/count has been reconciled successfully.
    for (object_type, name, max_fields, max_extensible_fields) in report_field_counts {
        let object = model
            .objects
            .get_mut(&object_type)
            .and_then(|objects| objects.get_mut(&name))
            .ok_or_else(|| IdfOrderError::new("reconciled report input disappeared"))?;
        object.fields.insert(
            FieldName("idf_max_fields".to_string()),
            RawValue::Number(max_fields.to_string()),
        );
        if let Some(count) = max_extensible_fields {
            object.fields.insert(
                FieldName("idf_max_extensible_fields".into()),
                RawValue::Number(count.to_string()),
            );
        }
    }
    model.idf_declaration_order = overlay;
    model.source_input_format = SourceInputFormat::Idf;
    Ok(())
}

pub(crate) fn apply_idf_declaration_order_bytes(
    model: &mut RawModel,
    idf: &[u8],
    targets: &[IdfOrderTarget],
) -> Result<(), IdfOrderError> {
    // EnergyPlus token delimiters are ASCII and comments may contain legacy bytes.
    // Lossy decoding therefore keeps token boundaries while reconciliation still
    // fails closed if replacement bytes occur in a targeted object name.
    apply_idf_declaration_order(model, &String::from_utf8_lossy(idf), targets)
}

fn validate_targets(targets: &[IdfOrderTarget]) -> Result<(), IdfOrderError> {
    for (index, target) in targets.iter().enumerate() {
        if target.object_type.trim().is_empty() {
            return Err(IdfOrderError::new(
                "IDF declaration-order target object type must not be blank",
            ));
        }
        if targets[..index]
            .iter()
            .any(|earlier| earlier.object_type.eq_ignore_ascii_case(target.object_type))
        {
            return Err(IdfOrderError::new(format!(
                "duplicate IDF declaration-order target {}",
                target.object_type
            )));
        }
    }
    Ok(())
}

fn scan_target_objects(
    idf: &str,
    targets: &[IdfOrderTarget],
) -> Result<BTreeMap<&'static str, Vec<ScannedObject>>, IdfOrderError> {
    let idf = idf.trim_start_matches('\u{feff}');
    let mut scanned = targets
        .iter()
        .map(|target| (target.object_type, Vec::new()))
        .collect::<BTreeMap<_, _>>();
    let mut fields = Vec::<String>::new();
    let mut field = String::new();
    let mut parsed_field_segment = None::<String>;
    let mut object_start = None::<(usize, usize)>;
    let mut in_comment = false;
    let mut idf_order = 0_u32;
    let mut line = 1_usize;
    let mut column = 1_usize;

    for character in idf.chars() {
        let character_line = line;
        let character_column = column;

        if in_comment {
            if character == '\n' {
                in_comment = false;
            }
            advance_position(character, &mut line, &mut column);
            continue;
        }

        match character {
            '!' => {
                // IdfParser treats a comment as a value-token boundary. If text
                // resumes before the comma, its last nonblank segment wins.
                let parsed_segment = normalize_field(&field);
                if !parsed_segment.is_empty() {
                    parsed_field_segment = Some(parsed_segment);
                }
                field.clear();
                in_comment = true;
            }
            ',' => {
                if object_start.is_none() && fields.is_empty() && normalized_field_is_empty(&field)
                {
                    return Err(IdfOrderError::new(format!(
                        "malformed IDF at line {character_line}, column {character_column}: unexpected comma before an object type"
                    )));
                }
                fields.push(finish_field(&mut field, &mut parsed_field_segment));
            }
            ';' => {
                if object_start.is_some()
                    || !fields.is_empty()
                    || !normalized_field_is_empty(&field)
                {
                    fields.push(finish_field(&mut field, &mut parsed_field_segment));
                    idf_order = idf_order.checked_add(1).ok_or_else(|| {
                        IdfOrderError::new("IDF declaration-order counter exceeded u32")
                    })?;
                    record_object(
                        &fields,
                        object_start.unwrap_or((character_line, character_column)),
                        idf_order,
                        targets,
                        &mut scanned,
                    )?;
                    fields.clear();
                    object_start = None;
                    parsed_field_segment = None;
                }
            }
            _ => {
                if object_start.is_none() && !is_idf_leading_whitespace(character) {
                    object_start = Some((character_line, character_column));
                }
                field.push(character);
            }
        }

        advance_position(character, &mut line, &mut column);
    }

    if object_start.is_some() || !fields.is_empty() || !normalized_field_is_empty(&field) {
        let (start_line, start_column) = object_start.unwrap_or((line, column));
        fields.push(finish_field(&mut field, &mut parsed_field_segment));
        idf_order = idf_order
            .checked_add(1)
            .ok_or_else(|| IdfOrderError::new("IDF declaration-order counter exceeded u32"))?;
        record_object(
            &fields,
            (start_line, start_column),
            idf_order,
            targets,
            &mut scanned,
        )?;
    }

    Ok(scanned)
}

fn unnamed_legacy_type(object_type: &str) -> bool {
    matches!(
        object_type,
        "GlobalGeometryRules"
            | "Version"
            | "HeatBalanceAlgorithm"
            | "Timestep"
            | "SimulationControl"
            | "SurfaceConvectionAlgorithm:Inside"
            | "SurfaceConvectionAlgorithm:Outside"
            | "ZoneAirHeatBalanceAlgorithm"
            | "Output:Surfaces:List"
            | "Output:Surfaces:Drawing"
            | "Output:VariableDictionary"
            | "Output:Constructions"
            | "Output:EnergyManagementSystem"
            | "Output:Variable"
    )
}

fn record_object(
    fields: &[String],
    (line, column): (usize, usize),
    idf_order: u32,
    targets: &[IdfOrderTarget],
    scanned: &mut BTreeMap<&'static str, Vec<ScannedObject>>,
) -> Result<(), IdfOrderError> {
    let Some(object_type) = fields.first() else {
        return Err(IdfOrderError::new(format!(
            "malformed IDF object at line {line}, column {column}: missing object type"
        )));
    };
    if object_type.is_empty() {
        return Err(IdfOrderError::new(format!(
            "malformed IDF object at line {line}, column {column}: blank object type"
        )));
    }

    let Some(target) = targets
        .iter()
        .find(|target| object_type.eq_ignore_ascii_case(target.object_type))
    else {
        return Ok(());
    };
    if unnamed_legacy_type(target.object_type) {
        let bucket = scanned.get_mut(target.object_type).ok_or_else(|| {
            IdfOrderError::new(format!(
                "internal IDF order recovery error: missing target bucket for {}",
                target.object_type
            ))
        })?;
        bucket.push(ScannedObject {
            idf_order,
            max_fields: fields.len() - 1,
            name: format!("{} {}", target.object_type, bucket.len() + 1),
            line,
            column,
        });
        return Ok(());
    }
    let field_index = target.name_field_index.checked_add(1).ok_or_else(|| {
        IdfOrderError::new(format!(
            "IDF declaration-order target {} has an invalid name-field index",
            target.object_type
        ))
    })?;
    let Some(name) = fields.get(field_index) else {
        return Err(IdfOrderError::new(format!(
            "malformed staged IDF {} object at line {line}, column {column}: missing required name field {}",
            target.object_type, target.name_field_index
        )));
    };
    if name.is_empty() {
        return Err(IdfOrderError::new(format!(
            "malformed staged IDF {} object at line {line}, column {column}: required name is blank",
            target.object_type
        )));
    }

    let bucket = scanned.get_mut(target.object_type).ok_or_else(|| {
        IdfOrderError::new(format!(
            "internal IDF order recovery error: missing target bucket for {}",
            target.object_type
        ))
    })?;
    bucket.push(ScannedObject {
        idf_order,
        max_fields: fields.len() - 1,
        name: name.clone(),
        line,
        column,
    });
    Ok(())
}

fn normalize_field(field: &str) -> String {
    field
        .trim_start_matches(is_idf_leading_whitespace)
        .trim_end_matches(is_idf_trailing_whitespace)
        .to_string()
}

fn normalized_field_is_empty(field: &str) -> bool {
    normalize_field(field).is_empty()
}

fn finish_field(field: &mut String, parsed_segment: &mut Option<String>) -> String {
    let current_segment = normalize_field(field);
    field.clear();
    if current_segment.is_empty() {
        parsed_segment.take().unwrap_or(current_segment)
    } else {
        *parsed_segment = None;
        current_segment
    }
}

fn is_idf_leading_whitespace(character: char) -> bool {
    matches!(character, ' ' | '\r' | '\t' | '\n')
}

fn is_idf_trailing_whitespace(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\0')
}

fn advance_position(character: char, line: &mut usize, column: &mut usize) {
    if character == '\n' {
        *line += 1;
        *column = 1;
    } else {
        *column += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        IDF_ORDER_TARGETS, IdfOrderError, apply_idf_declaration_order,
        apply_idf_declaration_order_bytes,
    };
    use crate::{FieldName, RawValue, parse_epjson_str, parse_epjson_str_with_idf_order};

    const TWO_SPECIAL_DAYS_EPJSON: &str = r#"{
        "RunPeriodControl:SpecialDays": {
            "Zulu Earlier Holiday": {
                "start_date": "6/15",
                "duration": 1,
                "special_day_type": "Holiday"
            },
            "Alpha Later Custom": {
                "start_date": "6/15",
                "duration": 1,
                "special_day_type": "CustomDay2"
            }
        }
    }"#;

    const REVERSE_ALPHABET_IDF: &str = r#"
        RunPeriodControl:SpecialDays,
          Zulu Earlier Holiday,
          6/15,
          1,
          Holiday;
        RunPeriodControl:SpecialDays,
          Alpha Later Custom,
          6/15,
          1,
          CustomDay2;
    "#;

    const SELECTED_SCHEDULE_EPJSON: &str = r#"{
        "SizingPeriod:DesignDay": {
            "A Design": {"month": 7},
            "Z Design": {"month": 12}
        },
        "ScheduleTypeLimits": {
            "A Type": {"lower_limit_value": 0, "upper_limit_value": 1},
            "Z Type": {"lower_limit_value": 0, "upper_limit_value": 1}
        },
        "Schedule:Constant": {
            "A Constant": {"schedule_type_limits_name": "A Type", "hourly_value": 0},
            "Z Constant": {"schedule_type_limits_name": "Z Type", "hourly_value": 1}
        },
        "Schedule:Compact": {
            "A Compact": {"schedule_type_limits_name": "A Type"},
            "Z Compact": {"schedule_type_limits_name": "Z Type"}
        }
    }"#;

    const SELECTED_SCHEDULE_IDF: &str = r#"
        SizingPeriod:DesignDay,z design,12;
        SizingPeriod:DesignDay,a design,7;
        Schedule:Constant,z constant,Z Type,1;
        ScheduleTypeLimits,z type,0,1,Continuous,Dimensionless;
        Schedule:Compact,z compact,Z Type,Through:12/31,For:AllDays,Until:24:00,1;
        Schedule:Compact,a compact,A Type,Through:12/31,For:AllDays,Until:24:00,0;
        ScheduleTypeLimits,a type,0,1,Continuous,Dimensionless;
        Schedule:Constant,a constant,A Type,0;
    "#;

    fn ordered_names(model: &crate::RawModel) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(model
            .ordered_instances("RunPeriodControl:SpecialDays")?
            .into_iter()
            .map(|(name, _object)| name.0.clone())
            .collect())
    }

    fn required_order_error(
        result: Result<(), IdfOrderError>,
        message: &str,
    ) -> Result<IdfOrderError, Box<dyn std::error::Error>> {
        match result {
            Ok(()) => Err(std::io::Error::other(message).into()),
            Err(error) => Ok(error),
        }
    }

    #[test]
    fn native_epjson_retains_canonical_name_order() -> Result<(), Box<dyn std::error::Error>> {
        let model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;

        assert!(!model.has_idf_declaration_order("RunPeriodControl:SpecialDays"));
        assert_eq!(
            ordered_names(&model)?,
            vec!["Alpha Later Custom", "Zulu Earlier Holiday"]
        );
        Ok(())
    }

    #[test]
    fn selected_schedule_families_retain_idf_order_without_changing_direct_epjson()
    -> Result<(), Box<dyn std::error::Error>> {
        let staged =
            parse_epjson_str_with_idf_order(SELECTED_SCHEDULE_EPJSON, SELECTED_SCHEDULE_IDF)?;
        let direct = parse_epjson_str(SELECTED_SCHEDULE_EPJSON)?;

        for (object_type, earlier, later) in [
            ("SizingPeriod:DesignDay", "Z Design", "A Design"),
            ("ScheduleTypeLimits", "Z Type", "A Type"),
            ("Schedule:Constant", "Z Constant", "A Constant"),
            ("Schedule:Compact", "Z Compact", "A Compact"),
        ] {
            let names = |model: &crate::RawModel| -> Result<Vec<String>, IdfOrderError> {
                Ok(model
                    .ordered_instances(object_type)?
                    .into_iter()
                    .map(|(name, _)| name.0.clone())
                    .collect())
            };
            assert_eq!(names(&staged)?, vec![earlier, later]);
            assert_eq!(names(&direct)?, vec![later, earlier]);
            assert!(staged.has_idf_declaration_order(object_type));
            assert!(!direct.has_idf_declaration_order(object_type));
        }
        assert_eq!(staged.objects, direct.objects);
        Ok(())
    }

    #[test]
    fn selected_schedule_overlay_rejects_duplicate_and_unmatched_names_transactionally()
    -> Result<(), Box<dyn std::error::Error>> {
        let original = parse_epjson_str(SELECTED_SCHEDULE_EPJSON)?;

        for later in ["a type", "a constant", "a compact"] {
            let earlier = later.replacen('a', "z", 1);
            for (replacement, expected_error) in [
                (earlier.to_uppercase(), "duplicate staged IDF"),
                (format!("Missing {later}"), "name mismatch"),
            ] {
                let mut model = original.clone();
                let idf = SELECTED_SCHEDULE_IDF.replace(later, &replacement);
                let error = required_order_error(
                    apply_idf_declaration_order(&mut model, &idf, IDF_ORDER_TARGETS),
                    "ambiguous or unmatched schedule names must fail",
                )?;
                assert!(error.to_string().contains(expected_error));
                assert_eq!(model, original);
            }
        }
        Ok(())
    }

    #[test]
    fn surface_declaration_order_survives_conversion_without_changing_native_epjson()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{"BuildingSurface:Detailed": {
            "Zulu Wall": {"surface_type":"Wall"},
            "Alpha Floor": {"surface_type":"Floor"},
            "Beta Roof": {"surface_type":"Roof"}
        }}"#;
        let idf = "BuildingSurface:Detailed,ZULU WALL,Wall;\n\
                   BuildingSurface:Detailed,beta roof,Roof;\n\
                   BuildingSurface:Detailed,Alpha Floor,Floor;";
        let staged = parse_epjson_str_with_idf_order(epjson, idf)?;
        let native = parse_epjson_str(epjson)?;
        let names = |model: &crate::RawModel| -> Result<Vec<String>, crate::EpJsonError> {
            Ok(model
                .ordered_instances("BuildingSurface:Detailed")?
                .into_iter()
                .map(|(name, _)| name.0.clone())
                .collect())
        };
        assert_eq!(names(&staged)?, ["Zulu Wall", "Beta Roof", "Alpha Floor"]);
        assert_eq!(names(&native)?, ["Alpha Floor", "Beta Roof", "Zulu Wall"]);
        for (_, object) in staged.ordered_instances("BuildingSurface:Detailed")? {
            assert_eq!(
                object.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number("20".into()))
            );
            assert_eq!(
                object
                    .fields
                    .get(&FieldName("idf_max_extensible_fields".into())),
                Some(&RawValue::Number("0".into()))
            );
        }
        let mut without_metadata = staged.objects.clone();
        for objects in without_metadata.values_mut() {
            for object in objects.values_mut() {
                object.fields.remove(&FieldName("idf_max_fields".into()));
                object
                    .fields
                    .remove(&FieldName("idf_max_extensible_fields".into()));
            }
        }
        assert_eq!(without_metadata, native.objects);
        let rules = r#"{"GlobalGeometryRules":{"GlobalGeometryRules 1":{"starting_vertex_position":"UpperLeftCorner","vertex_entry_direction":"Counterclockwise","coordinate_system":"Relative"}}}"#;
        let mut rules = parse_epjson_str_with_idf_order(
            rules,
            "GlobalGeometryRules,UpperLeftCorner,Counterclockwise,Relative;",
        )?;
        let before = rules.clone();
        assert_eq!(
            rules.ordered_instances("GlobalGeometryRules")?[0]
                .1
                .fields
                .get(&FieldName("idf_max_fields".into())),
            Some(&RawValue::Number("3".into()))
        );
        assert!(apply_idf_declaration_order(&mut rules,"GlobalGeometryRules,UpperLeftCorner,Counterclockwise,Relative;GlobalGeometryRules,UpperLeftCorner,Counterclockwise,Relative;",IDF_ORDER_TARGETS).is_err());
        assert_eq!(rules, before);
        assert!(staged.has_idf_declaration_order("BuildingSurface:Detailed"));
        Ok(())
    }

    #[test]
    fn staged_idf_overlay_retains_reverse_alphabet_declaration_order()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = parse_epjson_str_with_idf_order(TWO_SPECIAL_DAYS_EPJSON, REVERSE_ALPHABET_IDF)?;
        let native_model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;

        assert!(model.has_idf_declaration_order("RunPeriodControl:SpecialDays"));
        assert_eq!(
            ordered_names(&model)?,
            vec!["Zulu Earlier Holiday", "Alpha Later Custom"]
        );
        assert_eq!(model.objects, native_model.objects);
        assert_ne!(model, native_model);
        assert_eq!(model, model.clone());
        Ok(())
    }

    #[test]
    fn scanner_handles_bom_comments_multiple_objects_and_literal_quotes()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{
            "Version":{"Version 1":{"version_identifier":"26.1"}},
            "RunPeriodControl:SpecialDays": {
                "\"Zulu Definition\"": {"start_date": "6/15"},
                "Alpha Definition": {"start_date": "6/15"}
            }
        }"#;
        let idf = "\u{feff}!- header\nVersion,26.1; RunPeriodControl:SpecialDays,\"zulu definition\",6/15,1,Holiday; ! trailing\nrunperiodcontrol:specialdays,ALPHA DEFINITION,6/15,1,CustomDay1;";
        let model = parse_epjson_str_with_idf_order(epjson, idf)?;

        assert_eq!(
            ordered_names(&model)?,
            vec!["\"Zulu Definition\"", "Alpha Definition"]
        );
        Ok(())
    }

    #[test]
    fn byte_scanner_ignores_non_utf8_comment_bytes() -> Result<(), Box<dyn std::error::Error>> {
        let mut idf = REVERSE_ALPHABET_IDF.as_bytes().to_vec();
        idf.extend_from_slice(b"\n! Windows-1252 comment: ");
        idf.push(0xe9);
        let mut model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;

        apply_idf_declaration_order_bytes(&mut model, &idf, IDF_ORDER_TARGETS)?;

        assert_eq!(
            ordered_names(&model)?,
            vec!["Zulu Earlier Holiday", "Alpha Later Custom"]
        );
        Ok(())
    }

    #[test]
    fn scanner_matches_energyplus_asymmetric_field_whitespace()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{
            "RunPeriodControl:SpecialDays": {
                "Zulu Definition\n": {"start_date": "6/15"},
                "Alpha Definition": {"start_date": "6/15"}
            }
        }"#;
        let idf = "RunPeriodControl:SpecialDays,\n  Zulu Definition\n  ,6/15,1,Holiday;\nRunPeriodControl:SpecialDays,\n  Alpha Definition,6/15,1,CustomDay1;";

        let model = parse_epjson_str_with_idf_order(epjson, idf)?;

        assert_eq!(
            ordered_names(&model)?,
            vec!["Zulu Definition\n", "Alpha Definition"]
        );
        Ok(())
    }

    #[test]
    fn scanner_matches_energyplus_comment_segment_overwrite()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{
            "RunPeriodControl:SpecialDays": {
                "Alpha Tail": {"start_date": "6/15"},
                "Beta Definition": {"start_date": "6/16"}
            }
        }"#;
        let idf = "RunPeriodControl:SpecialDays, Zulu Head ! comment\n Alpha Tail, 6/15, 1, Holiday;\nRunPeriodControl:SpecialDays, Beta Definition, 6/16, 1, CustomDay1;";

        let model = parse_epjson_str_with_idf_order(epjson, idf)?;

        assert_eq!(
            ordered_names(&model)?,
            vec!["Alpha Tail", "Beta Definition"]
        );
        Ok(())
    }

    #[test]
    fn scanner_rejects_name_set_mismatch() -> Result<(), Box<dyn std::error::Error>> {
        let mut model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;
        let idf = REVERSE_ALPHABET_IDF.replace("Alpha Later Custom", "Missing Custom");

        let error = required_order_error(
            apply_idf_declaration_order(&mut model, &idf, IDF_ORDER_TARGETS),
            "mismatched target names must fail",
        )?;

        assert!(error.to_string().contains("name mismatch"));
        assert!(!model.has_idf_declaration_order("RunPeriodControl:SpecialDays"));

        let one_object_idf = r#"
            RunPeriodControl:SpecialDays,
              Zulu Earlier Holiday,
              6/15,
              1,
              Holiday;
        "#;
        let error = required_order_error(
            apply_idf_declaration_order(&mut model, one_object_idf, IDF_ORDER_TARGETS),
            "mismatched target counts must fail",
        )?;
        assert!(error.to_string().contains("count mismatch"));
        Ok(())
    }

    #[test]
    fn scanner_rejects_duplicate_target_names() -> Result<(), Box<dyn std::error::Error>> {
        let mut model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;
        let idf = REVERSE_ALPHABET_IDF.replace("Alpha Later Custom", "Zulu Earlier Holiday");

        let error = required_order_error(
            apply_idf_declaration_order(&mut model, &idf, IDF_ORDER_TARGETS),
            "duplicate target names must fail",
        )?;

        assert!(error.to_string().contains("duplicate staged IDF"));
        Ok(())
    }

    #[test]
    fn scanner_accepts_eof_terminated_object_and_rejects_malformed_idf()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;
        let eof_terminated = REVERSE_ALPHABET_IDF.trim_end().trim_end_matches(';');
        apply_idf_declaration_order(&mut model, eof_terminated, IDF_ORDER_TARGETS)?;
        assert_eq!(
            ordered_names(&model)?,
            vec!["Zulu Earlier Holiday", "Alpha Later Custom"]
        );

        let error = required_order_error(
            apply_idf_declaration_order(&mut model, ",", IDF_ORDER_TARGETS),
            "unexpected comma must fail",
        )?;
        assert!(error.to_string().contains("unexpected comma"));
        Ok(())
    }
}

#[cfg(test)]
mod unnamed_report_order_tests {
    use crate::{
        FieldName, IDF_ORDER_TARGETS, RawModel, RawValue, apply_idf_declaration_order,
        parse_epjson_str, parse_epjson_str_with_idf_order,
    };

    #[test]
    fn twelve_unnamed_reports_keep_actual_idf_count_order_and_field_counts()
    -> Result<(), Box<dyn std::error::Error>> {
        let objects = (1..=12)
            .map(|index| {
                format!("\"Output:Constructions {index}\":{{\"details_type_1\":\"Materials\"}}")
            })
            .collect::<Vec<_>>()
            .join(",");
        let epjson = format!("{{\"Output:Constructions\":{{{objects}}}}}");
        let idf = "Output:Constructions, Materials;".repeat(12);
        assert!(RawModel::default().is_epjson());
        assert!(RawModel::new(None, Default::default()).is_epjson());
        let mut plain = parse_epjson_str(&epjson)?;
        assert!(plain.is_epjson());
        let before_failure = plain.clone();
        assert!(apply_idf_declaration_order(&mut plain, "", IDF_ORDER_TARGETS).is_err());
        assert_eq!(plain, before_failure);
        let mut raw = parse_epjson_str_with_idf_order(&epjson, &idf)?;
        assert!(!raw.is_epjson());
        let before_failure = raw.clone();
        assert!(apply_idf_declaration_order(&mut raw, "", IDF_ORDER_TARGETS).is_err());
        assert_eq!(raw, before_failure);
        let rows = raw.ordered_instances("Output:Constructions")?;
        assert_eq!(rows.len(), 12);
        for (index, (name, object)) in rows.iter().enumerate() {
            assert_eq!(name.0, format!("Output:Constructions {}", index + 1));
            assert_eq!(
                object.fields.get(&FieldName("idf_max_fields".to_string())),
                Some(&RawValue::Number("1".to_string()))
            );
        }
        // The same unnamed Source parser counts blank positional fields too.
        // Named materials use the same strict source overlay, not lexical order.
        for (family, idf, expected) in [
            (
                "Material",
                "Material,Z,Rough,0.1,1,100,1000;Material,A,Smooth,0.2,1,100,1000;",
                None,
            ),
            (
                "Material:NoMass",
                "Material:NoMass,Z,Rough,1;Material:NoMass,A,Smooth,2,,,;",
                Some([3, 6]),
            ),
            (
                "Zone",
                "Zone,Z;Zone,A,0,0,0,0,1,1,autocalculate,autocalculate,autocalculate,,,Yes;",
                Some([1, 13]),
            ),
            (
                "Building",
                "Building,Z,0,Suburbs,0.04,0.4,MinimalShadowing,25,6;Building,A,0,Suburbs,0.04,0.4,MinimalShadowing,25,6;",
                Some([8, 8]),
            ),
            (
                "Construction",
                "Construction,Z,Layer;Construction,A,Layer1,Layer2;",
                Some([2, 3]),
            ),
        ] {
            let epjson = format!("{{\"{family}\":{{\"A\":{{}},\"Z\":{{}}}}}}");
            let raw = parse_epjson_str_with_idf_order(&epjson, idf)?;
            let rows = raw.ordered_instances(family)?;
            assert_eq!(
                rows.iter()
                    .map(|(name, _)| name.0.as_str())
                    .collect::<Vec<_>>(),
                ["Z", "A"]
            );
            for (index, (_, object)) in rows.iter().enumerate() {
                let count = object.fields.get(&FieldName("idf_max_fields".into()));
                if let Some(expected) = expected {
                    assert_eq!(count, Some(&RawValue::Number(expected[index].to_string())));
                } else {
                    assert!(count.is_none());
                }
            }
        }
        for (family, fields, expected) in [
            ("Version", "26.1", 1),
            (
                "HeatBalanceAlgorithm",
                "ConductionTransferFunction,200,0.1,1000",
                4,
            ),
            ("Timestep", "4", 1),
            ("SimulationControl", "No,No,No,No,Yes,,1", 7),
            ("SurfaceConvectionAlgorithm:Inside", "TARP", 1),
            ("SurfaceConvectionAlgorithm:Outside", "DOE-2", 1),
            (
                "ZoneAirHeatBalanceAlgorithm",
                "ThirdOrderBackwardDifference,No,No",
                3,
            ),
        ] {
            let epjson = format!("{{\"{family}\":{{\"{family} 1\":{{}}}}}}");
            let raw = parse_epjson_str_with_idf_order(&epjson, &format!("{family},{fields};"))?;
            assert!(!raw.is_epjson());
            let rows = raw.ordered_instances(family)?;
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].0.0, format!("{family} 1"));
            assert_eq!(
                rows[0].1.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number(expected.to_string()))
            );
        }
        // Building is named: preserve its actual source name and eight fields.
        let raw = parse_epjson_str_with_idf_order(
            r#"{"Building":{"Actual Building":{"north_axis":0}}}"#,
            "Building,Actual Building,0,Suburbs,0.04,0.4,MinimalShadowing,25,6;",
        )?;
        let rows = raw.ordered_instances("Building")?;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0.0, "Actual Building");
        assert_eq!(
            rows[0].1.fields.get(&FieldName("idf_max_fields".into())),
            Some(&RawValue::Number("8".into()))
        );
        Ok(())
    }
}
