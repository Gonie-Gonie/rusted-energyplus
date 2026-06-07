[CmdletBinding()]
param(
    [string]$Version = "0.30.0",
    [switch]$SkipGateRun
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
. (Join-Path $ScriptsRoot "lib\python.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$python = Get-ReportPythonExe
if (-not (Test-Path -LiteralPath $python -PathType Leaf)) {
    throw "Report Python environment is missing. Run .\scripts\dev.cmd setup first."
}

$script = Join-Path $RepoRoot "tools\reporting\conformance_evidence_report.py"
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Missing report generator: $script"
}

$arguments = @($script, "--repo-root", $RepoRoot, "--version", $Version)
if ($SkipGateRun) {
    $arguments += "--skip-gate-run"
}

& $python @arguments
if ($LASTEXITCODE -ne 0) {
    throw "Conformance evidence report generation failed with exit code $LASTEXITCODE"
}
