//! Raw epJSON-preserving model structures.

mod idf_order;

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

pub use idf_order::{
    IDF_ORDER_TARGETS, IdfOrderError, IdfOrderTarget, apply_idf_declaration_order,
};

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
    /// Per-type declaration order recovered from staged IDF input.
    idf_declaration_order: BTreeMap<ObjectType, Vec<ObjectName>>,
}

impl RawModel {
    /// Builds a model from its public epJSON fields without an IDF-order overlay.
    #[must_use]
    pub fn new(
        version: Option<String>,
        objects: BTreeMap<ObjectType, BTreeMap<ObjectName, RawObject>>,
    ) -> Self {
        Self {
            version,
            objects,
            idf_declaration_order: BTreeMap::new(),
        }
    }

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

    /// Returns one object type's instances in effective input order.
    ///
    /// Native epJSON models retain canonical object-name order. Converted IDF
    /// models use a validated declaration-order overlay for configured types.
    pub fn ordered_instances(
        &self,
        object_type: &str,
    ) -> Result<Vec<(&ObjectName, &RawObject)>, IdfOrderError> {
        let object_type = ObjectType(object_type.to_string());
        let instances = self.objects.get(&object_type);
        let declaration_order = self.idf_declaration_order.get(&object_type);

        match (instances, declaration_order) {
            (None, None) => Ok(Vec::new()),
            (Some(instances), None) => Ok(instances.iter().collect()),
            (None, Some(order)) => Err(IdfOrderError::new(format!(
                "IDF declaration-order overlay for {} has {} name(s), but the object map is absent",
                object_type.0,
                order.len()
            ))),
            (Some(instances), Some(order)) => {
                if order.len() != instances.len() {
                    return Err(IdfOrderError::new(format!(
                        "IDF declaration-order overlay count mismatch for {}: overlay has {}, object map has {}",
                        object_type.0,
                        order.len(),
                        instances.len()
                    )));
                }

                let mut ordered = Vec::with_capacity(order.len());
                let mut seen = std::collections::BTreeSet::new();
                for name in order {
                    if !seen.insert(name) {
                        return Err(IdfOrderError::new(format!(
                            "IDF declaration-order overlay for {} repeats object name {}",
                            object_type.0, name.0
                        )));
                    }
                    let Some((actual_name, object)) = instances.get_key_value(name) else {
                        return Err(IdfOrderError::new(format!(
                            "IDF declaration-order overlay for {} names missing object {}",
                            object_type.0, name.0
                        )));
                    };
                    ordered.push((actual_name, object));
                }
                Ok(ordered)
            }
        }
    }

    /// Returns true when one object type has a recovered IDF declaration order.
    #[must_use]
    pub fn has_idf_declaration_order(&self, object_type: &str) -> bool {
        self.idf_declaration_order
            .contains_key(&ObjectType(object_type.to_string()))
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
    /// Staged IDF read failed while recovering declaration order.
    IdfIo {
        /// IDF path that could not be read.
        path: PathBuf,
        /// Underlying filesystem error.
        source: std::io::Error,
    },
    /// Staged IDF declaration order could not be reconciled with epJSON.
    IdfOrder(IdfOrderError),
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
            Self::IdfIo { path, source } => write!(
                formatter,
                "failed to read staged IDF {} for declaration-order recovery: {source}",
                path.display()
            ),
            Self::IdfOrder(error) => {
                write!(
                    formatter,
                    "failed to recover staged IDF declaration order: {error}"
                )
            }
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
            Self::IdfIo { source, .. } => Some(source),
            Self::IdfOrder(error) => Some(error),
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

impl From<IdfOrderError> for EpJsonError {
    fn from(error: IdfOrderError) -> Self {
        Self::IdfOrder(error)
    }
}

/// Loads an epJSON file into a RawModel.
pub fn load_epjson_file(path: impl AsRef<Path>) -> Result<RawModel, EpJsonError> {
    let contents = std::fs::read_to_string(path)?;
    parse_epjson_str(&contents)
}

/// Loads converted epJSON and overlays configured declaration order from staged IDF.
///
/// The default target set is currently limited to `RunPeriodControl:SpecialDays`.
pub fn load_epjson_file_with_idf_order(
    epjson_path: impl AsRef<Path>,
    idf_path: impl AsRef<Path>,
) -> Result<RawModel, EpJsonError> {
    let mut model = load_epjson_file(epjson_path)?;
    let idf_path = idf_path.as_ref();
    let idf = std::fs::read(idf_path).map_err(|source| EpJsonError::IdfIo {
        path: idf_path.to_path_buf(),
        source,
    })?;
    idf_order::apply_idf_declaration_order_bytes(&mut model, &idf, IDF_ORDER_TARGETS)?;
    Ok(model)
}

/// Parses epJSON text into a RawModel.
pub fn parse_epjson_str(contents: &str) -> Result<RawModel, EpJsonError> {
    let contents = contents.trim_start_matches('\u{feff}');
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

/// Parses converted epJSON and overlays configured declaration order from staged IDF text.
pub fn parse_epjson_str_with_idf_order(epjson: &str, idf: &str) -> Result<RawModel, EpJsonError> {
    let mut model = parse_epjson_str(epjson)?;
    apply_idf_declaration_order(&mut model, idf, IDF_ORDER_TARGETS)?;
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

    #[test]
    fn parses_utf8_bom_epjson() -> Result<(), Box<dyn std::error::Error>> {
        let model = parse_epjson_str(
            "\u{feff}{\"Version\":{\"Version 1\":{\"version_identifier\":\"26.1\"}}}",
        )?;

        assert_eq!(model.version, Some("26.1".to_string()));
        assert_eq!(model.object_type_count(), 1);
        Ok(())
    }
}
