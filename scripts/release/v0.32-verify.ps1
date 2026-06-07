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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.32.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.32.0.md" -Description "v0.32 release notes"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.32"' -Description "v0.32 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "User Coverage Handbook" -Description "v0.32 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "reporting-infrastructure"' -Description "v0.32 claim boundary"
Assert-Contains -Path "scripts\release\user-coverage-handbook.ps1" -Pattern "user_coverage_handbook.py" -Description "user handbook wrapper"
Assert-Contains -Path "tools\reporting\user_coverage_handbook.py" -Pattern "User Coverage Handbook" -Description "user handbook generator"
Assert-Contains -Path "tools\reporting\release_evidence_manifest.py" -Pattern "user-coverage-handbook.pdf" -Description "manifest includes handbook asset"
Assert-Contains -Path "docs\src\conformance\user-coverage-handbook.md" -Pattern "user decision guide" -Description "user handbook docs"

Write-Host "milestone: v0.32.0"
Write-Host "scope: user coverage handbook"
Write-Host "claim: reporting infrastructure only"

Invoke-DevCommand -Command "support-coverage-report" -Arguments @("-Version", "0.32.0")
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.32.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.32.0")
Invoke-DevCommand -Command "user-coverage-handbook" -Arguments @("-Version", "0.32.0")
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\milestone-map.md" -Pattern "| 0.32 | User Coverage Handbook | complete" -Description "generated milestone status"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.32.0")
Invoke-DevCommand -Command "release-evidence-manifest" -Arguments @("-Version", "0.32.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.32.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.32 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.32.0.md" -Description "v0.32 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/conformance/user-coverage-handbook.md" -Description "v0.32 packaged handbook docs"
Assert-ZipEntry -ZipPath $package -Entry "scripts/release/user-coverage-handbook.ps1" -Description "v0.32 packaged handbook wrapper"
Assert-ZipEntry -ZipPath $package -Entry "tools/reporting/user_coverage_handbook.py" -Description "v0.32 packaged handbook generator"

Assert-FileExists -Path ".runtime\release-evidence\v0.32.0\user-coverage-handbook.md" -Description "user handbook markdown"
Assert-FileExists -Path ".runtime\release-evidence\v0.32.0\user-coverage-handbook.html" -Description "user handbook HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.32.0\user-coverage-handbook.pdf" -Description "user handbook PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.32.0\user-coverage-handbook.json" -Description "user handbook JSON"

$handbook = Get-Content -LiteralPath ".runtime\release-evidence\v0.32.0\user-coverage-handbook.json" -Raw | ConvertFrom-Json
if ($handbook.aggregate.status -ne "pass") {
    throw "Expected user coverage handbook status pass, found $($handbook.aggregate.status)"
}
if ($handbook.aggregate.typed_input_count -ne 14) {
    throw "Expected 14 typed inputs, found $($handbook.aggregate.typed_input_count)"
}
if ($handbook.aggregate.structural_input_count -ne 6) {
    throw "Expected 6 structural inputs, found $($handbook.aggregate.structural_input_count)"
}
if ($handbook.aggregate.conformance_output_variable_count -ne 30) {
    throw "Expected 30 conformance output variables, found $($handbook.aggregate.conformance_output_variable_count)"
}
if ($handbook.aggregate.diagnostic_output_variable_count -ne 9) {
    throw "Expected 9 diagnostic output variables, found $($handbook.aggregate.diagnostic_output_variable_count)"
}
if ($handbook.aggregate.baseline_output_variable_count -ne 11) {
    throw "Expected 11 baseline output variables, found $($handbook.aggregate.baseline_output_variable_count)"
}
if ($handbook.aggregate.conformance_algorithm_count -ne 2) {
    throw "Expected 2 conformance algorithms, found $($handbook.aggregate.conformance_algorithm_count)"
}
if ($handbook.aggregate.diagnostic_algorithm_count -ne 2) {
    throw "Expected 2 diagnostic algorithms, found $($handbook.aggregate.diagnostic_algorithm_count)"
}
if ($handbook.aggregate.conformance_case_count -ne 6) {
    throw "Expected 6 promoted conformance cases, found $($handbook.aggregate.conformance_case_count)"
}
if ($handbook.aggregate.declared_numerical_series_count -ne 12) {
    throw "Expected 12 declared numerical series, found $($handbook.aggregate.declared_numerical_series_count)"
}
if ($handbook.aggregate.passed_numerical_series_count -ne 12) {
    throw "Expected 12 passed numerical series, found $($handbook.aggregate.passed_numerical_series_count)"
}
if (@($handbook.user_decision_rules).Count -lt 4) {
    throw "Expected at least 4 user decision rules"
}

Assert-FileExists -Path ".runtime\release-evidence\v0.32.0\release-evidence-manifest.json" -Description "release manifest JSON"
$manifest = Get-Content -LiteralPath ".runtime\release-evidence\v0.32.0\release-evidence-manifest.json" -Raw | ConvertFrom-Json
if ($manifest.aggregate.required_asset_count -ne 16) {
    throw "Expected 16 required release assets, found $($manifest.aggregate.required_asset_count)"
}
if ($manifest.aggregate.present_required_asset_count -ne 16) {
    throw "Expected all required release assets to be present, found $($manifest.aggregate.present_required_asset_count)"
}
if ($manifest.aggregate.missing_required_asset_count -ne 0) {
    throw "Expected no missing release assets, found $($manifest.aggregate.missing_required_asset_count)"
}
if (@($manifest.assets | Where-Object { $_.role -eq "user-coverage-handbook-pdf" -and $_.exists }).Count -ne 1) {
    throw "Expected user coverage handbook PDF in release manifest"
}
if ($manifest.report_summaries.user_coverage_handbook.typed_inputs -ne 14) {
    throw "Expected user coverage handbook summary in release manifest"
}

Write-Host "result: pass"
Write-Host "v0.32.0 user coverage handbook verification passed."
