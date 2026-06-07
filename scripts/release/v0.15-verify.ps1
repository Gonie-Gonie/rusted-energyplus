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

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    Assert-FileExists -Path $Path -Description $Description
    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -eq $match) {
        throw "Missing $Description marker in $Path`: $Pattern"
    }
    Write-Host "OK $Description marker: $Pattern"
}

function Assert-ZipEntry {
    param(
        [Parameter(Mandatory = $true)][string]$ZipPath,
        [Parameter(Mandatory = $true)][string]$Entry,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-FileExists -Path $ZipPath -Description $Description
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $archive = [System.IO.Compression.ZipFile]::OpenRead((Resolve-Path -LiteralPath $ZipPath).Path)
    try {
        $expected = $Entry.Replace("/", "\")
        $match = $archive.Entries | Where-Object {
            $_.FullName.Replace("/", "\") -eq $expected
        }
        if ($null -eq $match) {
            throw "Missing $Description zip entry in $ZipPath`: $Entry"
        }
        Write-Host "OK $Description zip entry: $Entry"
    }
    finally {
        $archive.Dispose()
    }
}

Assert-FileExists -Path "docs\src\archive\pre-alpha\v0.15.0-plan.md" -Description "v0.15 plan"
Assert-FileExists -Path "docs\src\archive\pre-alpha\v0.15.0-readiness.md" -Description "v0.15 readiness"
Assert-FileExists -Path "docs\src\releases\v0.15.0.md" -Description "v0.15 release notes"
Assert-FileExists -Path "docs\src\porting-map\plant.md" -Description "plant porting map"
Assert-FileExists -Path "docs\src\porting-map\plant-source-map.md" -Description "plant source map"
Assert-FileExists -Path "docs\src\conformance\output-variable-matrix.md" -Description "output variable matrix"
Assert-FileExists -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Description "v0.15 plant diagnostic case"
Assert-FileExists -Path "data\conformance_cases\plant_loop_diagnostic_001\plant_loop_diagnostic.idf" -Description "v0.15 plant diagnostic IDF"
Assert-FileExists -Path "scripts\smoke\plant-loop-diagnostic-smoke.ps1" -Description "v0.15 plant diagnostic smoke"
Assert-FileExists -Path "scripts\release\conformance-evidence-report.ps1" -Description "numeric conformance evidence release script"

Assert-Contains -Path "crates\ep_conformance\src\conformance.rs" -Pattern "PlantState" -Description "plant-state variable class"
Assert-Contains -Path "crates\ep_conformance\src\conformance.rs" -Pattern "PlantEquipment" -Description "plant-equipment variable class"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern '"plant-state"' -Description "plant-state CLI label"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern '"plant-equipment"' -Description "plant-equipment CLI label"

Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'comparison_class = "diagnostic-only"' -Description "v0.15 diagnostic class"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern "conformance_claim = false" -Description "v0.15 claim boundary"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'weather = ".runtime/energyplus/26.1.0/WeatherData/USA_IL_Chicago-OHare.Intl.AP.725300_TMY3.epw"' -Description "v0.15 Chicago weather"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'class = "plant-state"' -Description "v0.15 plant-state outputs"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'class = "plant-equipment"' -Description "v0.15 plant-equipment outputs"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'script = "scripts/dev.cmd plant-loop-diagnostic-smoke"' -Description "v0.15 blocking smoke gate"

Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\plant_loop_diagnostic.idf" -Pattern "Pump:VariableSpeed" -Description "v0.15 plant fixture pump"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\plant_loop_diagnostic.idf" -Pattern "DistrictHeating:Water" -Description "v0.15 plant fixture source"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\plant_loop_diagnostic.idf" -Pattern "LoadProfile:Plant" -Description "v0.15 plant fixture load"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\plant_loop_diagnostic.idf" -Pattern "Output:Variable,Main Loop,Plant Supply Side Heating Demand Rate,Hourly;" -Description "v0.15 plant heating output"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\plant_loop_diagnostic.idf" -Pattern "Output:Variable,Pump,Pump Electricity Rate,Hourly;" -Description "v0.15 pump output"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\plant_loop_diagnostic.idf" -Pattern "Output:Variable,Load Profile 1,Plant Load Profile Heat Transfer Rate,Hourly;" -Description "v0.15 load-profile output"

Assert-Contains -Path "scripts\smoke\plant-loop-diagnostic-smoke.ps1" -Pattern "energyplus_warnings: 0" -Description "v0.15 warning gate"
Assert-Contains -Path "scripts\smoke\plant-loop-diagnostic-smoke.ps1" -Pattern "plant-state" -Description "v0.15 plant-state smoke check"
Assert-Contains -Path "scripts\smoke\plant-loop-diagnostic-smoke.ps1" -Pattern "plant-equipment" -Description "v0.15 plant-equipment smoke check"
Assert-Contains -Path "scripts\smoke\plant-loop-diagnostic-smoke.ps1" -Pattern "baseline_nonzero_count" -Description "v0.15 nonzero baseline gate"
Assert-Contains -Path "scripts\smoke\plant-loop-diagnostic-smoke.ps1" -Pattern "status: baseline-only" -Description "v0.15 baseline-only status"

Assert-Contains -Path "docs\src\archive\pre-alpha\v0.15.0-readiness.md" -Pattern "diagnostic-ready" -Description "v0.15 readiness status"
Assert-Contains -Path "docs\src\archive\pre-alpha\v0.15.0-readiness.md" -Pattern "not a plant numerical conformance claim" -Description "v0.15 claim boundary"
Assert-Contains -Path "docs\src\archive\pre-alpha\v0.15.0-plan.md" -Pattern "This is not a plant, HVAC, node, meter, sizing, or ExampleFiles numerical" -Description "v0.15 plan boundary"
Assert-Contains -Path "docs\src\porting-map\plant.md" -Pattern "v0.15 Plant Loop Diagnostic Baseline" -Description "plant map v0.15 section"
Assert-Contains -Path "docs\src\conformance\output-variable-matrix.md" -Pattern "plant_loop_diagnostic_001" -Description "output variable matrix v0.15 row"
Assert-Contains -Path "docs\src\conformance\numeric-release-evidence.md" -Pattern "For v0.15.0, that still means the earlier v0.8/v0.9 cases only" -Description "numeric evidence exclusion boundary"

Write-Host "milestone: v0.15.0"
Write-Host "scope: plant-loop diagnostic baseline evidence"
Write-Host "claim: diagnostic-only; no plant numerical conformance"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "plant-loop-skeleton-smoke"
Invoke-DevCommand -Command "plant-loop-diagnostic-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.15.0")

Assert-FileExists -Path ".runtime\release-evidence\v0.15.0\numeric-conformance-evidence.html" -Description "numeric conformance evidence HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.15.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.15.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"
Assert-Contains -Path ".runtime\release-evidence\v0.15.0\numeric-conformance-evidence.html" -Pattern "Table of Contents" -Description "numeric evidence table of contents"
Assert-Contains -Path ".runtime\release-evidence\v0.15.0\numeric-conformance-evidence.html" -Pattern "Accuracy Evidence" -Description "numeric evidence accuracy section"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.15.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.15.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.15 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.15.0.md" -Description "v0.15 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/archive/pre-alpha/v0.15.0-plan.md" -Description "v0.15 packaged plan"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/archive/pre-alpha/v0.15.0-readiness.md" -Description "v0.15 packaged readiness"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/porting-map/plant.md" -Description "v0.15 packaged plant map"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/porting-map/plant-source-map.md" -Description "v0.15 packaged plant source map"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/plant_loop_diagnostic_001/case.toml" -Description "v0.15 packaged plant diagnostic case"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/plant_loop_diagnostic_001/plant_loop_diagnostic.idf" -Description "v0.15 packaged plant diagnostic IDF"
Assert-ZipEntry -ZipPath $package -Entry "scripts/smoke/plant-loop-diagnostic-smoke.ps1" -Description "v0.15 packaged plant diagnostic smoke"
Assert-ZipEntry -ZipPath $package -Entry "evidence/v0.15.0/numeric-conformance-evidence.html" -Description "v0.15 packaged numeric conformance evidence HTML"
Assert-ZipEntry -ZipPath $package -Entry "evidence/v0.15.0/numeric-conformance-evidence.pdf" -Description "v0.15 packaged numeric conformance evidence PDF"
Assert-ZipEntry -ZipPath $package -Entry "evidence/v0.15.0/numeric-conformance-evidence.json" -Description "v0.15 packaged numeric conformance evidence JSON"

Write-Host "result: pass"
Write-Host "v0.15.0 plant-loop diagnostic verification passed."
