//! Actual selected DataGlobal/HybridModel constructor owners.
//!
//! Input DoZoneSizing/DoSystemSizing remain separate from calculation flags.
//! No input switch, weather clock or allocation sets those calculation flags.

/// Selected external callback identity; positive API invocation remains outside this port.
/// Native std::function default constructor is empty, represented by None on DataGlobals.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SourceExternalHvacCallback(pub(crate) fn());
impl PartialEq for SourceExternalHvacCallback {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::fn_addr_eq(self.0, other.0)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedHeatBalanceGlobals {
    /// DataGlobals.hh88/172 ctorfalse; this frontend has no ReadVars/API setter.
    pub(crate) run_read_vars: bool,
    pub(crate) running_via_api: bool,
    /// Actual DataGlobals.hh167/169 null progress callbacks on the sole global owner.
    pub(crate) progress_callbacks:
        crate::weather::day::source_next_environment::SourceProgressCallbacks,
    /// Actual DataGlobals.hh130 constructorfalse, distinct from the input request.
    pub(crate) doing_hvac_sizing_simulations: bool,
    /// Actual DataGlobals.hh173–174; count flag is true, independently of all phase controls.
    pub(crate) non_zone_equipment: SourceNonZoneEquipmentGlobals,
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
    /// DataGlobals.hh145/146. Only actual RunSimulation/Setup callers write these.
    pub(crate) kickoff_simulation: bool,
    pub(crate) kickoff_sizing: bool,
    /// Actual DataGlobals.hh96 default-empty function object; ordinary admission has no API writer.
    pub(crate) external_hvac_manager: Option<SourceExternalHvacCallback>,
    /// Actual DataGlobals.hh97 constructorfalse, not an inferred initialization status.
    pub(crate) external_hvac_manager_initialized: bool,
    /// Actual DataGlobals.hh95/144/157 constructor-false owners.
    pub(crate) stop_simulation: bool,
    pub(crate) meters_have_been_initialized: bool,
    pub(crate) run_opt_cond_ent_temp: bool,
    /// DataGlobals.hh150-154. Constructor false; positive input writers remain separate.
    pub(crate) any_surface_property_overrides_in_model: bool,
    pub(crate) any_construction_overrides_in_model: bool,
    pub(crate) any_shading_control_in_model: bool,
    pub(crate) any_inside_shelf: bool,
    pub(crate) any_local_environments_in_model: bool,
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

/// Same actual DataGlobal members, grouped solely to retain their different constructor defaults.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceNonZoneEquipmentGlobals {
    pub(crate) number_water_heaters: i32,
    pub(crate) count_equipment: bool,
}
impl Default for SourceNonZoneEquipmentGlobals {
    fn default() -> Self {
        Self {
            number_water_heaters: 0,
            count_equipment: true,
        }
    }
}
