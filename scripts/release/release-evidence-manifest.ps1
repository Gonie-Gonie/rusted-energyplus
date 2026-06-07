[CmdletBinding()]
param(
    [string]$Version = "0.31.0",
    [string]$Target = "windows-x64"
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

$script = Join-Path $RepoRoot "tools\reporting\release_evidence_manifest.py"
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Missing release evidence manifest generator: $script"
}

$arguments = @($script, "--repo-root", $RepoRoot, "--version", $Version, "--target", $Target)

& $python @arguments
if ($LASTEXITCODE -ne 0) {
    throw "Release evidence manifest generation failed with exit code $LASTEXITCODE"
}
