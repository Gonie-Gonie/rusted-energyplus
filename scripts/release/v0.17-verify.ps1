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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.17.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\operations\v0.17.0-plan.md" -Description "v0.17 plan"
Assert-FileExists -Path "docs\src\operations\v0.17.0-readiness.md" -Description "v0.17 readiness"
Assert-FileExists -Path "docs\src\releases\v0.17.0.md" -Description "v0.17 release notes"
Assert-FileExists -Path "scripts\conformance\manifest-validate-all.ps1" -Description "manifest v2 all-case gate"

Assert-Contains -Path "crates\ep_conformance\src\conformance.rs" -Pattern "CASE_MANIFEST_V2_SCHEMA" -Description "manifest v2 schema constant"
Assert-Contains -Path "crates\ep_conformance\src\conformance.rs" -Pattern "OutputLevel" -Description "output level enum"
Assert-Contains -Path "crates\ep_conformance\src\conformance.rs" -Pattern "MeterRequest" -Description "meter request schema"
Assert-Contains -Path "crates\ep_conformance\src\conformance.rs" -Pattern "Waiver" -Description "waiver schema"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "validate-case-v2" -Description "manifest v2 CLI command"
Assert-Contains -Path "scripts\conformance\manifest-validate-all.ps1" -Pattern "schema_v2: rusted-energyplus.case-manifest.v2" -Description "manifest v2 schema gate marker"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern 'level = "conformance"' -Description "v0.8 conformance-level output"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'level = "baseline"' -Description "plant diagnostic baseline level"
Assert-Contains -Path "docs\src\operations\v0.17.0-readiness.md" -Pattern "schema-v2-ready" -Description "v0.17 readiness status"
Assert-Contains -Path "docs\src\operations\v0.17.0-readiness.md" -Pattern "does not add a new numerical conformance claim" -Description "v0.17 claim boundary"
Assert-Contains -Path "docs\src\conformance\numeric-release-evidence.md" -Pattern "For v0.17.0, that still means the earlier v0.8/v0.9 cases only" -Description "numeric evidence exclusion boundary"

Write-Host "milestone: v0.17.0"
Write-Host "scope: Case Manifest and Output Request Schema v2"
Write-Host "claim: no new numerical conformance; promoted cases remain v0.8/v0.9 only"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "conformance-schema-smoke"
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.17.0")

Assert-FileExists -Path ".runtime\release-evidence\v0.17.0\numeric-conformance-evidence.html" -Description "numeric conformance evidence HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.17.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.17.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.17.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.17.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.17 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.17.0.md" -Description "v0.17 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/operations/v0.17.0-plan.md" -Description "v0.17 packaged plan"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/operations/v0.17.0-readiness.md" -Description "v0.17 packaged readiness"
Assert-ZipEntry -ZipPath $package -Entry "scripts/conformance/manifest-validate-all.ps1" -Description "v0.17 packaged manifest gate"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/heat_balance_nomass_001/case.toml" -Description "v0.17 packaged v2 conformance case"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/plant_loop_diagnostic_001/case.toml" -Description "v0.17 packaged v2 diagnostic case"
Assert-ZipEntry -ZipPath $package -Entry "evidence/v0.17.0/numeric-conformance-evidence.html" -Description "v0.17 packaged numeric conformance evidence HTML"
Assert-ZipEntry -ZipPath $package -Entry "evidence/v0.17.0/numeric-conformance-evidence.pdf" -Description "v0.17 packaged numeric conformance evidence PDF"
Assert-ZipEntry -ZipPath $package -Entry "evidence/v0.17.0/numeric-conformance-evidence.json" -Description "v0.17 packaged numeric conformance evidence JSON"

Write-Host "result: pass"
Write-Host "v0.17.0 manifest and output request schema v2 verification passed."
