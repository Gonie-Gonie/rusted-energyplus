//! Raw epJSON-preserving model structures.

use std::collections::BTreeMap;

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
    /// JSON number represented as f64 until a typed unit validates it.
    Number(f64),
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
}

#[cfg(test)]
mod tests {
    use super::{ObjectName, ObjectType, RawModel, RawObject};
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
}
