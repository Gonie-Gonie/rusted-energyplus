[CmdletBinding()]
param(
    [string]$JsonOutput = ".runtime\v026-dynamic-idf-inventory.json",
    [string]$MarkdownOutput = ".runtime\v026-dynamic-idf-inventory.md"
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

$script = Join-Path $RepoRoot "tools\reporting\v026_dynamic_idf_inventory.py"
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Missing v0.26 dynamic IDF inventory generator: $script"
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
    throw "v0.26 dynamic IDF inventory failed with exit code $LASTEXITCODE"
}
