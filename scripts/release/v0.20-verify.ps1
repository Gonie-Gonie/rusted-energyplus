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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.20.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.20.0.md" -Description "v0.20 release notes"
Assert-FileExists -Path "scripts\release\conformance-index-report.ps1" -Description "conformance index wrapper"
Assert-FileExists -Path "tools\reporting\conformance_index_report.py" -Description "conformance index generator"

Assert-Contains -Path "tools\reporting\conformance_index_report.py" -Pattern "build_conformance_index" -Description "index builder"
Assert-Contains -Path "tools\reporting\conformance_index_report.py" -Pattern "coverage_matrix" -Description "coverage matrix"
Assert-Contains -Path "tools\reporting\conformance_index_report.py" -Pattern "conformance-index-report.pdf" -Description "PDF artifact"
Assert-Contains -Path "tools\reporting\conformance_index_report.py" -Pattern "conformance-index.md" -Description "markdown index artifact"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "conformance-index-report" -Description "dev command registry"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "conformance-index-report" -Description "quality check report gate"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.20"' -Description "v0.20 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'status = "complete"' -Description "v0.20 completion status"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Conformance Report Generator" -Description "v0.20 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "reporting-infrastructure"' -Description "v0.20 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "new numerical conformance unless backed by generated evidence" -Description "v0.20 non-claim boundary"

Write-Host "milestone: v0.20.0"
Write-Host "scope: release conformance index report generator and coverage matrices"
Write-Host "claim: reporting infrastructure only; no new numerical conformance"

Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.20.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.20.0")
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Assert-FileExists -Path ".runtime\release-evidence\v0.20.0\conformance-index.md" -Description "conformance index markdown"
Assert-FileExists -Path ".runtime\release-evidence\v0.20.0\conformance-index-report.html" -Description "conformance index HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.20.0\conformance-index-report.pdf" -Description "conformance index PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.20.0\conformance-index-report.json" -Description "conformance index JSON"
Assert-FileExists -Path ".runtime\release-evidence\v0.20.0\numeric-conformance-evidence.html" -Description "numeric conformance evidence HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.20.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.20.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

Assert-Contains -Path ".runtime\release-evidence\v0.20.0\conformance-index.md" -Pattern "Conformance Index Report" -Description "markdown report header"
Assert-Contains -Path ".runtime\release-evidence\v0.20.0\conformance-index-report.json" -Pattern '"coverage_matrix"' -Description "JSON coverage matrix"
Assert-Contains -Path ".runtime\release-evidence\v0.20.0\conformance-index-report.json" -Pattern '"conformance_case_count": 2' -Description "promoted case count"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.20.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.20.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.20 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.20.0.md" -Description "v0.20 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "scripts/release/conformance-index-report.ps1" -Description "v0.20 packaged report wrapper"
Assert-ZipEntry -ZipPath $package -Entry "tools/reporting/conformance_index_report.py" -Description "v0.20 packaged report generator"

Write-Host "result: pass"
Write-Host "v0.20.0 conformance report generator verification passed."
