//! Read-only copies of actual compiled geometry and returned owner state.
use ep_model::SimulationModel;
use ep_runtime::error::RuntimeError;
use ep_runtime::heat_balance::ctf_initialization_context::{
    CtfInitializationContext, CtfInitializationReports,
};
use ep_runtime::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorState, SourceErrorTracking,
};
use ep_runtime::heat_balance::{
    HeatBalanceState, initialize_heat_balance_state_with_initialization_context,
};
use serde_json::{Value, json};
include!("../../../ep_runtime/src/heat_balance/predefined_tables_names.rs");
pub(super) const OBSERVER_PATHS: &[&str] = &[
    "crates/ep_run/examples/src06_ordinary_geometry_observer.rs",
    "crates/ep_run/examples/src06_ordinary_geometry_support/fields.rs",
    "crates/ep_runtime/examples/clk02_probe_support/digest.rs",
];
pub(super) const API_PATHS: &[&str] = &[
    "crates/ep_run/src/lib.rs",
    "crates/ep_run/src/porting_scope.rs",
    "crates/ep_run/src/porting_scope/settings.rs",
    "crates/ep_run/src/porting_scope/environment.rs",
    "crates/ep_raw_model/src/lib.rs",
    "crates/ep_raw_model/src/idf_order.rs",
    "crates/ep_compiler/src/lib.rs",
    "crates/ep_compiler/src/compiler.rs",
    "crates/ep_model/src/model.rs",
    "crates/ep_model/src/objects/building.rs",
    "crates/ep_model/src/objects/surfaces.rs",
    "crates/ep_model/src/units.rs",
    "crates/ep_runtime/src/error.rs",
    "crates/ep_runtime/src/geometry.rs",
    "crates/ep_runtime/src/geometry/source_geometry.rs",
    "crates/ep_runtime/src/geometry/zone_volume.rs",
    "crates/ep_runtime/src/geometry/inside_convection_owners.rs",
    "crates/ep_runtime/src/heat_balance/inside_convection_layout.rs",
    "crates/ep_runtime/src/heat_balance/state.rs",
    "crates/ep_runtime/src/heat_balance/initialization.rs",
    "crates/ep_runtime/src/heat_balance/initialization/schedule_cache.rs",
    "crates/ep_runtime/src/heat_balance/initialization/state_shell.rs",
    "crates/ep_runtime/src/heat_balance/ctf_initialization_context.rs",
    "crates/ep_runtime/src/heat_balance/error_backend.rs",
    "crates/ep_runtime/src/heat_balance/report_scan.rs",
    "crates/ep_runtime/src/heat_balance/predefined_tables.rs",
    "crates/ep_runtime/src/heat_balance/predefined_tables_names.rs",
    "crates/ep_run/src/ctf_initialization_outputs.rs",
];
fn allocated<T>(owner: &Option<Vec<T>>, mut copy: impl FnMut(&T) -> Value) -> Value {
    match owner {
        None => json!({"allocated":false,"length":null,"values":null}),
        Some(values) => json!({"allocated":true,"length":values.len(),
            "values":values.iter().enumerate().map(|(index,value)|json!({
                "vector_index":index,"source_one_based_index":index+1,"value":copy(value)
            })).collect::<Vec<_>>()}),
    }
}
fn bytes(owner: &[u8]) -> Value {
    json!({"size_bytes":owner.len(),"hex":owner.iter().map(|v|format!("{v:02x}")).collect::<String>(),
        "utf8":std::str::from_utf8(owner).ok()})
}
fn table_entries(
    owner: &Option<Vec<ep_runtime::heat_balance::ctf_reports::PredefinedTableEntry>>,
) -> Result<Value, std::str::Utf8Error> {
    match owner {
        None => Ok(json!({"allocated":false,"length":null,"values":null})),
        Some(values) => {
            let rows = values.iter().enumerate().map(|(index, v)| {
                let char_entry = std::str::from_utf8(&v.char_entry)?;
                Ok(json!({"vector_index":index,"source_one_based_index":index+1,
                    "value":json!({"char_entry":char_entry,
            "object_name":v.object_name,"index_column":v.index_column,"sub_table_index":v.sub_table_index,
            "unique_object_name":v.unique_object_name,"original_real_entry":scalar(v.original_real_entry),
            "significant_digits":v.significant_digits,"original_entry_is_real":v.original_entry_is_real})}))
            }).collect::<Result<Vec<_>, std::str::Utf8Error>>()?;
            Ok(json!({"allocated":true,"length":values.len(),"values":rows}))
        }
    }
}
fn reports(source: &CtfInitializationReports) -> Result<Value, std::str::Utf8Error> {
    let registry = &source.registry;
    let table = &source.table;
    let scan = &source.scan;
    let progress = &source.progress;
    Ok(json!({"registry":{
        "named_indices":registry.named_indices.iter().map(|(name,value)|json!({"name":name,"value":value})).collect::<Vec<_>>(),
        "declared_named_indices":NATIVE_INDEX_NAMES.iter().enumerate().map(|(index,name)|json!({
            "declaration_index":index,"name":name,"value":registry.named_indices.get(*name)
        })).collect::<Vec<_>>(),
        "named_index_owner_count":registry.named_indices.len(),
        "reports":allocated(&registry.reports,|v|json!({"name":v.name,"name_with_spaces":v.name_with_spaces,"abbreviation":v.abbreviation,"show":v.show})),
        "report_capacity":registry.report_capacity,"report_count":registry.report_count,
        "subtables":allocated(&registry.subtables,|v|json!({"name":v.name,"report_index":v.report_index,"footnote":v.footnote})),
        "subtable_capacity":registry.subtable_capacity,"subtable_count":registry.subtable_count,
        "columns":allocated(&registry.columns,|v|json!({"heading":v.heading,"subtable_index":v.subtable_index})),
        "column_capacity":registry.column_capacity,"column_count":registry.column_count,
        "opaque_layer_columns":registry.opaque_layer_columns,
        "unrelated_Native_aggregate_owners_available":false},
        "table":{"entries":table_entries(&table.entries)?,
            "size_table_entry":table.size_table_entry,"num_table_entry":table.num_table_entry},
        "scan":{"get_report_input":scan.get_report_input,"surface_vertices":scan.surface_vertices,
            "surface_details":scan.surface_details,"surface_details_with_vertices":scan.surface_details_with_vertices,
            "lines":scan.lines,"line_option":scan.line_option,"cost_info":scan.cost_info,
            "view_factor_info":scan.view_factor_info,"view_factor_option":scan.view_factor_option,
            "dxf":scan.dxf,"dxf_options":scan.dxf_options,"dxf_wireframe":scan.dxf_wireframe,
            "dxf_wireframe_options":scan.dxf_wireframe_options,"vrml":scan.vrml,"vrml_options":scan.vrml_options,
            "variable_dictionary":scan.variable_dictionary,"variable_dictionary_options":scan.variable_dictionary_options,
            "constructions":scan.constructions,"materials":scan.materials,"ems_output":scan.ems_output,
            "show_decay_curves_in_eio":scan.show_decay_curves_in_eio,
            "ems_actuator_availability":scan.ems_actuator_availability,
            "ems_internal_variable_availability":scan.ems_internal_variable_availability,
            "ems_errors":scan.ems_errors,"ems_full_trace":scan.ems_full_trace},
        "progress":{"layer_report_declared_indices":progress.layer_report_declared_indices,
            "construction_scan_result":progress.construction_scan_result,
            "report_headers_written":progress.report_headers_written,
            "coefficient_report_declared_indices":progress.coefficient_report_declared_indices,
            "final_fatal_invoked":progress.final_fatal_invoked,"postlude_returned":progress.postlude_returned}}))
}
fn backend(source: &SourceErrorState) -> Value {
    let t = &source.tracking;
    json!({"output_error_header":source.output_error_header,"version_string":source.version_string,
        "idd_version_string":source.idd_version_string,"doing_input_processing":source.doing_input_processing,
        "print_console_output":source.print_console_output,"kickoff_simulation":source.kickoff_simulation,
        "developer_flag":source.developer_flag,"warmup_flag":source.warmup_flag,"doing_sizing":source.doing_sizing,
        "tracking":{"match_counts":t.match_counts,"abort_processing":t.abort_processing,
            "total_severe_errors":t.total_severe_errors,"total_severe_errors_during_warmup":t.total_severe_errors_during_warmup,
            "total_severe_errors_during_sizing":t.total_severe_errors_during_sizing,"last_severe_error":t.last_severe_error,
            "total_warning_errors":t.total_warning_errors,"total_warning_errors_during_warmup":t.total_warning_errors_during_warmup,
            "total_warning_errors_during_sizing":t.total_warning_errors_during_sizing},
        "error_stream_present":true,"console_present":true,"out_unit1_present":false,"out_unit2_present":false,
        "f_message_present":false,"message_callback_present":false,"error_callback_present":false,"sqlite_present":false,
        "Native_header_strings_or_optional_backend_parity_claimed":false,
        "Native_registry_real_totals_component_or_shadow_owners_paired":false})
}

pub(super) struct Caller {
    pub(super) reports: CtfInitializationReports,
    state: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
    eio: Vec<u8>,
}
impl Caller {
    pub(super) fn new(version: &str) -> Self {
        // Same authored Rust ordinary caller initialization as CtfRunOutputs::new.
        // These are not Native state/answers, and optional adapters remain absent.
        Self {
            reports: CtfInitializationReports::default(),
            state: SourceErrorState {
                output_error_header: true,
                version_string: concat!("rusted-energyplus ", env!("CARGO_PKG_VERSION")).into(),
                idd_version_string: format!("Rust typed input version {version}"),
                doing_input_processing: false,
                print_console_output: true,
                kickoff_simulation: false,
                developer_flag: false,
                warmup_flag: false,
                doing_sizing: false,
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
                    last_severe_error: String::new(),
                    total_severe_errors_during_warmup: 0,
                    total_severe_errors_during_sizing: 0,
                    total_warning_errors: 0,
                    total_warning_errors_during_warmup: 0,
                    total_warning_errors_during_sizing: 0,
                },
            },
            error: Vec::new(),
            console: Vec::new(),
            eio: Vec::new(),
        }
    }
    pub(super) fn snapshot(&self) -> Result<Value, std::str::Utf8Error> {
        Ok(json!({"reports":reports(&self.reports)?,"backend":backend(&self.state),
            "prefixes":{"error":bytes(&self.error),"console":bytes(&self.console),"eio":bytes(&self.eio)},
            "actual_Rust_caller_owner_copied":true,"Native_intermediate_checkpoint_inferred":false}))
    }
    fn context(&mut self) -> CtfInitializationContext<'_> {
        CtfInitializationContext {
            reports: &mut self.reports,
            eio: &mut self.eio,
            backend: SourceErrorBackend {
                state: &mut self.state,
                error_stream: Some(&mut self.error),
                console: &mut self.console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        }
    }
    pub(super) fn initialize(
        &mut self,
        model: &SimulationModel,
    ) -> Result<HeatBalanceState, RuntimeError> {
        // Existing ordinary runtime constructor input; it is not geometry or a Native answer.
        initialize_heat_balance_state_with_initialization_context(model, 23.0, &mut self.context())
    }
}

pub(super) fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "NaN"
    } else if value == f64::INFINITY {
        "+Infinity"
    } else if value == f64::NEG_INFINITY {
        "-Infinity"
    } else if value == 0.0 && value.is_sign_negative() {
        "-0"
    } else if value == 0.0 {
        "+0"
    } else {
        "finite"
    };
    json!({"value":if value.is_finite() {Some(value)} else {None},"value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}
fn vector(value: [f64; 3]) -> Value {
    json!(value.map(scalar))
}
fn point(value: ep_model::Point3) -> Value {
    vector([value.x_m, value.y_m, value.z_m])
}
fn auto(value: ep_model::AutoOrNumber) -> Value {
    match value {
        ep_model::AutoOrNumber::AutoCalculate => {
            json!({"selector":"AutoCalculate","numeric_value":null})
        }
        ep_model::AutoOrNumber::Value(value) => {
            json!({"selector":"Value","numeric_value":scalar(value)})
        }
    }
}
fn raw_value(value: &ep_raw_model::RawValue) -> Value {
    use ep_raw_model::RawValue;
    match value {
        RawValue::Null => json!({"kind":"null"}),
        RawValue::Bool(v) => json!({"kind":"bool","value":v}),
        RawValue::Number(v) => json!({"kind":"number","lexical_value":v}),
        RawValue::String(v) => json!({"kind":"string","value":v}),
        RawValue::Array(v) => {
            json!({"kind":"array","values":v.iter().map(raw_value).collect::<Vec<_>>()})
        }
        RawValue::Object(v) => {
            json!({"kind":"object","fields":v.iter().map(|(k,v)|json!({"name":k.0,"value":raw_value(v)})).collect::<Vec<_>>()})
        }
    }
}
pub(super) fn raw(raw: &ep_raw_model::RawModel) -> Result<Value, ep_raw_model::IdfOrderError> {
    let mut families = Vec::new();
    for kind in raw.objects.keys() {
        families.push(json!({"object_type":kind.0,"has_IDF_declaration_order":raw.has_idf_declaration_order(&kind.0),
            "actual_ordered_instances":raw.ordered_instances(&kind.0)?.iter().enumerate().map(|(index,(name,object))|json!({
                "vector_index":index,"name":name.0,"source_span":object.source_span.as_ref().map(|s|json!({"line":s.line,"column":s.column})),
                "fields":object.fields.iter().map(|(name,value)|json!({"name":name.0,"value":raw_value(value)})).collect::<Vec<_>>()
            })).collect::<Vec<_>>()}));
    }
    Ok(
        json!({"version":raw.version,"object_count":raw.object_count(),"object_type_counts":raw.object_type_counts(),"families":families}),
    )
}
pub(super) fn compile(report: &ep_compiler::CompileReport) -> Value {
    json!({"completed_stages":report.completed_stages.iter().map(|v|format!("{v:?}")).collect::<Vec<_>>(),
        "raw_object_count":report.raw_object_count,"typed_object_count":report.typed_object_count,
        "diagnostics":report.diagnostics.iter().map(|v|json!({"severity":v.severity.to_string(),"code":v.code,"object_type":v.object_type,"object_name":v.object_name,"field":v.field,"message":v.message})).collect::<Vec<_>>(),
        "defaults_applied":report.defaults_applied.iter().map(|v|json!({"object_type":v.object_type,"object_name":v.object_name,"field":v.field,"value":v.value})).collect::<Vec<_>>(),
        "coverage":report.coverage.iter().map(|v|json!({"object_type":v.object_type,"object_count":v.object_count,"status":v.status.to_string()})).collect::<Vec<_>>()})
}
pub(super) fn typed(model: &ep_model::TypedModel) -> Value {
    json!({"version":model.version.to_string(),"building_input":format!("{:?}",model.building),
        "global_geometry_rules":format!("{:?}",model.global_geometry_rules),"timestep":format!("{:?}",model.timestep),
        "zones":model.zones.iter().enumerate().map(|(index,v)|json!({"vector_index":index,"id":v.id.0,"name":v.name.0,
            "direction_of_relative_north_deg":scalar(v.direction_of_relative_north_deg),"origin":point(v.origin),"zone_type":v.zone_type,
            "multiplier":v.multiplier,"list_multiplier":v.list_multiplier,"list_group":v.list_group.map(|id|id.0),
            "ceiling_height":auto(v.ceiling_height),"volume":auto(v.volume),"floor_area":auto(v.floor_area),
            "inside_convection_algorithm":format!("{:?}",v.inside_convection_algorithm),"outside_convection_algorithm":format!("{:?}",v.outside_convection_algorithm),
            "is_part_of_total_floor_area":v.is_part_of_total_floor_area,"is_nominal_controlled":v.is_nominal_controlled,
            "linked_outdoor_air_node":v.linked_outdoor_air_node.map(|id|id.0),"spaces":v.spaces.iter().map(|id|id.0).collect::<Vec<_>>()
        })).collect::<Vec<_>>(),
        "spaces":model.spaces.iter().enumerate().map(|(index,v)|json!({"vector_index":index,"id":v.id.0,"name":v.name.0,"zone_id":v.zone.0,
            "ceiling_height":auto(v.ceiling_height),"volume":auto(v.volume),"floor_area":auto(v.floor_area),"space_type":v.space_type.0,
            "space_type_id":v.space_type_id.0,"tags":v.tags.iter().map(|s|&s.0).collect::<Vec<_>>(),"origin":format!("{:?}",v.origin)
        })).collect::<Vec<_>>(),
        "surfaces":model.surfaces.iter().enumerate().map(|(index,v)|json!({"vector_index":index,"id":v.id.0,"name":v.name.0,
            "class":format!("{:?}",v.surface_type),"zone_id":v.zone.0,"space_id":v.space.0,"construction_id":v.construction.0,
            "outside_boundary_condition":format!("{:?}",v.outside_boundary_condition),"outside_boundary_condition_object":v.outside_boundary_condition_object.as_ref().map(|v|&v.0),
            "sun_exposure":format!("{:?}",v.sun_exposure),"wind_exposure":format!("{:?}",v.wind_exposure),"view_factor_to_ground":auto(v.view_factor_to_ground),
            "vertices":v.vertices.iter().copied().map(point).collect::<Vec<_>>(),"computed_geometry_input_attachment":format!("{:?}",v.computed_geometry)
        })).collect::<Vec<_>>(),
        "constructions":model.constructions.iter().enumerate().map(|(index,v)|json!({"vector_index":index,"id":v.id.0,"name":v.name.0,"kind":format!("{:?}",v.kind),"material_layers":v.layers.iter().map(|id|id.0).collect::<Vec<_>>()})).collect::<Vec<_>>(),
        "report_inputs":model.report_inputs.objects.iter().map(|v|json!({"object_name":v.object_name,"object_type":v.object_type.source_name(),"alphas":v.alphas,"alpha_blank":v.alpha_blank,"default_applied":v.default_applied,"num_alphas":v.num_alphas,"is_epjson":v.is_epjson})).collect::<Vec<_>>(),
        "debug_fields_are_readonly_input_context_not_reconstructed_Native_owners":true})
}
fn geometry(v: &ep_runtime::geometry::SurfaceGeometryProperties) -> Value {
    json!({"area_m2":scalar(v.area_m2),"gross_area_m2":scalar(v.gross_area_m2),"net_area_shadow_m2":scalar(v.net_area_shadow_m2),
        "azimuth_deg":scalar(v.azimuth_deg),"tilt_deg":scalar(v.tilt_deg),"newell_area_vector_m2":vector(v.newell_area_vector_m2),
        "newell_normal":vector(v.newell_normal),"out_norm":vector(v.out_norm),"centroid_m":point(v.centroid_m),
        "lcsx":vector(v.lcsx),"lcsy":vector(v.lcsy),"lcsz":vector(v.lcsz),"sin_azimuth":scalar(v.sin_azimuth),
        "cos_azimuth":scalar(v.cos_azimuth),"sin_tilt":scalar(v.sin_tilt),"cos_tilt":scalar(v.cos_tilt)})
}
fn zone_return(v: &ep_runtime::geometry::ZoneGeometryProperties) -> Value {
    let d = &v.diagnostics;
    json!({"volume_m3":scalar(v.volume_m3),"ceiling_height_m":scalar(v.ceiling_height_m),"ceiling_height_entered":v.ceiling_height_entered,
        "floor_area_m2":scalar(v.floor_area_m2),"user_entered_floor_area_m2":scalar(v.user_entered_floor_area_m2),
        "geometric_floor_area_m2":scalar(v.geometric_floor_area_m2),"ceiling_area_m2":scalar(v.ceiling_area_m2),"geometric_ceiling_area_m2":scalar(v.geometric_ceiling_area_m2),
        "has_floor":v.has_floor,"has_roof":v.has_roof,"p0_m":point(v.p0_m),"volume_calculation_count":v.volume_calculation_count,
        "faces":v.faces.iter().enumerate().map(|(index,f)|json!({"vector_index":index,"surface_id":f.surface_id.0,"name":f.name,"class":format!("{:?}",f.surface_type),
            "vertices":f.vertices.iter().copied().map(point).collect::<Vec<_>>(),"area_m2":scalar(f.area_m2),"gross_area_m2":scalar(f.gross_area_m2),
            "newell_area_vector_m2":vector(f.newell_area_vector_m2),"tilt_deg":scalar(f.tilt_deg)})).collect::<Vec<_>>(),
        "diagnostics":{"initial_unique_vertex_count":d.initial_unique_vertex_count,"initially_closed":d.initially_closed,"edges_winding_consistent":d.edges_winding_consistent,
            "signed_polyhedron_volume_m3":d.signed_polyhedron_volume_m3.map(scalar),"topology_rejection":d.topology_rejection,
            "floor_horizontal":d.floor_horizontal,"roof_horizontal":d.roof_horizontal,"walls_vertical":d.walls_vertical,"same_wall_height":d.same_wall_height,
            "volume_differs_by_more_than_five_percent":d.volume_differs_by_more_than_five_percent,
            "initial_edges_not_used_twice":d.initial_edges_not_used_twice.iter().map(|e|json!({"surface_id":e.surface_id.0,"start_m":point(e.start_m),"end_m":point(e.end_m),"count":e.count,"other_surface_ids":e.other_surface_ids.iter().map(|id|id.0).collect::<Vec<_>>()})).collect::<Vec<_>>()}})
}
pub(super) fn selected(state: &ep_runtime::heat_balance::HeatBalanceState) -> Value {
    let actual = &state.initialized_geometry;
    let opaque = match &actual.opaque_setup {
        Ok(v) => {
            json!({"owner_available":true,"surface_dimensions":v.surface_dimensions.iter().enumerate().map(|(index,v)|json!({"vector_index":index,"surface_id":v.surface_id.0,
            "perimeter_m":scalar(v.perimeter_m),"width_m":scalar(v.width_m),"height_m":scalar(v.height_m)})).collect::<Vec<_>>(),
            "zones":v.zones.iter().enumerate().map(|(index,v)|json!({"vector_index":index,"zone_id":v.zone_id.0,"bounds_m":v.bounds_m.map(scalar),
                "exterior_gross_wall_area_m2":scalar(v.exterior_gross_wall_area_m2),"exterior_window_area_m2":scalar(v.exterior_window_area_m2)})).collect::<Vec<_>>() })
        }
        Err(error) => {
            json!({"owner_available":false,"surface_dimensions":null,"zones":null,"actual_error":format!("{error:?}"),"reason":error.to_string(),"counted_as_PASS":false})
        }
    };
    json!({"owner_available":true,"zone_returns":actual.zone_returns.iter().enumerate().map(|(index,v)|json!({"vector_index":index,"zone_id":v.zone_id.0,"returned":zone_return(&v.returned)})).collect::<Vec<_>>(),
        "opaque_setup":opaque,"surface_geometry":state.surfaces.iter().enumerate().map(|(index,v)|json!({"vector_index":index,"surface_id":v.surface_id.0,"zone_id":v.zone_id.0,
            "surface_name":v.surface_name,"construction_id":v.construction_id.0,"class":format!("{:?}",v.surface_type),"geometry":geometry(&v.geometry)})).collect::<Vec<_>>(),
        "actual_boundary":"returned HeatBalanceState after one initializer call","producer_calls_reexecuted_by_observer":false,"counted_as_PASS":false})
}
pub(super) fn unavailable(reason: &str) -> Value {
    json!({"owner_available":false,"zone_returns":null,"opaque_setup":null,"surface_geometry":null,"reason":reason,"counted_as_PASS":false})
}
pub(super) fn error(error: &RuntimeError) -> Value {
    let mut chain = Vec::new();
    let mut next = std::error::Error::source(error);
    while let Some(source) = next {
        chain.push(source.to_string());
        next = source.source();
    }
    json!({"returned":false,"actual_type":format!("{error:?}"),"message":error.to_string(),"source_chain":chain,"counted_as_PASS":false})
}
