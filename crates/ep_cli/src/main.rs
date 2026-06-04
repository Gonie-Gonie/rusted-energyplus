//! Command line entry point for eplus-rs.

use ep_compiler::{CompileReport, DiagnosticSeverity, compile_raw_model};
use ep_model::TypedModel;
use ep_oracle::default_oracle_release;
use ep_raw_model::{RawModelSummary, load_epjson_file};
use ep_runtime::SimulationMode;

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
        Some(command) => {
            eprintln!("unsupported model command: {command}");
            eprintln!("usage: eplus-rs model inspect <input.epJSON>");
            eprintln!("usage: eplus-rs model compile <input.epJSON>");
            2
        }
        None => {
            eprintln!("missing model command");
            eprintln!("usage: eplus-rs model inspect <input.epJSON>");
            eprintln!("usage: eplus-rs model compile <input.epJSON>");
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

fn print_help() {
    println!("eplus-rs");
    println!();
    println!("Commands:");
    println!("  oracle-info   print locked EnergyPlus oracle metadata");
    println!("  modes         print planned simulation modes");
    println!("  model inspect <input.epJSON>");
    println!("  model compile <input.epJSON>");
    println!("  compile <input.epJSON>");
    println!();
    println!("Future commands:");
    println!("  model validate <input.epJSON>");
    println!("  graph validate <input.epJSON>");
    println!("  run <input.epJSON>");
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
    println!("  zones: {}", model.zones.len());
    println!("  surfaces: {}", model.surfaces.len());
    println!("  diagnostics: {}", report.diagnostics.len());
    println!("  defaults_applied: {}", report.defaults_applied.len());
}

fn print_compile_diagnostics(report: &CompileReport) {
    println!("Compile diagnostics");
    println!("  raw_objects: {}", report.raw_object_count);
    println!("  typed_objects: {}", report.typed_object_count);
    println!("  diagnostics: {}", report.diagnostics.len());
    println!("  defaults_applied: {}", report.defaults_applied.len());
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
    fn seed_coverage_reports_tracked_objects() {
        assert_eq!(super::seed_coverage_status("Version"), "tracked");
        assert_eq!(super::seed_coverage_status("Output:Variable"), "untracked");
    }
}
