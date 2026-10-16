//! Actual independent Source module members needed by SimHVAC and its real children.
use std::convert::Infallible;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceAirLoopData {
    /// Actual DataAirLoop.hh240 constructor0, written by MixedAir965.
    pub(crate) number_outside_air_systems: i32,
    /// Actual OutsideAirSys EPVector constructor-unallocated.
    pub(crate) outside_air_systems: Option<Vec<Infallible>>,
    /// DataAirLoop.hh241 constructor false; SimSelectedEquipment writes true.
    pub(crate) inputs_filled: bool,
    /// Actual EPVector default-unallocated/empty; positive row construction is unported.
    pub(crate) control_info: Option<Vec<Infallible>>,
    /// Actual SimAirServingZones.hh215-222 constructor flags, not phase inference.
    pub(crate) get_air_loop_input_flag: bool,
    pub(crate) initialize_air_loops_one_time: bool,
    pub(crate) initialize_air_loops_branch_sizing: bool,
    pub(crate) output_setup_flag: bool,
    pub(crate) my_environment_flag: bool,
    pub(crate) number_time_steps_in_day: i32,
    /// Native declaration242 has no initializer; actual InitAirLoops1466 writes it.
    pub(crate) mass_flow_set_tolerance: Option<f64>,
    pub(crate) saved_previous_hvac_time: f64,
    pub(crate) simulation_iteration_maximum: i32,
    pub(crate) simulation_iteration_total: i32,
    pub(crate) simulation_calls_total: i32,
    /// Genuine empty-only Source EPVector owners; positive row constructors unavailable.
    pub(crate) primary_air_systems: Option<Vec<Infallible>>,
    pub(crate) air_to_zone_node_info: Option<Vec<Infallible>>,
    pub(crate) air_loop_zone_info: Option<Vec<Infallible>>,
    pub(crate) air_to_oa_node_info: Option<Vec<Infallible>>,
    pub(crate) flow: Option<Vec<Infallible>>,
    pub(crate) afn_info: Option<Vec<Infallible>>,
    /// Sole AirLoopHVACDOASData::airloopDOAS actual std::vector constructor-empty.
    pub(crate) dedicated_outdoor_air_systems: Vec<Infallible>,
    // Actual six DataAirSystems Array1D connection owners, constructor-unallocated.
    pub(crate) zone_comp_to_plant: Option<Vec<Infallible>>,
    pub(crate) zone_sub_comp_to_plant: Option<Vec<Infallible>>,
    pub(crate) zone_sub_sub_comp_to_plant: Option<Vec<Infallible>>,
    pub(crate) air_sys_comp_to_plant: Option<Vec<Infallible>>,
    pub(crate) air_sys_sub_comp_to_plant: Option<Vec<Infallible>>,
    pub(crate) air_sys_sub_sub_comp_to_plant: Option<Vec<Infallible>>,
}
impl Default for SourceAirLoopData {
    fn default() -> Self {
        Self {
            number_outside_air_systems: 0,
            outside_air_systems: None,
            inputs_filled: false,
            control_info: None,
            get_air_loop_input_flag: true,
            initialize_air_loops_one_time: true,
            initialize_air_loops_branch_sizing: true,
            output_setup_flag: false,
            my_environment_flag: true,
            number_time_steps_in_day: 0,
            mass_flow_set_tolerance: None,
            saved_previous_hvac_time: 0.0,
            simulation_iteration_maximum: 0,
            simulation_iteration_total: 0,
            simulation_calls_total: 0,
            primary_air_systems: None,
            air_to_zone_node_info: None,
            air_loop_zone_info: None,
            air_to_oa_node_info: None,
            flow: None,
            afn_info: None,
            dedicated_outdoor_air_systems: Vec::new(),
            zone_comp_to_plant: None,
            zone_sub_comp_to_plant: None,
            zone_sub_sub_comp_to_plant: None,
            air_sys_comp_to_plant: None,
            air_sys_sub_comp_to_plant: None,
            air_sys_sub_sub_comp_to_plant: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSetPointManager {
    pub(crate) manager_on: bool,
    pub(crate) get_input_flag: bool,
    pub(crate) init_one_time_flag: bool,
    pub(crate) init_one_time_flag2: bool,
    pub(crate) no_ground_temp_object_warning: [bool; 4],
    pub(crate) environment_flag: bool,
    pub(crate) managers: Option<Vec<Infallible>>,
    pub(crate) manager_map: std::collections::BTreeMap<String, i32>,
}
impl Default for SourceSetPointManager {
    fn default() -> Self {
        Self {
            manager_on: false,
            get_input_flag: true,
            init_one_time_flag: true,
            init_one_time_flag2: true,
            no_ground_temp_object_warning: [true; 4],
            environment_flag: true,
            managers: None,
            manager_map: Default::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourcePlantManager {
    /// DataPlant.hh411-420; PlantLoop remains the existing Startup-owned array.
    pub(crate) total_loops: i32,
    pub(crate) total_half_loops: i32,
    pub(crate) manage_sub_iterations: i32,
    pub(crate) manage_half_loop_calls: i32,
    /// PlantManager.hh176; actual ReInit writer, independent of the HVAC flag.
    pub(crate) environment_flag: bool,
}
impl Default for SourcePlantManager {
    fn default() -> Self {
        Self {
            total_loops: 0,
            total_half_loops: 0,
            manage_sub_iterations: 0,
            manage_half_loop_calls: 0,
            environment_flag: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceCoolingDxModule {
    pub(crate) coils: Vec<Infallible>,
    pub(crate) get_input_flag: bool,
    pub(crate) object_name: String,
    /// Coils/CoilCoolingDX.hh190; ReportAllStandardRatings is a genuine later child.
    pub(crate) still_need_to_report_standard_ratings: bool,
}
impl Default for SourceCoolingDxModule {
    fn default() -> Self {
        Self {
            coils: Vec::new(),
            get_input_flag: true,
            object_name: "Coil:Cooling:DX".into(),
            still_need_to_report_standard_ratings: true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceHvacSimulationModules {
    pub(crate) air_loop: SourceAirLoopData,
    pub(crate) plant: SourcePlantManager,
    pub(crate) setpoints: SourceSetPointManager,
    pub(crate) cooling_dx: SourceCoolingDxModule,
}
