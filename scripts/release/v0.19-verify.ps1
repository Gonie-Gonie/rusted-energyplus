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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.19.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.19.0.md" -Description "v0.19 release notes"
Assert-FileExists -Path "scripts\compare\compare-series-v2-smoke.ps1" -Description "series v2 smoke gate"

Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "SeriesComparisonV2" -Description "v2 comparison summary"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "SeriesAlignment::Timestamp" -Description "timestamp alignment"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "rmse_delta" -Description "RMSE metric"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "max_rel_delta" -Description "relative delta metric"
Assert-Contains -Path "crates\ep_compare\src\eso.rs" -Pattern "parse_eso_time_series" -Description "timestamp-aware ESO parser"
Assert-Contains -Path "crates\ep_compare\src\eso.rs" -Pattern "EsoTimeSeries" -Description "ESO time-series type"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "compare-series-v2-smoke" -Description "dev command registry"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "compare-series-v2-smoke" -Description "quality check v2 gate"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.19"' -Description "v0.19 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'status = "complete"' -Description "v0.19 completion status"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Series Reader and Compare Engine v2" -Description "v0.19 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "comparison-infrastructure"' -Description "v0.19 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "meter conformance" -Description "v0.19 meter non-claim boundary"
Assert-Contains -Path "docs\src\conformance\numeric-release-evidence.md" -Pattern "earlier v0.8/v0.9 cases only" -Description "numeric evidence exclusion boundary"

Write-Host "milestone: v0.19.0"
Write-Host "scope: timestamp-aware selected series reader and compare engine v2"
Write-Host "claim: comparison infrastructure only; no new numerical conformance or meter conformance"

Invoke-DevCommand -Command "compare-series-v2-smoke"
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.19.0")

Assert-FileExists -Path ".runtime\release-evidence\v0.19.0\numeric-conformance-evidence.html" -Description "numeric conformance evidence HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.19.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.19.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.19.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.19.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.19 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.19.0.md" -Description "v0.19 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "scripts/compare/compare-series-v2-smoke.ps1" -Description "v0.19 packaged series v2 gate"

Write-Host "result: pass"
Write-Host "v0.19.0 series reader and compare engine v2 verification passed."
