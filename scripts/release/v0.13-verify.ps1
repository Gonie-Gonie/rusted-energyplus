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

Assert-FileExists -Path "docs\src\releases\v0.13.0.md" -Description "v0.13 release notes"
Assert-FileExists -Path "docs\src\porting-map\plant.md" -Description "plant porting map"
Assert-FileExists -Path "data\testcases\minimal\plant-loop-skeleton.epJSON" -Description "v0.13 plant-loop skeleton fixture"
Assert-FileExists -Path "scripts\smoke\plant-loop-skeleton-smoke.ps1" -Description "v0.13 plant-loop skeleton smoke"
Assert-FileExists -Path "scripts\release\conformance-evidence-report.ps1" -Description "numeric conformance evidence release script"
Assert-FileExists -Path "scripts\lib\python.ps1" -Description "portable Python setup library"
Assert-FileExists -Path "scripts\setup\python-smoke.ps1" -Description "report Python smoke"
Assert-FileExists -Path "tools\python\requirements-report.txt" -Description "report Python requirements"
Assert-FileExists -Path "tools\reporting\conformance_evidence_report.py" -Description "oodocs numerical evidence generator"

foreach ($relative in @(
    "src\EnergyPlus\Plant\PlantManager.cc",
    "src\EnergyPlus\Plant\Loop.cc",
    "src\EnergyPlus\Plant\LoopSide.cc",
    "src\EnergyPlus\Plant\Branch.cc",
    "src\EnergyPlus\Plant\Component.cc",
    "src\EnergyPlus\Plant\DataPlant.hh",
    "src\EnergyPlus\Plant\MixerData.hh",
    "src\EnergyPlus\Plant\SplitterData.hh",
    "src\EnergyPlus\Pumps.cc",
    "src\EnergyPlus\Boilers.cc",
    "src\EnergyPlus\ChillerElectricEIR.cc"
)) {
    Assert-FileExists -Path (Join-Path $SourceRoot $relative) -Description "EnergyPlus plant reference source"
}

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\PlantManager.cc" -Pattern "PlantLoop" -Description "EnergyPlus plant manager loop marker"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\Loop.cc" -Pattern "Loop" -Description "EnergyPlus plant loop source marker"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Plant\Branch.cc" -Pattern "Branch" -Description "EnergyPlus plant branch source marker"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Pumps.cc" -Pattern "Pump:ConstantSpeed" -Description "EnergyPlus pump object marker"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\Boilers.cc" -Pattern "Boiler:HotWater" -Description "EnergyPlus boiler object marker"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ChillerElectricEIR.cc" -Pattern "Chiller:Electric:EIR" -Description "EnergyPlus chiller object marker"

Assert-Contains -Path "docs\src\porting-map\plant.md" -Pattern "v0.13 Plant Loop Skeleton" -Description "plant porting map v0.13 section"
Assert-Contains -Path "docs\src\porting-map\plant.md" -Pattern "typed graph smoke" -Description "plant porting map smoke boundary"
Assert-Contains -Path "docs\src\operations\supported-object-coverage.md" -Pattern "PlantLoop | yes | partial | yes | yes | yes | partial | no | no" -Description "PlantLoop object coverage boundary"
Assert-Contains -Path "docs\src\conformance\numeric-release-evidence.md" -Pattern "oodocs" -Description "numeric evidence oodocs documentation"
Assert-Contains -Path "docs\src\conformance\numeric-release-evidence.md" -Pattern "matplotlib" -Description "numeric evidence matplotlib documentation"

Assert-Contains -Path "crates\ep_model\src\model.rs" -Pattern "PlantLoopBranchListEdge" -Description "PlantLoop graph edge model"
Assert-Contains -Path "crates\ep_model\src\model.rs" -Pattern "PlantBranchComponentEdge" -Description "Plant branch component edge model"
Assert-Contains -Path "crates\ep_compiler\src\compiler.rs" -Pattern "parse_plant_loops" -Description "PlantLoop compiler parser"
Assert-Contains -Path "crates\ep_compiler\src\compiler.rs" -Pattern "parse_pumps_constant_speed" -Description "pump compiler parser"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "plant_loop_branch_list_edges" -Description "PlantLoop CLI graph count"
Assert-Contains -Path "scripts\smoke\plant-loop-skeleton-smoke.ps1" -Pattern "Plant-loop skeleton smoke passed." -Description "v0.13 smoke success marker"
Assert-Contains -Path "data\testcases\minimal\plant-loop-skeleton.epJSON" -Pattern '"PlantLoop"' -Description "v0.13 plant fixture loop object"
Assert-Contains -Path "tools\python\requirements-report.txt" -Pattern "oodocs==1.0.1" -Description "oodocs dependency pin"
Assert-Contains -Path "tools\python\requirements-report.txt" -Pattern "matplotlib==3.10.9" -Description "matplotlib dependency pin"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern "TableOfContents" -Description "evidence report table of contents"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern 'matplotlib.use("Agg")' -Description "matplotlib headless backend"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern 'Figure(charts["accuracy"]' -Description "direct matplotlib figure insertion"
Assert-Contains -Path "scripts\release\conformance-evidence-report.ps1" -Pattern "Get-ReportPythonExe" -Description "Python report wrapper"

Write-Host "milestone: v0.13.0"
Write-Host "scope: PlantLoop typed graph skeleton"
Write-Host "claim: smoke only; no plant numerical conformance"

Invoke-DevCommand -Command "python-smoke"
Invoke-DevCommand -Command "plant-loop-skeleton-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.13.0")

Assert-FileExists -Path ".runtime\release-evidence\v0.13.0\numeric-conformance-evidence.html" -Description "numeric conformance evidence HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.13.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.13.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"
Assert-Contains -Path ".runtime\release-evidence\v0.13.0\numeric-conformance-evidence.html" -Pattern "Table of Contents" -Description "numeric evidence table of contents"
Assert-Contains -Path ".runtime\release-evidence\v0.13.0\numeric-conformance-evidence.html" -Pattern "Accuracy Evidence" -Description "numeric evidence accuracy section"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.13.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.13.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.13 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.13.0.md" -Description "v0.13 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/porting-map/plant.md" -Description "v0.13 packaged plant map"
Assert-ZipEntry -ZipPath $package -Entry "data/testcases/minimal/plant-loop-skeleton.epJSON" -Description "v0.13 packaged plant fixture"
Assert-ZipEntry -ZipPath $package -Entry "scripts/smoke/plant-loop-skeleton-smoke.ps1" -Description "v0.13 packaged plant smoke"
Assert-ZipEntry -ZipPath $package -Entry "scripts/lib/python.ps1" -Description "v0.13 packaged portable Python library"
Assert-ZipEntry -ZipPath $package -Entry "scripts/setup/python-smoke.ps1" -Description "v0.13 packaged Python smoke"
Assert-ZipEntry -ZipPath $package -Entry "tools/python/requirements-report.txt" -Description "v0.13 packaged report requirements"
Assert-ZipEntry -ZipPath $package -Entry "tools/reporting/conformance_evidence_report.py" -Description "v0.13 packaged oodocs evidence generator"

Write-Host "result: pass"
Write-Host "v0.13.0 PlantLoop typed graph skeleton verification passed."
