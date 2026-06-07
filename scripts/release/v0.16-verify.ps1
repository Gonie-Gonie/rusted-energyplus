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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.16.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\operations\v0.16.0-plan.md" -Description "v0.16 plan"
Assert-FileExists -Path "docs\src\operations\v0.16.0-readiness.md" -Description "v0.16 readiness"
Assert-FileExists -Path "docs\src\releases\v0.16.0.md" -Description "v0.16 release notes"
Assert-FileExists -Path "docs\src\porting-map\plant.md" -Description "plant porting map"
Assert-FileExists -Path "docs\src\porting-map\plant-source-map.md" -Description "plant source map"
Assert-FileExists -Path "scripts\smoke\plant-loop-projection-smoke.ps1" -Description "v0.16 plant projection smoke"
Assert-FileExists -Path "scripts\release\conformance-evidence-report.ps1" -Description "numeric conformance evidence release script"

Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "PlantStateProjectionEvidencePolicy" -Description "plant projection evidence policy"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "simulate_plant_state_projection" -Description "plant projection runtime function"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "plant-state-projection" -Description "plant projection CLI command"
Assert-Contains -Path "scripts\smoke\plant-loop-projection-smoke.ps1" -Pattern "algorithm_parity: false" -Description "v0.16 algorithm boundary"
Assert-Contains -Path "scripts\smoke\plant-loop-projection-smoke.ps1" -Pattern "plant-state-summary.json" -Description "v0.16 projection JSON artifact"
Assert-Contains -Path "scripts\smoke\plant-loop-projection-smoke.ps1" -Pattern "PlantLoop sizing-period baseline rows remain diagnostic-only" -Description "v0.16 sizing boundary"

Assert-Contains -Path "docs\src\operations\v0.16.0-readiness.md" -Pattern "diagnostic-ready" -Description "v0.16 readiness status"
Assert-Contains -Path "docs\src\operations\v0.16.0-readiness.md" -Pattern "not a plant numerical conformance claim" -Description "v0.16 claim boundary"
Assert-Contains -Path "docs\src\operations\v0.16.0-plan.md" -Pattern "This is not a plant, HVAC, node, meter, sizing, or ExampleFiles numerical" -Description "v0.16 plan boundary"
Assert-Contains -Path "docs\src\porting-map\plant.md" -Pattern "v0.16 Plant State Projection" -Description "plant map v0.16 section"
Assert-Contains -Path "docs\src\porting-map\algorithm-ledger.md" -Pattern "PlantLoadProfile projection" -Description "algorithm ledger v0.16 row"
Assert-Contains -Path "docs\src\conformance\numeric-release-evidence.md" -Pattern "For v0.16.0, that still means the earlier v0.8/v0.9 cases only" -Description "numeric evidence exclusion boundary"

Write-Host "milestone: v0.16.0"
Write-Host "scope: plant-state diagnostic projection evidence"
Write-Host "claim: diagnostic-only; no plant numerical conformance"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "plant-loop-skeleton-smoke"
Invoke-DevCommand -Command "plant-loop-diagnostic-smoke"
Invoke-DevCommand -Command "plant-loop-projection-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.16.0")

Assert-FileExists -Path ".runtime\plant-loop-diagnostic\26.1.0\plant-state-projection\plant-state-summary.md" -Description "plant projection markdown"
Assert-FileExists -Path ".runtime\plant-loop-diagnostic\26.1.0\plant-state-projection\plant-state-summary.json" -Description "plant projection JSON"
Assert-Contains -Path ".runtime\plant-loop-diagnostic\26.1.0\plant-state-projection\plant-state-summary.md" -Pattern "algorithm_parity: false" -Description "plant projection markdown algorithm boundary"
Assert-Contains -Path ".runtime\plant-loop-diagnostic\26.1.0\plant-state-projection\plant-state-summary.md" -Pattern "status: projected" -Description "plant projection markdown status"

Assert-FileExists -Path ".runtime\release-evidence\v0.16.0\numeric-conformance-evidence.html" -Description "numeric conformance evidence HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.16.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.16.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"
Assert-Contains -Path ".runtime\release-evidence\v0.16.0\numeric-conformance-evidence.html" -Pattern "Table of Contents" -Description "numeric evidence table of contents"
Assert-Contains -Path ".runtime\release-evidence\v0.16.0\numeric-conformance-evidence.html" -Pattern "Accuracy Evidence" -Description "numeric evidence accuracy section"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.16.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.16.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.16 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.16.0.md" -Description "v0.16 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/operations/v0.16.0-plan.md" -Description "v0.16 packaged plan"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/operations/v0.16.0-readiness.md" -Description "v0.16 packaged readiness"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/porting-map/plant.md" -Description "v0.16 packaged plant map"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/porting-map/plant-source-map.md" -Description "v0.16 packaged plant source map"
Assert-ZipEntry -ZipPath $package -Entry "scripts/smoke/plant-loop-projection-smoke.ps1" -Description "v0.16 packaged plant projection smoke"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/plant_loop_diagnostic_001/case.toml" -Description "v0.16 packaged plant diagnostic case"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/plant_loop_diagnostic_001/plant_loop_diagnostic.idf" -Description "v0.16 packaged plant diagnostic IDF"
Assert-ZipEntry -ZipPath $package -Entry "evidence/v0.16.0/numeric-conformance-evidence.html" -Description "v0.16 packaged numeric conformance evidence HTML"
Assert-ZipEntry -ZipPath $package -Entry "evidence/v0.16.0/numeric-conformance-evidence.pdf" -Description "v0.16 packaged numeric conformance evidence PDF"
Assert-ZipEntry -ZipPath $package -Entry "evidence/v0.16.0/numeric-conformance-evidence.json" -Description "v0.16 packaged numeric conformance evidence JSON"

Write-Host "result: pass"
Write-Host "v0.16.0 plant-state projection verification passed."
