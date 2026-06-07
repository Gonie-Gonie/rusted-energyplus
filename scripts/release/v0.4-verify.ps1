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

Write-Host "milestone: v0.4"
Write-Host "scope: time/weather/schedule smoke evidence, no tolerance-gated conformance claim"

Assert-FileExists -Path "docs\src\porting-map\time-weather-schedule.md" -Description "time/weather/schedule porting map"
Assert-FileExists -Path "docs\src\conformance\output-variable-matrix.md" -Description "output variable matrix"
Assert-FileExists -Path "data\conformance_cases\schedule_constant_001\case.toml" -Description "schedule case manifest"
Assert-FileExists -Path "data\conformance_cases\weather_fields_001\case.toml" -Description "weather case manifest"
Assert-FileExists -Path "data\conformance_cases\weather_fields_001\weather_fields.idf" -Description "weather case IDF"

Write-Host "required commands:"
Write-Host "- schedule-compact-smoke"
Write-Host "- compare-schedule-smoke"
Write-Host "- compare-weather-smoke"
Write-Host "- conformance-report-smoke"
Write-Host "- test"
Write-Host "- docs-check"
Write-Host "- strict-no-false-conformance"

Invoke-DevCommand -Command "schedule-compact-smoke"
Invoke-DevCommand -Command "compare-schedule-smoke"
Invoke-DevCommand -Command "compare-weather-smoke"
Invoke-DevCommand -Command "conformance-report-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Write-Host "result: pass"
Write-Host "v0.4.0 time/weather/schedule verification passed."
