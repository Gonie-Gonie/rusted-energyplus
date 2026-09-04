//! Lossless JSON serialization for one CP440 continue-warning call-site snapshot.

use ep_runtime::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot,
    heating_outdoor_air_maximum_flow_continue_warning_call_predecessor_cp439_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_first_warning_call::serialization::snapshot::snapshot_json as cp439_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot,
) -> Value {
    let predecessor =
        heating_outdoor_air_maximum_flow_continue_warning_call_predecessor_cp439_snapshot(snapshot);
    let mut value = cp439_snapshot_json(predecessor);
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
        "heating_outdoor_air_maximum_flow_continue_warning_call_site_reached".to_string(),
        json!(snapshot.heating_outdoor_air_maximum_flow_continue_warning_call_site_reached),
    );
    value
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_source_preserves_cp439_json_and_appends_only_the_call_marker() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp439_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.insert(").count(), 4);
        assert!(
            source.contains("heating_outdoor_air_maximum_flow_continue_warning_call_site_reached")
        );
        for forbidden in ["target.remove", "ieee_bits", "json_number"] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
