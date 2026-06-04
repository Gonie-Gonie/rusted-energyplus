//! Raw epJSON-preserving model structures.

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// EnergyPlus object type name as found in epJSON.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ObjectType(pub String);

/// EnergyPlus object instance name as found in epJSON.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ObjectName(pub String);

/// EnergyPlus object field name as found in epJSON.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FieldName(pub String);

/// Raw scalar or structured value preserved before typed conversion.
#[derive(Clone, Debug, PartialEq)]
pub enum RawValue {
    /// JSON null.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// JSON number represented textually until a typed unit validates it.
    Number(String),
    /// JSON string.
    String(String),
    /// JSON array.
    Array(Vec<RawValue>),
    /// JSON object.
    Object(BTreeMap<FieldName, RawValue>),
}

/// Source location for future structured diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceSpan {
    /// One-based line if known.
    pub line: u32,
    /// One-based column if known.
    pub column: u32,
}

/// Raw object with original fields preserved.
#[derive(Clone, Debug, PartialEq)]
pub struct RawObject {
    /// Original object fields.
    pub fields: BTreeMap<FieldName, RawValue>,
    /// Optional source span.
    pub source_span: Option<SourceSpan>,
}

/// Raw epJSON model before defaults, validation, typed conversion, or reference resolution.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawModel {
    /// EnergyPlus version string if present.
    pub version: Option<String>,
    /// Objects grouped by type and name.
    pub objects: BTreeMap<ObjectType, BTreeMap<ObjectName, RawObject>>,
}

impl RawModel {
    /// Returns the total object instance count.
    #[must_use]
    pub fn object_count(&self) -> usize {
        self.objects.values().map(BTreeMap::len).sum()
    }

    /// Returns true when the model has no objects.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Returns the number of object types in the model.
    #[must_use]
    pub fn object_type_count(&self) -> usize {
        self.objects.len()
    }

    /// Returns object instance counts by object type.
    #[must_use]
    pub fn object_type_counts(&self) -> BTreeMap<String, usize> {
        self.objects
            .iter()
            .map(|(object_type, instances)| (object_type.0.clone(), instances.len()))
            .collect()
    }

    /// Returns a compact inspection summary.
    #[must_use]
    pub fn summary(&self) -> RawModelSummary {
        RawModelSummary {
            version: self.version.clone(),
            object_type_count: self.object_type_count(),
            object_count: self.object_count(),
            object_type_counts: self.object_type_counts(),
        }
    }
}

/// Compact RawModel inspection summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawModelSummary {
    /// EnergyPlus input version if available.
    pub version: Option<String>,
    /// Count of top-level object types.
    pub object_type_count: usize,
    /// Count of object instances.
    pub object_count: usize,
    /// Object instance count by object type.
    pub object_type_counts: BTreeMap<String, usize>,
}

/// Error returned while reading or parsing epJSON.
#[derive(Debug)]
pub enum EpJsonError {
    /// File read failed.
    Io(std::io::Error),
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Top-level JSON value was not an object.
    TopLevelNotObject,
    /// A top-level object type did not contain an object map.
    ObjectTypeNotObject {
        /// Object type name.
        object_type: String,
    },
    /// An object instance did not contain a field map.
    ObjectInstanceNotObject {
        /// Object type name.
        object_type: String,
        /// Object instance name.
        object_name: String,
    },
}

impl Display for EpJsonError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read epJSON: {error}"),
            Self::Json(error) => write!(formatter, "failed to parse epJSON: {error}"),
            Self::TopLevelNotObject => {
                write!(formatter, "epJSON top-level value must be an object")
            }
            Self::ObjectTypeNotObject { object_type } => {
                write!(
                    formatter,
                    "epJSON object type '{object_type}' must contain an object map"
                )
            }
            Self::ObjectInstanceNotObject {
                object_type,
                object_name,
            } => write!(
                formatter,
                "epJSON object '{object_type}/{object_name}' must contain a field map"
            ),
        }
    }
}

impl std::error::Error for EpJsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::TopLevelNotObject
            | Self::ObjectTypeNotObject { .. }
            | Self::ObjectInstanceNotObject { .. } => None,
        }
    }
}

impl From<std::io::Error> for EpJsonError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for EpJsonError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

/// Loads an epJSON file into a RawModel.
pub fn load_epjson_file(path: impl AsRef<Path>) -> Result<RawModel, EpJsonError> {
    let contents = std::fs::read_to_string(path)?;
    parse_epjson_str(&contents)
}

/// Parses epJSON text into a RawModel.
pub fn parse_epjson_str(contents: &str) -> Result<RawModel, EpJsonError> {
    let value: serde_json::Value = serde_json::from_str(contents)?;
    let root = value.as_object().ok_or(EpJsonError::TopLevelNotObject)?;
    let mut model = RawModel::default();

    for (object_type_name, instances_value) in root {
        let instances =
            instances_value
                .as_object()
                .ok_or_else(|| EpJsonError::ObjectTypeNotObject {
                    object_type: object_type_name.clone(),
                })?;
        let mut raw_instances = BTreeMap::new();

        for (object_name, fields_value) in instances {
            let fields =
                fields_value
                    .as_object()
                    .ok_or_else(|| EpJsonError::ObjectInstanceNotObject {
                        object_type: object_type_name.clone(),
                        object_name: object_name.clone(),
                    })?;
            let raw_fields = fields
                .iter()
                .map(|(field_name, value)| {
                    (FieldName(field_name.clone()), raw_value_from_json(value))
                })
                .collect();

            raw_instances.insert(
                ObjectName(object_name.clone()),
                RawObject {
                    fields: raw_fields,
                    source_span: None,
                },
            );
        }

        model
            .objects
            .insert(ObjectType(object_type_name.clone()), raw_instances);
    }

    model.version = extract_version(&model);
    Ok(model)
}

fn raw_value_from_json(value: &serde_json::Value) -> RawValue {
    match value {
        serde_json::Value::Null => RawValue::Null,
        serde_json::Value::Bool(value) => RawValue::Bool(*value),
        serde_json::Value::Number(value) => RawValue::Number(value.to_string()),
        serde_json::Value::String(value) => RawValue::String(value.clone()),
        serde_json::Value::Array(values) => {
            RawValue::Array(values.iter().map(raw_value_from_json).collect())
        }
        serde_json::Value::Object(values) => RawValue::Object(
            values
                .iter()
                .map(|(field_name, value)| {
                    (FieldName(field_name.clone()), raw_value_from_json(value))
                })
                .collect(),
        ),
    }
}

fn extract_version(model: &RawModel) -> Option<String> {
    let version_objects = model.objects.get(&ObjectType("Version".to_string()))?;
    for object in version_objects.values() {
        if let Some(RawValue::String(version)) = object
            .fields
            .get(&FieldName("version_identifier".to_string()))
        {
            return Some(version.clone());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{
        FieldName, ObjectName, ObjectType, RawModel, RawObject, RawValue, parse_epjson_str,
    };
    use std::collections::BTreeMap;

    #[test]
    fn object_count_sums_instances() {
        let mut model = RawModel::default();
        let mut buildings = BTreeMap::new();
        buildings.insert(
            ObjectName("Main".to_string()),
            RawObject {
                fields: BTreeMap::new(),
                source_span: None,
            },
        );
        model
            .objects
            .insert(ObjectType("Building".to_string()), buildings);

        assert_eq!(model.object_count(), 1);
        assert!(!model.is_empty());
    }

    #[test]
    fn parses_epjson_object_tree() -> Result<(), Box<dyn std::error::Error>> {
        let model = parse_epjson_str(
            r#"{
                "Version": {
                    "Version 1": {
                        "version_identifier": "26.1"
                    }
                },
                "Building": {
                    "Small": {
                        "north_axis": 0,
                        "terrain": "Suburbs"
                    }
                },
                "Unknown:Object": {
                    "Kept": {
                        "nested": [{"field": true}]
                    }
                }
            }"#,
        )?;

        assert_eq!(model.version, Some("26.1".to_string()));
        assert_eq!(model.object_type_count(), 3);
        assert_eq!(model.object_count(), 3);

        let building = model
            .objects
            .get(&ObjectType("Building".to_string()))
            .and_then(|objects| objects.get(&ObjectName("Small".to_string())));
        let Some(building) = building else {
            return Err(std::io::Error::other("missing Building/Small").into());
        };

        assert_eq!(
            building.fields.get(&FieldName("north_axis".to_string())),
            Some(&RawValue::Number("0".to_string()))
        );

        let summary = model.summary();
        assert_eq!(summary.object_type_counts.get("Unknown:Object"), Some(&1));

        Ok(())
    }
}
