//! Value-producing finite EquipmentConnections getObjectItem and JSON fields.
//! Unconsumed Native NumAlphas/NumNums locals are not fabricated from JSON.
use super::Unavailable;
use crate::heat_balance::selected_input_processor::{SelectedInputProcessor, SourceInputStatisticsFields, SourceInputStatisticsValue as V};
pub(super) struct SourceEquipmentGetterLocals {
    pub(super) alphas: Vec<String>,
    pub(super) numbers: Vec<f64>,
    pub(super) alpha_names: Vec<String>,
    pub(super) numeric_names: Vec<String>,
    pub(super) alpha_blanks: Vec<bool>,
    pub(super) numeric_blanks: Vec<bool>,
}
fn unavailable(text: &'static str) -> Unavailable { Unavailable::Input(text.into()) }
pub(super) fn alpha(fields: &SourceInputStatisticsFields, field: &str, default: &str) -> Result<String, Unavailable> {
    match fields.get(field) {
        None => Ok(default.to_ascii_uppercase()),
        Some(V::String(value)) if value.is_empty() => Ok(default.to_ascii_uppercase()),
        Some(V::String(value)) => Ok(value.to_ascii_uppercase()),
        _ => Err(unavailable("actual Equipment JSON alpha type")),
    }
}
pub(super) fn integer(fields: &SourceInputStatisticsFields, field: &str) -> Result<i32, Unavailable> {
    match fields.get(field) {
        None => Ok(0),
        Some(V::String(value)) if value.is_empty() => Ok(0),
        Some(V::Number(value)) => value.parse::<i64>().ok().and_then(|value| i32::try_from(value).ok())
            .ok_or_else(|| unavailable("actual Equipment JSON getIntFieldValue integer domain")),
        _ => Err(unavailable("actual Equipment JSON getIntFieldValue type")),
    }
}
pub(super) fn sole_object(input: &SelectedInputProcessor, family: &str) -> Result<String, Unavailable> {
    let order = input.source_raw_object_names(family).map_err(|_| unavailable("actual Equipment retained raw object identity"))?;
    if order.len() != 1 || input.get_num_objects_found(family) != 1 {
        return Err(unavailable("actual single Equipment object getter ordinal; multi-IDF order not retained"));
    }
    order.first().cloned().ok_or(Unavailable::InconsistentOwner)
}
impl SourceEquipmentGetterLocals {
    pub(super) fn get_connections(&mut self, input: &mut SelectedInputProcessor, source_is_epjson: bool) -> Result<(), Unavailable> {
        const FIELDS: [(&str, &str); 8] = [
            ("zone_name", "Zone Name"),
            ("zone_conditioning_equipment_list_name", "Zone Conditioning Equipment List Name"),
            ("zone_air_inlet_node_or_nodelist_name", "Zone Air Inlet Node or NodeList Name"),
            ("zone_air_exhaust_node_or_nodelist_name", "Zone Air Exhaust Node or NodeList Name"),
            ("zone_air_node_name", "Zone Air Node Name"),
            ("zone_return_air_node_or_nodelist_name", "Zone Return Air Node or NodeList Name"),
            ("zone_return_air_node_1_flow_rate_fraction_schedule_name", "Zone Return Air Node 1 Flow Rate Fraction Schedule Name"),
            ("zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name", "Zone Return Air Node 1 Flow Rate Basis Node or NodeList Name"),
        ];
        let object = sole_object(input, "ZoneHVAC:EquipmentConnections")?;
        if self.alphas.len() < 8 || self.alpha_names.len() < 8 || self.alpha_blanks.len() < 8 {
            return Err(Unavailable::InconsistentOwner);
        }
        self.alphas.fill(String::new()); self.numbers.fill(0.0);
        self.numeric_blanks.fill(true); self.alpha_blanks.fill(true);
        self.alpha_names.fill(String::new()); self.numeric_names.fill(String::new());
        input.mark_object_as_used("ZoneHVAC:EquipmentConnections", &object);
        let fields = input.source_raw_object_fields("ZoneHVAC:EquipmentConnections", &object)
            .map_err(|_| unavailable("actual EquipmentConnections retained fields"))?;
        for (index, &(json, legacy)) in FIELDS.iter().enumerate() {
            self.alphas[index] = alpha(fields, json, "")?;
            self.alpha_blanks[index] = match fields.get(json) {
                None => true,
                Some(V::String(value)) => value.is_empty(),
                _ => return Err(unavailable("actual EquipmentConnections alpha blank type")),
            };
            self.alpha_names[index] = if source_is_epjson { json } else { legacy }.into();
        }
        Ok(())
    }
}
