//! Lossless JSON serialization for one CP439 first-warning call-site snapshot.

use ep_runtime::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot,
    heating_outdoor_air_maximum_flow_first_warning_call_predecessor_cp438_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_first_warning_counter_increment::serialization::snapshot::snapshot_json as cp438_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot,
) -> Value {
    let predecessor =
        heating_outdoor_air_maximum_flow_first_warning_call_predecessor_cp438_snapshot(snapshot);
    let mut value = cp438_snapshot_json(predecessor);
    let Value::Object(target) = &mut value else {
        return Value::Null;
    };
    target.insert("source".to_string(), json!(snapshot.source));
    target.insert(
        "first_excluded_source".to_string(),
        json!(snapshot.first_excluded_source),
    );
    target.insert("source_order".to_string(), json!(snapshot.source_order));
    target.insert(
        "heating_outdoor_air_maximum_flow_first_warning_call_site_reached".to_string(),
        json!(snapshot.heating_outdoor_air_maximum_flow_first_warning_call_site_reached),
    );
    value
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_source_preserves_cp438_json_and_appends_only_the_call_marker() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp438_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.insert(").count(), 4);
        assert!(
            source.contains("heating_outdoor_air_maximum_flow_first_warning_call_site_reached")
        );
        for forbidden in ["target.remove", "ieee_bits", "json_number"] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
