[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

Assert-FileExists -Path "docs\src\operations\v0.4.0-plan.md" -Description "v0.4 plan"
Assert-FileExists -Path "docs\src\operations\v0.4.0-readiness.md" -Description "v0.4 readiness"
Assert-FileExists -Path "data\conformance_cases\schedule_constant_001\case.toml" -Description "schedule case manifest"
Assert-FileExists -Path "data\conformance_cases\weather_fields_001\case.toml" -Description "weather case manifest"
Assert-FileExists -Path "data\conformance_cases\weather_fields_001\weather_fields.idf" -Description "weather case IDF"

Invoke-DevCommand -Command "schedule-compact-smoke"
Invoke-DevCommand -Command "compare-schedule-smoke"
Invoke-DevCommand -Command "compare-weather-smoke"
Invoke-DevCommand -Command "conformance-report-smoke"
Invoke-DevCommand -Command "compare-regression"
Invoke-DevCommand -Command "check"

Write-Host "v0.4.0 time/weather/schedule verification passed."
