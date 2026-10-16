//! Sole actual HVACManager.hh138-160 constructor slice, retained across calls.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceHvacManager {
    pub(crate) simulation: super::simulation::state::SourceHvacSimulationModules,
    pub(crate) ancillary: super::ancillary::SourceHvacAncillaryModules,
    pub(crate) manage_iteration: i32,
    pub(crate) report_iteration_air: i32,
    pub(crate) simulate_hvac_iteration_setup: bool,
    pub(crate) trigger_get_afn: bool,
    pub(crate) my_one_time_flag: bool,
    pub(crate) printed_warmup: bool,
    pub(crate) my_environment_flag: bool,
    pub(crate) debug_names_reported: bool,
    pub(crate) my_setpoint_init: bool,
    pub(crate) my_environment_flag2: bool,
    pub(crate) flow_max_available_already_reset: bool,
    pub(crate) flow_resolution_needed: bool,
    pub(crate) error_count: i32,
    pub(crate) maximum_error_count: i32,
    pub(crate) error_environment_name: String,
}
impl Default for SourceHvacManager {
    fn default() -> Self {
        Self {
            simulation: Default::default(),
            ancillary: Default::default(),
            manage_iteration: 0,
            report_iteration_air: 0,
            simulate_hvac_iteration_setup: false,
            trigger_get_afn: true,
            my_one_time_flag: true,
            printed_warmup: false,
            my_environment_flag: true,
            debug_names_reported: false,
            my_setpoint_init: true,
            my_environment_flag2: true,
            flow_max_available_already_reset: false,
            flow_resolution_needed: false,
            error_count: 0,
            maximum_error_count: 0,
            error_environment_name: String::new(),
        }
    }
}
