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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.18.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.18.0.md" -Description "v0.18 release notes"
Assert-FileExists -Path "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml" -Description "official baseline case manifest"
Assert-FileExists -Path "scripts\conformance\official-baseline-smoke.ps1" -Description "official baseline smoke gate"

Assert-Contains -Path "crates\ep_cli\src\conformance_artifacts.rs" -Pattern "stage_idf_with_output_requests" -Description "output injection staging"
Assert-Contains -Path "crates\ep_cli\src\conformance_artifacts.rs" -Pattern "render_output_request_injection" -Description "output injection renderer"
Assert-Contains -Path "crates\ep_cli\src\conformance_artifacts.rs" -Pattern "rusted-energyplus.output-injection.v1" -Description "expanded manifest injection schema"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "injected_outputs" -Description "baseline summary injection count"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "official-baseline-smoke" -Description "dev command registry"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "official-baseline-smoke" -Description "quality check official baseline gate"

Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml" -Pattern 'source_kind = "energy-plus-examplefile"' -Description "official source kind"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml" -Pattern 'level = "baseline"' -Description "baseline output level"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml" -Pattern "conformance_claim = false" -Description "baseline-only claim boundary"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml" -Pattern "scripts/dev.cmd official-baseline-smoke" -Description "case smoke gate"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.18"' -Description "v0.18 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'status = "complete"' -Description "v0.18 completion status"
Assert-Contains -Path "specs\milestones.toml" -Pattern "official_1zone_uncontrolled_baseline_001" -Description "v0.18 required case"
Assert-Contains -Path "specs\milestones.toml" -Pattern "ExampleFiles numerical conformance" -Description "v0.18 non-claim boundary"
Assert-Contains -Path "docs\src\conformance\numeric-release-evidence.md" -Pattern "earlier v0.8/v0.9 cases only" -Description "numeric evidence exclusion boundary"

Write-Host "milestone: v0.18.0"
Write-Host "scope: output request injection and official oracle baseline pipeline"
Write-Host "claim: baseline-only; no new numerical conformance or ExampleFiles compatibility"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "conformance-baseline-smoke"
Invoke-DevCommand -Command "official-baseline-smoke"
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.18.0")

Assert-FileExists -Path ".runtime\release-evidence\v0.18.0\numeric-conformance-evidence.html" -Description "numeric conformance evidence HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.18.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.18.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.18.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.18.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.18 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.18.0.md" -Description "v0.18 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "scripts/conformance/official-baseline-smoke.ps1" -Description "v0.18 packaged official baseline gate"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/official_1zone_uncontrolled_baseline_001/case.toml" -Description "v0.18 packaged official case"

Write-Host "result: pass"
Write-Host "v0.18.0 output injection and official baseline verification passed."
