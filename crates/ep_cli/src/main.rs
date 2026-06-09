//! Command line entry point for eplus-rs.

mod conformance_artifacts;
mod internal_gains;
mod static_model;
mod time_weather_schedule;

use conformance_artifacts::{
    BaselineSummary, generate_conformance_baseline, generate_conformance_baseline_in_dir,
    generate_conformance_report_skeleton,
};
use ep_compare::{
    Tolerance, compare_series, load_eio_construction_ctf, load_eio_construction_ctf_coefficients,
    load_eio_heat_transfer_surfaces, load_eio_material_ctf_summary,
    load_eio_other_equipment_nominal, load_eio_warmup_environments, load_eio_zone_geometry,
    load_eso_series, load_eso_time_series,
};
use ep_compiler::{CompileReport, DiagnosticSeverity, compile_raw_model};
use ep_conformance::{
    CaseSourceKind, CaseTier, ComparisonClass, ConformanceCase, EvidenceDomain, OutputFrequency,
    OutputLevel, ReportFormat, SourceArtifact, ToleranceRule, VariableClass, load_case_file,
    load_case_v2_file,
};
use ep_model::{
    Construction, Material, OtherEquipment, OutsideBoundaryCondition, ScheduleId, SimulationModel,
    SurfaceType, TypedModel,
};
use ep_oracle::default_oracle_release;
use ep_raw_model::{RawModelSummary, load_epjson_file};
use ep_runtime::{
    ConstructionCtfCoefficientOverride, ExecutionPlan, ExecutionStep, FirstZoneSimulationOptions,
    HeatBalanceCtfHistorySlotFirstSample, HeatBalanceCtfHistorySlotSample,
    HeatBalanceCtfInitialHistoryPolicy, HeatBalanceSimulationOptions,
    HeatBalanceSurfaceFirstSampleTrace, HeatBalanceWarmupSummary, HeatBalanceZoneAirAlgorithm,
    NodeStateProjection, NodeStateProjectionOptions, PlantStateProjection,
    PlantStateProjectionOptions, ResultStore, SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
    SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
    SURFACE_CTF_INSIDE_HISTORY_FLUX_TERM_RATE_VARIABLE,
    SURFACE_CTF_INSIDE_HISTORY_TEMPERATURE_TERM_RATE_VARIABLE,
    SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE,
    SURFACE_CTF_OUTSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
    SURFACE_CTF_OUTSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
    SURFACE_CTF_OUTSIDE_HISTORY_TERM_RATE_VARIABLE, SimulationMode, SurfaceGeometrySummary,
    ZoneGeometrySummary, append_surface_incident_solar_radiation_series, build_execution_plan,
    build_hourly_time_axis, load_epw_dry_bulb_series, load_epw_records,
    simulate_constant_schedules, simulate_first_zone_uncontrolled,
    simulate_heat_balance_zone_air_temperatures,
    simulate_heat_balance_zone_air_temperatures_with_weather_records_and_ctf_coefficients,
    simulate_ideal_loads_node_state_projection, simulate_plant_state_projection, surface_area_m2,
    surface_geometry_summaries, zone_geometry_summaries,
};
use internal_gains::{generate_internal_gains_report, run_compare_internal_convective_gain};
use static_model::generate_static_model_report;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use time_weather_schedule::generate_time_weather_schedule_report;

const HEAT_BALANCE_BOTTLENECK_LIMIT: usize = 8;
const HEAT_BALANCE_MAX_SAMPLE_CONTEXT_LIMIT: usize = 8;
const HEAT_BALANCE_CTF_SEED_POLICY_ENV: &str = "RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_SEED_POLICY";
const HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY_ENV: &str =
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY";
const HEAT_BALANCE_ZONE_AIR_ALGORITHM_ENV: &str =
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_AIR_ALGORITHM";
const HEAT_BALANCE_WARMUP_MINIMUM_DAYS_ENV: &str =
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_WARMUP_MINIMUM_DAYS";
const HEAT_BALANCE_SURFACE_ITERATIONS_ENV: &str =
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_ITERATIONS";
const HEAT_BALANCE_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K: f64 = 5.0;

const ZONE_TEMPERATURE_COMPARE_USAGE: &str = "usage: eplus-rs compare zone-temperature <input.epJSON> <weather.epw> <eplusout.eso> [--report-dir DIR]";
const CONFORMANCE_DIAGNOSTIC_REPORT_USAGE: &str =
    "usage: eplus-rs conformance diagnostic-report <case.toml> <oracle-root> <output-root>";
const CONFORMANCE_HEAT_BALANCE_REPORT_USAGE: &str =
    "usage: eplus-rs conformance heat-balance-report <case.toml> <oracle-root> <output-root>";
const CONFORMANCE_HEAT_BALANCE_DIAGNOSTIC_REPORT_USAGE: &str = "usage: eplus-rs conformance heat-balance-diagnostic-report <case.toml> <oracle-root> <output-root>";
const CONFORMANCE_STATIC_MODEL_REPORT_USAGE: &str =
    "usage: eplus-rs conformance static-model-report <case.toml> <oracle-root> <output-root>";
const CONFORMANCE_TIME_WEATHER_SCHEDULE_REPORT_USAGE: &str = "usage: eplus-rs conformance time-weather-schedule-report <case.toml> <oracle-root> <output-root>";
const CONFORMANCE_INTERNAL_GAINS_REPORT_USAGE: &str =
    "usage: eplus-rs conformance internal-gains-report <case.toml> <oracle-root> <output-root>";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let exit_code = run(&args);
    std::process::exit(exit_code);
}

fn run(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        None | Some("--help") | Some("-h") => {
            print_help();
            0
        }
        Some("--version") | Some("-V") => {
            println!("eplus-rs {}", env!("CARGO_PKG_VERSION"));
            0
        }
        Some("oracle-info") => {
            print_oracle_info();
            0
        }
        Some("modes") => {
            print_modes();
            0
        }
        Some("compare") => run_compare_command(&args[1..]),
        Some("conformance") => run_conformance_command(&args[1..]),
        Some("compile") => run_compile_command(&args[1..]),
        Some("model") => run_model_command(&args[1..]),
        Some("run") => run_run_command(&args[1..]),
        Some(command) => {
            eprintln!("unsupported command: {command}");
            eprintln!("Try: eplus-rs model inspect <input.epJSON>");
            2
        }
    }
}

fn run_model_command(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("inspect") => {
            let Some(path) = args.get(1) else {
                eprintln!("missing input path");
                eprintln!("usage: eplus-rs model inspect <input.epJSON>");
                return 2;
            };
            match load_epjson_file(path) {
                Ok(model) => {
                    print_raw_model_summary(&model.summary());
                    0
                }
                Err(error) => {
                    eprintln!("{error}");
                    1
                }
            }
        }
        Some("compile") => run_compile_command(&args[1..]),
        Some("plan") => run_plan_command(&args[1..]),
        Some("geometry") => run_geometry_command(&args[1..]),
        Some(command) => {
            eprintln!("unsupported model command: {command}");
            eprintln!("usage: eplus-rs model inspect <input.epJSON>");
            eprintln!("usage: eplus-rs model compile <input.epJSON>");
            eprintln!("usage: eplus-rs model plan <input.epJSON>");
            eprintln!("usage: eplus-rs model geometry <input.epJSON>");
            2
        }
        None => {
            eprintln!("missing model command");
            eprintln!("usage: eplus-rs model inspect <input.epJSON>");
            eprintln!("usage: eplus-rs model compile <input.epJSON>");
            eprintln!("usage: eplus-rs model plan <input.epJSON>");
            eprintln!("usage: eplus-rs model geometry <input.epJSON>");
            2
        }
    }
}

fn run_compile_command(args: &[String]) -> i32 {
    let Some(path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs model compile <input.epJSON>");
        return 2;
    };

    let raw_model = match load_epjson_file(path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    let result = compile_raw_model(&raw_model);
    if let Some(model) = result.model.as_ref() {
        print_typed_model_summary(model, &result.report);
        return 0;
    }

    print_compile_diagnostics(&result.report);
    1
}

fn run_plan_command(args: &[String]) -> i32 {
    let Some(path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs model plan <input.epJSON>");
        return 2;
    };

    let raw_model = match load_epjson_file(path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    let simulation_model = SimulationModel::from_typed(model);
    let plan = build_execution_plan(&simulation_model);

    print_plan_summary(&simulation_model, &plan);
    0
}

fn run_geometry_command(args: &[String]) -> i32 {
    let Some(path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs model geometry <input.epJSON>");
        return 2;
    };

    let raw_model = match load_epjson_file(path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    let summaries = zone_geometry_summaries(&model);

    print_geometry_summary(&summaries);
    0
}

fn print_help() {
    println!("eplus-rs");
    println!();
    println!("Commands:");
    println!("  oracle-info   print locked EnergyPlus oracle metadata");
    println!("  modes         print planned simulation modes");
    println!("  model inspect <input.epJSON>");
    println!("  model compile <input.epJSON>");
    println!("  model plan <input.epJSON>");
    println!("  model geometry <input.epJSON>");
    println!("  run first-zone <input.epJSON> <weather.epw> [--hours N]");
    println!("  run node-state-projection <input.epJSON> <output-dir> [--hours N]");
    println!("  run plant-state-projection <input.epJSON> <output-dir> [--hours N]");
    println!("  compile <input.epJSON>");
    println!("  compare schedule-value <input.epJSON> <eplusout.eso>");
    println!("  compare geometry <input.epJSON> <eplusout.eio>");
    println!("  compare surface-geometry <input.epJSON> <eplusout.eio>");
    println!("  compare construction-materials <input.epJSON> <eplusout.eio>");
    println!("  compare internal-gains <input.epJSON> <eplusout.eio>");
    println!("  compare internal-convective-gain <input.epJSON> <eplusout.eso>");
    println!("  compare weather-fields <weather.epw> <eplusout.eso>");
    println!("  compare weather-drybulb <weather.epw> <eplusout.eso>");
    println!("{ZONE_TEMPERATURE_COMPARE_USAGE}");
    println!("  conformance validate-case <case.toml>");
    println!("  conformance validate-case-v2 <case.toml>");
    println!("  conformance baseline <case.toml> <oracle-root> <output-root>");
    println!("  conformance report-skeleton <case.toml> <baseline-case-dir> <report-root>");
    println!("{CONFORMANCE_DIAGNOSTIC_REPORT_USAGE}");
    println!("{CONFORMANCE_HEAT_BALANCE_REPORT_USAGE}");
    println!("{CONFORMANCE_HEAT_BALANCE_DIAGNOSTIC_REPORT_USAGE}");
    println!("{CONFORMANCE_STATIC_MODEL_REPORT_USAGE}");
    println!("{CONFORMANCE_TIME_WEATHER_SCHEDULE_REPORT_USAGE}");
    println!();
    println!("Future commands:");
    println!("  model validate <input.epJSON>");
    println!("  graph validate <input.epJSON>");
}

fn print_plan_summary(model: &SimulationModel, plan: &ExecutionPlan) {
    println!("ExecutionPlan");
    println!("  zones: {}", model.typed.zones.len());
    println!("  surfaces: {}", model.typed.surfaces.len());
    println!("  constructions: {}", model.typed.constructions.len());
    println!("  materials: {}", model.typed.materials.len());
    println!(
        "  schedules: {}",
        model.typed.schedules.len() + model.typed.compact_schedules.len()
    );
    println!("  constant_schedules: {}", model.typed.schedules.len());
    println!(
        "  compact_schedules: {}",
        model.typed.compact_schedules.len()
    );
    println!("  other_equipment: {}", model.typed.other_equipment.len());
    println!(
        "  thermostat_dual_setpoints: {}",
        model.typed.thermostat_dual_setpoints.len()
    );
    println!("  zone_thermostats: {}", model.typed.zone_thermostats.len());
    println!(
        "  ideal_loads_air_systems: {}",
        model.typed.ideal_loads_air_systems.len()
    );
    println!(
        "  zone_equipment_lists: {}",
        model.typed.zone_equipment_lists.len()
    );
    println!(
        "  zone_equipment_connections: {}",
        model.typed.zone_equipment_connections.len()
    );
    println!("  nodes: {}", model.typed.nodes.len());
    println!("  node_lists: {}", model.typed.node_lists.len());
    println!("  plant_loops: {}", model.typed.plant_loops.len());
    println!("  plant_branches: {}", model.typed.plant_branches.len());
    println!(
        "  plant_branch_lists: {}",
        model.typed.plant_branch_lists.len()
    );
    println!("  plant_connectors: {}", model.typed.plant_connectors.len());
    println!(
        "  plant_connector_lists: {}",
        model.typed.plant_connector_lists.len()
    );
    println!(
        "  pumps_constant_speed: {}",
        model.typed.pumps_constant_speed.len()
    );
    println!(
        "  boilers_hot_water: {}",
        model.typed.boilers_hot_water.len()
    );
    println!(
        "  chillers_electric_eir: {}",
        model.typed.chillers_electric_eir.len()
    );
    println!("  run_periods: {}", model.typed.run_periods.len());
    println!("  zone_surface_edges: {}", model.graph.zone_surfaces.len());
    println!(
        "  construction_material_edges: {}",
        model.graph.construction_materials.len()
    );
    println!(
        "  zone_thermostat_edges: {}",
        model.graph.zone_thermostats.len()
    );
    println!(
        "  thermostat_setpoint_edges: {}",
        model.graph.thermostat_setpoints.len()
    );
    println!(
        "  zone_ideal_loads_edges: {}",
        model.graph.zone_ideal_loads.len()
    );
    println!(
        "  node_list_member_edges: {}",
        model.graph.node_list_members.len()
    );
    println!(
        "  ideal_loads_supply_node_edges: {}",
        model.graph.ideal_loads_supply_nodes.len()
    );
    println!(
        "  zone_air_node_edges: {}",
        model.graph.zone_air_nodes.len()
    );
    println!(
        "  plant_loop_branch_list_edges: {}",
        model.graph.plant_loop_branch_lists.len()
    );
    println!(
        "  plant_branch_list_member_edges: {}",
        model.graph.plant_branch_list_members.len()
    );
    println!(
        "  plant_connector_list_member_edges: {}",
        model.graph.plant_connector_list_members.len()
    );
    println!(
        "  plant_branch_component_edges: {}",
        model.graph.plant_branch_components.len()
    );
    println!("  stages: {}", plan.stages.len());
    println!("  steps: {}", plan.step_count());
    for stage in &plan.stages {
        println!("    {}: {}", stage.name, stage.steps.len());
        for (index, step) in stage.steps.iter().enumerate() {
            println!("      {index}: {}", execution_step_label(step));
        }
    }
}

fn execution_step_label(step: &ExecutionStep) -> String {
    match step {
        ExecutionStep::UpdateWeather => "UpdateWeather".to_string(),
        ExecutionStep::EvaluateSchedule(id) => format!("EvaluateSchedule({})", id.0),
        ExecutionStep::EvaluateZoneThermostat(id) => {
            format!("EvaluateZoneThermostat({})", id.0)
        }
        ExecutionStep::SolveZone(id) => format!("SolveZone({})", id.0),
        ExecutionStep::EvaluateIdealLoadsAirSystem(id) => {
            format!("EvaluateIdealLoadsAirSystem({})", id.0)
        }
        ExecutionStep::WriteOutput(id) => format!("WriteOutput({})", id.0),
    }
}

fn print_geometry_summary(summaries: &[ZoneGeometrySummary]) {
    println!("Geometry Summary");
    println!("  zones: {}", summaries.len());
    for summary in summaries {
        let volume_m3 = summary
            .volume_m3
            .map(|volume_m3| format!("{volume_m3:.6}"))
            .unwrap_or_else(|| "unavailable".to_string());
        println!(
            "  zone: {} surfaces: {} floor_area_m2: {:.6} volume_m3: {} exterior_wall_area_m2: {:.6}",
            summary.zone_name,
            summary.surface_count,
            summary.floor_area_m2,
            volume_m3,
            summary.exterior_wall_area_m2
        );
    }
    println!("  status: summarized");
}

fn run_conformance_command(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("validate-case") => run_conformance_validate_case(&args[1..]),
        Some("validate-case-v2") => run_conformance_validate_case_v2(&args[1..]),
        Some("baseline") => run_conformance_baseline(&args[1..]),
        Some("report-skeleton") => run_conformance_report_skeleton(&args[1..]),
        Some("diagnostic-report") => run_conformance_diagnostic_report(&args[1..]),
        Some("heat-balance-report") => run_conformance_heat_balance_report(&args[1..]),
        Some("heat-balance-diagnostic-report") => {
            run_conformance_heat_balance_diagnostic_report(&args[1..])
        }
        Some("static-model-report") => run_conformance_static_model_report(&args[1..]),
        Some("time-weather-schedule-report") => {
            run_conformance_time_weather_schedule_report(&args[1..])
        }
        Some("internal-gains-report") => run_conformance_internal_gains_report(&args[1..]),
        Some(command) => {
            eprintln!("unsupported conformance command: {command}");
            eprintln!("usage: eplus-rs conformance validate-case <case.toml>");
            eprintln!("usage: eplus-rs conformance validate-case-v2 <case.toml>");
            eprintln!(
                "usage: eplus-rs conformance baseline <case.toml> <oracle-root> <output-root>"
            );
            eprintln!(
                "usage: eplus-rs conformance report-skeleton <case.toml> <baseline-case-dir> <report-root>"
            );
            eprintln!("{CONFORMANCE_DIAGNOSTIC_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_HEAT_BALANCE_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_HEAT_BALANCE_DIAGNOSTIC_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_STATIC_MODEL_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_TIME_WEATHER_SCHEDULE_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_INTERNAL_GAINS_REPORT_USAGE}");
            2
        }
        None => {
            eprintln!("missing conformance command");
            eprintln!("usage: eplus-rs conformance validate-case <case.toml>");
            eprintln!("usage: eplus-rs conformance validate-case-v2 <case.toml>");
            eprintln!(
                "usage: eplus-rs conformance baseline <case.toml> <oracle-root> <output-root>"
            );
            eprintln!(
                "usage: eplus-rs conformance report-skeleton <case.toml> <baseline-case-dir> <report-root>"
            );
            eprintln!("{CONFORMANCE_DIAGNOSTIC_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_HEAT_BALANCE_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_HEAT_BALANCE_DIAGNOSTIC_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_STATIC_MODEL_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_TIME_WEATHER_SCHEDULE_REPORT_USAGE}");
            eprintln!("{CONFORMANCE_INTERNAL_GAINS_REPORT_USAGE}");
            2
        }
    }
}

fn run_conformance_validate_case_v2(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!("usage: eplus-rs conformance validate-case-v2 <case.toml>");
        return 2;
    };

    let manifest = match load_case_v2_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    println!("Conformance Case v2");
    print_conformance_case_summary(&manifest);
    println!("  status: valid");
    0
}

fn run_conformance_validate_case(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!("usage: eplus-rs conformance validate-case <case.toml>");
        return 2;
    };

    let manifest = match load_case_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    println!("Conformance Case");
    print_conformance_case_summary(&manifest);
    println!("  status: valid");
    0
}

fn run_conformance_baseline(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!("usage: eplus-rs conformance baseline <case.toml> <oracle-root> <output-root>");
        return 2;
    };
    let Some(oracle_root) = args.get(1) else {
        eprintln!("missing oracle root path");
        eprintln!("usage: eplus-rs conformance baseline <case.toml> <oracle-root> <output-root>");
        return 2;
    };
    let Some(output_root) = args.get(2) else {
        eprintln!("missing output root path");
        eprintln!("usage: eplus-rs conformance baseline <case.toml> <oracle-root> <output-root>");
        return 2;
    };

    let manifest = match load_case_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if manifest.oracle_version != default_oracle_release().version {
        eprintln!(
            "case oracle_version {} does not match locked oracle {}",
            manifest.oracle_version,
            default_oracle_release().version
        );
        return 1;
    }

    match generate_conformance_baseline(
        Path::new(case_path),
        &manifest,
        Path::new(oracle_root),
        Path::new(output_root),
    ) {
        Ok(summary) => {
            println!("Conformance Baseline");
            print_conformance_case_summary(&manifest);
            println!("  output_dir: {}", summary.output_dir.display());
            println!("  idf: {}", summary.idf.display());
            if let Some(weather) = summary.weather.as_ref() {
                println!("  weather: {}", weather.display());
            }
            println!("  epjson: {}", summary.epjson.display());
            println!("  eso: {}", summary.eso.display());
            println!(
                "  expanded_manifest: {}",
                summary.expanded_manifest.display()
            );
            println!("  injected_outputs: {}", summary.injected_outputs);
            println!("  injected_meters: {}", summary.injected_meters);
            println!("  status: generated");
            0
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

fn run_conformance_report_skeleton(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!(
            "usage: eplus-rs conformance report-skeleton <case.toml> <baseline-case-dir> <report-root>"
        );
        return 2;
    };
    let Some(baseline_case_dir) = args.get(1) else {
        eprintln!("missing baseline case directory");
        eprintln!(
            "usage: eplus-rs conformance report-skeleton <case.toml> <baseline-case-dir> <report-root>"
        );
        return 2;
    };
    let Some(report_root) = args.get(2) else {
        eprintln!("missing report root path");
        eprintln!(
            "usage: eplus-rs conformance report-skeleton <case.toml> <baseline-case-dir> <report-root>"
        );
        return 2;
    };

    let manifest = match load_case_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    match generate_conformance_report_skeleton(
        &manifest,
        Path::new(baseline_case_dir),
        Path::new(report_root),
    ) {
        Ok(summary) => {
            println!("Conformance Report Skeleton");
            print_conformance_case_summary(&manifest);
            println!("  report: {}", summary.report_path.display());
            println!("  series: {}", summary.series);
            println!("  energyplus_warnings: {}", summary.warning_count);
            println!("  energyplus_severes: {}", summary.severe_count);
            println!("  energyplus_fatals: {}", summary.fatal_count);
            println!("  tolerance_policy: none");
            println!("  status: baseline-only");
            0
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

fn run_conformance_diagnostic_report(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!("{CONFORMANCE_DIAGNOSTIC_REPORT_USAGE}");
        return 2;
    };
    let Some(oracle_root) = args.get(1) else {
        eprintln!("missing oracle root path");
        eprintln!("{CONFORMANCE_DIAGNOSTIC_REPORT_USAGE}");
        return 2;
    };
    let Some(output_root) = args.get(2) else {
        eprintln!("missing output root path");
        eprintln!("{CONFORMANCE_DIAGNOSTIC_REPORT_USAGE}");
        return 2;
    };

    let manifest = match load_case_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if manifest.oracle_version != default_oracle_release().version {
        eprintln!(
            "case oracle_version {} does not match locked oracle {}",
            manifest.oracle_version,
            default_oracle_release().version
        );
        return 1;
    }

    match generate_conformance_diagnostic_report(
        Path::new(case_path),
        &manifest,
        Path::new(oracle_root),
        Path::new(output_root),
    ) {
        Ok(summary) => {
            println!("Conformance Diagnostic Report");
            print_conformance_case_summary(&manifest);
            println!("  baseline_dir: {}", summary.baseline.output_dir.display());
            println!("  report_dir: {}", summary.report_dir.display());
            println!("  compare_report: {}", summary.compare_report.display());
            println!("  compare_summary: {}", summary.compare_summary.display());
            println!("  samples: {}", summary.samples);
            println!("  tolerance_policy: none");
            println!("  status: {}", summary.status);
            0
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

fn run_conformance_heat_balance_report(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!("{CONFORMANCE_HEAT_BALANCE_REPORT_USAGE}");
        return 2;
    };
    let Some(oracle_root) = args.get(1) else {
        eprintln!("missing oracle root path");
        eprintln!("{CONFORMANCE_HEAT_BALANCE_REPORT_USAGE}");
        return 2;
    };
    let Some(output_root) = args.get(2) else {
        eprintln!("missing output root path");
        eprintln!("{CONFORMANCE_HEAT_BALANCE_REPORT_USAGE}");
        return 2;
    };

    let manifest = match load_case_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if manifest.oracle_version != default_oracle_release().version {
        eprintln!(
            "case oracle_version {} does not match locked oracle {}",
            manifest.oracle_version,
            default_oracle_release().version
        );
        return 1;
    }

    match generate_conformance_heat_balance_report(
        Path::new(case_path),
        &manifest,
        Path::new(oracle_root),
        Path::new(output_root),
    ) {
        Ok(summary) => {
            println!("Conformance Heat Balance Report");
            print_conformance_case_summary(&manifest);
            println!("  baseline_dir: {}", summary.baseline.output_dir.display());
            println!("  report_dir: {}", summary.report_dir.display());
            println!("  compare_report: {}", summary.compare_report.display());
            println!("  compare_summary: {}", summary.compare_summary.display());
            println!("  compare_digest: {}", summary.compare_digest.display());
            println!("  samples: {}", summary.samples);
            println!(
                "  heat_balance_timesteps: {}",
                summary.heat_balance_timesteps
            );
            println!(
                "  heat_balance_run_period_timesteps: {}",
                summary.heat_balance_run_period_timesteps
            );
            print_heat_balance_warmup("  ", &summary.heat_balance_warmup);
            println!("  tolerance_policy: {}", summary.tolerance_policy);
            println!("  zone_air_algorithm: {}", summary.zone_air_algorithm);
            println!(
                "  surface_iteration_count: {}",
                summary.surface_iteration_count
            );
            println!(
                "  ctf_initial_history_policy: {}",
                summary.ctf_initial_history_policy
            );
            println!("  status: {}", summary.status);
            if summary.status == "pass" { 0 } else { 1 }
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

fn run_conformance_heat_balance_diagnostic_report(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!("{CONFORMANCE_HEAT_BALANCE_DIAGNOSTIC_REPORT_USAGE}");
        return 2;
    };
    let Some(oracle_root) = args.get(1) else {
        eprintln!("missing oracle root path");
        eprintln!("{CONFORMANCE_HEAT_BALANCE_DIAGNOSTIC_REPORT_USAGE}");
        return 2;
    };
    let Some(output_root) = args.get(2) else {
        eprintln!("missing output root path");
        eprintln!("{CONFORMANCE_HEAT_BALANCE_DIAGNOSTIC_REPORT_USAGE}");
        return 2;
    };

    let manifest = match load_case_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if manifest.oracle_version != default_oracle_release().version {
        eprintln!(
            "case oracle_version {} does not match locked oracle {}",
            manifest.oracle_version,
            default_oracle_release().version
        );
        return 1;
    }

    match generate_conformance_heat_balance_diagnostic_report(
        Path::new(case_path),
        &manifest,
        Path::new(oracle_root),
        Path::new(output_root),
    ) {
        Ok(summary) => {
            println!("Diagnostic Heat Balance Report");
            print_conformance_case_summary(&manifest);
            println!("  baseline_dir: {}", summary.baseline.output_dir.display());
            println!("  report_dir: {}", summary.report_dir.display());
            println!("  compare_report: {}", summary.compare_report.display());
            println!("  compare_summary: {}", summary.compare_summary.display());
            println!("  compare_digest: {}", summary.compare_digest.display());
            println!("  samples: {}", summary.samples);
            println!(
                "  heat_balance_timesteps: {}",
                summary.heat_balance_timesteps
            );
            println!(
                "  heat_balance_run_period_timesteps: {}",
                summary.heat_balance_run_period_timesteps
            );
            print_heat_balance_warmup("  ", &summary.heat_balance_warmup);
            println!("  tolerance_policy: {}", summary.tolerance_policy);
            println!("  zone_air_algorithm: {}", summary.zone_air_algorithm);
            println!(
                "  surface_iteration_count: {}",
                summary.surface_iteration_count
            );
            println!(
                "  ctf_initial_history_policy: {}",
                summary.ctf_initial_history_policy
            );
            println!("  status: {}", summary.status);
            0
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

fn run_conformance_time_weather_schedule_report(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!("{CONFORMANCE_TIME_WEATHER_SCHEDULE_REPORT_USAGE}");
        return 2;
    };
    let Some(oracle_root) = args.get(1) else {
        eprintln!("missing oracle root path");
        eprintln!("{CONFORMANCE_TIME_WEATHER_SCHEDULE_REPORT_USAGE}");
        return 2;
    };
    let Some(output_root) = args.get(2) else {
        eprintln!("missing output root path");
        eprintln!("{CONFORMANCE_TIME_WEATHER_SCHEDULE_REPORT_USAGE}");
        return 2;
    };

    let manifest = match load_case_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if manifest.oracle_version != default_oracle_release().version {
        eprintln!(
            "case oracle_version {} does not match locked oracle {}",
            manifest.oracle_version,
            default_oracle_release().version
        );
        return 1;
    }

    match generate_time_weather_schedule_report(
        Path::new(case_path),
        &manifest,
        Path::new(oracle_root),
        Path::new(output_root),
    ) {
        Ok(summary) => {
            println!("Time, Weather, and Schedule Conformance Report");
            print_conformance_case_summary(&manifest);
            println!("  baseline_dir: {}", summary.baseline.output_dir.display());
            println!("  report_dir: {}", summary.report_dir.display());
            println!("  compare_report: {}", summary.compare_report.display());
            println!("  compare_summary: {}", summary.compare_summary.display());
            println!("  series: {}", summary.series_count);
            println!("  conformance_series: {}", summary.conformance_series_count);
            println!("  status: {}", summary.status);
            if summary.status == "pass" { 0 } else { 1 }
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

fn run_conformance_internal_gains_report(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!("{CONFORMANCE_INTERNAL_GAINS_REPORT_USAGE}");
        return 2;
    };
    let Some(oracle_root) = args.get(1) else {
        eprintln!("missing oracle root path");
        eprintln!("{CONFORMANCE_INTERNAL_GAINS_REPORT_USAGE}");
        return 2;
    };
    let Some(output_root) = args.get(2) else {
        eprintln!("missing output root path");
        eprintln!("{CONFORMANCE_INTERNAL_GAINS_REPORT_USAGE}");
        return 2;
    };

    let manifest = match load_case_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if manifest.oracle_version != default_oracle_release().version {
        eprintln!(
            "case oracle_version {} does not match locked oracle {}",
            manifest.oracle_version,
            default_oracle_release().version
        );
        return 1;
    }

    match generate_internal_gains_report(
        Path::new(case_path),
        &manifest,
        Path::new(oracle_root),
        Path::new(output_root),
    ) {
        Ok(summary) => {
            println!("Internal Gains Conformance Report");
            print_conformance_case_summary(&manifest);
            println!("  baseline_dir: {}", summary.baseline.output_dir.display());
            println!("  report_dir: {}", summary.report_dir.display());
            println!("  compare_report: {}", summary.compare_report.display());
            println!("  compare_summary: {}", summary.compare_summary.display());
            println!("  series: {}", summary.series_count);
            println!("  conformance_series: {}", summary.conformance_series_count);
            println!("  status: {}", summary.status);
            if summary.status == "pass" { 0 } else { 1 }
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

fn run_conformance_static_model_report(args: &[String]) -> i32 {
    let Some(case_path) = args.first() else {
        eprintln!("missing case manifest path");
        eprintln!("{CONFORMANCE_STATIC_MODEL_REPORT_USAGE}");
        return 2;
    };
    let Some(oracle_root) = args.get(1) else {
        eprintln!("missing oracle root path");
        eprintln!("{CONFORMANCE_STATIC_MODEL_REPORT_USAGE}");
        return 2;
    };
    let Some(output_root) = args.get(2) else {
        eprintln!("missing output root path");
        eprintln!("{CONFORMANCE_STATIC_MODEL_REPORT_USAGE}");
        return 2;
    };

    let manifest = match load_case_file(case_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if manifest.oracle_version != default_oracle_release().version {
        eprintln!(
            "case oracle_version {} does not match locked oracle {}",
            manifest.oracle_version,
            default_oracle_release().version
        );
        return 1;
    }

    match generate_static_model_report(
        Path::new(case_path),
        &manifest,
        Path::new(oracle_root),
        Path::new(output_root),
    ) {
        Ok(summary) => {
            println!("Static Model Conformance Report");
            print_conformance_case_summary(&manifest);
            println!("  baseline_dir: {}", summary.baseline.output_dir.display());
            println!("  report_dir: {}", summary.report_dir.display());
            println!("  compare_report: {}", summary.compare_report.display());
            println!("  compare_summary: {}", summary.compare_summary.display());
            println!("  outputs: {}", summary.output_count);
            println!(
                "  conformance_outputs: {}",
                summary.conformance_output_count
            );
            println!("  status: {}", summary.status);
            if summary.status == "pass" { 0 } else { 1 }
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

struct DiagnosticReportSummary {
    baseline: BaselineSummary,
    report_dir: PathBuf,
    compare_report: PathBuf,
    compare_summary: PathBuf,
    samples: usize,
    status: &'static str,
}

struct HeatBalanceReportSummary {
    baseline: BaselineSummary,
    report_dir: PathBuf,
    compare_report: PathBuf,
    compare_summary: PathBuf,
    compare_digest: PathBuf,
    samples: usize,
    heat_balance_timesteps: usize,
    heat_balance_run_period_timesteps: usize,
    heat_balance_warmup: HeatBalanceWarmupDiagnostic,
    tolerance_policy: String,
    zone_air_algorithm: &'static str,
    surface_iteration_count: u32,
    ctf_initial_history_policy: &'static str,
    status: &'static str,
}

fn generate_conformance_diagnostic_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<DiagnosticReportSummary, String> {
    let report_context = zone_temperature_report_context_from_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");

    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let weather = baseline
        .weather
        .as_ref()
        .ok_or_else(|| "zone-temperature diagnostic requires input.weather".to_string())?;
    let diagnostic = build_zone_temperature_diagnostic(&baseline.epjson, weather, &baseline.eso)?;
    write_zone_temperature_diagnostic_report(&compare_dir, &diagnostic, Some(&report_context))?;

    Ok(DiagnosticReportSummary {
        baseline,
        report_dir: compare_dir.clone(),
        compare_report: compare_dir.join("compare-report.md"),
        compare_summary: compare_dir.join("compare-summary.json"),
        samples: diagnostic.samples,
        status: diagnostic.status,
    })
}

fn generate_conformance_heat_balance_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<HeatBalanceReportSummary, String> {
    let report_context = heat_balance_conformance_context_from_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");

    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let weather = baseline
        .weather
        .as_ref()
        .ok_or_else(|| "heat-balance conformance requires input.weather".to_string())?;
    let diagnostic = build_heat_balance_conformance_diagnostic(
        &baseline.epjson,
        weather,
        &baseline.eio,
        &baseline.eso,
        &report_context,
    )?;
    let conformance = evaluate_heat_balance_conformance(&diagnostic, &report_context);
    write_heat_balance_conformance_report(&compare_dir, &diagnostic, &conformance)?;

    Ok(HeatBalanceReportSummary {
        baseline,
        report_dir: compare_dir.clone(),
        compare_report: compare_dir.join("compare-report.md"),
        compare_summary: compare_dir.join("compare-summary.json"),
        compare_digest: compare_dir.join("compare-digest.json"),
        samples: diagnostic.samples,
        heat_balance_timesteps: diagnostic.heat_balance_timesteps,
        heat_balance_run_period_timesteps: diagnostic.heat_balance_run_period_timesteps,
        heat_balance_warmup: diagnostic.heat_balance_warmup.clone(),
        tolerance_policy: report_context.tolerance_label(),
        zone_air_algorithm: diagnostic.zone_air_algorithm,
        surface_iteration_count: diagnostic.surface_iteration_count,
        ctf_initial_history_policy: diagnostic.ctf_initial_history_policy,
        status: conformance.status,
    })
}

fn generate_conformance_heat_balance_diagnostic_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<HeatBalanceReportSummary, String> {
    let report_context = heat_balance_diagnostic_context_from_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");

    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let weather = baseline
        .weather
        .as_ref()
        .ok_or_else(|| "heat-balance diagnostic requires input.weather".to_string())?;
    let diagnostic = build_heat_balance_conformance_diagnostic(
        &baseline.epjson,
        weather,
        &baseline.eio,
        &baseline.eso,
        &report_context,
    )?;
    let conformance = evaluate_heat_balance_conformance(&diagnostic, &report_context);
    write_heat_balance_conformance_report(&compare_dir, &diagnostic, &conformance)?;

    Ok(HeatBalanceReportSummary {
        baseline,
        report_dir: compare_dir.clone(),
        compare_report: compare_dir.join("compare-report.md"),
        compare_summary: compare_dir.join("compare-summary.json"),
        compare_digest: compare_dir.join("compare-digest.json"),
        samples: diagnostic.samples,
        heat_balance_timesteps: diagnostic.heat_balance_timesteps,
        heat_balance_run_period_timesteps: diagnostic.heat_balance_run_period_timesteps,
        heat_balance_warmup: diagnostic.heat_balance_warmup.clone(),
        tolerance_policy: report_context.tolerance_label(),
        zone_air_algorithm: diagnostic.zone_air_algorithm,
        surface_iteration_count: diagnostic.surface_iteration_count,
        ctf_initial_history_policy: diagnostic.ctf_initial_history_policy,
        status: conformance.status,
    })
}

fn validate_zone_temperature_diagnostic_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    if manifest.comparison_class != ComparisonClass::DiagnosticOnly {
        return Err(format!(
            "diagnostic report requires comparison_class diagnostic-only, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if manifest.conformance_claim {
        return Err("diagnostic report cannot claim conformance".to_string());
    }
    if !manifest.tolerances.is_empty() {
        return Err("diagnostic report must not declare tolerance rules".to_string());
    }
    if manifest.outputs.len() != 1 {
        return Err(format!(
            "zone-temperature diagnostic requires exactly one output request, got {}",
            manifest.outputs.len()
        ));
    }

    let output = &manifest.outputs[0];
    if !output
        .variable
        .eq_ignore_ascii_case("Zone Mean Air Temperature")
    {
        return Err(format!(
            "zone-temperature diagnostic requires Zone Mean Air Temperature, got {}",
            output.variable
        ));
    }
    if output.frequency != OutputFrequency::Hourly {
        return Err(format!(
            "zone-temperature diagnostic requires hourly output, got {}",
            output_frequency_label(output.frequency)
        ));
    }
    if output.class != VariableClass::ZoneState {
        return Err(format!(
            "zone-temperature diagnostic requires zone-state class, got {}",
            variable_class_label(output.class)
        ));
    }
    if output.source != SourceArtifact::Eso {
        return Err(format!(
            "zone-temperature diagnostic requires eso source, got {}",
            source_artifact_label(output.source)
        ));
    }

    let Some(report) = manifest.report.as_ref() else {
        return Err("diagnostic report requires a report contract".to_string());
    };
    if report.path.trim().is_empty() {
        return Err("diagnostic report contract has an empty path".to_string());
    }

    let Some(gate) = manifest.gate.as_ref() else {
        return Err("diagnostic report requires a non-blocking gate contract".to_string());
    };
    if gate.script.trim().is_empty() {
        return Err("diagnostic gate contract has an empty script".to_string());
    }
    if gate.blocking {
        return Err("diagnostic report gate must be non-blocking".to_string());
    }

    Ok(())
}

fn validate_heat_balance_conformance_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    if manifest.comparison_class != ComparisonClass::Conformance {
        return Err(format!(
            "heat-balance conformance requires comparison_class conformance, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if !manifest.conformance_claim {
        return Err("heat-balance conformance requires conformance_claim=true".to_string());
    }
    if manifest.outputs.is_empty() {
        return Err("heat-balance conformance requires at least one output request".to_string());
    }

    for output in &manifest.outputs {
        if output.frequency != OutputFrequency::Hourly {
            return Err(format!(
                "heat-balance conformance requires hourly output, got {} for {}",
                output_frequency_label(output.frequency),
                output.variable
            ));
        }
        if output.source != SourceArtifact::Eso {
            return Err(format!(
                "heat-balance conformance requires eso source, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
        if !matches!(
            output.class,
            VariableClass::ZoneState | VariableClass::SurfaceState
        ) {
            return Err(format!(
                "heat-balance conformance requires zone-state or surface-state class, got {} for {}",
                variable_class_label(output.class),
                output.variable
            ));
        }
        if !is_supported_heat_balance_output_variable(&output.variable) {
            return Err(format!(
                "unsupported heat-balance conformance output variable: {}",
                output.variable
            ));
        }
    }

    let Some(report) = manifest.report.as_ref() else {
        return Err("heat-balance conformance requires a report contract".to_string());
    };
    if report.path.trim().is_empty() {
        return Err("heat-balance conformance report contract has an empty path".to_string());
    }

    let Some(gate) = manifest.gate.as_ref() else {
        return Err("heat-balance conformance requires a blocking gate contract".to_string());
    };
    if gate.script.trim().is_empty() {
        return Err("heat-balance conformance gate contract has an empty script".to_string());
    }
    if !gate.blocking {
        return Err("heat-balance conformance gate must be blocking".to_string());
    }

    for variable_class in heat_balance_variable_classes(manifest) {
        heat_balance_tolerance_from_manifest(manifest, variable_class)?;
    }

    Ok(())
}

fn validate_heat_balance_diagnostic_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    if manifest.comparison_class != ComparisonClass::DiagnosticOnly {
        return Err(format!(
            "heat-balance diagnostic requires comparison_class diagnostic-only, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if manifest.conformance_claim {
        return Err("heat-balance diagnostic cannot claim conformance".to_string());
    }
    if manifest.outputs.is_empty() {
        return Err("heat-balance diagnostic requires at least one output request".to_string());
    }

    for output in &manifest.outputs {
        if output.frequency != OutputFrequency::Hourly {
            return Err(format!(
                "heat-balance diagnostic requires hourly output, got {} for {}",
                output_frequency_label(output.frequency),
                output.variable
            ));
        }
        if output.source != SourceArtifact::Eso {
            return Err(format!(
                "heat-balance diagnostic requires eso source, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
        if !matches!(
            output.class,
            VariableClass::ZoneState | VariableClass::SurfaceState
        ) {
            return Err(format!(
                "heat-balance diagnostic requires zone-state or surface-state class, got {} for {}",
                variable_class_label(output.class),
                output.variable
            ));
        }
        if !is_supported_heat_balance_output_variable(&output.variable) {
            return Err(format!(
                "unsupported heat-balance diagnostic output variable: {}",
                output.variable
            ));
        }
        if output.level == Some(OutputLevel::Conformance) {
            return Err(format!(
                "heat-balance diagnostic output cannot use conformance level: {}",
                output.variable
            ));
        }
    }

    let Some(report) = manifest.report.as_ref() else {
        return Err("heat-balance diagnostic requires a report contract".to_string());
    };
    if report.path.trim().is_empty() {
        return Err("heat-balance diagnostic report contract has an empty path".to_string());
    }

    let Some(gate) = manifest.gate.as_ref() else {
        return Err("heat-balance diagnostic requires a non-blocking gate contract".to_string());
    };
    if gate.script.trim().is_empty() {
        return Err("heat-balance diagnostic gate contract has an empty script".to_string());
    }
    if gate.blocking {
        return Err("heat-balance diagnostic gate must be non-blocking".to_string());
    }

    for variable_class in heat_balance_variable_classes(manifest) {
        heat_balance_tolerance_from_manifest(manifest, variable_class)?;
    }

    Ok(())
}

fn first_value_label(values: &[f64]) -> String {
    values
        .first()
        .map_or_else(|| "missing".to_string(), |value| format!("{value:.6}"))
}

fn last_value_label(values: &[f64]) -> String {
    values
        .last()
        .map_or_else(|| "missing".to_string(), |value| format!("{value:.6}"))
}

fn min_value_label(values: &[f64]) -> String {
    values
        .iter()
        .copied()
        .reduce(f64::min)
        .map_or_else(|| "missing".to_string(), |value| format!("{value:.6}"))
}

fn max_value_label(values: &[f64]) -> String {
    values
        .iter()
        .copied()
        .reduce(f64::max)
        .map_or_else(|| "missing".to_string(), |value| format!("{value:.6}"))
}

fn mean_value_label(values: &[f64]) -> String {
    if values.is_empty() {
        return "missing".to_string();
    }
    format!("{:.6}", values.iter().sum::<f64>() / values.len() as f64)
}

fn nonzero_count(values: &[f64]) -> usize {
    values.iter().filter(|value| value.abs() > 1.0e-9).count()
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

fn write_node_state_projection_artifacts(
    output_dir: &Path,
    projection: &NodeStateProjection,
) -> Result<(), String> {
    std::fs::create_dir_all(output_dir)
        .map_err(|error| format!("failed to create node-state output directory: {error}"))?;
    std::fs::write(
        output_dir.join("node-state-summary.md"),
        render_node_state_projection_markdown(projection),
    )
    .map_err(|error| format!("failed to write node-state markdown summary: {error}"))?;
    std::fs::write(
        output_dir.join("node-state-summary.json"),
        render_node_state_projection_summary_json(projection),
    )
    .map_err(|error| format!("failed to write node-state JSON summary: {error}"))?;
    Ok(())
}

fn render_node_state_projection_markdown(projection: &NodeStateProjection) -> String {
    let mut report = String::new();
    report.push_str("# Node State Projection\n\n");
    report.push_str("runtime_class: ideal-loads-node-state-projection\n");
    report.push_str("comparison_class: diagnostic-only\n");
    report.push_str("conformance_claim: false\n");
    report.push_str("algorithm_parity: false\n");
    report.push_str("tolerance_policy: none\n");
    report.push_str("status: projected\n");
    report.push_str(&format!("samples: {}\n", projection.summary.samples));
    report.push_str(&format!("nodes: {}\n", projection.summary.node_count));
    report.push_str(&format!(
        "state_nodes: {}\n",
        projection.summary.state_node_count
    ));
    report.push_str(&format!("series: {}\n\n", projection.summary.series_count));
    report.push_str(&format!(
        "source_map: {}\n",
        projection.summary.evidence_policy.source_map_path
    ));
    report.push_str(&format!(
        "timestamp_rule: {}\n",
        projection.summary.evidence_policy.timestamp_rule
    ));
    report.push_str(&format!(
        "warmup_rule: {}\n",
        projection.summary.evidence_policy.warmup_rule
    ));
    report.push_str(&format!(
        "sentinel_rule: {}\n",
        projection.summary.evidence_policy.sentinel_rule
    ));
    report.push_str(&format!(
        "excluded_variable: {}\n\n",
        projection.summary.evidence_policy.excluded_variable
    ));
    report.push_str(
        "| key | role | variable | units | samples | first | last | nonzero_count | status |\n",
    );
    report.push_str("|---|---|---|---|---:|---:|---:|---:|---|\n");
    for series in &projection.results.series {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | projected |\n",
            markdown_cell(&series.key),
            projection_role_for_key(projection, &series.key),
            markdown_cell(&series.variable_name),
            markdown_cell(&series.units),
            series.values.len(),
            first_value_label(&series.values),
            last_value_label(&series.values),
            nonzero_count(&series.values)
        ));
    }
    report
}

fn render_node_state_projection_summary_json(projection: &NodeStateProjection) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str("  \"runtime_class\": \"ideal-loads-node-state-projection\",\n");
    json.push_str("  \"comparison_class\": \"diagnostic-only\",\n");
    json.push_str("  \"conformance_claim\": false,\n");
    json.push_str("  \"algorithm_parity\": false,\n");
    json.push_str("  \"tolerance_policy\": \"none\",\n");
    json.push_str("  \"status\": \"projected\",\n");
    json.push_str(&format!("  \"samples\": {},\n", projection.summary.samples));
    json.push_str(&format!(
        "  \"nodes\": {},\n",
        projection.summary.node_count
    ));
    json.push_str(&format!(
        "  \"state_nodes\": {},\n",
        projection.summary.state_node_count
    ));
    json.push_str(&format!(
        "  \"series\": {},\n",
        projection.summary.series_count
    ));
    json.push_str("  \"evidence_policy\": {\n");
    json.push_str(&format!(
        "    \"source_map\": {},\n",
        json_string(projection.summary.evidence_policy.source_map_path)
    ));
    json.push_str(&format!(
        "    \"timestamp_rule\": {},\n",
        json_string(projection.summary.evidence_policy.timestamp_rule)
    ));
    json.push_str(&format!(
        "    \"warmup_rule\": {},\n",
        json_string(projection.summary.evidence_policy.warmup_rule)
    ));
    json.push_str(&format!(
        "    \"sentinel_rule\": {},\n",
        json_string(projection.summary.evidence_policy.sentinel_rule)
    ));
    json.push_str(&format!(
        "    \"excluded_variable\": {}\n",
        json_string(projection.summary.evidence_policy.excluded_variable)
    ));
    json.push_str("  },\n");
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"summary_markdown\": \"node-state-summary.md\",\n");
    json.push_str("    \"summary_json\": \"node-state-summary.json\"\n");
    json.push_str("  },\n");
    json.push_str("  \"node_order\": [\n");
    for (index, node) in projection.summary.nodes.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"node_id\": {},\n", node.node_id.0));
        json.push_str(&format!(
            "      \"key\": {},\n",
            json_string(&node.node_name)
        ));
        json.push_str(&format!(
            "      \"role\": {}\n",
            json_string(node.role.label())
        ));
        if index + 1 == projection.summary.nodes.len() {
            json.push_str("    }\n");
        } else {
            json.push_str("    },\n");
        }
    }
    json.push_str("  ],\n");
    json.push_str("  \"result_series\": [\n");
    for (index, series) in projection.results.series.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"handle\": {},\n", series.handle.0));
        json.push_str(&format!("      \"key\": {},\n", json_string(&series.key)));
        json.push_str(&format!(
            "      \"role\": {},\n",
            json_string(projection_role_for_key(projection, &series.key))
        ));
        json.push_str(&format!(
            "      \"variable\": {},\n",
            json_string(&series.variable_name)
        ));
        json.push_str(&format!(
            "      \"units\": {},\n",
            json_string(&series.units)
        ));
        json.push_str(&format!("      \"samples\": {},\n", series.values.len()));
        json.push_str(&format!(
            "      \"first\": {},\n",
            json_optional_number(series.values.first().copied())
        ));
        json.push_str(&format!(
            "      \"last\": {},\n",
            json_optional_number(series.values.last().copied())
        ));
        json.push_str(&format!(
            "      \"nonzero_count\": {},\n",
            nonzero_count(&series.values)
        ));
        json.push_str("      \"status\": \"projected\"\n");
        if index + 1 == projection.results.series.len() {
            json.push_str("    }\n");
        } else {
            json.push_str("    },\n");
        }
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn projection_role_for_key<'a>(projection: &'a NodeStateProjection, key: &str) -> &'a str {
    projection
        .summary
        .nodes
        .iter()
        .find(|node| node.node_name == key)
        .map_or("unknown", |node| node.role.label())
}

fn write_plant_state_projection_artifacts(
    output_dir: &Path,
    projection: &PlantStateProjection,
) -> Result<(), String> {
    std::fs::create_dir_all(output_dir)
        .map_err(|error| format!("failed to create plant-state output directory: {error}"))?;
    std::fs::write(
        output_dir.join("plant-state-summary.md"),
        render_plant_state_projection_markdown(projection),
    )
    .map_err(|error| format!("failed to write plant-state markdown summary: {error}"))?;
    std::fs::write(
        output_dir.join("plant-state-summary.json"),
        render_plant_state_projection_summary_json(projection),
    )
    .map_err(|error| format!("failed to write plant-state JSON summary: {error}"))?;
    Ok(())
}

fn render_plant_state_projection_markdown(projection: &PlantStateProjection) -> String {
    let mut report = String::new();
    report.push_str("# Plant State Projection\n\n");
    report.push_str("runtime_class: plant-loop-state-projection\n");
    report.push_str("comparison_class: diagnostic-only\n");
    report.push_str("conformance_claim: false\n");
    report.push_str("algorithm_parity: false\n");
    report.push_str("tolerance_policy: none\n");
    report.push_str("status: projected\n");
    report.push_str(&format!("samples: {}\n", projection.summary.samples));
    report.push_str(&format!("loops: {}\n", projection.summary.loop_count));
    report.push_str(&format!(
        "equipment: {}\n",
        projection.summary.equipment_count
    ));
    report.push_str(&format!("series: {}\n\n", projection.summary.series_count));
    report.push_str(&format!(
        "source_map: {}\n",
        projection.summary.evidence_policy.source_map_path
    ));
    report.push_str(&format!(
        "timestamp_rule: {}\n",
        projection.summary.evidence_policy.timestamp_rule
    ));
    report.push_str(&format!(
        "warmup_rule: {}\n",
        projection.summary.evidence_policy.warmup_rule
    ));
    report.push_str(&format!(
        "sizing_rule: {}\n\n",
        projection.summary.evidence_policy.sizing_rule
    ));
    report.push_str(
        "| key | class | role | variable | units | samples | first | last | nonzero_count | status |\n",
    );
    report.push_str("|---|---|---|---|---|---:|---:|---:|---:|---|\n");
    for series in &projection.results.series {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | projected |\n",
            markdown_cell(&series.key),
            plant_projection_class_for_key(projection, &series.key),
            plant_projection_role_for_key(projection, &series.key),
            markdown_cell(&series.variable_name),
            markdown_cell(&series.units),
            series.values.len(),
            first_value_label(&series.values),
            last_value_label(&series.values),
            nonzero_count(&series.values)
        ));
    }
    report
}

fn render_plant_state_projection_summary_json(projection: &PlantStateProjection) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str("  \"runtime_class\": \"plant-loop-state-projection\",\n");
    json.push_str("  \"comparison_class\": \"diagnostic-only\",\n");
    json.push_str("  \"conformance_claim\": false,\n");
    json.push_str("  \"algorithm_parity\": false,\n");
    json.push_str("  \"tolerance_policy\": \"none\",\n");
    json.push_str("  \"status\": \"projected\",\n");
    json.push_str(&format!("  \"samples\": {},\n", projection.summary.samples));
    json.push_str(&format!(
        "  \"loops\": {},\n",
        projection.summary.loop_count
    ));
    json.push_str(&format!(
        "  \"equipment\": {},\n",
        projection.summary.equipment_count
    ));
    json.push_str(&format!(
        "  \"series\": {},\n",
        projection.summary.series_count
    ));
    json.push_str("  \"evidence_policy\": {\n");
    json.push_str(&format!(
        "    \"source_map\": {},\n",
        json_string(projection.summary.evidence_policy.source_map_path)
    ));
    json.push_str(&format!(
        "    \"timestamp_rule\": {},\n",
        json_string(projection.summary.evidence_policy.timestamp_rule)
    ));
    json.push_str(&format!(
        "    \"warmup_rule\": {},\n",
        json_string(projection.summary.evidence_policy.warmup_rule)
    ));
    json.push_str(&format!(
        "    \"sizing_rule\": {}\n",
        json_string(projection.summary.evidence_policy.sizing_rule)
    ));
    json.push_str("  },\n");
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"summary_markdown\": \"plant-state-summary.md\",\n");
    json.push_str("    \"summary_json\": \"plant-state-summary.json\"\n");
    json.push_str("  },\n");
    json.push_str("  \"loop_order\": [\n");
    for (index, plant_loop) in projection.summary.loops.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"loop_id\": {},\n", plant_loop.loop_id.0));
        json.push_str(&format!(
            "      \"key\": {},\n",
            json_string(&plant_loop.loop_name)
        ));
        json.push_str(&format!(
            "      \"supply_inlet_node\": {},\n",
            json_string(&plant_loop.supply_inlet_node_name)
        ));
        json.push_str(&format!(
            "      \"supply_outlet_node\": {}\n",
            json_string(&plant_loop.supply_outlet_node_name)
        ));
        if index + 1 == projection.summary.loops.len() {
            json.push_str("    }\n");
        } else {
            json.push_str("    },\n");
        }
    }
    json.push_str("  ],\n");
    json.push_str("  \"equipment_order\": [\n");
    for (index, equipment) in projection.summary.equipment.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"object_type\": {},\n",
            json_string(&equipment.object_type)
        ));
        json.push_str(&format!(
            "      \"key\": {},\n",
            json_string(&equipment.equipment_name)
        ));
        json.push_str(&format!(
            "      \"role\": {}\n",
            json_string(equipment.role.label())
        ));
        if index + 1 == projection.summary.equipment.len() {
            json.push_str("    }\n");
        } else {
            json.push_str("    },\n");
        }
    }
    json.push_str("  ],\n");
    json.push_str("  \"result_series\": [\n");
    for (index, series) in projection.results.series.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"handle\": {},\n", series.handle.0));
        json.push_str(&format!("      \"key\": {},\n", json_string(&series.key)));
        json.push_str(&format!(
            "      \"class\": {},\n",
            json_string(plant_projection_class_for_key(projection, &series.key))
        ));
        json.push_str(&format!(
            "      \"role\": {},\n",
            json_string(plant_projection_role_for_key(projection, &series.key))
        ));
        json.push_str(&format!(
            "      \"variable\": {},\n",
            json_string(&series.variable_name)
        ));
        json.push_str(&format!(
            "      \"units\": {},\n",
            json_string(&series.units)
        ));
        json.push_str(&format!("      \"samples\": {},\n", series.values.len()));
        json.push_str(&format!(
            "      \"first\": {},\n",
            json_optional_number(series.values.first().copied())
        ));
        json.push_str(&format!(
            "      \"last\": {},\n",
            json_optional_number(series.values.last().copied())
        ));
        json.push_str(&format!(
            "      \"nonzero_count\": {},\n",
            nonzero_count(&series.values)
        ));
        json.push_str("      \"status\": \"projected\"\n");
        if index + 1 == projection.results.series.len() {
            json.push_str("    }\n");
        } else {
            json.push_str("    },\n");
        }
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn plant_projection_class_for_key<'a>(projection: &'a PlantStateProjection, key: &str) -> &'a str {
    if projection
        .summary
        .loops
        .iter()
        .any(|plant_loop| plant_loop.loop_name == key)
    {
        "plant-state"
    } else {
        "plant-equipment"
    }
}

fn plant_projection_role_for_key<'a>(projection: &'a PlantStateProjection, key: &str) -> &'a str {
    if projection
        .summary
        .loops
        .iter()
        .any(|plant_loop| plant_loop.loop_name == key)
    {
        return "plant-loop";
    }

    projection
        .summary
        .equipment
        .iter()
        .find(|equipment| equipment.equipment_name == key)
        .map_or("unknown", |equipment| equipment.role.label())
}

fn resolve_manifest_path(case_path: &Path, value: &str) -> Result<PathBuf, std::io::Error> {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        return Ok(path);
    }

    let cwd_candidate = std::env::current_dir()?.join(&path);
    if cwd_candidate.exists() {
        return Ok(cwd_candidate);
    }

    let parent = case_path.parent().unwrap_or_else(|| Path::new("."));
    Ok(parent.join(path))
}

fn print_conformance_case_summary(manifest: &ConformanceCase) {
    println!("  id: {}", manifest.id);
    if let Some(metadata) = manifest.manifest_v2.as_ref() {
        println!("  schema_v2: {}", metadata.schema);
        println!(
            "  source_kind: {}",
            case_source_kind_label(metadata.source_kind)
        );
        println!("  source_file: {}", metadata.source_file);
        println!("  tier: {}", case_tier_label(metadata.tier));
    }
    if let Some(scope) = manifest.scope.as_ref() {
        let domains = scope
            .domains
            .iter()
            .map(|domain| evidence_domain_label(*domain))
            .collect::<Vec<_>>()
            .join(",");
        println!("  domains: {domains}");
        println!("  has_zone: {}", scope.has_zone);
        println!("  has_surface: {}", scope.has_surface);
        println!("  has_air_loop: {}", scope.has_air_loop);
        println!("  has_plant_loop: {}", scope.has_plant_loop);
    }
    println!(
        "  comparison_class: {}",
        comparison_class_label(manifest.comparison_class)
    );
    println!("  conformance_claim: {}", manifest.conformance_claim);
    println!("  oracle_version: {}", manifest.oracle_version);
    println!("  outputs: {}", manifest.outputs.len());
    for output in &manifest.outputs {
        println!(
            "    {} / {} / {} / {} / {}",
            output.key,
            output.variable,
            output_frequency_label(output.frequency),
            variable_class_label(output.class),
            source_artifact_label(output.source)
        );
        if let (Some(domain), Some(level)) = (output.domain, output.level) {
            println!(
                "      v2: domain={} level={}",
                evidence_domain_label(domain),
                output_level_label(level)
            );
        }
    }
    println!("  meters: {}", manifest.meters.len());
    for meter in &manifest.meters {
        println!(
            "    {} / {} / {} / {} / {}",
            meter.name,
            output_frequency_label(meter.frequency),
            source_artifact_label(meter.source),
            evidence_domain_label(meter.domain),
            output_level_label(meter.level)
        );
    }
}

fn case_source_kind_label(kind: CaseSourceKind) -> &'static str {
    match kind {
        CaseSourceKind::LocalFixture => "local-fixture",
        CaseSourceKind::EnergyPlusExamplefile => "energy-plus-examplefile",
        CaseSourceKind::EnergyPlusTestfile => "energy-plus-testfile",
        CaseSourceKind::MinimalEpjson => "minimal-epjson",
    }
}

fn case_tier_label(tier: CaseTier) -> &'static str {
    match tier {
        CaseTier::A => "A",
        CaseTier::B => "B",
        CaseTier::C => "C",
    }
}

fn evidence_domain_label(domain: EvidenceDomain) -> &'static str {
    match domain {
        EvidenceDomain::Weather => "weather",
        EvidenceDomain::Schedule => "schedule",
        EvidenceDomain::Zone => "zone",
        EvidenceDomain::Surface => "surface",
        EvidenceDomain::Construction => "construction",
        EvidenceDomain::InternalGain => "internal-gain",
        EvidenceDomain::Node => "node",
        EvidenceDomain::Hvac => "hvac",
        EvidenceDomain::Plant => "plant",
        EvidenceDomain::Meter => "meter",
        EvidenceDomain::Diagnostic => "diagnostic",
    }
}

fn output_level_label(level: OutputLevel) -> &'static str {
    match level {
        OutputLevel::Required => "required",
        OutputLevel::Optional => "optional",
        OutputLevel::Baseline => "baseline",
        OutputLevel::Diagnostic => "diagnostic",
        OutputLevel::Conformance => "conformance",
    }
}

fn comparison_class_label(class: ComparisonClass) -> &'static str {
    match class {
        ComparisonClass::Smoke => "smoke",
        ComparisonClass::DiagnosticOnly => "diagnostic-only",
        ComparisonClass::Conformance => "conformance",
        ComparisonClass::Regression => "regression",
        ComparisonClass::Performance => "performance",
    }
}

fn output_frequency_label(frequency: OutputFrequency) -> &'static str {
    match frequency {
        OutputFrequency::Static => "static",
        OutputFrequency::Timestep => "timestep",
        OutputFrequency::Hourly => "hourly",
        OutputFrequency::Daily => "daily",
        OutputFrequency::Monthly => "monthly",
        OutputFrequency::Annual => "annual",
        OutputFrequency::RunPeriod => "run-period",
    }
}

fn output_frequency_idf_label(frequency: OutputFrequency) -> &'static str {
    match frequency {
        OutputFrequency::Static => "RunPeriod",
        OutputFrequency::Timestep => "Timestep",
        OutputFrequency::Hourly => "Hourly",
        OutputFrequency::Daily => "Daily",
        OutputFrequency::Monthly => "Monthly",
        OutputFrequency::Annual => "Annual",
        OutputFrequency::RunPeriod => "RunPeriod",
    }
}

fn variable_class_label(class: VariableClass) -> &'static str {
    match class {
        VariableClass::Schedule => "schedule",
        VariableClass::Weather => "weather",
        VariableClass::ConstructionMaterial => "construction-material",
        VariableClass::InternalGain => "internal-gain",
        VariableClass::ZoneState => "zone-state",
        VariableClass::SurfaceState => "surface-state",
        VariableClass::NodeState => "node-state",
        VariableClass::HvacState => "hvac-state",
        VariableClass::PlantState => "plant-state",
        VariableClass::PlantEquipment => "plant-equipment",
        VariableClass::Meter => "meter",
        VariableClass::InternalVariable => "internal-variable",
        VariableClass::Diagnostic => "diagnostic",
    }
}

fn source_artifact_label(source: SourceArtifact) -> &'static str {
    match source {
        SourceArtifact::Eio => "eio",
        SourceArtifact::Eso => "eso",
        SourceArtifact::Mtr => "mtr",
        SourceArtifact::Sql => "sql",
        SourceArtifact::Csv => "csv",
    }
}

fn report_format_label(format: ReportFormat) -> &'static str {
    match format {
        ReportFormat::Markdown => "markdown",
        ReportFormat::Json => "json",
    }
}

fn run_run_command(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("first-zone") => run_first_zone_command(&args[1..]),
        Some("node-state-projection") => run_node_state_projection_command(&args[1..]),
        Some("plant-state-projection") => run_plant_state_projection_command(&args[1..]),
        Some(command) => {
            eprintln!("unsupported run command: {command}");
            eprintln!("usage: eplus-rs run first-zone <input.epJSON> <weather.epw> [--hours N]");
            eprintln!(
                "usage: eplus-rs run node-state-projection <input.epJSON> <output-dir> [--hours N]"
            );
            eprintln!(
                "usage: eplus-rs run plant-state-projection <input.epJSON> <output-dir> [--hours N]"
            );
            2
        }
        None => {
            eprintln!("missing run command");
            eprintln!("usage: eplus-rs run first-zone <input.epJSON> <weather.epw> [--hours N]");
            eprintln!(
                "usage: eplus-rs run node-state-projection <input.epJSON> <output-dir> [--hours N]"
            );
            eprintln!(
                "usage: eplus-rs run plant-state-projection <input.epJSON> <output-dir> [--hours N]"
            );
            2
        }
    }
}

fn run_first_zone_command(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs run first-zone <input.epJSON> <weather.epw> [--hours N]");
        return 2;
    };
    let Some(weather_path) = args.get(1) else {
        eprintln!("missing weather path");
        eprintln!("usage: eplus-rs run first-zone <input.epJSON> <weather.epw> [--hours N]");
        return 2;
    };
    let hours = match parse_hours_arg(&args[2..], 24) {
        Ok(hours) => hours,
        Err(error) => {
            eprintln!("{error}");
            eprintln!("usage: eplus-rs run first-zone <input.epJSON> <weather.epw> [--hours N]");
            return 2;
        }
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    let weather_values = match load_epw_dry_bulb_series(weather_path) {
        Ok(values) => values,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let simulation_model = SimulationModel::from_typed(model);
    let simulation = match simulate_first_zone_uncontrolled(
        &simulation_model,
        &weather_values,
        FirstZoneSimulationOptions::hourly_samples(hours),
    ) {
        Ok(simulation) => simulation,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let Some(zone_series) = simulation
        .results
        .find_series(&simulation.summary.zone_name, "Zone Mean Air Temperature")
    else {
        eprintln!("first-zone simulation did not write zone temperature output");
        return 1;
    };
    let first_zone_temp_c = zone_series.values.first().copied().unwrap_or(0.0);
    let last_zone_temp_c = zone_series.values.last().copied().unwrap_or(0.0);

    println!("First Zone Runtime Diagnostic");
    println!("  runtime_class: diagnostic-toy");
    println!("  conformance_claim: false");
    println!("  algorithm_parity: false");
    println!("  zone: {}", simulation.summary.zone_name);
    println!("  samples: {}", simulation.summary.samples);
    println!("  result_series: {}", simulation.results.series.len());
    println!("  volume_m3: {:.6}", simulation.summary.volume_m3);
    println!(
        "  exterior_area_m2: {:.6}",
        simulation.summary.exterior_area_m2
    );
    println!(
        "  conductance_w_per_k: {:.6}",
        simulation.summary.conductance_w_per_k
    );
    println!(
        "  internal_gain_w: {:.6}",
        simulation.summary.internal_gain_w
    );
    println!("  first_zone_temp_c: {first_zone_temp_c:.6}");
    println!("  last_zone_temp_c: {last_zone_temp_c:.6}");
    println!("  status: extracted");

    0
}

fn run_node_state_projection_command(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!(
            "usage: eplus-rs run node-state-projection <input.epJSON> <output-dir> [--hours N]"
        );
        return 2;
    };
    let Some(output_dir) = args.get(1) else {
        eprintln!("missing output directory");
        eprintln!(
            "usage: eplus-rs run node-state-projection <input.epJSON> <output-dir> [--hours N]"
        );
        return 2;
    };
    let hours = match parse_hours_arg(&args[2..], 24) {
        Ok(hours) => hours,
        Err(error) => {
            eprintln!("{error}");
            eprintln!(
                "usage: eplus-rs run node-state-projection <input.epJSON> <output-dir> [--hours N]"
            );
            return 2;
        }
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    let simulation_model = SimulationModel::from_typed(model);
    let projection = match simulate_ideal_loads_node_state_projection(
        &simulation_model,
        NodeStateProjectionOptions::hourly_samples(hours),
    ) {
        Ok(projection) => projection,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let output_dir = Path::new(output_dir);
    if let Err(error) = write_node_state_projection_artifacts(output_dir, &projection) {
        eprintln!("{error}");
        return 1;
    }

    println!("Node State Projection");
    println!("  runtime_class: ideal-loads-node-state-projection");
    println!("  comparison_class: diagnostic-only");
    println!("  conformance_claim: false");
    println!("  algorithm_parity: false");
    println!("  tolerance_policy: none");
    println!("  nodes: {}", projection.summary.node_count);
    println!("  state_nodes: {}", projection.summary.state_node_count);
    println!("  samples: {}", projection.summary.samples);
    println!("  series: {}", projection.summary.series_count);
    println!(
        "  summary_json: {}",
        output_dir.join("node-state-summary.json").display()
    );
    println!("  status: projected");

    0
}

fn run_plant_state_projection_command(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!(
            "usage: eplus-rs run plant-state-projection <input.epJSON> <output-dir> [--hours N]"
        );
        return 2;
    };
    let Some(output_dir) = args.get(1) else {
        eprintln!("missing output directory");
        eprintln!(
            "usage: eplus-rs run plant-state-projection <input.epJSON> <output-dir> [--hours N]"
        );
        return 2;
    };
    let hours = match parse_hours_arg(&args[2..], 48) {
        Ok(hours) => hours,
        Err(error) => {
            eprintln!("{error}");
            eprintln!(
                "usage: eplus-rs run plant-state-projection <input.epJSON> <output-dir> [--hours N]"
            );
            return 2;
        }
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    let simulation_model = SimulationModel::from_typed(model);
    let projection = match simulate_plant_state_projection(
        &simulation_model,
        PlantStateProjectionOptions::hourly_samples(hours),
    ) {
        Ok(projection) => projection,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let output_dir = Path::new(output_dir);
    if let Err(error) = write_plant_state_projection_artifacts(output_dir, &projection) {
        eprintln!("{error}");
        return 1;
    }

    println!("Plant State Projection");
    println!("  runtime_class: plant-loop-state-projection");
    println!("  comparison_class: diagnostic-only");
    println!("  conformance_claim: false");
    println!("  algorithm_parity: false");
    println!("  tolerance_policy: none");
    println!("  loops: {}", projection.summary.loop_count);
    println!("  equipment: {}", projection.summary.equipment_count);
    println!("  samples: {}", projection.summary.samples);
    println!("  series: {}", projection.summary.series_count);
    println!(
        "  summary_json: {}",
        output_dir.join("plant-state-summary.json").display()
    );
    println!("  status: projected");

    0
}

fn parse_hours_arg(args: &[String], default: usize) -> Result<usize, String> {
    let mut hours = default;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--hours" => {
                let Some(value) = args.get(index + 1) else {
                    return Err("--hours requires a positive integer".to_string());
                };
                hours = value
                    .parse::<usize>()
                    .map_err(|_error| "--hours requires a positive integer".to_string())?;
                if hours == 0 {
                    return Err("--hours requires a positive integer".to_string());
                }
                index += 2;
            }
            option => return Err(format!("unsupported option: {option}")),
        }
    }

    Ok(hours)
}

fn run_compare_command(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("schedule-value") => run_compare_schedule_value(&args[1..]),
        Some("geometry") => run_compare_geometry(&args[1..]),
        Some("surface-geometry") => run_compare_surface_geometry(&args[1..]),
        Some("construction-materials") => run_compare_construction_materials(&args[1..]),
        Some("internal-gains") => run_compare_internal_gains(&args[1..]),
        Some("internal-convective-gain") => run_compare_internal_convective_gain(&args[1..]),
        Some("weather-fields") | Some("weather-drybulb") => run_compare_weather_fields(&args[1..]),
        Some("zone-temperature") => run_compare_zone_temperature(&args[1..]),
        Some(command) => {
            eprintln!("unsupported compare command: {command}");
            eprintln!("usage: eplus-rs compare schedule-value <input.epJSON> <eplusout.eso>");
            eprintln!("usage: eplus-rs compare geometry <input.epJSON> <eplusout.eio>");
            eprintln!("usage: eplus-rs compare surface-geometry <input.epJSON> <eplusout.eio>");
            eprintln!(
                "usage: eplus-rs compare construction-materials <input.epJSON> <eplusout.eio>"
            );
            eprintln!("usage: eplus-rs compare internal-gains <input.epJSON> <eplusout.eio>");
            eprintln!(
                "usage: eplus-rs compare internal-convective-gain <input.epJSON> <eplusout.eso>"
            );
            eprintln!("usage: eplus-rs compare weather-fields <weather.epw> <eplusout.eso>");
            eprintln!("usage: eplus-rs compare weather-drybulb <weather.epw> <eplusout.eso>");
            eprintln!("{ZONE_TEMPERATURE_COMPARE_USAGE}");
            2
        }
        None => {
            eprintln!("missing compare command");
            eprintln!("usage: eplus-rs compare schedule-value <input.epJSON> <eplusout.eso>");
            eprintln!("usage: eplus-rs compare geometry <input.epJSON> <eplusout.eio>");
            eprintln!("usage: eplus-rs compare surface-geometry <input.epJSON> <eplusout.eio>");
            eprintln!(
                "usage: eplus-rs compare construction-materials <input.epJSON> <eplusout.eio>"
            );
            eprintln!("usage: eplus-rs compare internal-gains <input.epJSON> <eplusout.eio>");
            eprintln!(
                "usage: eplus-rs compare internal-convective-gain <input.epJSON> <eplusout.eso>"
            );
            eprintln!("usage: eplus-rs compare weather-fields <weather.epw> <eplusout.eso>");
            eprintln!("usage: eplus-rs compare weather-drybulb <weather.epw> <eplusout.eso>");
            eprintln!("{ZONE_TEMPERATURE_COMPARE_USAGE}");
            2
        }
    }
}

fn run_compare_schedule_value(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs compare schedule-value <input.epJSON> <eplusout.eso>");
        return 2;
    };
    let Some(eso_path) = args.get(1) else {
        eprintln!("missing eplusout.eso path");
        eprintln!("usage: eplus-rs compare schedule-value <input.epJSON> <eplusout.eso>");
        return 2;
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model.as_ref() else {
        print_compile_diagnostics(&result.report);
        return 1;
    };

    if model.schedules.is_empty() {
        eprintln!("no Schedule:Constant objects are available for comparison");
        return 1;
    }

    let mut oracle_series = Vec::new();
    for schedule in &model.schedules {
        let values = match load_eso_series(eso_path, &schedule.name.0, "Schedule Value") {
            Ok(values) => values,
            Err(error) => {
                eprintln!("{error}");
                return 1;
            }
        };
        oracle_series.push((schedule.id, schedule.name.0.clone(), values));
    }

    let sample_count = oracle_series
        .iter()
        .map(|(_id, _name, values)| values.len())
        .max()
        .unwrap_or(0);
    let traces = simulate_constant_schedules(model, sample_count);
    let mut passed = true;

    println!("Schedule Value Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: default");
    println!("  schedules: {}", oracle_series.len());
    for (schedule_id, schedule_name, expected_values) in oracle_series {
        let Some(trace) = traces.iter().find(|trace| trace.schedule_id == schedule_id) else {
            eprintln!("missing Rust schedule trace: {schedule_name}");
            return 1;
        };
        let comparison = compare_series(
            &expected_values,
            &trace.values[..expected_values.len()],
            Tolerance::default(),
        );
        if !comparison.passed {
            passed = false;
        }
        println!(
            "  schedule: {} samples: {} max_abs_delta: {} status: {}",
            schedule_name,
            comparison.samples,
            comparison.max_abs_delta,
            if comparison.passed { "pass" } else { "fail" }
        );
        print_first_divergence("  ", comparison.first_divergence);
    }
    println!("  status: {}", if passed { "pass" } else { "fail" });

    if passed { 0 } else { 1 }
}

fn run_compare_geometry(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs compare geometry <input.epJSON> <eplusout.eio>");
        return 2;
    };
    let Some(eio_path) = args.get(1) else {
        eprintln!("missing eplusout.eio path");
        eprintln!("usage: eplus-rs compare geometry <input.epJSON> <eplusout.eio>");
        return 2;
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    let rust_zones = zone_geometry_summaries(&model);
    if rust_zones.is_empty() {
        eprintln!("no Zone objects are available for geometry comparison");
        return 1;
    }

    let oracle_zones = match load_eio_zone_geometry(eio_path) {
        Ok(zones) => zones,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    let tolerance = Tolerance {
        absolute: 0.02,
        relative: 1.0e-6,
    };
    let mut passed = rust_zones.len() == oracle_zones.len();
    let mut first_divergence = None;
    if rust_zones.len() != oracle_zones.len() {
        first_divergence = Some(format!(
            "zone_count expected {} observed {}",
            oracle_zones.len(),
            rust_zones.len()
        ));
    }

    println!("Geometry Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: absolute-0.02");
    println!("  zones: {}", rust_zones.len());
    println!("  oracle_zones: {}", oracle_zones.len());
    for rust_zone in &rust_zones {
        let Some(oracle_zone) = oracle_zones
            .iter()
            .find(|zone| zone.zone_name.eq_ignore_ascii_case(&rust_zone.zone_name))
        else {
            passed = false;
            record_first_divergence(
                &mut first_divergence,
                format!("zone {} missing_in_eio", rust_zone.zone_name),
            );
            println!("  zone: {} status: fail", rust_zone.zone_name);
            continue;
        };

        let surface_pass = oracle_zone.surface_count == rust_zone.surface_count;
        let floor_area_pass = tolerance.accepts(oracle_zone.floor_area_m2, rust_zone.floor_area_m2);
        let volume_pass = rust_zone
            .volume_m3
            .is_some_and(|volume_m3| tolerance.accepts(oracle_zone.volume_m3, volume_m3));
        let exterior_wall_area_pass = tolerance.accepts(
            oracle_zone.exterior_gross_wall_area_m2,
            rust_zone.exterior_wall_area_m2,
        );
        let zone_pass = surface_pass && floor_area_pass && volume_pass && exterior_wall_area_pass;
        if !zone_pass {
            passed = false;
            record_geometry_field_divergence(
                &mut first_divergence,
                rust_zone,
                oracle_zone,
                tolerance,
            );
        }

        let rust_volume = rust_zone
            .volume_m3
            .map(|volume_m3| format!("{volume_m3:.6}"))
            .unwrap_or_else(|| "unavailable".to_string());
        println!(
            "  zone: {} surfaces: {}/{} floor_area_m2: {:.6}/{:.6} volume_m3: {:.6}/{} exterior_wall_area_m2: {:.6}/{:.6} status: {}",
            rust_zone.zone_name,
            oracle_zone.surface_count,
            rust_zone.surface_count,
            oracle_zone.floor_area_m2,
            rust_zone.floor_area_m2,
            oracle_zone.volume_m3,
            rust_volume,
            oracle_zone.exterior_gross_wall_area_m2,
            rust_zone.exterior_wall_area_m2,
            if zone_pass { "pass" } else { "fail" }
        );
    }

    println!(
        "  first_divergence: {}",
        first_divergence.unwrap_or_else(|| "none".to_string())
    );
    println!("  status: {}", if passed { "pass" } else { "fail" });

    if passed { 0 } else { 1 }
}

fn run_compare_surface_geometry(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs compare surface-geometry <input.epJSON> <eplusout.eio>");
        return 2;
    };
    let Some(eio_path) = args.get(1) else {
        eprintln!("missing eplusout.eio path");
        eprintln!("usage: eplus-rs compare surface-geometry <input.epJSON> <eplusout.eio>");
        return 2;
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    let rust_surfaces = surface_geometry_summaries(&model);
    if rust_surfaces.is_empty() {
        eprintln!(
            "no BuildingSurface:Detailed objects are available for surface geometry comparison"
        );
        return 1;
    }

    let oracle_surfaces = match load_eio_heat_transfer_surfaces(eio_path) {
        Ok(surfaces) => surfaces,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    let tolerance = Tolerance {
        absolute: 0.01,
        relative: 1.0e-6,
    };
    let mut passed = rust_surfaces.len() == oracle_surfaces.len();
    let mut first_divergence = None;
    if rust_surfaces.len() != oracle_surfaces.len() {
        first_divergence = Some(format!(
            "surface_count expected {} observed {}",
            oracle_surfaces.len(),
            rust_surfaces.len()
        ));
    }

    println!("Surface Geometry Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: absolute-0.01-relative-0.000001");
    println!("  surfaces: {}", rust_surfaces.len());
    println!("  oracle_surfaces: {}", oracle_surfaces.len());
    for rust_surface in &rust_surfaces {
        let Some(oracle_surface) = oracle_surfaces.iter().find(|surface| {
            surface
                .surface_name
                .eq_ignore_ascii_case(&rust_surface.surface_name)
        }) else {
            passed = false;
            record_first_divergence(
                &mut first_divergence,
                format!("surface {} missing_in_eio", rust_surface.surface_name),
            );
            println!("  surface: {} status: fail", rust_surface.surface_name);
            continue;
        };

        let surface_pass = surface_geometry_row_matches(rust_surface, oracle_surface, tolerance);
        if !surface_pass {
            passed = false;
            record_surface_geometry_field_divergence(
                &mut first_divergence,
                rust_surface,
                oracle_surface,
                tolerance,
            );
        }

        println!(
            "  surface: {} class: {}/{} area_net_m2: {:.6}/{:.6} area_gross_m2: {:.6}/{:.6} azimuth_deg: {:.6}/{:.6} tilt_deg: {:.6}/{:.6} zone: {} status: {}",
            rust_surface.surface_name,
            oracle_surface.surface_class,
            surface_type_label(rust_surface.surface_type),
            oracle_surface.area_net_m2,
            rust_surface.area_m2,
            oracle_surface.area_gross_m2,
            rust_surface.area_m2,
            oracle_surface.azimuth_deg,
            rust_surface.azimuth_deg,
            oracle_surface.tilt_deg,
            rust_surface.tilt_deg,
            rust_surface.zone_name,
            if surface_pass { "pass" } else { "fail" }
        );
    }

    println!(
        "  first_divergence: {}",
        first_divergence.unwrap_or_else(|| "none".to_string())
    );
    println!("  status: {}", if passed { "pass" } else { "fail" });

    if passed { 0 } else { 1 }
}

fn run_compare_construction_materials(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs compare construction-materials <input.epJSON> <eplusout.eio>");
        return 2;
    };
    let Some(eio_path) = args.get(1) else {
        eprintln!("missing eplusout.eio path");
        eprintln!("usage: eplus-rs compare construction-materials <input.epJSON> <eplusout.eio>");
        return 2;
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    let rust_rows = match construction_material_rows(&model) {
        Ok(rows) => rows,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if rust_rows.is_empty() {
        eprintln!("no Construction objects are available for construction/material comparison");
        return 1;
    }

    let oracle_constructions = match load_eio_construction_ctf(eio_path) {
        Ok(constructions) => constructions,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let oracle_materials = match load_eio_material_ctf_summary(eio_path) {
        Ok(materials) => materials,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    let rust_material_count = construction_layer_material_count(&model);
    let tolerance = Tolerance {
        absolute: 0.001,
        relative: 1.0e-6,
    };
    let mut passed = rust_rows.len() == oracle_constructions.len();
    let mut first_divergence = None;
    if rust_rows.len() != oracle_constructions.len() {
        first_divergence = Some(format!(
            "construction_count expected {} observed {}",
            oracle_constructions.len(),
            rust_rows.len()
        ));
    }

    println!("Construction Material Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: absolute-0.001");
    println!("  constructions: {}", rust_rows.len());
    println!("  oracle_constructions: {}", oracle_constructions.len());
    println!("  materials: {}", rust_material_count);
    println!("  oracle_materials: {}", oracle_materials.len());
    for rust_row in &rust_rows {
        let Some(oracle_construction) = oracle_constructions.iter().find(|row| {
            row.construction_name
                .eq_ignore_ascii_case(&rust_row.construction_name)
        }) else {
            passed = false;
            record_first_divergence(
                &mut first_divergence,
                format!("construction {} missing_in_eio", rust_row.construction_name),
            );
            println!(
                "  construction: {} status: fail",
                rust_row.construction_name
            );
            continue;
        };
        let Some(oracle_material) = oracle_materials.iter().find(|row| {
            row.material_name
                .eq_ignore_ascii_case(&rust_row.outside_layer_material_name)
        }) else {
            passed = false;
            record_first_divergence(
                &mut first_divergence,
                format!(
                    "construction {} material {} missing_in_eio",
                    rust_row.construction_name, rust_row.outside_layer_material_name
                ),
            );
            println!(
                "  construction: {} status: fail",
                rust_row.construction_name
            );
            continue;
        };

        let row_pass = construction_material_row_matches(
            rust_row,
            oracle_construction,
            oracle_material,
            tolerance,
        );
        if !row_pass {
            passed = false;
            record_construction_material_field_divergence(
                &mut first_divergence,
                rust_row,
                oracle_construction,
                oracle_material,
                tolerance,
            );
        }

        println!(
            "  construction: {} layers: {}/{} material: {}/{} thermal_conductance_w_per_m2_k: {:.6}/{:.6} material_resistance_m2_k_per_w: {:.6}/{:.6} status: {}",
            rust_row.construction_name,
            oracle_construction.layer_count,
            rust_row.layer_count,
            oracle_material.material_name,
            rust_row.outside_layer_material_name,
            oracle_construction.thermal_conductance_w_per_m2_k,
            rust_row.thermal_conductance_w_per_m2_k,
            oracle_material.thermal_resistance_m2_k_per_w,
            rust_row.material_thermal_resistance_m2_k_per_w,
            if row_pass { "pass" } else { "fail" }
        );
    }

    println!(
        "  first_divergence: {}",
        first_divergence.unwrap_or_else(|| "none".to_string())
    );
    println!("  status: {}", if passed { "pass" } else { "fail" });

    if passed { 0 } else { 1 }
}

fn run_compare_internal_gains(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs compare internal-gains <input.epJSON> <eplusout.eio>");
        return 2;
    };
    let Some(eio_path) = args.get(1) else {
        eprintln!("missing eplusout.eio path");
        eprintln!("usage: eplus-rs compare internal-gains <input.epJSON> <eplusout.eio>");
        return 2;
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    let rust_equipment = other_equipment_nominal_rows(&model);
    if rust_equipment.is_empty() {
        eprintln!("no OtherEquipment objects are available for internal-gains comparison");
        return 1;
    }

    let oracle_equipment = match load_eio_other_equipment_nominal(eio_path) {
        Ok(equipment) => equipment,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    let tolerance = Tolerance {
        absolute: 0.02,
        relative: 1.0e-6,
    };
    let mut passed = rust_equipment.len() == oracle_equipment.len();
    let mut first_divergence = None;
    if rust_equipment.len() != oracle_equipment.len() {
        first_divergence = Some(format!(
            "other_equipment_count expected {} observed {}",
            oracle_equipment.len(),
            rust_equipment.len()
        ));
    }

    println!("OtherEquipment Internal Gains Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: absolute-0.02");
    println!("  other_equipment: {}", rust_equipment.len());
    println!("  oracle_other_equipment: {}", oracle_equipment.len());
    for rust_row in &rust_equipment {
        let Some(oracle_row) = oracle_equipment.iter().find(|row| {
            row.equipment_name
                .eq_ignore_ascii_case(&rust_row.equipment_name)
        }) else {
            passed = false;
            record_first_divergence(
                &mut first_divergence,
                format!("other_equipment {} missing_in_eio", rust_row.equipment_name),
            );
            println!(
                "  other_equipment: {} status: fail",
                rust_row.equipment_name
            );
            continue;
        };

        let row_pass = internal_gain_row_matches(rust_row, oracle_row, tolerance);
        if !row_pass {
            passed = false;
            record_internal_gain_field_divergence(
                &mut first_divergence,
                rust_row,
                oracle_row,
                tolerance,
            );
        }

        println!(
            "  other_equipment: {} zone: {}/{} schedule: {}/{} design_level_w: {:.6}/{:.6} floor_area_m2: {:.6}/{:.6} equipment_per_floor_area_w_per_m2: {:.6}/{:.6} fractions_latent_radiant_lost_convected: {:.6},{:.6},{:.6},{:.6}/{:.6},{:.6},{:.6},{:.6} status: {}",
            rust_row.equipment_name,
            oracle_row.zone_name,
            rust_row.zone_name,
            oracle_row.schedule_name,
            rust_row.schedule_name,
            oracle_row.equipment_level_w,
            rust_row.equipment_level_w,
            oracle_row.zone_floor_area_m2,
            rust_row.zone_floor_area_m2,
            oracle_row.equipment_per_floor_area_w_per_m2,
            rust_row.equipment_per_floor_area_w_per_m2,
            oracle_row.fraction_latent,
            oracle_row.fraction_radiant,
            oracle_row.fraction_lost,
            oracle_row.fraction_convected,
            rust_row.fraction_latent,
            rust_row.fraction_radiant,
            rust_row.fraction_lost,
            rust_row.fraction_convected,
            if row_pass { "pass" } else { "fail" }
        );
    }

    println!(
        "  first_divergence: {}",
        first_divergence.unwrap_or_else(|| "none".to_string())
    );
    println!("  status: {}", if passed { "pass" } else { "fail" });

    if passed { 0 } else { 1 }
}

#[derive(Clone, Debug, PartialEq)]
struct ZoneTemperatureCompareArgs {
    input_path: PathBuf,
    weather_path: PathBuf,
    eso_path: PathBuf,
    report_dir: Option<PathBuf>,
}

fn parse_zone_temperature_compare_args(
    args: &[String],
) -> Result<ZoneTemperatureCompareArgs, String> {
    let Some(input_path) = args.first() else {
        return Err("missing input path".to_string());
    };
    let Some(weather_path) = args.get(1) else {
        return Err("missing weather path".to_string());
    };
    let Some(eso_path) = args.get(2) else {
        return Err("missing eplusout.eso path".to_string());
    };

    let mut report_dir = None;
    let mut index = 3;
    while index < args.len() {
        match args[index].as_str() {
            "--report-dir" => {
                let Some(value) = args.get(index + 1) else {
                    return Err("missing --report-dir value".to_string());
                };
                report_dir = Some(PathBuf::from(value));
                index += 2;
            }
            option => {
                return Err(format!(
                    "unsupported compare zone-temperature argument: {option}"
                ));
            }
        }
    }

    Ok(ZoneTemperatureCompareArgs {
        input_path: PathBuf::from(input_path),
        weather_path: PathBuf::from(weather_path),
        eso_path: PathBuf::from(eso_path),
        report_dir,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DeltaPoint {
    index: usize,
    oracle_c: f64,
    rust_c: f64,
    abs_delta_c: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DeltaSummary {
    samples: usize,
    max_abs_delta_c: f64,
    mean_abs_delta_c: f64,
    rmse_delta_c: f64,
    max_rel_delta: f64,
    first_delta_sample: Option<DeltaPoint>,
    max_delta_sample: Option<DeltaPoint>,
    length_match: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct ZoneTemperatureDiagnostic {
    zone_name: String,
    samples: usize,
    heat_balance_timesteps: usize,
    heat_balance_run_period_timesteps: usize,
    heat_balance_warmup: HeatBalanceWarmupDiagnostic,
    zone_count: usize,
    surface_count: usize,
    oracle_first_c: f64,
    rust_first_c: f64,
    oracle_last_c: f64,
    rust_last_c: f64,
    delta: DeltaSummary,
    status: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ZoneTemperatureReportContext {
    case_id: String,
    oracle_version: String,
    output: ZoneTemperatureReportOutput,
    report: Option<ZoneTemperatureReportContract>,
    gate: Option<ZoneTemperatureGateContract>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ZoneTemperatureReportOutput {
    key: String,
    variable: String,
    frequency: &'static str,
    class: &'static str,
    source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ZoneTemperatureReportContract {
    format: &'static str,
    path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ZoneTemperatureGateContract {
    script: String,
    blocking: bool,
}

fn zone_temperature_report_context_from_manifest(
    manifest: &ConformanceCase,
) -> Result<ZoneTemperatureReportContext, String> {
    validate_zone_temperature_diagnostic_manifest(manifest)?;
    let output = manifest
        .outputs
        .first()
        .ok_or_else(|| "zone-temperature diagnostic requires one output request".to_string())?;

    Ok(ZoneTemperatureReportContext {
        case_id: manifest.id.clone(),
        oracle_version: manifest.oracle_version.clone(),
        output: ZoneTemperatureReportOutput {
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output_frequency_label(output.frequency),
            class: variable_class_label(output.class),
            source: source_artifact_label(output.source),
        },
        report: manifest
            .report
            .as_ref()
            .map(|report| ZoneTemperatureReportContract {
                format: report_format_label(report.format),
                path: report.path.clone(),
            }),
        gate: manifest
            .gate
            .as_ref()
            .map(|gate| ZoneTemperatureGateContract {
                script: gate.script.clone(),
                blocking: gate.blocking,
            }),
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct HeatBalanceToleranceReport {
    variable_class_label: &'static str,
    max_abs_c: Option<f64>,
    max_rmse_c: Option<f64>,
    max_rel: Option<f64>,
}

impl HeatBalanceToleranceReport {
    fn label(self) -> String {
        let max_abs = optional_tolerance_label(self.max_abs_c);
        let max_rmse = optional_tolerance_label(self.max_rmse_c);
        let max_rel = optional_tolerance_label(self.max_rel);
        format!(
            "{} max_abs={} max_rmse={} max_rel={}",
            self.variable_class_label, max_abs, max_rmse, max_rel
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceConformanceContext {
    case_id: String,
    oracle_version: String,
    outputs: Vec<ZoneTemperatureReportOutput>,
    tolerances: Vec<HeatBalanceToleranceReport>,
    report: Option<ZoneTemperatureReportContract>,
    gate: Option<ZoneTemperatureGateContract>,
    comparison_class: &'static str,
    conformance_claim: bool,
}

impl HeatBalanceConformanceContext {
    fn tolerance_label(&self) -> String {
        self.tolerances
            .iter()
            .map(|tolerance| tolerance.label())
            .collect::<Vec<_>>()
            .join("; ")
    }

    fn tolerance_for_class(&self, variable_class: &str) -> Option<HeatBalanceToleranceReport> {
        self.tolerances
            .iter()
            .copied()
            .find(|tolerance| tolerance.variable_class_label == variable_class)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceSeriesDiagnostic {
    output: ZoneTemperatureReportOutput,
    samples: usize,
    oracle_first_c: f64,
    rust_first_c: f64,
    oracle_last_c: f64,
    rust_last_c: f64,
    delta: DeltaSummary,
    sample_rows: Vec<DeltaPoint>,
    status: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceMaxSampleContext {
    trigger_rank: usize,
    trigger_output: ZoneTemperatureReportOutput,
    sample_index: usize,
    rows: Vec<HeatBalanceMaxSampleContextRow>,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceMaxSampleContextRow {
    output: ZoneTemperatureReportOutput,
    oracle_c: f64,
    rust_c: f64,
    abs_delta_c: f64,
    series_rmse_delta_c: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceConformanceDiagnostic {
    samples: usize,
    heat_balance_timesteps: usize,
    heat_balance_run_period_timesteps: usize,
    heat_balance_warmup: HeatBalanceWarmupDiagnostic,
    ctf_seed: HeatBalanceCtfSeedDiagnostic,
    zone_air_algorithm: &'static str,
    surface_iteration_count: u32,
    ctf_initial_history_policy: &'static str,
    zone_count: usize,
    surface_count: usize,
    ctf_component_first_samples: Vec<HeatBalanceCtfComponentFirstSample>,
    ctf_history_first_sample_deltas: Vec<HeatBalanceCtfHistoryFirstSampleDelta>,
    ctf_history_series_deltas: Vec<HeatBalanceCtfHistorySeriesDelta>,
    ctf_storage_max_sample_deltas: Vec<HeatBalanceCtfStorageMaxSampleDelta>,
    inside_balance_max_sample_deltas: Vec<HeatBalanceInsideBalanceMaxSampleDelta>,
    inside_solve_max_sample_deltas: Vec<HeatBalanceInsideSolveMaxSampleDelta>,
    adiabatic_history_max_sample_deltas: Vec<HeatBalanceAdiabaticHistoryMaxSampleDelta>,
    ctf_history_run_period_initial_slots: Vec<HeatBalanceCtfHistorySlotSample>,
    ctf_history_first_sample_slots: Vec<HeatBalanceCtfHistorySlotFirstSample>,
    surface_first_sample_trace: Vec<HeatBalanceSurfaceFirstSampleTrace>,
    series: Vec<HeatBalanceSeriesDiagnostic>,
    status: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceCtfComponentFirstSample {
    key: String,
    inside_conduction_rate_w: f64,
    inside_current_outside_term_w: f64,
    inside_current_inside_term_w: f64,
    inside_history_term_w: f64,
    outside_conduction_rate_w: f64,
    outside_current_outside_term_w: f64,
    outside_current_inside_term_w: f64,
    outside_history_term_w: f64,
    heat_storage_rate_w: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceCtfHistoryFirstSampleDelta {
    key: String,
    construction_name: String,
    area_m2: f64,
    ctf_outside_0_w_per_m2_k: f64,
    ctf_cross_0_w_per_m2_k: f64,
    ctf_inside_0_w_per_m2_k: f64,
    oracle_outside_face_temperature_c: f64,
    rust_outside_face_temperature_c: f64,
    outside_face_temperature_delta_c: f64,
    oracle_inside_face_temperature_c: f64,
    rust_inside_face_temperature_c: f64,
    inside_face_temperature_delta_c: f64,
    oracle_inside_current_term_w: f64,
    rust_inside_current_term_w: f64,
    inside_current_delta_w: f64,
    oracle_inside_history_term_w: f64,
    rust_inside_history_term_w: f64,
    inside_history_delta_w: f64,
    oracle_outside_current_term_w: f64,
    rust_outside_current_term_w: f64,
    outside_current_delta_w: f64,
    oracle_outside_history_term_w: f64,
    rust_outside_history_term_w: f64,
    outside_history_delta_w: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceCtfHistorySeriesDelta {
    key: String,
    construction_name: String,
    area_m2: f64,
    samples: usize,
    inside_current_delta: DeltaSummary,
    inside_history_delta: DeltaSummary,
    outside_current_delta: DeltaSummary,
    outside_history_delta: DeltaSummary,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceCtfStorageMaxSampleDelta {
    key: String,
    construction_name: String,
    sample_index: usize,
    area_m2: f64,
    oracle_inside_conduction_w: f64,
    rust_inside_conduction_w: f64,
    inside_conduction_delta_w: f64,
    oracle_outside_conduction_w: f64,
    rust_outside_conduction_w: f64,
    outside_conduction_delta_w: f64,
    oracle_storage_w: f64,
    rust_storage_w: f64,
    storage_delta_w: f64,
    oracle_inside_current_term_w: f64,
    rust_inside_current_term_w: f64,
    inside_current_delta_w: f64,
    oracle_inside_history_term_w: f64,
    rust_inside_history_term_w: f64,
    inside_history_delta_w: f64,
    oracle_outside_current_term_w: f64,
    rust_outside_current_term_w: f64,
    outside_current_delta_w: f64,
    oracle_outside_history_term_w: f64,
    rust_outside_history_term_w: f64,
    outside_history_delta_w: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceInsideBalanceMaxSampleDelta {
    key: String,
    construction_name: String,
    sample_index: usize,
    area_m2: f64,
    oracle_inside_face_temperature_c: f64,
    rust_inside_face_temperature_c: f64,
    inside_face_temperature_delta_c: f64,
    oracle_inside_convection_coefficient_w_per_m2_k: f64,
    rust_inside_convection_coefficient_w_per_m2_k: f64,
    inside_convection_coefficient_delta_w_per_m2_k: f64,
    oracle_inside_conduction_w: f64,
    rust_inside_conduction_w: f64,
    inside_conduction_delta_w: f64,
    oracle_inside_convection_w: f64,
    rust_inside_convection_w: f64,
    inside_convection_delta_w: f64,
    oracle_inside_net_longwave_w: f64,
    rust_inside_net_longwave_w: f64,
    inside_net_longwave_delta_w: f64,
    oracle_inside_balance_residual_w: f64,
    rust_inside_balance_residual_w: f64,
    inside_balance_residual_delta_w: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceInsideSolveMaxSampleDelta {
    key: String,
    construction_name: String,
    outside_boundary_condition: String,
    sample_index: usize,
    area_m2: f64,
    ctf_inside_0_w_per_m2_k: f64,
    ctf_cross_0_w_per_m2_k: f64,
    iter_damp_w_per_m2_k: f64,
    oracle_inside_face_temperature_c: f64,
    rust_inside_face_temperature_c: f64,
    inside_face_temperature_delta_c: f64,
    oracle_inferred_reference_air_temperature_c: f64,
    rust_inferred_reference_air_temperature_c: f64,
    inferred_reference_air_temperature_delta_c: f64,
    oracle_solve_denominator_w_per_m2_k: f64,
    rust_solve_denominator_w_per_m2_k: f64,
    solve_denominator_delta_w_per_m2_k: f64,
    oracle_implied_solve_numerator_w: f64,
    rust_implied_solve_numerator_w: f64,
    implied_solve_numerator_delta_w: f64,
    oracle_reference_air_source_w: f64,
    rust_reference_air_source_w: f64,
    reference_air_source_delta_w: f64,
    oracle_outside_temperature_source_w: f64,
    rust_outside_temperature_source_w: f64,
    outside_temperature_source_delta_w: f64,
    oracle_inside_history_term_w: f64,
    rust_inside_history_term_w: f64,
    inside_history_delta_w: f64,
    rust_inside_history_temperature_term_w: f64,
    rust_inside_history_flux_term_w: f64,
    oracle_inside_net_longwave_w: f64,
    rust_inside_net_longwave_w: f64,
    inside_net_longwave_delta_w: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceAdiabaticHistoryMaxSampleDelta {
    key: String,
    construction_name: String,
    sample_index: usize,
    area_m2: f64,
    ctf_inside_0_w_per_m2_k: f64,
    ctf_cross_0_w_per_m2_k: f64,
    oracle_inside_face_temperature_c: f64,
    rust_inside_face_temperature_c: f64,
    inside_face_temperature_delta_c: f64,
    oracle_outside_face_temperature_c: f64,
    rust_outside_face_temperature_c: f64,
    outside_face_temperature_delta_c: f64,
    oracle_outside_minus_inside_c: f64,
    rust_outside_minus_inside_c: f64,
    outside_minus_inside_delta_c: f64,
    oracle_inside_current_term_w: f64,
    rust_inside_current_term_w: f64,
    inside_current_delta_w: f64,
    oracle_inside_current_if_outside_synced_w: f64,
    rust_inside_current_if_outside_synced_w: f64,
    inside_current_if_outside_synced_delta_w: f64,
    oracle_inside_current_sync_shift_w: f64,
    rust_inside_current_sync_shift_w: f64,
    oracle_inside_history_term_w: f64,
    rust_inside_history_term_w: f64,
    inside_history_delta_w: f64,
    oracle_inside_history_if_outside_synced_w: f64,
    rust_inside_history_if_outside_synced_w: f64,
    inside_history_if_outside_synced_delta_w: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceCtfSeedDiagnostic {
    policy: &'static str,
    included_constructions: Vec<String>,
    skipped_constructions: Vec<HeatBalanceSkippedCtfConstruction>,
    construction_summaries: Vec<HeatBalanceCtfConstructionSummary>,
    included_coefficients: usize,
    skipped_coefficients: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HeatBalanceCtfSeedPolicy {
    SteadyNoMassOnly,
    AllEio,
}

impl HeatBalanceCtfSeedPolicy {
    fn label(self) -> &'static str {
        match self {
            Self::SteadyNoMassOnly => "steady-no-mass-only",
            Self::AllEio => "all-eio",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceSkippedCtfConstruction {
    construction_name: String,
    ctf_count: usize,
    timestep_hours: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceCtfConstructionSummary {
    construction_name: String,
    ctf_count: usize,
    timestep_hours: f64,
    included: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceWarmupDiagnostic {
    enabled: bool,
    day_count: u32,
    timestep_count: usize,
    hours_per_day: usize,
    converged: bool,
    final_max_zone_temperature_delta_c: f64,
    oracle_run_period_day_count: Option<u32>,
}

impl From<HeatBalanceWarmupSummary> for HeatBalanceWarmupDiagnostic {
    fn from(summary: HeatBalanceWarmupSummary) -> Self {
        Self {
            enabled: summary.enabled,
            day_count: summary.day_count,
            timestep_count: summary.timestep_count,
            hours_per_day: summary.hours_per_day,
            converged: summary.converged,
            final_max_zone_temperature_delta_c: summary.final_max_zone_temperature_delta_c,
            oracle_run_period_day_count: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceConformance<'a> {
    context: &'a HeatBalanceConformanceContext,
    status: &'static str,
    failure_reasons: Vec<String>,
}

fn heat_balance_conformance_context_from_manifest(
    manifest: &ConformanceCase,
) -> Result<HeatBalanceConformanceContext, String> {
    validate_heat_balance_conformance_manifest(manifest)?;
    heat_balance_context_from_manifest(manifest)
}

fn heat_balance_diagnostic_context_from_manifest(
    manifest: &ConformanceCase,
) -> Result<HeatBalanceConformanceContext, String> {
    validate_heat_balance_diagnostic_manifest(manifest)?;
    heat_balance_context_from_manifest(manifest)
}

fn heat_balance_context_from_manifest(
    manifest: &ConformanceCase,
) -> Result<HeatBalanceConformanceContext, String> {
    let outputs = manifest
        .outputs
        .iter()
        .map(|output| ZoneTemperatureReportOutput {
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output_frequency_label(output.frequency),
            class: variable_class_label(output.class),
            source: source_artifact_label(output.source),
        })
        .collect::<Vec<_>>();
    let tolerances = heat_balance_variable_classes(manifest)
        .into_iter()
        .map(|variable_class| heat_balance_tolerance_from_manifest(manifest, variable_class))
        .collect::<Result<Vec<_>, String>>()?;

    Ok(HeatBalanceConformanceContext {
        case_id: manifest.id.clone(),
        oracle_version: manifest.oracle_version.clone(),
        outputs,
        tolerances,
        report: manifest
            .report
            .as_ref()
            .map(|report| ZoneTemperatureReportContract {
                format: report_format_label(report.format),
                path: report.path.clone(),
            }),
        gate: manifest
            .gate
            .as_ref()
            .map(|gate| ZoneTemperatureGateContract {
                script: gate.script.clone(),
                blocking: gate.blocking,
            }),
        comparison_class: comparison_class_label(manifest.comparison_class),
        conformance_claim: manifest.conformance_claim,
    })
}

fn heat_balance_tolerance_from_manifest(
    manifest: &ConformanceCase,
    variable_class: VariableClass,
) -> Result<HeatBalanceToleranceReport, String> {
    let tolerance = manifest
        .tolerances
        .iter()
        .find(|tolerance| tolerance.variable_class == variable_class)
        .ok_or_else(|| {
            format!(
                "heat-balance conformance requires a {} tolerance",
                variable_class_label(variable_class)
            )
        })?;

    Ok(heat_balance_tolerance_report(*tolerance))
}

fn heat_balance_tolerance_report(tolerance: ToleranceRule) -> HeatBalanceToleranceReport {
    HeatBalanceToleranceReport {
        variable_class_label: variable_class_label(tolerance.variable_class),
        max_abs_c: tolerance.max_abs,
        max_rmse_c: tolerance.max_rmse,
        max_rel: tolerance.max_rel,
    }
}

fn heat_balance_variable_classes(manifest: &ConformanceCase) -> Vec<VariableClass> {
    let mut classes = Vec::new();
    for output in &manifest.outputs {
        if !classes.contains(&output.class) {
            classes.push(output.class);
        }
    }
    classes
}

fn is_supported_heat_balance_output_variable(variable: &str) -> bool {
    variable.eq_ignore_ascii_case("Zone Mean Air Temperature")
        || variable.eq_ignore_ascii_case("Surface Inside Face Temperature")
        || variable.eq_ignore_ascii_case("Surface Outside Face Temperature")
        || variable.eq_ignore_ascii_case("Surface Inside Face Convection Heat Transfer Coefficient")
        || variable.eq_ignore_ascii_case("Surface Inside Face Convection Heat Gain Rate")
        || variable.eq_ignore_ascii_case("Surface Inside Face Convection Heat Gain Rate per Area")
        || variable.eq_ignore_ascii_case(
            "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate",
        )
        || variable.eq_ignore_ascii_case(
            "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate per Area",
        )
        || variable.eq_ignore_ascii_case("Surface Inside Face Conduction Heat Transfer Rate")
        || variable.eq_ignore_ascii_case("Surface Inside Face Conduction Heat Gain Rate")
        || variable.eq_ignore_ascii_case("Surface Inside Face Conduction Heat Loss Rate")
        || variable
            .eq_ignore_ascii_case("Surface Inside Face Conduction Heat Transfer Rate per Area")
        || variable.eq_ignore_ascii_case("Surface Outside Face Conduction Heat Transfer Rate")
        || variable.eq_ignore_ascii_case("Surface Outside Face Conduction Heat Gain Rate")
        || variable.eq_ignore_ascii_case("Surface Outside Face Conduction Heat Loss Rate")
        || variable
            .eq_ignore_ascii_case("Surface Outside Face Conduction Heat Transfer Rate per Area")
        || variable.eq_ignore_ascii_case("Surface Heat Storage Rate")
        || variable.eq_ignore_ascii_case("Surface Heat Storage Rate per Area")
        || variable
            .eq_ignore_ascii_case("Surface Outside Face Incident Solar Radiation Rate per Area")
        || variable.eq_ignore_ascii_case(
            "Surface Outside Face Incident Beam Solar Radiation Rate per Area",
        )
        || variable.eq_ignore_ascii_case(
            "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area",
        )
        || variable.eq_ignore_ascii_case(
            "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area",
        )
        || variable.eq_ignore_ascii_case("Surface Outside Face Convection Heat Gain Rate")
        || variable.eq_ignore_ascii_case("Surface Outside Face Convection Heat Gain Rate per Area")
        || variable
            .eq_ignore_ascii_case("Surface Outside Face Convection Heat Transfer Coefficient")
        || variable
            .eq_ignore_ascii_case("Surface Outside Face Net Thermal Radiation Heat Gain Rate")
        || variable.eq_ignore_ascii_case(
            "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area",
        )
        || variable.eq_ignore_ascii_case("Surface Outside Face Solar Radiation Heat Gain Rate")
        || variable
            .eq_ignore_ascii_case("Surface Outside Face Solar Radiation Heat Gain Rate per Area")
        || variable.eq_ignore_ascii_case("Zone Opaque Surface Inside Faces Conduction Rate")
        || variable
            .eq_ignore_ascii_case("Zone Opaque Surface Inside Faces Conduction Heat Gain Rate")
        || variable
            .eq_ignore_ascii_case("Zone Opaque Surface Inside Faces Conduction Heat Loss Rate")
        || variable.eq_ignore_ascii_case("Zone Opaque Surface Outside Faces Conduction Rate")
        || variable
            .eq_ignore_ascii_case("Zone Opaque Surface Outside Faces Conduction Heat Gain Rate")
        || variable
            .eq_ignore_ascii_case("Zone Opaque Surface Outside Faces Conduction Heat Loss Rate")
        || variable.eq_ignore_ascii_case("Zone Air Heat Balance Internal Convective Heat Gain Rate")
        || variable.eq_ignore_ascii_case("Zone Air Heat Balance Surface Convection Rate")
        || variable.eq_ignore_ascii_case("Zone Air Heat Balance Air Energy Storage Rate")
}

fn evaluate_heat_balance_conformance<'a>(
    diagnostic: &HeatBalanceConformanceDiagnostic,
    context: &'a HeatBalanceConformanceContext,
) -> HeatBalanceConformance<'a> {
    let mut failure_reasons = Vec::new();
    if diagnostic.status != "extracted" {
        failure_reasons.push(format!(
            "diagnostic extraction status was {}",
            diagnostic.status
        ));
    }
    for series in &diagnostic.series {
        let label = format!("{}/{}", series.output.key, series.output.variable);
        if series.status != "extracted" {
            failure_reasons.push(format!("{label} extraction status was {}", series.status));
        }
        if !series.delta.length_match {
            failure_reasons.push(format!("{label} series length mismatch"));
        }
        let Some(tolerance) = context.tolerance_for_class(series.output.class) else {
            failure_reasons.push(format!("{label} missing {} tolerance", series.output.class));
            continue;
        };
        if let Some(max_abs_c) = tolerance.max_abs_c
            && series.delta.max_abs_delta_c > max_abs_c
        {
            failure_reasons.push(format!(
                "{label} max_abs_delta_c {:.12} exceeds {:.12}",
                series.delta.max_abs_delta_c, max_abs_c
            ));
        }
        if let Some(max_rmse_c) = tolerance.max_rmse_c
            && series.delta.rmse_delta_c > max_rmse_c
        {
            failure_reasons.push(format!(
                "{label} rmse_delta_c {:.12} exceeds {:.12}",
                series.delta.rmse_delta_c, max_rmse_c
            ));
        }
        if let Some(max_rel) = tolerance.max_rel
            && series.delta.max_rel_delta > max_rel
        {
            failure_reasons.push(format!(
                "{label} max_rel_delta {:.12} exceeds {:.12}",
                series.delta.max_rel_delta, max_rel
            ));
        }
    }

    HeatBalanceConformance {
        context,
        status: if failure_reasons.is_empty() {
            "pass"
        } else {
            "fail"
        },
        failure_reasons,
    }
}

fn build_heat_balance_conformance_diagnostic(
    input_path: &Path,
    weather_path: &Path,
    eio_path: &Path,
    eso_path: &Path,
    context: &HeatBalanceConformanceContext,
) -> Result<HeatBalanceConformanceDiagnostic, String> {
    let raw_model = load_epjson_file(input_path).map_err(|error| error.to_string())?;
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        return Err(format_compile_diagnostics(&result.report));
    };

    let mut oracle_series = Vec::with_capacity(context.outputs.len());
    for output in &context.outputs {
        let time_series = load_eso_time_series(eso_path, &output.key, &output.variable)
            .map_err(|error| error.to_string())?;
        let values = run_period_eso_values(&time_series);
        if values.is_empty() {
            return Err(format!(
                "EnergyPlus series is empty: {}/{}",
                output.key, output.variable
            ));
        }
        oracle_series.push((output.clone(), values));
    }
    let sample_count = oracle_series
        .iter()
        .map(|(_output, values)| values.len())
        .max()
        .unwrap_or(0);

    let weather_records = load_epw_records(weather_path).map_err(|error| error.to_string())?;
    let weather_values = weather_records
        .iter()
        .map(|record| record.dry_bulb_c)
        .collect::<Vec<_>>();
    if weather_values.len() < sample_count {
        return Err(format!(
            "EPW dry-bulb series has {} samples but ESO requires {}",
            weather_values.len(),
            sample_count
        ));
    }

    let simulation_model = SimulationModel::from_typed(model);
    let zone_air_algorithm = if context.conformance_claim {
        HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
    } else {
        heat_balance_zone_air_algorithm_from_env()?
    };
    let simulation_options = if context.conformance_claim {
        HeatBalanceSimulationOptions::hourly_samples(sample_count)
    } else {
        apply_heat_balance_ctf_initial_history_policy_from_env(
            apply_heat_balance_surface_iterations_from_env(
                apply_heat_balance_warmup_minimum_days_from_env(
                    HeatBalanceSimulationOptions::hourly_samples_with_model_warmup(
                        &simulation_model,
                        sample_count,
                    ),
                )?,
            )?,
        )?
    }
    .with_zone_air_algorithm(zone_air_algorithm);
    let (ctf_coefficients, ctf_seed) = if context.conformance_claim {
        (Vec::new(), disabled_heat_balance_ctf_seed_diagnostic())
    } else {
        load_runtime_ctf_coefficients_from_eio(eio_path)?
    };
    let mut simulation =
        simulate_heat_balance_zone_air_temperatures_with_weather_records_and_ctf_coefficients(
            &simulation_model,
            &weather_records,
            simulation_options,
            &ctf_coefficients,
        )
        .map_err(|error| error.to_string())?;
    append_surface_incident_solar_radiation_series(
        &mut simulation.results,
        &simulation_model,
        &weather_records,
        sample_count,
    );
    let mut heat_balance_warmup: HeatBalanceWarmupDiagnostic = simulation.summary.warmup.into();
    heat_balance_warmup.oracle_run_period_day_count =
        eio_run_period_warmup_days(eio_path).map_err(|error| error.to_string())?;

    let mut series = Vec::with_capacity(oracle_series.len());
    for (output, oracle_values) in oracle_series {
        let Some(rust_series) = simulation
            .results
            .find_series(&output.key, &output.variable)
        else {
            return Err(format!(
                "heat-balance simulation did not write output: {}/{}",
                output.key, output.variable
            ));
        };

        let delta = delta_summary(&oracle_values, &rust_series.values);
        let sample_rows = delta_points(&oracle_values, &rust_series.values);
        let finite = oracle_values
            .iter()
            .chain(rust_series.values.iter())
            .all(|value| value.is_finite());
        let extracted = finite && delta.length_match;
        series.push(HeatBalanceSeriesDiagnostic {
            output,
            samples: delta.samples,
            oracle_first_c: oracle_values[0],
            rust_first_c: rust_series.values.first().copied().unwrap_or(f64::NAN),
            oracle_last_c: oracle_values[oracle_values.len() - 1],
            rust_last_c: rust_series.values.last().copied().unwrap_or(f64::NAN),
            delta,
            sample_rows,
            status: if extracted { "extracted" } else { "failed" },
        });
    }

    let extracted = series.iter().all(|series| series.status == "extracted");
    let ctf_component_first_samples = heat_balance_ctf_component_first_samples(&simulation.results);
    let ctf_history_first_sample_deltas = heat_balance_ctf_history_first_sample_deltas(
        &simulation_model,
        &series,
        &ctf_coefficients,
        &ctf_component_first_samples,
    );
    let ctf_history_series_deltas = heat_balance_ctf_history_series_deltas(
        &simulation_model,
        &series,
        &ctf_coefficients,
        &simulation.results,
    );
    let ctf_storage_max_sample_deltas = heat_balance_ctf_storage_max_sample_deltas(
        &simulation_model,
        &series,
        &ctf_coefficients,
        &simulation.results,
    );
    let inside_balance_max_sample_deltas =
        heat_balance_inside_balance_max_sample_deltas(&simulation_model, &series);
    let inside_solve_max_sample_deltas = heat_balance_inside_solve_max_sample_deltas(
        &simulation_model,
        &series,
        &ctf_coefficients,
        &simulation.results,
    );
    let adiabatic_history_max_sample_deltas = heat_balance_adiabatic_history_max_sample_deltas(
        &simulation_model,
        &series,
        &ctf_coefficients,
        &simulation.results,
    );
    Ok(HeatBalanceConformanceDiagnostic {
        samples: sample_count,
        heat_balance_timesteps: simulation.summary.timestep_count,
        heat_balance_run_period_timesteps: simulation.summary.run_period_timestep_count,
        heat_balance_warmup,
        ctf_seed,
        zone_air_algorithm: heat_balance_zone_air_algorithm_label(zone_air_algorithm),
        surface_iteration_count: simulation.summary.surface_iteration_count,
        ctf_initial_history_policy: heat_balance_ctf_initial_history_policy_label(
            simulation.summary.ctf_initial_history_policy,
        ),
        zone_count: simulation.summary.zone_count,
        surface_count: simulation.summary.surface_count,
        ctf_component_first_samples,
        ctf_history_first_sample_deltas,
        ctf_history_series_deltas,
        ctf_storage_max_sample_deltas,
        inside_balance_max_sample_deltas,
        inside_solve_max_sample_deltas,
        adiabatic_history_max_sample_deltas,
        ctf_history_run_period_initial_slots: simulation
            .summary
            .run_period_initial_ctf_history_slots,
        ctf_history_first_sample_slots: simulation.summary.first_sample_ctf_history_slots,
        surface_first_sample_trace: simulation.summary.surface_first_sample_trace,
        series,
        status: if extracted { "extracted" } else { "failed" },
    })
}

fn heat_balance_ctf_component_first_samples(
    results: &ResultStore,
) -> Vec<HeatBalanceCtfComponentFirstSample> {
    let keys = results
        .series
        .iter()
        .filter(|series| {
            series
                .variable_name
                .eq_ignore_ascii_case(SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE)
        })
        .map(|series| series.key.clone())
        .collect::<BTreeSet<_>>();

    keys.into_iter()
        .filter_map(|key| heat_balance_ctf_component_first_sample_for_key(results, &key))
        .collect()
}

fn heat_balance_ctf_component_first_sample_for_key(
    results: &ResultStore,
    key: &str,
) -> Option<HeatBalanceCtfComponentFirstSample> {
    Some(HeatBalanceCtfComponentFirstSample {
        key: key.to_string(),
        inside_conduction_rate_w: heat_balance_first_series_value(
            results,
            key,
            "Surface Inside Face Conduction Heat Transfer Rate",
        )?,
        inside_current_outside_term_w: heat_balance_first_series_value(
            results,
            key,
            SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
        )?,
        inside_current_inside_term_w: heat_balance_first_series_value(
            results,
            key,
            SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
        )?,
        inside_history_term_w: heat_balance_first_series_value(
            results,
            key,
            SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE,
        )?,
        outside_conduction_rate_w: heat_balance_first_series_value(
            results,
            key,
            "Surface Outside Face Conduction Heat Transfer Rate",
        )?,
        outside_current_outside_term_w: heat_balance_first_series_value(
            results,
            key,
            SURFACE_CTF_OUTSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
        )?,
        outside_current_inside_term_w: heat_balance_first_series_value(
            results,
            key,
            SURFACE_CTF_OUTSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
        )?,
        outside_history_term_w: heat_balance_first_series_value(
            results,
            key,
            SURFACE_CTF_OUTSIDE_HISTORY_TERM_RATE_VARIABLE,
        )?,
        heat_storage_rate_w: heat_balance_first_series_value(
            results,
            key,
            "Surface Heat Storage Rate",
        )?,
    })
}

fn heat_balance_first_series_value(
    results: &ResultStore,
    key: &str,
    variable_name: &str,
) -> Option<f64> {
    results
        .find_series(key, variable_name)
        .and_then(|series| series.values.first().copied())
        .filter(|value| value.is_finite())
}

fn heat_balance_ctf_history_first_sample_deltas(
    model: &SimulationModel,
    series: &[HeatBalanceSeriesDiagnostic],
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
    component_rows: &[HeatBalanceCtfComponentFirstSample],
) -> Vec<HeatBalanceCtfHistoryFirstSampleDelta> {
    model
        .typed
        .surfaces
        .iter()
        .filter_map(|surface| {
            let construction = model
                .typed
                .constructions
                .iter()
                .find(|construction| construction.id == surface.construction)?;
            let zero = ctf_coefficients.iter().find(|coefficient| {
                coefficient.time_index == 0
                    && coefficient
                        .construction_name
                        .eq_ignore_ascii_case(&construction.name.0)
            })?;
            let component = component_rows
                .iter()
                .find(|component| component.key.eq_ignore_ascii_case(&surface.name.0))?;
            let area_m2 = surface_area_m2(&surface.vertices);
            if area_m2 <= 0.0 {
                return None;
            }
            let oracle_inside_temperature_c = heat_balance_oracle_first_value(
                series,
                &surface.name.0,
                "Surface Inside Face Temperature",
            )?;
            let oracle_outside_temperature_c = heat_balance_oracle_first_value(
                series,
                &surface.name.0,
                "Surface Outside Face Temperature",
            )?;
            let rust_inside_temperature_c = heat_balance_rust_first_value(
                series,
                &surface.name.0,
                "Surface Inside Face Temperature",
            )?;
            let rust_outside_temperature_c = heat_balance_rust_first_value(
                series,
                &surface.name.0,
                "Surface Outside Face Temperature",
            )?;
            let oracle_inside_conduction_rate_w = heat_balance_oracle_first_value(
                series,
                &surface.name.0,
                "Surface Inside Face Conduction Heat Transfer Rate",
            )?;
            let oracle_outside_conduction_rate_w = heat_balance_oracle_first_value(
                series,
                &surface.name.0,
                "Surface Outside Face Conduction Heat Transfer Rate",
            )?;

            let oracle_inside_current_term_w = area_m2
                * (oracle_outside_temperature_c * zero.cross_w_per_m2_k
                    - oracle_inside_temperature_c * zero.inside_w_per_m2_k);
            let oracle_outside_current_term_w = -area_m2
                * (oracle_outside_temperature_c * zero.outside_w_per_m2_k
                    - oracle_inside_temperature_c * zero.cross_w_per_m2_k);
            let rust_inside_current_term_w =
                component.inside_current_outside_term_w + component.inside_current_inside_term_w;
            let rust_outside_current_term_w =
                component.outside_current_outside_term_w + component.outside_current_inside_term_w;
            let oracle_inside_history_term_w =
                oracle_inside_conduction_rate_w - oracle_inside_current_term_w;
            let oracle_outside_history_term_w =
                oracle_outside_conduction_rate_w - oracle_outside_current_term_w;

            Some(HeatBalanceCtfHistoryFirstSampleDelta {
                key: surface.name.0.clone(),
                construction_name: construction.name.0.clone(),
                area_m2,
                ctf_outside_0_w_per_m2_k: zero.outside_w_per_m2_k,
                ctf_cross_0_w_per_m2_k: zero.cross_w_per_m2_k,
                ctf_inside_0_w_per_m2_k: zero.inside_w_per_m2_k,
                oracle_outside_face_temperature_c: oracle_outside_temperature_c,
                rust_outside_face_temperature_c: rust_outside_temperature_c,
                outside_face_temperature_delta_c: (oracle_outside_temperature_c
                    - rust_outside_temperature_c)
                    .abs(),
                oracle_inside_face_temperature_c: oracle_inside_temperature_c,
                rust_inside_face_temperature_c: rust_inside_temperature_c,
                inside_face_temperature_delta_c: (oracle_inside_temperature_c
                    - rust_inside_temperature_c)
                    .abs(),
                oracle_inside_current_term_w,
                rust_inside_current_term_w,
                inside_current_delta_w: (oracle_inside_current_term_w - rust_inside_current_term_w)
                    .abs(),
                oracle_inside_history_term_w,
                rust_inside_history_term_w: component.inside_history_term_w,
                inside_history_delta_w: (oracle_inside_history_term_w
                    - component.inside_history_term_w)
                    .abs(),
                oracle_outside_current_term_w,
                rust_outside_current_term_w,
                outside_current_delta_w: (oracle_outside_current_term_w
                    - rust_outside_current_term_w)
                    .abs(),
                oracle_outside_history_term_w,
                rust_outside_history_term_w: component.outside_history_term_w,
                outside_history_delta_w: (oracle_outside_history_term_w
                    - component.outside_history_term_w)
                    .abs(),
            })
        })
        .collect()
}

fn heat_balance_ctf_history_series_deltas(
    model: &SimulationModel,
    series: &[HeatBalanceSeriesDiagnostic],
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
    results: &ResultStore,
) -> Vec<HeatBalanceCtfHistorySeriesDelta> {
    model
        .typed
        .surfaces
        .iter()
        .filter_map(|surface| {
            let construction = model
                .typed
                .constructions
                .iter()
                .find(|construction| construction.id == surface.construction)?;
            let zero = ctf_coefficients.iter().find(|coefficient| {
                coefficient.time_index == 0
                    && coefficient
                        .construction_name
                        .eq_ignore_ascii_case(&construction.name.0)
            })?;
            let area_m2 = surface_area_m2(&surface.vertices);
            if area_m2 <= 0.0 {
                return None;
            }

            let oracle_inside_temperatures = heat_balance_oracle_series_values(
                series,
                &surface.name.0,
                "Surface Inside Face Temperature",
            )?;
            let oracle_outside_temperatures = heat_balance_oracle_series_values(
                series,
                &surface.name.0,
                "Surface Outside Face Temperature",
            )?;
            let oracle_inside_conduction = heat_balance_oracle_series_values(
                series,
                &surface.name.0,
                "Surface Inside Face Conduction Heat Transfer Rate",
            )?;
            let oracle_outside_conduction = heat_balance_oracle_series_values(
                series,
                &surface.name.0,
                "Surface Outside Face Conduction Heat Transfer Rate",
            )?;
            let rust_inside_current_outside = heat_balance_result_series_values(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
            )?;
            let rust_inside_current_inside = heat_balance_result_series_values(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
            )?;
            let rust_inside_history = heat_balance_result_series_values(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE,
            )?;
            let rust_outside_current_outside = heat_balance_result_series_values(
                results,
                &surface.name.0,
                SURFACE_CTF_OUTSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
            )?;
            let rust_outside_current_inside = heat_balance_result_series_values(
                results,
                &surface.name.0,
                SURFACE_CTF_OUTSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
            )?;
            let rust_outside_history = heat_balance_result_series_values(
                results,
                &surface.name.0,
                SURFACE_CTF_OUTSIDE_HISTORY_TERM_RATE_VARIABLE,
            )?;

            let samples = [
                oracle_inside_temperatures.len(),
                oracle_outside_temperatures.len(),
                oracle_inside_conduction.len(),
                oracle_outside_conduction.len(),
                rust_inside_current_outside.len(),
                rust_inside_current_inside.len(),
                rust_inside_history.len(),
                rust_outside_current_outside.len(),
                rust_outside_current_inside.len(),
                rust_outside_history.len(),
            ]
            .into_iter()
            .min()
            .unwrap_or(0);
            if samples == 0 {
                return None;
            }

            let mut oracle_inside_current = Vec::with_capacity(samples);
            let mut rust_inside_current = Vec::with_capacity(samples);
            let mut oracle_inside_history = Vec::with_capacity(samples);
            let mut oracle_outside_current = Vec::with_capacity(samples);
            let mut rust_outside_current = Vec::with_capacity(samples);
            let mut oracle_outside_history = Vec::with_capacity(samples);
            for index in 0..samples {
                let oracle_inside_current_term_w = area_m2
                    * (oracle_outside_temperatures[index] * zero.cross_w_per_m2_k
                        - oracle_inside_temperatures[index] * zero.inside_w_per_m2_k);
                let oracle_outside_current_term_w = -area_m2
                    * (oracle_outside_temperatures[index] * zero.outside_w_per_m2_k
                        - oracle_inside_temperatures[index] * zero.cross_w_per_m2_k);
                let rust_inside_current_term_w =
                    rust_inside_current_outside[index] + rust_inside_current_inside[index];
                let rust_outside_current_term_w =
                    rust_outside_current_outside[index] + rust_outside_current_inside[index];

                oracle_inside_current.push(oracle_inside_current_term_w);
                rust_inside_current.push(rust_inside_current_term_w);
                oracle_inside_history
                    .push(oracle_inside_conduction[index] - oracle_inside_current_term_w);
                oracle_outside_current.push(oracle_outside_current_term_w);
                rust_outside_current.push(rust_outside_current_term_w);
                oracle_outside_history
                    .push(oracle_outside_conduction[index] - oracle_outside_current_term_w);
            }

            Some(HeatBalanceCtfHistorySeriesDelta {
                key: surface.name.0.clone(),
                construction_name: construction.name.0.clone(),
                area_m2,
                samples,
                inside_current_delta: delta_summary(&oracle_inside_current, &rust_inside_current),
                inside_history_delta: delta_summary(&oracle_inside_history, &rust_inside_history),
                outside_current_delta: delta_summary(
                    &oracle_outside_current,
                    &rust_outside_current,
                ),
                outside_history_delta: delta_summary(
                    &oracle_outside_history,
                    &rust_outside_history,
                ),
            })
        })
        .collect()
}

fn heat_balance_ctf_storage_max_sample_deltas(
    model: &SimulationModel,
    series: &[HeatBalanceSeriesDiagnostic],
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
    results: &ResultStore,
) -> Vec<HeatBalanceCtfStorageMaxSampleDelta> {
    model
        .typed
        .surfaces
        .iter()
        .filter_map(|surface| {
            let construction = model
                .typed
                .constructions
                .iter()
                .find(|construction| construction.id == surface.construction)?;
            let zero = ctf_coefficients.iter().find(|coefficient| {
                coefficient.time_index == 0
                    && coefficient
                        .construction_name
                        .eq_ignore_ascii_case(&construction.name.0)
            })?;
            let storage_series =
                heat_balance_series(series, &surface.name.0, "Surface Heat Storage Rate")?;
            let storage_point = storage_series.delta.max_delta_sample?;
            let sample_index = storage_point.index;
            let area_m2 = surface_area_m2(&surface.vertices);
            if area_m2 <= 0.0 {
                return None;
            }

            let inside_temperature = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Temperature",
                sample_index,
            )?;
            let outside_temperature = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Outside Face Temperature",
                sample_index,
            )?;
            let inside_conduction = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Conduction Heat Transfer Rate",
                sample_index,
            )?;
            let outside_conduction = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Outside Face Conduction Heat Transfer Rate",
                sample_index,
            )?;
            let rust_inside_current_outside = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
                sample_index,
            )?;
            let rust_inside_current_inside = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
                sample_index,
            )?;
            let rust_inside_history = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE,
                sample_index,
            )?;
            let rust_outside_current_outside = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_OUTSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
                sample_index,
            )?;
            let rust_outside_current_inside = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_OUTSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
                sample_index,
            )?;
            let rust_outside_history = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_OUTSIDE_HISTORY_TERM_RATE_VARIABLE,
                sample_index,
            )?;

            let oracle_inside_current_term_w = area_m2
                * (outside_temperature.oracle_c * zero.cross_w_per_m2_k
                    - inside_temperature.oracle_c * zero.inside_w_per_m2_k);
            let rust_inside_current_term_w =
                rust_inside_current_outside + rust_inside_current_inside;
            let oracle_inside_history_term_w =
                inside_conduction.oracle_c - oracle_inside_current_term_w;
            let oracle_outside_current_term_w = -area_m2
                * (outside_temperature.oracle_c * zero.outside_w_per_m2_k
                    - inside_temperature.oracle_c * zero.cross_w_per_m2_k);
            let rust_outside_current_term_w =
                rust_outside_current_outside + rust_outside_current_inside;
            let oracle_outside_history_term_w =
                outside_conduction.oracle_c - oracle_outside_current_term_w;

            Some(HeatBalanceCtfStorageMaxSampleDelta {
                key: surface.name.0.clone(),
                construction_name: construction.name.0.clone(),
                sample_index,
                area_m2,
                oracle_inside_conduction_w: inside_conduction.oracle_c,
                rust_inside_conduction_w: inside_conduction.rust_c,
                inside_conduction_delta_w: inside_conduction.abs_delta_c,
                oracle_outside_conduction_w: outside_conduction.oracle_c,
                rust_outside_conduction_w: outside_conduction.rust_c,
                outside_conduction_delta_w: outside_conduction.abs_delta_c,
                oracle_storage_w: storage_point.oracle_c,
                rust_storage_w: storage_point.rust_c,
                storage_delta_w: storage_point.abs_delta_c,
                oracle_inside_current_term_w,
                rust_inside_current_term_w,
                inside_current_delta_w: (oracle_inside_current_term_w - rust_inside_current_term_w)
                    .abs(),
                oracle_inside_history_term_w,
                rust_inside_history_term_w: rust_inside_history,
                inside_history_delta_w: (oracle_inside_history_term_w - rust_inside_history).abs(),
                oracle_outside_current_term_w,
                rust_outside_current_term_w,
                outside_current_delta_w: (oracle_outside_current_term_w
                    - rust_outside_current_term_w)
                    .abs(),
                oracle_outside_history_term_w,
                rust_outside_history_term_w: rust_outside_history,
                outside_history_delta_w: (oracle_outside_history_term_w - rust_outside_history)
                    .abs(),
            })
        })
        .collect()
}

fn heat_balance_inside_balance_max_sample_deltas(
    model: &SimulationModel,
    series: &[HeatBalanceSeriesDiagnostic],
) -> Vec<HeatBalanceInsideBalanceMaxSampleDelta> {
    model
        .typed
        .surfaces
        .iter()
        .filter_map(|surface| {
            let construction = model
                .typed
                .constructions
                .iter()
                .find(|construction| construction.id == surface.construction)?;
            let storage_series =
                heat_balance_series(series, &surface.name.0, "Surface Heat Storage Rate")?;
            let storage_point = storage_series.delta.max_delta_sample?;
            let sample_index = storage_point.index;
            let area_m2 = surface_area_m2(&surface.vertices);
            if area_m2 <= 0.0 {
                return None;
            }

            let inside_temperature = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Temperature",
                sample_index,
            )?;
            let inside_convection_coefficient = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Convection Heat Transfer Coefficient",
                sample_index,
            )?;
            let inside_conduction = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Conduction Heat Transfer Rate",
                sample_index,
            )?;
            let inside_convection = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Convection Heat Gain Rate",
                sample_index,
            )?;
            let inside_net_longwave = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate",
                sample_index,
            )?;
            let oracle_inside_balance_residual_w = inside_conduction.oracle_c
                + inside_convection.oracle_c
                + inside_net_longwave.oracle_c;
            let rust_inside_balance_residual_w =
                inside_conduction.rust_c + inside_convection.rust_c + inside_net_longwave.rust_c;

            Some(HeatBalanceInsideBalanceMaxSampleDelta {
                key: surface.name.0.clone(),
                construction_name: construction.name.0.clone(),
                sample_index,
                area_m2,
                oracle_inside_face_temperature_c: inside_temperature.oracle_c,
                rust_inside_face_temperature_c: inside_temperature.rust_c,
                inside_face_temperature_delta_c: inside_temperature.abs_delta_c,
                oracle_inside_convection_coefficient_w_per_m2_k: inside_convection_coefficient
                    .oracle_c,
                rust_inside_convection_coefficient_w_per_m2_k: inside_convection_coefficient.rust_c,
                inside_convection_coefficient_delta_w_per_m2_k: inside_convection_coefficient
                    .abs_delta_c,
                oracle_inside_conduction_w: inside_conduction.oracle_c,
                rust_inside_conduction_w: inside_conduction.rust_c,
                inside_conduction_delta_w: inside_conduction.abs_delta_c,
                oracle_inside_convection_w: inside_convection.oracle_c,
                rust_inside_convection_w: inside_convection.rust_c,
                inside_convection_delta_w: inside_convection.abs_delta_c,
                oracle_inside_net_longwave_w: inside_net_longwave.oracle_c,
                rust_inside_net_longwave_w: inside_net_longwave.rust_c,
                inside_net_longwave_delta_w: inside_net_longwave.abs_delta_c,
                oracle_inside_balance_residual_w,
                rust_inside_balance_residual_w,
                inside_balance_residual_delta_w: (oracle_inside_balance_residual_w
                    - rust_inside_balance_residual_w)
                    .abs(),
            })
        })
        .collect()
}

fn heat_balance_inside_solve_max_sample_deltas(
    model: &SimulationModel,
    series: &[HeatBalanceSeriesDiagnostic],
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
    results: &ResultStore,
) -> Vec<HeatBalanceInsideSolveMaxSampleDelta> {
    model
        .typed
        .surfaces
        .iter()
        .filter_map(|surface| {
            let construction = model
                .typed
                .constructions
                .iter()
                .find(|construction| construction.id == surface.construction)?;
            let zero = ctf_coefficients.iter().find(|coefficient| {
                coefficient.time_index == 0
                    && coefficient
                        .construction_name
                        .eq_ignore_ascii_case(&construction.name.0)
            })?;
            let storage_series =
                heat_balance_series(series, &surface.name.0, "Surface Heat Storage Rate")?;
            let storage_point = storage_series.delta.max_delta_sample?;
            let sample_index = storage_point.index;
            let area_m2 = surface_area_m2(&surface.vertices);
            if area_m2 <= 0.0 {
                return None;
            }

            let inside_temperature = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Temperature",
                sample_index,
            )?;
            let outside_temperature = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Outside Face Temperature",
                sample_index,
            )?;
            let inside_convection_coefficient = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Convection Heat Transfer Coefficient",
                sample_index,
            )?;
            let inside_convection = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Convection Heat Gain Rate",
                sample_index,
            )?;
            let inside_conduction = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Conduction Heat Transfer Rate",
                sample_index,
            )?;
            let inside_net_longwave = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate",
                sample_index,
            )?;
            let rust_inside_history = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE,
                sample_index,
            )?;
            let rust_inside_history_temperature = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_HISTORY_TEMPERATURE_TERM_RATE_VARIABLE,
                sample_index,
            )?;
            let rust_inside_history_flux = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_HISTORY_FLUX_TERM_RATE_VARIABLE,
                sample_index,
            )?;

            let adiabatic_cross =
                if surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic {
                    zero.cross_w_per_m2_k
                } else {
                    0.0
                };
            let oracle_solve_denominator_w_per_m2_k = zero.inside_w_per_m2_k - adiabatic_cross
                + inside_convection_coefficient.oracle_c
                + HEAT_BALANCE_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K;
            let rust_solve_denominator_w_per_m2_k = zero.inside_w_per_m2_k - adiabatic_cross
                + inside_convection_coefficient.rust_c
                + HEAT_BALANCE_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K;
            let oracle_inferred_reference_air_temperature_c =
                heat_balance_inferred_reference_air_temperature_c(
                    inside_temperature.oracle_c,
                    inside_convection_coefficient.oracle_c,
                    area_m2,
                    inside_convection.oracle_c,
                );
            let rust_inferred_reference_air_temperature_c =
                heat_balance_inferred_reference_air_temperature_c(
                    inside_temperature.rust_c,
                    inside_convection_coefficient.rust_c,
                    area_m2,
                    inside_convection.rust_c,
                );
            let oracle_reference_air_source_w = area_m2
                * inside_convection_coefficient.oracle_c
                * oracle_inferred_reference_air_temperature_c;
            let rust_reference_air_source_w = area_m2
                * inside_convection_coefficient.rust_c
                * rust_inferred_reference_air_temperature_c;
            let oracle_outside_temperature_source_w = area_m2
                * heat_balance_inside_solve_outside_temperature_source_w_per_m2(
                    surface.outside_boundary_condition,
                    zero.cross_w_per_m2_k,
                    outside_temperature.oracle_c,
                );
            let rust_outside_temperature_source_w = area_m2
                * heat_balance_inside_solve_outside_temperature_source_w_per_m2(
                    surface.outside_boundary_condition,
                    zero.cross_w_per_m2_k,
                    outside_temperature.rust_c,
                );
            let oracle_inside_current_term_w = area_m2
                * (outside_temperature.oracle_c * zero.cross_w_per_m2_k
                    - inside_temperature.oracle_c * zero.inside_w_per_m2_k);
            let oracle_inside_history_term_w =
                inside_conduction.oracle_c - oracle_inside_current_term_w;
            let oracle_implied_solve_numerator_w =
                area_m2 * oracle_solve_denominator_w_per_m2_k * inside_temperature.oracle_c;
            let rust_implied_solve_numerator_w =
                area_m2 * rust_solve_denominator_w_per_m2_k * inside_temperature.rust_c;

            Some(HeatBalanceInsideSolveMaxSampleDelta {
                key: surface.name.0.clone(),
                construction_name: construction.name.0.clone(),
                outside_boundary_condition: heat_balance_outside_boundary_condition_label(
                    surface.outside_boundary_condition,
                )
                .to_string(),
                sample_index,
                area_m2,
                ctf_inside_0_w_per_m2_k: zero.inside_w_per_m2_k,
                ctf_cross_0_w_per_m2_k: zero.cross_w_per_m2_k,
                iter_damp_w_per_m2_k: HEAT_BALANCE_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K,
                oracle_inside_face_temperature_c: inside_temperature.oracle_c,
                rust_inside_face_temperature_c: inside_temperature.rust_c,
                inside_face_temperature_delta_c: inside_temperature.abs_delta_c,
                oracle_inferred_reference_air_temperature_c,
                rust_inferred_reference_air_temperature_c,
                inferred_reference_air_temperature_delta_c:
                    (oracle_inferred_reference_air_temperature_c
                        - rust_inferred_reference_air_temperature_c)
                        .abs(),
                oracle_solve_denominator_w_per_m2_k,
                rust_solve_denominator_w_per_m2_k,
                solve_denominator_delta_w_per_m2_k: (oracle_solve_denominator_w_per_m2_k
                    - rust_solve_denominator_w_per_m2_k)
                    .abs(),
                oracle_implied_solve_numerator_w,
                rust_implied_solve_numerator_w,
                implied_solve_numerator_delta_w: (oracle_implied_solve_numerator_w
                    - rust_implied_solve_numerator_w)
                    .abs(),
                oracle_reference_air_source_w,
                rust_reference_air_source_w,
                reference_air_source_delta_w: (oracle_reference_air_source_w
                    - rust_reference_air_source_w)
                    .abs(),
                oracle_outside_temperature_source_w,
                rust_outside_temperature_source_w,
                outside_temperature_source_delta_w: (oracle_outside_temperature_source_w
                    - rust_outside_temperature_source_w)
                    .abs(),
                oracle_inside_history_term_w,
                rust_inside_history_term_w: rust_inside_history,
                inside_history_delta_w: (oracle_inside_history_term_w - rust_inside_history).abs(),
                rust_inside_history_temperature_term_w: rust_inside_history_temperature,
                rust_inside_history_flux_term_w: rust_inside_history_flux,
                oracle_inside_net_longwave_w: inside_net_longwave.oracle_c,
                rust_inside_net_longwave_w: inside_net_longwave.rust_c,
                inside_net_longwave_delta_w: inside_net_longwave.abs_delta_c,
            })
        })
        .collect()
}

fn heat_balance_adiabatic_history_max_sample_deltas(
    model: &SimulationModel,
    series: &[HeatBalanceSeriesDiagnostic],
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
    results: &ResultStore,
) -> Vec<HeatBalanceAdiabaticHistoryMaxSampleDelta> {
    model
        .typed
        .surfaces
        .iter()
        .filter(|surface| surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic)
        .filter_map(|surface| {
            let construction = model
                .typed
                .constructions
                .iter()
                .find(|construction| construction.id == surface.construction)?;
            let zero = ctf_coefficients.iter().find(|coefficient| {
                coefficient.time_index == 0
                    && coefficient
                        .construction_name
                        .eq_ignore_ascii_case(&construction.name.0)
            })?;
            let storage_series =
                heat_balance_series(series, &surface.name.0, "Surface Heat Storage Rate")?;
            let storage_point = storage_series.delta.max_delta_sample?;
            let sample_index = storage_point.index;
            let area_m2 = surface_area_m2(&surface.vertices);
            if area_m2 <= 0.0 {
                return None;
            }

            let inside_temperature = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Temperature",
                sample_index,
            )?;
            let outside_temperature = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Outside Face Temperature",
                sample_index,
            )?;
            let inside_conduction = heat_balance_sample_point_for_output(
                series,
                &surface.name.0,
                "Surface Inside Face Conduction Heat Transfer Rate",
                sample_index,
            )?;
            let rust_inside_current_outside = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
                sample_index,
            )?;
            let rust_inside_current_inside = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
                sample_index,
            )?;
            let rust_inside_history = heat_balance_result_series_value(
                results,
                &surface.name.0,
                SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE,
                sample_index,
            )?;

            let oracle_inside_current_term_w = area_m2
                * (outside_temperature.oracle_c * zero.cross_w_per_m2_k
                    - inside_temperature.oracle_c * zero.inside_w_per_m2_k);
            let rust_inside_current_term_w =
                rust_inside_current_outside + rust_inside_current_inside;
            let oracle_inside_current_if_outside_synced_w = area_m2
                * (inside_temperature.oracle_c * zero.cross_w_per_m2_k
                    - inside_temperature.oracle_c * zero.inside_w_per_m2_k);
            let rust_inside_current_if_outside_synced_w = area_m2
                * (inside_temperature.rust_c * zero.cross_w_per_m2_k
                    - inside_temperature.rust_c * zero.inside_w_per_m2_k);
            let oracle_inside_history_term_w =
                inside_conduction.oracle_c - oracle_inside_current_term_w;
            let oracle_inside_history_if_outside_synced_w =
                inside_conduction.oracle_c - oracle_inside_current_if_outside_synced_w;
            let rust_inside_history_if_outside_synced_w =
                inside_conduction.rust_c - rust_inside_current_if_outside_synced_w;
            let oracle_outside_minus_inside_c =
                outside_temperature.oracle_c - inside_temperature.oracle_c;
            let rust_outside_minus_inside_c =
                outside_temperature.rust_c - inside_temperature.rust_c;

            Some(HeatBalanceAdiabaticHistoryMaxSampleDelta {
                key: surface.name.0.clone(),
                construction_name: construction.name.0.clone(),
                sample_index,
                area_m2,
                ctf_inside_0_w_per_m2_k: zero.inside_w_per_m2_k,
                ctf_cross_0_w_per_m2_k: zero.cross_w_per_m2_k,
                oracle_inside_face_temperature_c: inside_temperature.oracle_c,
                rust_inside_face_temperature_c: inside_temperature.rust_c,
                inside_face_temperature_delta_c: inside_temperature.abs_delta_c,
                oracle_outside_face_temperature_c: outside_temperature.oracle_c,
                rust_outside_face_temperature_c: outside_temperature.rust_c,
                outside_face_temperature_delta_c: outside_temperature.abs_delta_c,
                oracle_outside_minus_inside_c,
                rust_outside_minus_inside_c,
                outside_minus_inside_delta_c: (oracle_outside_minus_inside_c
                    - rust_outside_minus_inside_c)
                    .abs(),
                oracle_inside_current_term_w,
                rust_inside_current_term_w,
                inside_current_delta_w: (oracle_inside_current_term_w - rust_inside_current_term_w)
                    .abs(),
                oracle_inside_current_if_outside_synced_w,
                rust_inside_current_if_outside_synced_w,
                inside_current_if_outside_synced_delta_w:
                    (oracle_inside_current_if_outside_synced_w
                        - rust_inside_current_if_outside_synced_w)
                        .abs(),
                oracle_inside_current_sync_shift_w: oracle_inside_current_if_outside_synced_w
                    - oracle_inside_current_term_w,
                rust_inside_current_sync_shift_w: rust_inside_current_if_outside_synced_w
                    - rust_inside_current_term_w,
                oracle_inside_history_term_w,
                rust_inside_history_term_w: rust_inside_history,
                inside_history_delta_w: (oracle_inside_history_term_w - rust_inside_history).abs(),
                oracle_inside_history_if_outside_synced_w,
                rust_inside_history_if_outside_synced_w,
                inside_history_if_outside_synced_delta_w:
                    (oracle_inside_history_if_outside_synced_w
                        - rust_inside_history_if_outside_synced_w)
                        .abs(),
            })
        })
        .collect()
}

fn heat_balance_inferred_reference_air_temperature_c(
    inside_face_temperature_c: f64,
    inside_convection_coefficient_w_per_m2_k: f64,
    area_m2: f64,
    inside_convection_heat_gain_rate_w: f64,
) -> f64 {
    let conductance_w_per_k = inside_convection_coefficient_w_per_m2_k * area_m2;
    if conductance_w_per_k.abs() <= f64::EPSILON {
        f64::NAN
    } else {
        inside_face_temperature_c + inside_convection_heat_gain_rate_w / conductance_w_per_k
    }
}

fn heat_balance_inside_solve_outside_temperature_source_w_per_m2(
    outside_boundary_condition: OutsideBoundaryCondition,
    ctf_cross_0_w_per_m2_k: f64,
    outside_face_temperature_c: f64,
) -> f64 {
    if outside_boundary_condition == OutsideBoundaryCondition::Adiabatic {
        0.0
    } else {
        ctf_cross_0_w_per_m2_k * outside_face_temperature_c
    }
}

fn heat_balance_outside_boundary_condition_label(
    outside_boundary_condition: OutsideBoundaryCondition,
) -> &'static str {
    match outside_boundary_condition {
        OutsideBoundaryCondition::Adiabatic => "adiabatic",
        OutsideBoundaryCondition::Foundation => "foundation",
        OutsideBoundaryCondition::Ground => "ground",
        OutsideBoundaryCondition::Outdoors => "outdoors",
        OutsideBoundaryCondition::Space => "space",
        OutsideBoundaryCondition::Surface => "surface",
        OutsideBoundaryCondition::Zone => "zone",
        OutsideBoundaryCondition::Other => "other",
    }
}

fn heat_balance_oracle_series_values(
    series: &[HeatBalanceSeriesDiagnostic],
    key: &str,
    variable_name: &str,
) -> Option<Vec<f64>> {
    series
        .iter()
        .find(|series| {
            series.output.key.eq_ignore_ascii_case(key)
                && series.output.variable.eq_ignore_ascii_case(variable_name)
        })
        .map(|series| {
            series
                .sample_rows
                .iter()
                .map(|point| point.oracle_c)
                .collect::<Vec<_>>()
        })
}

fn heat_balance_result_series_values(
    results: &ResultStore,
    key: &str,
    variable_name: &str,
) -> Option<Vec<f64>> {
    results
        .find_series(key, variable_name)
        .map(|series| series.values.clone())
}

fn heat_balance_result_series_value(
    results: &ResultStore,
    key: &str,
    variable_name: &str,
    sample_index: usize,
) -> Option<f64> {
    results
        .find_series(key, variable_name)
        .and_then(|series| series.values.get(sample_index).copied())
        .filter(|value| value.is_finite())
}

fn heat_balance_series<'a>(
    series: &'a [HeatBalanceSeriesDiagnostic],
    key: &str,
    variable_name: &str,
) -> Option<&'a HeatBalanceSeriesDiagnostic> {
    series.iter().find(|series| {
        series.output.key.eq_ignore_ascii_case(key)
            && series.output.variable.eq_ignore_ascii_case(variable_name)
    })
}

fn heat_balance_sample_point_for_output(
    series: &[HeatBalanceSeriesDiagnostic],
    key: &str,
    variable_name: &str,
    sample_index: usize,
) -> Option<DeltaPoint> {
    heat_balance_series(series, key, variable_name)
        .and_then(|series| heat_balance_sample_point(series, sample_index))
}

fn heat_balance_oracle_first_value(
    series: &[HeatBalanceSeriesDiagnostic],
    key: &str,
    variable_name: &str,
) -> Option<f64> {
    series
        .iter()
        .find(|series| {
            series.output.key.eq_ignore_ascii_case(key)
                && series.output.variable.eq_ignore_ascii_case(variable_name)
        })
        .map(|series| series.oracle_first_c)
        .filter(|value| value.is_finite())
}

fn heat_balance_rust_first_value(
    series: &[HeatBalanceSeriesDiagnostic],
    key: &str,
    variable_name: &str,
) -> Option<f64> {
    series
        .iter()
        .find(|series| {
            series.output.key.eq_ignore_ascii_case(key)
                && series.output.variable.eq_ignore_ascii_case(variable_name)
        })
        .map(|series| series.rust_first_c)
        .filter(|value| value.is_finite())
}

fn load_runtime_ctf_coefficients_from_eio(
    eio_path: &Path,
) -> Result<
    (
        Vec<ConstructionCtfCoefficientOverride>,
        HeatBalanceCtfSeedDiagnostic,
    ),
    String,
> {
    let policy = heat_balance_ctf_seed_policy_from_env()?;
    load_runtime_ctf_coefficients_from_eio_with_policy(eio_path, policy)
}

fn load_runtime_ctf_coefficients_from_eio_with_policy(
    eio_path: &Path,
    policy: HeatBalanceCtfSeedPolicy,
) -> Result<
    (
        Vec<ConstructionCtfCoefficientOverride>,
        HeatBalanceCtfSeedDiagnostic,
    ),
    String,
> {
    let constructions = load_eio_construction_ctf(eio_path).map_err(|error| error.to_string())?;
    let steady_constructions = constructions
        .iter()
        .filter(|construction| construction.ctf_count <= 1)
        .map(|construction| construction.construction_name.clone())
        .collect::<BTreeSet<_>>();
    let included_constructions = match policy {
        HeatBalanceCtfSeedPolicy::SteadyNoMassOnly => steady_constructions.clone(),
        HeatBalanceCtfSeedPolicy::AllEio => constructions
            .iter()
            .map(|construction| construction.construction_name.clone())
            .collect::<BTreeSet<_>>(),
    };
    let skipped_constructions = constructions
        .iter()
        .filter(|construction| !included_constructions.contains(&construction.construction_name))
        .map(|construction| HeatBalanceSkippedCtfConstruction {
            construction_name: construction.construction_name.clone(),
            ctf_count: construction.ctf_count,
            timestep_hours: construction.timestep_hours,
        })
        .collect::<Vec<_>>();
    let construction_summaries = constructions
        .iter()
        .map(|construction| HeatBalanceCtfConstructionSummary {
            construction_name: construction.construction_name.clone(),
            ctf_count: construction.ctf_count,
            timestep_hours: construction.timestep_hours,
            included: included_constructions.contains(&construction.construction_name),
        })
        .collect::<Vec<_>>();
    let coefficients =
        load_eio_construction_ctf_coefficients(eio_path).map_err(|error| error.to_string())?;
    let mut included_coefficients = Vec::new();
    let mut skipped_coefficients = 0;
    for coefficient in coefficients {
        if included_constructions.contains(&coefficient.construction_name) {
            included_coefficients.push(ConstructionCtfCoefficientOverride {
                construction_name: coefficient.construction_name,
                time_index: coefficient.time_index,
                outside_w_per_m2_k: coefficient.outside,
                cross_w_per_m2_k: coefficient.cross,
                inside_w_per_m2_k: coefficient.inside,
                flux: coefficient.flux,
            });
        } else {
            skipped_coefficients += 1;
        }
    }
    let ctf_seed = HeatBalanceCtfSeedDiagnostic {
        policy: policy.label(),
        included_constructions: included_constructions.into_iter().collect(),
        skipped_constructions,
        construction_summaries,
        included_coefficients: included_coefficients.len(),
        skipped_coefficients,
    };
    Ok((included_coefficients, ctf_seed))
}

fn disabled_heat_balance_ctf_seed_diagnostic() -> HeatBalanceCtfSeedDiagnostic {
    HeatBalanceCtfSeedDiagnostic {
        policy: "disabled",
        included_constructions: Vec::new(),
        skipped_constructions: Vec::new(),
        construction_summaries: Vec::new(),
        included_coefficients: 0,
        skipped_coefficients: 0,
    }
}

fn heat_balance_ctf_seed_policy_from_env() -> Result<HeatBalanceCtfSeedPolicy, String> {
    match std::env::var(HEAT_BALANCE_CTF_SEED_POLICY_ENV) {
        Ok(value) => parse_heat_balance_ctf_seed_policy(&value),
        Err(std::env::VarError::NotPresent) => Ok(HeatBalanceCtfSeedPolicy::SteadyNoMassOnly),
        Err(error) => Err(format!(
            "failed to read {HEAT_BALANCE_CTF_SEED_POLICY_ENV}: {error}"
        )),
    }
}

fn parse_heat_balance_ctf_seed_policy(value: &str) -> Result<HeatBalanceCtfSeedPolicy, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "" | "steady-no-mass-only" => Ok(HeatBalanceCtfSeedPolicy::SteadyNoMassOnly),
        "all-eio" => Ok(HeatBalanceCtfSeedPolicy::AllEio),
        other => Err(format!(
            "unsupported {HEAT_BALANCE_CTF_SEED_POLICY_ENV}: {other}; expected steady-no-mass-only or all-eio"
        )),
    }
}

fn apply_heat_balance_ctf_initial_history_policy_from_env(
    options: HeatBalanceSimulationOptions,
) -> Result<HeatBalanceSimulationOptions, String> {
    match std::env::var(HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY_ENV) {
        Ok(value) => parse_heat_balance_ctf_initial_history_policy(&value)
            .map(|policy| options.with_ctf_initial_history_policy(policy)),
        Err(std::env::VarError::NotPresent) => Ok(options),
        Err(error) => Err(format!(
            "failed to read {HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY_ENV}: {error}"
        )),
    }
}

fn parse_heat_balance_ctf_initial_history_policy(
    value: &str,
) -> Result<HeatBalanceCtfInitialHistoryPolicy, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "" | "boundary-u-value" => {
            Ok(HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue)
        }
        "energyplus-surf-initial" => Ok(HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial),
        other => Err(format!(
            "unsupported {HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY_ENV}: {other}; expected boundary-u-value or energyplus-surf-initial"
        )),
    }
}

fn heat_balance_zone_air_algorithm_from_env() -> Result<HeatBalanceZoneAirAlgorithm, String> {
    match std::env::var(HEAT_BALANCE_ZONE_AIR_ALGORITHM_ENV) {
        Ok(value) => parse_heat_balance_zone_air_algorithm(&value),
        Err(std::env::VarError::NotPresent) => {
            Ok(HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical)
        }
        Err(error) => Err(format!(
            "failed to read {HEAT_BALANCE_ZONE_AIR_ALGORITHM_ENV}: {error}"
        )),
    }
}

fn parse_heat_balance_zone_air_algorithm(
    value: &str,
) -> Result<HeatBalanceZoneAirAlgorithm, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "" | "simplified-analytical" => Ok(HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical),
        "energyplus-analytical-probe" => Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe),
        "energyplus-analytical-surface-first-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe)
        }
        "energyplus-analytical-coupled-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe)
        }
        "energyplus-analytical-coupled-previous-inside-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe)
        }
        "energyplus-analytical-coupled-previous-inside-doe2-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe)
        }
        "energyplus-analytical-coupled-previous-inside-quick-outside-probe" => Ok(
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe,
        ),
        "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-probe" => Ok(
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe,
        ),
        "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe)
        }
        "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe)
        }
        "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-longwave-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe)
        }
        "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-adiabatic-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-frozen-outside-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-commit-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-adiabatic-history-commit-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe)
        }
        "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-current-adiabatic-history-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe)
        }
        "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-interior-longwave-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe)
        }
        "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-probe" => Ok(
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe,
        ),
        "energyplus-analytical-coupled-previous-inside-quick-outside-interior-longwave-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe)
        }
        "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe)
        }
        "energyplus-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe)
        }
        "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe)
        }
        "energyplus-analytical-coupled-previous-boundary-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe)
        }
        "energyplus-third-order-probe" => {
            Ok(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe)
        }
        other => Err(format!(
            "unsupported {HEAT_BALANCE_ZONE_AIR_ALGORITHM_ENV}: {other}; expected simplified-analytical, energyplus-analytical-probe, energyplus-analytical-surface-first-probe, energyplus-analytical-coupled-probe, energyplus-analytical-coupled-previous-inside-probe, energyplus-analytical-coupled-previous-inside-doe2-probe, energyplus-analytical-coupled-previous-inside-quick-outside-probe, energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-probe, energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe, energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe, energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-longwave-probe, energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-adiabatic-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-frozen-outside-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-commit-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-adiabatic-history-commit-probe, energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-current-adiabatic-history-probe, energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-interior-longwave-probe, energyplus-analytical-coupled-previous-inside-quick-outside-doe2-probe, energyplus-analytical-coupled-previous-inside-quick-outside-interior-longwave-probe, energyplus-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-probe, energyplus-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-probe, energyplus-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-probe, energyplus-analytical-coupled-previous-boundary-probe, or energyplus-third-order-probe"
        )),
    }
}

fn apply_heat_balance_warmup_minimum_days_from_env(
    options: HeatBalanceSimulationOptions,
) -> Result<HeatBalanceSimulationOptions, String> {
    match std::env::var(HEAT_BALANCE_WARMUP_MINIMUM_DAYS_ENV) {
        Ok(value) => {
            parse_heat_balance_warmup_minimum_days(&value).map(|minimum_days| match minimum_days {
                Some(minimum_days) => options.with_warmup_minimum_days(minimum_days),
                None => options,
            })
        }
        Err(std::env::VarError::NotPresent) => Ok(options),
        Err(error) => Err(format!(
            "failed to read {HEAT_BALANCE_WARMUP_MINIMUM_DAYS_ENV}: {error}"
        )),
    }
}

fn parse_heat_balance_warmup_minimum_days(value: &str) -> Result<Option<u32>, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let days = trimmed.parse::<u32>().map_err(|error| {
        format!("unsupported {HEAT_BALANCE_WARMUP_MINIMUM_DAYS_ENV}: {trimmed}; expected positive integer days ({error})")
    })?;
    if days == 0 {
        return Ok(None);
    }
    Ok(Some(days))
}

fn apply_heat_balance_surface_iterations_from_env(
    options: HeatBalanceSimulationOptions,
) -> Result<HeatBalanceSimulationOptions, String> {
    match std::env::var(HEAT_BALANCE_SURFACE_ITERATIONS_ENV) {
        Ok(value) => parse_heat_balance_surface_iterations(&value).map(|iteration_count| {
            match iteration_count {
                Some(iteration_count) => options.with_surface_iteration_count(iteration_count),
                None => options,
            }
        }),
        Err(std::env::VarError::NotPresent) => Ok(options),
        Err(error) => Err(format!(
            "failed to read {HEAT_BALANCE_SURFACE_ITERATIONS_ENV}: {error}"
        )),
    }
}

fn parse_heat_balance_surface_iterations(value: &str) -> Result<Option<u32>, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let iteration_count = trimmed.parse::<u32>().map_err(|error| {
        format!("unsupported {HEAT_BALANCE_SURFACE_ITERATIONS_ENV}: {trimmed}; expected positive integer iteration count ({error})")
    })?;
    if iteration_count == 0 {
        return Ok(None);
    }
    Ok(Some(iteration_count))
}

fn heat_balance_ctf_initial_history_policy_label(
    ctf_initial_history_policy: HeatBalanceCtfInitialHistoryPolicy,
) -> &'static str {
    match ctf_initial_history_policy {
        HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue => "boundary-u-value",
        HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial => "energyplus-surf-initial",
    }
}

fn heat_balance_zone_air_algorithm_label(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> &'static str {
    match zone_air_algorithm {
        HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical => "simplified-analytical",
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe => "energyplus-analytical-probe",
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe => {
            "energyplus-analytical-surface-first-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe => {
            "energyplus-analytical-coupled-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe => {
            "energyplus-analytical-coupled-previous-inside-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe => {
            "energyplus-analytical-coupled-previous-inside-doe2-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-longwave-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-adiabatic-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-frozen-outside-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-commit-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-adiabatic-history-commit-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe => {
            "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-current-adiabatic-history-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-interior-longwave-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-interior-longwave-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe => {
            "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe => {
            "energyplus-analytical-coupled-previous-boundary-probe"
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe => "energyplus-third-order-probe",
    }
}

fn run_period_eso_values(series: &ep_compare::EsoTimeSeries) -> Vec<f64> {
    let run_period_values = series
        .samples
        .iter()
        .filter(|sample| {
            sample
                .timestamp
                .as_deref()
                .and_then(eso_timestamp_environment)
                .is_some_and(|environment| {
                    environment.to_ascii_uppercase().starts_with("RUN PERIOD")
                })
        })
        .map(|sample| sample.value)
        .collect::<Vec<_>>();

    if run_period_values.is_empty() {
        series.samples.iter().map(|sample| sample.value).collect()
    } else {
        run_period_values
    }
}

fn eso_timestamp_environment(timestamp: &str) -> Option<&str> {
    timestamp
        .strip_prefix("env=")?
        .split(';')
        .next()
        .filter(|environment| !environment.is_empty())
}

fn eio_run_period_warmup_days(path: &Path) -> Result<Option<u32>, ep_compare::EioError> {
    let rows = load_eio_warmup_environments(path)?;
    Ok(rows
        .iter()
        .rev()
        .find(|row| {
            row.environment_type
                .eq_ignore_ascii_case("WeatherFileRunPeriod")
                || row.environment_name.starts_with("RUN PERIOD")
        })
        .map(|row| row.warmup_days))
}

fn heat_balance_max_abs_delta(diagnostic: &HeatBalanceConformanceDiagnostic) -> f64 {
    diagnostic
        .series
        .iter()
        .map(|series| series.delta.max_abs_delta_c)
        .fold(0.0, f64::max)
}

fn heat_balance_max_rmse_delta(diagnostic: &HeatBalanceConformanceDiagnostic) -> f64 {
    diagnostic
        .series
        .iter()
        .map(|series| series.delta.rmse_delta_c)
        .fold(0.0, f64::max)
}

fn heat_balance_max_rel_delta(diagnostic: &HeatBalanceConformanceDiagnostic) -> f64 {
    diagnostic
        .series
        .iter()
        .map(|series| series.delta.max_rel_delta)
        .fold(0.0, f64::max)
}

fn build_zone_temperature_diagnostic(
    input_path: &Path,
    weather_path: &Path,
    eso_path: &Path,
) -> Result<ZoneTemperatureDiagnostic, String> {
    let raw_model = load_epjson_file(input_path).map_err(|error| error.to_string())?;
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        return Err(format_compile_diagnostics(&result.report));
    };
    let Some(zone) = model.zones.first() else {
        return Err("no Zone objects are available for comparison".to_string());
    };
    let zone_name = zone.name.0.clone();

    let oracle_values = load_eso_series(eso_path, &zone_name, "Zone Mean Air Temperature")
        .map_err(|error| error.to_string())?;
    if oracle_values.is_empty() {
        return Err("EnergyPlus zone temperature series is empty".to_string());
    }

    let weather_values =
        load_epw_dry_bulb_series(weather_path).map_err(|error| error.to_string())?;
    if weather_values.len() < oracle_values.len() {
        return Err(format!(
            "EPW dry-bulb series has {} samples but ESO requires {}",
            weather_values.len(),
            oracle_values.len()
        ));
    }

    let simulation_model = SimulationModel::from_typed(model);
    let simulation = simulate_heat_balance_zone_air_temperatures(
        &simulation_model,
        &weather_values,
        HeatBalanceSimulationOptions::hourly_samples(oracle_values.len()),
    )
    .map_err(|error| error.to_string())?;
    let Some(rust_series) = simulation
        .results
        .find_series(&zone_name, "Zone Mean Air Temperature")
    else {
        return Err("heat-balance simulation did not write zone temperature output".to_string());
    };

    let delta = delta_summary(&oracle_values, &rust_series.values);
    let finite = oracle_values
        .iter()
        .chain(rust_series.values.iter())
        .all(|value| value.is_finite());
    let extracted = finite && delta.length_match;

    Ok(ZoneTemperatureDiagnostic {
        zone_name,
        samples: delta.samples,
        heat_balance_timesteps: simulation.summary.timestep_count,
        heat_balance_run_period_timesteps: simulation.summary.run_period_timestep_count,
        heat_balance_warmup: simulation.summary.warmup.into(),
        zone_count: simulation.summary.zone_count,
        surface_count: simulation.summary.surface_count,
        oracle_first_c: oracle_values[0],
        rust_first_c: rust_series.values.first().copied().unwrap_or(f64::NAN),
        oracle_last_c: oracle_values[oracle_values.len() - 1],
        rust_last_c: rust_series.values.last().copied().unwrap_or(f64::NAN),
        delta,
        status: if extracted { "extracted" } else { "failed" },
    })
}

fn run_compare_zone_temperature(args: &[String]) -> i32 {
    let parsed = match parse_zone_temperature_compare_args(args) {
        Ok(parsed) => parsed,
        Err(error) => {
            eprintln!("{error}");
            eprintln!("{ZONE_TEMPERATURE_COMPARE_USAGE}");
            return 2;
        }
    };

    let input_path = parsed.input_path.as_path();
    let weather_path = parsed.weather_path.as_path();
    let eso_path = parsed.eso_path.as_path();

    let diagnostic = match build_zone_temperature_diagnostic(input_path, weather_path, eso_path) {
        Ok(diagnostic) => diagnostic,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    if let Some(report_dir) = &parsed.report_dir
        && let Err(error) = write_zone_temperature_diagnostic_report(report_dir, &diagnostic, None)
    {
        eprintln!("{error}");
        return 1;
    }

    println!("Zone Temperature Diagnostic");
    println!("  comparison_class: diagnostic-only");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: none");
    println!("  runtime_class: heat-balance-state-shell");
    println!("  zone: {}", diagnostic.zone_name);
    println!(
        "  heat_balance_timesteps: {}",
        diagnostic.heat_balance_timesteps
    );
    println!(
        "  heat_balance_run_period_timesteps: {}",
        diagnostic.heat_balance_run_period_timesteps
    );
    print_heat_balance_warmup("  ", &diagnostic.heat_balance_warmup);
    println!("  zone_count: {}", diagnostic.zone_count);
    println!("  surface_count: {}", diagnostic.surface_count);
    println!("  samples: {}", diagnostic.samples);
    println!("  length_match: {}", diagnostic.delta.length_match);
    println!("  max_abs_delta: {:.6}", diagnostic.delta.max_abs_delta_c);
    println!("  mean_abs_delta: {:.6}", diagnostic.delta.mean_abs_delta_c);
    print_delta_sample("  first_delta_sample", diagnostic.delta.first_delta_sample);
    print_delta_sample("  max_delta_sample", diagnostic.delta.max_delta_sample);
    println!("  oracle_first_c: {:.6}", diagnostic.oracle_first_c);
    println!("  rust_first_c: {:.6}", diagnostic.rust_first_c);
    println!("  oracle_last_c: {:.6}", diagnostic.oracle_last_c);
    println!("  rust_last_c: {:.6}", diagnostic.rust_last_c);
    println!("  exact_match: not_available");
    println!("  exit_code_semantics: extraction-only");
    if let Some(report_dir) = &parsed.report_dir {
        println!("  report_dir: {}", report_dir.display());
    }
    println!("  status: {}", diagnostic.status);

    if diagnostic.status == "extracted" {
        0
    } else {
        1
    }
}

fn run_compare_weather_fields(args: &[String]) -> i32 {
    let Some(epw_path) = args.first() else {
        eprintln!("missing weather path");
        eprintln!("usage: eplus-rs compare weather-fields <weather.epw> <eplusout.eso>");
        return 2;
    };
    let Some(eso_path) = args.get(1) else {
        eprintln!("missing eplusout.eso path");
        eprintln!("usage: eplus-rs compare weather-fields <weather.epw> <eplusout.eso>");
        return 2;
    };

    let weather_records = match load_epw_records(epw_path) {
        Ok(records) => records,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };

    let tolerance = Tolerance {
        absolute: 1.0e-5,
        relative: 1.0e-6,
    };
    let mut passed = true;
    println!("Weather Field Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: absolute-0.00001-relative-0.000001");
    println!("  fields: {}", WEATHER_COMPARE_FIELDS.len());
    for field in WEATHER_COMPARE_FIELDS {
        let oracle_values = match load_eso_series(eso_path, "Environment", field.variable_name) {
            Ok(values) => values,
            Err(error) => {
                eprintln!("{error}");
                return 1;
            }
        };
        if weather_records.len() < oracle_values.len() {
            eprintln!(
                "EPW {} series has {} samples but ESO requires {}",
                field.label,
                weather_records.len(),
                oracle_values.len()
            );
            return 1;
        }

        let weather_values = weather_records
            .iter()
            .take(oracle_values.len())
            .map(|record| (field.epw_value)(record))
            .collect::<Vec<_>>();
        let comparison = compare_series(&oracle_values, &weather_values, tolerance);
        passed &= comparison.passed;

        println!(
            "  field: {} variable: {} samples: {} max_abs_delta: {} status: {}",
            field.label,
            field.variable_name,
            comparison.samples,
            comparison.max_abs_delta,
            if comparison.passed { "pass" } else { "fail" }
        );
        print_first_divergence("    ", comparison.first_divergence);
    }
    println!("  status: {}", if passed { "pass" } else { "fail" });

    if passed { 0 } else { 1 }
}

struct WeatherCompareField {
    label: &'static str,
    variable_name: &'static str,
    epw_value: fn(&ep_runtime::EpwRecord) -> f64,
}

const WEATHER_COMPARE_FIELDS: &[WeatherCompareField] = &[
    WeatherCompareField {
        label: "dry_bulb_c",
        variable_name: "Site Outdoor Air Drybulb Temperature",
        epw_value: |record| record.dry_bulb_c,
    },
    WeatherCompareField {
        label: "dew_point_c",
        variable_name: "Site Outdoor Air Dewpoint Temperature",
        epw_value: |record| record.dew_point_c,
    },
    WeatherCompareField {
        label: "relative_humidity_percent",
        variable_name: "Site Outdoor Air Relative Humidity",
        epw_value: |record| record.relative_humidity_percent,
    },
    WeatherCompareField {
        label: "barometric_pressure_pa",
        variable_name: "Site Outdoor Air Barometric Pressure",
        epw_value: |record| record.atmospheric_pressure_pa,
    },
    WeatherCompareField {
        label: "wind_speed_m_per_s",
        variable_name: "Site Wind Speed",
        epw_value: |record| record.wind_speed_m_per_s,
    },
    WeatherCompareField {
        label: "wind_direction_deg",
        variable_name: "Site Wind Direction",
        epw_value: |record| record.wind_direction_deg,
    },
];

struct ConstructionMaterialRow {
    construction_name: String,
    layer_count: usize,
    outside_layer_material_name: String,
    thermal_conductance_w_per_m2_k: f64,
    material_thickness_m: Option<f64>,
    material_conductivity_w_per_m_k: Option<f64>,
    material_density_kg_per_m3: Option<f64>,
    material_specific_heat_j_per_kg_k: Option<f64>,
    material_thermal_resistance_m2_k_per_w: f64,
}

fn construction_material_rows(model: &TypedModel) -> Result<Vec<ConstructionMaterialRow>, String> {
    model
        .constructions
        .iter()
        .map(|construction| construction_material_row(model, construction))
        .collect()
}

fn construction_layer_material_count(model: &TypedModel) -> usize {
    model
        .constructions
        .iter()
        .flat_map(|construction| {
            let layer_ids = if construction.layers.is_empty() {
                std::slice::from_ref(&construction.outside_layer)
            } else {
                construction.layers.as_slice()
            };
            layer_ids.iter().copied()
        })
        .collect::<BTreeSet<_>>()
        .len()
}

fn construction_material_row(
    model: &TypedModel,
    construction: &Construction,
) -> Result<ConstructionMaterialRow, String> {
    let layer_materials = materials_for_construction(model, construction)?;
    let material = layer_materials.first().ok_or_else(|| {
        format!(
            "construction {} references no material layers",
            construction.name.0
        )
    })?;
    let material_thermal_resistance_m2_k_per_w =
        material.thermal_resistance().ok_or_else(|| {
            format!(
                "construction {} outside layer {} has no positive thermal resistance",
                construction.name.0, material.name.0
            )
        })?;
    let mut construction_thermal_resistance_m2_k_per_w = 0.0;
    for material in &layer_materials {
        construction_thermal_resistance_m2_k_per_w +=
            material.thermal_resistance().ok_or_else(|| {
                format!(
                    "construction {} layer {} has no positive thermal resistance",
                    construction.name.0, material.name.0
                )
            })?;
    }
    Ok(ConstructionMaterialRow {
        construction_name: construction.name.0.clone(),
        layer_count: layer_materials.len(),
        outside_layer_material_name: material.name.0.clone(),
        thermal_conductance_w_per_m2_k: 1.0 / construction_thermal_resistance_m2_k_per_w,
        material_thickness_m: material.thickness_m,
        material_conductivity_w_per_m_k: material.conductivity_w_per_m_k,
        material_density_kg_per_m3: material.density_kg_per_m3,
        material_specific_heat_j_per_kg_k: material.specific_heat_j_per_kg_k,
        material_thermal_resistance_m2_k_per_w,
    })
}

fn materials_for_construction<'a>(
    model: &'a TypedModel,
    construction: &Construction,
) -> Result<Vec<&'a Material>, String> {
    let layer_ids = if construction.layers.is_empty() {
        std::slice::from_ref(&construction.outside_layer)
    } else {
        construction.layers.as_slice()
    };
    layer_ids
        .iter()
        .map(|layer_id| {
            model
                .materials
                .iter()
                .find(|material| material.id == *layer_id)
                .ok_or_else(|| {
                    format!(
                        "construction {} references missing material layer",
                        construction.name.0
                    )
                })
        })
        .collect()
}

struct OtherEquipmentNominalRow {
    equipment_name: String,
    schedule_name: String,
    zone_name: String,
    zone_floor_area_m2: f64,
    equipment_level_w: f64,
    equipment_per_floor_area_w_per_m2: f64,
    fraction_latent: f64,
    fraction_radiant: f64,
    fraction_lost: f64,
    fraction_convected: f64,
}

fn other_equipment_nominal_rows(model: &TypedModel) -> Vec<OtherEquipmentNominalRow> {
    let geometry = zone_geometry_summaries(model);
    model
        .other_equipment
        .iter()
        .map(|equipment| {
            let zone_name = model
                .zones
                .iter()
                .find(|zone| zone.id == equipment.zone)
                .map(|zone| zone.name.0.clone())
                .unwrap_or_else(|| "MISSING ZONE".to_string());
            let zone_floor_area_m2 = geometry
                .iter()
                .find(|summary| summary.zone_id == equipment.zone)
                .map(|summary| summary.floor_area_m2)
                .unwrap_or(0.0);
            let equipment_per_floor_area_w_per_m2 = if zone_floor_area_m2.abs() > f64::EPSILON {
                equipment.design_level_w / zone_floor_area_m2
            } else {
                0.0
            };

            OtherEquipmentNominalRow {
                equipment_name: equipment.name.0.clone(),
                schedule_name: schedule_name_for_id(model, equipment.schedule),
                zone_name,
                zone_floor_area_m2,
                equipment_level_w: equipment.design_level_w,
                equipment_per_floor_area_w_per_m2,
                fraction_latent: equipment.fraction_latent,
                fraction_radiant: equipment.fraction_radiant,
                fraction_lost: equipment.fraction_lost,
                fraction_convected: fraction_convected(equipment),
            }
        })
        .collect()
}

fn schedule_name_for_id(model: &TypedModel, schedule_id: Option<ScheduleId>) -> String {
    let Some(schedule_id) = schedule_id else {
        return "NONE".to_string();
    };

    model
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .map(|schedule| schedule.name.0.clone())
        .or_else(|| {
            model
                .compact_schedules
                .iter()
                .find(|schedule| schedule.id == schedule_id)
                .map(|schedule| schedule.name.0.clone())
        })
        .unwrap_or_else(|| "MISSING SCHEDULE".to_string())
}

fn fraction_convected(equipment: &OtherEquipment) -> f64 {
    1.0 - equipment.fraction_latent - equipment.fraction_radiant - equipment.fraction_lost
}

fn internal_gain_row_matches(
    rust_row: &OtherEquipmentNominalRow,
    oracle_row: &ep_compare::EioOtherEquipmentNominal,
    tolerance: Tolerance,
) -> bool {
    oracle_row
        .zone_name
        .eq_ignore_ascii_case(&rust_row.zone_name)
        && oracle_row
            .schedule_name
            .eq_ignore_ascii_case(&rust_row.schedule_name)
        && tolerance.accepts(oracle_row.zone_floor_area_m2, rust_row.zone_floor_area_m2)
        && tolerance.accepts(oracle_row.equipment_level_w, rust_row.equipment_level_w)
        && tolerance.accepts(
            oracle_row.equipment_per_floor_area_w_per_m2,
            rust_row.equipment_per_floor_area_w_per_m2,
        )
        && tolerance.accepts(oracle_row.fraction_latent, rust_row.fraction_latent)
        && tolerance.accepts(oracle_row.fraction_radiant, rust_row.fraction_radiant)
        && tolerance.accepts(oracle_row.fraction_lost, rust_row.fraction_lost)
        && tolerance.accepts(oracle_row.fraction_convected, rust_row.fraction_convected)
}

fn construction_material_row_matches(
    rust_row: &ConstructionMaterialRow,
    oracle_construction: &ep_compare::EioConstructionCtf,
    oracle_material: &ep_compare::EioMaterialCtfSummary,
    tolerance: Tolerance,
) -> bool {
    oracle_construction.layer_count == rust_row.layer_count
        && oracle_material
            .material_name
            .eq_ignore_ascii_case(&rust_row.outside_layer_material_name)
        && tolerance.accepts(
            oracle_construction.thermal_conductance_w_per_m2_k,
            rust_row.thermal_conductance_w_per_m2_k,
        )
        && material_field_matches(
            oracle_material.thickness_m,
            rust_row.material_thickness_m,
            tolerance,
        )
        && material_field_matches(
            oracle_material.conductivity_w_per_m_k,
            rust_row.material_conductivity_w_per_m_k,
            tolerance,
        )
        && material_field_matches(
            oracle_material.density_kg_per_m3,
            rust_row.material_density_kg_per_m3,
            tolerance,
        )
        && material_field_matches(
            oracle_material.specific_heat_j_per_kg_k,
            rust_row.material_specific_heat_j_per_kg_k,
            tolerance,
        )
        && tolerance.accepts(
            oracle_material.thermal_resistance_m2_k_per_w,
            rust_row.material_thermal_resistance_m2_k_per_w,
        )
}

fn material_field_matches(expected: f64, observed: Option<f64>, tolerance: Tolerance) -> bool {
    tolerance.accepts(expected, observed.unwrap_or(0.0))
}

fn record_first_divergence(first_divergence: &mut Option<String>, value: String) {
    if first_divergence.is_none() {
        *first_divergence = Some(value);
    }
}

fn record_geometry_field_divergence(
    first_divergence: &mut Option<String>,
    rust_zone: &ZoneGeometrySummary,
    oracle_zone: &ep_compare::EioZoneGeometry,
    tolerance: Tolerance,
) {
    if oracle_zone.surface_count != rust_zone.surface_count {
        record_first_divergence(
            first_divergence,
            format!(
                "zone {} field surfaces expected {} observed {}",
                rust_zone.zone_name, oracle_zone.surface_count, rust_zone.surface_count
            ),
        );
        return;
    }

    if !tolerance.accepts(oracle_zone.floor_area_m2, rust_zone.floor_area_m2) {
        record_first_divergence(
            first_divergence,
            format!(
                "zone {} field floor_area_m2 expected {:.6} observed {:.6}",
                rust_zone.zone_name, oracle_zone.floor_area_m2, rust_zone.floor_area_m2
            ),
        );
        return;
    }

    let Some(volume_m3) = rust_zone.volume_m3 else {
        record_first_divergence(
            first_divergence,
            format!(
                "zone {} field volume_m3 expected {:.6} observed unavailable",
                rust_zone.zone_name, oracle_zone.volume_m3
            ),
        );
        return;
    };
    if !tolerance.accepts(oracle_zone.volume_m3, volume_m3) {
        record_first_divergence(
            first_divergence,
            format!(
                "zone {} field volume_m3 expected {:.6} observed {:.6}",
                rust_zone.zone_name, oracle_zone.volume_m3, volume_m3
            ),
        );
        return;
    }

    if !tolerance.accepts(
        oracle_zone.exterior_gross_wall_area_m2,
        rust_zone.exterior_wall_area_m2,
    ) {
        record_first_divergence(
            first_divergence,
            format!(
                "zone {} field exterior_wall_area_m2 expected {:.6} observed {:.6}",
                rust_zone.zone_name,
                oracle_zone.exterior_gross_wall_area_m2,
                rust_zone.exterior_wall_area_m2
            ),
        );
    }
}

fn surface_geometry_row_matches(
    rust_surface: &SurfaceGeometrySummary,
    oracle_surface: &ep_compare::EioHeatTransferSurface,
    tolerance: Tolerance,
) -> bool {
    oracle_surface
        .surface_class
        .eq_ignore_ascii_case(surface_type_label(rust_surface.surface_type))
        && tolerance.accepts(oracle_surface.area_net_m2, rust_surface.area_m2)
        && tolerance.accepts(oracle_surface.area_gross_m2, rust_surface.area_m2)
        && angle_accepts(
            oracle_surface.azimuth_deg,
            rust_surface.azimuth_deg,
            tolerance,
        )
        && angle_accepts(oracle_surface.tilt_deg, rust_surface.tilt_deg, tolerance)
}

fn record_surface_geometry_field_divergence(
    first_divergence: &mut Option<String>,
    rust_surface: &SurfaceGeometrySummary,
    oracle_surface: &ep_compare::EioHeatTransferSurface,
    tolerance: Tolerance,
) {
    let rust_class = surface_type_label(rust_surface.surface_type);
    if !oracle_surface
        .surface_class
        .eq_ignore_ascii_case(rust_class)
    {
        record_first_divergence(
            first_divergence,
            format!(
                "surface {} field class expected {} observed {}",
                rust_surface.surface_name, oracle_surface.surface_class, rust_class
            ),
        );
        return;
    }

    if !tolerance.accepts(oracle_surface.area_net_m2, rust_surface.area_m2) {
        record_first_divergence(
            first_divergence,
            format!(
                "surface {} field area_net_m2 expected {:.6} observed {:.6}",
                rust_surface.surface_name, oracle_surface.area_net_m2, rust_surface.area_m2
            ),
        );
        return;
    }

    if !tolerance.accepts(oracle_surface.area_gross_m2, rust_surface.area_m2) {
        record_first_divergence(
            first_divergence,
            format!(
                "surface {} field area_gross_m2 expected {:.6} observed {:.6}",
                rust_surface.surface_name, oracle_surface.area_gross_m2, rust_surface.area_m2
            ),
        );
        return;
    }

    if !angle_accepts(
        oracle_surface.azimuth_deg,
        rust_surface.azimuth_deg,
        tolerance,
    ) {
        record_first_divergence(
            first_divergence,
            format!(
                "surface {} field azimuth_deg expected {:.6} observed {:.6}",
                rust_surface.surface_name, oracle_surface.azimuth_deg, rust_surface.azimuth_deg
            ),
        );
        return;
    }

    if !angle_accepts(oracle_surface.tilt_deg, rust_surface.tilt_deg, tolerance) {
        record_first_divergence(
            first_divergence,
            format!(
                "surface {} field tilt_deg expected {:.6} observed {:.6}",
                rust_surface.surface_name, oracle_surface.tilt_deg, rust_surface.tilt_deg
            ),
        );
    }
}

fn surface_type_label(surface_type: SurfaceType) -> &'static str {
    match surface_type {
        SurfaceType::Ceiling => "Ceiling",
        SurfaceType::Floor => "Floor",
        SurfaceType::Roof => "Roof",
        SurfaceType::Wall => "Wall",
    }
}

fn angle_accepts(expected: f64, observed: f64, tolerance: Tolerance) -> bool {
    let delta = angle_abs_delta_deg(expected, observed);
    if delta <= tolerance.absolute {
        return true;
    }

    let scale = expected.abs().max(observed.abs());
    delta <= tolerance.relative * scale
}

fn angle_abs_delta_deg(expected: f64, observed: f64) -> f64 {
    let delta = (expected - observed).rem_euclid(360.0);
    delta.min(360.0 - delta)
}

fn record_construction_material_field_divergence(
    first_divergence: &mut Option<String>,
    rust_row: &ConstructionMaterialRow,
    oracle_construction: &ep_compare::EioConstructionCtf,
    oracle_material: &ep_compare::EioMaterialCtfSummary,
    tolerance: Tolerance,
) {
    if oracle_construction.layer_count != rust_row.layer_count {
        record_first_divergence(
            first_divergence,
            format!(
                "construction {} field layer_count expected {} observed {}",
                rust_row.construction_name, oracle_construction.layer_count, rust_row.layer_count
            ),
        );
        return;
    }

    if !oracle_material
        .material_name
        .eq_ignore_ascii_case(&rust_row.outside_layer_material_name)
    {
        record_first_divergence(
            first_divergence,
            format!(
                "construction {} field outside_layer expected {} observed {}",
                rust_row.construction_name,
                oracle_material.material_name,
                rust_row.outside_layer_material_name
            ),
        );
        return;
    }

    if !tolerance.accepts(
        oracle_construction.thermal_conductance_w_per_m2_k,
        rust_row.thermal_conductance_w_per_m2_k,
    ) {
        record_first_divergence(
            first_divergence,
            format!(
                "construction {} field thermal_conductance_w_per_m2_k expected {:.6} observed {:.6}",
                rust_row.construction_name,
                oracle_construction.thermal_conductance_w_per_m2_k,
                rust_row.thermal_conductance_w_per_m2_k
            ),
        );
        return;
    }

    for (field, expected, observed) in [
        (
            "material_thickness_m",
            oracle_material.thickness_m,
            rust_row.material_thickness_m,
        ),
        (
            "material_conductivity_w_per_m_k",
            oracle_material.conductivity_w_per_m_k,
            rust_row.material_conductivity_w_per_m_k,
        ),
        (
            "material_density_kg_per_m3",
            oracle_material.density_kg_per_m3,
            rust_row.material_density_kg_per_m3,
        ),
        (
            "material_specific_heat_j_per_kg_k",
            oracle_material.specific_heat_j_per_kg_k,
            rust_row.material_specific_heat_j_per_kg_k,
        ),
    ] {
        if !material_field_matches(expected, observed, tolerance) {
            record_first_divergence(
                first_divergence,
                format!(
                    "construction {} field {} expected {:.6} observed {:.6}",
                    rust_row.construction_name,
                    field,
                    expected,
                    observed.unwrap_or(0.0)
                ),
            );
            return;
        }
    }

    if !tolerance.accepts(
        oracle_material.thermal_resistance_m2_k_per_w,
        rust_row.material_thermal_resistance_m2_k_per_w,
    ) {
        record_first_divergence(
            first_divergence,
            format!(
                "construction {} field material_thermal_resistance_m2_k_per_w expected {:.6} observed {:.6}",
                rust_row.construction_name,
                oracle_material.thermal_resistance_m2_k_per_w,
                rust_row.material_thermal_resistance_m2_k_per_w
            ),
        );
    }
}

fn record_internal_gain_field_divergence(
    first_divergence: &mut Option<String>,
    rust_row: &OtherEquipmentNominalRow,
    oracle_row: &ep_compare::EioOtherEquipmentNominal,
    tolerance: Tolerance,
) {
    if !oracle_row
        .zone_name
        .eq_ignore_ascii_case(&rust_row.zone_name)
    {
        record_first_divergence(
            first_divergence,
            format!(
                "other_equipment {} field zone expected {} observed {}",
                rust_row.equipment_name, oracle_row.zone_name, rust_row.zone_name
            ),
        );
        return;
    }

    if !oracle_row
        .schedule_name
        .eq_ignore_ascii_case(&rust_row.schedule_name)
    {
        record_first_divergence(
            first_divergence,
            format!(
                "other_equipment {} field schedule expected {} observed {}",
                rust_row.equipment_name, oracle_row.schedule_name, rust_row.schedule_name
            ),
        );
        return;
    }

    if !tolerance.accepts(oracle_row.zone_floor_area_m2, rust_row.zone_floor_area_m2) {
        record_first_divergence(
            first_divergence,
            format!(
                "other_equipment {} field zone_floor_area_m2 expected {:.6} observed {:.6}",
                rust_row.equipment_name, oracle_row.zone_floor_area_m2, rust_row.zone_floor_area_m2
            ),
        );
        return;
    }

    if !tolerance.accepts(oracle_row.equipment_level_w, rust_row.equipment_level_w) {
        record_first_divergence(
            first_divergence,
            format!(
                "other_equipment {} field equipment_level_w expected {:.6} observed {:.6}",
                rust_row.equipment_name, oracle_row.equipment_level_w, rust_row.equipment_level_w
            ),
        );
        return;
    }

    if !tolerance.accepts(
        oracle_row.equipment_per_floor_area_w_per_m2,
        rust_row.equipment_per_floor_area_w_per_m2,
    ) {
        record_first_divergence(
            first_divergence,
            format!(
                "other_equipment {} field equipment_per_floor_area_w_per_m2 expected {:.6} observed {:.6}",
                rust_row.equipment_name,
                oracle_row.equipment_per_floor_area_w_per_m2,
                rust_row.equipment_per_floor_area_w_per_m2
            ),
        );
        return;
    }

    for (field, expected, observed) in [
        (
            "fraction_latent",
            oracle_row.fraction_latent,
            rust_row.fraction_latent,
        ),
        (
            "fraction_radiant",
            oracle_row.fraction_radiant,
            rust_row.fraction_radiant,
        ),
        (
            "fraction_lost",
            oracle_row.fraction_lost,
            rust_row.fraction_lost,
        ),
        (
            "fraction_convected",
            oracle_row.fraction_convected,
            rust_row.fraction_convected,
        ),
    ] {
        if !tolerance.accepts(expected, observed) {
            record_first_divergence(
                first_divergence,
                format!(
                    "other_equipment {} field {} expected {:.6} observed {:.6}",
                    rust_row.equipment_name, field, expected, observed
                ),
            );
            return;
        }
    }
}

fn delta_summary(expected: &[f64], observed: &[f64]) -> DeltaSummary {
    let samples = expected.len().min(observed.len());
    if samples == 0 {
        return DeltaSummary {
            samples: 0,
            max_abs_delta_c: 0.0,
            mean_abs_delta_c: 0.0,
            rmse_delta_c: 0.0,
            max_rel_delta: 0.0,
            first_delta_sample: None,
            max_delta_sample: None,
            length_match: expected.len() == observed.len(),
        };
    }

    let mut max_abs_delta: f64 = 0.0;
    let mut max_delta_sample = None;
    let mut first_delta_sample = None;
    let mut sum_abs_delta = 0.0;
    let mut sum_squared_delta = 0.0;
    let mut max_rel_delta: f64 = 0.0;
    for (index, (expected, observed)) in expected.iter().zip(observed).take(samples).enumerate() {
        let delta = (expected - observed).abs();
        let scale = expected.abs().max(observed.abs());
        let relative_delta = if scale > 0.0 { delta / scale } else { 0.0 };
        let point = DeltaPoint {
            index,
            oracle_c: *expected,
            rust_c: *observed,
            abs_delta_c: delta,
        };
        if delta > 0.0 && first_delta_sample.is_none() {
            first_delta_sample = Some(point);
        }
        if max_delta_sample.is_none() || delta > max_abs_delta {
            max_delta_sample = Some(point);
        }
        max_abs_delta = max_abs_delta.max(delta);
        max_rel_delta = max_rel_delta.max(relative_delta);
        sum_abs_delta += delta;
        sum_squared_delta += delta * delta;
    }

    DeltaSummary {
        samples,
        max_abs_delta_c: max_abs_delta,
        mean_abs_delta_c: sum_abs_delta / samples as f64,
        rmse_delta_c: (sum_squared_delta / samples as f64).sqrt(),
        max_rel_delta,
        first_delta_sample,
        max_delta_sample,
        length_match: expected.len() == observed.len(),
    }
}

fn delta_points(expected: &[f64], observed: &[f64]) -> Vec<DeltaPoint> {
    expected
        .iter()
        .zip(observed)
        .enumerate()
        .map(|(index, (expected, observed))| DeltaPoint {
            index,
            oracle_c: *expected,
            rust_c: *observed,
            abs_delta_c: (expected - observed).abs(),
        })
        .collect()
}

fn print_delta_sample(label: &str, point: Option<DeltaPoint>) {
    let Some(point) = point else {
        println!("{label}: none");
        return;
    };

    println!(
        "{label}: index {} oracle {:.6} rust {:.6} abs_delta {:.6}",
        point.index, point.oracle_c, point.rust_c, point.abs_delta_c
    );
}

fn print_heat_balance_warmup(prefix: &str, warmup: &HeatBalanceWarmupDiagnostic) {
    println!("{prefix}warmup_enabled: {}", warmup.enabled);
    println!("{prefix}warmup_days: {}", warmup.day_count);
    println!(
        "{prefix}oracle_run_period_warmup_days: {}",
        warmup
            .oracle_run_period_day_count
            .map(|days| days.to_string())
            .unwrap_or_else(|| "none".to_string())
    );
    println!(
        "{prefix}warmup_day_count_delta: {}",
        heat_balance_warmup_day_count_delta(warmup)
            .map(|delta| delta.to_string())
            .unwrap_or_else(|| "none".to_string())
    );
    println!("{prefix}warmup_timesteps: {}", warmup.timestep_count);
    println!("{prefix}warmup_hours_per_day: {}", warmup.hours_per_day);
    println!("{prefix}warmup_converged: {}", warmup.converged);
    println!(
        "{prefix}warmup_final_max_zone_temperature_delta_c: {:.12}",
        warmup.final_max_zone_temperature_delta_c
    );
}

fn heat_balance_warmup_day_count_delta(warmup: &HeatBalanceWarmupDiagnostic) -> Option<i64> {
    warmup
        .oracle_run_period_day_count
        .map(|oracle_days| i64::from(warmup.day_count) - i64::from(oracle_days))
}

fn write_zone_temperature_diagnostic_report(
    report_dir: &Path,
    diagnostic: &ZoneTemperatureDiagnostic,
    context: Option<&ZoneTemperatureReportContext>,
) -> Result<(), String> {
    std::fs::create_dir_all(report_dir)
        .map_err(|error| format!("failed to create report directory: {error}"))?;

    let summary_path = report_dir.join("compare-summary.json");
    let report_path = report_dir.join("compare-report.md");

    std::fs::write(
        &summary_path,
        render_zone_temperature_summary_json(diagnostic, context),
    )
    .map_err(|error| format!("failed to write zone-temperature summary: {error}"))?;
    std::fs::write(
        &report_path,
        render_zone_temperature_report(diagnostic, context),
    )
    .map_err(|error| format!("failed to write zone-temperature report: {error}"))?;

    Ok(())
}

fn render_zone_temperature_summary_json(
    diagnostic: &ZoneTemperatureDiagnostic,
    context: Option<&ZoneTemperatureReportContext>,
) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    match context {
        Some(context) => {
            json.push_str(&format!(
                "  \"case_id\": {},\n",
                json_string(&context.case_id)
            ));
            json.push_str(&format!(
                "  \"oracle_version\": {},\n",
                json_string(&context.oracle_version)
            ));
            json.push_str(&format!(
                "  \"output\": {},\n",
                zone_temperature_output_json(&context.output)
            ));
            json.push_str(&format!(
                "  \"report_contract\": {},\n",
                zone_temperature_report_contract_json(context.report.as_ref())
            ));
            json.push_str(&format!(
                "  \"gate\": {},\n",
                zone_temperature_gate_json(context.gate.as_ref())
            ));
        }
        None => {
            json.push_str("  \"case_id\": null,\n");
            json.push_str("  \"oracle_version\": null,\n");
            json.push_str("  \"output\": null,\n");
            json.push_str("  \"report_contract\": null,\n");
            json.push_str("  \"gate\": null,\n");
        }
    }
    json.push_str("  \"comparison_class\": \"diagnostic-only\",\n");
    json.push_str("  \"conformance_claim\": false,\n");
    json.push_str("  \"tolerance_policy\": \"none\",\n");
    json.push_str("  \"runtime_class\": \"heat-balance-state-shell\",\n");
    json.push_str(&format!(
        "  \"status\": {},\n",
        json_string(diagnostic.status)
    ));
    json.push_str(&format!(
        "  \"zone\": {},\n",
        json_string(&diagnostic.zone_name)
    ));
    json.push_str(&format!("  \"samples\": {},\n", diagnostic.samples));
    json.push_str(&format!(
        "  \"heat_balance_timesteps\": {},\n",
        diagnostic.heat_balance_timesteps
    ));
    json.push_str(&format!(
        "  \"heat_balance_run_period_timesteps\": {},\n",
        diagnostic.heat_balance_run_period_timesteps
    ));
    json.push_str(&format!(
        "  \"heat_balance_warmup\": {},\n",
        heat_balance_warmup_json(&diagnostic.heat_balance_warmup)
    ));
    json.push_str(&format!("  \"zone_count\": {},\n", diagnostic.zone_count));
    json.push_str(&format!(
        "  \"surface_count\": {},\n",
        diagnostic.surface_count
    ));
    json.push_str(&format!(
        "  \"length_match\": {},\n",
        diagnostic.delta.length_match
    ));
    json.push_str(&format!(
        "  \"max_abs_delta_c\": {},\n",
        json_number(diagnostic.delta.max_abs_delta_c)
    ));
    json.push_str(&format!(
        "  \"mean_abs_delta_c\": {},\n",
        json_number(diagnostic.delta.mean_abs_delta_c)
    ));
    json.push_str(&format!(
        "  \"rmse_delta_c\": {},\n",
        json_number(diagnostic.delta.rmse_delta_c)
    ));
    json.push_str(&format!(
        "  \"max_rel_delta\": {},\n",
        json_number(diagnostic.delta.max_rel_delta)
    ));
    json.push_str(&format!(
        "  \"first_delta_sample\": {},\n",
        delta_point_json(diagnostic.delta.first_delta_sample)
    ));
    json.push_str(&format!(
        "  \"max_delta_sample\": {},\n",
        delta_point_json(diagnostic.delta.max_delta_sample)
    ));
    json.push_str(&format!(
        "  \"oracle_first_c\": {},\n",
        json_number(diagnostic.oracle_first_c)
    ));
    json.push_str(&format!(
        "  \"rust_first_c\": {},\n",
        json_number(diagnostic.rust_first_c)
    ));
    json.push_str(&format!(
        "  \"oracle_last_c\": {},\n",
        json_number(diagnostic.oracle_last_c)
    ));
    json.push_str(&format!(
        "  \"rust_last_c\": {},\n",
        json_number(diagnostic.rust_last_c)
    ));
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"compare_report_md\": \"compare-report.md\",\n");
    json.push_str("    \"compare_summary_json\": \"compare-summary.json\"\n");
    json.push_str("  }\n");
    json.push_str("}\n");
    json
}

fn render_zone_temperature_report(
    diagnostic: &ZoneTemperatureDiagnostic,
    context: Option<&ZoneTemperatureReportContext>,
) -> String {
    let mut report = String::new();
    report.push_str("# Zone Temperature Diagnostic Report\n\n");
    if let Some(context) = context {
        report.push_str("## Manifest\n\n");
        report.push_str(&format!("case_id: {}\n", context.case_id));
        report.push_str(&format!("oracle_version: {}\n", context.oracle_version));
        report.push_str(&format!("output_key: {}\n", context.output.key));
        report.push_str(&format!("output_variable: {}\n", context.output.variable));
        report.push_str(&format!("output_frequency: {}\n", context.output.frequency));
        report.push_str(&format!("output_class: {}\n", context.output.class));
        report.push_str(&format!("output_source: {}\n", context.output.source));
        if let Some(report_contract) = &context.report {
            report.push_str(&format!("report_format: {}\n", report_contract.format));
            report.push_str(&format!("report_path: {}\n", report_contract.path));
        }
        if let Some(gate) = &context.gate {
            report.push_str(&format!("gate_script: {}\n", gate.script));
            report.push_str(&format!("gate_blocking: {}\n", gate.blocking));
        }
        report.push('\n');
    }
    report.push_str("## Diagnostic\n\n");
    report.push_str("comparison_class: diagnostic-only\n");
    report.push_str("conformance_claim: false\n");
    report.push_str("tolerance_policy: none\n");
    report.push_str("runtime_class: heat-balance-state-shell\n");
    report.push_str(&format!("status: {}\n", diagnostic.status));
    report.push_str(&format!("zone: {}\n", diagnostic.zone_name));
    report.push_str(&format!("samples: {}\n", diagnostic.samples));
    report.push_str(&format!(
        "heat_balance_timesteps: {}\n",
        diagnostic.heat_balance_timesteps
    ));
    report.push_str(&format!(
        "heat_balance_run_period_timesteps: {}\n",
        diagnostic.heat_balance_run_period_timesteps
    ));
    report.push_str(&format!(
        "warmup_enabled: {}\n",
        diagnostic.heat_balance_warmup.enabled
    ));
    report.push_str(&format!(
        "warmup_days: {}\n",
        diagnostic.heat_balance_warmup.day_count
    ));
    report.push_str(&format!(
        "oracle_run_period_warmup_days: {}\n",
        diagnostic
            .heat_balance_warmup
            .oracle_run_period_day_count
            .map(|days| days.to_string())
            .unwrap_or_else(|| "none".to_string())
    ));
    report.push_str(&format!(
        "warmup_day_count_delta: {}\n",
        heat_balance_warmup_day_count_delta(&diagnostic.heat_balance_warmup)
            .map(|delta| delta.to_string())
            .unwrap_or_else(|| "none".to_string())
    ));
    report.push_str(&format!(
        "warmup_timesteps: {}\n",
        diagnostic.heat_balance_warmup.timestep_count
    ));
    report.push_str(&format!(
        "warmup_hours_per_day: {}\n",
        diagnostic.heat_balance_warmup.hours_per_day
    ));
    report.push_str(&format!(
        "warmup_converged: {}\n",
        diagnostic.heat_balance_warmup.converged
    ));
    report.push_str(&format!(
        "warmup_final_max_zone_temperature_delta_c: {:.12}\n",
        diagnostic
            .heat_balance_warmup
            .final_max_zone_temperature_delta_c
    ));
    report.push_str(&format!("zone_count: {}\n", diagnostic.zone_count));
    report.push_str(&format!("surface_count: {}\n", diagnostic.surface_count));
    report.push_str(&format!(
        "length_match: {}\n",
        diagnostic.delta.length_match
    ));
    report.push_str(&format!(
        "max_abs_delta_c: {:.6}\n",
        diagnostic.delta.max_abs_delta_c
    ));
    report.push_str(&format!(
        "mean_abs_delta_c: {:.6}\n",
        diagnostic.delta.mean_abs_delta_c
    ));
    report.push_str(&format!(
        "rmse_delta_c: {:.6}\n",
        diagnostic.delta.rmse_delta_c
    ));
    report.push_str(&format!(
        "max_rel_delta: {:.12}\n\n",
        diagnostic.delta.max_rel_delta
    ));
    report.push_str("## Delta Samples\n\n");
    report.push_str("| sample | index | oracle_c | rust_c | abs_delta_c |\n");
    report.push_str("|---|---:|---:|---:|---:|\n");
    report_delta_row(
        &mut report,
        "first_delta_sample",
        diagnostic.delta.first_delta_sample,
    );
    report_delta_row(
        &mut report,
        "max_delta_sample",
        diagnostic.delta.max_delta_sample,
    );
    report
}

fn write_heat_balance_conformance_report(
    report_dir: &Path,
    diagnostic: &HeatBalanceConformanceDiagnostic,
    conformance: &HeatBalanceConformance<'_>,
) -> Result<(), String> {
    std::fs::create_dir_all(report_dir)
        .map_err(|error| format!("failed to create report directory: {error}"))?;

    let summary_path = report_dir.join("compare-summary.json");
    let digest_path = report_dir.join("compare-digest.json");
    let report_path = report_dir.join("compare-report.md");

    std::fs::write(
        &summary_path,
        render_heat_balance_conformance_summary_json(diagnostic, conformance),
    )
    .map_err(|error| format!("failed to write heat-balance summary: {error}"))?;
    std::fs::write(
        &digest_path,
        render_heat_balance_conformance_digest_json(diagnostic, conformance),
    )
    .map_err(|error| format!("failed to write heat-balance digest: {error}"))?;
    std::fs::write(
        &report_path,
        render_heat_balance_conformance_report(diagnostic, conformance),
    )
    .map_err(|error| format!("failed to write heat-balance report: {error}"))?;

    Ok(())
}

fn render_heat_balance_conformance_summary_json(
    diagnostic: &HeatBalanceConformanceDiagnostic,
    conformance: &HeatBalanceConformance<'_>,
) -> String {
    render_heat_balance_conformance_json(diagnostic, conformance, true)
}

fn render_heat_balance_conformance_digest_json(
    diagnostic: &HeatBalanceConformanceDiagnostic,
    conformance: &HeatBalanceConformance<'_>,
) -> String {
    render_heat_balance_conformance_json(diagnostic, conformance, false)
}

fn render_heat_balance_conformance_json(
    diagnostic: &HeatBalanceConformanceDiagnostic,
    conformance: &HeatBalanceConformance<'_>,
    include_sample_rows: bool,
) -> String {
    let context = conformance.context;
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!(
        "  \"case_id\": {},\n",
        json_string(&context.case_id)
    ));
    json.push_str(&format!(
        "  \"oracle_version\": {},\n",
        json_string(&context.oracle_version)
    ));
    json.push_str(&format!(
        "  \"output\": {},\n",
        context
            .outputs
            .first()
            .map_or_else(|| "null".to_string(), zone_temperature_output_json)
    ));
    json.push_str(&format!(
        "  \"outputs\": {},\n",
        heat_balance_outputs_json(&context.outputs)
    ));
    json.push_str(&format!(
        "  \"report_contract\": {},\n",
        zone_temperature_report_contract_json(context.report.as_ref())
    ));
    json.push_str(&format!(
        "  \"gate\": {},\n",
        zone_temperature_gate_json(context.gate.as_ref())
    ));
    json.push_str(&format!(
        "  \"comparison_class\": {},\n",
        json_string(context.comparison_class)
    ));
    json.push_str(&format!(
        "  \"conformance_claim\": {},\n",
        context.conformance_claim
    ));
    json.push_str(&format!(
        "  \"tolerance_policy\": {},\n",
        heat_balance_tolerances_json(&context.tolerances)
    ));
    json.push_str(&format!(
        "  \"tolerance_policy_label\": {},\n",
        json_string(&context.tolerance_label())
    ));
    json.push_str("  \"runtime_class\": \"heat-balance-state-shell\",\n");
    json.push_str(&format!(
        "  \"status\": {},\n",
        json_string(conformance.status)
    ));
    json.push_str(&format!(
        "  \"failure_reasons\": {},\n",
        string_array_json(&conformance.failure_reasons)
    ));
    json.push_str(&format!("  \"samples\": {},\n", diagnostic.samples));
    json.push_str(&format!(
        "  \"heat_balance_timesteps\": {},\n",
        diagnostic.heat_balance_timesteps
    ));
    json.push_str(&format!(
        "  \"heat_balance_run_period_timesteps\": {},\n",
        diagnostic.heat_balance_run_period_timesteps
    ));
    json.push_str(&format!(
        "  \"heat_balance_warmup\": {},\n",
        heat_balance_warmup_json(&diagnostic.heat_balance_warmup)
    ));
    json.push_str(&format!(
        "  \"ctf_seed\": {},\n",
        heat_balance_ctf_seed_json(&diagnostic.ctf_seed)
    ));
    json.push_str(&format!(
        "  \"zone_air_algorithm\": {},\n",
        json_string(diagnostic.zone_air_algorithm)
    ));
    json.push_str(&format!(
        "  \"surface_iteration_count\": {},\n",
        diagnostic.surface_iteration_count
    ));
    json.push_str(&format!(
        "  \"ctf_initial_history_policy\": {},\n",
        json_string(diagnostic.ctf_initial_history_policy)
    ));
    json.push_str(&format!("  \"zone_count\": {},\n", diagnostic.zone_count));
    json.push_str(&format!(
        "  \"surface_count\": {},\n",
        diagnostic.surface_count
    ));
    json.push_str(&format!(
        "  \"series_count\": {},\n",
        diagnostic.series.len()
    ));
    json.push_str(&format!(
        "  \"max_abs_delta_c\": {},\n",
        json_number(heat_balance_max_abs_delta(diagnostic))
    ));
    json.push_str(&format!(
        "  \"rmse_delta_c\": {},\n",
        json_number(heat_balance_max_rmse_delta(diagnostic))
    ));
    json.push_str(&format!(
        "  \"max_rel_delta\": {},\n",
        json_number(heat_balance_max_rel_delta(diagnostic))
    ));
    json.push_str(&format!(
        "  \"bottlenecks\": {},\n",
        heat_balance_bottlenecks_json(&diagnostic.series)
    ));
    json.push_str(&format!(
        "  \"max_sample_contexts\": {},\n",
        heat_balance_max_sample_contexts_json(&diagnostic.series)
    ));
    json.push_str(&format!(
        "  \"first_sample_bottlenecks\": {},\n",
        heat_balance_first_sample_bottlenecks_json(&diagnostic.series)
    ));
    json.push_str(&format!(
        "  \"surface_first_sample_trace\": {},\n",
        heat_balance_surface_first_sample_trace_json(&diagnostic.surface_first_sample_trace)
    ));
    json.push_str(&format!(
        "  \"ctf_component_first_samples\": {},\n",
        heat_balance_ctf_component_first_samples_json(&diagnostic.ctf_component_first_samples)
    ));
    json.push_str(&format!(
        "  \"ctf_history_first_sample_deltas\": {},\n",
        heat_balance_ctf_history_first_sample_deltas_json(
            &diagnostic.ctf_history_first_sample_deltas
        )
    ));
    json.push_str(&format!(
        "  \"ctf_history_series_deltas\": {},\n",
        heat_balance_ctf_history_series_deltas_json(&diagnostic.ctf_history_series_deltas)
    ));
    json.push_str(&format!(
        "  \"ctf_storage_max_sample_deltas\": {},\n",
        heat_balance_ctf_storage_max_sample_deltas_json(&diagnostic.ctf_storage_max_sample_deltas)
    ));
    json.push_str(&format!(
        "  \"inside_balance_max_sample_deltas\": {},\n",
        heat_balance_inside_balance_max_sample_deltas_json(
            &diagnostic.inside_balance_max_sample_deltas
        )
    ));
    json.push_str(&format!(
        "  \"inside_solve_max_sample_deltas\": {},\n",
        heat_balance_inside_solve_max_sample_deltas_json(
            &diagnostic.inside_solve_max_sample_deltas
        )
    ));
    json.push_str(&format!(
        "  \"adiabatic_history_max_sample_deltas\": {},\n",
        heat_balance_adiabatic_history_max_sample_deltas_json(
            &diagnostic.adiabatic_history_max_sample_deltas
        )
    ));
    json.push_str(&format!(
        "  \"ctf_history_run_period_initial_slots\": {},\n",
        heat_balance_ctf_history_slots_json(&diagnostic.ctf_history_run_period_initial_slots)
    ));
    json.push_str(&format!(
        "  \"ctf_history_first_sample_slots\": {},\n",
        heat_balance_ctf_history_first_sample_slots_json(
            &diagnostic.ctf_history_first_sample_slots
        )
    ));
    let series_json = if include_sample_rows {
        heat_balance_series_json(&diagnostic.series)
    } else {
        heat_balance_series_json_with_sample_rows(&diagnostic.series, false)
    };
    json.push_str(&format!("  \"series\": {},\n", series_json));
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"compare_report_md\": \"compare-report.md\",\n");
    json.push_str("    \"compare_summary_json\": \"compare-summary.json\",\n");
    json.push_str("    \"compare_digest_json\": \"compare-digest.json\"\n");
    json.push_str("  }\n");
    json.push_str("}\n");
    json
}

fn render_heat_balance_conformance_report(
    diagnostic: &HeatBalanceConformanceDiagnostic,
    conformance: &HeatBalanceConformance<'_>,
) -> String {
    let context = conformance.context;
    let mut report = String::new();
    if context.conformance_claim {
        report.push_str("# Heat Balance Conformance Report\n\n");
    } else {
        report.push_str("# Heat Balance Diagnostic Report\n\n");
    }
    report.push_str("## Manifest\n\n");
    report.push_str(&format!("case_id: {}\n", context.case_id));
    report.push_str(&format!("oracle_version: {}\n", context.oracle_version));
    report.push_str(&format!("outputs: {}\n", context.outputs.len()));
    for output in &context.outputs {
        report.push_str(&format!(
            "output: {} / {} / {} / {} / {}\n",
            output.key, output.variable, output.frequency, output.class, output.source
        ));
    }
    report.push_str(&format!(
        "tolerance_policy: {}\n",
        context.tolerance_label()
    ));
    if let Some(report_contract) = &context.report {
        report.push_str(&format!("report_format: {}\n", report_contract.format));
        report.push_str(&format!("report_path: {}\n", report_contract.path));
    }
    if let Some(gate) = &context.gate {
        report.push_str(&format!("gate_script: {}\n", gate.script));
        report.push_str(&format!("gate_blocking: {}\n", gate.blocking));
    }
    report.push('\n');

    report.push_str("## Result\n\n");
    report.push_str(&format!("comparison_class: {}\n", context.comparison_class));
    report.push_str(&format!(
        "conformance_claim: {}\n",
        context.conformance_claim
    ));
    report.push_str("runtime_class: heat-balance-state-shell\n");
    report.push_str(&format!("status: {}\n", conformance.status));
    if conformance.failure_reasons.is_empty() {
        report.push_str("failure_reasons: none\n");
    } else {
        report.push_str("failure_reasons:\n");
        for reason in &conformance.failure_reasons {
            report.push_str(&format!("- {}\n", reason));
        }
    }
    report.push_str(&format!("samples: {}\n", diagnostic.samples));
    report.push_str(&format!(
        "heat_balance_timesteps: {}\n",
        diagnostic.heat_balance_timesteps
    ));
    report.push_str(&format!(
        "heat_balance_run_period_timesteps: {}\n",
        diagnostic.heat_balance_run_period_timesteps
    ));
    report.push_str(&format!(
        "warmup_enabled: {}\n",
        diagnostic.heat_balance_warmup.enabled
    ));
    report.push_str(&format!(
        "warmup_days: {}\n",
        diagnostic.heat_balance_warmup.day_count
    ));
    report.push_str(&format!(
        "oracle_run_period_warmup_days: {}\n",
        diagnostic
            .heat_balance_warmup
            .oracle_run_period_day_count
            .map(|days| days.to_string())
            .unwrap_or_else(|| "none".to_string())
    ));
    report.push_str(&format!(
        "warmup_day_count_delta: {}\n",
        heat_balance_warmup_day_count_delta(&diagnostic.heat_balance_warmup)
            .map(|delta| delta.to_string())
            .unwrap_or_else(|| "none".to_string())
    ));
    report.push_str(&format!(
        "warmup_timesteps: {}\n",
        diagnostic.heat_balance_warmup.timestep_count
    ));
    report.push_str(&format!(
        "warmup_hours_per_day: {}\n",
        diagnostic.heat_balance_warmup.hours_per_day
    ));
    report.push_str(&format!(
        "warmup_converged: {}\n",
        diagnostic.heat_balance_warmup.converged
    ));
    report.push_str(&format!(
        "warmup_final_max_zone_temperature_delta_c: {:.12}\n",
        diagnostic
            .heat_balance_warmup
            .final_max_zone_temperature_delta_c
    ));
    report.push_str(&format!(
        "ctf_seed_policy: {}\n",
        diagnostic.ctf_seed.policy
    ));
    report.push_str(&format!(
        "ctf_seed_included_constructions: {}\n",
        heat_balance_ctf_seed_included_label(&diagnostic.ctf_seed)
    ));
    report.push_str(&format!(
        "ctf_seed_skipped_constructions: {}\n",
        heat_balance_ctf_seed_skipped_label(&diagnostic.ctf_seed)
    ));
    report.push_str(&format!(
        "ctf_seed_construction_summaries: {}\n",
        heat_balance_ctf_seed_construction_summary_label(&diagnostic.ctf_seed)
    ));
    report.push_str(&format!(
        "ctf_seed_included_coefficients: {}\n",
        diagnostic.ctf_seed.included_coefficients
    ));
    report.push_str(&format!(
        "ctf_seed_skipped_coefficients: {}\n",
        diagnostic.ctf_seed.skipped_coefficients
    ));
    report.push_str(&format!(
        "zone_air_algorithm: {}\n",
        diagnostic.zone_air_algorithm
    ));
    report.push_str(&format!(
        "surface_iteration_count: {}\n",
        diagnostic.surface_iteration_count
    ));
    report.push_str(&format!(
        "ctf_initial_history_policy: {}\n",
        diagnostic.ctf_initial_history_policy
    ));
    report.push_str(&format!("zone_count: {}\n", diagnostic.zone_count));
    report.push_str(&format!("surface_count: {}\n", diagnostic.surface_count));
    report.push_str(&format!(
        "max_abs_delta_c: {:.12}\n",
        heat_balance_max_abs_delta(diagnostic)
    ));
    report.push_str(&format!(
        "rmse_delta_c: {:.12}\n",
        heat_balance_max_rmse_delta(diagnostic)
    ));
    report.push_str(&format!(
        "max_rel_delta: {:.12}\n\n",
        heat_balance_max_rel_delta(diagnostic)
    ));

    report.push_str("## Bottlenecks\n\n");
    heat_balance_report_bottleneck_rows(&mut report, &diagnostic.series);
    report.push('\n');

    report.push_str("## Max-Sample Contexts\n\n");
    heat_balance_report_max_sample_context_rows(&mut report, &diagnostic.series);
    report.push('\n');

    report.push_str("## First-Sample Bottlenecks\n\n");
    heat_balance_report_first_sample_bottleneck_rows(&mut report, &diagnostic.series);
    report.push('\n');

    report.push_str("## Rust Surface First-Sample Trace\n\n");
    heat_balance_report_surface_first_sample_trace_rows(
        &mut report,
        &diagnostic.surface_first_sample_trace,
    );
    report.push('\n');

    report.push_str("## Rust CTF First-Sample Components\n\n");
    heat_balance_report_ctf_component_first_sample_rows(
        &mut report,
        &diagnostic.ctf_component_first_samples,
    );
    report.push('\n');

    report.push_str("## CTF History First-Sample Deltas\n\n");
    heat_balance_report_ctf_history_first_sample_delta_rows(
        &mut report,
        &diagnostic.ctf_history_first_sample_deltas,
    );
    report.push('\n');

    report.push_str("## CTF History Series Deltas\n\n");
    heat_balance_report_ctf_history_series_delta_rows(
        &mut report,
        &diagnostic.ctf_history_series_deltas,
    );
    report.push('\n');

    report.push_str("## CTF Storage Max-Sample Deltas\n\n");
    heat_balance_report_ctf_storage_max_sample_delta_rows(
        &mut report,
        &diagnostic.ctf_storage_max_sample_deltas,
    );
    report.push('\n');

    report.push_str("## Inside Balance Max-Sample Deltas\n\n");
    heat_balance_report_inside_balance_max_sample_delta_rows(
        &mut report,
        &diagnostic.inside_balance_max_sample_deltas,
    );
    report.push('\n');

    report.push_str("## Inside Solve Max-Sample Deltas\n\n");
    heat_balance_report_inside_solve_max_sample_delta_rows(
        &mut report,
        &diagnostic.inside_solve_max_sample_deltas,
    );
    report.push('\n');

    report.push_str("## Adiabatic History Max-Sample Deltas\n\n");
    heat_balance_report_adiabatic_history_max_sample_delta_rows(
        &mut report,
        &diagnostic.adiabatic_history_max_sample_deltas,
    );
    report.push('\n');

    report.push_str("## Rust CTF History Run-Period Initial Slots\n\n");
    heat_balance_report_ctf_history_slot_rows(
        &mut report,
        &diagnostic.ctf_history_run_period_initial_slots,
    );
    report.push('\n');

    report.push_str("## Rust CTF History First-Sample Slots\n\n");
    heat_balance_report_ctf_history_first_sample_slot_rows(
        &mut report,
        &diagnostic.ctf_history_first_sample_slots,
    );
    report.push('\n');

    report.push_str("## Series\n\n");
    heat_balance_report_series_rows(&mut report, &diagnostic.series);
    report.push('\n');

    report.push_str("## Delta Samples\n\n");
    heat_balance_report_delta_rows(&mut report, &diagnostic.series);
    report.push('\n');

    report.push_str("## Hourly Samples\n\n");
    heat_balance_report_sample_rows(&mut report, &diagnostic.series);
    report
}

fn report_delta_row(report: &mut String, label: &str, point: Option<DeltaPoint>) {
    match point {
        Some(point) => report.push_str(&format!(
            "| {label} | {} | {:.6} | {:.6} | {:.6} |\n",
            point.index, point.oracle_c, point.rust_c, point.abs_delta_c
        )),
        None => {
            report.push_str("| ");
            report.push_str(label);
            report.push_str(" | n/a | n/a | n/a | n/a |\n");
        }
    }
}

fn zone_temperature_output_json(output: &ZoneTemperatureReportOutput) -> String {
    format!(
        "{{ \"key\": {}, \"variable\": {}, \"frequency\": {}, \"class\": {}, \"source\": {} }}",
        json_string(&output.key),
        json_string(&output.variable),
        json_string(output.frequency),
        json_string(output.class),
        json_string(output.source)
    )
}

fn zone_temperature_report_contract_json(report: Option<&ZoneTemperatureReportContract>) -> String {
    match report {
        Some(report) => format!(
            "{{ \"format\": {}, \"path\": {} }}",
            json_string(report.format),
            json_string(&report.path)
        ),
        None => "null".to_string(),
    }
}

fn zone_temperature_gate_json(gate: Option<&ZoneTemperatureGateContract>) -> String {
    match gate {
        Some(gate) => format!(
            "{{ \"script\": {}, \"blocking\": {} }}",
            json_string(&gate.script),
            gate.blocking
        ),
        None => "null".to_string(),
    }
}

fn heat_balance_warmup_json(warmup: &HeatBalanceWarmupDiagnostic) -> String {
    format!(
        concat!(
            "{{ \"enabled\": {}, \"day_count\": {}, \"timestep_count\": {}, ",
            "\"hours_per_day\": {}, \"converged\": {}, ",
            "\"final_max_zone_temperature_delta_c\": {}, ",
            "\"oracle_run_period_day_count\": {}, \"day_count_delta\": {} }}"
        ),
        warmup.enabled,
        warmup.day_count,
        warmup.timestep_count,
        warmup.hours_per_day,
        warmup.converged,
        json_number(warmup.final_max_zone_temperature_delta_c),
        json_optional_u32(warmup.oracle_run_period_day_count),
        json_optional_i64(heat_balance_warmup_day_count_delta(warmup))
    )
}

fn heat_balance_ctf_seed_json(ctf_seed: &HeatBalanceCtfSeedDiagnostic) -> String {
    format!(
        concat!(
            "{{ \"policy\": {}, \"included_constructions\": {}, ",
            "\"skipped_constructions\": {}, \"construction_summaries\": {}, ",
            "\"included_coefficients\": {}, \"skipped_coefficients\": {} }}"
        ),
        json_string(ctf_seed.policy),
        string_array_json(&ctf_seed.included_constructions),
        heat_balance_skipped_ctf_constructions_json(&ctf_seed.skipped_constructions),
        heat_balance_ctf_construction_summaries_json(&ctf_seed.construction_summaries),
        ctf_seed.included_coefficients,
        ctf_seed.skipped_coefficients
    )
}

fn heat_balance_skipped_ctf_constructions_json(
    skipped_constructions: &[HeatBalanceSkippedCtfConstruction],
) -> String {
    let mut json = String::from("[");
    for (index, construction) in skipped_constructions.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"construction_name\": {}, \"ctf_count\": {}, ",
                "\"timestep_hours\": {} }}"
            ),
            json_string(&construction.construction_name),
            construction.ctf_count,
            json_number(construction.timestep_hours)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_ctf_construction_summaries_json(
    construction_summaries: &[HeatBalanceCtfConstructionSummary],
) -> String {
    let mut json = String::from("[");
    for (index, construction) in construction_summaries.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"construction_name\": {}, \"ctf_count\": {}, ",
                "\"timestep_hours\": {}, \"included\": {} }}"
            ),
            json_string(&construction.construction_name),
            construction.ctf_count,
            json_number(construction.timestep_hours),
            construction.included
        ));
    }
    json.push(']');
    json
}

fn heat_balance_ctf_seed_included_label(ctf_seed: &HeatBalanceCtfSeedDiagnostic) -> String {
    if ctf_seed.included_constructions.is_empty() {
        "none".to_string()
    } else {
        ctf_seed.included_constructions.join(", ")
    }
}

fn heat_balance_ctf_seed_skipped_label(ctf_seed: &HeatBalanceCtfSeedDiagnostic) -> String {
    if ctf_seed.skipped_constructions.is_empty() {
        "none".to_string()
    } else {
        ctf_seed
            .skipped_constructions
            .iter()
            .map(|construction| {
                format!(
                    "{} (#CTFs={}) @ dt={:.3}h",
                    construction.construction_name,
                    construction.ctf_count,
                    construction.timestep_hours
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn heat_balance_ctf_seed_construction_summary_label(
    ctf_seed: &HeatBalanceCtfSeedDiagnostic,
) -> String {
    if ctf_seed.construction_summaries.is_empty() {
        "none".to_string()
    } else {
        ctf_seed
            .construction_summaries
            .iter()
            .map(|construction| {
                let status = if construction.included {
                    "included"
                } else {
                    "skipped"
                };
                format!(
                    "{} (#CTFs={}) @ dt={:.3}h [{}]",
                    construction.construction_name,
                    construction.ctf_count,
                    construction.timestep_hours,
                    status
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn heat_balance_tolerance_json(tolerance: HeatBalanceToleranceReport) -> String {
    format!(
        "{{ \"variable_class\": {}, \"max_abs_c\": {}, \"max_rmse_c\": {}, \"max_rel\": {} }}",
        json_string(tolerance.variable_class_label),
        json_optional_number(tolerance.max_abs_c),
        json_optional_number(tolerance.max_rmse_c),
        json_optional_number(tolerance.max_rel)
    )
}

fn heat_balance_tolerances_json(tolerances: &[HeatBalanceToleranceReport]) -> String {
    let mut json = String::from("[");
    for (index, tolerance) in tolerances.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&heat_balance_tolerance_json(*tolerance));
    }
    json.push(']');
    json
}

fn heat_balance_outputs_json(outputs: &[ZoneTemperatureReportOutput]) -> String {
    let mut json = String::from("[");
    for (index, output) in outputs.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&zone_temperature_output_json(output));
    }
    json.push(']');
    json
}

fn heat_balance_bottleneck_rows(
    series: &[HeatBalanceSeriesDiagnostic],
) -> Vec<&HeatBalanceSeriesDiagnostic> {
    let mut rows = series.iter().collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        right
            .delta
            .rmse_delta_c
            .total_cmp(&left.delta.rmse_delta_c)
            .then_with(|| {
                right
                    .delta
                    .max_abs_delta_c
                    .total_cmp(&left.delta.max_abs_delta_c)
            })
            .then_with(|| left.output.key.cmp(&right.output.key))
            .then_with(|| left.output.variable.cmp(&right.output.variable))
    });
    rows.truncate(HEAT_BALANCE_BOTTLENECK_LIMIT);
    rows
}

fn heat_balance_max_sample_contexts(
    series: &[HeatBalanceSeriesDiagnostic],
) -> Vec<HeatBalanceMaxSampleContext> {
    let mut seen_sample_indices = BTreeSet::new();
    heat_balance_bottleneck_rows(series)
        .into_iter()
        .enumerate()
        .filter_map(|(rank_index, bottleneck)| {
            let sample_index = bottleneck.delta.max_delta_sample?.index;
            if !seen_sample_indices.insert(sample_index) {
                return None;
            }
            let mut rows = series
                .iter()
                .filter_map(|row| {
                    heat_balance_sample_point(row, sample_index).map(|point| {
                        HeatBalanceMaxSampleContextRow {
                            output: row.output.clone(),
                            oracle_c: point.oracle_c,
                            rust_c: point.rust_c,
                            abs_delta_c: point.abs_delta_c,
                            series_rmse_delta_c: row.delta.rmse_delta_c,
                        }
                    })
                })
                .collect::<Vec<_>>();
            rows.sort_by(|left, right| {
                right
                    .abs_delta_c
                    .total_cmp(&left.abs_delta_c)
                    .then_with(|| {
                        right
                            .series_rmse_delta_c
                            .total_cmp(&left.series_rmse_delta_c)
                    })
                    .then_with(|| left.output.key.cmp(&right.output.key))
                    .then_with(|| left.output.variable.cmp(&right.output.variable))
            });
            rows.truncate(HEAT_BALANCE_MAX_SAMPLE_CONTEXT_LIMIT);
            Some(HeatBalanceMaxSampleContext {
                trigger_rank: rank_index + 1,
                trigger_output: bottleneck.output.clone(),
                sample_index,
                rows,
            })
        })
        .collect()
}

fn heat_balance_sample_point(
    row: &HeatBalanceSeriesDiagnostic,
    sample_index: usize,
) -> Option<DeltaPoint> {
    row.sample_rows
        .get(sample_index)
        .copied()
        .filter(|point| point.index == sample_index)
        .or_else(|| {
            row.sample_rows
                .iter()
                .copied()
                .find(|point| point.index == sample_index)
        })
}

fn heat_balance_first_sample_bottleneck_rows(
    series: &[HeatBalanceSeriesDiagnostic],
) -> Vec<&HeatBalanceSeriesDiagnostic> {
    let mut rows = series
        .iter()
        .filter(|row| row.samples > 0 && heat_balance_first_sample_delta(row).abs_delta_c > 0.0)
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        let left_delta = heat_balance_first_sample_delta(left).abs_delta_c;
        let right_delta = heat_balance_first_sample_delta(right).abs_delta_c;
        right_delta
            .total_cmp(&left_delta)
            .then_with(|| right.delta.rmse_delta_c.total_cmp(&left.delta.rmse_delta_c))
            .then_with(|| left.output.key.cmp(&right.output.key))
            .then_with(|| left.output.variable.cmp(&right.output.variable))
    });
    rows.truncate(HEAT_BALANCE_BOTTLENECK_LIMIT);
    rows
}

fn heat_balance_first_sample_delta(row: &HeatBalanceSeriesDiagnostic) -> DeltaPoint {
    DeltaPoint {
        index: 0,
        oracle_c: row.oracle_first_c,
        rust_c: row.rust_first_c,
        abs_delta_c: (row.oracle_first_c - row.rust_first_c).abs(),
    }
}

fn heat_balance_bottleneck_rows_json(rows: &[&HeatBalanceSeriesDiagnostic]) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"rank\": {}, \"output\": {}, \"status\": {}, ",
                "\"max_abs_delta_c\": {}, \"mean_abs_delta_c\": {}, ",
                "\"rmse_delta_c\": {}, \"max_rel_delta\": {}, ",
                "\"first_delta_sample\": {}, \"max_delta_sample\": {} }}"
            ),
            index + 1,
            zone_temperature_output_json(&row.output),
            json_string(row.status),
            json_number(row.delta.max_abs_delta_c),
            json_number(row.delta.mean_abs_delta_c),
            json_number(row.delta.rmse_delta_c),
            json_number(row.delta.max_rel_delta),
            delta_point_json(row.delta.first_delta_sample),
            delta_point_json(row.delta.max_delta_sample)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_bottlenecks_json(series: &[HeatBalanceSeriesDiagnostic]) -> String {
    heat_balance_bottleneck_rows_json(&heat_balance_bottleneck_rows(series))
}

fn heat_balance_max_sample_contexts_json(series: &[HeatBalanceSeriesDiagnostic]) -> String {
    let contexts = heat_balance_max_sample_contexts(series);
    let mut json = String::from("[");
    for (index, context) in contexts.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"trigger_rank\": {}, ",
                "\"trigger_output\": {}, ",
                "\"sample_index\": {}, ",
                "\"rows\": {} }}"
            ),
            context.trigger_rank,
            zone_temperature_output_json(&context.trigger_output),
            context.sample_index,
            heat_balance_max_sample_context_rows_json(&context.rows)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_max_sample_context_rows_json(rows: &[HeatBalanceMaxSampleContextRow]) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"output\": {}, ",
                "\"oracle_c\": {}, ",
                "\"rust_c\": {}, ",
                "\"abs_delta_c\": {}, ",
                "\"series_rmse_delta_c\": {} }}"
            ),
            zone_temperature_output_json(&row.output),
            json_number(row.oracle_c),
            json_number(row.rust_c),
            json_number(row.abs_delta_c),
            json_number(row.series_rmse_delta_c)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_first_sample_bottlenecks_json(series: &[HeatBalanceSeriesDiagnostic]) -> String {
    let rows = heat_balance_first_sample_bottleneck_rows(series);
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"rank\": {}, \"output\": {}, \"status\": {}, ",
                "\"first_sample_delta\": {}, ",
                "\"max_abs_delta_c\": {}, \"mean_abs_delta_c\": {}, ",
                "\"rmse_delta_c\": {}, \"max_rel_delta\": {}, ",
                "\"first_delta_sample\": {}, \"max_delta_sample\": {} }}"
            ),
            index + 1,
            zone_temperature_output_json(&row.output),
            json_string(row.status),
            delta_point_json(Some(heat_balance_first_sample_delta(row))),
            json_number(row.delta.max_abs_delta_c),
            json_number(row.delta.mean_abs_delta_c),
            json_number(row.delta.rmse_delta_c),
            json_number(row.delta.max_rel_delta),
            delta_point_json(row.delta.first_delta_sample),
            delta_point_json(row.delta.max_delta_sample)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_surface_first_sample_trace_json(
    rows: &[HeatBalanceSurfaceFirstSampleTrace],
) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"construction_name\": {}, ",
                "\"timestep_index\": {}, ",
                "\"outdoor_dry_bulb_c\": {}, ",
                "\"zone_mean_air_temperature_c\": {}, ",
                "\"inside_face_temperature_c\": {}, ",
                "\"outside_face_temperature_c\": {}, ",
                "\"inside_convection_heat_gain_rate_w\": {}, ",
                "\"inside_net_surface_thermal_radiation_heat_gain_rate_w\": {}, ",
                "\"inside_conduction_rate_w\": {}, ",
                "\"outside_conduction_rate_w\": {}, ",
                "\"heat_storage_rate_w\": {}, ",
                "\"outside_convection_heat_gain_rate_w\": {}, ",
                "\"outside_net_thermal_radiation_heat_gain_rate_w\": {}, ",
                "\"outside_solar_radiation_heat_gain_rate_w\": {} }}"
            ),
            json_string(&row.surface_name),
            json_string(&row.construction_name),
            row.timestep_index,
            json_number(row.outdoor_dry_bulb_c),
            json_number(row.zone_mean_air_temperature_c),
            json_number(row.inside_face_temperature_c),
            json_number(row.outside_face_temperature_c),
            json_number(row.inside_convection_heat_gain_rate_w),
            json_number(row.inside_net_surface_thermal_radiation_heat_gain_rate_w),
            json_number(row.inside_conduction_rate_w),
            json_number(row.outside_conduction_rate_w),
            json_number(row.heat_storage_rate_w),
            json_number(row.outside_convection_heat_gain_rate_w),
            json_number(row.outside_net_thermal_radiation_heat_gain_rate_w),
            json_number(row.outside_solar_radiation_heat_gain_rate_w)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_ctf_component_first_samples_json(
    rows: &[HeatBalanceCtfComponentFirstSample],
) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"inside_conduction_rate_w\": {}, ",
                "\"inside_current_outside_term_w\": {}, ",
                "\"inside_current_inside_term_w\": {}, ",
                "\"inside_history_term_w\": {}, ",
                "\"outside_conduction_rate_w\": {}, ",
                "\"outside_current_outside_term_w\": {}, ",
                "\"outside_current_inside_term_w\": {}, ",
                "\"outside_history_term_w\": {}, ",
                "\"heat_storage_rate_w\": {} }}"
            ),
            json_string(&row.key),
            json_number(row.inside_conduction_rate_w),
            json_number(row.inside_current_outside_term_w),
            json_number(row.inside_current_inside_term_w),
            json_number(row.inside_history_term_w),
            json_number(row.outside_conduction_rate_w),
            json_number(row.outside_current_outside_term_w),
            json_number(row.outside_current_inside_term_w),
            json_number(row.outside_history_term_w),
            json_number(row.heat_storage_rate_w)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_ctf_history_first_sample_deltas_json(
    rows: &[HeatBalanceCtfHistoryFirstSampleDelta],
) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"construction_name\": {}, ",
                "\"area_m2\": {}, ",
                "\"ctf_outside_0_w_per_m2_k\": {}, ",
                "\"ctf_cross_0_w_per_m2_k\": {}, ",
                "\"ctf_inside_0_w_per_m2_k\": {}, ",
                "\"oracle_outside_face_temperature_c\": {}, ",
                "\"rust_outside_face_temperature_c\": {}, ",
                "\"outside_face_temperature_delta_c\": {}, ",
                "\"oracle_inside_face_temperature_c\": {}, ",
                "\"rust_inside_face_temperature_c\": {}, ",
                "\"inside_face_temperature_delta_c\": {}, ",
                "\"oracle_inside_current_term_w\": {}, ",
                "\"rust_inside_current_term_w\": {}, ",
                "\"inside_current_delta_w\": {}, ",
                "\"oracle_inside_history_term_w\": {}, ",
                "\"rust_inside_history_term_w\": {}, ",
                "\"inside_history_delta_w\": {}, ",
                "\"oracle_outside_current_term_w\": {}, ",
                "\"rust_outside_current_term_w\": {}, ",
                "\"outside_current_delta_w\": {}, ",
                "\"oracle_outside_history_term_w\": {}, ",
                "\"rust_outside_history_term_w\": {}, ",
                "\"outside_history_delta_w\": {} }}"
            ),
            json_string(&row.key),
            json_string(&row.construction_name),
            json_number(row.area_m2),
            json_number(row.ctf_outside_0_w_per_m2_k),
            json_number(row.ctf_cross_0_w_per_m2_k),
            json_number(row.ctf_inside_0_w_per_m2_k),
            json_number(row.oracle_outside_face_temperature_c),
            json_number(row.rust_outside_face_temperature_c),
            json_number(row.outside_face_temperature_delta_c),
            json_number(row.oracle_inside_face_temperature_c),
            json_number(row.rust_inside_face_temperature_c),
            json_number(row.inside_face_temperature_delta_c),
            json_number(row.oracle_inside_current_term_w),
            json_number(row.rust_inside_current_term_w),
            json_number(row.inside_current_delta_w),
            json_number(row.oracle_inside_history_term_w),
            json_number(row.rust_inside_history_term_w),
            json_number(row.inside_history_delta_w),
            json_number(row.oracle_outside_current_term_w),
            json_number(row.rust_outside_current_term_w),
            json_number(row.outside_current_delta_w),
            json_number(row.oracle_outside_history_term_w),
            json_number(row.rust_outside_history_term_w),
            json_number(row.outside_history_delta_w)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_ctf_history_series_deltas_json(
    rows: &[HeatBalanceCtfHistorySeriesDelta],
) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"construction_name\": {}, ",
                "\"area_m2\": {}, ",
                "\"samples\": {}, ",
                "\"inside_current_delta\": {}, ",
                "\"inside_history_delta\": {}, ",
                "\"outside_current_delta\": {}, ",
                "\"outside_history_delta\": {} }}"
            ),
            json_string(&row.key),
            json_string(&row.construction_name),
            json_number(row.area_m2),
            row.samples,
            delta_summary_json(&row.inside_current_delta),
            delta_summary_json(&row.inside_history_delta),
            delta_summary_json(&row.outside_current_delta),
            delta_summary_json(&row.outside_history_delta)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_ctf_storage_max_sample_deltas_json(
    rows: &[HeatBalanceCtfStorageMaxSampleDelta],
) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"construction_name\": {}, ",
                "\"sample_index\": {}, ",
                "\"area_m2\": {}, ",
                "\"oracle_inside_conduction_w\": {}, ",
                "\"rust_inside_conduction_w\": {}, ",
                "\"inside_conduction_delta_w\": {}, ",
                "\"oracle_outside_conduction_w\": {}, ",
                "\"rust_outside_conduction_w\": {}, ",
                "\"outside_conduction_delta_w\": {}, ",
                "\"oracle_storage_w\": {}, ",
                "\"rust_storage_w\": {}, ",
                "\"storage_delta_w\": {}, ",
                "\"oracle_inside_current_term_w\": {}, ",
                "\"rust_inside_current_term_w\": {}, ",
                "\"inside_current_delta_w\": {}, ",
                "\"oracle_inside_history_term_w\": {}, ",
                "\"rust_inside_history_term_w\": {}, ",
                "\"inside_history_delta_w\": {}, ",
                "\"oracle_outside_current_term_w\": {}, ",
                "\"rust_outside_current_term_w\": {}, ",
                "\"outside_current_delta_w\": {}, ",
                "\"oracle_outside_history_term_w\": {}, ",
                "\"rust_outside_history_term_w\": {}, ",
                "\"outside_history_delta_w\": {} }}"
            ),
            json_string(&row.key),
            json_string(&row.construction_name),
            row.sample_index,
            json_number(row.area_m2),
            json_number(row.oracle_inside_conduction_w),
            json_number(row.rust_inside_conduction_w),
            json_number(row.inside_conduction_delta_w),
            json_number(row.oracle_outside_conduction_w),
            json_number(row.rust_outside_conduction_w),
            json_number(row.outside_conduction_delta_w),
            json_number(row.oracle_storage_w),
            json_number(row.rust_storage_w),
            json_number(row.storage_delta_w),
            json_number(row.oracle_inside_current_term_w),
            json_number(row.rust_inside_current_term_w),
            json_number(row.inside_current_delta_w),
            json_number(row.oracle_inside_history_term_w),
            json_number(row.rust_inside_history_term_w),
            json_number(row.inside_history_delta_w),
            json_number(row.oracle_outside_current_term_w),
            json_number(row.rust_outside_current_term_w),
            json_number(row.outside_current_delta_w),
            json_number(row.oracle_outside_history_term_w),
            json_number(row.rust_outside_history_term_w),
            json_number(row.outside_history_delta_w)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_inside_balance_max_sample_deltas_json(
    rows: &[HeatBalanceInsideBalanceMaxSampleDelta],
) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"construction_name\": {}, ",
                "\"sample_index\": {}, ",
                "\"area_m2\": {}, ",
                "\"oracle_inside_face_temperature_c\": {}, ",
                "\"rust_inside_face_temperature_c\": {}, ",
                "\"inside_face_temperature_delta_c\": {}, ",
                "\"oracle_inside_convection_coefficient_w_per_m2_k\": {}, ",
                "\"rust_inside_convection_coefficient_w_per_m2_k\": {}, ",
                "\"inside_convection_coefficient_delta_w_per_m2_k\": {}, ",
                "\"oracle_inside_conduction_w\": {}, ",
                "\"rust_inside_conduction_w\": {}, ",
                "\"inside_conduction_delta_w\": {}, ",
                "\"oracle_inside_convection_w\": {}, ",
                "\"rust_inside_convection_w\": {}, ",
                "\"inside_convection_delta_w\": {}, ",
                "\"oracle_inside_net_longwave_w\": {}, ",
                "\"rust_inside_net_longwave_w\": {}, ",
                "\"inside_net_longwave_delta_w\": {}, ",
                "\"oracle_inside_balance_residual_w\": {}, ",
                "\"rust_inside_balance_residual_w\": {}, ",
                "\"inside_balance_residual_delta_w\": {} }}"
            ),
            json_string(&row.key),
            json_string(&row.construction_name),
            row.sample_index,
            json_number(row.area_m2),
            json_number(row.oracle_inside_face_temperature_c),
            json_number(row.rust_inside_face_temperature_c),
            json_number(row.inside_face_temperature_delta_c),
            json_number(row.oracle_inside_convection_coefficient_w_per_m2_k),
            json_number(row.rust_inside_convection_coefficient_w_per_m2_k),
            json_number(row.inside_convection_coefficient_delta_w_per_m2_k),
            json_number(row.oracle_inside_conduction_w),
            json_number(row.rust_inside_conduction_w),
            json_number(row.inside_conduction_delta_w),
            json_number(row.oracle_inside_convection_w),
            json_number(row.rust_inside_convection_w),
            json_number(row.inside_convection_delta_w),
            json_number(row.oracle_inside_net_longwave_w),
            json_number(row.rust_inside_net_longwave_w),
            json_number(row.inside_net_longwave_delta_w),
            json_number(row.oracle_inside_balance_residual_w),
            json_number(row.rust_inside_balance_residual_w),
            json_number(row.inside_balance_residual_delta_w)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_inside_solve_max_sample_deltas_json(
    rows: &[HeatBalanceInsideSolveMaxSampleDelta],
) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"construction_name\": {}, ",
                "\"outside_boundary_condition\": {}, ",
                "\"sample_index\": {}, ",
                "\"area_m2\": {}, ",
                "\"ctf_inside_0_w_per_m2_k\": {}, ",
                "\"ctf_cross_0_w_per_m2_k\": {}, ",
                "\"iter_damp_w_per_m2_k\": {}, ",
                "\"oracle_inside_face_temperature_c\": {}, ",
                "\"rust_inside_face_temperature_c\": {}, ",
                "\"inside_face_temperature_delta_c\": {}, ",
                "\"oracle_inferred_reference_air_temperature_c\": {}, ",
                "\"rust_inferred_reference_air_temperature_c\": {}, ",
                "\"inferred_reference_air_temperature_delta_c\": {}, ",
                "\"oracle_solve_denominator_w_per_m2_k\": {}, ",
                "\"rust_solve_denominator_w_per_m2_k\": {}, ",
                "\"solve_denominator_delta_w_per_m2_k\": {}, ",
                "\"oracle_implied_solve_numerator_w\": {}, ",
                "\"rust_implied_solve_numerator_w\": {}, ",
                "\"implied_solve_numerator_delta_w\": {}, ",
                "\"oracle_reference_air_source_w\": {}, ",
                "\"rust_reference_air_source_w\": {}, ",
                "\"reference_air_source_delta_w\": {}, ",
                "\"oracle_outside_temperature_source_w\": {}, ",
                "\"rust_outside_temperature_source_w\": {}, ",
                "\"outside_temperature_source_delta_w\": {}, ",
                "\"oracle_inside_history_term_w\": {}, ",
                "\"rust_inside_history_term_w\": {}, ",
                "\"inside_history_delta_w\": {}, ",
                "\"rust_inside_history_temperature_term_w\": {}, ",
                "\"rust_inside_history_flux_term_w\": {}, ",
                "\"oracle_inside_net_longwave_w\": {}, ",
                "\"rust_inside_net_longwave_w\": {}, ",
                "\"inside_net_longwave_delta_w\": {} }}"
            ),
            json_string(&row.key),
            json_string(&row.construction_name),
            json_string(&row.outside_boundary_condition),
            row.sample_index,
            json_number(row.area_m2),
            json_number(row.ctf_inside_0_w_per_m2_k),
            json_number(row.ctf_cross_0_w_per_m2_k),
            json_number(row.iter_damp_w_per_m2_k),
            json_number(row.oracle_inside_face_temperature_c),
            json_number(row.rust_inside_face_temperature_c),
            json_number(row.inside_face_temperature_delta_c),
            json_number(row.oracle_inferred_reference_air_temperature_c),
            json_number(row.rust_inferred_reference_air_temperature_c),
            json_number(row.inferred_reference_air_temperature_delta_c),
            json_number(row.oracle_solve_denominator_w_per_m2_k),
            json_number(row.rust_solve_denominator_w_per_m2_k),
            json_number(row.solve_denominator_delta_w_per_m2_k),
            json_number(row.oracle_implied_solve_numerator_w),
            json_number(row.rust_implied_solve_numerator_w),
            json_number(row.implied_solve_numerator_delta_w),
            json_number(row.oracle_reference_air_source_w),
            json_number(row.rust_reference_air_source_w),
            json_number(row.reference_air_source_delta_w),
            json_number(row.oracle_outside_temperature_source_w),
            json_number(row.rust_outside_temperature_source_w),
            json_number(row.outside_temperature_source_delta_w),
            json_number(row.oracle_inside_history_term_w),
            json_number(row.rust_inside_history_term_w),
            json_number(row.inside_history_delta_w),
            json_number(row.rust_inside_history_temperature_term_w),
            json_number(row.rust_inside_history_flux_term_w),
            json_number(row.oracle_inside_net_longwave_w),
            json_number(row.rust_inside_net_longwave_w),
            json_number(row.inside_net_longwave_delta_w)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_adiabatic_history_max_sample_deltas_json(
    rows: &[HeatBalanceAdiabaticHistoryMaxSampleDelta],
) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"construction_name\": {}, ",
                "\"sample_index\": {}, ",
                "\"area_m2\": {}, ",
                "\"ctf_inside_0_w_per_m2_k\": {}, ",
                "\"ctf_cross_0_w_per_m2_k\": {}, ",
                "\"oracle_inside_face_temperature_c\": {}, ",
                "\"rust_inside_face_temperature_c\": {}, ",
                "\"inside_face_temperature_delta_c\": {}, ",
                "\"oracle_outside_face_temperature_c\": {}, ",
                "\"rust_outside_face_temperature_c\": {}, ",
                "\"outside_face_temperature_delta_c\": {}, ",
                "\"oracle_outside_minus_inside_c\": {}, ",
                "\"rust_outside_minus_inside_c\": {}, ",
                "\"outside_minus_inside_delta_c\": {}, ",
                "\"oracle_inside_current_term_w\": {}, ",
                "\"rust_inside_current_term_w\": {}, ",
                "\"inside_current_delta_w\": {}, ",
                "\"oracle_inside_current_if_outside_synced_w\": {}, ",
                "\"rust_inside_current_if_outside_synced_w\": {}, ",
                "\"inside_current_if_outside_synced_delta_w\": {}, ",
                "\"oracle_inside_current_sync_shift_w\": {}, ",
                "\"rust_inside_current_sync_shift_w\": {}, ",
                "\"oracle_inside_history_term_w\": {}, ",
                "\"rust_inside_history_term_w\": {}, ",
                "\"inside_history_delta_w\": {}, ",
                "\"oracle_inside_history_if_outside_synced_w\": {}, ",
                "\"rust_inside_history_if_outside_synced_w\": {}, ",
                "\"inside_history_if_outside_synced_delta_w\": {} }}"
            ),
            json_string(&row.key),
            json_string(&row.construction_name),
            row.sample_index,
            json_number(row.area_m2),
            json_number(row.ctf_inside_0_w_per_m2_k),
            json_number(row.ctf_cross_0_w_per_m2_k),
            json_number(row.oracle_inside_face_temperature_c),
            json_number(row.rust_inside_face_temperature_c),
            json_number(row.inside_face_temperature_delta_c),
            json_number(row.oracle_outside_face_temperature_c),
            json_number(row.rust_outside_face_temperature_c),
            json_number(row.outside_face_temperature_delta_c),
            json_number(row.oracle_outside_minus_inside_c),
            json_number(row.rust_outside_minus_inside_c),
            json_number(row.outside_minus_inside_delta_c),
            json_number(row.oracle_inside_current_term_w),
            json_number(row.rust_inside_current_term_w),
            json_number(row.inside_current_delta_w),
            json_number(row.oracle_inside_current_if_outside_synced_w),
            json_number(row.rust_inside_current_if_outside_synced_w),
            json_number(row.inside_current_if_outside_synced_delta_w),
            json_number(row.oracle_inside_current_sync_shift_w),
            json_number(row.rust_inside_current_sync_shift_w),
            json_number(row.oracle_inside_history_term_w),
            json_number(row.rust_inside_history_term_w),
            json_number(row.inside_history_delta_w),
            json_number(row.oracle_inside_history_if_outside_synced_w),
            json_number(row.rust_inside_history_if_outside_synced_w),
            json_number(row.inside_history_if_outside_synced_delta_w)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_ctf_history_slots_json(rows: &[HeatBalanceCtfHistorySlotSample]) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"construction_name\": {}, ",
                "\"slot_index\": {}, ",
                "\"area_m2\": {}, ",
                "\"outside_history_coefficient_w_per_m2_k\": {}, ",
                "\"cross_history_coefficient_w_per_m2_k\": {}, ",
                "\"inside_history_coefficient_w_per_m2_k\": {}, ",
                "\"flux_history_coefficient\": {}, ",
                "\"outside_temperature_history_c\": {}, ",
                "\"inside_temperature_history_c\": {}, ",
                "\"outside_flux_history_w_per_m2\": {}, ",
                "\"inside_flux_history_w_per_m2\": {}, ",
                "\"inside_temperature_term_w\": {}, ",
                "\"inside_flux_term_w\": {}, ",
                "\"inside_total_term_w\": {}, ",
                "\"outside_temperature_term_w\": {}, ",
                "\"outside_flux_term_w\": {}, ",
                "\"outside_total_term_w\": {} }}"
            ),
            json_string(&row.surface_name),
            json_string(&row.construction_name),
            row.slot_index,
            json_number(row.area_m2),
            json_number(row.outside_history_coefficient_w_per_m2_k),
            json_number(row.cross_history_coefficient_w_per_m2_k),
            json_number(row.inside_history_coefficient_w_per_m2_k),
            json_number(row.flux_history_coefficient),
            json_number(row.outside_temperature_history_c),
            json_number(row.inside_temperature_history_c),
            json_number(row.outside_flux_history_w_per_m2),
            json_number(row.inside_flux_history_w_per_m2),
            json_number(row.inside_temperature_term_w),
            json_number(row.inside_flux_term_w),
            json_number(row.inside_total_term_w),
            json_number(row.outside_temperature_term_w),
            json_number(row.outside_flux_term_w),
            json_number(row.outside_total_term_w)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_ctf_history_first_sample_slots_json(
    rows: &[HeatBalanceCtfHistorySlotFirstSample],
) -> String {
    let mut json = String::from("[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&format!(
            concat!(
                "{{ \"key\": {}, ",
                "\"construction_name\": {}, ",
                "\"slot_index\": {}, ",
                "\"area_m2\": {}, ",
                "\"timestep_count\": {}, ",
                "\"outside_history_coefficient_w_per_m2_k\": {}, ",
                "\"cross_history_coefficient_w_per_m2_k\": {}, ",
                "\"inside_history_coefficient_w_per_m2_k\": {}, ",
                "\"flux_history_coefficient\": {}, ",
                "\"outside_temperature_history_c\": {}, ",
                "\"inside_temperature_history_c\": {}, ",
                "\"outside_flux_history_w_per_m2\": {}, ",
                "\"inside_flux_history_w_per_m2\": {}, ",
                "\"inside_temperature_term_w\": {}, ",
                "\"inside_flux_term_w\": {}, ",
                "\"inside_total_term_w\": {}, ",
                "\"outside_temperature_term_w\": {}, ",
                "\"outside_flux_term_w\": {}, ",
                "\"outside_total_term_w\": {} }}"
            ),
            json_string(&row.surface_name),
            json_string(&row.construction_name),
            row.slot_index,
            json_number(row.area_m2),
            row.timestep_count,
            json_number(row.outside_history_coefficient_w_per_m2_k),
            json_number(row.cross_history_coefficient_w_per_m2_k),
            json_number(row.inside_history_coefficient_w_per_m2_k),
            json_number(row.flux_history_coefficient),
            json_number(row.outside_temperature_history_c),
            json_number(row.inside_temperature_history_c),
            json_number(row.outside_flux_history_w_per_m2),
            json_number(row.inside_flux_history_w_per_m2),
            json_number(row.inside_temperature_term_w),
            json_number(row.inside_flux_term_w),
            json_number(row.inside_total_term_w),
            json_number(row.outside_temperature_term_w),
            json_number(row.outside_flux_term_w),
            json_number(row.outside_total_term_w)
        ));
    }
    json.push(']');
    json
}

fn heat_balance_series_json(series: &[HeatBalanceSeriesDiagnostic]) -> String {
    heat_balance_series_json_with_sample_rows(series, true)
}

fn heat_balance_series_json_with_sample_rows(
    series: &[HeatBalanceSeriesDiagnostic],
    include_sample_rows: bool,
) -> String {
    let mut json = String::from("[\n");
    for (index, row) in series.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"output\": {},\n",
            zone_temperature_output_json(&row.output)
        ));
        json.push_str(&format!("      \"status\": {},\n", json_string(row.status)));
        json.push_str(&format!("      \"samples\": {},\n", row.samples));
        json.push_str(&format!(
            "      \"length_match\": {},\n",
            row.delta.length_match
        ));
        json.push_str(&format!(
            "      \"max_abs_delta_c\": {},\n",
            json_number(row.delta.max_abs_delta_c)
        ));
        json.push_str(&format!(
            "      \"mean_abs_delta_c\": {},\n",
            json_number(row.delta.mean_abs_delta_c)
        ));
        json.push_str(&format!(
            "      \"rmse_delta_c\": {},\n",
            json_number(row.delta.rmse_delta_c)
        ));
        json.push_str(&format!(
            "      \"max_rel_delta\": {},\n",
            json_number(row.delta.max_rel_delta)
        ));
        json.push_str(&format!(
            "      \"first_delta_sample\": {},\n",
            delta_point_json(row.delta.first_delta_sample)
        ));
        json.push_str(&format!(
            "      \"max_delta_sample\": {},\n",
            delta_point_json(row.delta.max_delta_sample)
        ));
        if include_sample_rows {
            json.push_str(&format!(
                "      \"sample_rows\": {},\n",
                delta_points_json(&row.sample_rows)
            ));
        }
        json.push_str(&format!(
            "      \"oracle_first_c\": {},\n",
            json_number(row.oracle_first_c)
        ));
        json.push_str(&format!(
            "      \"rust_first_c\": {},\n",
            json_number(row.rust_first_c)
        ));
        json.push_str(&format!(
            "      \"oracle_last_c\": {},\n",
            json_number(row.oracle_last_c)
        ));
        json.push_str(&format!(
            "      \"rust_last_c\": {}\n",
            json_number(row.rust_last_c)
        ));
        if index + 1 == series.len() {
            json.push_str("    }\n");
        } else {
            json.push_str("    },\n");
        }
    }
    json.push_str("  ]");
    json
}

fn heat_balance_report_bottleneck_rows(
    report: &mut String,
    series: &[HeatBalanceSeriesDiagnostic],
) {
    report.push_str(
        "| rank | key | variable | class | max_abs_delta_c | mean_abs_delta_c | rmse_delta_c | status |\n",
    );
    report.push_str("|---:|---|---|---|---:|---:|---:|---|\n");
    for (index, row) in heat_balance_bottleneck_rows(series).iter().enumerate() {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {} |\n",
            index + 1,
            markdown_cell(&row.output.key),
            markdown_cell(&row.output.variable),
            row.output.class,
            row.delta.max_abs_delta_c,
            row.delta.mean_abs_delta_c,
            row.delta.rmse_delta_c,
            row.status
        ));
    }
}

fn heat_balance_report_max_sample_context_rows(
    report: &mut String,
    series: &[HeatBalanceSeriesDiagnostic],
) {
    report.push_str(
        "| trigger_rank | sample_index | trigger | key | variable | class | oracle_c | rust_c | abs_delta_c | series_rmse_delta_c |\n",
    );
    report.push_str("|---:|---:|---|---|---|---|---:|---:|---:|---:|\n");
    for context in heat_balance_max_sample_contexts(series) {
        let trigger = format!(
            "{}/{}",
            context.trigger_output.key, context.trigger_output.variable
        );
        if context.rows.is_empty() {
            report.push_str(&format!(
                "| {} | {} | {} | n/a | n/a | n/a | n/a | n/a | n/a | n/a |\n",
                context.trigger_rank,
                context.sample_index,
                markdown_cell(&trigger)
            ));
            continue;
        }
        for row in context.rows {
            report.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
                context.trigger_rank,
                context.sample_index,
                markdown_cell(&trigger),
                markdown_cell(&row.output.key),
                markdown_cell(&row.output.variable),
                row.output.class,
                row.oracle_c,
                row.rust_c,
                row.abs_delta_c,
                row.series_rmse_delta_c
            ));
        }
    }
}

fn heat_balance_report_first_sample_bottleneck_rows(
    report: &mut String,
    series: &[HeatBalanceSeriesDiagnostic],
) {
    report.push_str(
        "| rank | key | variable | class | first_abs_delta_c | first_oracle_c | first_rust_c | rmse_delta_c | status |\n",
    );
    report.push_str("|---:|---|---|---|---:|---:|---:|---:|---|\n");
    for (index, row) in heat_balance_first_sample_bottleneck_rows(series)
        .iter()
        .enumerate()
    {
        let first_delta = heat_balance_first_sample_delta(row);
        report.push_str(&format!(
            "| {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {} |\n",
            index + 1,
            markdown_cell(&row.output.key),
            markdown_cell(&row.output.variable),
            row.output.class,
            first_delta.abs_delta_c,
            first_delta.oracle_c,
            first_delta.rust_c,
            row.delta.rmse_delta_c,
            row.status
        ));
    }
}

fn heat_balance_report_surface_first_sample_trace_rows(
    report: &mut String,
    rows: &[HeatBalanceSurfaceFirstSampleTrace],
) {
    report.push_str(
        "| key | construction | timestep | outdoor_db_c | zone_mat_c | inside_temp_c | outside_temp_c | inside_conv_w | inside_lw_w | inside_cond_w | outside_cond_w | storage_w | outside_conv_w | outside_lw_w | outside_solar_w |\n",
    );
    report.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.surface_name),
            markdown_cell(&row.construction_name),
            row.timestep_index,
            row.outdoor_dry_bulb_c,
            row.zone_mean_air_temperature_c,
            row.inside_face_temperature_c,
            row.outside_face_temperature_c,
            row.inside_convection_heat_gain_rate_w,
            row.inside_net_surface_thermal_radiation_heat_gain_rate_w,
            row.inside_conduction_rate_w,
            row.outside_conduction_rate_w,
            row.heat_storage_rate_w,
            row.outside_convection_heat_gain_rate_w,
            row.outside_net_thermal_radiation_heat_gain_rate_w,
            row.outside_solar_radiation_heat_gain_rate_w
        ));
    }
}

fn heat_balance_report_ctf_component_first_sample_rows(
    report: &mut String,
    rows: &[HeatBalanceCtfComponentFirstSample],
) {
    report.push_str(
        "| key | inside_rate_w | in_x0_out_w | in_z0_in_w | in_history_w | outside_rate_w | out_x0_out_w | out_y0_in_w | out_history_w | storage_w |\n",
    );
    report.push_str("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.key),
            row.inside_conduction_rate_w,
            row.inside_current_outside_term_w,
            row.inside_current_inside_term_w,
            row.inside_history_term_w,
            row.outside_conduction_rate_w,
            row.outside_current_outside_term_w,
            row.outside_current_inside_term_w,
            row.outside_history_term_w,
            row.heat_storage_rate_w
        ));
    }
}

fn heat_balance_report_ctf_history_first_sample_delta_rows(
    report: &mut String,
    rows: &[HeatBalanceCtfHistoryFirstSampleDelta],
) {
    report.push_str(
        "| key | construction | area_m2 | ctf_x0 | ctf_y0 | ctf_z0 | oracle_out_temp_c | rust_out_temp_c | out_temp_abs_delta_c | oracle_in_temp_c | rust_in_temp_c | in_temp_abs_delta_c | oracle_in_current_w | rust_in_current_w | in_current_abs_delta_w | oracle_in_history_w | rust_in_history_w | in_history_abs_delta_w | oracle_out_current_w | rust_out_current_w | out_current_abs_delta_w | oracle_out_history_w | rust_out_history_w | out_history_abs_delta_w |\n",
    );
    report.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.construction_name),
            row.area_m2,
            row.ctf_outside_0_w_per_m2_k,
            row.ctf_cross_0_w_per_m2_k,
            row.ctf_inside_0_w_per_m2_k,
            row.oracle_outside_face_temperature_c,
            row.rust_outside_face_temperature_c,
            row.outside_face_temperature_delta_c,
            row.oracle_inside_face_temperature_c,
            row.rust_inside_face_temperature_c,
            row.inside_face_temperature_delta_c,
            row.oracle_inside_current_term_w,
            row.rust_inside_current_term_w,
            row.inside_current_delta_w,
            row.oracle_inside_history_term_w,
            row.rust_inside_history_term_w,
            row.inside_history_delta_w,
            row.oracle_outside_current_term_w,
            row.rust_outside_current_term_w,
            row.outside_current_delta_w,
            row.oracle_outside_history_term_w,
            row.rust_outside_history_term_w,
            row.outside_history_delta_w
        ));
    }
}

fn heat_balance_report_ctf_history_series_delta_rows(
    report: &mut String,
    rows: &[HeatBalanceCtfHistorySeriesDelta],
) {
    report.push_str(
        "| key | construction | samples | in_current_rmse_w | in_history_rmse_w | out_current_rmse_w | out_history_rmse_w | in_current_max_w | in_history_max_w | out_current_max_w | out_history_max_w | in_current_mean_w | in_history_mean_w | out_current_mean_w | out_history_mean_w |\n",
    );
    report.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    let mut sorted = rows.iter().collect::<Vec<_>>();
    sorted.sort_by(|left, right| {
        heat_balance_ctf_history_series_max_rmse(right)
            .total_cmp(&heat_balance_ctf_history_series_max_rmse(left))
    });
    for row in sorted {
        report.push_str(&format!(
            "| {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.construction_name),
            row.samples,
            row.inside_current_delta.rmse_delta_c,
            row.inside_history_delta.rmse_delta_c,
            row.outside_current_delta.rmse_delta_c,
            row.outside_history_delta.rmse_delta_c,
            row.inside_current_delta.max_abs_delta_c,
            row.inside_history_delta.max_abs_delta_c,
            row.outside_current_delta.max_abs_delta_c,
            row.outside_history_delta.max_abs_delta_c,
            row.inside_current_delta.mean_abs_delta_c,
            row.inside_history_delta.mean_abs_delta_c,
            row.outside_current_delta.mean_abs_delta_c,
            row.outside_history_delta.mean_abs_delta_c
        ));
    }
}

fn heat_balance_report_ctf_storage_max_sample_delta_rows(
    report: &mut String,
    rows: &[HeatBalanceCtfStorageMaxSampleDelta],
) {
    report.push_str(
        "| key | construction | sample_index | storage_delta_w | in_cond_delta_w | out_cond_delta_w | in_current_delta_w | in_history_delta_w | out_current_delta_w | out_history_delta_w | oracle_storage_w | rust_storage_w | oracle_in_current_w | rust_in_current_w | oracle_in_history_w | rust_in_history_w | oracle_out_current_w | rust_out_current_w | oracle_out_history_w | rust_out_history_w |\n",
    );
    report.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.construction_name),
            row.sample_index,
            row.storage_delta_w,
            row.inside_conduction_delta_w,
            row.outside_conduction_delta_w,
            row.inside_current_delta_w,
            row.inside_history_delta_w,
            row.outside_current_delta_w,
            row.outside_history_delta_w,
            row.oracle_storage_w,
            row.rust_storage_w,
            row.oracle_inside_current_term_w,
            row.rust_inside_current_term_w,
            row.oracle_inside_history_term_w,
            row.rust_inside_history_term_w,
            row.oracle_outside_current_term_w,
            row.rust_outside_current_term_w,
            row.oracle_outside_history_term_w,
            row.rust_outside_history_term_w
        ));
    }
}

fn heat_balance_report_inside_balance_max_sample_delta_rows(
    report: &mut String,
    rows: &[HeatBalanceInsideBalanceMaxSampleDelta],
) {
    report.push_str(
        "| key | construction | sample_index | residual_delta_w | cond_delta_w | conv_delta_w | net_lw_delta_w | hconv_delta_w_per_m2_k | in_temp_delta_c | oracle_residual_w | rust_residual_w | oracle_cond_w | rust_cond_w | oracle_conv_w | rust_conv_w | oracle_net_lw_w | rust_net_lw_w |\n",
    );
    report.push_str(
        "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.construction_name),
            row.sample_index,
            row.inside_balance_residual_delta_w,
            row.inside_conduction_delta_w,
            row.inside_convection_delta_w,
            row.inside_net_longwave_delta_w,
            row.inside_convection_coefficient_delta_w_per_m2_k,
            row.inside_face_temperature_delta_c,
            row.oracle_inside_balance_residual_w,
            row.rust_inside_balance_residual_w,
            row.oracle_inside_conduction_w,
            row.rust_inside_conduction_w,
            row.oracle_inside_convection_w,
            row.rust_inside_convection_w,
            row.oracle_inside_net_longwave_w,
            row.rust_inside_net_longwave_w
        ));
    }
}

fn heat_balance_report_inside_solve_max_sample_delta_rows(
    report: &mut String,
    rows: &[HeatBalanceInsideSolveMaxSampleDelta],
) {
    report.push_str(
        "| key | construction | boundary | sample_index | implied_numerator_delta_w | denominator_delta_w_per_m2_k | ref_air_delta_c | ref_air_source_delta_w | outside_source_delta_w | history_delta_w | rust_history_temp_w | rust_history_flux_w | net_lw_delta_w | in_temp_delta_c | oracle_numerator_w | rust_numerator_w | oracle_denominator_w_per_m2_k | rust_denominator_w_per_m2_k | oracle_ref_air_c | rust_ref_air_c |\n",
    );
    report.push_str(
        "|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.construction_name),
            markdown_cell(&row.outside_boundary_condition),
            row.sample_index,
            row.implied_solve_numerator_delta_w,
            row.solve_denominator_delta_w_per_m2_k,
            row.inferred_reference_air_temperature_delta_c,
            row.reference_air_source_delta_w,
            row.outside_temperature_source_delta_w,
            row.inside_history_delta_w,
            row.rust_inside_history_temperature_term_w,
            row.rust_inside_history_flux_term_w,
            row.inside_net_longwave_delta_w,
            row.inside_face_temperature_delta_c,
            row.oracle_implied_solve_numerator_w,
            row.rust_implied_solve_numerator_w,
            row.oracle_solve_denominator_w_per_m2_k,
            row.rust_solve_denominator_w_per_m2_k,
            row.oracle_inferred_reference_air_temperature_c,
            row.rust_inferred_reference_air_temperature_c
        ));
    }
}

fn heat_balance_report_adiabatic_history_max_sample_delta_rows(
    report: &mut String,
    rows: &[HeatBalanceAdiabaticHistoryMaxSampleDelta],
) {
    report.push_str(
        "| key | construction | sample_index | out_minus_in_delta_c | oracle_out_minus_in_c | rust_out_minus_in_c | in_current_delta_w | in_current_synced_delta_w | oracle_current_sync_shift_w | rust_current_sync_shift_w | in_history_delta_w | in_history_synced_delta_w | in_temp_delta_c | out_temp_delta_c |\n",
    );
    report.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.construction_name),
            row.sample_index,
            row.outside_minus_inside_delta_c,
            row.oracle_outside_minus_inside_c,
            row.rust_outside_minus_inside_c,
            row.inside_current_delta_w,
            row.inside_current_if_outside_synced_delta_w,
            row.oracle_inside_current_sync_shift_w,
            row.rust_inside_current_sync_shift_w,
            row.inside_history_delta_w,
            row.inside_history_if_outside_synced_delta_w,
            row.inside_face_temperature_delta_c,
            row.outside_face_temperature_delta_c
        ));
    }
}

fn heat_balance_ctf_history_series_max_rmse(row: &HeatBalanceCtfHistorySeriesDelta) -> f64 {
    row.inside_current_delta
        .rmse_delta_c
        .max(row.inside_history_delta.rmse_delta_c)
        .max(row.outside_current_delta.rmse_delta_c)
        .max(row.outside_history_delta.rmse_delta_c)
}

fn heat_balance_report_ctf_history_slot_rows(
    report: &mut String,
    rows: &[HeatBalanceCtfHistorySlotSample],
) {
    report.push_str(
        "| key | construction | slot | x_hist | y_hist | z_hist | q_hist | out_temp_c | in_temp_c | out_flux_w_per_m2 | in_flux_w_per_m2 | in_temp_term_w | in_flux_term_w | in_total_w | out_temp_term_w | out_flux_term_w | out_total_w |\n",
    );
    report.push_str(
        "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.surface_name),
            markdown_cell(&row.construction_name),
            row.slot_index,
            row.outside_history_coefficient_w_per_m2_k,
            row.cross_history_coefficient_w_per_m2_k,
            row.inside_history_coefficient_w_per_m2_k,
            row.flux_history_coefficient,
            row.outside_temperature_history_c,
            row.inside_temperature_history_c,
            row.outside_flux_history_w_per_m2,
            row.inside_flux_history_w_per_m2,
            row.inside_temperature_term_w,
            row.inside_flux_term_w,
            row.inside_total_term_w,
            row.outside_temperature_term_w,
            row.outside_flux_term_w,
            row.outside_total_term_w
        ));
    }
}

fn heat_balance_report_ctf_history_first_sample_slot_rows(
    report: &mut String,
    rows: &[HeatBalanceCtfHistorySlotFirstSample],
) {
    report.push_str(
        "| key | construction | slot | timesteps | x_hist | y_hist | z_hist | q_hist | out_temp_c | in_temp_c | out_flux_w_per_m2 | in_flux_w_per_m2 | in_temp_term_w | in_flux_term_w | in_total_w | out_temp_term_w | out_flux_term_w | out_total_w |\n",
    );
    report.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} | {:.12} |\n",
            markdown_cell(&row.surface_name),
            markdown_cell(&row.construction_name),
            row.slot_index,
            row.timestep_count,
            row.outside_history_coefficient_w_per_m2_k,
            row.cross_history_coefficient_w_per_m2_k,
            row.inside_history_coefficient_w_per_m2_k,
            row.flux_history_coefficient,
            row.outside_temperature_history_c,
            row.inside_temperature_history_c,
            row.outside_flux_history_w_per_m2,
            row.inside_flux_history_w_per_m2,
            row.inside_temperature_term_w,
            row.inside_flux_term_w,
            row.inside_total_term_w,
            row.outside_temperature_term_w,
            row.outside_flux_term_w,
            row.outside_total_term_w
        ));
    }
}

fn heat_balance_report_series_rows(report: &mut String, series: &[HeatBalanceSeriesDiagnostic]) {
    report.push_str(
        "| key | variable | class | samples | max_abs_delta_c | mean_abs_delta_c | rmse_delta_c | status |\n",
    );
    report.push_str("|---|---|---|---:|---:|---:|---:|---|\n");
    for row in series {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {} |\n",
            markdown_cell(&row.output.key),
            markdown_cell(&row.output.variable),
            row.output.class,
            row.samples,
            row.delta.max_abs_delta_c,
            row.delta.mean_abs_delta_c,
            row.delta.rmse_delta_c,
            row.status
        ));
    }
}

fn heat_balance_report_delta_rows(report: &mut String, series: &[HeatBalanceSeriesDiagnostic]) {
    report.push_str("| output | sample | index | oracle_c | rust_c | abs_delta_c |\n");
    report.push_str("|---|---|---:|---:|---:|---:|\n");
    for row in series {
        let output = format!("{}/{}", row.output.key, row.output.variable);
        report_named_delta_row(
            report,
            &output,
            "first_delta_sample",
            row.delta.first_delta_sample,
        );
        report_named_delta_row(
            report,
            &output,
            "max_delta_sample",
            row.delta.max_delta_sample,
        );
    }
}

fn heat_balance_report_sample_rows(report: &mut String, series: &[HeatBalanceSeriesDiagnostic]) {
    report.push_str("| output | index | oracle | rust | abs_delta |\n");
    report.push_str("|---|---:|---:|---:|---:|\n");
    for row in series {
        let output = format!("{}/{}", row.output.key, row.output.variable);
        if row.sample_rows.is_empty() {
            report.push_str(&format!(
                "| {} | n/a | n/a | n/a | n/a |\n",
                markdown_cell(&output)
            ));
            continue;
        }
        for point in &row.sample_rows {
            report.push_str(&format!(
                "| {} | {} | {:.12} | {:.12} | {:.12} |\n",
                markdown_cell(&output),
                point.index,
                point.oracle_c,
                point.rust_c,
                point.abs_delta_c
            ));
        }
    }
}

fn report_named_delta_row(
    report: &mut String,
    output: &str,
    label: &str,
    point: Option<DeltaPoint>,
) {
    match point {
        Some(point) => report.push_str(&format!(
            "| {} | {label} | {} | {:.6} | {:.6} | {:.6} |\n",
            markdown_cell(output),
            point.index,
            point.oracle_c,
            point.rust_c,
            point.abs_delta_c
        )),
        None => report.push_str(&format!(
            "| {} | {label} | n/a | n/a | n/a | n/a |\n",
            markdown_cell(output)
        )),
    }
}

fn delta_point_json(point: Option<DeltaPoint>) -> String {
    match point {
        Some(point) => format!(
            "{{ \"index\": {}, \"oracle_c\": {}, \"rust_c\": {}, \"abs_delta_c\": {} }}",
            point.index,
            json_number(point.oracle_c),
            json_number(point.rust_c),
            json_number(point.abs_delta_c)
        ),
        None => "null".to_string(),
    }
}

fn delta_summary_json(delta: &DeltaSummary) -> String {
    format!(
        concat!(
            "{{ \"samples\": {}, ",
            "\"length_match\": {}, ",
            "\"max_abs_delta_c\": {}, ",
            "\"mean_abs_delta_c\": {}, ",
            "\"rmse_delta_c\": {}, ",
            "\"max_rel_delta\": {}, ",
            "\"first_delta_sample\": {}, ",
            "\"max_delta_sample\": {} }}"
        ),
        delta.samples,
        delta.length_match,
        json_number(delta.max_abs_delta_c),
        json_number(delta.mean_abs_delta_c),
        json_number(delta.rmse_delta_c),
        json_number(delta.max_rel_delta),
        delta_point_json(delta.first_delta_sample),
        delta_point_json(delta.max_delta_sample)
    )
}

fn delta_points_json(points: &[DeltaPoint]) -> String {
    let mut json = String::from("[");
    for (index, point) in points.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&delta_point_json(Some(*point)));
    }
    json.push(']');
    json
}

fn json_optional_number(value: Option<f64>) -> String {
    value.map_or_else(|| "null".to_string(), json_number)
}

fn json_optional_u32(value: Option<u32>) -> String {
    value.map_or_else(|| "null".to_string(), |value| value.to_string())
}

fn json_optional_i64(value: Option<i64>) -> String {
    value.map_or_else(|| "null".to_string(), |value| value.to_string())
}

fn json_number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.12}")
    } else {
        "null".to_string()
    }
}

fn json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

fn string_array_json(values: &[String]) -> String {
    let mut output = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        output.push_str(&json_string(value));
    }
    output.push(']');
    output
}

fn optional_tolerance_label(value: Option<f64>) -> String {
    value.map_or_else(|| "none".to_string(), |value| format!("{value:.12}"))
}

fn print_first_divergence(prefix: &str, divergence: Option<ep_compare::SeriesDivergence>) {
    let Some(divergence) = divergence else {
        println!("{prefix}first_divergence: none");
        return;
    };

    match (
        divergence.expected,
        divergence.observed,
        divergence.abs_delta,
    ) {
        (Some(expected), Some(observed), Some(abs_delta)) => println!(
            "{prefix}first_divergence: index {} expected {:.12} observed {:.12} abs_delta {:.12}",
            divergence.index, expected, observed, abs_delta
        ),
        (expected, observed, None) => println!(
            "{prefix}first_divergence: index {} expected {} observed {} length_mismatch",
            divergence.index,
            optional_number_label(expected),
            optional_number_label(observed)
        ),
        (expected, observed, Some(abs_delta)) => println!(
            "{prefix}first_divergence: index {} expected {} observed {} abs_delta {:.12}",
            divergence.index,
            optional_number_label(expected),
            optional_number_label(observed),
            abs_delta
        ),
    }
}

fn optional_number_label(value: Option<f64>) -> String {
    match value {
        Some(value) => format!("{value:.12}"),
        None => "missing".to_string(),
    }
}

fn print_raw_model_summary(summary: &RawModelSummary) {
    println!("RawModel");
    println!(
        "  version: {}",
        summary.version.as_deref().unwrap_or("unknown")
    );
    println!("  object_types: {}", summary.object_type_count);
    println!("  objects: {}", summary.object_count);
    println!("  object_type_counts:");
    for (object_type, count) in &summary.object_type_counts {
        let coverage = seed_coverage_status(object_type);
        println!("    {object_type}: {count} [{coverage}]");
    }
}

fn print_typed_model_summary(model: &TypedModel, report: &CompileReport) {
    println!("TypedModel");
    println!("  version: {}", model.version);
    println!("  raw_objects: {}", report.raw_object_count);
    println!("  typed_objects: {}", report.typed_object_count);
    println!("  building: {}", usize::from(model.building.is_some()));
    println!(
        "  timestep: {}",
        model.timestep.number_of_timesteps_per_hour
    );
    println!("  run_periods: {}", model.run_periods.len());
    match build_hourly_time_axis(model) {
        Ok(axis) => println!("  time_axis_hours: {}", axis.sample_count()),
        Err(error) => println!("  time_axis_error: {error}"),
    }
    println!("  site_locations: {}", usize::from(model.site.is_some()));
    println!("  materials: {}", model.materials.len());
    println!("  constructions: {}", model.constructions.len());
    println!(
        "  schedule_type_limits: {}",
        model.schedule_type_limits.len()
    );
    println!(
        "  schedules: {}",
        model.schedules.len() + model.compact_schedules.len()
    );
    println!("  constant_schedules: {}", model.schedules.len());
    println!("  compact_schedules: {}", model.compact_schedules.len());
    println!("  other_equipment: {}", model.other_equipment.len());
    println!(
        "  thermostat_dual_setpoints: {}",
        model.thermostat_dual_setpoints.len()
    );
    println!("  zone_thermostats: {}", model.zone_thermostats.len());
    println!(
        "  ideal_loads_air_systems: {}",
        model.ideal_loads_air_systems.len()
    );
    println!(
        "  zone_equipment_lists: {}",
        model.zone_equipment_lists.len()
    );
    println!(
        "  zone_equipment_connections: {}",
        model.zone_equipment_connections.len()
    );
    println!("  nodes: {}", model.nodes.len());
    println!("  node_lists: {}", model.node_lists.len());
    println!("  plant_loops: {}", model.plant_loops.len());
    println!("  plant_branches: {}", model.plant_branches.len());
    println!("  plant_branch_lists: {}", model.plant_branch_lists.len());
    println!("  plant_connectors: {}", model.plant_connectors.len());
    println!(
        "  plant_connector_lists: {}",
        model.plant_connector_lists.len()
    );
    println!(
        "  pumps_constant_speed: {}",
        model.pumps_constant_speed.len()
    );
    println!("  boilers_hot_water: {}", model.boilers_hot_water.len());
    println!(
        "  chillers_electric_eir: {}",
        model.chillers_electric_eir.len()
    );
    println!("  zones: {}", model.zones.len());
    println!("  surfaces: {}", model.surfaces.len());
    println!("  diagnostics: {}", report.diagnostics.len());
    println!("  defaults_applied: {}", report.defaults_applied.len());
    print_compile_coverage(report);
}

fn print_compile_diagnostics(report: &CompileReport) {
    println!("Compile diagnostics");
    println!("  raw_objects: {}", report.raw_object_count);
    println!("  typed_objects: {}", report.typed_object_count);
    println!("  diagnostics: {}", report.diagnostics.len());
    println!("  defaults_applied: {}", report.defaults_applied.len());
    print_compile_coverage(report);
    for diagnostic in &report.diagnostics {
        let severity = match diagnostic.severity {
            DiagnosticSeverity::Error => "error",
            DiagnosticSeverity::Warning => "warning",
        };
        let object_name = diagnostic.object_name.as_deref().unwrap_or("*");
        let field = diagnostic.field.as_deref().unwrap_or("*");
        println!(
            "    {severity} {} {}/{} field {}: {}",
            diagnostic.code, diagnostic.object_type, object_name, field, diagnostic.message
        );
    }
}

fn format_compile_diagnostics(report: &CompileReport) -> String {
    let mut message = String::new();
    message.push_str("Compile diagnostics\n");
    message.push_str(&format!("  raw_objects: {}\n", report.raw_object_count));
    message.push_str(&format!("  typed_objects: {}\n", report.typed_object_count));
    message.push_str(&format!("  diagnostics: {}\n", report.diagnostics.len()));
    message.push_str(&format!(
        "  defaults_applied: {}\n",
        report.defaults_applied.len()
    ));
    for diagnostic in &report.diagnostics {
        let severity = match diagnostic.severity {
            DiagnosticSeverity::Error => "error",
            DiagnosticSeverity::Warning => "warning",
        };
        let object_name = diagnostic.object_name.as_deref().unwrap_or("*");
        let field = diagnostic.field.as_deref().unwrap_or("*");
        message.push_str(&format!(
            "    {severity} {} {}/{} field {}: {}\n",
            diagnostic.code, diagnostic.object_type, object_name, field, diagnostic.message
        ));
    }
    message
}

fn print_compile_coverage(report: &CompileReport) {
    println!("  coverage:");
    for entry in &report.coverage {
        println!(
            "    {}: {} [{}]",
            entry.object_type, entry.object_count, entry.status
        );
    }
}

fn seed_coverage_status(object_type: &str) -> &'static str {
    const TRACKED_OBJECT_TYPES: &[&str] = &[
        "Version",
        "Building",
        "Timestep",
        "RunPeriod",
        "Site:Location",
        "Zone",
        "BuildingSurface:Detailed",
        "FenestrationSurface:Detailed",
        "Schedule:Constant",
        "Schedule:Compact",
        "ThermostatSetpoint:DualSetpoint",
        "ZoneControl:Thermostat",
        "NodeList",
        "ZoneHVAC:EquipmentConnections",
        "ZoneHVAC:EquipmentList",
        "ZoneHVAC:IdealLoadsAirSystem",
        "PlantLoop",
    ];

    if TRACKED_OBJECT_TYPES.contains(&object_type) {
        "tracked"
    } else {
        "untracked"
    }
}

fn print_oracle_info() {
    let release = default_oracle_release();

    println!("EnergyPlus oracle");
    println!("  version: {}", release.version);
    println!("  tag: {}", release.tag);
    println!("  commit: {}", release.commit);
    println!("  windows_x86_64_zip: {}", release.windows_x86_64_zip);
    println!("  windows_x86_64_sha256: {}", release.windows_x86_64_sha256);
}

fn print_modes() {
    let modes = [
        SimulationMode::Compatibility,
        SimulationMode::Diagnostic,
        SimulationMode::Fast,
        SimulationMode::Experimental,
    ];

    for mode in modes {
        println!("{mode:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    fn disabled_heat_balance_warmup() -> super::HeatBalanceWarmupDiagnostic {
        super::HeatBalanceWarmupDiagnostic {
            enabled: false,
            day_count: 0,
            timestep_count: 0,
            hours_per_day: 0,
            converged: false,
            final_max_zone_temperature_delta_c: 0.0,
            oracle_run_period_day_count: None,
        }
    }

    #[test]
    fn version_command_succeeds() {
        let args = vec!["--version".to_string()];

        assert_eq!(run(&args), 0);
    }

    #[test]
    fn unknown_command_fails() {
        let args = vec!["run".to_string()];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn missing_model_inspect_path_fails() {
        let args = vec!["model".to_string(), "inspect".to_string()];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn missing_model_compile_path_fails() {
        let args = vec!["model".to_string(), "compile".to_string()];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn missing_model_plan_path_fails() {
        let args = vec!["model".to_string(), "plan".to_string()];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn missing_model_geometry_path_fails() {
        let args = vec!["model".to_string(), "geometry".to_string()];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn missing_compare_geometry_path_fails() {
        let args = vec!["compare".to_string(), "geometry".to_string()];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn missing_compare_surface_geometry_path_fails() {
        let args = vec!["compare".to_string(), "surface-geometry".to_string()];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn missing_compare_internal_gains_path_fails() {
        let args = vec!["compare".to_string(), "internal-gains".to_string()];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn missing_compare_internal_convective_gain_path_fails() {
        let args = vec![
            "compare".to_string(),
            "internal-convective-gain".to_string(),
        ];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn missing_conformance_diagnostic_report_path_fails() {
        let args = vec!["conformance".to_string(), "diagnostic-report".to_string()];

        assert_eq!(run(&args), 2);
    }

    #[test]
    fn parse_zone_temperature_compare_args_accepts_report_dir()
    -> Result<(), Box<dyn std::error::Error>> {
        let args = vec![
            "input.epJSON".to_string(),
            "weather.epw".to_string(),
            "eplusout.eso".to_string(),
            "--report-dir".to_string(),
            "reports".to_string(),
        ];

        let parsed =
            super::parse_zone_temperature_compare_args(&args).map_err(std::io::Error::other)?;

        assert_eq!(parsed.input_path, std::path::PathBuf::from("input.epJSON"));
        assert_eq!(parsed.weather_path, std::path::PathBuf::from("weather.epw"));
        assert_eq!(parsed.eso_path, std::path::PathBuf::from("eplusout.eso"));
        assert_eq!(parsed.report_dir, Some(std::path::PathBuf::from("reports")));

        Ok(())
    }

    #[test]
    fn runtime_ctf_eio_seed_filters_mass_constructions() -> Result<(), Box<dyn std::error::Error>> {
        let path = std::env::temp_dir().join(format!(
            "rusted-energyplus-cli-ctf-{}.eio",
            std::process::id()
        ));
        std::fs::write(
            &path,
            r#"! <Construction CTF>,Construction Name,...
! <CTF>,Time,Outside,Cross,Inside,Flux (except final one)
 Construction CTF,R13WALL,   1,   1,   1,   0.250,         0.4365,   0.900,   0.900,   0.750,   0.750,Rough
 CTF,   1,            0.0000,            0.0000,             0.0000,          0.0000
 CTF,   0,            0.4365,            0.4365,             0.4365
 Construction CTF,FLOOR,   2,   1,   5,   0.250,          17.04,   0.900,   0.900,   0.650,   0.650,MediumRough
 CTF,   1,          -62.622544,           4.7096437,          -62.622544,          0.60555731
 CTF,   0,            58.08561,          0.72354869,            58.08561
"#,
        )?;

        let (coefficients, ctf_seed) = super::load_runtime_ctf_coefficients_from_eio_with_policy(
            &path,
            super::HeatBalanceCtfSeedPolicy::SteadyNoMassOnly,
        )?;
        let (all_coefficients, all_ctf_seed) =
            super::load_runtime_ctf_coefficients_from_eio_with_policy(
                &path,
                super::HeatBalanceCtfSeedPolicy::AllEio,
            )?;
        std::fs::remove_file(&path)?;

        assert_eq!(coefficients.len(), 2);
        assert!(
            coefficients
                .iter()
                .all(|coefficient| coefficient.construction_name == "R13WALL")
        );
        assert_eq!(coefficients[0].time_index, 1);
        assert_eq!(coefficients[1].time_index, 0);
        assert_eq!(ctf_seed.policy, "steady-no-mass-only");
        assert_eq!(ctf_seed.included_constructions, vec!["R13WALL"]);
        assert_eq!(
            ctf_seed.skipped_constructions,
            vec![super::HeatBalanceSkippedCtfConstruction {
                construction_name: "FLOOR".to_string(),
                ctf_count: 5,
                timestep_hours: 0.25,
            }]
        );
        assert_eq!(
            ctf_seed.construction_summaries,
            vec![
                super::HeatBalanceCtfConstructionSummary {
                    construction_name: "R13WALL".to_string(),
                    ctf_count: 1,
                    timestep_hours: 0.25,
                    included: true,
                },
                super::HeatBalanceCtfConstructionSummary {
                    construction_name: "FLOOR".to_string(),
                    ctf_count: 5,
                    timestep_hours: 0.25,
                    included: false,
                },
            ]
        );
        assert_eq!(ctf_seed.included_coefficients, 2);
        assert_eq!(ctf_seed.skipped_coefficients, 2);
        assert_eq!(all_coefficients.len(), 4);
        assert_eq!(all_ctf_seed.policy, "all-eio");
        assert_eq!(
            all_ctf_seed.included_constructions,
            vec!["FLOOR", "R13WALL"]
        );
        assert!(all_ctf_seed.skipped_constructions.is_empty());
        assert_eq!(
            all_ctf_seed.construction_summaries,
            vec![
                super::HeatBalanceCtfConstructionSummary {
                    construction_name: "R13WALL".to_string(),
                    ctf_count: 1,
                    timestep_hours: 0.25,
                    included: true,
                },
                super::HeatBalanceCtfConstructionSummary {
                    construction_name: "FLOOR".to_string(),
                    ctf_count: 5,
                    timestep_hours: 0.25,
                    included: true,
                },
            ]
        );
        assert_eq!(all_ctf_seed.included_coefficients, 4);
        assert_eq!(all_ctf_seed.skipped_coefficients, 0);

        Ok(())
    }

    #[test]
    fn heat_balance_ctf_seed_policy_parser_accepts_probe_policy() {
        assert_eq!(
            super::parse_heat_balance_ctf_seed_policy("").unwrap(),
            super::HeatBalanceCtfSeedPolicy::SteadyNoMassOnly
        );
        assert_eq!(
            super::parse_heat_balance_ctf_seed_policy("steady-no-mass-only").unwrap(),
            super::HeatBalanceCtfSeedPolicy::SteadyNoMassOnly
        );
        assert_eq!(
            super::parse_heat_balance_ctf_seed_policy("all-eio").unwrap(),
            super::HeatBalanceCtfSeedPolicy::AllEio
        );
        assert!(super::parse_heat_balance_ctf_seed_policy("mass-only").is_err());
    }

    #[test]
    fn heat_balance_ctf_initial_history_policy_parser_accepts_probe_policy() {
        assert_eq!(
            super::parse_heat_balance_ctf_initial_history_policy("").unwrap(),
            ep_runtime::HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue
        );
        assert_eq!(
            super::parse_heat_balance_ctf_initial_history_policy("boundary-u-value").unwrap(),
            ep_runtime::HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue
        );
        assert_eq!(
            super::parse_heat_balance_ctf_initial_history_policy("energyplus-surf-initial")
                .unwrap(),
            ep_runtime::HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial
        );
        assert!(super::parse_heat_balance_ctf_initial_history_policy("initial-dry-bulb").is_err());
    }

    #[test]
    fn heat_balance_zone_air_algorithm_parser_accepts_probe_algorithm() {
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm("").unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm("simplified-analytical").unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm("energyplus-analytical-probe").unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-surface-first-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm("energyplus-analytical-coupled-probe")
                .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-doe2-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-longwave-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-adiabatic-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-frozen-outside-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-commit-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-adiabatic-history-commit-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-current-adiabatic-history-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-interior-longwave-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-interior-longwave-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm(
                "energyplus-analytical-coupled-previous-boundary-probe"
            )
            .unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
        );
        assert_eq!(
            super::parse_heat_balance_zone_air_algorithm("energyplus-third-order-probe").unwrap(),
            ep_runtime::HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe
        );
        assert!(super::parse_heat_balance_zone_air_algorithm("third-order").is_err());
    }

    #[test]
    fn heat_balance_warmup_minimum_days_parser_accepts_empty_or_positive_days() {
        assert_eq!(
            super::parse_heat_balance_warmup_minimum_days("").unwrap(),
            None
        );
        assert_eq!(
            super::parse_heat_balance_warmup_minimum_days("0").unwrap(),
            None
        );
        assert_eq!(
            super::parse_heat_balance_warmup_minimum_days("20").unwrap(),
            Some(20)
        );
        assert!(super::parse_heat_balance_warmup_minimum_days("-1").is_err());
        assert!(super::parse_heat_balance_warmup_minimum_days("oracle").is_err());
    }

    #[test]
    fn heat_balance_surface_iterations_parser_accepts_empty_or_positive_count() {
        assert_eq!(
            super::parse_heat_balance_surface_iterations("").unwrap(),
            None
        );
        assert_eq!(
            super::parse_heat_balance_surface_iterations("0").unwrap(),
            None
        );
        assert_eq!(
            super::parse_heat_balance_surface_iterations("3").unwrap(),
            Some(3)
        );
        assert!(super::parse_heat_balance_surface_iterations("-1").is_err());
        assert!(super::parse_heat_balance_surface_iterations("many").is_err());
    }

    #[test]
    fn zone_temperature_delta_summary_tracks_diagnostic_samples() {
        let summary = super::delta_summary(&[1.0, 3.0, 8.0], &[1.0, 4.5, 4.0]);

        assert_eq!(summary.samples, 3);
        assert_eq!(summary.mean_abs_delta_c, (0.0 + 1.5 + 4.0) / 3.0);
        assert_eq!(summary.max_abs_delta_c, 4.0);
        assert_eq!(
            summary.first_delta_sample,
            Some(super::DeltaPoint {
                index: 1,
                oracle_c: 3.0,
                rust_c: 4.5,
                abs_delta_c: 1.5,
            })
        );
        assert_eq!(
            summary.max_delta_sample,
            Some(super::DeltaPoint {
                index: 2,
                oracle_c: 8.0,
                rust_c: 4.0,
                abs_delta_c: 4.0,
            })
        );
        assert!(summary.length_match);
    }

    #[test]
    fn zone_temperature_summary_json_keeps_diagnostic_boundary() {
        let diagnostic = super::ZoneTemperatureDiagnostic {
            zone_name: "ZONE ONE".to_string(),
            samples: 2,
            heat_balance_timesteps: 8,
            heat_balance_run_period_timesteps: 8,
            heat_balance_warmup: disabled_heat_balance_warmup(),
            zone_count: 1,
            surface_count: 6,
            oracle_first_c: 21.0,
            rust_first_c: 20.5,
            oracle_last_c: 22.0,
            rust_last_c: 20.8,
            delta: super::delta_summary(&[21.0, 22.0], &[20.5, 20.8]),
            status: "extracted",
        };

        let json = super::render_zone_temperature_summary_json(&diagnostic, None);

        assert!(json.contains("\"case_id\": null"));
        assert!(json.contains("\"comparison_class\": \"diagnostic-only\""));
        assert!(json.contains("\"conformance_claim\": false"));
        assert!(json.contains("\"tolerance_policy\": \"none\""));
        assert!(json.contains("\"first_delta_sample\""));
        assert!(json.contains("\"max_delta_sample\""));
    }

    #[test]
    fn zone_temperature_summary_json_includes_manifest_context() {
        let diagnostic = super::ZoneTemperatureDiagnostic {
            zone_name: "ZONE ONE".to_string(),
            samples: 2,
            heat_balance_timesteps: 8,
            heat_balance_run_period_timesteps: 8,
            heat_balance_warmup: disabled_heat_balance_warmup(),
            zone_count: 1,
            surface_count: 6,
            oracle_first_c: 21.0,
            rust_first_c: 20.5,
            oracle_last_c: 22.0,
            rust_last_c: 20.8,
            delta: super::delta_summary(&[21.0, 22.0], &[20.5, 20.8]),
            status: "extracted",
        };
        let context = super::ZoneTemperatureReportContext {
            case_id: "zone_temperature_diagnostic_001".to_string(),
            oracle_version: "26.1.0".to_string(),
            output: super::ZoneTemperatureReportOutput {
                key: "ZONE ONE".to_string(),
                variable: "Zone Mean Air Temperature".to_string(),
                frequency: "hourly",
                class: "zone-state",
                source: "eso",
            },
            report: Some(super::ZoneTemperatureReportContract {
                format: "markdown",
                path: ".runtime/compare-report.md".to_string(),
            }),
            gate: Some(super::ZoneTemperatureGateContract {
                script: "scripts/dev.cmd compare-zone-smoke".to_string(),
                blocking: false,
            }),
        };

        let json = super::render_zone_temperature_summary_json(&diagnostic, Some(&context));
        let report = super::render_zone_temperature_report(&diagnostic, Some(&context));

        assert!(json.contains("\"case_id\": \"zone_temperature_diagnostic_001\""));
        assert!(json.contains("\"oracle_version\": \"26.1.0\""));
        assert!(json.contains("\"output\""));
        assert!(json.contains("\"variable\": \"Zone Mean Air Temperature\""));
        assert!(json.contains("\"source\": \"eso\""));
        assert!(json.contains("\"gate\""));
        assert!(json.contains("\"blocking\": false"));
        assert!(report.contains("## Manifest"));
        assert!(report.contains("case_id: zone_temperature_diagnostic_001"));
        assert!(report.contains("output_variable: Zone Mean Air Temperature"));
        assert!(report.contains("output_source: eso"));
        assert!(report.contains("gate_blocking: false"));
    }

    #[test]
    fn heat_balance_conformance_report_records_pass_and_tolerances() {
        let diagnostic = super::HeatBalanceConformanceDiagnostic {
            samples: 2,
            heat_balance_timesteps: 8,
            heat_balance_run_period_timesteps: 8,
            heat_balance_warmup: disabled_heat_balance_warmup(),
            ctf_seed: super::disabled_heat_balance_ctf_seed_diagnostic(),
            zone_air_algorithm: "simplified-analytical",
            surface_iteration_count: 1,
            ctf_initial_history_policy: "boundary-u-value",
            zone_count: 1,
            surface_count: 6,
            ctf_component_first_samples: vec![super::HeatBalanceCtfComponentFirstSample {
                key: "FLOOR".to_string(),
                inside_conduction_rate_w: -10.0,
                inside_current_outside_term_w: 30.0,
                inside_current_inside_term_w: -40.0,
                inside_history_term_w: 0.0,
                outside_conduction_rate_w: 10.0,
                outside_current_outside_term_w: -30.0,
                outside_current_inside_term_w: 40.0,
                outside_history_term_w: 0.0,
                heat_storage_rate_w: 0.0,
            }],
            ctf_history_first_sample_deltas: vec![super::HeatBalanceCtfHistoryFirstSampleDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                area_m2: 100.0,
                ctf_outside_0_w_per_m2_k: 11.0,
                ctf_cross_0_w_per_m2_k: 12.0,
                ctf_inside_0_w_per_m2_k: 13.0,
                oracle_outside_face_temperature_c: 14.0,
                rust_outside_face_temperature_c: 15.0,
                outside_face_temperature_delta_c: 1.0,
                oracle_inside_face_temperature_c: 16.0,
                rust_inside_face_temperature_c: 17.0,
                inside_face_temperature_delta_c: 1.0,
                oracle_inside_current_term_w: 9.0,
                rust_inside_current_term_w: 10.0,
                inside_current_delta_w: 1.0,
                oracle_inside_history_term_w: 1.0,
                rust_inside_history_term_w: 0.0,
                inside_history_delta_w: 1.0,
                oracle_outside_current_term_w: -8.0,
                rust_outside_current_term_w: -10.0,
                outside_current_delta_w: 2.0,
                oracle_outside_history_term_w: 2.0,
                rust_outside_history_term_w: 0.0,
                outside_history_delta_w: 2.0,
            }],
            ctf_history_series_deltas: vec![super::HeatBalanceCtfHistorySeriesDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                area_m2: 100.0,
                samples: 2,
                inside_current_delta: super::delta_summary(&[9.0, 11.0], &[10.0, 13.0]),
                inside_history_delta: super::delta_summary(&[1.0, 2.0], &[0.0, 4.0]),
                outside_current_delta: super::delta_summary(&[-8.0, -7.0], &[-10.0, -7.5]),
                outside_history_delta: super::delta_summary(&[2.0, 3.0], &[0.0, 6.0]),
            }],
            ctf_storage_max_sample_deltas: vec![super::HeatBalanceCtfStorageMaxSampleDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                sample_index: 1,
                area_m2: 100.0,
                oracle_inside_conduction_w: 20.0,
                rust_inside_conduction_w: 18.0,
                inside_conduction_delta_w: 2.0,
                oracle_outside_conduction_w: -8.0,
                rust_outside_conduction_w: -7.0,
                outside_conduction_delta_w: 1.0,
                oracle_storage_w: -12.0,
                rust_storage_w: -11.0,
                storage_delta_w: 1.0,
                oracle_inside_current_term_w: 9.0,
                rust_inside_current_term_w: 10.0,
                inside_current_delta_w: 1.0,
                oracle_inside_history_term_w: 11.0,
                rust_inside_history_term_w: 8.0,
                inside_history_delta_w: 3.0,
                oracle_outside_current_term_w: -7.0,
                rust_outside_current_term_w: -9.0,
                outside_current_delta_w: 2.0,
                oracle_outside_history_term_w: -1.0,
                rust_outside_history_term_w: 2.0,
                outside_history_delta_w: 3.0,
            }],
            inside_balance_max_sample_deltas: vec![super::HeatBalanceInsideBalanceMaxSampleDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                sample_index: 1,
                area_m2: 100.0,
                oracle_inside_face_temperature_c: 16.0,
                rust_inside_face_temperature_c: 17.0,
                inside_face_temperature_delta_c: 1.0,
                oracle_inside_convection_coefficient_w_per_m2_k: 2.0,
                rust_inside_convection_coefficient_w_per_m2_k: 3.0,
                inside_convection_coefficient_delta_w_per_m2_k: 1.0,
                oracle_inside_conduction_w: 20.0,
                rust_inside_conduction_w: 18.0,
                inside_conduction_delta_w: 2.0,
                oracle_inside_convection_w: -4.0,
                rust_inside_convection_w: -5.0,
                inside_convection_delta_w: 1.0,
                oracle_inside_net_longwave_w: -12.0,
                rust_inside_net_longwave_w: -10.0,
                inside_net_longwave_delta_w: 2.0,
                oracle_inside_balance_residual_w: 4.0,
                rust_inside_balance_residual_w: 3.0,
                inside_balance_residual_delta_w: 1.0,
            }],
            inside_solve_max_sample_deltas: vec![super::HeatBalanceInsideSolveMaxSampleDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                outside_boundary_condition: "adiabatic".to_string(),
                sample_index: 1,
                area_m2: 100.0,
                ctf_inside_0_w_per_m2_k: 4.0,
                ctf_cross_0_w_per_m2_k: 1.0,
                iter_damp_w_per_m2_k: 5.0,
                oracle_inside_face_temperature_c: 16.0,
                rust_inside_face_temperature_c: 17.0,
                inside_face_temperature_delta_c: 1.0,
                oracle_inferred_reference_air_temperature_c: 14.0,
                rust_inferred_reference_air_temperature_c: 15.0,
                inferred_reference_air_temperature_delta_c: 1.0,
                oracle_solve_denominator_w_per_m2_k: 8.0,
                rust_solve_denominator_w_per_m2_k: 9.0,
                solve_denominator_delta_w_per_m2_k: 1.0,
                oracle_implied_solve_numerator_w: 12800.0,
                rust_implied_solve_numerator_w: 15300.0,
                implied_solve_numerator_delta_w: 2500.0,
                oracle_reference_air_source_w: 2800.0,
                rust_reference_air_source_w: 4500.0,
                reference_air_source_delta_w: 1700.0,
                oracle_outside_temperature_source_w: 0.0,
                rust_outside_temperature_source_w: 0.0,
                outside_temperature_source_delta_w: 0.0,
                oracle_inside_history_term_w: 11.0,
                rust_inside_history_term_w: 8.0,
                inside_history_delta_w: 3.0,
                rust_inside_history_temperature_term_w: 6.0,
                rust_inside_history_flux_term_w: 2.0,
                oracle_inside_net_longwave_w: -12.0,
                rust_inside_net_longwave_w: -10.0,
                inside_net_longwave_delta_w: 2.0,
            }],
            adiabatic_history_max_sample_deltas: vec![
                super::HeatBalanceAdiabaticHistoryMaxSampleDelta {
                    key: "FLOOR".to_string(),
                    construction_name: "FLOOR".to_string(),
                    sample_index: 1,
                    area_m2: 100.0,
                    ctf_inside_0_w_per_m2_k: 4.0,
                    ctf_cross_0_w_per_m2_k: 1.0,
                    oracle_inside_face_temperature_c: 16.0,
                    rust_inside_face_temperature_c: 17.0,
                    inside_face_temperature_delta_c: 1.0,
                    oracle_outside_face_temperature_c: 15.0,
                    rust_outside_face_temperature_c: 14.0,
                    outside_face_temperature_delta_c: 1.0,
                    oracle_outside_minus_inside_c: -1.0,
                    rust_outside_minus_inside_c: -3.0,
                    outside_minus_inside_delta_c: 2.0,
                    oracle_inside_current_term_w: -4900.0,
                    rust_inside_current_term_w: -5400.0,
                    inside_current_delta_w: 500.0,
                    oracle_inside_current_if_outside_synced_w: -4800.0,
                    rust_inside_current_if_outside_synced_w: -5100.0,
                    inside_current_if_outside_synced_delta_w: 300.0,
                    oracle_inside_current_sync_shift_w: 100.0,
                    rust_inside_current_sync_shift_w: 300.0,
                    oracle_inside_history_term_w: 4920.0,
                    rust_inside_history_term_w: 5418.0,
                    inside_history_delta_w: 498.0,
                    oracle_inside_history_if_outside_synced_w: 4820.0,
                    rust_inside_history_if_outside_synced_w: 5118.0,
                    inside_history_if_outside_synced_delta_w: 298.0,
                },
            ],
            ctf_history_run_period_initial_slots: vec![super::HeatBalanceCtfHistorySlotSample {
                surface_name: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                slot_index: 1,
                area_m2: 100.0,
                outside_history_coefficient_w_per_m2_k: 1.0,
                cross_history_coefficient_w_per_m2_k: 2.0,
                inside_history_coefficient_w_per_m2_k: 3.0,
                flux_history_coefficient: 0.5,
                outside_temperature_history_c: 9.0,
                inside_temperature_history_c: 19.0,
                outside_flux_history_w_per_m2: -2.0,
                inside_flux_history_w_per_m2: 2.0,
                inside_temperature_term_w: -3900.0,
                inside_flux_term_w: 100.0,
                inside_total_term_w: -3800.0,
                outside_temperature_term_w: 2900.0,
                outside_flux_term_w: 100.0,
                outside_total_term_w: 3000.0,
            }],
            ctf_history_first_sample_slots: vec![super::HeatBalanceCtfHistorySlotFirstSample {
                surface_name: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                slot_index: 1,
                area_m2: 100.0,
                timestep_count: 4,
                outside_history_coefficient_w_per_m2_k: 1.0,
                cross_history_coefficient_w_per_m2_k: 2.0,
                inside_history_coefficient_w_per_m2_k: 3.0,
                flux_history_coefficient: 0.5,
                outside_temperature_history_c: 10.0,
                inside_temperature_history_c: 20.0,
                outside_flux_history_w_per_m2: -1.0,
                inside_flux_history_w_per_m2: 1.0,
                inside_temperature_term_w: -4000.0,
                inside_flux_term_w: 50.0,
                inside_total_term_w: -3950.0,
                outside_temperature_term_w: 3000.0,
                outside_flux_term_w: 50.0,
                outside_total_term_w: 3050.0,
            }],
            surface_first_sample_trace: vec![super::HeatBalanceSurfaceFirstSampleTrace {
                surface_name: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                timestep_index: 1,
                outdoor_dry_bulb_c: 10.0,
                zone_mean_air_temperature_c: 23.0,
                inside_face_temperature_c: 22.0,
                outside_face_temperature_c: 11.0,
                inside_convection_heat_gain_rate_w: 1.0,
                inside_net_surface_thermal_radiation_heat_gain_rate_w: 2.0,
                inside_conduction_rate_w: -10.0,
                outside_conduction_rate_w: 10.0,
                heat_storage_rate_w: 0.0,
                outside_convection_heat_gain_rate_w: -3.0,
                outside_net_thermal_radiation_heat_gain_rate_w: -4.0,
                outside_solar_radiation_heat_gain_rate_w: 5.0,
            }],
            series: vec![
                super::HeatBalanceSeriesDiagnostic {
                    output: super::ZoneTemperatureReportOutput {
                        key: "ZONE ONE".to_string(),
                        variable: "Zone Mean Air Temperature".to_string(),
                        frequency: "hourly",
                        class: "zone-state",
                        source: "eso",
                    },
                    samples: 2,
                    oracle_first_c: 23.0,
                    rust_first_c: 23.0,
                    oracle_last_c: 23.0,
                    rust_last_c: 23.0,
                    delta: super::delta_summary(&[23.0, 23.0], &[23.0, 23.0]),
                    sample_rows: super::delta_points(&[23.0, 23.0], &[23.0, 23.0]),
                    status: "extracted",
                },
                super::HeatBalanceSeriesDiagnostic {
                    output: super::ZoneTemperatureReportOutput {
                        key: "FLOOR".to_string(),
                        variable: "Surface Inside Face Temperature".to_string(),
                        frequency: "hourly",
                        class: "surface-state",
                        source: "eso",
                    },
                    samples: 2,
                    oracle_first_c: 23.0,
                    rust_first_c: 23.0,
                    oracle_last_c: 23.0,
                    rust_last_c: 23.0,
                    delta: super::delta_summary(&[23.0, 23.0], &[23.0, 23.0]),
                    sample_rows: super::delta_points(&[23.0, 23.0], &[23.0, 23.0]),
                    status: "extracted",
                },
            ],
            status: "extracted",
        };
        let context = super::HeatBalanceConformanceContext {
            case_id: "heat_balance_nomass_001".to_string(),
            oracle_version: "26.1.0".to_string(),
            outputs: diagnostic
                .series
                .iter()
                .map(|series| series.output.clone())
                .collect(),
            tolerances: vec![
                super::HeatBalanceToleranceReport {
                    variable_class_label: "zone-state",
                    max_abs_c: Some(0.000001),
                    max_rmse_c: Some(0.000001),
                    max_rel: None,
                },
                super::HeatBalanceToleranceReport {
                    variable_class_label: "surface-state",
                    max_abs_c: Some(0.000001),
                    max_rmse_c: Some(0.000001),
                    max_rel: None,
                },
            ],
            report: Some(super::ZoneTemperatureReportContract {
                format: "markdown",
                path: ".runtime/heat-balance-conformance/report.md".to_string(),
            }),
            gate: Some(super::ZoneTemperatureGateContract {
                script: "scripts/dev.cmd compare-heat-balance-conformance".to_string(),
                blocking: true,
            }),
            comparison_class: "conformance",
            conformance_claim: true,
        };
        let conformance = super::evaluate_heat_balance_conformance(&diagnostic, &context);

        let json = super::render_heat_balance_conformance_summary_json(&diagnostic, &conformance);
        let digest = super::render_heat_balance_conformance_digest_json(&diagnostic, &conformance);
        let report = super::render_heat_balance_conformance_report(&diagnostic, &conformance);

        assert_eq!(conformance.status, "pass");
        assert!(json.contains("\"case_id\": \"heat_balance_nomass_001\""));
        assert!(json.contains("\"comparison_class\": \"conformance\""));
        assert!(json.contains("\"conformance_claim\": true"));
        assert!(json.contains("\"status\": \"pass\""));
        assert!(json.contains("\"ctf_seed\""));
        assert!(json.contains("\"policy\": \"disabled\""));
        assert!(json.contains("\"zone_air_algorithm\": \"simplified-analytical\""));
        assert!(json.contains("\"surface_iteration_count\": 1"));
        assert!(json.contains("\"ctf_initial_history_policy\": \"boundary-u-value\""));
        assert!(json.contains("\"bottlenecks\""));
        assert!(json.contains("\"first_sample_bottlenecks\""));
        assert!(json.contains("\"surface_first_sample_trace\""));
        assert!(json.contains("\"outside_face_temperature_c\": 11.000000000000"));
        assert!(json.contains("\"ctf_component_first_samples\""));
        assert!(json.contains("\"inside_current_outside_term_w\""));
        assert!(json.contains("\"ctf_history_first_sample_deltas\""));
        assert!(json.contains("\"inside_current_delta_w\""));
        assert!(json.contains("\"inside_history_delta_w\""));
        assert!(json.contains("\"ctf_history_series_deltas\""));
        assert!(json.contains("\"inside_current_delta\""));
        assert!(json.contains("\"inside_history_delta\""));
        assert!(json.contains("\"ctf_storage_max_sample_deltas\""));
        assert!(json.contains("\"storage_delta_w\""));
        assert!(json.contains("\"inside_balance_max_sample_deltas\""));
        assert!(json.contains("\"inside_balance_residual_delta_w\""));
        assert!(json.contains("\"inside_solve_max_sample_deltas\""));
        assert!(json.contains("\"implied_solve_numerator_delta_w\""));
        assert!(json.contains("\"rust_inside_history_temperature_term_w\""));
        assert!(json.contains("\"adiabatic_history_max_sample_deltas\""));
        assert!(json.contains("\"outside_minus_inside_delta_c\""));
        assert!(json.contains("\"ctf_history_run_period_initial_slots\""));
        assert!(json.contains("\"ctf_history_first_sample_slots\""));
        assert!(json.contains("\"inside_total_term_w\""));
        assert!(json.contains("\"rank\": 1"));
        assert!(json.contains("\"first_delta_sample\""));
        assert!(json.contains("\"max_delta_sample\""));
        assert!(json.contains("\"max_sample_contexts\""));
        assert!(json.contains("\"trigger_rank\": 1"));
        assert!(json.contains("\"sample_index\": 0"));
        assert!(json.contains("\"max_abs_c\": 0.000001000000"));
        assert!(json.contains("\"series_count\": 2"));
        assert!(json.contains("\"variable\": \"Surface Inside Face Temperature\""));
        assert!(json.contains("\"sample_rows\""));
        assert!(json.contains("\"compare_digest_json\": \"compare-digest.json\""));
        assert!(json.contains("\"blocking\": true"));
        assert!(digest.contains("\"case_id\": \"heat_balance_nomass_001\""));
        assert!(digest.contains("\"series_count\": 2"));
        assert!(digest.contains("\"variable\": \"Surface Inside Face Temperature\""));
        assert!(digest.contains("\"first_sample_bottlenecks\""));
        assert!(digest.contains("\"surface_first_sample_trace\""));
        assert!(digest.contains("\"outdoor_dry_bulb_c\": 10.000000000000"));
        assert!(digest.contains("\"ctf_component_first_samples\""));
        assert!(digest.contains("\"inside_current_outside_term_w\""));
        assert!(digest.contains("\"ctf_history_first_sample_deltas\""));
        assert!(digest.contains("\"inside_current_delta_w\""));
        assert!(digest.contains("\"inside_history_delta_w\""));
        assert!(digest.contains("\"ctf_history_series_deltas\""));
        assert!(digest.contains("\"inside_current_delta\""));
        assert!(digest.contains("\"inside_history_delta\""));
        assert!(digest.contains("\"ctf_storage_max_sample_deltas\""));
        assert!(digest.contains("\"storage_delta_w\""));
        assert!(digest.contains("\"inside_balance_max_sample_deltas\""));
        assert!(digest.contains("\"inside_balance_residual_delta_w\""));
        assert!(digest.contains("\"inside_solve_max_sample_deltas\""));
        assert!(digest.contains("\"implied_solve_numerator_delta_w\""));
        assert!(digest.contains("\"rust_inside_history_flux_term_w\""));
        assert!(digest.contains("\"ctf_cross_0_w_per_m2_k\""));
        assert!(digest.contains("\"inside_face_temperature_delta_c\""));
        assert!(digest.contains("\"adiabatic_history_max_sample_deltas\""));
        assert!(digest.contains("\"inside_current_if_outside_synced_delta_w\""));
        assert!(digest.contains("\"ctf_history_run_period_initial_slots\""));
        assert!(digest.contains("\"ctf_history_first_sample_slots\""));
        assert!(digest.contains("\"inside_total_term_w\""));
        assert!(digest.contains("\"first_delta_sample\""));
        assert!(digest.contains("\"max_delta_sample\""));
        assert!(digest.contains("\"max_sample_contexts\""));
        assert!(digest.contains("\"trigger_output\""));
        assert!(digest.contains("\"compare_summary_json\": \"compare-summary.json\""));
        assert!(digest.contains("\"compare_digest_json\": \"compare-digest.json\""));
        assert!(!digest.contains("\"sample_rows\""));
        assert!(report.contains("Heat Balance Conformance Report"));
        assert!(report.contains("comparison_class: conformance"));
        assert!(report.contains("conformance_claim: true"));
        assert!(report.contains("status: pass"));
        assert!(report.contains("Rust Surface First-Sample Trace"));
        assert!(report.contains("surface_iteration_count: 1"));
        assert!(report.contains("ctf_initial_history_policy: boundary-u-value"));
        assert!(report.contains("failure_reasons: none"));
        assert!(report.contains("ctf_seed_policy: disabled"));
        assert!(report.contains("zone_air_algorithm: simplified-analytical"));
        assert!(report.contains("## Bottlenecks"));
        assert!(report.contains("## Max-Sample Contexts"));
        assert!(report.contains("## First-Sample Bottlenecks"));
        assert!(report.contains("## Rust CTF First-Sample Components"));
        assert!(
            report.contains("| FLOOR | -10.000000000000 | 30.000000000000 | -40.000000000000 |")
        );
        assert!(report.contains("## CTF History First-Sample Deltas"));
        assert!(report.contains(
            "| FLOOR | FLOOR | 100.000000000000 | 11.000000000000 | 12.000000000000 | 13.000000000000 | 14.000000000000 | 15.000000000000 | 1.000000000000 | 16.000000000000 | 17.000000000000 | 1.000000000000 | 9.000000000000 | 10.000000000000 | 1.000000000000 |"
        ));
        assert!(report.contains("## CTF History Series Deltas"));
        assert!(report.contains("in_history_rmse_w"));
        assert!(report.contains("## CTF Storage Max-Sample Deltas"));
        assert!(report.contains("storage_delta_w"));
        assert!(report.contains("## Inside Balance Max-Sample Deltas"));
        assert!(report.contains("residual_delta_w"));
        assert!(report.contains("## Inside Solve Max-Sample Deltas"));
        assert!(report.contains("implied_numerator_delta_w"));
        assert!(report.contains("rust_history_temp_w"));
        assert!(report.contains("## Adiabatic History Max-Sample Deltas"));
        assert!(report.contains("out_minus_in_delta_c"));
        assert!(report.contains("## Rust CTF History Run-Period Initial Slots"));
        assert!(report.contains("## Rust CTF History First-Sample Slots"));
        assert!(report.contains("| FLOOR | FLOOR | 1 | 4 |"));
        assert!(report.contains("gate_blocking: true"));
        assert!(report.contains("Surface Inside Face Temperature"));
        assert!(report.contains("## Hourly Samples"));
        assert!(report.contains("| ZONE ONE/Zone Mean Air Temperature | 0 | 23.000000000000 | 23.000000000000 | 0.000000000000 |"));
    }

    #[test]
    fn heat_balance_report_can_render_diagnostic_non_claim_boundary() {
        let diagnostic = super::HeatBalanceConformanceDiagnostic {
            samples: 1,
            heat_balance_timesteps: 4,
            heat_balance_run_period_timesteps: 4,
            heat_balance_warmup: super::HeatBalanceWarmupDiagnostic {
                enabled: true,
                day_count: 6,
                timestep_count: 576,
                hours_per_day: 24,
                converged: true,
                final_max_zone_temperature_delta_c: 0.0005,
                oracle_run_period_day_count: Some(20),
            },
            ctf_seed: super::HeatBalanceCtfSeedDiagnostic {
                policy: "steady-no-mass-only",
                included_constructions: vec!["R13WALL".to_string(), "ROOF31".to_string()],
                skipped_constructions: vec![super::HeatBalanceSkippedCtfConstruction {
                    construction_name: "FLOOR".to_string(),
                    ctf_count: 5,
                    timestep_hours: 0.25,
                }],
                construction_summaries: vec![
                    super::HeatBalanceCtfConstructionSummary {
                        construction_name: "R13WALL".to_string(),
                        ctf_count: 1,
                        timestep_hours: 0.25,
                        included: true,
                    },
                    super::HeatBalanceCtfConstructionSummary {
                        construction_name: "FLOOR".to_string(),
                        ctf_count: 5,
                        timestep_hours: 0.25,
                        included: false,
                    },
                    super::HeatBalanceCtfConstructionSummary {
                        construction_name: "ROOF31".to_string(),
                        ctf_count: 1,
                        timestep_hours: 0.25,
                        included: true,
                    },
                ],
                included_coefficients: 4,
                skipped_coefficients: 6,
            },
            zone_air_algorithm: "energyplus-third-order-probe",
            surface_iteration_count: 3,
            ctf_initial_history_policy: "energyplus-surf-initial",
            zone_count: 1,
            surface_count: 6,
            ctf_component_first_samples: vec![super::HeatBalanceCtfComponentFirstSample {
                key: "FLOOR".to_string(),
                inside_conduction_rate_w: -2.0,
                inside_current_outside_term_w: 3.0,
                inside_current_inside_term_w: -5.0,
                inside_history_term_w: 0.0,
                outside_conduction_rate_w: 1.0,
                outside_current_outside_term_w: -3.0,
                outside_current_inside_term_w: 4.0,
                outside_history_term_w: 0.0,
                heat_storage_rate_w: 1.0,
            }],
            ctf_history_first_sample_deltas: vec![super::HeatBalanceCtfHistoryFirstSampleDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                area_m2: 100.0,
                ctf_outside_0_w_per_m2_k: 11.0,
                ctf_cross_0_w_per_m2_k: 12.0,
                ctf_inside_0_w_per_m2_k: 13.0,
                oracle_outside_face_temperature_c: 14.0,
                rust_outside_face_temperature_c: 15.0,
                outside_face_temperature_delta_c: 1.0,
                oracle_inside_face_temperature_c: 16.0,
                rust_inside_face_temperature_c: 17.0,
                inside_face_temperature_delta_c: 1.0,
                oracle_inside_current_term_w: 6.0,
                rust_inside_current_term_w: 7.0,
                inside_current_delta_w: 1.0,
                oracle_inside_history_term_w: 4.0,
                rust_inside_history_term_w: 0.0,
                inside_history_delta_w: 4.0,
                oracle_outside_current_term_w: -6.0,
                rust_outside_current_term_w: -8.0,
                outside_current_delta_w: 2.0,
                oracle_outside_history_term_w: 5.0,
                rust_outside_history_term_w: 0.0,
                outside_history_delta_w: 5.0,
            }],
            ctf_history_series_deltas: vec![super::HeatBalanceCtfHistorySeriesDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                area_m2: 100.0,
                samples: 2,
                inside_current_delta: super::delta_summary(&[6.0, 8.0], &[7.0, 9.0]),
                inside_history_delta: super::delta_summary(&[4.0, 5.0], &[0.0, 3.0]),
                outside_current_delta: super::delta_summary(&[-6.0, -9.0], &[-8.0, -8.0]),
                outside_history_delta: super::delta_summary(&[5.0, 6.0], &[0.0, 4.0]),
            }],
            ctf_storage_max_sample_deltas: vec![super::HeatBalanceCtfStorageMaxSampleDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                sample_index: 1,
                area_m2: 100.0,
                oracle_inside_conduction_w: 12.0,
                rust_inside_conduction_w: 10.0,
                inside_conduction_delta_w: 2.0,
                oracle_outside_conduction_w: -5.0,
                rust_outside_conduction_w: -8.0,
                outside_conduction_delta_w: 3.0,
                oracle_storage_w: -7.0,
                rust_storage_w: -2.0,
                storage_delta_w: 5.0,
                oracle_inside_current_term_w: 6.0,
                rust_inside_current_term_w: 7.0,
                inside_current_delta_w: 1.0,
                oracle_inside_history_term_w: 6.0,
                rust_inside_history_term_w: 3.0,
                inside_history_delta_w: 3.0,
                oracle_outside_current_term_w: -6.0,
                rust_outside_current_term_w: -8.0,
                outside_current_delta_w: 2.0,
                oracle_outside_history_term_w: 1.0,
                rust_outside_history_term_w: 4.0,
                outside_history_delta_w: 3.0,
            }],
            inside_balance_max_sample_deltas: vec![super::HeatBalanceInsideBalanceMaxSampleDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                sample_index: 1,
                area_m2: 100.0,
                oracle_inside_face_temperature_c: 16.0,
                rust_inside_face_temperature_c: 17.0,
                inside_face_temperature_delta_c: 1.0,
                oracle_inside_convection_coefficient_w_per_m2_k: 2.0,
                rust_inside_convection_coefficient_w_per_m2_k: 3.5,
                inside_convection_coefficient_delta_w_per_m2_k: 1.5,
                oracle_inside_conduction_w: 12.0,
                rust_inside_conduction_w: 10.0,
                inside_conduction_delta_w: 2.0,
                oracle_inside_convection_w: -5.0,
                rust_inside_convection_w: -4.0,
                inside_convection_delta_w: 1.0,
                oracle_inside_net_longwave_w: -3.0,
                rust_inside_net_longwave_w: -2.0,
                inside_net_longwave_delta_w: 1.0,
                oracle_inside_balance_residual_w: 4.0,
                rust_inside_balance_residual_w: 4.0,
                inside_balance_residual_delta_w: 0.0,
            }],
            inside_solve_max_sample_deltas: vec![super::HeatBalanceInsideSolveMaxSampleDelta {
                key: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                outside_boundary_condition: "adiabatic".to_string(),
                sample_index: 1,
                area_m2: 100.0,
                ctf_inside_0_w_per_m2_k: 4.0,
                ctf_cross_0_w_per_m2_k: 1.0,
                iter_damp_w_per_m2_k: 5.0,
                oracle_inside_face_temperature_c: 16.0,
                rust_inside_face_temperature_c: 17.0,
                inside_face_temperature_delta_c: 1.0,
                oracle_inferred_reference_air_temperature_c: 13.5,
                rust_inferred_reference_air_temperature_c: 15.8,
                inferred_reference_air_temperature_delta_c: 2.3,
                oracle_solve_denominator_w_per_m2_k: 8.0,
                rust_solve_denominator_w_per_m2_k: 9.5,
                solve_denominator_delta_w_per_m2_k: 1.5,
                oracle_implied_solve_numerator_w: 12800.0,
                rust_implied_solve_numerator_w: 16150.0,
                implied_solve_numerator_delta_w: 3350.0,
                oracle_reference_air_source_w: 2700.0,
                rust_reference_air_source_w: 5530.0,
                reference_air_source_delta_w: 2830.0,
                oracle_outside_temperature_source_w: 0.0,
                rust_outside_temperature_source_w: 0.0,
                outside_temperature_source_delta_w: 0.0,
                oracle_inside_history_term_w: 6.0,
                rust_inside_history_term_w: 3.0,
                inside_history_delta_w: 3.0,
                rust_inside_history_temperature_term_w: 2.5,
                rust_inside_history_flux_term_w: 0.5,
                oracle_inside_net_longwave_w: -3.0,
                rust_inside_net_longwave_w: -2.0,
                inside_net_longwave_delta_w: 1.0,
            }],
            adiabatic_history_max_sample_deltas: vec![
                super::HeatBalanceAdiabaticHistoryMaxSampleDelta {
                    key: "FLOOR".to_string(),
                    construction_name: "FLOOR".to_string(),
                    sample_index: 1,
                    area_m2: 100.0,
                    ctf_inside_0_w_per_m2_k: 4.0,
                    ctf_cross_0_w_per_m2_k: 1.0,
                    oracle_inside_face_temperature_c: 16.0,
                    rust_inside_face_temperature_c: 17.0,
                    inside_face_temperature_delta_c: 1.0,
                    oracle_outside_face_temperature_c: 15.0,
                    rust_outside_face_temperature_c: 14.0,
                    outside_face_temperature_delta_c: 1.0,
                    oracle_outside_minus_inside_c: -1.0,
                    rust_outside_minus_inside_c: -3.0,
                    outside_minus_inside_delta_c: 2.0,
                    oracle_inside_current_term_w: -4900.0,
                    rust_inside_current_term_w: -5400.0,
                    inside_current_delta_w: 500.0,
                    oracle_inside_current_if_outside_synced_w: -4800.0,
                    rust_inside_current_if_outside_synced_w: -5100.0,
                    inside_current_if_outside_synced_delta_w: 300.0,
                    oracle_inside_current_sync_shift_w: 100.0,
                    rust_inside_current_sync_shift_w: 300.0,
                    oracle_inside_history_term_w: 4912.0,
                    rust_inside_history_term_w: 5410.0,
                    inside_history_delta_w: 498.0,
                    oracle_inside_history_if_outside_synced_w: 4812.0,
                    rust_inside_history_if_outside_synced_w: 5110.0,
                    inside_history_if_outside_synced_delta_w: 298.0,
                },
            ],
            ctf_history_run_period_initial_slots: vec![super::HeatBalanceCtfHistorySlotSample {
                surface_name: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                slot_index: 1,
                area_m2: 100.0,
                outside_history_coefficient_w_per_m2_k: 1.0,
                cross_history_coefficient_w_per_m2_k: 2.0,
                inside_history_coefficient_w_per_m2_k: 3.0,
                flux_history_coefficient: 0.5,
                outside_temperature_history_c: 9.0,
                inside_temperature_history_c: 19.0,
                outside_flux_history_w_per_m2: -2.0,
                inside_flux_history_w_per_m2: 2.0,
                inside_temperature_term_w: -3900.0,
                inside_flux_term_w: 100.0,
                inside_total_term_w: -3800.0,
                outside_temperature_term_w: 2900.0,
                outside_flux_term_w: 100.0,
                outside_total_term_w: 3000.0,
            }],
            ctf_history_first_sample_slots: vec![super::HeatBalanceCtfHistorySlotFirstSample {
                surface_name: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                slot_index: 1,
                area_m2: 100.0,
                timestep_count: 4,
                outside_history_coefficient_w_per_m2_k: 1.0,
                cross_history_coefficient_w_per_m2_k: 2.0,
                inside_history_coefficient_w_per_m2_k: 3.0,
                flux_history_coefficient: 0.5,
                outside_temperature_history_c: 10.0,
                inside_temperature_history_c: 20.0,
                outside_flux_history_w_per_m2: -1.0,
                inside_flux_history_w_per_m2: 1.0,
                inside_temperature_term_w: -4000.0,
                inside_flux_term_w: 50.0,
                inside_total_term_w: -3950.0,
                outside_temperature_term_w: 3000.0,
                outside_flux_term_w: 50.0,
                outside_total_term_w: 3050.0,
            }],
            surface_first_sample_trace: vec![super::HeatBalanceSurfaceFirstSampleTrace {
                surface_name: "FLOOR".to_string(),
                construction_name: "FLOOR".to_string(),
                timestep_index: 1,
                outdoor_dry_bulb_c: 10.0,
                zone_mean_air_temperature_c: 23.0,
                inside_face_temperature_c: 22.0,
                outside_face_temperature_c: 11.0,
                inside_convection_heat_gain_rate_w: 1.0,
                inside_net_surface_thermal_radiation_heat_gain_rate_w: 2.0,
                inside_conduction_rate_w: -2.0,
                outside_conduction_rate_w: 1.0,
                heat_storage_rate_w: 1.0,
                outside_convection_heat_gain_rate_w: -3.0,
                outside_net_thermal_radiation_heat_gain_rate_w: -4.0,
                outside_solar_radiation_heat_gain_rate_w: 5.0,
            }],
            series: vec![super::HeatBalanceSeriesDiagnostic {
                output: super::ZoneTemperatureReportOutput {
                    key: "ZONE ONE".to_string(),
                    variable: "Zone Mean Air Temperature".to_string(),
                    frequency: "hourly",
                    class: "zone-state",
                    source: "eso",
                },
                samples: 1,
                oracle_first_c: 1.0,
                rust_first_c: 2.0,
                oracle_last_c: 1.0,
                rust_last_c: 2.0,
                delta: super::delta_summary(&[1.0], &[2.0]),
                sample_rows: super::delta_points(&[1.0], &[2.0]),
                status: "extracted",
            }],
            status: "extracted",
        };
        let context = super::HeatBalanceConformanceContext {
            case_id: "official_1zone_uncontrolled_dynamic_diagnostic_001".to_string(),
            oracle_version: "26.1.0".to_string(),
            outputs: diagnostic
                .series
                .iter()
                .map(|series| series.output.clone())
                .collect(),
            tolerances: vec![super::HeatBalanceToleranceReport {
                variable_class_label: "zone-state",
                max_abs_c: Some(0.000001),
                max_rmse_c: Some(0.000001),
                max_rel: None,
            }],
            report: None,
            gate: None,
            comparison_class: "diagnostic-only",
            conformance_claim: false,
        };
        let comparison = super::evaluate_heat_balance_conformance(&diagnostic, &context);

        let json = super::render_heat_balance_conformance_summary_json(&diagnostic, &comparison);
        let digest = super::render_heat_balance_conformance_digest_json(&diagnostic, &comparison);
        let report = super::render_heat_balance_conformance_report(&diagnostic, &comparison);

        assert_eq!(comparison.status, "fail");
        assert!(json.contains("\"comparison_class\": \"diagnostic-only\""));
        assert!(json.contains("\"conformance_claim\": false"));
        assert!(json.contains("\"oracle_run_period_day_count\": 20"));
        assert!(json.contains("\"day_count_delta\": -14"));
        assert!(json.contains("\"policy\": \"steady-no-mass-only\""));
        assert!(json.contains("\"construction_summaries\""));
        assert!(json.contains("\"timestep_hours\": 0.250000000000"));
        assert!(json.contains("\"included\": false"));
        assert!(json.contains("\"zone_air_algorithm\": \"energyplus-third-order-probe\""));
        assert!(json.contains("\"surface_iteration_count\": 3"));
        assert!(json.contains("\"ctf_initial_history_policy\": \"energyplus-surf-initial\""));
        assert!(json.contains("\"construction_name\": \"FLOOR\""));
        assert!(json.contains("\"bottlenecks\""));
        assert!(json.contains("\"first_sample_bottlenecks\""));
        assert!(json.contains("\"surface_first_sample_trace\""));
        assert!(json.contains("\"ctf_component_first_samples\""));
        assert!(json.contains("\"ctf_history_first_sample_deltas\""));
        assert!(json.contains("\"inside_current_delta_w\""));
        assert!(json.contains("\"ctf_history_series_deltas\""));
        assert!(json.contains("\"outside_history_delta\""));
        assert!(json.contains("\"ctf_storage_max_sample_deltas\""));
        assert!(json.contains("\"outside_history_delta_w\""));
        assert!(json.contains("\"inside_balance_max_sample_deltas\""));
        assert!(json.contains("\"inside_balance_residual_delta_w\""));
        assert!(json.contains("\"inside_solve_max_sample_deltas\""));
        assert!(json.contains("\"implied_solve_numerator_delta_w\""));
        assert!(json.contains("\"rust_inside_history_temperature_term_w\""));
        assert!(json.contains("\"adiabatic_history_max_sample_deltas\""));
        assert!(json.contains("\"outside_minus_inside_delta_c\""));
        assert!(json.contains("\"ctf_history_run_period_initial_slots\""));
        assert!(json.contains("\"ctf_history_first_sample_slots\""));
        assert!(json.contains("\"first_sample_delta\""));
        assert!(json.contains("\"max_delta_sample\""));
        assert!(json.contains("\"max_sample_contexts\""));
        assert!(json.contains("\"trigger_output\""));
        assert!(digest.contains("\"comparison_class\": \"diagnostic-only\""));
        assert!(digest.contains("\"construction_summaries\""));
        assert!(digest.contains("\"construction_name\": \"FLOOR\""));
        assert!(digest.contains("\"bottlenecks\""));
        assert!(digest.contains("\"max_sample_contexts\""));
        assert!(digest.contains("\"first_sample_bottlenecks\""));
        assert!(digest.contains("\"surface_first_sample_trace\""));
        assert!(digest.contains("\"ctf_component_first_samples\""));
        assert!(digest.contains("\"ctf_history_first_sample_deltas\""));
        assert!(digest.contains("\"inside_current_delta_w\""));
        assert!(digest.contains("\"ctf_history_series_deltas\""));
        assert!(digest.contains("\"outside_history_delta\""));
        assert!(digest.contains("\"ctf_storage_max_sample_deltas\""));
        assert!(digest.contains("\"outside_history_delta_w\""));
        assert!(digest.contains("\"inside_balance_max_sample_deltas\""));
        assert!(digest.contains("\"inside_balance_residual_delta_w\""));
        assert!(digest.contains("\"inside_solve_max_sample_deltas\""));
        assert!(digest.contains("\"implied_solve_numerator_delta_w\""));
        assert!(digest.contains("\"rust_inside_history_flux_term_w\""));
        assert!(digest.contains("\"adiabatic_history_max_sample_deltas\""));
        assert!(digest.contains("\"inside_current_if_outside_synced_delta_w\""));
        assert!(digest.contains("\"ctf_history_run_period_initial_slots\""));
        assert!(digest.contains("\"ctf_history_first_sample_slots\""));
        assert!(digest.contains("\"first_sample_delta\""));
        assert!(digest.contains("\"first_delta_sample\""));
        assert!(digest.contains("\"max_delta_sample\""));
        assert!(digest.contains("\"series\""));
        assert!(!digest.contains("\"sample_rows\""));
        assert!(report.contains("Heat Balance Diagnostic Report"));
        assert!(report.contains("Rust Surface First-Sample Trace"));
        assert!(report.contains("comparison_class: diagnostic-only"));
        assert!(report.contains("conformance_claim: false"));
        assert!(report.contains("oracle_run_period_warmup_days: 20"));
        assert!(report.contains("warmup_day_count_delta: -14"));
        assert!(report.contains("ctf_seed_policy: steady-no-mass-only"));
        assert!(report.contains("zone_air_algorithm: energyplus-third-order-probe"));
        assert!(report.contains("surface_iteration_count: 3"));
        assert!(report.contains("ctf_initial_history_policy: energyplus-surf-initial"));
        assert!(report.contains("ctf_seed_included_constructions: R13WALL, ROOF31"));
        assert!(report.contains("ctf_seed_skipped_constructions: FLOOR (#CTFs=5)"));
        assert!(
            report.contains("ctf_seed_construction_summaries: R13WALL (#CTFs=1) @ dt=0.250h [included], FLOOR (#CTFs=5) @ dt=0.250h [skipped]")
        );
        assert!(report.contains("## Bottlenecks"));
        assert!(report.contains("## Max-Sample Contexts"));
        assert!(report.contains("## First-Sample Bottlenecks"));
        assert!(report.contains("## Rust CTF First-Sample Components"));
        assert!(report.contains("## CTF History First-Sample Deltas"));
        assert!(report.contains("## CTF History Series Deltas"));
        assert!(report.contains("out_history_rmse_w"));
        assert!(report.contains("## CTF Storage Max-Sample Deltas"));
        assert!(report.contains("out_history_delta_w"));
        assert!(report.contains("## Inside Balance Max-Sample Deltas"));
        assert!(report.contains("residual_delta_w"));
        assert!(report.contains("## Inside Solve Max-Sample Deltas"));
        assert!(report.contains("implied_numerator_delta_w"));
        assert!(report.contains("rust_history_temp_w"));
        assert!(report.contains("## Adiabatic History Max-Sample Deltas"));
        assert!(report.contains("out_minus_in_delta_c"));
        assert!(report.contains("## Rust CTF History Run-Period Initial Slots"));
        assert!(report.contains("## Rust CTF History First-Sample Slots"));
        assert!(report.contains("status: fail"));
    }

    #[test]
    fn heat_balance_uses_run_period_eso_samples_when_design_days_exist() {
        let series = ep_compare::EsoTimeSeries {
            metadata: ep_compare::EsoSeriesMetadata {
                id: "7".to_string(),
                key: "ZONE ONE".to_string(),
                variable: "Zone Mean Air Temperature".to_string(),
                units: Some("C".to_string()),
                frequency: Some("Hourly".to_string()),
            },
            samples: vec![
                ep_compare::SeriesSample::timestamped(
                    0,
                    "env=CHICAGO ANN HTG 99.6% CONDNS DB;day=1;month=1;date=21;dst=0;hour=1;start=0.00;end=60.00;day_type=WinterDesignDay",
                    -17.0,
                ),
                ep_compare::SeriesSample::timestamped(
                    1,
                    "env=RUN PERIOD 1;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=60.00;day_type=Tuesday",
                    23.0,
                ),
                ep_compare::SeriesSample::timestamped(
                    2,
                    "env=RUN PERIOD 1;day=1;month=1;date=1;dst=0;hour=2;start=0.00;end=60.00;day_type=Tuesday",
                    24.0,
                ),
            ],
        };

        assert_eq!(super::run_period_eso_values(&series), vec![23.0, 24.0]);
    }

    #[test]
    fn seed_coverage_reports_tracked_objects() {
        assert_eq!(super::seed_coverage_status("Version"), "tracked");
        assert_eq!(super::seed_coverage_status("Output:Variable"), "untracked");
        assert_eq!(
            super::seed_coverage_status("ZoneHVAC:IdealLoadsAirSystem"),
            "tracked"
        );
    }
}
