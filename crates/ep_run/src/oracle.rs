//! EnergyPlus oracle discovery, IDF conversion, and baseline execution.

use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use ep_oracle::default_oracle_release;
use serde::Serialize;

/// Input format accepted by the arbitrary-run pipeline.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OracleInputKind {
    /// EnergyPlus IDF input.
    Idf,
    /// EnergyPlus epJSON input.
    EpJson,
}

impl OracleInputKind {
    /// Infers input kind from a path extension.
    #[must_use]
    pub fn from_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_string_lossy();
        if extension.eq_ignore_ascii_case("idf") {
            Some(Self::Idf)
        } else if extension.eq_ignore_ascii_case("epjson") || extension.eq_ignore_ascii_case("json")
        {
            Some(Self::EpJson)
        } else {
            None
        }
    }

    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Idf => "idf",
            Self::EpJson => "epjson",
        }
    }
}

/// Discovered EnergyPlus oracle executable paths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OraclePaths {
    /// Oracle root directory.
    pub root: PathBuf,
    /// `energyplus.exe`.
    pub energyplus_exe: PathBuf,
    /// `ConvertInputFormat.exe`.
    pub convert_input_format_exe: PathBuf,
}

/// One output variable request to inject into an oracle IDF.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OracleOutputRequest {
    /// EnergyPlus key value.
    pub key: String,
    /// EnergyPlus output variable name.
    pub variable_name: String,
    /// Reporting frequency.
    pub frequency: String,
}

impl OracleOutputRequest {
    /// Creates an hourly request.
    #[must_use]
    pub fn hourly(key: impl Into<String>, variable_name: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            variable_name: variable_name.into(),
            frequency: "Hourly".to_string(),
        }
    }
}

/// EnergyPlus oracle baseline timing and artifacts.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OracleBaselineSummary {
    /// EnergyPlus oracle version.
    pub oracle_version: String,
    /// Oracle output directory.
    pub output_dir: String,
    /// Staged input path.
    pub input_path: String,
    /// Staged weather path, when present.
    pub weather_path: Option<String>,
    /// EnergyPlus ERR path.
    pub err_path: String,
    /// EnergyPlus ESO path.
    pub eso_path: String,
    /// EnergyPlus EIO path.
    pub eio_path: String,
    /// Number of output variable requests injected into IDF input.
    pub injected_output_requests: usize,
    /// EnergyPlus process wall-clock seconds.
    pub energyplus_wall_seconds: f64,
    /// Total baseline staging and execution wall-clock seconds.
    pub total_wall_seconds: f64,
}

/// Error produced by oracle operations.
#[derive(Debug)]
pub struct OracleError {
    message: String,
}

impl OracleError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for OracleError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for OracleError {}

/// Resolves the EnergyPlus oracle root from override, environment, package, or repo-local paths.
pub fn resolve_oracle_paths(override_root: Option<&Path>) -> Result<OraclePaths, OracleError> {
    let release = default_oracle_release();
    let mut candidates = Vec::new();
    if let Some(root) = override_root {
        candidates.push(root.to_path_buf());
    }
    if let Ok(root) = std::env::var("RUSTED_ENERGYPLUS_ORACLE_ROOT") {
        candidates.push(PathBuf::from(root));
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(bin_dir) = exe.parent()
        && let Some(package_root) = bin_dir.parent()
    {
        candidates.push(
            package_root
                .join("oracle")
                .join("energyplus")
                .join(release.version),
        );
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(
            cwd.join(".runtime")
                .join("energyplus")
                .join(release.version),
        );
        candidates.push(cwd.join("oracle").join("energyplus").join(release.version));
    }

    for root in candidates {
        let energyplus_exe = root.join("energyplus.exe");
        let convert_input_format_exe = root.join("ConvertInputFormat.exe");
        if energyplus_exe.is_file() && convert_input_format_exe.is_file() {
            return Ok(OraclePaths {
                root,
                energyplus_exe,
                convert_input_format_exe,
            });
        }
    }

    Err(OracleError::new(
        "missing EnergyPlus oracle root; set --oracle-root or RUSTED_ENERGYPLUS_ORACLE_ROOT",
    ))
}

/// Converts one staged IDF to epJSON using `ConvertInputFormat.exe`.
pub fn convert_idf_to_epjson(
    converter: &Path,
    idf_path: &Path,
    destination_epjson: &Path,
) -> Result<f64, OracleError> {
    let parent = idf_path
        .parent()
        .ok_or_else(|| OracleError::new("IDF input has no parent directory"))?;
    let file_name = idf_path
        .file_name()
        .ok_or_else(|| OracleError::new("IDF input has no file name"))?;
    let generated = idf_path.with_extension("epJSON");
    if generated.is_file() {
        std::fs::remove_file(&generated).map_err(|error| {
            OracleError::new(format!(
                "failed to remove previous converted epJSON {}: {error}",
                generated.display()
            ))
        })?;
    }

    let start = Instant::now();
    let output = Command::new(converter)
        .arg(file_name)
        .current_dir(parent)
        .output()
        .map_err(|error| OracleError::new(format!("failed to start IDF converter: {error}")))?;
    let seconds = start.elapsed().as_secs_f64();
    if !output.status.success() {
        return Err(OracleError::new(command_failure_message(
            "IDF conversion",
            &output,
            None,
        )));
    }
    if !generated.is_file() {
        return Err(OracleError::new(format!(
            "IDF converter did not write {}",
            generated.display()
        )));
    }
    if let Some(parent) = destination_epjson.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            OracleError::new(format!(
                "failed to create converted epJSON directory {}: {error}",
                parent.display()
            ))
        })?;
    }
    std::fs::copy(&generated, destination_epjson).map_err(|error| {
        OracleError::new(format!(
            "failed to stage converted epJSON {}: {error}",
            destination_epjson.display()
        ))
    })?;
    Ok(seconds)
}

/// Runs an EnergyPlus oracle baseline under the provided output directory.
pub fn run_oracle_baseline(
    oracle_paths: &OraclePaths,
    source_input: &Path,
    input_kind: OracleInputKind,
    weather_path: Option<&Path>,
    output_dir: &Path,
    output_requests: &[OracleOutputRequest],
) -> Result<OracleBaselineSummary, OracleError> {
    let total_start = Instant::now();
    std::fs::create_dir_all(output_dir).map_err(|error| {
        OracleError::new(format!(
            "failed to create oracle output directory {}: {error}",
            output_dir.display()
        ))
    })?;

    let staged_input = match input_kind {
        OracleInputKind::Idf => {
            let staged = output_dir.join("input.idf");
            let mut idf = std::fs::read_to_string(source_input).map_err(|error| {
                OracleError::new(format!(
                    "failed to read IDF for oracle staging {}: {error}",
                    source_input.display()
                ))
            })?;
            let injection = render_output_request_injection(output_requests);
            if !injection.is_empty() {
                if !idf.ends_with('\n') {
                    idf.push('\n');
                }
                idf.push_str(&injection);
            }
            std::fs::write(&staged, idf).map_err(|error| {
                OracleError::new(format!(
                    "failed to write staged oracle IDF {}: {error}",
                    staged.display()
                ))
            })?;
            staged
        }
        OracleInputKind::EpJson => {
            let staged = output_dir.join("input.epJSON");
            std::fs::copy(source_input, &staged).map_err(|error| {
                OracleError::new(format!(
                    "failed to stage oracle epJSON {}: {error}",
                    staged.display()
                ))
            })?;
            staged
        }
    };

    let staged_weather = match weather_path {
        Some(weather_path) => {
            let staged = output_dir.join("weather.epw");
            std::fs::copy(weather_path, &staged).map_err(|error| {
                OracleError::new(format!(
                    "failed to stage oracle weather {}: {error}",
                    staged.display()
                ))
            })?;
            Some(staged)
        }
        None => None,
    };

    let mut command = Command::new(&oracle_paths.energyplus_exe);
    command.current_dir(output_dir);
    if staged_weather.is_some() {
        command.arg("-w").arg("weather.epw");
    }
    command
        .arg("-d")
        .arg(".")
        .arg(staged_input_file_name(&staged_input)?);

    let energyplus_start = Instant::now();
    let command_output = command
        .output()
        .map_err(|error| OracleError::new(format!("failed to start EnergyPlus: {error}")))?;
    let energyplus_wall_seconds = energyplus_start.elapsed().as_secs_f64();
    write_oracle_command_log(output_dir, &command_output, energyplus_wall_seconds)?;

    let err_path = output_dir.join("eplusout.err");
    if !command_output.status.success() {
        return Err(OracleError::new(command_failure_message(
            "EnergyPlus oracle baseline",
            &command_output,
            Some(&err_path),
        )));
    }

    let eso_path = output_dir.join("eplusout.eso");
    let eio_path = output_dir.join("eplusout.eio");
    for path in [&err_path, &eso_path, &eio_path] {
        if !path.is_file() {
            return Err(OracleError::new(format!(
                "EnergyPlus oracle did not write {}",
                path.display()
            )));
        }
    }

    let release = default_oracle_release();
    Ok(OracleBaselineSummary {
        oracle_version: release.version.to_string(),
        output_dir: output_dir.display().to_string(),
        input_path: staged_input.display().to_string(),
        weather_path: staged_weather.map(|path| path.display().to_string()),
        err_path: err_path.display().to_string(),
        eso_path: eso_path.display().to_string(),
        eio_path: eio_path.display().to_string(),
        injected_output_requests: output_requests.len(),
        energyplus_wall_seconds,
        total_wall_seconds: total_start.elapsed().as_secs_f64(),
    })
}

fn staged_input_file_name(path: &Path) -> Result<&std::ffi::OsStr, OracleError> {
    path.file_name()
        .ok_or_else(|| OracleError::new("staged oracle input has no file name"))
}

fn render_output_request_injection(output_requests: &[OracleOutputRequest]) -> String {
    if output_requests.is_empty() {
        return String::new();
    }

    let mut idf = String::new();
    idf.push_str("\n!- eplus-rs arbitrary-run oracle output injection begin\n");
    idf.push_str("Output:VariableDictionary,Regular;\n\n");
    let mut seen = BTreeSet::new();
    for request in output_requests {
        let identity = format!(
            "{}\n{}\n{}",
            request.key.to_ascii_lowercase(),
            request.variable_name.to_ascii_lowercase(),
            request.frequency.to_ascii_lowercase()
        );
        if !seen.insert(identity) {
            continue;
        }
        idf.push_str("Output:Variable,\n");
        idf.push_str(&format!("  {},  !- Key Value\n", idf_field(&request.key)));
        idf.push_str(&format!(
            "  {},  !- Variable Name\n",
            idf_field(&request.variable_name)
        ));
        idf.push_str(&format!(
            "  {};  !- Reporting Frequency\n\n",
            idf_field(&request.frequency)
        ));
    }
    idf.push_str("!- eplus-rs arbitrary-run oracle output injection end\n");
    idf
}

fn idf_field(value: &str) -> String {
    value
        .replace(['\r', '\n'], " ")
        .replace([';', ','], " ")
        .trim()
        .to_string()
}

fn write_oracle_command_log(
    output_dir: &Path,
    output: &std::process::Output,
    seconds: f64,
) -> Result<(), OracleError> {
    let log = format!(
        "command: energyplus -d . <input>\nwall_seconds: {seconds:.9}\nstatus: {}\n\nstdout:\n{}\n\nstderr:\n{}\n",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::write(output_dir.join("command.log"), log)
        .map_err(|error| OracleError::new(format!("failed to write oracle command log: {error}")))
}

fn command_failure_message(
    label: &str,
    output: &std::process::Output,
    err_path: Option<&Path>,
) -> String {
    let mut message = format!("{label} failed with status {}", output.status);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.trim().is_empty() {
        message.push_str(&format!("; stderr: {}", stderr.trim()));
    }
    if let Some(err_path) = err_path
        && let Ok(err_text) = std::fs::read_to_string(err_path)
    {
        let tail = err_text
            .lines()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
        if !tail.trim().is_empty() {
            message.push_str(&format!("; eplusout.err tail:\n{tail}"));
        }
    }
    message
}

use std::collections::BTreeSet;
