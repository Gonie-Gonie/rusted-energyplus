//! Command line entry point for eplus-rs.

use ep_compare::{Tolerance, compare_series, load_eso_series};
use ep_compiler::{CompileReport, DiagnosticSeverity, compile_raw_model};
use ep_conformance::{
    ComparisonClass, ConformanceCase, OutputFrequency, OutputRegistry, VariableClass,
    load_case_file,
};
use ep_model::{SimulationModel, TypedModel};
use ep_oracle::default_oracle_release;
use ep_raw_model::{RawModelSummary, load_epjson_file};
use ep_runtime::{
    ExecutionPlan, FirstZoneSimulationOptions, SimulationMode, ZoneGeometrySummary,
    build_execution_plan, build_hourly_time_axis, load_epw_dry_bulb_series,
    simulate_constant_schedules, simulate_first_zone_uncontrolled, zone_geometry_summaries,
};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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
    println!("  compare weather-drybulb <weather.epw> <eplusout.eso>");
    println!("  compare zone-temperature <input.epJSON> <weather.epw> <eplusout.eso>");
    println!("  conformance validate-case <case.toml>");
    println!("  conformance baseline <case.toml> <oracle-root> <output-root>");
    println!("  conformance report-skeleton <case.toml> <baseline-case-dir> <report-root>");
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
        Some(command) => {
            eprintln!("unsupported conformance command: {command}");
            eprintln!("usage: eplus-rs conformance validate-case <case.toml>");
            eprintln!(
                "usage: eplus-rs conformance baseline <case.toml> <oracle-root> <output-root>"
            );
            eprintln!(
                "usage: eplus-rs conformance report-skeleton <case.toml> <baseline-case-dir> <report-root>"
            );
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

struct BaselineSummary {
    output_dir: PathBuf,
    idf: PathBuf,
    weather: Option<PathBuf>,
    epjson: PathBuf,
    eso: PathBuf,
}

struct ReportSkeletonSummary {
    report_path: PathBuf,
    series: usize,
}

fn generate_conformance_baseline(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
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

    let output_dir = output_root.join(&manifest.id);
    std::fs::create_dir_all(&output_dir)
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
        .arg(&output_dir)
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
        .current_dir(&output_dir)
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
    let epjson = output_dir.join("input.epJSON");
    if !epjson.is_file() {
        return Err(format!("IDF converter did not write {}", epjson.display()));
    }

    Ok(BaselineSummary {
        output_dir,
        idf: input_idf,
        weather: source_weather,
        epjson,
        eso,
    })
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

    let registry = OutputRegistry::from_case(manifest)
        .map_err(|error| format!("invalid registry: {error}"))?;
    let mut rows = Vec::new();
    for output in registry.series() {
        let values = load_eso_series(&eso, &output.key, &output.variable)
            .map_err(|error| format!("failed to load baseline series: {error}"))?;
        rows.push(ReportSeriesRow {
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output_frequency_label(output.frequency),
            variable_class: variable_class_label(output.class),
            samples: values.len(),
            first: first_value_label(&values),
            last: last_value_label(&values),
        });
    }

    let report = render_report_skeleton(manifest, &rows);
    std::fs::write(&report_path, report)
        .map_err(|error| format!("failed to write report skeleton: {error}"))?;

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
        "| key | variable | frequency | class | baseline_samples | first | last | status |\n",
    );
    report.push_str("|---|---|---|---|---:|---:|---:|---|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | baseline-only |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.variable),
            row.frequency,
            row.variable_class,
            row.samples,
            row.first,
            row.last
        ));
    }
    report
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
            "    {} / {} / {} / {}",
            output.key,
            output.variable,
            output_frequency_label(output.frequency),
            variable_class_label(output.class)
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
        VariableClass::ZoneState => "zone-state",
        VariableClass::SurfaceState => "surface-state",
        VariableClass::Meter => "meter",
        VariableClass::InternalVariable => "internal-variable",
        VariableClass::Diagnostic => "diagnostic",
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
        Some("weather-drybulb") => run_compare_weather_drybulb(&args[1..]),
        Some("zone-temperature") => run_compare_zone_temperature(&args[1..]),
        Some(command) => {
            eprintln!("unsupported compare command: {command}");
            eprintln!("usage: eplus-rs compare schedule-value <input.epJSON> <eplusout.eso>");
            eprintln!("usage: eplus-rs compare weather-drybulb <weather.epw> <eplusout.eso>");
            eprintln!(
                "usage: eplus-rs compare zone-temperature <input.epJSON> <weather.epw> <eplusout.eso>"
            );
            2
        }
        None => {
            eprintln!("missing compare command");
            eprintln!("usage: eplus-rs compare schedule-value <input.epJSON> <eplusout.eso>");
            eprintln!("usage: eplus-rs compare weather-drybulb <weather.epw> <eplusout.eso>");
            eprintln!(
                "usage: eplus-rs compare zone-temperature <input.epJSON> <weather.epw> <eplusout.eso>"
            );
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

fn run_compare_zone_temperature(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!(
            "usage: eplus-rs compare zone-temperature <input.epJSON> <weather.epw> <eplusout.eso>"
        );
        return 2;
    };
    let Some(weather_path) = args.get(1) else {
        eprintln!("missing weather path");
        eprintln!(
            "usage: eplus-rs compare zone-temperature <input.epJSON> <weather.epw> <eplusout.eso>"
        );
        return 2;
    };
    let Some(eso_path) = args.get(2) else {
        eprintln!("missing eplusout.eso path");
        eprintln!(
            "usage: eplus-rs compare zone-temperature <input.epJSON> <weather.epw> <eplusout.eso>"
        );
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
    let Some(zone) = model.zones.first() else {
        eprintln!("no Zone objects are available for comparison");
        return 1;
    };

    let oracle_values = match load_eso_series(eso_path, &zone.name.0, "Zone Mean Air Temperature") {
        Ok(values) => values,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if oracle_values.is_empty() {
        eprintln!("EnergyPlus zone temperature series is empty");
        return 1;
    }

    let weather_values = match load_epw_dry_bulb_series(weather_path) {
        Ok(values) => values,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if weather_values.len() < oracle_values.len() {
        eprintln!(
            "EPW dry-bulb series has {} samples but ESO requires {}",
            weather_values.len(),
            oracle_values.len()
        );
        return 1;
    }

    let simulation_model = SimulationModel::from_typed(model);
    let simulation = match simulate_first_zone_uncontrolled(
        &simulation_model,
        &weather_values,
        FirstZoneSimulationOptions::hourly_samples(oracle_values.len()),
    ) {
        Ok(simulation) => simulation,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let Some(rust_series) = simulation
        .results
        .find_series(&simulation.summary.zone_name, "Zone Mean Air Temperature")
    else {
        eprintln!("first-zone simulation did not write zone temperature output");
        return 1;
    };

    let (samples, max_abs_delta, mean_abs_delta) =
        delta_summary(&oracle_values, &rust_series.values);
    let finite = oracle_values
        .iter()
        .chain(rust_series.values.iter())
        .all(|value| value.is_finite());
    let extracted = finite && samples == oracle_values.len() && samples == rust_series.values.len();

    println!("Zone Temperature Diagnostic");
    println!("  comparison_class: diagnostic-only");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: none");
    println!("  zone: {}", simulation.summary.zone_name);
    println!("  samples: {samples}");
    println!("  max_abs_delta: {max_abs_delta:.6}");
    println!("  mean_abs_delta: {mean_abs_delta:.6}");
    println!("  oracle_first_c: {:.6}", oracle_values[0]);
    println!("  rust_first_c: {:.6}", rust_series.values[0]);
    println!(
        "  oracle_last_c: {:.6}",
        oracle_values[oracle_values.len() - 1]
    );
    println!(
        "  rust_last_c: {:.6}",
        rust_series.values[rust_series.values.len() - 1]
    );
    println!("  exact_match: not_available");
    println!("  exit_code_semantics: extraction-only");
    println!(
        "  status: {}",
        if extracted { "extracted" } else { "failed" }
    );

    if extracted { 0 } else { 1 }
}

fn run_compare_weather_drybulb(args: &[String]) -> i32 {
    let Some(epw_path) = args.first() else {
        eprintln!("missing weather path");
        eprintln!("usage: eplus-rs compare weather-drybulb <weather.epw> <eplusout.eso>");
        return 2;
    };
    let Some(eso_path) = args.get(1) else {
        eprintln!("missing eplusout.eso path");
        eprintln!("usage: eplus-rs compare weather-drybulb <weather.epw> <eplusout.eso>");
        return 2;
    };

    let oracle_values = match load_eso_series(
        eso_path,
        "Environment",
        "Site Outdoor Air Drybulb Temperature",
    ) {
        Ok(values) => values,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let weather_values = match load_epw_dry_bulb_series(epw_path) {
        Ok(values) => values,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if weather_values.len() < oracle_values.len() {
        eprintln!(
            "EPW dry-bulb series has {} samples but ESO requires {}",
            weather_values.len(),
            oracle_values.len()
        );
        return 1;
    }

    let comparison = compare_series(
        &oracle_values,
        &weather_values[..oracle_values.len()],
        Tolerance::default(),
    );

    println!("Weather Drybulb Comparison");
    println!("  samples: {}", comparison.samples);
    println!("  max_abs_delta: {}", comparison.max_abs_delta);
    print_first_divergence("  ", comparison.first_divergence);
    println!(
        "  status: {}",
        if comparison.passed { "pass" } else { "fail" }
    );

    if comparison.passed { 0 } else { 1 }
}

fn delta_summary(expected: &[f64], observed: &[f64]) -> (usize, f64, f64) {
    let samples = expected.len().min(observed.len());
    if samples == 0 {
        return (0, 0.0, 0.0);
    }

    let mut max_abs_delta: f64 = 0.0;
    let mut sum_abs_delta = 0.0;
    for (expected, observed) in expected.iter().zip(observed).take(samples) {
        let delta = (expected - observed).abs();
        max_abs_delta = max_abs_delta.max(delta);
        sum_abs_delta += delta;
    }

    (samples, max_abs_delta, sum_abs_delta / samples as f64)
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
    fn seed_coverage_reports_tracked_objects() {
        assert_eq!(super::seed_coverage_status("Version"), "tracked");
        assert_eq!(super::seed_coverage_status("Output:Variable"), "untracked");
    }
}
