//! Command line entry point for eplus-rs.

use ep_compare::{Tolerance, compare_series, load_eso_series};
use ep_compiler::{CompileReport, DiagnosticSeverity, compile_raw_model};
use ep_model::{SimulationModel, TypedModel};
use ep_oracle::default_oracle_release;
use ep_raw_model::{RawModelSummary, load_epjson_file};
use ep_runtime::{
    ExecutionPlan, SimulationMode, build_execution_plan, load_epw_dry_bulb_series,
    simulate_constant_schedules,
};

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
        Some("compile") => run_compile_command(&args[1..]),
        Some("model") => run_model_command(&args[1..]),
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
        Some(command) => {
            eprintln!("unsupported model command: {command}");
            eprintln!("usage: eplus-rs model inspect <input.epJSON>");
            eprintln!("usage: eplus-rs model compile <input.epJSON>");
            eprintln!("usage: eplus-rs model plan <input.epJSON>");
            2
        }
        None => {
            eprintln!("missing model command");
            eprintln!("usage: eplus-rs model inspect <input.epJSON>");
            eprintln!("usage: eplus-rs model compile <input.epJSON>");
            eprintln!("usage: eplus-rs model plan <input.epJSON>");
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

fn print_help() {
    println!("eplus-rs");
    println!();
    println!("Commands:");
    println!("  oracle-info   print locked EnergyPlus oracle metadata");
    println!("  modes         print planned simulation modes");
    println!("  model inspect <input.epJSON>");
    println!("  model compile <input.epJSON>");
    println!("  model plan <input.epJSON>");
    println!("  compile <input.epJSON>");
    println!("  compare schedule-value <input.epJSON> <eplusout.eso>");
    println!("  compare weather-drybulb <weather.epw> <eplusout.eso>");
    println!();
    println!("Future commands:");
    println!("  model validate <input.epJSON>");
    println!("  graph validate <input.epJSON>");
    println!("  run <input.epJSON>");
}

fn print_plan_summary(model: &SimulationModel, plan: &ExecutionPlan) {
    println!("ExecutionPlan");
    println!("  zones: {}", model.typed.zones.len());
    println!("  surfaces: {}", model.typed.surfaces.len());
    println!("  constructions: {}", model.typed.constructions.len());
    println!("  materials: {}", model.typed.materials.len());
    println!("  other_equipment: {}", model.typed.other_equipment.len());
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

fn run_compare_command(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("schedule-value") => run_compare_schedule_value(&args[1..]),
        Some("weather-drybulb") => run_compare_weather_drybulb(&args[1..]),
        Some(command) => {
            eprintln!("unsupported compare command: {command}");
            eprintln!("usage: eplus-rs compare schedule-value <input.epJSON> <eplusout.eso>");
            eprintln!("usage: eplus-rs compare weather-drybulb <weather.epw> <eplusout.eso>");
            2
        }
        None => {
            eprintln!("missing compare command");
            eprintln!("usage: eplus-rs compare schedule-value <input.epJSON> <eplusout.eso>");
            eprintln!("usage: eplus-rs compare weather-drybulb <weather.epw> <eplusout.eso>");
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
    }
    println!("  status: {}", if passed { "pass" } else { "fail" });

    if passed { 0 } else { 1 }
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
    println!(
        "  status: {}",
        if comparison.passed { "pass" } else { "fail" }
    );

    if comparison.passed { 0 } else { 1 }
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
    println!("  site_locations: {}", usize::from(model.site.is_some()));
    println!("  materials: {}", model.materials.len());
    println!("  constructions: {}", model.constructions.len());
    println!(
        "  schedule_type_limits: {}",
        model.schedule_type_limits.len()
    );
    println!("  schedules: {}", model.schedules.len());
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
    fn seed_coverage_reports_tracked_objects() {
        assert_eq!(super::seed_coverage_status("Version"), "tracked");
        assert_eq!(super::seed_coverage_status("Output:Variable"), "untracked");
    }
}
