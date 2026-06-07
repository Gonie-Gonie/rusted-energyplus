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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.30.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.30.0.md" -Description "v0.30 release notes"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.30"' -Description "v0.30 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Algorithm Coverage Metadata" -Description "v0.30 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "reporting-infrastructure"' -Description "v0.30 claim boundary"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern "first_evidence" -Description "algorithm first evidence metadata"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern "support_boundary" -Description "algorithm support boundary metadata"
Assert-Contains -Path "tools\docs\generate_docs.py" -Pattern "First evidence" -Description "generated algorithm first evidence column"
Assert-Contains -Path "tools\docs\generate_docs.py" -Pattern "Boundary" -Description "generated algorithm boundary column"
Assert-Contains -Path "tools\reporting\support_coverage_report.py" -Pattern "algorithms_with_boundary_count" -Description "algorithm boundary aggregate"

Write-Host "milestone: v0.30.0"
Write-Host "scope: algorithm coverage metadata"
Write-Host "claim: reporting infrastructure only"

Invoke-DevCommand -Command "support-coverage-report" -Arguments @("-Version", "0.30.0")
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.30.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.30.0")
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\milestone-map.md" -Pattern "| 0.30 | Algorithm Coverage Metadata | complete" -Description "generated milestone status"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern "First evidence" -Description "generated algorithm first evidence column"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern "Boundary" -Description "generated algorithm boundary column"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern "Diagnostic node-state projection only" -Description "generated algorithm diagnostic boundary"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Assert-FileExists -Path ".runtime\release-evidence\v0.30.0\support-coverage.md" -Description "support coverage markdown"
Assert-FileExists -Path ".runtime\release-evidence\v0.30.0\support-coverage-report.html" -Description "support coverage HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.30.0\support-coverage-report.pdf" -Description "support coverage PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.30.0\support-coverage-report.json" -Description "support coverage JSON"

$coverage = Get-Content -LiteralPath ".runtime\release-evidence\v0.30.0\support-coverage-report.json" -Raw | ConvertFrom-Json
if ($coverage.aggregate.algorithm_count -ne 4) {
    throw "Expected 4 tracked algorithms, found $($coverage.aggregate.algorithm_count)"
}
if ($coverage.aggregate.algorithms_with_first_evidence_count -ne 4) {
    throw "Expected first evidence for all 4 algorithms, found $($coverage.aggregate.algorithms_with_first_evidence_count)"
}
if ($coverage.aggregate.algorithms_with_boundary_count -ne 4) {
    throw "Expected support boundaries for all 4 algorithms, found $($coverage.aggregate.algorithms_with_boundary_count)"
}
if (@($coverage.algorithms | Where-Object { [string]::IsNullOrWhiteSpace($_.support_boundary) }).Count -ne 0) {
    throw "Every algorithm must have a support_boundary"
}
if (@($coverage.algorithms | Where-Object { $_.id -eq "zone_air_heat_balance" -and $_.support_boundary -like "*no general heat-balance*" }).Count -ne 1) {
    throw "Expected zone heat-balance limited support boundary"
}
if (@($coverage.algorithms | Where-Object { $_.id -eq "plant_loop_state_projection" -and $_.support_boundary -like "*no plant loop*" }).Count -ne 1) {
    throw "Expected plant diagnostic support boundary"
}
if ($coverage.aggregate.tracked_output_variable_count -ne 45) {
    throw "Expected 45 tracked output variables, found $($coverage.aggregate.tracked_output_variable_count)"
}

$evidence = Get-Content -LiteralPath ".runtime\release-evidence\v0.30.0\numeric-conformance-evidence.json" -Raw | ConvertFrom-Json
if ($evidence.aggregate.case_count -ne 5) {
    throw "Expected 5 promoted numerical conformance cases, found $($evidence.aggregate.case_count)"
}
if ($evidence.aggregate.series_count -ne 7) {
    throw "Expected 7 promoted numerical conformance series, found $($evidence.aggregate.series_count)"
}

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.30.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.30.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.30 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.30.0.md" -Description "v0.30 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "specs/algorithm_ledger.toml" -Description "v0.30 packaged algorithm ledger"
Assert-ZipEntry -ZipPath $package -Entry "tools/reporting/support_coverage_report.py" -Description "v0.30 packaged support coverage generator"

Write-Host "result: pass"
Write-Host "v0.30.0 algorithm coverage metadata verification passed."
