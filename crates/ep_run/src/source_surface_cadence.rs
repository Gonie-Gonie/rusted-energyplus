//! Optional ancillary observations around the one actual bounded Source run.
//! No numerical invocation, feature or target is added by this adapter.
use crate::{PortingScope, RunConfig, RunError, RunExitCode, RunOutcome};
use ep_runtime::heat_balance::source_surface_cadence;
use std::io::{self, BufWriter, Write};
use std::path::{Component, Path, PathBuf};

const SWITCH: &str = "RUSTED_ENERGYPLUS_SOURCE_SURFACE_CADENCE";
const RECORD_LIMIT: usize = 200_000;

fn diagnostic(message: impl Into<String>) -> RunError {
    RunError {
        exit_code: RunExitCode::OutputExport,
        message: message.into(),
    }
}

fn ancillary_path(config: &RunConfig, value: &std::ffi::OsStr) -> Result<PathBuf, RunError> {
    let relative = Path::new(value);
    let mut components = relative.components();
    if components.next() != Some(Component::Normal(std::ffi::OsStr::new("traces")))
        || relative.extension() != Some(std::ffi::OsStr::new("json"))
        || !components
            .clone()
            .all(|part| matches!(part, Component::Normal(_)))
        || components.next().is_none()
    {
        return Err(diagnostic(
            "Source surface cadence requires an internal relative traces/<name>.json path",
        ));
    }
    Ok(config.output_dir.join(relative))
}

fn export(path: &Path, trace: &source_surface_cadence::Trace) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("Source surface cadence has no ancillary parent"))?;
    std::fs::create_dir_all(parent)?;
    // Preserve any existing observation. A failed export is never a scientific PASS.
    let file = std::fs::File::options()
        .write(true)
        .create_new(true)
        .open(path)?;
    let mut writer = BufWriter::new(file);
    trace.write_json(&mut writer)?;
    writer.flush()
}

pub(crate) fn observe(
    config: &RunConfig,
    scope: Option<PortingScope>,
    call: impl FnOnce() -> Result<RunOutcome, RunError>,
) -> Result<RunOutcome, RunError> {
    let Some(value) = std::env::var_os(SWITCH) else {
        return call();
    };
    if scope.is_none() || config.dry_run || config.oracle_baseline || config.compare_oracle {
        return Err(diagnostic(
            "Source surface cadence requires the bounded normal Source run without dry-run or oracle",
        ));
    }
    let path = ancillary_path(config, &value)?;
    // Installs one bounded collector and restores the prior collector before any
    // artifact IO. The original actual pipeline is invoked exactly once.
    let (outcome, trace) = source_surface_cadence::capture(RECORD_LIMIT, call)
        .map_err(|error| diagnostic(format!("Source surface cadence capture: {error}")))?;
    let artifact = export(&path, &trace);
    match outcome {
        Err(original) => {
            // Preserve the actual pipeline failure, including its exit code.
            if let Err(error) = artifact {
                eprintln!("Source surface cadence ancillary export failed: {error}");
            }
            Err(original)
        }
        Ok(outcome) => {
            artifact.map_err(|error| {
                diagnostic(format!("Source surface cadence ancillary export: {error}"))
            })?;
            if !trace.complete() || trace.calls[..6].iter().any(|count| *count == 0) {
                return Err(diagnostic(
                    "Source surface cadence is incomplete or did not observe all six actual surface procedures",
                ));
            }
            Ok(outcome)
        }
    }
}
