[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }

    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -eq $match) {
        throw "Missing $Description marker in $Path`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

if ($null -eq (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

cargo test -p ep_compare -- series_v2
if ($LASTEXITCODE -ne 0) {
    throw "ep_compare series_v2 tests failed"
}

cargo test -p ep_compare -- parses_eso_time_series
if ($LASTEXITCODE -ne 0) {
    throw "ep_compare ESO time-series tests failed"
}

Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "SeriesComparisonV2" -Description "v2 comparison summary"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "SeriesAlignment::Timestamp" -Description "timestamp alignment"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "rmse_delta" -Description "RMSE metric"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "max_rel_delta" -Description "relative delta metric"
Assert-Contains -Path "crates\ep_compare\src\eso.rs" -Pattern "parse_eso_time_series" -Description "timestamp-aware ESO parser"
Assert-Contains -Path "crates\ep_compare\src\eso.rs" -Pattern "EsoTimeSeries" -Description "ESO time-series type"
Assert-Contains -Path "crates\ep_compare\src\eso.rs" -Pattern "hourly_timestamp_label" -Description "hourly timestamp labeler"

Write-Host "Series v2 smoke passed."
