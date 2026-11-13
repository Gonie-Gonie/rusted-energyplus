//! Common explicit Source NodeList schema/order admission; no Node state producer.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(super) fn admitted_field(name: &str, value: &RawValue, source_is_epjson: bool) -> bool {
    // Native direct-JSON getObjectItem/findMaxFields/getJSONObjNum ignore these
    // metadata fields. Staged IDF keeps its genuine numeric/overlay authority.
    if !source_is_epjson
        && matches!(
            name,
            "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
        )
    {
        return matches!(value, RawValue::Number(text) if text.parse::<usize>().is_ok());
    }
    if name == "nodes" {
        // The official optional field is an array, with required string members.
        // Exact empty/whitespace strings and ignored item metadata remain raw.
        return matches!(value, RawValue::Array(items) if items.iter().all(|item| {
            matches!(item, RawValue::Object(fields)
                if matches!(fields.get(&FieldName("node_name".into())), Some(RawValue::String(_))))
        }));
    }
    // The official instance schema permits additional unconsumed members.
    true
}

pub(super) fn validate_declaration_order(raw: &RawModel) -> Result<(), String> {
    // A caller can explicitly apply only a custom subset of IDF overlays.
    // Multiple lists need their actual legacy getter order on that path too.
    if !raw.is_epjson()
        && raw
            .objects
            .get(&ObjectType("NodeList".into()))
            .is_some_and(|rows| rows.len() > 1)
        && !raw.has_idf_declaration_order("NodeList")
    {
        return Err(
            "Multiple NodeLists in IDF require their actual recovered declaration order".into(),
        );
    }
    Ok(())
}
