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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.31.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.31.0.md" -Description "v0.31 release notes"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.31"' -Description "v0.31 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Release Evidence Asset Manifest" -Description "v0.31 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "reporting-infrastructure"' -Description "v0.31 claim boundary"
Assert-Contains -Path "scripts\release\release-evidence-manifest.ps1" -Pattern "release_evidence_manifest.py" -Description "release manifest wrapper"
Assert-Contains -Path "tools\reporting\release_evidence_manifest.py" -Pattern "sha256_file" -Description "release manifest hash generation"
Assert-Contains -Path "docs\src\conformance\release-evidence-manifest.md" -Pattern "GitHub Release asset manifest" -Description "release manifest docs"

Write-Host "milestone: v0.31.0"
Write-Host "scope: release evidence asset manifest"
Write-Host "claim: reporting infrastructure only"

Invoke-DevCommand -Command "support-coverage-report" -Arguments @("-Version", "0.31.0")
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.31.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.31.0")
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\milestone-map.md" -Pattern "| 0.31 | Release Evidence Asset Manifest | complete" -Description "generated milestone status"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.31.0")
Invoke-DevCommand -Command "release-evidence-manifest" -Arguments @("-Version", "0.31.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.31.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.31 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.31.0.md" -Description "v0.31 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/conformance/release-evidence-manifest.md" -Description "v0.31 packaged manifest docs"
Assert-ZipEntry -ZipPath $package -Entry "scripts/release/release-evidence-manifest.ps1" -Description "v0.31 packaged manifest wrapper"
Assert-ZipEntry -ZipPath $package -Entry "tools/reporting/release_evidence_manifest.py" -Description "v0.31 packaged manifest generator"

Assert-FileExists -Path ".runtime\release-evidence\v0.31.0\release-evidence-manifest.md" -Description "release manifest markdown"
Assert-FileExists -Path ".runtime\release-evidence\v0.31.0\release-evidence-manifest.html" -Description "release manifest HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.31.0\release-evidence-manifest.pdf" -Description "release manifest PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.31.0\release-evidence-manifest.json" -Description "release manifest JSON"

$manifest = Get-Content -LiteralPath ".runtime\release-evidence\v0.31.0\release-evidence-manifest.json" -Raw | ConvertFrom-Json
if ($manifest.aggregate.status -ne "pass") {
    throw "Expected release evidence manifest status pass, found $($manifest.aggregate.status)"
}
if ($manifest.aggregate.required_asset_count -ne 12) {
    throw "Expected 12 required release assets, found $($manifest.aggregate.required_asset_count)"
}
if ($manifest.aggregate.present_required_asset_count -ne 12) {
    throw "Expected all required release assets to be present, found $($manifest.aggregate.present_required_asset_count)"
}
if ($manifest.aggregate.missing_required_asset_count -ne 0) {
    throw "Expected no missing release assets, found $($manifest.aggregate.missing_required_asset_count)"
}
if (@($manifest.assets | Where-Object { $_.role -eq "binary-package" -and $_.sha256.Length -eq 64 }).Count -ne 1) {
    throw "Expected binary package asset with SHA-256"
}
if (@($manifest.assets | Where-Object { $_.role -eq "support-coverage-pdf" -and $_.exists }).Count -ne 1) {
    throw "Expected support coverage PDF in release manifest"
}
if ($manifest.report_summaries.support_coverage.input_objects -ne 20) {
    throw "Expected 20 input objects in support coverage summary, found $($manifest.report_summaries.support_coverage.input_objects)"
}
if ($manifest.report_summaries.support_coverage.output_variables -ne 45) {
    throw "Expected 45 output variables in support coverage summary, found $($manifest.report_summaries.support_coverage.output_variables)"
}
if ($manifest.report_summaries.support_coverage.algorithms -ne 4) {
    throw "Expected 4 algorithms in support coverage summary, found $($manifest.report_summaries.support_coverage.algorithms)"
}
if ($manifest.report_summaries.numeric_conformance.cases -ne 5) {
    throw "Expected 5 numeric conformance cases, found $($manifest.report_summaries.numeric_conformance.cases)"
}
if ($manifest.report_summaries.numeric_conformance.series -ne 7) {
    throw "Expected 7 numeric conformance series, found $($manifest.report_summaries.numeric_conformance.series)"
}

Write-Host "result: pass"
Write-Host "v0.31.0 release evidence asset manifest verification passed."
