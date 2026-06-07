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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.24.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.24.0.md" -Description "v0.24 release notes"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.24"' -Description "v0.24 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Runtime State and Output Registry Hardening" -Description "v0.24 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'status = "complete"' -Description "v0.24 completion status"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "runtime-infrastructure"' -Description "v0.24 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "new numerical conformance" -Description "v0.24 numerical non-claim"
Assert-Contains -Path "specs\milestones.toml" -Pattern "meter conformance" -Description "v0.24 meter non-claim"

Assert-Contains -Path "crates\ep_runtime\src\lib.rs" -Pattern "mod output;" -Description "runtime output module"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "RuntimeOutputRegistry" -Description "runtime output registry"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "RuntimeMeterRegistry" -Description "runtime meter registry"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "OutputVariableUnavailable" -Description "unavailable output diagnostic"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "MeterUnavailable" -Description "unavailable meter diagnostic"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "ResultStoreProfile" -Description "result profile scaffold"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "RuntimeOutputRegistry::from_model" -Description "execution plan registry handles"
Assert-Contains -Path "scripts\smoke\runtime-registry-smoke.ps1" -Pattern "Runtime registry smoke passed." -Description "runtime registry smoke"
Assert-Contains -Path "scripts\smoke\model-plan-smoke.ps1" -Pattern "WriteOutput(4)" -Description "registry-backed plan smoke"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "runtime-registry-smoke" -Description "quality gate wiring"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "v0.24-verify" -Description "dev command wiring"

Write-Host "milestone: v0.24.0"
Write-Host "scope: runtime state, output registry, meter registry, ResultStore diagnostics"
Write-Host "claim: runtime infrastructure only; no new numerical or meter conformance"

Invoke-DevCommand -Command "runtime-registry-smoke"
Invoke-DevCommand -Command "model-plan-smoke"
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\milestone-map.md" -Pattern "| 0.24 | Runtime State and Output Registry Hardening | complete" -Description "generated milestone status"
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.24.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.24.0")
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "file-size-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Assert-FileExists -Path ".runtime\release-evidence\v0.24.0\conformance-index-report.pdf" -Description "conformance index PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.24.0\conformance-index-report.json" -Description "conformance index JSON"
Assert-FileExists -Path ".runtime\release-evidence\v0.24.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.24.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

$index = Get-Content -LiteralPath ".runtime\release-evidence\v0.24.0\conformance-index-report.json" -Raw | ConvertFrom-Json
if ($index.aggregate.case_count -ne 13) {
    throw "Expected 13 indexed cases, found $($index.aggregate.case_count)"
}
if ($index.aggregate.conformance_case_count -ne 5) {
    throw "Expected 5 conformance cases in index, found $($index.aggregate.conformance_case_count)"
}

$evidence = Get-Content -LiteralPath ".runtime\release-evidence\v0.24.0\numeric-conformance-evidence.json" -Raw | ConvertFrom-Json
if ($evidence.aggregate.case_count -ne 4) {
    throw "Expected 4 promoted numerical conformance cases, found $($evidence.aggregate.case_count)"
}
if ($evidence.aggregate.series_count -ne 6) {
    throw "Expected 6 promoted numerical conformance series, found $($evidence.aggregate.series_count)"
}
if ($evidence.cases | Where-Object { $_.case_id -eq "official_1zone_static_model_001" }) {
    throw "Static model evidence must not be mixed into the numeric conformance PDF"
}

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.24.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.24.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.24 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.24.0.md" -Description "v0.24 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "scripts/smoke/runtime-registry-smoke.ps1" -Description "v0.24 packaged runtime registry smoke"
Assert-ZipEntry -ZipPath $package -Entry "specs/milestones.toml" -Description "v0.24 packaged milestone spec"

Write-Host "result: pass"
Write-Host "v0.24.0 runtime registry verification passed."
