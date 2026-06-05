[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Invoke-Inspect {
    param([Parameter(Mandatory = $true)][string]$Path)

    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -eq $cargo) {
        throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
    }

    $output = & $cargo.Source run -p ep_cli --quiet -- model inspect $Path 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "RawModel inspect failed for $Path"
    }

    return ($output -join "`n")
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch [regex]::Escape($Pattern)) {
        Write-Host $Text
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

$fixture = "data\testcases\minimal\minimal.epJSON"
if (-not (Test-Path -LiteralPath $fixture -PathType Leaf)) {
    throw "Missing RawModel fixture: $fixture"
}

Write-Host "Inspecting fixture: $fixture"
$fixtureOutput = Invoke-Inspect -Path $fixture
Assert-Contains -Text $fixtureOutput -Pattern "version: 26.1" -Description "fixture version"
Assert-Contains -Text $fixtureOutput -Pattern "object_types: 5" -Description "fixture object type count"
Assert-Contains -Text $fixtureOutput -Pattern "objects: 5" -Description "fixture object count"
Assert-Contains -Text $fixtureOutput -Pattern "Schedule:Constant: 1 [tracked]" -Description "fixture tracked object"
Assert-Contains -Text $fixtureOutput -Pattern "Unknown:Diagnostic: 1 [untracked]" -Description "fixture unknown object preservation"

$oracleEpjson = ".runtime\oracle-smoke\26.1.0\convert\smoke.epJSON"
if (-not (Test-Path -LiteralPath $oracleEpjson -PathType Leaf)) {
    Write-Host "Oracle smoke epJSON is missing; running oracle smoke first."
    Invoke-DevCommand -Command "oracle-smoke"
}

Write-Host "Inspecting oracle-generated epJSON: $oracleEpjson"
$oracleOutput = Invoke-Inspect -Path $oracleEpjson
Assert-Contains -Text $oracleOutput -Pattern "version: 26.1" -Description "oracle epJSON version"
Assert-Contains -Text $oracleOutput -Pattern "object_types: 28" -Description "oracle epJSON object type count"
Assert-Contains -Text $oracleOutput -Pattern "objects: 88" -Description "oracle epJSON object count"
Assert-Contains -Text $oracleOutput -Pattern "Building: 1 [tracked]" -Description "oracle tracked object report"
Assert-Contains -Text $oracleOutput -Pattern "Output:Variable: 48 [untracked]" -Description "oracle untracked object report"

Write-Host "RawModel smoke passed."
