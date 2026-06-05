[CmdletBinding()]
param(
    [switch]$SkipOracleSmoke
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "common.ps1")
Add-CargoBinToPath

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
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

function Assert-DirectoryExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Container)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

Assert-FileExists -Path "rust-toolchain.toml" -Description "pinned Rust toolchain"
Assert-FileExists -Path "Cargo.toml" -Description "Cargo workspace"
Assert-FileExists -Path "Cargo.lock" -Description "Cargo lock"
Assert-FileExists -Path "docs\src\architecture\rust-only-policy.md" -Description "Rust-only policy"
Assert-FileExists -Path "docs\src\development-plan-v2.md" -Description "copied development plan"
Assert-FileExists -Path "tools\oracle\energyplus.lock.toml" -Description "EnergyPlus oracle lock"
Assert-FileExists -Path "tools\oracle\NOTICE.md" -Description "EnergyPlus oracle notice"
Assert-FileExists -Path "tools\docs\docs.lock.toml" -Description "docs tool lock"
Assert-FileExists -Path "config\local.toml" -Description "generated local config"
Assert-FileExists -Path ".reference\energyplus-src\26.1.0\source.sha256" -Description "reference source bootstrap digest"
Assert-DirectoryExists -Path ".runtime\energyplus\26.1.0" -Description "portable oracle runtime"
Assert-DirectoryExists -Path ".reference\energyplus-src\26.1.0" -Description "reference source tree"
Assert-FileExists -Path "data\testcases\minimal\case.toml" -Description "minimal testcase manifest"
Assert-FileExists -Path "data\testcases\minimal\raw-model.case.toml" -Description "RawModel testcase manifest"
Assert-FileExists -Path "data\testcases\minimal\typed-model.case.toml" -Description "TypedModel testcase manifest"
Assert-FileExists -Path "data\testcases\minimal\minimal.epJSON" -Description "RawModel fixture"
Assert-FileExists -Path "data\testcases\minimal\typed-model.epJSON" -Description "TypedModel fixture"
Assert-FileExists -Path "data\testcases\minimal\missing-reference.epJSON" -Description "TypedModel negative fixture"
Assert-FileExists -Path "data\testcases\minimal\invalid-enum.epJSON" -Description "TypedModel invalid enum fixture"
Assert-FileExists -Path "docs\src\operations\v0.1.0-readiness.md" -Description "v0.1 readiness document"
Assert-FileExists -Path "docs\src\releases\v0.1.0.md" -Description "v0.1 release note"
Assert-FileExists -Path "scripts\typed-model-smoke.ps1" -Description "TypedModel smoke script"

& (Join-Path $RepoRoot "scripts\source-smoke.ps1")
& (Join-Path $RepoRoot "scripts\check.ps1")

if (-not $SkipOracleSmoke) {
    & (Join-Path $RepoRoot "scripts\oracle-smoke.ps1")
}

& (Join-Path $RepoRoot "scripts\raw-model-smoke.ps1")
& (Join-Path $RepoRoot "scripts\typed-model-smoke.ps1")
& (Join-Path $RepoRoot "scripts\package.ps1") -Version "0.1.0"

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.1.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.1 release package"

Write-Host "v0.1.0 runnable release verification passed."
