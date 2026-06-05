[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$SourceRoot = Join-Path $RepoRoot ".reference\energyplus-src\26.1.0"
$RuntimeRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "Found $Description`: $Path"
}

function Assert-DirectoryExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Container)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "Found $Description`: $Path"
}

Assert-DirectoryExists -Path $SourceRoot -Description "EnergyPlus 26.1.0 reference source"
Assert-DirectoryExists -Path $RuntimeRoot -Description "EnergyPlus 26.1.0 runtime oracle"

$expectedSourceFiles = @(
    @("CMakeLists.txt", "source build root marker"),
    @("README.md", "source README"),
    @("LICENSE.txt", "source license"),
    @("testfiles\1ZoneUncontrolled.idf", "minimal source testfile"),
    @("idd\schema\generate_epJSON_schema.py", "epJSON schema generation script"),
    @("idd\embedded\generate_embeddable_epJSON_schema.cpp", "embedded epJSON schema generator"),
    @("src\EnergyPlus\PlantUtilities.cc", "plant utility reference source"),
    @("src\EnergyPlus\HVACInterfaceManager.cc", "HVAC interface manager reference source"),
    @("src\EnergyPlus\Plant\DataPlant.hh", "plant data reference header"),
    @("tst\EnergyPlus\unit\PlantUtilities.unit.cc", "plant utility unit reference"),
    @("tst\EnergyPlus\unit\HVACInterfaceManager.unit.cc", "HVAC interface unit reference")
)

foreach ($entry in $expectedSourceFiles) {
    Assert-FileExists -Path (Join-Path $SourceRoot $entry[0]) -Description $entry[1]
}

$expectedRuntimeFiles = @(
    @("energyplus.exe", "oracle executable"),
    @("ConvertInputFormat.exe", "IDF to epJSON converter"),
    @("Energy+.schema.epJSON", "packaged epJSON schema"),
    @("ExampleFiles\1ZoneUncontrolled.idf", "runtime example IDF"),
    @("WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw", "runtime smoke EPW")
)

foreach ($entry in $expectedRuntimeFiles) {
    Assert-FileExists -Path (Join-Path $RuntimeRoot $entry[0]) -Description $entry[1]
}

$testfileCount = (Get-ChildItem -LiteralPath (Join-Path $SourceRoot "testfiles") -File -Filter "*.idf").Count
if ($testfileCount -lt 1) {
    throw "Expected at least one source IDF testfile."
}
Write-Host "Source IDF testfile count: $testfileCount"

$schemaPath = Join-Path $RuntimeRoot "Energy+.schema.epJSON"
$schemaFirstLine = Get-Content -LiteralPath $schemaPath -TotalCount 1
if ($schemaFirstLine -notmatch "^\s*\{") {
    throw "Packaged schema does not look like JSON: $schemaPath"
}

Write-Host "Source smoke passed."
