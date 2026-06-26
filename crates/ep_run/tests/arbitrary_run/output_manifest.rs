//! Golden output manifests for arbitrary-run integration tests.

use std::path::Path;

pub(crate) const BLOCKED_AFTER_SUPPORT_MANIFEST: &[&str] = &[
    "diagnostics.json",
    "eplusrs.err",
    "input/converted.epJSON",
    "input/input-hashes.json",
    "input/original.epJSON",
    "logs/command.log",
    "model/raw-model-summary.json",
    "model/typed-model-summary.json",
    "reports/compatibility-boundary.md",
    "reports/run-report.md",
    "run-summary.json",
    "support-assessment.json",
    "support-report.md",
];

pub(crate) const SUPPORTED_RUNTIME_MANIFEST: &[&str] = &[
    "diagnostics.json",
    "eplusrs.err",
    "input/converted.epJSON",
    "input/input-hashes.json",
    "input/original.epJSON",
    "logs/command.log",
    "model/execution-plan.json",
    "model/graph-summary.json",
    "model/raw-model-summary.json",
    "model/typed-model-summary.json",
    "reports/compatibility-boundary.md",
    "reports/run-report.md",
    "results/meters.csv",
    "results/result-store.json",
    "results/selected-outputs.csv",
    "run-summary.json",
    "support-assessment.json",
    "support-report.md",
];

pub(crate) fn assert_output_manifest(
    output_dir: &Path,
    expected: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = output_file_manifest(output_dir)?;
    let expected = expected
        .iter()
        .map(|entry| entry.to_string())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
    Ok(())
}

fn output_file_manifest(output_dir: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    collect_output_files(output_dir, output_dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_output_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_output_files(root, &path, files)?;
        } else if path.is_file() {
            let relative = path.strip_prefix(root)?;
            let normalized = relative
                .components()
                .map(|component| component.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");
            files.push(normalized);
        }
    }
    Ok(())
}
