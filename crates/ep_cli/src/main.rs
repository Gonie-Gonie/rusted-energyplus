//! Command line entry point for eplus-rs.

use ep_compare::{
    Tolerance, compare_series, load_eio_construction_ctf, load_eio_heat_transfer_surfaces,
    load_eio_material_ctf_summary, load_eio_other_equipment_nominal, load_eio_zone_geometry,
    load_eso_series,
};
use ep_compiler::{CompileReport, DiagnosticSeverity, compile_raw_model};
use ep_conformance::{
    CaseSourceKind, CaseTier, ComparisonClass, ConformanceCase, EvidenceDomain, OutputFrequency,
    OutputLevel, OutputRegistry, ReportFormat, SourceArtifact, ToleranceRule, VariableClass,
    load_case_file, load_case_v2_file,
};
use ep_model::{
    Construction, Material, OtherEquipment, ScheduleId, SimulationModel, SurfaceType, TypedModel,
};
use ep_oracle::default_oracle_release;
use ep_raw_model::{RawModelSummary, load_epjson_file};
use ep_runtime::{
    ExecutionPlan, ExecutionStep, FirstZoneSimulationOptions, HeatBalanceSimulationOptions,
    NodeStateProjection, NodeStateProjectionOptions, PlantStateProjection,
    PlantStateProjectionOptions, SimulationMode, SurfaceGeometrySummary, ZoneGeometrySummary,
    build_execution_plan, build_hourly_time_axis, load_epw_dry_bulb_series, load_epw_records,
    simulate_constant_schedules, simulate_first_zone_uncontrolled,
    simulate_heat_balance_zone_air_temperatures, simulate_ideal_loads_node_state_projection,
    simulate_plant_state_projection, simulate_zone_internal_convective_gains,
    surface_geometry_summaries, zone_geometry_summaries,
};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const ZONE_TEMPERATURE_COMPARE_USAGE: &str = "usage: eplus-rs compare zone-temperature <input.epJSON> <weather.epw> <eplusout.eso> [--report-dir DIR]";
const CONFORMANCE_DIAGNOSTIC_REPORT_USAGE: &str =
    "usage: eplus-rs conformance diagnostic-report <case.toml> <oracle-root> <output-root>";
const CONFORMANCE_HEAT_BALANCE_REPORT_USAGE: &str =
    "usage: eplus-rs conformance heat-balance-report <case.toml> <oracle-root> <output-root>";

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
            println!("  samples: {}", summary.samples);
            println!("  tolerance_policy: {}", summary.tolerance_policy);
            println!("  status: {}", summary.status);
            if summary.status == "pass" { 0 } else { 1 }
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

struct BaselineSummary {
    output_dir: PathBuf,
    idf: PathBuf,
    weather: Option<PathBuf>,
    epjson: PathBuf,
    eso: PathBuf,
    expanded_manifest: PathBuf,
}

struct ReportSkeletonSummary {
    report_path: PathBuf,
    series: usize,
    warning_count: usize,
    severe_count: usize,
    fatal_count: usize,
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
    samples: usize,
    tolerance_policy: String,
    status: &'static str,
}

fn generate_conformance_baseline(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<BaselineSummary, String> {
    generate_conformance_baseline_in_dir(
        case_path,
        manifest,
        oracle_root,
        &output_root.join(&manifest.id),
    )
}

fn generate_conformance_baseline_in_dir(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_dir: &Path,
) -> Result<BaselineSummary, String> {
    let energyplus = oracle_root.join("energyplus.exe");
    if !energyplus.is_file() {
        return Err(format!(
            "missing EnergyPlus executable: {}",
            energyplus.display()
        ));
    }
    let converter = oracle_root.join("ConvertInputFormat.exe");
    if !converter.is_file() {
        return Err(format!("missing IDF converter: {}", converter.display()));
    }

    let source_idf = resolve_manifest_path(case_path, &manifest.input.idf)
        .map_err(|error| format!("failed to resolve input.idf: {error}"))?;
    if !source_idf.is_file() {
        return Err(format!("missing case IDF: {}", source_idf.display()));
    }
    let source_weather = match manifest.input.weather.as_deref() {
        Some(weather) => {
            let resolved = resolve_manifest_path(case_path, weather)
                .map_err(|error| format!("failed to resolve input.weather: {error}"))?;
            if !resolved.is_file() {
                return Err(format!("missing case weather: {}", resolved.display()));
            }
            Some(resolved)
        }
        None => None,
    };

    std::fs::create_dir_all(output_dir)
        .map_err(|error| format!("failed to create baseline output directory: {error}"))?;
    let input_idf = output_dir.join("input.idf");
    std::fs::copy(&source_idf, &input_idf)
        .map_err(|error| format!("failed to stage case IDF: {error}"))?;

    let mut energyplus_command = Command::new(&energyplus);
    if let Some(weather) = source_weather.as_ref() {
        energyplus_command.arg("-w").arg(weather);
    }
    energyplus_command
        .arg("-d")
        .arg(output_dir)
        .arg(&input_idf)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let energyplus_status = energyplus_command
        .status()
        .map_err(|error| format!("failed to start EnergyPlus: {error}"))?;
    if !energyplus_status.success() {
        return Err(format!(
            "EnergyPlus baseline failed with status {energyplus_status}"
        ));
    }

    let converter_status = Command::new(&converter)
        .arg("input.idf")
        .current_dir(output_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("failed to start IDF converter: {error}"))?;
    if !converter_status.success() {
        return Err(format!(
            "IDF conversion failed with status {converter_status}"
        ));
    }

    let eso = output_dir.join("eplusout.eso");
    if !eso.is_file() {
        return Err(format!("EnergyPlus did not write {}", eso.display()));
    }
    let err = output_dir.join("eplusout.err");
    if !err.is_file() {
        return Err(format!("EnergyPlus did not write {}", err.display()));
    }
    let epjson = output_dir.join("input.epJSON");
    if !epjson.is_file() {
        return Err(format!("IDF converter did not write {}", epjson.display()));
    }
    let expanded_manifest = output_dir.join("case-expanded.toml");
    std::fs::write(
        &expanded_manifest,
        render_expanded_case_manifest(manifest, source_weather.as_deref()),
    )
    .map_err(|error| format!("failed to write expanded case manifest: {error}"))?;

    Ok(BaselineSummary {
        output_dir: output_dir.to_path_buf(),
        idf: input_idf,
        weather: source_weather,
        epjson,
        eso,
        expanded_manifest,
    })
}

fn render_expanded_case_manifest(
    manifest: &ConformanceCase,
    source_weather: Option<&Path>,
) -> String {
    let mut toml = String::new();
    toml.push_str("# Generated by eplus-rs conformance baseline.\n");
    toml.push_str("schema = \"rusted-energyplus.baseline-expanded.v1\"\n");
    push_toml_string_field(&mut toml, "id", &manifest.id);
    push_toml_string_field(&mut toml, "title", &manifest.title);
    push_toml_string_field(&mut toml, "milestone", &manifest.milestone);
    push_toml_string_field(
        &mut toml,
        "comparison_class",
        comparison_class_label(manifest.comparison_class),
    );
    toml.push_str(&format!(
        "conformance_claim = {}\n",
        manifest.conformance_claim
    ));
    push_toml_string_field(&mut toml, "oracle_version", &manifest.oracle_version);
    toml.push('\n');

    toml.push_str("[input]\n");
    push_toml_string_field(&mut toml, "source_idf", &manifest.input.idf);
    push_toml_string_field(&mut toml, "staged_idf", "input.idf");
    if let Some(weather) = source_weather {
        push_toml_string_field(&mut toml, "source_weather", &weather.display().to_string());
    }
    push_toml_string_field(&mut toml, "converted_epjson", "input.epJSON");
    toml.push('\n');

    toml.push_str("[artifacts]\n");
    push_toml_string_field(&mut toml, "err", "eplusout.err");
    push_toml_string_field(&mut toml, "eso", "eplusout.eso");
    push_toml_string_field(&mut toml, "eio", "eplusout.eio");
    push_toml_string_field(&mut toml, "rdd", "eplusout.rdd");
    push_toml_string_field(&mut toml, "mdd", "eplusout.mdd");
    push_toml_string_field(&mut toml, "expanded_manifest", "case-expanded.toml");
    toml.push('\n');

    for output in &manifest.outputs {
        toml.push_str("[[outputs]]\n");
        push_toml_string_field(&mut toml, "key", &output.key);
        push_toml_string_field(&mut toml, "variable", &output.variable);
        push_toml_string_field(
            &mut toml,
            "frequency",
            output_frequency_label(output.frequency),
        );
        push_toml_string_field(&mut toml, "class", variable_class_label(output.class));
        push_toml_string_field(&mut toml, "source", source_artifact_label(output.source));
        toml.push('\n');
    }

    if let Some(report) = manifest.report.as_ref() {
        toml.push_str("[report]\n");
        push_toml_string_field(&mut toml, "format", report_format_label(report.format));
        push_toml_string_field(&mut toml, "path", &report.path);
        toml.push('\n');
    }
    if let Some(gate) = manifest.gate.as_ref() {
        toml.push_str("[gate]\n");
        push_toml_string_field(&mut toml, "script", &gate.script);
        toml.push_str(&format!("blocking = {}\n", gate.blocking));
    }

    toml
}

fn push_toml_string_field(output: &mut String, key: &str, value: &str) {
    output.push_str(key);
    output.push_str(" = ");
    output.push_str(&json_string(value));
    output.push('\n');
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
        samples: diagnostic.samples,
        tolerance_policy: report_context.tolerance_label(),
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

fn generate_conformance_report_skeleton(
    manifest: &ConformanceCase,
    baseline_case_dir: &Path,
    report_root: &Path,
) -> Result<ReportSkeletonSummary, String> {
    let eso = baseline_case_dir.join("eplusout.eso");
    if !eso.is_file() {
        return Err(format!("missing baseline ESO: {}", eso.display()));
    }
    let err = baseline_case_dir.join("eplusout.err");
    if !err.is_file() {
        return Err(format!("missing baseline ERR: {}", err.display()));
    }
    let warning_summary = read_energyplus_err_summary(&err)?;

    let report_dir = report_root.join(&manifest.id);
    std::fs::create_dir_all(&report_dir)
        .map_err(|error| format!("failed to create report directory: {error}"))?;
    let report_path = report_dir.join("compare-report.md");
    let summary_path = report_dir.join("compare-summary.json");

    let registry = OutputRegistry::from_case(manifest)
        .map_err(|error| format!("invalid registry: {error}"))?;
    let mut rows = Vec::new();
    for output in registry.series() {
        if output.source != SourceArtifact::Eso {
            return Err(format!(
                "report skeleton currently supports eso output sources, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
        let values = load_eso_series(&eso, &output.key, &output.variable)
            .map_err(|error| format!("failed to load baseline series: {error}"))?;
        rows.push(ReportSeriesRow {
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output_frequency_label(output.frequency),
            variable_class: variable_class_label(output.class),
            source: source_artifact_label(output.source),
            samples: values.len(),
            first: first_value_label(&values),
            last: last_value_label(&values),
            min: min_value_label(&values),
            max: max_value_label(&values),
            nonzero_count: nonzero_count(&values),
        });
    }

    let report = render_report_skeleton(manifest, &rows, &warning_summary);
    std::fs::write(&report_path, report)
        .map_err(|error| format!("failed to write report skeleton: {error}"))?;
    std::fs::write(
        &summary_path,
        render_report_skeleton_summary_json(manifest, &rows, &warning_summary),
    )
    .map_err(|error| format!("failed to write report summary: {error}"))?;

    Ok(ReportSkeletonSummary {
        report_path,
        series: rows.len(),
        warning_count: warning_summary.warning_count,
        severe_count: warning_summary.severe_count,
        fatal_count: warning_summary.fatal_count,
    })
}

struct EnergyPlusErrSummary {
    warning_count: usize,
    severe_count: usize,
    fatal_count: usize,
    warnings: Vec<String>,
}

struct ReportSeriesRow {
    key: String,
    variable: String,
    frequency: &'static str,
    variable_class: &'static str,
    source: &'static str,
    samples: usize,
    first: String,
    last: String,
    min: String,
    max: String,
    nonzero_count: usize,
}

fn render_report_skeleton(
    manifest: &ConformanceCase,
    rows: &[ReportSeriesRow],
    warning_summary: &EnergyPlusErrSummary,
) -> String {
    let mut report = String::new();
    report.push_str("# Conformance Report Skeleton\n\n");
    report.push_str(&format!("case_id: {}\n", manifest.id));
    report.push_str(&format!(
        "comparison_class: {}\n",
        comparison_class_label(manifest.comparison_class)
    ));
    report.push_str(&format!(
        "conformance_claim: {}\n",
        manifest.conformance_claim
    ));
    report.push_str(&format!("oracle_version: {}\n", manifest.oracle_version));
    report.push_str("tolerance_policy: none\n");
    report.push_str("status: baseline-only\n\n");
    report.push_str("## EnergyPlus ERR\n\n");
    report.push_str(&format!(
        "energyplus_warnings: {}\n",
        warning_summary.warning_count
    ));
    report.push_str(&format!(
        "energyplus_severes: {}\n",
        warning_summary.severe_count
    ));
    report.push_str(&format!(
        "energyplus_fatals: {}\n\n",
        warning_summary.fatal_count
    ));
    if !warning_summary.warnings.is_empty() {
        report.push_str("| index | warning |\n");
        report.push_str("|---:|---|\n");
        for (index, warning) in warning_summary.warnings.iter().enumerate() {
            report.push_str(&format!("| {} | {} |\n", index + 1, markdown_cell(warning)));
        }
        report.push('\n');
    }
    report.push_str("## Series\n\n");
    report.push_str(
        "| key | variable | frequency | class | source | baseline_samples | first | last | baseline_min | baseline_max | baseline_nonzero_count | status |\n",
    );
    report.push_str("|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | baseline-only |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.variable),
            row.frequency,
            row.variable_class,
            row.source,
            row.samples,
            row.first,
            row.last,
            row.min,
            row.max,
            row.nonzero_count
        ));
    }
    report
}

fn render_report_skeleton_summary_json(
    manifest: &ConformanceCase,
    rows: &[ReportSeriesRow],
    warning_summary: &EnergyPlusErrSummary,
) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!("  \"case_id\": {},\n", json_string(&manifest.id)));
    json.push_str(&format!(
        "  \"oracle_version\": {},\n",
        json_string(&manifest.oracle_version)
    ));
    json.push_str(&format!(
        "  \"comparison_class\": {},\n",
        json_string(comparison_class_label(manifest.comparison_class))
    ));
    json.push_str(&format!(
        "  \"conformance_claim\": {},\n",
        manifest.conformance_claim
    ));
    json.push_str("  \"tolerance_policy\": \"none\",\n");
    json.push_str("  \"status\": \"baseline-only\",\n");
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"compare_report_md\": \"compare-report.md\",\n");
    json.push_str("    \"compare_summary_json\": \"compare-summary.json\"\n");
    json.push_str("  },\n");
    json.push_str("  \"energyplus_err\": {\n");
    json.push_str(&format!(
        "    \"warnings\": {},\n",
        warning_summary.warning_count
    ));
    json.push_str(&format!(
        "    \"severes\": {},\n",
        warning_summary.severe_count
    ));
    json.push_str(&format!(
        "    \"fatals\": {},\n",
        warning_summary.fatal_count
    ));
    json.push_str("    \"warning_messages\": [");
    for (index, warning) in warning_summary.warnings.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&json_string(warning));
    }
    json.push_str("]\n");
    json.push_str("  },\n");
    json.push_str("  \"requested_outputs\": [\n");
    for (index, row) in rows.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"key\": {},\n", json_string(&row.key)));
        json.push_str(&format!(
            "      \"variable\": {},\n",
            json_string(&row.variable)
        ));
        json.push_str(&format!(
            "      \"frequency\": {},\n",
            json_string(row.frequency)
        ));
        json.push_str(&format!(
            "      \"class\": {},\n",
            json_string(row.variable_class)
        ));
        json.push_str(&format!("      \"source\": {},\n", json_string(row.source)));
        json.push_str(&format!("      \"baseline_samples\": {},\n", row.samples));
        json.push_str(&format!("      \"first\": {},\n", json_string(&row.first)));
        json.push_str(&format!("      \"last\": {},\n", json_string(&row.last)));
        json.push_str(&format!(
            "      \"baseline_min\": {},\n",
            json_string(&row.min)
        ));
        json.push_str(&format!(
            "      \"baseline_max\": {},\n",
            json_string(&row.max)
        ));
        json.push_str(&format!(
            "      \"baseline_nonzero_count\": {},\n",
            row.nonzero_count
        ));
        json.push_str("      \"status\": \"baseline-only\"\n");
        if index + 1 == rows.len() {
            json.push_str("    }\n");
        } else {
            json.push_str("    },\n");
        }
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn read_energyplus_err_summary(path: &Path) -> Result<EnergyPlusErrSummary, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read EnergyPlus ERR: {error}"))?;
    Ok(energyplus_err_summary(&contents))
}

fn energyplus_err_summary(contents: &str) -> EnergyPlusErrSummary {
    let mut warnings = Vec::new();
    let mut severe_count = 0;
    let mut fatal_count = 0;

    for line in contents.lines() {
        if line.contains("** Warning **") {
            warnings.push(clean_energyplus_message(line));
        } else if line.contains("** Severe  **") || line.contains("** Severe **") {
            severe_count += 1;
        } else if line.contains("** Fatal  **") || line.contains("** Fatal **") {
            fatal_count += 1;
        }
    }

    EnergyPlusErrSummary {
        warning_count: warnings.len(),
        severe_count,
        fatal_count,
        warnings,
    }
}

fn clean_energyplus_message(line: &str) -> String {
    line.replace("** Warning **", "").trim().to_string()
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
        CaseSourceKind::EnergyPlusExamplefile => "energyplus-examplefile",
        CaseSourceKind::EnergyPlusTestfile => "energyplus-testfile",
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

    let rust_material_count = rust_rows
        .iter()
        .map(|row| row.outside_layer_material_name.as_str())
        .collect::<BTreeSet<_>>()
        .len();
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

fn run_compare_internal_convective_gain(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs compare internal-convective-gain <input.epJSON> <eplusout.eso>");
        return 2;
    };
    let Some(eso_path) = args.get(1) else {
        eprintln!("missing eplusout.eso path");
        eprintln!("usage: eplus-rs compare internal-convective-gain <input.epJSON> <eplusout.eso>");
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
    if model.zones.is_empty() {
        eprintln!("no Zone objects are available for internal-convective-gain comparison");
        return 1;
    }

    let mut oracle_series = Vec::new();
    for zone in &model.zones {
        let values = match load_eso_series(
            eso_path,
            &zone.name.0,
            "Zone Total Internal Convective Heating Rate",
        ) {
            Ok(values) => values,
            Err(error) => {
                eprintln!("{error}");
                return 1;
            }
        };
        oracle_series.push((zone.id, zone.name.0.clone(), values));
    }

    let sample_count = oracle_series
        .iter()
        .map(|(_id, _name, values)| values.len())
        .max()
        .unwrap_or(0);
    let traces = simulate_zone_internal_convective_gains(model, sample_count);
    let mut passed = true;

    println!("Internal Convective Gain Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: default");
    println!("  zones: {}", oracle_series.len());
    for (zone_id, zone_name, expected_values) in oracle_series {
        let Some(trace) = traces.iter().find(|trace| trace.zone_id == zone_id) else {
            eprintln!("missing Rust internal convective gain trace: {zone_name}");
            return 1;
        };
        let comparison = compare_series(
            &expected_values,
            &trace.values_w[..expected_values.len()],
            Tolerance::default(),
        );
        if !comparison.passed {
            passed = false;
        }
        println!(
            "  zone: {} samples: {} max_abs_delta: {} status: {}",
            zone_name,
            comparison.samples,
            comparison.max_abs_delta,
            if comparison.passed { "pass" } else { "fail" }
        );
        print_first_divergence("  ", comparison.first_divergence);
    }
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
    status: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatBalanceConformanceDiagnostic {
    samples: usize,
    heat_balance_timesteps: usize,
    zone_count: usize,
    surface_count: usize,
    series: Vec<HeatBalanceSeriesDiagnostic>,
    status: &'static str,
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
        let values = load_eso_series(eso_path, &output.key, &output.variable)
            .map_err(|error| error.to_string())?;
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

    let weather_values =
        load_epw_dry_bulb_series(weather_path).map_err(|error| error.to_string())?;
    if weather_values.len() < sample_count {
        return Err(format!(
            "EPW dry-bulb series has {} samples but ESO requires {}",
            weather_values.len(),
            sample_count
        ));
    }

    let simulation_model = SimulationModel::from_typed(model);
    let simulation = simulate_heat_balance_zone_air_temperatures(
        &simulation_model,
        &weather_values,
        HeatBalanceSimulationOptions::hourly_samples(sample_count),
    )
    .map_err(|error| error.to_string())?;

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
            status: if extracted { "extracted" } else { "failed" },
        });
    }

    let extracted = series.iter().all(|series| series.status == "extracted");
    Ok(HeatBalanceConformanceDiagnostic {
        samples: sample_count,
        heat_balance_timesteps: simulation.summary.timestep_count,
        zone_count: simulation.summary.zone_count,
        surface_count: simulation.summary.surface_count,
        series,
        status: if extracted { "extracted" } else { "failed" },
    })
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

fn construction_material_row(
    model: &TypedModel,
    construction: &Construction,
) -> Result<ConstructionMaterialRow, String> {
    let material = material_for_construction(model, construction)?;
    let material_thermal_resistance_m2_k_per_w =
        material.thermal_resistance().ok_or_else(|| {
            format!(
                "construction {} outside layer {} has no positive thermal resistance",
                construction.name.0, material.name.0
            )
        })?;
    Ok(ConstructionMaterialRow {
        construction_name: construction.name.0.clone(),
        layer_count: 1,
        outside_layer_material_name: material.name.0.clone(),
        thermal_conductance_w_per_m2_k: 1.0 / material_thermal_resistance_m2_k_per_w,
        material_thickness_m: material.thickness_m,
        material_conductivity_w_per_m_k: material.conductivity_w_per_m_k,
        material_density_kg_per_m3: material.density_kg_per_m3,
        material_specific_heat_j_per_kg_k: material.specific_heat_j_per_kg_k,
        material_thermal_resistance_m2_k_per_w,
    })
}

fn material_for_construction<'a>(
    model: &'a TypedModel,
    construction: &Construction,
) -> Result<&'a Material, String> {
    model
        .materials
        .iter()
        .find(|material| material.id == construction.outside_layer)
        .ok_or_else(|| {
            format!(
                "construction {} references missing outside layer material",
                construction.name.0
            )
        })
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
    let report_path = report_dir.join("compare-report.md");

    std::fs::write(
        &summary_path,
        render_heat_balance_conformance_summary_json(diagnostic, conformance),
    )
    .map_err(|error| format!("failed to write heat-balance summary: {error}"))?;
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
    json.push_str("  \"comparison_class\": \"conformance\",\n");
    json.push_str("  \"conformance_claim\": true,\n");
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
        "  \"series\": {},\n",
        heat_balance_series_json(&diagnostic.series)
    ));
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"compare_report_md\": \"compare-report.md\",\n");
    json.push_str("    \"compare_summary_json\": \"compare-summary.json\"\n");
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
    report.push_str("# Heat Balance Conformance Report\n\n");
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
    report.push_str("comparison_class: conformance\n");
    report.push_str("conformance_claim: true\n");
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

    report.push_str("## Series\n\n");
    heat_balance_report_series_rows(&mut report, &diagnostic.series);
    report.push('\n');

    report.push_str("## Delta Samples\n\n");
    heat_balance_report_delta_rows(&mut report, &diagnostic.series);
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

fn heat_balance_series_json(series: &[HeatBalanceSeriesDiagnostic]) -> String {
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

fn heat_balance_report_series_rows(report: &mut String, series: &[HeatBalanceSeriesDiagnostic]) {
    report.push_str(
        "| key | variable | class | samples | max_abs_delta_c | rmse_delta_c | status |\n",
    );
    report.push_str("|---|---|---|---:|---:|---:|---|\n");
    for row in series {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {:.12} | {:.12} | {} |\n",
            markdown_cell(&row.output.key),
            markdown_cell(&row.output.variable),
            row.output.class,
            row.samples,
            row.delta.max_abs_delta_c,
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

fn json_optional_number(value: Option<f64>) -> String {
    value.map_or_else(|| "null".to_string(), json_number)
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
            zone_count: 1,
            surface_count: 6,
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
        };
        let conformance = super::evaluate_heat_balance_conformance(&diagnostic, &context);

        let json = super::render_heat_balance_conformance_summary_json(&diagnostic, &conformance);
        let report = super::render_heat_balance_conformance_report(&diagnostic, &conformance);

        assert_eq!(conformance.status, "pass");
        assert!(json.contains("\"case_id\": \"heat_balance_nomass_001\""));
        assert!(json.contains("\"comparison_class\": \"conformance\""));
        assert!(json.contains("\"conformance_claim\": true"));
        assert!(json.contains("\"status\": \"pass\""));
        assert!(json.contains("\"max_abs_c\": 0.000001000000"));
        assert!(json.contains("\"series_count\": 2"));
        assert!(json.contains("\"variable\": \"Surface Inside Face Temperature\""));
        assert!(json.contains("\"blocking\": true"));
        assert!(report.contains("Heat Balance Conformance Report"));
        assert!(report.contains("comparison_class: conformance"));
        assert!(report.contains("conformance_claim: true"));
        assert!(report.contains("status: pass"));
        assert!(report.contains("failure_reasons: none"));
        assert!(report.contains("gate_blocking: true"));
        assert!(report.contains("Surface Inside Face Temperature"));
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
