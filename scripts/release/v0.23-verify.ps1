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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.23.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.23.0.md" -Description "v0.23 release notes"

Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern 'source_kind = "energy-plus-examplefile"' -Description "official static source kind"
Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "static conformance class"
Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern 'level = "conformance"' -Description "static conformance outputs"
Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern "compare-static-model-conformance" -Description "static blocking gate"
Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern "dynamic heat-balance, HVAC, plant, solar, fenestration, sizing, warmup, or meter conformance" -Description "static non-claim boundary"
Assert-Contains -Path "crates\ep_cli\src\static_model.rs" -Pattern "static EIO model evidence only" -Description "static report claim boundary"
Assert-Contains -Path "crates\ep_cli\src\conformance_artifacts.rs" -Pattern "Output:Surfaces:List,Details" -Description "surface detail staging"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "compare-static-model-conformance" -Description "static dev gate"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "compare-static-model-conformance" -Description "quality static gate"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.23"' -Description "v0.23 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'status = "complete"' -Description "v0.23 completion status"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Static Model Evidence Expansion" -Description "v0.23 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "static-evidence"' -Description "v0.23 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "dynamic heat-balance compatibility" -Description "v0.23 dynamic non-claim boundary"

Write-Host "milestone: v0.23.0"
Write-Host "scope: official 1ZoneUncontrolled static EIO model evidence"
Write-Host "claim: static evidence only; no dynamic heat-balance or system compatibility"

Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "compare-static-model-conformance"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\milestone-map.md" -Pattern "| 0.23 | Static Model Evidence Expansion | complete" -Description "generated milestone status"
Assert-Contains -Path "docs\src\generated\conformance-case-index.md" -Pattern "official_1zone_static_model_001" -Description "generated static case index"
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.23.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.23.0")
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Assert-FileExists -Path ".runtime\static-model-conformance\26.1.0\official_1zone_static_model_001\compare\compare-report.md" -Description "static model markdown evidence"
Assert-FileExists -Path ".runtime\static-model-conformance\26.1.0\official_1zone_static_model_001\compare\compare-summary.json" -Description "static model JSON evidence"
Assert-FileExists -Path ".runtime\release-evidence\v0.23.0\conformance-index-report.pdf" -Description "conformance index PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.23.0\conformance-index-report.json" -Description "conformance index JSON"
Assert-FileExists -Path ".runtime\release-evidence\v0.23.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.23.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

$static = Get-Content -LiteralPath ".runtime\static-model-conformance\26.1.0\official_1zone_static_model_001\compare\compare-summary.json" -Raw | ConvertFrom-Json
if ($static.status -ne "pass") {
    throw "Static model summary did not pass: $($static.status)"
}
if (@($static.rows).Count -ne 19) {
    throw "Expected 19 static rows, found $(@($static.rows).Count)"
}
$staticRows = @($static.rows)
if (@($staticRows | Where-Object { $_.status -ne "pass" }).Count -gt 0) {
    throw "Static model evidence contains failing rows"
}
if ($static.object_counts.surfaces -ne 6 -or $static.object_counts.constructions -ne 3 -or $static.object_counts.other_equipment -ne 2) {
    throw "Static model object counts are outside the declared v0.23 fixture boundary"
}

$index = Get-Content -LiteralPath ".runtime\release-evidence\v0.23.0\conformance-index-report.json" -Raw | ConvertFrom-Json
if ($index.aggregate.case_count -ne 13) {
    throw "Expected 13 indexed cases, found $($index.aggregate.case_count)"
}
if ($index.aggregate.conformance_case_count -ne 5) {
    throw "Expected 5 conformance cases in index, found $($index.aggregate.conformance_case_count)"
}

$evidence = Get-Content -LiteralPath ".runtime\release-evidence\v0.23.0\numeric-conformance-evidence.json" -Raw | ConvertFrom-Json
if ($evidence.aggregate.case_count -ne 4) {
    throw "Expected 4 promoted numerical conformance cases, found $($evidence.aggregate.case_count)"
}
if ($evidence.aggregate.series_count -ne 6) {
    throw "Expected 6 promoted numerical conformance series, found $($evidence.aggregate.series_count)"
}
if ($evidence.cases | Where-Object { $_.case_id -eq "official_1zone_static_model_001" }) {
    throw "Static model evidence must not be mixed into the numeric conformance PDF"
}

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.23.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.23.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.23 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.23.0.md" -Description "v0.23 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "scripts/compare/compare-static-model-conformance.ps1" -Description "v0.23 packaged static gate"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/official_1zone_static_model_001/case.toml" -Description "v0.23 packaged static manifest"

Write-Host "result: pass"
Write-Host "v0.23.0 static model evidence verification passed."
