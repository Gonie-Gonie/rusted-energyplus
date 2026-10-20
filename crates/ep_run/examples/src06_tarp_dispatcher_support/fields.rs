//! Read-only actual retained dispatcher owners and real borrowed backend transport.
use ep_model::{ReportInputs, SurfaceId, ZoneId};
use ep_runtime::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorState, SourceErrorTracking,
};
use ep_runtime::heat_balance::inside_convection_dispatcher::*;
use ep_runtime::heat_balance::inside_convection_layout::InsideConvectionLayout;
use ep_runtime::heat_balance::report_scan::ReportScanState;
use serde_json::{Value, json};
use std::cell::RefCell;
use std::rc::Rc;

pub(super) const OBSERVER_PATHS: [&str; 3] = [
    "crates/ep_run/examples/src06_tarp_dispatcher_main.rs",
    "crates/ep_run/examples/src06_tarp_dispatcher_support/fields.rs",
    "crates/ep_runtime/examples/clk02_probe_support/digest.rs",
];
pub(super) const API_PATHS: [&str; 15] = [
    "crates/ep_runtime/src/heat_balance/inside_convection_layout.rs",
    "crates/ep_runtime/src/heat_balance/inside_convection_dispatcher.rs",
    "crates/ep_runtime/src/heat_balance/inside_convection_dispatcher/geometry.rs",
    "crates/ep_runtime/src/heat_balance/convection.rs",
    "crates/ep_runtime/src/heat_balance/convection/tarp_inside.rs",
    "crates/ep_runtime/src/heat_balance/report_scan.rs",
    "crates/ep_runtime/src/heat_balance/error_backend.rs",
    "crates/ep_model/src/ids.rs",
    "crates/ep_model/src/objects/building.rs",
    "crates/ep_model/src/objects/surfaces.rs",
    "crates/ep_model/src/report_inputs.rs",
    "crates/ep_raw_model/src/lib.rs",
    "crates/ep_raw_model/src/idf_order.rs",
    "crates/ep_compiler/src/compiler.rs",
    "crates/ep_compiler/src/compiler/report_inputs.rs",
];
pub(super) const UNAVAILABLE: [&str; 8] = [
    "Native EnergyPlusData fresh constructor and InputProcessor semantics",
    "actual linked Native inner surface invocation/branch trace",
    "upstream geometry and space-MAT producers; ordinary clock/cadence/handoff",
    "unselected Native zone/space/surface/node/global owner members",
    "Details/advanced static geometry EIO writer",
    "adaptive/window/Kiva/resimulation/representative/user/EMS/radiant physics",
    "Native version/IDD/current-time strings and input-process/disposal transport",
    "generated nonfinite derived metadata or coefficient is diagnostic, never PASS",
];

// Exact scalar copier used by the existing CTF observation transport.
pub(super) fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value.is_infinite() {
        if value.is_sign_negative() {
            "negative_infinity"
        } else {
            "positive_infinity"
        }
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value":if value.is_finite() { Some(value) } else { None },"value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}
pub(super) fn reports(reports: &ReportInputs) -> Value {
    json!({"objects":reports.objects.iter().map(|v|json!({"object_name":v.object_name,"family":v.object_type.source_name(),"alphas":v.alphas,
        "alpha_blank":v.alpha_blank,"default_applied":v.default_applied,"num_alphas":v.num_alphas,"is_epjson":v.is_epjson})).collect::<Vec<_>>(),
        "legacy_variable_dictionary_sections":reports.legacy_variable_dictionary_sections})
}
fn cache(s: &ReportScanState) -> Value {
    json!({"GetReportInput":s.get_report_input,"SurfVert":s.surface_vertices,"SurfDet":s.surface_details,"SurfDetWVert":s.surface_details_with_vertices,
        "DXFReport":s.dxf,"DXFWFReport":s.dxf_wireframe,"VRMLReport":s.vrml,"CostInfo":s.cost_info,"ViewFactorInfo":s.view_factor_info,
        "Constructions":s.constructions,"Materials":s.materials,"LineRpt":s.lines,"VarDict":s.variable_dictionary,"EMSoutput":s.ems_output,
        "DXFOption1":s.dxf_options[0],"DXFOption2":s.dxf_options[1],"DXFWFOption1":s.dxf_wireframe_options[0],"DXFWFOption2":s.dxf_wireframe_options[1],
        "VRMLOption1":s.vrml_options[0],"VRMLOption2":s.vrml_options[1],"ViewRptOption1":s.view_factor_option,"LineRptOption1":s.line_option,
        "VarDictOption1":s.variable_dictionary_options[0],"VarDictOption2":s.variable_dictionary_options[1],"ShowDecayCurvesInEIO":s.show_decay_curves_in_eio,
        "OutputEMSActuatorAvailSmall":s.ems_actuator_availability[0],"OutputEMSActuatorAvailFull":s.ems_actuator_availability[1],
        "OutputEMSInternalVarsSmall":s.ems_internal_variable_availability[0],"OutputEMSInternalVarsFull":s.ems_internal_variable_availability[1],
        "OutputEMSErrors":s.ems_errors,"OutputFullEMSTrace":s.ems_full_trace})
}
fn backend(s: &SourceErrorState) -> Value {
    let t = &s.tracking;
    json!({"TotalWarningErrors":t.total_warning_errors,"TotalSevereErrors":t.total_severe_errors,"TotalWarningErrorsDuringWarmup":t.total_warning_errors_during_warmup,
        "TotalSevereErrorsDuringWarmup":t.total_severe_errors_during_warmup,"TotalWarningErrorsDuringSizing":t.total_warning_errors_during_sizing,
        "TotalSevereErrorsDuringSizing":t.total_severe_errors_during_sizing,"LastSevereError":t.last_severe_error,"AbortProcessing":t.abort_processing,
        "MatchCounts":t.match_counts,"outputErrorHeader":s.output_error_header,"DoingInputProcessing":s.doing_input_processing,"printConsoleOutput":s.print_console_output,
        "KickOffSimulation":s.kickoff_simulation,"DeveloperFlag":s.developer_flag,"WarmupFlag":s.warmup_flag,"DoingSizing":s.doing_sizing,
        "errorCallback_present":true,"messageCallback_present":true,"fMessagePtr_present":false,"sqlite_present":false,
        "VerStringVar":s.version_string,"IDDVerString":s.idd_version_string,"Native_header_strings_paired":false})
}
pub(super) struct Caller {
    pub(super) scan: ReportScanState,
    state: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
    eio: Vec<u8>,
    callbacks: Rc<RefCell<Vec<Value>>>,
}
impl Caller {
    pub(super) fn new(context: &Value) -> super::Result<Self> {
        super::keys(
            context,
            &[
                "version_string",
                "idd_version_string",
                "doing_input_processing",
                "print_console_output",
                "kickoff_simulation",
                "developer_flag",
                "warmup_flag",
                "doing_sizing",
            ],
        )?;
        super::require(
            context["doing_input_processing"] == false
                && context["print_console_output"] == true
                && context["kickoff_simulation"] == false
                && context["developer_flag"] == false
                && context["warmup_flag"] == false
                && context["doing_sizing"] == false,
            "selected actual constructor backend flag context required",
        )?;
        Ok(Self {
            scan: ReportScanState::default(),
            state: SourceErrorState {
                output_error_header: true,
                version_string: super::text(&context["version_string"])?.to_owned(),
                idd_version_string: super::text(&context["idd_version_string"])?.to_owned(),
                doing_input_processing: super::boolean(&context["doing_input_processing"])?,
                print_console_output: super::boolean(&context["print_console_output"])?,
                kickoff_simulation: super::boolean(&context["kickoff_simulation"])?,
                developer_flag: super::boolean(&context["developer_flag"])?,
                warmup_flag: super::boolean(&context["warmup_flag"])?,
                doing_sizing: super::boolean(&context["doing_sizing"])?,
                tracking: SourceErrorTracking {
                    ask_for_plant_check_on_abort: false,
                    exit_during_simulations: false,
                    ask_for_connections_report: false,
                    ask_for_surfaces_report: false,
                    total_multiplied_windows: 0,
                    total_coincident_vertices: 0,
                    total_degenerate_surfaces: 0,
                    total_receiving_non_convex_surfaces: 0,
                    total_casting_non_convex_surfaces: 0,
                    match_counts: [0; 20],
                    abort_processing: false,
                    total_severe_errors: 0,
                    total_severe_errors_during_warmup: 0,
                    total_severe_errors_during_sizing: 0,
                    last_severe_error: String::new(),
                    total_warning_errors: 0,
                    total_warning_errors_during_warmup: 0,
                    total_warning_errors_during_sizing: 0,
                },
            },
            error: Vec::new(),
            console: Vec::new(),
            eio: Vec::new(),
            callbacks: Rc::new(RefCell::new(Vec::new())),
        })
    }
    #[allow(clippy::too_many_arguments)] // Preserve the actual selected API's independent caller operands.
    pub(super) fn invoke(
        &mut self,
        layout: &InsideConvectionLayout,
        zones: &[InsideConvectionZone],
        surfaces: &[InsideConvectionSurface],
        no_user: NoUserConvectionInput,
        state: &mut InsideConvectionState,
        active_elements: &mut [(ZoneId, [bool; 3])],
        coefficients: &mut [(SurfaceId, f64)],
        space_air: &SpaceAirTemperatures,
        temperatures: InsideSurfaceTemperatures<'_>,
        readiness: InsideConvectionReadiness,
        actual_low_limit: f64,
        reports: &ReportInputs,
    ) -> std::result::Result<(), InsideConvectionError> {
        let error_log = Rc::clone(&self.callbacks);
        let message_log = Rc::clone(&self.callbacks);
        let mut error_callback =
            move |_: &mut SourceErrorState,
                  level: ep_runtime::heat_balance::error_backend::SourceErrorLevel,
                  message: &str| {
                let mut rows = error_log.borrow_mut();
                let ordinal = rows.len() + 1;
                rows.push(json!({"actual_ordinal":ordinal,"kind":"errorCallback","level":level as i32,"message":message}));
            };
        let mut message_callback = move |_: &mut SourceErrorState, message: &str| {
            let mut rows = message_log.borrow_mut();
            let ordinal = rows.len() + 1;
            rows.push(json!({"actual_ordinal":ordinal,"kind":"messageCallback","level":null,"message":message}));
        };
        let mut backend = SourceErrorBackend {
            state: &mut self.state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: Some(&mut message_callback),
            error_callback: Some(&mut error_callback),
            sqlite: None,
        };
        initialize_inside_convection(
            layout,
            zones,
            surfaces,
            no_user,
            state,
            active_elements,
            coefficients,
            space_air,
            temperatures,
            readiness,
            actual_low_limit,
            reports,
            &mut self.scan,
            &mut backend,
        )
    }
}
fn bytes(value: &[u8]) -> Value {
    json!({"size_bytes":value.len(),"hex":value.iter().map(|v|format!("{v:02x}")).collect::<String>(),"utf8":std::str::from_utf8(value).ok()})
}
fn objexx<T>(rows: &[T], mut copy: impl FnMut(&T) -> Value) -> Value {
    json!({"representation":"Rust owned Vec; selected Source one-based transport","allocated":true,"size":rows.len(),"lower":1,"upper":rows.len(),"values_available":true,
        "rows":rows.iter().enumerate().map(|(i,v)|json!({"actual_array_index":i+1,"value":copy(v)})).collect::<Vec<_>>()})
}
fn vector<T>(rows: &[T], mut copy: impl FnMut(&T) -> Value) -> Value {
    json!({"representation":"Rust owned Vec; selected EPVector transport","allocated":true,"size":rows.len(),"indexing":"EPVector one-based",
        "rows":rows.iter().enumerate().map(|(i,v)|json!({"actual_array_index":i+1,"value":copy(v)})).collect::<Vec<_>>()})
}
fn empty_user(owner: &Option<Vec<AbsentUserConvectionRecord>>, objexx: bool) -> Value {
    let allocated = owner.is_some();
    let size = owner.as_ref().map_or(0, Vec::len);
    if objexx {
        json!({"representation":"Rust Option<Vec<uninhabited>>; explicit empty Objexx shape transport","allocated":allocated,"size":size,
        "lower":if allocated {Some(1)} else {None},"upper":if allocated {Some(0)} else {None},"values_available":true,"rows":[]})
    } else {
        json!({"representation":"Rust Option<Vec<uninhabited>>; explicit empty EPVector shape transport","allocated":allocated,"size":size,"rows":[],"nonempty_rows_available":false,"indexing":"EPVector one-based"})
    }
}
#[allow(clippy::too_many_arguments)] // Copy independent actual owners without a fabricated combined state.
pub(super) fn snapshot(
    layout: &InsideConvectionLayout,
    zones: &[InsideConvectionZone],
    surfaces: &[InsideConvectionSurface],
    state: &InsideConvectionState,
    active_elements: &[(ZoneId, [bool; 3])],
    coefficients: &[(SurfaceId, f64)],
    space_air: &SpaceAirTemperatures,
    zone_mat: f64,
    actual_low_limit: f64,
    temp_in: &[(SurfaceId, f64)],
    temp_tmp: &[(SurfaceId, f64)],
    readiness: InsideConvectionReadiness,
    caller: &Caller,
) -> Value {
    let int_conv = |s: &InsideConvectionSurfaceState| {
        json!({"convClass":s.interior_classification[0],"convClassRpt":s.interior_classification[1],"model":s.model.source_id(),
        "userModelNum":s.interior_classification[2],"hcModelEq":s.interior_classification[3],"hcModelEqRpt":s.interior_classification[4],"hcUserCurveNum":s.interior_classification[5],
        "zoneWallHeight":scalar(s.interior_geometry[0]),"zonePerimLength":scalar(s.interior_geometry[1]),"zoneHorizHydrDiam":scalar(s.interior_geometry[2]),"windowWallRatio":scalar(s.interior_geometry[3]),
        "windowLocation":s.window_location,"getsRadiantHeat":s.radiant_and_active[0],"hasActiveInIt":s.radiant_and_active[1]})
    };
    let ext_conv = |s: &InsideConvectionSurfaceState| {
        json!({"convClass":s.exterior_classification[0],"convClassRpt":s.exterior_classification[1],"model":s.exterior_classification[2],
        "userModelNum":s.exterior_classification[3],"hfModelEq":s.exterior_classification[4],"hfModelEqRpt":s.exterior_classification[5],"hfUserCurveNum":s.exterior_classification[6],
        "hnModelEq":s.exterior_classification[7],"hnModelEqRpt":s.exterior_classification[8],"hnUserCurveNum":s.exterior_classification[9],
        "faceArea":scalar(s.exterior_geometry[0]),"facePerimeter":scalar(s.exterior_geometry[1]),"faceHeight":scalar(s.exterior_geometry[2])})
    };
    let callbacks = caller.callbacks.borrow();
    json!({"owner":{"NumOfZones":layout.zones().len(),"numSpaces":layout.spaces().len(),"TotSurfaces":layout.surfaces().len(),
        "BeginEnvrnFlag":readiness.begin_environment,"SysSizingCalc":readiness.system_sizing,"ZoneSizingCalc":readiness.zone_sizing,
        "ZoneEquipInputsFilled":readiness.equipment_inputs_filled,"ZoneEquipSimulatedOnce":readiness.equipment_simulated_once,
        "ordered_layout":{"zones":layout.zones().iter().map(|v|v.0).collect::<Vec<_>>(),"spaces":layout.spaces().iter().map(|v|json!([v.0.0,v.1.0])).collect::<Vec<_>>(),
            "surface_associations":layout.surfaces().iter().map(|v|json!([v.surface_id.0,v.zone_id.0,v.space_id.0])).collect::<Vec<_>>(),
            "groups":layout.groups().iter().map(|v|json!({"zone":v.zone_id.0,"space":v.space_id.0,"surfaces":v.surfaces.iter().map(|s|s.0).collect::<Vec<_>>()})).collect::<Vec<_>>()},
        "typed_zones":zones.iter().map(|z|json!({"zone_id":z.zone_id.0,"tarp":z.inside_algorithm == ep_model::InsideSurfaceConvectionAlgorithm::Tarp,"CeilingHeight":scalar(z.ceiling_height),"Volume":scalar(z.volume),"FloorArea":scalar(z.floor_area),
            "MinimumX":scalar(z.xy_bounds[0]),"MaximumX":scalar(z.xy_bounds[1]),"MinimumY":scalar(z.xy_bounds[2]),"MaximumY":scalar(z.xy_bounds[3]),
            "ExtGrossWallArea":scalar(z.exterior_gross_wall_area),"ExtWindowArea":scalar(z.exterior_window_area),"Multiplier":z.multiplier,"ListMultiplier":z.list_multiplier})).collect::<Vec<_>>(),
        "typed_surfaces":surfaces.iter().map(|s|json!({"surface_id":s.surface_id.0,"class":match s.class {ep_model::SurfaceType::Wall=>"Wall",ep_model::SurfaceType::Roof=>"Roof",ep_model::SurfaceType::Ceiling=>"Ceiling",ep_model::SurfaceType::Floor=>"Floor"},
            "exterior":s.exterior,"heat_transfer":s.heat_transfer,"construction_is_window":s.construction_is_window,"kiva":s.kiva,"representative_calculations":s.representative_calculations,"ems_override":s.ems_override,
            "CosTilt":scalar(s.outward_cos_tilt),"Tilt":scalar(s.tilt),"Azimuth":scalar(s.azimuth),"Area":scalar(s.area),"GrossArea":scalar(s.gross_area),"Width":scalar(s.width),"Perimeter":scalar(s.perimeter),
            "Vertex":s.vertices.iter().map(|v|json!({"x":scalar(v[0]),"y":scalar(v[1]),"z":scalar(v[2])})).collect::<Vec<_>>()})).collect::<Vec<_>>(),
        "zoneHeatBalance_MAT_input_owner":scalar(zone_mat),"spaceHeatBalance":vector(space_air.rows(), |s|json!({"MAT":scalar(s.mat),"ZT":scalar(s.zt),"ZTAV":scalar(s.ztav)})),
        "SurfTempIn":objexx(temp_in, |v|scalar(v.1)),"SurfTempInTmp":objexx(temp_tmp, |v|scalar(v.1)),"SurfHConvInt":objexx(coefficients, |v|scalar(v.1)),
        "SurfTAirRef":objexx(&state.surfaces, |v|json!(v.reference_air[0])),"SurfTAirRefRpt":objexx(&state.surfaces, |v|json!(v.reference_air[1])),
        "surfIntConv":vector(&state.surfaces,int_conv),"surfExtConv":vector(&state.surfaces,ext_conv),"LowHConvLimit":scalar(actual_low_limit),
        "GetUserSuppliedConvectionCoeffs":state.get_user_input,"NodeCheck":state.node_check,"ActiveSurfaceCheck":state.active_surface_check,
        "MyEnvirnFlag":state.my_environment,"FirstRoofSurf":state.first_roof_surface,"CubeRootOfOverallBuildingVolume":scalar(state.cube_root_building_volume),
        "RoofLongAxisOutwardAzimuth":scalar(state.roof_long_axis_outward_azimuth),"TotUserIntConvModels":state.user_model_counts[0],"TotUserExtConvModels":state.user_model_counts[1],
        "hcIntUserCurve":empty_user(&state.user_arrays[0],true),"hcExtUserCurve":empty_user(&state.user_arrays[1],true),"userIntConvModels":empty_user(&state.user_arrays[2],false),"userExtConvModels":empty_user(&state.user_arrays[3],false),
        "ZoneEquipConfig_selected_members":objexx(active_elements,|v|json!({"InWallActiveElement":v.1[0],"InCeilingActiveElement":v.1[1],"InFloorActiveElement":v.1[2]})),
        "Node_allocated_input_owner":readiness.nodes_allocated,"Scan_cache":cache(&caller.scan),"backend":backend(&caller.state)},
        "error_stream":bytes(&caller.error),"console":bytes(&caller.console),"eio":bytes(&caller.eio),"callbacks":&*callbacks,"actual_callback_count":callbacks.len(),
        "inner_surface_visit_trace_available":false,"unavailable_owner_families":UNAVAILABLE})
}
