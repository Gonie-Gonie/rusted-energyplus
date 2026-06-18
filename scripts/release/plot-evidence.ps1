[CmdletBinding()]
param(
    [string]$Version = "0.32.0",
    [string]$OutputDir = "",
    [string]$LatestDir = ""
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

$script = Join-Path $RepoRoot "tools\reporting\plot_evidence.py"
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Missing evidence plot generator: $script"
}

$arguments = @($script, "--repo-root", $RepoRoot, "--version", $Version)
if ($OutputDir) {
    $arguments += @("--output-dir", $OutputDir)
}
if ($LatestDir) {
    $arguments += @("--latest-dir", $LatestDir)
}

& $python @arguments
if ($LASTEXITCODE -ne 0) {
    throw "Evidence plot generation failed with exit code $LASTEXITCODE"
}
