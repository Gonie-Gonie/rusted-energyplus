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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.27.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.27.0.md" -Description "v0.27 release notes"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.27"' -Description "v0.27 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "User Support Coverage Report" -Description "v0.27 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "reporting-infrastructure"' -Description "v0.27 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "full EnergyPlus compatibility" -Description "v0.27 non-claim boundary"

Assert-Contains -Path "tools\reporting\support_coverage_report.py" -Pattern "build_support_coverage" -Description "support coverage builder"
Assert-Contains -Path "tools\reporting\support_coverage_report.py" -Pattern "Supported Inputs" -Description "supported inputs chapter"
Assert-Contains -Path "tools\reporting\support_coverage_report.py" -Pattern "Supported Outputs" -Description "supported outputs chapter"
Assert-Contains -Path "tools\reporting\support_coverage_report.py" -Pattern "Supported Algorithms" -Description "supported algorithms chapter"
Assert-Contains -Path "scripts\release\support-coverage-report.ps1" -Pattern "support_coverage_report.py" -Description "support coverage wrapper"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "support-coverage-report" -Description "dev command wiring"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "support-coverage-report" -Description "quality gate wiring"
Assert-Contains -Path "docs\src\conformance\support-coverage-report.md" -Pattern "which EnergyPlus input objects" -Description "user coverage docs"

Write-Host "milestone: v0.27.0"
Write-Host "scope: user-facing support coverage report"
Write-Host "claim: reporting infrastructure only"

Invoke-DevCommand -Command "support-coverage-report" -Arguments @("-Version", "0.27.0")
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.27.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.27.0")
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\milestone-map.md" -Pattern "| 0.27 | User Support Coverage Report | complete" -Description "generated milestone status"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Assert-FileExists -Path ".runtime\release-evidence\v0.27.0\support-coverage.md" -Description "support coverage markdown"
Assert-FileExists -Path ".runtime\release-evidence\v0.27.0\support-coverage-report.html" -Description "support coverage HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.27.0\support-coverage-report.pdf" -Description "support coverage PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.27.0\support-coverage-report.json" -Description "support coverage JSON"

$coverage = Get-Content -LiteralPath ".runtime\release-evidence\v0.27.0\support-coverage-report.json" -Raw | ConvertFrom-Json
if ($coverage.aggregate.input_object_count -ne 20) {
    throw "Expected 20 tracked input objects, found $($coverage.aggregate.input_object_count)"
}
if ($coverage.aggregate.tracked_output_variable_count -ne 45) {
    throw "Expected 45 tracked output variables, found $($coverage.aggregate.tracked_output_variable_count)"
}
if ($coverage.aggregate.manifest_output_request_count -ne 78) {
    throw "Expected 78 manifest output requests, found $($coverage.aggregate.manifest_output_request_count)"
}
if ($coverage.aggregate.algorithm_count -ne 4) {
    throw "Expected 4 tracked algorithms, found $($coverage.aggregate.algorithm_count)"
}
if ($coverage.aggregate.conformance_case_count -ne 6) {
    throw "Expected 6 conformance cases, found $($coverage.aggregate.conformance_case_count)"
}
if (@($coverage.known_gaps | Where-Object { $_ -like "*No full EnergyPlus ExampleFiles compatibility claim*" }).Count -ne 1) {
    throw "Expected explicit ExampleFiles compatibility non-claim in support coverage JSON"
}

$index = Get-Content -LiteralPath ".runtime\release-evidence\v0.27.0\conformance-index-report.json" -Raw | ConvertFrom-Json
if ($index.aggregate.case_count -ne 13) {
    throw "Expected 13 indexed cases, found $($index.aggregate.case_count)"
}
if ($index.aggregate.conformance_case_count -ne 6) {
    throw "Expected 6 conformance cases in index, found $($index.aggregate.conformance_case_count)"
}

$evidence = Get-Content -LiteralPath ".runtime\release-evidence\v0.27.0\numeric-conformance-evidence.json" -Raw | ConvertFrom-Json
if ($evidence.aggregate.case_count -ne 5) {
    throw "Expected 5 promoted numerical conformance cases, found $($evidence.aggregate.case_count)"
}
if ($evidence.aggregate.series_count -ne 7) {
    throw "Expected 7 promoted numerical conformance series, found $($evidence.aggregate.series_count)"
}

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.27.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.27.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.27 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.27.0.md" -Description "v0.27 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/conformance/support-coverage-report.md" -Description "v0.27 packaged support coverage docs"
Assert-ZipEntry -ZipPath $package -Entry "scripts/release/support-coverage-report.ps1" -Description "v0.27 packaged support coverage wrapper"
Assert-ZipEntry -ZipPath $package -Entry "tools/reporting/support_coverage_report.py" -Description "v0.27 packaged support coverage generator"

Write-Host "result: pass"
Write-Host "v0.27.0 support coverage report verification passed."
