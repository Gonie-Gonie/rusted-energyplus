//! Seven independent real getter/Sim admission calls; expected states are read only.
use super::*;
use hb::error_backend::SourceFatalStop;

const WORKERS: [&str; 7] = [
    "missing_availability_schedule",
    "disconnected_supply_node",
    "disconnected_exhaust_node",
    "missing_component_name",
    "invalid_high_comp_index",
    "invalid_negative_comp_index",
    "mismatched_cached_component_name",
];

fn caller_backend<'a>(
    state: &'a mut SourceErrorState,
    error: &'a mut Vec<u8>,
    console: &'a mut Vec<u8>,
) -> SourceErrorBackend<'a> {
    // The same explicitly borrowed sinks as the existing real P1 caller.
    SourceErrorBackend {
        state,
        error_stream: Some(error),
        console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    }
}

fn assert_actual_phase(source: &Factory) {
    assert_eq!(
        source.error_state.warmup_flag,
        source.weather.global.warmup_flag
    );
    assert_eq!(source.error_state.doing_sizing, source.globals.doing_sizing);
    assert_eq!(
        source.error_state.kickoff_simulation,
        source.globals.kickoff_simulation
    );
    assert_eq!(
        source.error_state.developer_flag,
        source.project.system.developer_flag
    );
}

fn observe(source: &mut Factory) -> Result<Value> {
    assert_actual_phase(source);
    let mut out = snapshot(source)?;
    out["errors"]["LastSevereError"] = json!(source.error_state.tracking.last_severe_error);
    out["errors"]["AbortProcessing"] = json!(source.error_state.tracking.abort_processing);
    out["errors"]["ExitDuringSimulations"] =
        json!(source.error_state.tracking.exit_during_simulations);
    let ems = source.globals.any_energy_management_system_in_model;
    let api = source.globals.running_via_api;
    // This is the actual call-binding view, not inferred presence from a result.
    let backend = caller_backend(
        &mut source.error_state,
        &mut source.error,
        &mut source.console,
    );
    out["diagnostic_sinks"] = json!({
        "err_stream_present":backend.error_stream.is_some(),
        "sqlite_present":backend.sqlite.is_some(),
        "error_callback_present":backend.error_callback.is_some(),
        "message_callback_present":backend.message_callback.is_some(),
        "fMessagePtr_present":backend.f_message.is_some(),
        "printConsoleOutput":backend.state.print_console_output,
        "KickOffSimulation":backend.state.kickoff_simulation,
        "DeveloperFlag":backend.state.developer_flag,
        "AnyEnergyManagementSystemInModel":ems,
        "eplusRunningViaAPI":api
    });
    Ok(out)
}

// Only Node row-address mappings have an allocator-scoped lifetime. The
// complete logical NodeID/ordinal/shape/value comparison below never resets.
#[derive(Default)]
struct NodeAllocationEpoch {
    previous: Option<(Value, Value)>,
    previous_node_ids: Option<(Value, Value)>,
    transitions: usize,
}
impl NodeAllocationEpoch {
    fn witness(state: &Value) -> Result<Value> {
        let nodes = member(member(state, "nodes")?, "Node")?;
        let items = array(member(nodes, "items")?)?;
        let size = count(member(nodes, "size")?)?;
        assert_eq!(size, items.len());
        let addresses = items
            .iter()
            .map(|row| Ok(member(row, "address")?.clone()))
            .collect::<Result<Vec<_>>>()?;
        Ok(
            json!({"allocated":boolean(member(nodes,"allocated")?)?,"size":size,"row_addresses":addresses}),
        )
    }
    fn check(
        &mut self,
        ids: &mut Identities,
        source: &Value,
        native: &Value,
        path: &str,
    ) -> Result {
        let current = (Self::witness(source)?, Self::witness(native)?);
        let node_ids = (
            member(member(member(source, "nodes")?, "NodeID")?, "items")?.clone(),
            member(member(member(native, "nodes")?, "NodeID")?, "items")?.clone(),
        );
        assert_eq!(
            member(&current.0, "allocated")?,
            member(&current.1, "allocated")?
        );
        assert_eq!(member(&current.0, "size")?, member(&current.1, "size")?);
        if let Some(previous) = &self.previous {
            let old_source_size = count(member(&previous.0, "size")?)?;
            let old_native_size = count(member(&previous.1, "size")?)?;
            let source_size = count(member(&current.0, "size")?)?;
            let native_size = count(member(&current.1, "size")?)?;
            assert_eq!(old_source_size, old_native_size);
            if source_size > old_source_size {
                assert_eq!(
                    source_size,
                    old_source_size + 1,
                    "{path} one actual newly named Node"
                );
                assert_eq!(
                    native_size,
                    old_native_size + 1,
                    "{path} matched real Native Node growth"
                );
                assert_eq!(
                    self.transitions, 0,
                    "{path} single declared Node-growth input"
                );
                // Array1D and Vec may make different relocation choices only
                // across this witnessed matching physical allocation growth.
                if let Some(previous_ids) = &self.previous_node_ids {
                    for (before, after) in [
                        (&previous_ids.0, &node_ids.0),
                        (&previous_ids.1, &node_ids.1),
                    ] {
                        let before = array(before)?;
                        let after = array(after)?;
                        assert!(
                            before.len() < after.len(),
                            "{path} actual append-only Node input growth"
                        );
                        assert_eq!(
                            before.as_slice(),
                            &after[..before.len()],
                            "{path} same physical Node ordinals retain actual names"
                        );
                    }
                }
                ids.pairs.retain(|(kind, _), _| kind != "Node");
                self.transitions += 1;
            } else {
                // Equal-size replacement is not a new valid ownership epoch.
                assert_eq!(
                    &current, previous,
                    "{path} unchanged Node extent preserves real row storage"
                );
            }
        }
        self.previous = Some(current);
        self.previous_node_ids = Some(node_ids);
        ids.check(source, native, path)
    }
}
fn check(
    ids: &mut Identities,
    epoch: &mut NodeAllocationEpoch,
    source: &mut Factory,
    native: &Value,
    path: &str,
) -> Result {
    epoch.check(ids, &observe(source)?, native, path)
}

fn definition(base: &Value, name: &str) -> Result<Value> {
    let mut result = base.clone();
    let fields = result
        .get_mut("raw")
        .and_then(|v| v.get_mut("ZoneHVAC:IdealLoadsAirSystem"))
        .and_then(Value::as_object_mut)
        .filter(|rows| rows.len() == 1)
        .and_then(|rows| rows.get_mut("ZONE ONE IDEAL LOADS"))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| invalid("one actual fixed IdealLoads input row"))?;
    match name {
        "missing_availability_schedule" => {
            assert_eq!(
                fields.get("availability_schedule_name"),
                Some(&json!("CON-01 System Availability"))
            );
            fields.insert("availability_schedule_name".into(), json!("UNREGISTERED"));
        }
        "disconnected_supply_node" => {
            assert_eq!(
                fields.get("zone_supply_air_node_name"),
                Some(&json!("ZONE ONE INLETS"))
            );
            fields.insert(
                "zone_supply_air_node_name".into(),
                json!("UNCONNECTED SUPPLY"),
            );
        }
        "disconnected_exhaust_node" => {
            assert!(!fields.contains_key("zone_exhaust_air_node_name"));
            fields.insert(
                "zone_exhaust_air_node_name".into(),
                json!("UNCONNECTED EXHAUST"),
            );
        }
        "missing_component_name"
        | "invalid_high_comp_index"
        | "invalid_negative_comp_index"
        | "mismatched_cached_component_name" => {}
        _ => return Err(invalid("one of seven fixed admission workers").into()),
    }
    Ok(result)
}

fn arguments(worker: &Value, ordinal: usize) -> Result<(&str, i32)> {
    let expected = match ordinal {
        0..=2 => ("ZONE ONE IDEAL LOADS", 0),
        3 => ("UNREGISTERED COMPONENT", 0),
        4 => ("ZONE ONE IDEAL LOADS", 2),
        5 => ("ZONE ONE IDEAL LOADS", -1),
        6 => ("WRONG CACHED COMPONENT", 1),
        _ => return Err(invalid("seven actual mutable Sim arguments").into()),
    };
    let name = text(member(worker, "name_argument")?)?;
    let comp_index = int(member(worker, "comp_index_argument")?)?;
    assert_eq!((name, comp_index), expected);
    Ok((name, comp_index))
}

fn simulate(source: &mut Factory, name: &str) -> std::result::Result<(), Error> {
    assert_actual_phase(source);
    let mut backend = caller_backend(
        &mut source.error_state,
        &mut source.error,
        &mut source.console,
    );
    let mut initialization = SourcePurchasedAirInitOwners {
        sizing: &mut source.sizing,
        fan: &source.fan,
        globals: &source.globals,
        clock: &source.weather.global,
        hvac: &source.startup.hvac_globals,
        air_loop: &source.hvac_manager.simulation.air_loop,
        source_is_epjson: &false,
    };
    let mut calculation = SourcePurchasedAirCalcOwners {
        availability: &mut source.hvac_manager.ancillary.availability,
        controller: &source.controller,
        air: &source.air,
        fan: &source.fan,
        hvac: &source.startup.hvac_globals,
        environment: &mut source.weather.environment,
        control: &source.control,
        psychrometrics: &mut source.psych,
        backend: &mut backend,
    };
    let mut dictionary = SourceOutputDictionaryStreams {
        eso: Some(&mut source.eso),
        sqlite_present: false,
        results_framework_output_variables: &mut source.rf,
    };
    let mut output = SourcePurchasedAirReportingOutputContext {
        registration: &mut source.registration,
        input: &mut source.input,
        prescan: &source.prescan,
        reports: &source.report,
        environment: crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(&source.project.system.minimum_report_frequency, &source.globals, &source.project.system.developer_flag, Some(&source.weather.global)),
        minutes_in_time_step: source.weather.global.minutes_in_time_step,
        host: &mut source.files,
        dictionary_host: &mut dictionary,
        eio: &mut source.eio,
    };
    source.equipment.simulate_source_purchased_air(
        name,
        source.list,
        source.pointer,
        source.zone,
        &mut source.shortcut,
        false,
        &mut source.schedules,
        source.globals.num_of_zones,
        &mut source.hvac_manager.ancillary.branch_node_connections,
        source.globals.any_energy_management_system_in_model,
        &mut output,
        &mut initialization,
        &mut calculation,
        &mut source.registry,
        true,
        &mut source.sensible,
        &mut source.moisture,
    )
}

fn outcome(actual: std::result::Result<(), Error>, native: &Value, name: &str) -> Result<bool> {
    match actual {
        Ok(()) => {
            assert_eq!(name, "disconnected_exhaust_node");
            assert_eq!(member(native, "kind")?, "returned");
            assert!(member(native, "exception_type")?.is_null());
            assert!(member(native, "message")?.is_null());
            Ok(false)
        }
        Err(Error::Fatal(SourceFatalStop::Fatal(stop))) => {
            assert_ne!(name, "disconnected_exhaust_node");
            assert_eq!(member(native, "kind")?, "fatal_error_exception");
            // Compare the equivalent typed termination, not a Rust/C++ type identity.
            assert_eq!(member(native, "exception_type")?, "EnergyPlus::FatalError");
            assert_eq!(text(member(native, "message")?)?, stop.message.as_str());
            Ok(true)
        }
        Err(stop) => Err(stop.into()),
    }
}

pub(super) fn run(fixture: &Value) -> Result<usize> {
    let ingress: Value = serde_json::from_str(input::SOURCE_PURCHASED_AIR_INPUT_JSON)?;
    assert_eq!(
        member(&ingress, "schema")?,
        "src06-purchased-air-Source-input-only.v1"
    );
    let definitions = member(&ingress, "definitions")?;
    let factories = array(member(&ingress, "factories")?)?;
    assert_eq!(factories.len(), 4);
    let base = factories
        .first()
        .ok_or_else(|| invalid("actual NoLimit input-only factory"))?;
    assert_eq!(member(base, "case")?, "B-NOLIMIT-24H");
    let first = array(member(base, "steps")?)?
        .first()
        .ok_or_else(|| invalid("existing first_heat primary inputs"))?;
    assert_eq!(member(first, "name")?, "first_heat");
    let primary = member(first, "inputs")?;
    let named = member(member(fixture, "named_inputs")?, "purchased_air_admission")?;
    assert_eq!(
        member(named, "schema")?,
        "src06-purchased-air-admission-input.v1"
    );
    let named = array(member(named, "cases")?)?;
    let native = member(
        member(member(fixture, "native_output")?, "groups")?,
        "purchased_air_admission",
    )?;
    assert_eq!(
        member(native, "schema")?,
        "src06-purchased-air-admission-workers.v1"
    );
    let workers = array(member(native, "workers")?)?;
    let consoles = array(member(native, "console_after_gate")?)?;
    assert_eq!(named.len(), WORKERS.len());
    assert_eq!(workers.len(), WORKERS.len());
    assert_eq!(consoles.len(), WORKERS.len());
    let mut calls = 0;
    let mut fatal = 0;
    let mut bits = 0;
    let mut node_epochs = 0;
    for (ordinal, ((worker, expected), name)) in named.iter().zip(workers).zip(WORKERS).enumerate()
    {
        assert_eq!(member(worker, "name")?, name);
        assert_eq!(member(worker, "inputs")?, primary);
        assert_eq!(
            member(expected, "schema")?,
            "src06-purchased-air-admission-worker.v1"
        );
        assert_eq!(member(expected, "normal_completion")?, true);
        assert_eq!(member(expected, "selected_worker")?, name);
        let group = member(expected, "native_group")?;
        assert_eq!(
            member(group, "schema")?,
            "src06-purchased-air-admission-output.v1"
        );
        assert_eq!(member(group, "selected_worker")?, name);
        assert_eq!(count(member(group, "full_Sim_calls")?)?, 1);
        let cases = array(member(group, "cases")?)?;
        assert_eq!(cases.len(), 1);
        let expected = cases
            .first()
            .ok_or_else(|| invalid("one whole full-Sim Native worker record"))?;
        assert_eq!(member(expected, "name")?, name);
        assert_eq!(member(expected, "input")?, worker);
        let definition = definition(base, name)?;
        let mut source = Box::new(
            Factory::new_with_console_output(definitions, &definition, true).map_err(|error| {
                invalid(&format!("EH worker {name} actual Factory::new: {error}"))
            })?,
        );
        // Each Native worker has its own fresh process/lifetime; never join addresses across workers.
        let mut ids = Identities::default();
        let mut node_epoch = NodeAllocationEpoch::default();
        check(
            &mut ids,
            &mut node_epoch,
            &mut source,
            member(expected, "setup")?,
            "setup",
        )?;
        source.phase_inputs(primary)?;
        let prefix = member(expected, "actual_caller_InitZoneEquipment_prefix")?;
        check(
            &mut ids,
            &mut node_epoch,
            &mut source,
            member(prefix, "entry")?,
            "InitZoneEquipment.entry",
        )?;
        source.initialize_prefix(true, false)?;
        check(
            &mut ids,
            &mut node_epoch,
            &mut source,
            member(prefix, "exit")?,
            "InitZoneEquipment.exit",
        )?;
        source.caller_inputs(primary)?;
        let (argument_name, comp_index) = arguments(worker, ordinal)?;
        *source
            .equipment
            .purchased_equipment_index(source.list, source.pointer)? = comp_index;
        check(
            &mut ids,
            &mut node_epoch,
            &mut source,
            member(expected, "entry")?,
            "entry",
        )?;
        let arguments = member(expected, "call_arguments")?;
        assert_eq!(member(arguments, "name")?, argument_name);
        assert_eq!(int(member(arguments, "CompIndex")?)?, comp_index);
        assert_eq!(member(arguments, "FirstHVACIteration")?, true);
        assert_eq!(int(member(arguments, "ControlledZoneNum")?)?, source.zone);
        let actual = simulate(&mut source, argument_name);
        fatal += usize::from(outcome(actual, member(expected, "outcome")?, name)?);
        check(
            &mut ids,
            &mut node_epoch,
            &mut source,
            member(expected, "exit")?,
            "exit",
        )?;
        // Root retains the original stdout bytes. The fixture removes only the
        // exact pre-science PID-gate line and transports physical CRLF as LF.
        assert_eq!(
            std::str::from_utf8(&source.console)?,
            text(&consoles[ordinal])?,
            "{name} actual complete console body"
        );
        assert_eq!(
            node_epoch.transitions,
            usize::from(matches!(
                name,
                "disconnected_supply_node" | "disconnected_exhaust_node"
            )),
            "{name} only the two declared new Node names grow storage"
        );
        node_epochs += node_epoch.transitions;
        bits += ids.raw_bits;
        calls += 1;
    }
    assert_eq!(calls, 7);
    assert_eq!(fatal, 6);
    assert!(bits > 0);
    println!(
        "PurchasedAir admission full Sim calls={calls}; real typed fatal outcomes={fatal}; returned=1; finite raw-bit observations={bits}; independent fresh owners=7; observed Node allocation epochs={node_epochs}"
    );
    Ok(calls)
}

impl OrdinaryEquipment {
    pub(crate) fn run_native_gap_purchased_air_admission_cohort(fixture: &Value) -> Result<usize> {
        run(fixture)
    }
}
