[CmdletBinding()]
param(
    [string]$Version = "0.1.0"
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

$script = Join-Path $RepoRoot "tools\reporting\performance_summary.py"
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Missing performance summary generator: $script"
}

& $python $script --repo-root $RepoRoot --version $Version
if ($LASTEXITCODE -ne 0) {
    throw "Performance summary generation failed with exit code $LASTEXITCODE"
}
