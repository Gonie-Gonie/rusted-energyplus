//! Command line entry point for eplus-rs.

use ep_compare::{
    Tolerance, compare_series, load_eio_construction_ctf, load_eio_heat_transfer_surfaces,
    load_eio_material_ctf_summary, load_eio_other_equipment_nominal, load_eio_zone_geometry,
    load_eso_series,
};
use ep_compiler::{CompileReport, DiagnosticSeverity, compile_raw_model};
use ep_conformance::{
    ComparisonClass, ConformanceCase, OutputFrequency, OutputRegistry, ReportFormat,
    SourceArtifact, ToleranceRule, VariableClass, load_case_file,
};
use ep_model::{
    Construction, Material, OtherEquipment, ScheduleId, SimulationModel, SurfaceType, TypedModel,
};
use ep_oracle::default_oracle_release;
use ep_raw_model::{RawModelSummary, load_epjson_file};
use ep_runtime::{
    ExecutionPlan, FirstZoneSimulationOptions, HeatBalanceSimulationOptions, SimulationMode,
    SurfaceGeometrySummary, ZoneGeometrySummary, build_execution_plan, build_hourly_time_axis,
    load_epw_dry_bulb_series, load_epw_records, simulate_constant_schedules,
    simulate_first_zone_uncontrolled, simulate_heat_balance_zone_air_temperatures,
    simulate_zone_internal_convective_gains, surface_geometry_summaries, zone_geometry_summaries,
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
    println!("  run_periods: {}", model.typed.run_periods.len());
    println!("  zone_surface_edges: {}", model.graph.zone_surfaces.len());
    println!(
        "  construction_material_edges: {}",
        model.graph.construction_materials.len()
    );
    println!("  stages: {}", plan.stages.len());
    println!("  steps: {}", plan.step_count());
    for stage in &plan.stages {
        println!("    {}: {}", stage.name, stage.steps.len());
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
        Some("baseline") => run_conformance_baseline(&args[1..]),
        Some("report-skeleton") => run_conformance_report_skeleton(&args[1..]),
        Some("diagnostic-report") => run_conformance_diagnostic_report(&args[1..]),
        Some("heat-balance-report") => run_conformance_heat_balance_report(&args[1..]),
        Some(command) => {
            eprintln!("unsupported conformance command: {command}");
            eprintln!("usage: eplus-rs conformance validate-case <case.toml>");
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
    let report_context = zone_temperature_conformance_context_from_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");

    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let weather = baseline
        .weather
        .as_ref()
        .ok_or_else(|| "heat-balance conformance requires input.weather".to_string())?;
    let diagnostic = build_zone_temperature_diagnostic(&baseline.epjson, weather, &baseline.eso)?;
    let conformance = evaluate_zone_temperature_conformance(&diagnostic, &report_context);
    write_zone_temperature_conformance_report(&compare_dir, &diagnostic, &conformance)?;

    Ok(HeatBalanceReportSummary {
        baseline,
        report_dir: compare_dir.clone(),
        compare_report: compare_dir.join("compare-report.md"),
        compare_summary: compare_dir.join("compare-summary.json"),
        samples: diagnostic.samples,
        tolerance_policy: report_context.tolerance.label(),
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

fn validate_zone_temperature_conformance_manifest(
    manifest: &ConformanceCase,
) -> Result<(), String> {
    if manifest.comparison_class != ComparisonClass::Conformance {
        return Err(format!(
            "heat-balance conformance requires comparison_class conformance, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if !manifest.conformance_claim {
        return Err("heat-balance conformance requires conformance_claim=true".to_string());
    }
    if manifest.outputs.len() != 1 {
        return Err(format!(
            "heat-balance conformance requires exactly one output request, got {}",
            manifest.outputs.len()
        ));
    }

    let output = &manifest.outputs[0];
    if !output
        .variable
        .eq_ignore_ascii_case("Zone Mean Air Temperature")
    {
        return Err(format!(
            "heat-balance conformance requires Zone Mean Air Temperature, got {}",
            output.variable
        ));
    }
    if output.frequency != OutputFrequency::Hourly {
        return Err(format!(
            "heat-balance conformance requires hourly output, got {}",
            output_frequency_label(output.frequency)
        ));
    }
    if output.class != VariableClass::ZoneState {
        return Err(format!(
            "heat-balance conformance requires zone-state class, got {}",
            variable_class_label(output.class)
        ));
    }
    if output.source != SourceArtifact::Eso {
        return Err(format!(
            "heat-balance conformance requires eso source, got {}",
            source_artifact_label(output.source)
        ));
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

    zone_temperature_tolerance_from_manifest(manifest)?;

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
        });
    }

    let report = render_report_skeleton(manifest, &rows);
    std::fs::write(&report_path, report)
        .map_err(|error| format!("failed to write report skeleton: {error}"))?;
    std::fs::write(
        &summary_path,
        render_report_skeleton_summary_json(manifest, &rows),
    )
    .map_err(|error| format!("failed to write report summary: {error}"))?;

    Ok(ReportSkeletonSummary {
        report_path,
        series: rows.len(),
    })
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
}

fn render_report_skeleton(manifest: &ConformanceCase, rows: &[ReportSeriesRow]) -> String {
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
    report.push_str("## Series\n\n");
    report.push_str(
        "| key | variable | frequency | class | source | baseline_samples | first | last | status |\n",
    );
    report.push_str("|---|---|---|---|---|---:|---:|---:|---|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | baseline-only |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.variable),
            row.frequency,
            row.variable_class,
            row.source,
            row.samples,
            row.first,
            row.last
        ));
    }
    report
}

fn render_report_skeleton_summary_json(
    manifest: &ConformanceCase,
    rows: &[ReportSeriesRow],
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

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|")
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
        Some(command) => {
            eprintln!("unsupported run command: {command}");
            eprintln!("usage: eplus-rs run first-zone <input.epJSON> <weather.epw> [--hours N]");
            2
        }
        None => {
            eprintln!("missing run command");
            eprintln!("usage: eplus-rs run first-zone <input.epJSON> <weather.epw> [--hours N]");
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct ZoneTemperatureToleranceReport {
    variable_class: &'static str,
    max_abs_c: Option<f64>,
    max_rmse_c: Option<f64>,
    max_rel: Option<f64>,
}

impl ZoneTemperatureToleranceReport {
    fn label(self) -> String {
        let max_abs = optional_tolerance_label(self.max_abs_c);
        let max_rmse = optional_tolerance_label(self.max_rmse_c);
        let max_rel = optional_tolerance_label(self.max_rel);
        format!(
            "{} max_abs={} max_rmse={} max_rel={}",
            self.variable_class, max_abs, max_rmse, max_rel
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ZoneTemperatureConformanceContext {
    case_id: String,
    oracle_version: String,
    output: ZoneTemperatureReportOutput,
    tolerance: ZoneTemperatureToleranceReport,
    report: Option<ZoneTemperatureReportContract>,
    gate: Option<ZoneTemperatureGateContract>,
}

#[derive(Clone, Debug, PartialEq)]
struct ZoneTemperatureConformance<'a> {
    context: &'a ZoneTemperatureConformanceContext,
    status: &'static str,
    failure_reasons: Vec<String>,
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

fn zone_temperature_conformance_context_from_manifest(
    manifest: &ConformanceCase,
) -> Result<ZoneTemperatureConformanceContext, String> {
    validate_zone_temperature_conformance_manifest(manifest)?;
    let output = manifest
        .outputs
        .first()
        .ok_or_else(|| "heat-balance conformance requires one output request".to_string())?;
    let tolerance = zone_temperature_tolerance_from_manifest(manifest)?;

    Ok(ZoneTemperatureConformanceContext {
        case_id: manifest.id.clone(),
        oracle_version: manifest.oracle_version.clone(),
        output: ZoneTemperatureReportOutput {
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output_frequency_label(output.frequency),
            class: variable_class_label(output.class),
            source: source_artifact_label(output.source),
        },
        tolerance,
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

fn zone_temperature_tolerance_from_manifest(
    manifest: &ConformanceCase,
) -> Result<ZoneTemperatureToleranceReport, String> {
    let tolerance = manifest
        .tolerances
        .iter()
        .find(|tolerance| tolerance.variable_class == VariableClass::ZoneState)
        .ok_or_else(|| "heat-balance conformance requires a zone-state tolerance".to_string())?;

    Ok(zone_temperature_tolerance_report(*tolerance))
}

fn zone_temperature_tolerance_report(tolerance: ToleranceRule) -> ZoneTemperatureToleranceReport {
    ZoneTemperatureToleranceReport {
        variable_class: variable_class_label(tolerance.variable_class),
        max_abs_c: tolerance.max_abs,
        max_rmse_c: tolerance.max_rmse,
        max_rel: tolerance.max_rel,
    }
}

fn evaluate_zone_temperature_conformance<'a>(
    diagnostic: &ZoneTemperatureDiagnostic,
    context: &'a ZoneTemperatureConformanceContext,
) -> ZoneTemperatureConformance<'a> {
    let mut failure_reasons = Vec::new();
    if diagnostic.status != "extracted" {
        failure_reasons.push(format!(
            "diagnostic extraction status was {}",
            diagnostic.status
        ));
    }
    if !diagnostic.delta.length_match {
        failure_reasons.push("series length mismatch".to_string());
    }
    if let Some(max_abs_c) = context.tolerance.max_abs_c
        && diagnostic.delta.max_abs_delta_c > max_abs_c
    {
        failure_reasons.push(format!(
            "max_abs_delta_c {:.12} exceeds {:.12}",
            diagnostic.delta.max_abs_delta_c, max_abs_c
        ));
    }
    if let Some(max_rmse_c) = context.tolerance.max_rmse_c
        && diagnostic.delta.rmse_delta_c > max_rmse_c
    {
        failure_reasons.push(format!(
            "rmse_delta_c {:.12} exceeds {:.12}",
            diagnostic.delta.rmse_delta_c, max_rmse_c
        ));
    }
    if let Some(max_rel) = context.tolerance.max_rel
        && diagnostic.delta.max_rel_delta > max_rel
    {
        failure_reasons.push(format!(
            "max_rel_delta {:.12} exceeds {:.12}",
            diagnostic.delta.max_rel_delta, max_rel
        ));
    }

    ZoneTemperatureConformance {
        context,
        status: if failure_reasons.is_empty() {
            "pass"
        } else {
            "fail"
        },
        failure_reasons,
    }
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

fn write_zone_temperature_conformance_report(
    report_dir: &Path,
    diagnostic: &ZoneTemperatureDiagnostic,
    conformance: &ZoneTemperatureConformance<'_>,
) -> Result<(), String> {
    std::fs::create_dir_all(report_dir)
        .map_err(|error| format!("failed to create report directory: {error}"))?;

    let summary_path = report_dir.join("compare-summary.json");
    let report_path = report_dir.join("compare-report.md");

    std::fs::write(
        &summary_path,
        render_zone_temperature_conformance_summary_json(diagnostic, conformance),
    )
    .map_err(|error| format!("failed to write heat-balance summary: {error}"))?;
    std::fs::write(
        &report_path,
        render_zone_temperature_conformance_report(diagnostic, conformance),
    )
    .map_err(|error| format!("failed to write heat-balance report: {error}"))?;

    Ok(())
}

fn render_zone_temperature_conformance_summary_json(
    diagnostic: &ZoneTemperatureDiagnostic,
    conformance: &ZoneTemperatureConformance<'_>,
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
    json.push_str("  \"comparison_class\": \"conformance\",\n");
    json.push_str("  \"conformance_claim\": true,\n");
    json.push_str(&format!(
        "  \"tolerance_policy\": {},\n",
        zone_temperature_tolerance_json(context.tolerance)
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

fn render_zone_temperature_conformance_report(
    diagnostic: &ZoneTemperatureDiagnostic,
    conformance: &ZoneTemperatureConformance<'_>,
) -> String {
    let context = conformance.context;
    let mut report = String::new();
    report.push_str("# Heat Balance Conformance Report\n\n");
    report.push_str("## Manifest\n\n");
    report.push_str(&format!("case_id: {}\n", context.case_id));
    report.push_str(&format!("oracle_version: {}\n", context.oracle_version));
    report.push_str(&format!("output_key: {}\n", context.output.key));
    report.push_str(&format!("output_variable: {}\n", context.output.variable));
    report.push_str(&format!("output_frequency: {}\n", context.output.frequency));
    report.push_str(&format!("output_class: {}\n", context.output.class));
    report.push_str(&format!("output_source: {}\n", context.output.source));
    report.push_str(&format!(
        "tolerance_policy: {}\n",
        context.tolerance.label()
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
        "max_abs_delta_c: {:.12}\n",
        diagnostic.delta.max_abs_delta_c
    ));
    report.push_str(&format!(
        "mean_abs_delta_c: {:.12}\n",
        diagnostic.delta.mean_abs_delta_c
    ));
    report.push_str(&format!(
        "rmse_delta_c: {:.12}\n",
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

fn zone_temperature_tolerance_json(tolerance: ZoneTemperatureToleranceReport) -> String {
    format!(
        "{{ \"variable_class\": {}, \"max_abs_c\": {}, \"max_rmse_c\": {}, \"max_rel\": {} }}",
        json_string(tolerance.variable_class),
        json_optional_number(tolerance.max_abs_c),
        json_optional_number(tolerance.max_rmse_c),
        json_optional_number(tolerance.max_rel)
    )
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
    fn zone_temperature_conformance_report_records_pass_and_tolerance() {
        let diagnostic = super::ZoneTemperatureDiagnostic {
            zone_name: "ZONE ONE".to_string(),
            samples: 2,
            heat_balance_timesteps: 8,
            zone_count: 1,
            surface_count: 6,
            oracle_first_c: 23.0,
            rust_first_c: 23.0,
            oracle_last_c: 23.0,
            rust_last_c: 23.0,
            delta: super::delta_summary(&[23.0, 23.0], &[23.0, 23.0]),
            status: "extracted",
        };
        let context = super::ZoneTemperatureConformanceContext {
            case_id: "heat_balance_nomass_001".to_string(),
            oracle_version: "26.1.0".to_string(),
            output: super::ZoneTemperatureReportOutput {
                key: "ZONE ONE".to_string(),
                variable: "Zone Mean Air Temperature".to_string(),
                frequency: "hourly",
                class: "zone-state",
                source: "eso",
            },
            tolerance: super::ZoneTemperatureToleranceReport {
                variable_class: "zone-state",
                max_abs_c: Some(0.000001),
                max_rmse_c: Some(0.000001),
                max_rel: None,
            },
            report: Some(super::ZoneTemperatureReportContract {
                format: "markdown",
                path: ".runtime/heat-balance-conformance/report.md".to_string(),
            }),
            gate: Some(super::ZoneTemperatureGateContract {
                script: "scripts/dev.cmd compare-heat-balance-conformance".to_string(),
                blocking: true,
            }),
        };
        let conformance = super::evaluate_zone_temperature_conformance(&diagnostic, &context);

        let json =
            super::render_zone_temperature_conformance_summary_json(&diagnostic, &conformance);
        let report = super::render_zone_temperature_conformance_report(&diagnostic, &conformance);

        assert_eq!(conformance.status, "pass");
        assert!(json.contains("\"case_id\": \"heat_balance_nomass_001\""));
        assert!(json.contains("\"comparison_class\": \"conformance\""));
        assert!(json.contains("\"conformance_claim\": true"));
        assert!(json.contains("\"status\": \"pass\""));
        assert!(json.contains("\"max_abs_c\": 0.000001000000"));
        assert!(json.contains("\"blocking\": true"));
        assert!(report.contains("Heat Balance Conformance Report"));
        assert!(report.contains("comparison_class: conformance"));
        assert!(report.contains("conformance_claim: true"));
        assert!(report.contains("status: pass"));
        assert!(report.contains("failure_reasons: none"));
        assert!(report.contains("gate_blocking: true"));
    }

    #[test]
    fn seed_coverage_reports_tracked_objects() {
        assert_eq!(super::seed_coverage_status("Version"), "tracked");
        assert_eq!(super::seed_coverage_status("Output:Variable"), "untracked");
    }
}
