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

Assert-FileExists -Path "docs\src\operations\v0.6.0-plan.md" -Description "v0.6 plan"
Assert-FileExists -Path "docs\src\operations\v0.6.0-readiness.md" -Description "v0.6 readiness"
Assert-FileExists -Path "docs\src\porting-map\output-reporting.md" -Description "output/reporting porting map"
Assert-FileExists -Path "docs\src\architecture\result-store.md" -Description "ResultStore architecture note"
Assert-FileExists -Path "docs\src\architecture\diagnostics-trace.md" -Description "diagnostics/trace architecture note"
Assert-FileExists -Path "data\conformance_cases\zone_temperature_diagnostic_001\case.toml" -Description "zone-temperature diagnostic manifest"
Assert-FileExists -Path "data\conformance_cases\zone_temperature_diagnostic_001\zone_temperature.idf" -Description "zone-temperature diagnostic IDF"
Assert-FileExists -Path "scripts\smoke\first-zone-smoke.ps1" -Description "ResultStore diagnostic smoke"
Assert-FileExists -Path "scripts\compare\compare-zone-smoke.ps1" -Description "zone-temperature diagnostic compare"
Assert-FileExists -Path "scripts\compare\compare-regression.ps1" -Description "compare regression artifacts"
Assert-FileExists -Path "scripts\conformance\conformance-diagnostic-report-smoke.ps1" -Description "manifest-driven diagnostic report smoke"

Write-Host "milestone: v0.6.0"
Write-Host "scope: output, trace, compare, and diagnostic report infrastructure"
Write-Host "claim: diagnostic-only; no heat-balance, zone-temperature, HVAC, or plant conformance"

Invoke-DevCommand -Command "first-zone-smoke"
Invoke-DevCommand -Command "compare-zone-smoke"
Invoke-DevCommand -Command "conformance-diagnostic-report-smoke"
Invoke-DevCommand -Command "compare-regression"
Invoke-DevCommand -Command "conformance-schema-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Write-Host "result: pass"
Write-Host "v0.6.0 output/trace/report verification passed."
