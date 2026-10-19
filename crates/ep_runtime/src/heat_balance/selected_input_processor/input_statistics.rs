//! Genuine InputProcessor::reportIDFRecordsStats1479-1649 on retained raw input.
use super::{SelectedInputProcessor, SelectedInputUnavailable, SourceInputFamily};
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use std::collections::BTreeMap;
mod schema;

/// Raw JSON information read by the genuine statistics routine.
/// Numeric and boolean scalars share a branch: is_string=false and size=1.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceInputStatisticsValue {
    /// Actual JSON null, with size zero.
    Null,
    /// Actual JSON number or boolean; its numerical value is never read here.
    OtherScalar,
    /// Exact SourceB JSON number token, without a parse/roundtrip or default.
    Number(String),
    /// Exact SourceB JSON boolean scalar.
    Bool(bool),
    /// Actual raw string, preserving its case and blank contents.
    String(String),
    /// Actual raw array, retaining each extensible item and its order.
    Array(Vec<Self>),
    /// Actual raw object fields and exact-case identity.
    Object(BTreeMap<String, Self>),
}
impl SourceInputStatisticsValue {
    fn json_size(&self) -> usize {
        match self {
            Self::Null => 0,
            Self::OtherScalar | Self::Number(_) | Self::Bool(_) | Self::String(_) => 1,
            Self::Array(rows) => rows.len(),
            Self::Object(fields) => fields.len(),
        }
    }
}
/// Raw field map; presence remains distinct from a blank/defaulted value.
pub type SourceInputStatisticsFields = BTreeMap<String, SourceInputStatisticsValue>;
/// Same original raw family/object maps before any definition query inserts null.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceInputStatisticsInput {
    /// Exact family, object-name and field spelling in lexical JSON map order.
    pub objects: BTreeMap<String, BTreeMap<String, SourceInputStatisticsFields>>,
    /// SourceB effective input order from the same RawModel, including original
    /// validated IDF declaration order for Compact. Empty for older A callers.
    pub object_order: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Copy)]
enum SchemaDefault {
    Absent,
    NonString,
    String(&'static str),
}
#[derive(Clone, Copy)]
enum SchemaAutomatic {
    Autosize,
    Autocalculate,
}
struct SchemaField {
    name: &'static str,
    default: SchemaDefault,
    // Preserve the original anyOf/enum token order and multiplicity.
    automatic: &'static [SchemaAutomatic],
}
struct SchemaDefinition {
    has_name: bool,
    name_has_default: bool,
    regular: &'static [SchemaField],
    extension: Option<&'static str>,
    extensible: &'static [SchemaField],
}
fn increment(value: &mut i32) -> Result<(), SelectedInputUnavailable> {
    *value = value.checked_add(1).ok_or(SelectedInputUnavailable::Size)?;
    Ok(())
}
fn process_field(
    field: &SchemaField,
    raw: &SourceInputStatisticsFields,
    output: &mut SourceOutputPreScan,
) -> Result<(), SelectedInputUnavailable> {
    let mut has_default = false;
    let mut can_be_autosized = false;
    let mut can_be_autocalculated = false;
    let mut default_value = "";
    if !matches!(field.default, SchemaDefault::Absent) {
        increment(&mut output.total_fields_with_defaults)?;
        has_default = true;
        if let SchemaDefault::String(value) = field.default {
            default_value = value;
        }
    }
    for automatic in field.automatic {
        match automatic {
            SchemaAutomatic::Autosize => {
                increment(&mut output.total_autosizable_fields)?;
                can_be_autosized = true;
            }
            SchemaAutomatic::Autocalculate => {
                increment(&mut output.total_autocalculatable_fields)?;
                can_be_autocalculated = true;
            }
        }
    }
    if let Some(value) = raw.get(field.name) {
        if let SourceInputStatisticsValue::String(value) = value {
            if can_be_autosized && value == "Autosize" {
                increment(&mut output.number_autosized_fields)?;
            } else if can_be_autocalculated && value == "Autocalculate" {
                increment(&mut output.number_autocalculated_fields)?;
            }
        }
    } else if has_default {
        increment(&mut output.number_defaulted_fields)?;
        if can_be_autosized && default_value == "Autosize" {
            increment(&mut output.number_autosized_fields)?;
        } else if can_be_autocalculated && default_value == "Autocalculate" {
            increment(&mut output.number_autocalculated_fields)?;
        }
    }
    Ok(())
}
impl SelectedInputProcessor {
    /// Retain the original raw operands on the same IP before getter mutations.
    /// No statistics or defaulted field values are computed at this transfer.
    ///
    /// # Errors
    /// Rejects a second transfer or a different family/object/field-size owner.
    pub fn retain_source_input_statistics(
        &mut self,
        raw: SourceInputStatisticsInput,
    ) -> Result<(), SelectedInputUnavailable> {
        if self.statistics_input.is_some() || raw.objects.len() != self.families.len() {
            return Err(SelectedInputUnavailable::Association);
        }
        for (family_name, family) in &self.families {
            let objects = raw
                .objects
                .get(family_name)
                .ok_or(SelectedInputUnavailable::Association)?;
            let SourceInputFamily::Objects(shapes) = family else {
                return Err(SelectedInputUnavailable::Association);
            };
            if shapes.len() != objects.len() {
                return Err(SelectedInputUnavailable::Association);
            }
            for shape in shapes {
                let fields = objects
                    .get(&shape.object_name)
                    .ok_or(SelectedInputUnavailable::Association)?;
                if fields.len() != shape.field_sizes.len()
                    || fields.iter().any(|(name, value)| {
                        shape.field_sizes.get(name) != Some(&value.json_size())
                    })
                {
                    return Err(SelectedInputUnavailable::Association);
                }
            }
        }
        if !raw.object_order.is_empty() {
            if raw.object_order.len() != raw.objects.len() {
                return Err(SelectedInputUnavailable::Association);
            }
            for (family, objects) in &raw.objects {
                let order = raw
                    .object_order
                    .get(family)
                    .ok_or(SelectedInputUnavailable::Association)?;
                let names: std::collections::BTreeSet<_> = order.iter().collect();
                if order.len() != objects.len()
                    || names.len() != objects.len()
                    || order.iter().any(|name| !objects.contains_key(name))
                {
                    return Err(SelectedInputUnavailable::Association);
                }
            }
        }
        self.statistics_input = Some(raw);
        Ok(())
    }

    /// Borrow retained original object fields for a later genuine Source getter.
    /// Exact-case identity and raw array order are preserved; no values/defaults,
    /// usage marking, indices or schedule/node state are produced by this read.
    ///
    /// # Errors
    /// Rejects a missing raw owner, family or object association.
    pub fn source_raw_object_fields(
        &self,
        family: &str,
        object: &str,
    ) -> Result<&SourceInputStatisticsFields, SelectedInputUnavailable> {
        self.statistics_input
            .as_ref()
            .and_then(|raw| raw.objects.get(family))
            .and_then(|objects| objects.get(object))
            .ok_or(SelectedInputUnavailable::Association)
    }

    /// Borrow effective original object order retained by explicit SourceB.
    ///
    /// # Errors
    /// Rejects a missing raw order/family instead of inventing lexical order.
    pub fn source_raw_object_names(
        &self,
        family: &str,
    ) -> Result<&[String], SelectedInputUnavailable> {
        self.statistics_input
            .as_ref()
            .and_then(|raw| raw.object_order.get(family))
            .map(Vec::as_slice)
            .ok_or(SelectedInputUnavailable::Association)
    }

    /// Native getObjectItemNum1298 field overload on the same retained IP.
    /// Known absent families acquire the actual epJSON operator[] null; raw
    /// operands, object usage, shared arguments and defaults remain untouched.
    /// Direct epJSON retains JSON ordinals. A positive IDF match is admitted
    /// only for one object, where getIDFObjNum is the exact identity conversion.
    // Retain this real Native operand/local and its initialization/request timing for the unrepresented child path.
    #[allow(unused_variables)]
    pub(crate) fn get_object_item_num_by_field(
        &mut self,
        object_type: &str,
        field: &str,
        object_name: &str,
        source_is_epjson: bool,
    ) -> Result<i32, SelectedInputUnavailable> {
        let Ok(canonical) = self.canonical_type(object_type) else {
            return Ok(-1);
        };
        match self
            .families
            .entry(canonical.clone())
            .or_insert(SourceInputFamily::Null)
        {
            SourceInputFamily::Null => return Ok(0),
            SourceInputFamily::Objects(objects) if objects.is_empty() => return Ok(0),
            SourceInputFamily::Objects(_) => {}
        }
        let raw = self
            .statistics_input
            .as_ref()
            .ok_or(SelectedInputUnavailable::Association)?;
        let objects = raw
            .objects
            .get(&canonical)
            .ok_or(SelectedInputUnavailable::Association)?;
        // Native JSON map traversal precedes getIDFObjNum: duplicate field
        // values select the first lexical object, not the first IDF object.
        for (json_index, (name, fields)) in objects.iter().enumerate() {
            let Some(value) = fields.get(field) else {
                continue;
            };
            let SourceInputStatisticsValue::String(value) = value else {
                return Err(SelectedInputUnavailable::Association);
            };
            if value.eq_ignore_ascii_case(object_name) {
                // EquipmentConnections has no validated IDF-order overlay.
                // Its actual finite single-object domain has ordinal 1 in
                // both orders; multi-object IDF conversion stays unavailable.
                if !source_is_epjson && objects.len() != 1 {
                    return Err(SelectedInputUnavailable::Association);
                }
                let index = json_index;
                return i32::try_from(index.checked_add(1).ok_or(SelectedInputUnavailable::Size)?)
                    .map_err(|_| SelectedInputUnavailable::Size);
            }
        }
        Ok(0)
    }

    /// Actual Native322 child, after the six real shared shortcut allocations.
    /// Counters belong to the same DataOutputs owner used by later TAB reports.
    ///
    /// # Errors
    /// Retains every preceding reset/increment on unavailable metadata/raw
    /// operands or arithmetic outside the defined Native signed-int domain.
    pub fn report_source_idf_records_stats(
        &self,
        output: &mut SourceOutputPreScan,
    ) -> Result<(), SelectedInputUnavailable> {
        output.number_input_records = 0;
        output.number_defaulted_fields = 0;
        output.total_fields_with_defaults = 0;
        output.number_autosized_fields = 0;
        output.total_autosizable_fields = 0;
        output.number_autocalculated_fields = 0;
        output.total_autocalculatable_fields = 0;
        let raw = self
            .statistics_input
            .as_ref()
            .ok_or(SelectedInputUnavailable::Association)?;
        for (family_name, objects) in &raw.objects {
            let definition = schema::definition(family_name).ok_or_else(|| {
                SelectedInputUnavailable::Definition(format!(
                    "actual input statistics schema for {family_name}"
                ))
            })?;
            for fields in objects.values() {
                increment(&mut output.number_input_records)?;
                for field in definition.regular {
                    if definition.has_name && field.name == "name" {
                        if definition.name_has_default {
                            increment(&mut output.total_fields_with_defaults)?;
                            if !fields.contains_key("name") {
                                increment(&mut output.number_defaulted_fields)?;
                            }
                        }
                        continue;
                    }
                    process_field(field, fields, output)?;
                }
                if !definition.extensible.is_empty() {
                    let extension = definition
                        .extension
                        .ok_or(SelectedInputUnavailable::Association)?;
                    if let Some(value) = fields.get(extension) {
                        let SourceInputStatisticsValue::Array(items) = value else {
                            return Err(SelectedInputUnavailable::Association);
                        };
                        for item in items {
                            let SourceInputStatisticsValue::Object(extension_fields) = item else {
                                return Err(SelectedInputUnavailable::Association);
                            };
                            for field in definition.extensible {
                                process_field(field, extension_fields, output)?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
