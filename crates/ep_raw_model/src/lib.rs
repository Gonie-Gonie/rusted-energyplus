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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum SourceInputFormat {
    #[default]
    EpJson,
    Idf,
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
    source_input_format: SourceInputFormat,
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
            source_input_format: SourceInputFormat::EpJson,
        }
    }

    /// Whether the actual source was epJSON rather than staged IDF.
    /// Successful IDF recovery retains this independently of per-family targets.
    #[must_use]
    pub fn is_epjson(&self) -> bool {
        self.source_input_format == SourceInputFormat::EpJson
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
    /// The default direct epJSON view retains lexical BTree object-name order.
    /// Converted IDF uses a validated declaration overlay for configured types.
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

    /// Returns explicit Source input order for one object type.
    ///
    /// Genuine staged IDF keeps the existing validated declaration overlay and
    /// its errors. Direct epJSON uses the pinned Windows doj comparator: signed
    /// bytes, ASCII digits, and wrapping 32-bit unsigned number accumulation.
    /// Only finite name sets with a strict pairwise order are available here.
    /// NUL names, comparator aliases, and non-strict or cyclic relations remain
    /// an explicit Source boundary; parsing never coalesces distinct raw keys.
    pub fn source_ordered_instances(
        &self,
        object_type: &str,
    ) -> Result<Vec<(&ObjectName, &RawObject)>, String> {
        if !self.is_epjson() {
            return self
                .ordered_instances(object_type)
                .map_err(|error| error.to_string());
        }
        self.source_json_instances(object_type)
    }

    /// Returns the pinned Source JSON map order, including for converted IDF.
    ///
    /// Direct DOM readers such as the reporting pre-scan never apply the IDF
    /// declaration overlay used by getObjectItem. The same strict finite-order
    /// checks apply to both input formats; no raw names are coalesced.
    pub fn source_json_instances(
        &self,
        object_type: &str,
    ) -> Result<Vec<(&ObjectName, &RawObject)>, String> {
        let Some(instances) = self.objects.get(&ObjectType(object_type.into())) else {
            return Ok(Vec::new());
        };
        let unavailable = || {
            format!(
                "SourceObjectOrderUnavailable: {object_type} requires a strict finite pinned Windows doj name order; NUL and distinct comparator aliases are not available"
            )
        };
        let mut ordered: Vec<(&ObjectName, &RawObject)> = Vec::with_capacity(instances.len());
        for row in instances {
            if row.0.0.as_bytes().contains(&0) {
                return Err(unavailable());
            }
            // Insertion does not require the comparator to implement Rust Ord
            // or a sorting algorithm's strict-weak-order precondition.
            let index = ordered
                .iter()
                .position(|other| {
                    source_windows_doj_compare(row.0.0.as_bytes(), other.0.0.as_bytes()) < 0
                })
                .unwrap_or(ordered.len());
            ordered.insert(index, row);
        }
        // Check every pair, including the reverse relation. This rejects
        // aliases, wrapping antipodes, and cycles instead of guessing an order.
        for (index, left) in ordered.iter().enumerate() {
            for right in &ordered[index + 1..] {
                if source_windows_doj_compare(left.0.0.as_bytes(), right.0.0.as_bytes()) >= 0
                    || source_windows_doj_compare(right.0.0.as_bytes(), left.0.0.as_bytes()) <= 0
                {
                    return Err(unavailable());
                }
            }
        }
        Ok(ordered)
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

// Literal non-locale doj::alphanum_impl for the pinned Windows ABI. This is
// deliberately not an Ord implementation: wrapping subtraction can fail strict
// ordering, which source_ordered_instances checks for the entire finite set.
fn source_windows_doj_compare(left: &[u8], right: &[u8]) -> i32 {
    let mut l = 0;
    let mut r = 0;
    loop {
        let l_char = left.get(l).copied().unwrap_or(0);
        let r_char = right.get(r).copied().unwrap_or(0);
        if l_char == 0 {
            return if r_char == 0 { 0 } else { -1 };
        }
        if r_char == 0 {
            return 1;
        }
        let l_digit = l_char.is_ascii_digit();
        let r_digit = r_char.is_ascii_digit();
        if l_digit && r_digit {
            let mut l_int = 0_u32;
            let mut r_int = 0_u32;
            let mut l_len = 0_u32;
            let mut r_len = 0_u32;
            while let Some(byte) = left.get(l).copied().filter(u8::is_ascii_digit) {
                l_int = l_int.wrapping_mul(10).wrapping_add(u32::from(byte - b'0'));
                l_len = l_len.wrapping_add(1);
                l += 1;
            }
            while let Some(byte) = right.get(r).copied().filter(u8::is_ascii_digit) {
                r_int = r_int.wrapping_mul(10).wrapping_add(u32::from(byte - b'0'));
                r_len = r_len.wrapping_add(1);
                r += 1;
            }
            let difference = i32::from_ne_bytes(l_int.wrapping_sub(r_int).to_ne_bytes());
            if difference != 0 {
                return difference;
            }
            if l_len < r_len {
                return 1;
            }
            if l_len > r_len {
                return -1;
            }
            continue;
        }
        if l_digit {
            return -1;
        }
        if r_digit {
            return 1;
        }
        let difference =
            i32::from(i8::from_ne_bytes([l_char])) - i32::from(i8::from_ne_bytes([r_char]));
        if difference != 0 {
            return difference;
        }
        l += 1;
        r += 1;
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
/// The default target set covers source-order-sensitive special days and construction families.
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
    // Keep the existing complete semantic validation first. In particular,
    // an out-of-range number in an overwritten duplicate must still fail.
    // No serde feature or generic Number behavior is changed.
    let _: serde_json::Value = serde_json::from_str(contents)?;
    let RawValue::Object(root) = ValidatedJsonLexemes {
        contents,
        cursor: 0,
    }
    .value()?
    else {
        return Err(EpJsonError::TopLevelNotObject);
    };
    let mut model = RawModel::default();
    for (FieldName(object_type_name), instances_value) in root {
        let RawValue::Object(instances) = instances_value else {
            return Err(EpJsonError::ObjectTypeNotObject {
                object_type: object_type_name,
            });
        };
        let mut raw_instances = BTreeMap::new();
        for (FieldName(object_name), fields_value) in instances {
            let RawValue::Object(fields) = fields_value else {
                return Err(EpJsonError::ObjectInstanceNotObject {
                    object_type: object_type_name.clone(),
                    object_name,
                });
            };
            raw_instances.insert(
                ObjectName(object_name),
                RawObject {
                    fields,
                    source_span: None,
                },
            );
        }
        model
            .objects
            .insert(ObjectType(object_type_name), raw_instances);
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

// This cursor is private and is used only after the full serde_json::Value
// validation above. It does not broaden JSON syntax, finite-number admission,
// depth, BOM or comment behavior. Strings still use the same serde decoder;
// only the original number token is copied instead of Number::to_string().
struct ValidatedJsonLexemes<'a> {
    contents: &'a str,
    cursor: usize,
}

impl ValidatedJsonLexemes<'_> {
    fn whitespace(&mut self) {
        while matches!(
            self.contents.as_bytes().get(self.cursor),
            Some(b' ' | b'\t' | b'\n' | b'\r')
        ) {
            self.cursor += 1;
        }
    }

    fn string(&mut self) -> Result<String, serde_json::Error> {
        let begin = self.cursor;
        self.cursor += 1; // Opening quote, already validated.
        while let Some(&byte) = self.contents.as_bytes().get(self.cursor) {
            self.cursor += 1;
            if byte == b'"' {
                break;
            }
            if byte == b'\\' && self.cursor < self.contents.len() {
                // Skip an escaped quote/backslash without interpreting it.
                // Full Unicode/surrogate decoding stays with serde below.
                self.cursor += 1;
            }
        }
        serde_json::from_str(&self.contents[begin..self.cursor])
    }

    fn value(&mut self) -> Result<RawValue, serde_json::Error> {
        self.whitespace();
        match self.contents.as_bytes().get(self.cursor) {
            Some(b'n') => {
                self.cursor += 4;
                Ok(RawValue::Null)
            }
            Some(b't') => {
                self.cursor += 4;
                Ok(RawValue::Bool(true))
            }
            Some(b'f') => {
                self.cursor += 5;
                Ok(RawValue::Bool(false))
            }
            Some(b'"') => self.string().map(RawValue::String),
            Some(b'[') => {
                self.cursor += 1;
                self.whitespace();
                let mut values = Vec::new();
                if self.contents.as_bytes().get(self.cursor) != Some(&b']') {
                    loop {
                        values.push(self.value()?);
                        self.whitespace();
                        if self.contents.as_bytes().get(self.cursor) == Some(&b']') {
                            break;
                        }
                        self.cursor += 1; // Validated comma.
                    }
                }
                self.cursor += 1;
                Ok(RawValue::Array(values))
            }
            Some(b'{') => {
                self.cursor += 1;
                self.whitespace();
                let mut values = BTreeMap::new();
                if self.contents.as_bytes().get(self.cursor) != Some(&b'}') {
                    loop {
                        self.whitespace();
                        let name = self.string()?;
                        self.whitespace();
                        self.cursor += 1; // Validated colon.
                        let value = self.value()?;
                        // Decode keys before insertion; a duplicate replaces
                        // its entire earlier subtree, exactly Value's map.
                        values.insert(FieldName(name), value);
                        self.whitespace();
                        if self.contents.as_bytes().get(self.cursor) == Some(&b'}') {
                            break;
                        }
                        self.cursor += 1; // Validated comma.
                    }
                }
                self.cursor += 1;
                Ok(RawValue::Object(values))
            }
            _ => {
                let begin = self.cursor;
                while self
                    .contents
                    .as_bytes()
                    .get(self.cursor)
                    .is_some_and(|byte| {
                        !matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b',' | b']' | b'}')
                    })
                {
                    self.cursor += 1;
                }
                let token = &self.contents[begin..self.cursor];
                // Retain ordinary JSON's finite Number check and error type.
                // Its parsed representation is deliberately not stored.
                let _: serde_json::Number = serde_json::from_str(token)?;
                Ok(RawValue::Number(token.to_owned()))
            }
        }
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

        // Number spelling survives actual text parsing, while nonnumeric
        // strings/keys/DOM and duplicate-key replacement remain unchanged.
        let lexical = parse_epjson_str(
            r#"{"Unknown:Object":{"Kept":{
            "integer":-0,"decimal":-0.0,"exponent":-0e0,"positive":1E+0,
            "underflow":-1e-9999,"uint":18446744073709551615,"outside":18446744073709551616,
            "nested":[-0,{"n":-0.0,"text":"escaped\\\" -0 [1E+0]"},null,true,false],
            "reserved":{"$serde_json::private::RawValue":"hello","$serde_json::private::Number":"0"},
            "string":"\u002d0",
            "v":{"stale":-0},"\u0076":{"kept":-0.0}
        }}}"#,
        )?;
        let fields = &lexical.objects[&ObjectType("Unknown:Object".into())]
            [&ObjectName("Kept".into())]
            .fields;
        for (name, token) in [
            ("integer", "-0"),
            ("decimal", "-0.0"),
            ("exponent", "-0e0"),
            ("positive", "1E+0"),
            ("underflow", "-1e-9999"),
            ("uint", "18446744073709551615"),
            ("outside", "18446744073709551616"),
        ] {
            assert_eq!(
                fields.get(&FieldName(name.into())),
                Some(&RawValue::Number(token.into()))
            );
        }
        assert_eq!(
            fields.get(&FieldName("string".into())),
            Some(&RawValue::String("-0".into()))
        );
        let Some(RawValue::Array(nested)) = fields.get(&FieldName("nested".into())) else {
            return Err(std::io::Error::other("actual nested lexical array").into());
        };
        assert_eq!(nested[0], RawValue::Number("-0".into()));
        let RawValue::Object(object) = &nested[1] else {
            return Err(std::io::Error::other("actual nested lexical object").into());
        };
        assert_eq!(
            object.get(&FieldName("n".into())),
            Some(&RawValue::Number("-0.0".into()))
        );
        assert_eq!(
            object.get(&FieldName("text".into())),
            Some(&RawValue::String("escaped\\\" -0 [1E+0]".into()))
        );
        assert_eq!(
            &nested[2..],
            &[RawValue::Null, RawValue::Bool(true), RawValue::Bool(false)]
        );
        let Some(RawValue::Object(reserved)) = fields.get(&FieldName("reserved".into())) else {
            return Err(std::io::Error::other("actual reserved metadata object").into());
        };
        assert_eq!(
            reserved.get(&FieldName("$serde_json::private::RawValue".into())),
            Some(&RawValue::String("hello".into()))
        );
        assert_eq!(
            reserved.get(&FieldName("$serde_json::private::Number".into())),
            Some(&RawValue::String("0".into()))
        );
        let Some(RawValue::Object(last)) = fields.get(&FieldName("v".into())) else {
            return Err(std::io::Error::other("actual escaped duplicate key").into());
        };
        assert_eq!(last.len(), 1);
        assert_eq!(
            last.get(&FieldName("kept".into())),
            Some(&RawValue::Number("-0.0".into()))
        );
        let duplicate = parse_epjson_str(
            r#"{"X":{"Old":{"n":-0}},"\u0058":{"Same":{"old":-0},"Same":{"new":[-0e0]}}}"#,
        )?;
        assert_eq!(duplicate.object_count(), 1);
        let same = &duplicate.objects[&ObjectType("X".into())][&ObjectName("Same".into())];
        assert_eq!(same.fields.len(), 1);
        assert_eq!(
            same.fields.get(&FieldName("new".into())),
            Some(&RawValue::Array(vec![RawValue::Number("-0e0".into())]))
        );
        for text in [
            r#"{"X":{"Y":{"n":1e9999,"n":0}}}"#,
            r#"{"X":{"Y":{"n":1e9999},"Y":{}}}"#,
            r#"{"X":{"Y":{"n":1e9999}},"X":{}}"#,
            r#"{"X":{"Y":{"n":NaN}}}"#,
            r#"{"X":{"Y":{"n":1.}}}"#,
        ] {
            assert!(
                matches!(parse_epjson_str(text), Err(super::EpJsonError::Json(_))),
                "{text}"
            );
        }
        assert!(matches!(
            parse_epjson_str("[]"),
            Err(super::EpJsonError::TopLevelNotObject)
        ));
        assert!(
            matches!(parse_epjson_str(r#"{"X":[]}"#),Err(super::EpJsonError::ObjectTypeNotObject{object_type})if object_type=="X")
        );
        assert!(
            matches!(parse_epjson_str(r#"{"X":{"Y":[]}}"#),Err(super::EpJsonError::ObjectInstanceNotObject{object_type,object_name})if object_type=="X"&&object_name=="Y")
        );

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
