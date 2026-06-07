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

$SourceRoot = ".reference\energyplus-src\26.1.0"

Assert-FileExists -Path "docs\src\releases\v0.14.0.md" -Description "v0.14 release notes"
Assert-FileExists -Path "docs\src\porting-map\plant-source-map.md" -Description "plant source map"
Assert-FileExists -Path "docs\src\porting-map\plant.md" -Description "plant porting map"
Assert-FileExists -Path "docs\src\conformance\output-variable-matrix.md" -Description "output variable matrix"
Assert-FileExists -Path "data\testcases\minimal\plant-loop-skeleton.epJSON" -Description "v0.13 plant-loop skeleton fixture"
Assert-FileExists -Path "scripts\smoke\plant-loop-skeleton-smoke.ps1" -Description "v0.13 plant-loop skeleton smoke"
Assert-FileExists -Path "scripts\release\conformance-evidence-report.ps1" -Description "numeric conformance evidence release script"

foreach ($relative in @(
    "src\EnergyPlus\Plant\PlantManager.cc",
    "src\EnergyPlus\Plant\PlantManager.hh",
    "src\EnergyPlus\Plant\Loop.hh",
    "src\EnergyPlus\Plant\LoopSide.hh",
    "src\EnergyPlus\Plant\LoopSide.cc",
    "src\EnergyPlus\Plant\Branch.hh",
    "src\EnergyPlus\Plant\Component.hh",
    "src\EnergyPlus\Plant\Component.cc",
    "src\EnergyPlus\Plant\DataPlant.hh",
    "src\EnergyPlus\Plant\PlantLocation.hh",
    "src\EnergyPlus\Plant\MixerData.hh",
    "src\EnergyPlus\Plant\SplitterData.hh",
    "src\EnergyPlus\PlantUtilities.cc",
    "src\EnergyPlus\PlantUtilities.hh",
    "src\EnergyPlus\Pumps.cc",
    "src\EnergyPlus\Pumps.hh",
    "src\EnergyPlus\Boilers.cc",
    "src\EnergyPlus\Boilers.hh",
    "src\EnergyPlus\ChillerElectricEIR.cc",
    "src\EnergyPlus\ChillerElectricEIR.hh"
)) {
    Assert-FileExists -Path (Join-Path $SourceRoot $relative) -Description "EnergyPlus plant reference source"
}

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "void ManagePlantLoops" -Description "EnergyPlus plant manager entry"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "void GetPlantLoopData" -Description "EnergyPlus plant input routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "void SetupInitialPlantCallingOrder" -Description "EnergyPlus plant calling order routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "FindLoopSideInCallingOrder" -Description "EnergyPlus plant calling order lookup"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "ReInitPlantLoopsAtFirstHVACIteration" -Description "EnergyPlus plant first-iteration reset"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "SizePlantLoop" -Description "EnergyPlus plant sizing routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "Plant Supply Side Cooling Demand Rate" -Description "EnergyPlus plant cooling demand output"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "Plant Supply Side Heating Demand Rate" -Description "EnergyPlus plant heating demand output"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "Plant Supply Side Inlet Temperature" -Description "EnergyPlus plant inlet temperature output"

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\LoopSide.hh" -Pattern "SimLoopSideNeeded" -Description "EnergyPlus plant loop-side simulation flag"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\LoopSide.hh" -Pattern "FlowLock" -Description "EnergyPlus plant flow lock field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\LoopSide.cc" -Pattern "HalfLoopData::simulate" -Description "EnergyPlus half-loop simulation routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\LoopSide.cc" -Pattern "EvaluateLoopSetPointLoad" -Description "EnergyPlus loop demand routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\LoopSide.cc" -Pattern "UpdateAnyLoopDemandAlterations" -Description "EnergyPlus loop demand alteration routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\LoopSide.cc" -Pattern "UpdatePlantMixer" -Description "EnergyPlus plant mixer update"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\LoopSide.cc" -Pattern "UpdatePlantSplitter" -Description "EnergyPlus plant splitter update"

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\Component.cc" -Pattern "CompData::simulate" -Description "EnergyPlus plant component dispatch"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\Component.hh" -Pattern "PlantEquipmentCtrlType" -Description "EnergyPlus plant equipment control type table"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\Component.hh" -Pattern '"Boiler:HotWater"' -Description "EnergyPlus boiler component enum entry"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\Component.hh" -Pattern '"Chiller:Electric:EIR"' -Description "EnergyPlus chiller component enum entry"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\Component.hh" -Pattern '"Pump:ConstantSpeed"' -Description "EnergyPlus pump component enum entry"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\DataPlant.hh" -Pattern "Array1D<DataPlant::PlantLoopData> PlantLoop" -Description "EnergyPlus plant loop storage"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantLocation.hh" -Pattern "struct PlantLocation" -Description "EnergyPlus plant location struct"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantLocation.hh" -Pattern "loopSideNum" -Description "EnergyPlus plant location side field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantLocation.hh" -Pattern "branchNum" -Description "EnergyPlus plant location branch field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantLocation.hh" -Pattern "compNum" -Description "EnergyPlus plant location component field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\MixerData.hh" -Pattern "struct MixerData" -Description "EnergyPlus plant mixer data"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\SplitterData.hh" -Pattern "struct SplitterData" -Description "EnergyPlus plant splitter data"

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PlantUtilities.cc" -Pattern "InitComponentNodes" -Description "EnergyPlus plant component node init"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PlantUtilities.cc" -Pattern "SetComponentFlowRate" -Description "EnergyPlus plant flow request"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PlantUtilities.cc" -Pattern "InterConnectTwoPlantLoopSides" -Description "EnergyPlus plant inter-loop connection"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PlantUtilities.cc" -Pattern "SafeCopyPlantNode" -Description "EnergyPlus plant node copy"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PlantUtilities.cc" -Pattern "ScanPlantLoopsForObject" -Description "EnergyPlus plant object scan"

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Pumps.cc" -Pattern "void SimPumps" -Description "EnergyPlus pump simulation routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Pumps.cc" -Pattern "Pump:ConstantSpeed" -Description "EnergyPlus constant-speed pump object"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Pumps.cc" -Pattern "Pump Electricity Rate" -Description "EnergyPlus pump electricity output"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Pumps.cc" -Pattern "ScanPlantLoopsForObject" -Description "EnergyPlus pump plant scan"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Boilers.cc" -Pattern "BoilerSpecs::simulate" -Description "EnergyPlus boiler simulation routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Boilers.cc" -Pattern "BoilerSpecs::onInitLoopEquip" -Description "EnergyPlus boiler plant init hook"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Boilers.cc" -Pattern "Boiler:HotWater" -Description "EnergyPlus hot-water boiler object"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Boilers.cc" -Pattern "Boiler Heating Rate" -Description "EnergyPlus boiler heating output"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ChillerElectricEIR.cc" -Pattern "ElectricEIRChillerSpecs::simulate" -Description "EnergyPlus chiller simulation routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ChillerElectricEIR.cc" -Pattern "ElectricEIRChillerSpecs::onInitLoopEquip" -Description "EnergyPlus chiller plant init hook"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ChillerElectricEIR.cc" -Pattern "Chiller:Electric:EIR" -Description "EnergyPlus electric EIR chiller object"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ChillerElectricEIR.cc" -Pattern "Chiller Electricity Rate" -Description "EnergyPlus chiller electricity output"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ChillerElectricEIR.cc" -Pattern "Chiller Evaporator Cooling Rate" -Description "EnergyPlus chiller cooling output"

Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "Reference version: EnergyPlus 26.1.0" -Description "plant map version"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "ManagePlantLoops" -Description "plant map manager entry"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "HalfLoopData::simulate" -Description "plant map half-loop routine"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "PlantUtilities::SetComponentFlowRate" -Description "plant map flow routine"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "Pump Electricity Rate" -Description "plant map pump output"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "Boiler Heating Rate" -Description "plant map boiler output"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "Chiller Evaporator Cooling Rate" -Description "plant map chiller output"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "Stop Rule" -Description "plant map stop rule"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "diagnostic-only evidence" -Description "plant map non-claim boundary"
Assert-Contains -Path "docs\src\porting-map\plant.md" -Pattern "v0.14 Plant Source Map" -Description "plant porting map v0.14 section"
Assert-Contains -Path "docs\src\conformance\output-variable-matrix.md" -Pattern "plant loop and equipment" -Description "plant output-variable matrix row"

Assert-Contains -Path "crates\ep_model\src\model.rs" -Pattern "PlantLoopBranchListEdge" -Description "PlantLoop graph edge model"
Assert-Contains -Path "crates\ep_model\src\model.rs" -Pattern "PlantBranchComponentEdge" -Description "Plant branch component edge model"
Assert-Contains -Path "crates\ep_compiler\src\compiler.rs" -Pattern "parse_plant_loops" -Description "PlantLoop compiler parser"
Assert-Contains -Path "crates\ep_compiler\src\compiler.rs" -Pattern "parse_pumps_constant_speed" -Description "pump compiler parser"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "plant_loop_branch_list_edges" -Description "PlantLoop CLI graph count"
Assert-Contains -Path "scripts\smoke\plant-loop-skeleton-smoke.ps1" -Pattern "Plant-loop skeleton smoke passed." -Description "v0.13 smoke success marker"
Assert-Contains -Path "data\testcases\minimal\plant-loop-skeleton.epJSON" -Pattern '"PlantLoop"' -Description "v0.13 plant fixture loop object"

Write-Host "milestone: v0.14.0"
Write-Host "scope: source-function map for plant loop and first equipment output paths"
Write-Host "claim: planning guard only; no plant numerical conformance"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "plant-loop-skeleton-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.14.0")

Assert-FileExists -Path ".runtime\release-evidence\v0.14.0\numeric-conformance-evidence.html" -Description "numeric conformance evidence HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.14.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.14.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"
Assert-Contains -Path ".runtime\release-evidence\v0.14.0\numeric-conformance-evidence.html" -Pattern "Table of Contents" -Description "numeric evidence table of contents"
Assert-Contains -Path ".runtime\release-evidence\v0.14.0\numeric-conformance-evidence.html" -Pattern "Accuracy Evidence" -Description "numeric evidence accuracy section"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.14.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.14.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.14 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.14.0.md" -Description "v0.14 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/porting-map/plant-source-map.md" -Description "v0.14 packaged plant source map"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/porting-map/plant.md" -Description "v0.14 packaged plant map"
Assert-ZipEntry -ZipPath $package -Entry "data/testcases/minimal/plant-loop-skeleton.epJSON" -Description "v0.14 packaged plant fixture"
Assert-ZipEntry -ZipPath $package -Entry "scripts/smoke/plant-loop-skeleton-smoke.ps1" -Description "v0.14 packaged plant smoke"

Write-Host "result: pass"
Write-Host "v0.14.0 plant source mapping verification passed."
