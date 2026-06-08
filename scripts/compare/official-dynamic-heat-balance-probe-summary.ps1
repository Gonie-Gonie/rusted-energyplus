[CmdletBinding()]
param(
    [string]$JsonOutput = ".runtime\official-dynamic-probe-summary.json",
    [string]$MarkdownOutput = ".runtime\official-dynamic-probe-summary.md"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
. (Join-Path $ScriptsRoot "lib\python.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Resolve-RepoOutputPath {
    param([Parameter(Mandatory = $true)][string]$Path)

    if ([System.IO.Path]::IsPathRooted($Path)) {
        return $Path
    }
    return (Join-Path $RepoRoot $Path)
}

$python = Get-ReportPythonExe
if (-not (Test-Path -LiteralPath $python -PathType Leaf)) {
    throw "Report Python environment is missing. Run .\scripts\dev.cmd setup first."
}

$script = Join-Path $RepoRoot "tools\reporting\dynamic_heat_balance_probe_summary.py"
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Missing dynamic heat-balance probe summary generator: $script"
}

$arguments = @(
    $script,
    "--repo-root",
    $RepoRoot,
    "--json-output",
    (Resolve-RepoOutputPath -Path $JsonOutput),
    "--markdown-output",
    (Resolve-RepoOutputPath -Path $MarkdownOutput)
)

& $python @arguments
if ($LASTEXITCODE -ne 0) {
    throw "Dynamic heat-balance probe summary failed with exit code $LASTEXITCODE"
}
