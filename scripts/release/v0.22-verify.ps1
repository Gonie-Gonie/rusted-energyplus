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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.22.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.22.0.md" -Description "v0.22 release notes"

Assert-Contains -Path "data\conformance_cases\schedule_constant_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "schedule conformance class"
Assert-Contains -Path "data\conformance_cases\schedule_constant_001\case.toml" -Pattern 'level = "conformance"' -Description "schedule conformance output"
Assert-Contains -Path "data\conformance_cases\schedule_constant_001\case.toml" -Pattern "compare-schedule-conformance" -Description "schedule blocking gate"
Assert-Contains -Path "data\conformance_cases\weather_fields_001\case.toml" -Pattern 'Site Outdoor Air Drybulb Temperature' -Description "weather dry-bulb variable"
Assert-Contains -Path "data\conformance_cases\weather_fields_001\case.toml" -Pattern 'level = "conformance"' -Description "weather conformance output"
Assert-Contains -Path "data\conformance_cases\weather_fields_001\case.toml" -Pattern "Dew point, relative humidity, barometric pressure, wind speed, and wind direction remain diagnostic rows" -Description "weather claim boundary"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "compare-schedule-conformance" -Description "schedule dev gate"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "compare-weather-conformance" -Description "weather dev gate"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "compare-schedule-conformance" -Description "quality schedule gate"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "compare-weather-conformance" -Description "quality weather gate"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern "compare-schedule-conformance" -Description "evidence schedule gate"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern "compare-weather-conformance" -Description "evidence weather gate"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.22"' -Description "v0.22 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'status = "complete"' -Description "v0.22 completion status"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Time, Weather, and Schedule Conformance Expansion" -Description "v0.22 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "declared-variables-only"' -Description "v0.22 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "general runtime compatibility" -Description "v0.22 non-claim boundary"

Write-Host "milestone: v0.22.0"
Write-Host "scope: timestamp-aligned Schedule Value and dry-bulb conformance"
Write-Host "claim: declared variables only; no general runtime compatibility"

Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "compare-schedule-conformance"
Invoke-DevCommand -Command "compare-weather-conformance"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\milestone-map.md" -Pattern "| 0.22 | Time, Weather, and Schedule Conformance Expansion | complete" -Description "generated milestone status"
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.22.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.22.0")
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Assert-FileExists -Path ".runtime\release-evidence\v0.22.0\conformance-index-report.pdf" -Description "conformance index PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.22.0\conformance-index-report.json" -Description "conformance index JSON"
Assert-FileExists -Path ".runtime\release-evidence\v0.22.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.22.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

$evidence = Get-Content -LiteralPath ".runtime\release-evidence\v0.22.0\numeric-conformance-evidence.json" -Raw | ConvertFrom-Json
if ($evidence.aggregate.case_count -ne 4) {
    throw "Expected 4 promoted conformance cases, found $($evidence.aggregate.case_count)"
}
if ($evidence.aggregate.series_count -ne 6) {
    throw "Expected 6 promoted conformance series, found $($evidence.aggregate.series_count)"
}
if (-not ($evidence.cases | Where-Object { $_.case_id -eq "schedule_constant_001" })) {
    throw "Missing schedule_constant_001 in release evidence"
}
if (-not ($evidence.cases | Where-Object { $_.case_id -eq "weather_fields_001" })) {
    throw "Missing weather_fields_001 in release evidence"
}

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.22.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.22.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.22 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.22.0.md" -Description "v0.22 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "scripts/compare/compare-schedule-conformance.ps1" -Description "v0.22 packaged schedule gate"
Assert-ZipEntry -ZipPath $package -Entry "scripts/compare/compare-weather-conformance.ps1" -Description "v0.22 packaged weather gate"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/schedule_constant_001/case.toml" -Description "v0.22 packaged schedule manifest"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/weather_fields_001/case.toml" -Description "v0.22 packaged weather manifest"

Write-Host "result: pass"
Write-Host "v0.22.0 time/weather/schedule conformance verification passed."
