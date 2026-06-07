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

Assert-Contains -Path "Cargo.toml" -Pattern 'version = "0.21.0"' -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\v0.21.0.md" -Description "v0.21 release notes"
Assert-FileExists -Path "scripts\quality\algorithm-ledger-check.ps1" -Description "algorithm ledger check wrapper"
Assert-FileExists -Path "tools\docs\validate_algorithm_ledger.py" -Description "algorithm ledger validator"

Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern "source_map =" -Description "algorithm source-map field"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'status = "conformance"' -Description "promoted conformance ledger status"
Assert-Contains -Path "tools\docs\validate_algorithm_ledger.py" -Pattern "No source map, no algorithm port." -Description "source-map rule"
Assert-Contains -Path "tools\docs\validate_algorithm_ledger.py" -Pattern "conformance claim requires blocking gate" -Description "blocking gate rule"
Assert-Contains -Path "tools\docs\generate_docs.py" -Pattern '"Source map"' -Description "generated ledger source-map column"
Assert-Contains -Path "scripts\dev.ps1" -Pattern "algorithm-ledger-check" -Description "dev command registry"
Assert-Contains -Path "scripts\quality\check.ps1" -Pattern "algorithm-ledger-check" -Description "quality check ledger gate"

Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.21"' -Description "v0.21 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'status = "complete"' -Description "v0.21 completion status"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Source Map and Algorithm Ledger v1" -Description "v0.21 title"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "planning-guard"' -Description "v0.21 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "algorithm completion without source map" -Description "v0.21 non-claim boundary"

Write-Host "milestone: v0.21.0"
Write-Host "scope: source-map and algorithm ledger validation gate"
Write-Host "claim: planning guard only; no new numerical conformance"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "algorithm-ledger-check"
Invoke-DevCommand -Command "docs-generate"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern "Source map" -Description "generated algorithm ledger source-map column"
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", "0.21.0")
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.21.0")
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Assert-FileExists -Path ".runtime\release-evidence\v0.21.0\conformance-index-report.pdf" -Description "conformance index PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.21.0\conformance-index-report.json" -Description "conformance index JSON"
Assert-FileExists -Path ".runtime\release-evidence\v0.21.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.21.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.21.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.21.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.21 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.21.0.md" -Description "v0.21 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "scripts/quality/algorithm-ledger-check.ps1" -Description "v0.21 packaged ledger gate"
Assert-ZipEntry -ZipPath $package -Entry "tools/docs/validate_algorithm_ledger.py" -Description "v0.21 packaged ledger validator"
Assert-ZipEntry -ZipPath $package -Entry "specs/algorithm_ledger.toml" -Description "v0.21 packaged algorithm ledger"

Write-Host "result: pass"
Write-Host "v0.21.0 source map and algorithm ledger verification passed."
