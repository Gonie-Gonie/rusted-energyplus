//! Actual selected DataGlobal/HybridModel constructor owners.
//!
//! Input DoZoneSizing/DoSystemSizing remain separate from calculation flags.
//! No input switch, weather clock or allocation sets those calculation flags.

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedHeatBalanceGlobals {
    /// Actual GetZoneData/GetSpaceData counts, independently initialized zero.
    pub(crate) num_of_zones: i32,
    pub(crate) num_spaces: i32,
    pub(crate) num_space_types: i32,
    /// Actual DataGlobals GetProjectData/diagnostics owners; all ctor false/0.
    pub(crate) do_zone_sizing: bool,
    pub(crate) do_system_sizing: bool,
    pub(crate) do_plant_sizing: bool,
    pub(crate) do_hvac_sizing_simulation: bool,
    pub(crate) hvac_sizing_sim_max_iterations: i32,
    pub(crate) display_all_warnings: bool,
    pub(crate) display_extra_warnings: bool,
    pub(crate) display_unused_objects: bool,
    pub(crate) display_unused_schedules: bool,
    pub(crate) display_advanced_report_variables: bool,
    pub(crate) display_zone_air_heat_balance_off_balance: bool,
    pub(crate) do_coil_direct_solutions: bool,
    pub(crate) create_perf_log: bool,
    /// DataGlobals.hh107; positive PerformancePrecision override remains unavailable.
    pub(crate) override_timestep: bool,
    /// DataGlobals.hh124; actual GetProjectData may later set it true.
    pub(crate) do_pure_load_calc: bool,
    /// DataGlobals.hh180, actual isInputObjectUsed DOAS count writer.
    pub(crate) air_loop_hvac_doas_used_in_sim: bool,
    /// DataGlobals.hh149, actual repeated CheckIfAnyEMS writer.
    pub(crate) any_energy_management_system_in_model: bool,
    /// DataGlobals.hh155, actual CheckIfAnyPlant writer.
    pub(crate) any_plant_in_model: bool,
    /// DataGlobals.hh162, actual CheckIfAnySlabs writer.
    pub(crate) any_slabs_in_model: bool,
    /// DataGlobals.hh163, actual CheckIfAnyBasements writer.
    pub(crate) any_basements_in_model: bool,
    /// DataGlobals.hh156, actual ideal-condenser setpoint count writer.
    pub(crate) any_ideal_cond_ent_setpoint_in_model: bool,
    /// DataGlobals.hh117; written by the real sizing caller, not DoZoneSizing.
    pub(crate) zone_sizing_calculation: bool,
    /// DataGlobals.hh118; written by the real sizing caller, not DoSystemSizing.
    pub(crate) system_sizing_calculation: bool,
    /// DataGlobals.hh129; SimulationManager233/240 is the actual caller writer.
    pub(crate) doing_sizing: bool,
    /// DataGlobals.hh158 and160, independently retained constructor members.
    pub(crate) component_load_report_requested: bool,
    pub(crate) load_component_pulse_now: bool,
    /// HybridModel.hh126; the admitted input excludes its positive getter.
    pub(crate) hybrid_model_people_count: bool,
}
