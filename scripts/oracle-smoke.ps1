[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\oracle-smoke\26.1.0"

function New-Directory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        New-Item -ItemType Directory -Force -Path $Path | Out-Null
    }
}

function Invoke-External {
    param(
        [Parameter(Mandatory = $true)][string]$FilePath,
        [Parameter(Mandatory = $true)][string[]]$Arguments
    )
    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed ($LASTEXITCODE): $FilePath $($Arguments -join ' ')"
    }
}

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
if (-not (Test-Path -LiteralPath $energyPlus)) {
    throw "EnergyPlus oracle is missing. Run .\scripts\setup.ps1 first."
}

Write-Host "EnergyPlus executable: $energyPlus"
Invoke-External -FilePath $energyPlus -Arguments @("--version")

$exampleDir = Join-Path $OracleRoot "ExampleFiles"
$weatherDir = Join-Path $OracleRoot "WeatherData"
$idf = Join-Path $exampleDir "1ZoneUncontrolled.idf"
if (-not (Test-Path -LiteralPath $idf)) {
    $idfItem = Get-ChildItem -LiteralPath $exampleDir -Filter "*.idf" -File | Select-Object -First 1
    if ($null -eq $idfItem) {
        Write-Warning "No bundled IDF example found; version smoke passed."
        return
    }
    $idf = $idfItem.FullName
}

$weather = Join-Path $weatherDir "USA_CO_Golden-NREL.724666_TMY3.epw"
if (-not (Test-Path -LiteralPath $weather)) {
    $weatherItem = Get-ChildItem -LiteralPath $weatherDir -Filter "*.epw" -File | Select-Object -First 1
    if ($null -eq $weatherItem) {
        Write-Warning "No bundled EPW weather file found; version smoke passed."
        return
    }
    $weather = $weatherItem.FullName
}

New-Directory -Path $OutputRoot
Write-Host "Running oracle smoke case:"
Write-Host "  IDF: $idf"
Write-Host "  EPW: $weather"
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$errFile = Join-Path $OutputRoot "eplusout.err"
if (-not (Test-Path -LiteralPath $errFile)) {
    throw "Oracle smoke did not produce eplusout.err"
}

$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
if (Test-Path -LiteralPath $converter) {
    $convertDir = Join-Path $OutputRoot "convert"
    New-Directory -Path $convertDir
    $copiedIdf = Join-Path $convertDir "smoke.idf"
    Copy-Item -LiteralPath $idf -Destination $copiedIdf -Force
    Push-Location $convertDir
    try {
        Invoke-External -FilePath $converter -Arguments @("smoke.idf")
    }
    finally {
        Pop-Location
    }
}

Write-Host "Oracle smoke passed."

