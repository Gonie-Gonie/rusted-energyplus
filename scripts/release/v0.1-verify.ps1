[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot
$Version = "0.1.0"
$Tag = "v$Version"

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

Assert-Contains -Path "Cargo.toml" -Pattern "version = `"$Version`"" -Description "workspace version"
Assert-FileExists -Path "docs\src\releases\$Tag.md" -Description "v0.1.0 release notes"
Assert-Contains -Path "docs\src\releases\$Tag.md" -Pattern "limited-conformance" -Description "release conformance scope"
Assert-Contains -Path "docs\src\releases\$Tag.md" -Pattern "conformance_claim=false" -Description "arbitrary run non-claim boundary"
Assert-Contains -Path ".github\workflows\release.yml" -Pattern 'tags:' -Description "tag release workflow"
Assert-Contains -Path ".github\workflows\release.yml" -Pattern 'v*.*.*' -Description "semver tag trigger"
Assert-Contains -Path "scripts\gui\build-launcher-exe.ps1" -Pattern "WindowsApplication" -Description "no-console launcher builder"
Assert-Contains -Path "scripts\release\package.ps1" -Pattern "eplus-rs-launch.exe" -Description "packaged launcher exe"
Assert-Contains -Path "specs\capabilities.toml" -Pattern "conformance_claim=true" -Description "arbitrary run claim boundary"

Write-Host "release: $Tag"
Write-Host "scope: public limited-conformance pre-1.0 release"

Invoke-DevCommand -Command "build-launcher-exe" -Arguments @("-SelfTest")
Invoke-DevCommand -Command "launch-ui" -Arguments @("-SelfTest")
Invoke-DevCommand -Command "arbitrary-run-smoke"
Invoke-DevCommand -Command "manifest-validate-all"
Invoke-DevCommand -Command "docs-generate"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "conformance-index-report" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "support-coverage-report" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "user-coverage-handbook" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "package" -Arguments @("-Version", $Version)
Invoke-DevCommand -Command "release-evidence-manifest" -Arguments @("-Version", $Version)

$package = Join-Path $RepoRoot "dist\eplus-rs-$Tag-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.1.0 release package"
Assert-ZipEntry -ZipPath $package -Entry "bin/eplus-rs.exe" -Description "packaged CLI"
Assert-ZipEntry -ZipPath $package -Entry "eplus-rs-launch.exe" -Description "packaged no-console launcher"
Assert-ZipEntry -ZipPath $package -Entry "scripts/gui/eplus-rs-launch.ps1" -Description "packaged launcher UI script"
Assert-ZipEntry -ZipPath $package -Entry "scripts/gui/build-launcher-exe.ps1" -Description "packaged launcher builder"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.1.0.md" -Description "packaged release notes"
Assert-ZipEntry -ZipPath $package -Entry "specs/capabilities.toml" -Description "packaged capabilities registry"
Assert-ZipEntry -ZipPath $package -Entry "oracle/energyplus/26.1.0/energyplus.exe" -Description "packaged EnergyPlus oracle"
Assert-ZipEntry -ZipPath $package -Entry "oracle/energyplus/26.1.0/ConvertInputFormat.exe" -Description "packaged EnergyPlus converter"

Assert-FileExists -Path ".runtime\release-evidence\$Tag\numeric-conformance-evidence.pdf" -Description "numeric evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\$Tag\conformance-index-report.pdf" -Description "conformance index PDF"
Assert-FileExists -Path ".runtime\release-evidence\$Tag\support-coverage-report.pdf" -Description "support coverage PDF"
Assert-FileExists -Path ".runtime\release-evidence\$Tag\user-coverage-handbook.pdf" -Description "user handbook PDF"
Assert-FileExists -Path ".runtime\release-evidence\$Tag\release-evidence-manifest.json" -Description "release manifest JSON"

$publicAssets = @(
    & ".\scripts\release\select-release-assets.ps1" `
        -Artifact $package `
        -EvidenceRoot ".runtime\release-evidence\$Tag" `
        -RequireEvidenceAssets |
        ForEach-Object { Split-Path -Leaf $_ }
)
$expectedPublicAssets = @(
    "eplus-rs-$Tag-windows-x64.zip",
    "numeric-conformance-evidence.pdf",
    "conformance-index-report.pdf",
    "support-coverage-report.pdf",
    "user-coverage-handbook.pdf"
)
$missingPublicAssets = @($expectedPublicAssets | Where-Object { $publicAssets -notcontains $_ })
$unexpectedPublicAssets = @($publicAssets | Where-Object { $expectedPublicAssets -notcontains $_ })
if ($missingPublicAssets.Count -gt 0 -or $unexpectedPublicAssets.Count -gt 0) {
    throw "Unexpected public release asset set. Missing=[$($missingPublicAssets -join ', ')] Unexpected=[$($unexpectedPublicAssets -join ', ')]"
}
Write-Host "OK curated public release asset set: $($expectedPublicAssets -join ', ')"

$numeric = Get-Content -LiteralPath ".runtime\release-evidence\$Tag\numeric-conformance-evidence.json" -Raw | ConvertFrom-Json
if ($numeric.aggregate.status -ne "pass") {
    throw "Expected numeric conformance evidence status pass, found $($numeric.aggregate.status)"
}
if ($numeric.aggregate.case_count -lt 1) {
    throw "Expected at least one promoted conformance case"
}

$manifest = Get-Content -LiteralPath ".runtime\release-evidence\$Tag\release-evidence-manifest.json" -Raw | ConvertFrom-Json
if ($manifest.aggregate.missing_required_asset_count -ne 0) {
    throw "Expected no missing required release assets, found $($manifest.aggregate.missing_required_asset_count)"
}
if ($manifest.aggregate.present_required_asset_count -lt 1) {
    throw "Expected release manifest to record present assets"
}

Write-Host "result: pass"
Write-Host "v0.1.0 public release verification passed."
