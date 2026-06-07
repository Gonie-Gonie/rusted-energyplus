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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.25.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.25.0.md" -Description "v0.25 release notes"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.25"' -Description "v0.25 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Opaque No-Mass Heat Balance Generalization" -Description "v0.25 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'status = "complete"' -Description "v0.25 completion status"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "limited-conformance"' -Description "v0.25 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "heat_balance_nomass_001" -Description "v0.25 heat-balance case"
Assert-Contains -Path "specs\milestones.toml" -Pattern "surface_temperature_nomass_001" -Description "v0.25 surface-temperature case"
Assert-Contains -Path "specs\milestones.toml" -Pattern "general heat-balance compatibility" -Description "v0.25 non-claim boundary"

Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "outside_boundary_condition_object_name" -Description "boundary object state"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "outside_boundary_target_surface_id" -Description "adjacent surface target state"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "outside_boundary_target_zone_id" -Description "adjacent zone target state"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "surface_boundary_temperature_c" -Description "boundary temperature resolver"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "MissingSurfaceBoundaryTarget" -Description "missing surface target error"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "MissingZoneBoundaryTarget" -Description "missing zone target error"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "heat_balance_interzone_surface_uses_adjacent_zone_temperature" -Description "interzone boundary test"
Assert-Contains -Path "scripts\smoke\heat-balance-generalization-smoke.ps1" -Pattern "Heat-balance generalization smoke passed." -Description "heat-balance generalization smoke"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "heat-balance-generalization-smoke" -Description "quality gate wiring"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "v0.25-verify" -Description "dev command wiring"

Write-Host "milestone: v0.25.0"
Write-Host "scope: opaque no-mass heat-balance boundary generalization"
Write-Host "claim: limited conformance only for declared existing no-mass cases and variables"

Invoke-DevCommand -Command "heat-balance-generalization-smoke"
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\milestone-map.md" -Pattern "| 0.25 | Opaque No-Mass Heat Balance Generalization | complete" -Description "generated milestone status"
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.25.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.25.0")
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "file-size-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Assert-FileExists -Path ".runtime\release-evidence\v0.25.0\conformance-index-report.pdf" -Description "conformance index PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.25.0\conformance-index-report.json" -Description "conformance index JSON"
Assert-FileExists -Path ".runtime\release-evidence\v0.25.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.25.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

$index = Get-Content -LiteralPath ".runtime\release-evidence\v0.25.0\conformance-index-report.json" -Raw | ConvertFrom-Json
if ($index.aggregate.case_count -ne 13) {
    throw "Expected 13 indexed cases, found $($index.aggregate.case_count)"
}
if ($index.aggregate.conformance_case_count -ne 5) {
    throw "Expected 5 conformance cases in index, found $($index.aggregate.conformance_case_count)"
}

$evidence = Get-Content -LiteralPath ".runtime\release-evidence\v0.25.0\numeric-conformance-evidence.json" -Raw | ConvertFrom-Json
if ($evidence.aggregate.case_count -ne 4) {
    throw "Expected 4 promoted numerical conformance cases, found $($evidence.aggregate.case_count)"
}
if ($evidence.aggregate.series_count -ne 6) {
    throw "Expected 6 promoted numerical conformance series, found $($evidence.aggregate.series_count)"
}

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.25.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.25.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.25 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.25.0.md" -Description "v0.25 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "scripts/smoke/heat-balance-generalization-smoke.ps1" -Description "v0.25 packaged heat-balance smoke"
Assert-ZipEntry -ZipPath $package -Entry "specs/milestones.toml" -Description "v0.25 packaged milestone spec"

Write-Host "result: pass"
Write-Host "v0.25.0 heat-balance generalization verification passed."
