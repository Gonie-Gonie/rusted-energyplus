//! Actual simple-air constructor and reached allocation owners; no numeric mirror.
use std::collections::{BTreeMap, BTreeSet};
use std::convert::Infallible;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneAirBalanceData {
    pub(crate) name: String,
    pub(crate) zone_name: String,
    pub(crate) zone_ptr: i32,
    pub(crate) balance_method: i32,
    pub(crate) induced_air_rate: f64,
    pub(crate) induced_air_schedule: Option<Infallible>,
    pub(crate) balanced_mass_flow_rate: f64,
    pub(crate) infiltration_mass_flow_rate: f64,
    pub(crate) natural_mass_flow_rate: f64,
    pub(crate) exhaust_mass_flow_rate: f64,
    pub(crate) intake_mass_flow_rate: f64,
    pub(crate) erv_mass_flow_rate: f64,
    pub(crate) one_time_flag: bool,
    pub(crate) num_ervs: i32,
    pub(crate) erv_inlet_nodes: Option<Vec<i32>>,
    pub(crate) erv_exhaust_nodes: Option<Vec<i32>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneMassConservationData {
    pub(crate) name: String,
    pub(crate) zone_ptr: i32,
    pub(crate) inlet_mass_flow_rate: f64,
    pub(crate) exhaust_mass_flow_rate: f64,
    pub(crate) return_mass_flow_rate: f64,
    pub(crate) mixing_mass_flow_rate: f64,
    pub(crate) mixing_source_mass_flow_rate: f64,
    pub(crate) num_source_zones_mixing_object: i32,
    pub(crate) num_receiving_zones_mixing_object: i32,
    pub(crate) is_only_source_zone: bool,
    pub(crate) is_source_and_receiving_zone: bool,
    pub(crate) infiltration_ptr: i32,
    pub(crate) infiltration_mass_flow_rate: f64,
    pub(crate) include_infiltration_to_zone_mass_balance: i32,
    pub(crate) zone_mixing_sources_ptr: Option<Vec<i32>>,
    pub(crate) zone_mixing_receiving_ptr: Option<Vec<i32>>,
    pub(crate) zone_mixing_receiving_fraction: Option<Vec<f64>>,
}

/// Other fields of the same actual ZoneAirMassFlow aggregate. Its existing
/// EnforceZoneMassBalance cell remains on SelectedHeatBalanceInputState.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceZoneMassFlowControl {
    pub(crate) zone_flow_adjustment: i32,
    pub(crate) infiltration_treatment: i32,
    pub(crate) infiltration_for_zones: i32,
    pub(crate) adjust_zone_mixing_flow: bool,
    pub(crate) adjust_zone_infiltration_flow: bool,
}
impl Default for SourceZoneMassFlowControl {
    fn default() -> Self {
        Self {
            zone_flow_adjustment: 4,
            infiltration_treatment: 0,
            infiltration_for_zones: -1,
            adjust_zone_mixing_flow: false,
            adjust_zone_infiltration_flow: false,
        }
    }
}

/// The admitted input helper returns only zero instances. Unallocated and
/// allocated-empty Native collections remain distinct; positive element
/// constructors/getters are unavailable before these selected allocations.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSimpleAir {
    /// HeatBalanceAirMgrData HH101; cleared only after genuine full getter.
    pub(crate) manage_air_heat_balance_get_input_flag: bool,
    /// HeatBalanceAirMgrData HH102; actual report child owns its successful clear.
    pub(crate) calc_extra_report_var_one_time_flag: bool,
    pub(crate) air_flow_flag: bool,
    pub(crate) total_zone_air_balance: i32,
    pub(crate) total_infiltration: i32,
    pub(crate) total_ventilation: i32,
    pub(crate) total_mixing: i32,
    pub(crate) total_cross_mixing: i32,
    pub(crate) total_refrigeration_door_mixing: i32,
    pub(crate) zone_air_balance: Option<Vec<SourceZoneAirBalanceData>>,
    pub(crate) infiltration: Option<Vec<Infallible>>,
    pub(crate) ventilation: Option<Vec<Infallible>>,
    pub(crate) mixing: Option<Vec<Infallible>>,
    pub(crate) cross_mixing: Option<Vec<Infallible>>,
    pub(crate) refrigeration_door_mixing: Option<Vec<Infallible>>,
    pub(crate) air_boundary_mixing: Vec<Infallible>,
    pub(crate) mass_conservation: Option<Vec<SourceZoneMassConservationData>>,
    pub(crate) mass_flow_control: SourceZoneMassFlowControl,
    pub(crate) zone_total_exfiltration_heat_loss: f64,
    pub(crate) zone_total_exhaust_heat_loss: f64,
    pub(crate) unique_zone_names: BTreeSet<String>,
    pub(crate) unique_infiltration_names: BTreeMap<String, String>,
}

impl Default for SourceSimpleAir {
    fn default() -> Self {
        Self {
            manage_air_heat_balance_get_input_flag: true,
            calc_extra_report_var_one_time_flag: true,
            air_flow_flag: false,
            total_zone_air_balance: 0,
            total_infiltration: 0,
            total_ventilation: 0,
            total_mixing: 0,
            total_cross_mixing: 0,
            total_refrigeration_door_mixing: 0,
            zone_air_balance: None,
            infiltration: None,
            ventilation: None,
            mixing: None,
            cross_mixing: None,
            refrigeration_door_mixing: None,
            air_boundary_mixing: Vec::default(),
            mass_conservation: None,
            mass_flow_control: SourceZoneMassFlowControl::default(),
            zone_total_exfiltration_heat_loss: 0.0,
            zone_total_exhaust_heat_loss: 0.0,
            unique_zone_names: BTreeSet::default(),
            unique_infiltration_names: BTreeMap::default(),
        }
    }
}
