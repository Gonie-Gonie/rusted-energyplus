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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.26.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.26.0.md" -Description "v0.26 release notes"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.26"' -Description "v0.26 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Internal Convective Gains Conformance" -Description "v0.26 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'status = "complete"' -Description "v0.26 completion status"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "declared-variables-only"' -Description "v0.26 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "internal_gains_001" -Description "v0.26 internal-gains case"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Zone Total Internal Convective Heating Rate" -Description "v0.26 proof variable"
Assert-Contains -Path "specs\milestones.toml" -Pattern "zone air temperature response to internal gains" -Description "v0.26 non-claim boundary"

Assert-Contains -Path "data\conformance_cases\internal_gains_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "internal-gains conformance class"
Assert-Contains -Path "data\conformance_cases\internal_gains_001\case.toml" -Pattern "conformance_claim = true" -Description "internal-gains conformance claim"
Assert-Contains -Path "data\conformance_cases\internal_gains_001\case.toml" -Pattern 'level = "conformance"' -Description "internal-gains output level"
Assert-Contains -Path "data\conformance_cases\internal_gains_001\case.toml" -Pattern 'variable_class = "internal-gain"' -Description "internal-gains tolerance class"
Assert-Contains -Path "data\conformance_cases\internal_gains_001\case.toml" -Pattern "compare-internal-convective-gain-conformance" -Description "internal-gains blocking gate"
Assert-Contains -Path "data\conformance_cases\internal_gains_001\case.toml" -Pattern "blocking = true" -Description "internal-gains gate blocking"

Assert-Contains -Path "crates\ep_cli\src\internal_gains.rs" -Pattern "generate_internal_gains_report" -Description "internal-gains report module"
Assert-Contains -Path "crates\ep_cli\src\internal_gains.rs" -Pattern "simulate_zone_internal_convective_gains" -Description "internal-gains runtime trace"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "internal-gains-report" -Description "CLI conformance command wiring"
Assert-Contains -Path "scripts\compare\compare-internal-convective-gain-conformance.ps1" -Pattern "Internal convective gain conformance gate passed." -Description "internal-gains conformance gate"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "compare-internal-convective-gain-conformance" -Description "quality gate wiring"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "v0.26-verify" -Description "dev command wiring"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern "compare-internal-convective-gain-conformance" -Description "release evidence case wiring"

Write-Host "milestone: v0.26.0"
Write-Host "scope: internal convective gains declared-variable conformance"
Write-Host "claim: Zone Total Internal Convective Heating Rate only"

Invoke-DevCommand -Command "compare-internal-convective-gain-conformance"
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\milestone-map.md" -Pattern "| 0.26 | Internal Convective Gains Conformance | complete" -Description "generated milestone status"
Assert-Contains -Path "docs\src\generated\conformance-case-index.md" -Pattern "| internal_gains_001 | v0.26-internal-convective-gains | conformance | true" -Description "generated internal-gains case index"
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.26.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.26.0")
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "file-size-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Assert-FileExists -Path ".runtime\release-evidence\v0.26.0\conformance-index-report.pdf" -Description "conformance index PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.26.0\conformance-index-report.json" -Description "conformance index JSON"
Assert-FileExists -Path ".runtime\release-evidence\v0.26.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.26.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

$index = Get-Content -LiteralPath ".runtime\release-evidence\v0.26.0\conformance-index-report.json" -Raw | ConvertFrom-Json
if ($index.aggregate.case_count -ne 13) {
    throw "Expected 13 indexed cases, found $($index.aggregate.case_count)"
}
if ($index.aggregate.conformance_case_count -ne 6) {
    throw "Expected 6 conformance cases in index, found $($index.aggregate.conformance_case_count)"
}

$evidence = Get-Content -LiteralPath ".runtime\release-evidence\v0.26.0\numeric-conformance-evidence.json" -Raw | ConvertFrom-Json
if ($evidence.aggregate.case_count -ne 5) {
    throw "Expected 5 promoted numerical conformance cases, found $($evidence.aggregate.case_count)"
}
if ($evidence.aggregate.series_count -ne 7) {
    throw "Expected 7 promoted numerical conformance series, found $($evidence.aggregate.series_count)"
}
if (@($evidence.cases | Where-Object { $_.case_id -eq "internal_gains_001" }).Count -ne 1) {
    throw "Expected internal_gains_001 in numeric evidence cases"
}

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.26.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.26.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.26 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.26.0.md" -Description "v0.26 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "scripts/compare/compare-internal-convective-gain-conformance.ps1" -Description "v0.26 packaged internal-gains gate"
Assert-ZipEntry -ZipPath $package -Entry "specs/milestones.toml" -Description "v0.26 packaged milestone spec"

Write-Host "result: pass"
Write-Host "v0.26.0 internal convective gains verification passed."
