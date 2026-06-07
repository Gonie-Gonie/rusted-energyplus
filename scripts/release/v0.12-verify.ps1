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

Assert-FileExists -Path "docs\src\conformance\numeric-release-evidence.md" -Description "numeric release evidence docs"
Assert-FileExists -Path "docs\src\porting-map\node-state-source-map.md" -Description "node-state source map"
Assert-FileExists -Path "docs\src\porting-map\output-variable-source-map.md" -Description "output-variable source map"
Assert-FileExists -Path "docs\src\porting-map\hvac.md" -Description "HVAC porting map"
Assert-FileExists -Path "scripts\release\conformance-evidence-report.ps1" -Description "numeric conformance evidence release script"

foreach ($relative in @(
    "src\EnergyPlus\NodeInputManager.cc",
    "src\EnergyPlus\NodeInputManager.hh",
    "src\EnergyPlus\DataLoopNode.hh",
    "src\EnergyPlus\DataZoneEquipment.hh",
    "src\EnergyPlus\DataZoneEquipment.cc",
    "src\EnergyPlus\DataHeatBalance.hh",
    "src\EnergyPlus\ZoneTempPredictorCorrector.cc",
    "src\EnergyPlus\ZoneEquipmentManager.cc",
    "src\EnergyPlus\PurchasedAirManager.cc",
    "src\EnergyPlus\OutputProcessor.cc"
)) {
    Assert-FileExists -Path (Join-Path $SourceRoot $relative) -Description "EnergyPlus node reference source"
}

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\NodeInputManager.cc" -Pattern "System Node Temperature" -Description "EnergyPlus node temperature registration"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\NodeInputManager.cc" -Pattern "System Node Mass Flow Rate" -Description "EnergyPlus node flow registration"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\NodeInputManager.cc" -Pattern "System Node Humidity Ratio" -Description "EnergyPlus node humidity registration"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\NodeInputManager.cc" -Pattern "System Node Setpoint Temperature" -Description "EnergyPlus node setpoint registration"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\NodeInputManager.cc" -Pattern "AssignNodeNumber" -Description "EnergyPlus node allocation routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\NodeInputManager.cc" -Pattern "CalcMoreNodeInfo" -Description "EnergyPlus derived node reporting routine"

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataLoopNode.hh" -Pattern "struct NodeData" -Description "EnergyPlus node data struct"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataLoopNode.hh" -Pattern "Real64 Temp = 0.0" -Description "EnergyPlus node temperature field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataLoopNode.hh" -Pattern "Real64 MassFlowRate = 0.0" -Description "EnergyPlus node mass-flow field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataLoopNode.hh" -Pattern "Real64 HumRat = 0.0" -Description "EnergyPlus node humidity field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataLoopNode.hh" -Pattern "Real64 TempSetPoint = SensedNodeFlagValue" -Description "EnergyPlus node setpoint sentinel"

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataZoneEquipment.hh" -Pattern "int ZoneNode" -Description "EnergyPlus zone node field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataZoneEquipment.hh" -Pattern "Array1D_int InletNode" -Description "EnergyPlus inlet node field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataZoneEquipment.hh" -Pattern "Array1D_int ReturnNode" -Description "EnergyPlus return node field"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataZoneEquipment.cc" -Pattern "setTotalInletFlows" -Description "EnergyPlus zone node flow aggregation"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\DataHeatBalance.hh" -Pattern "SystemZoneNodeNumber" -Description "EnergyPlus zone system node field"

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PurchasedAirManager.cc" -Pattern "ZoneHVAC:IdealLoadsAirSystem" -Description "EnergyPlus IdealLoads input object"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PurchasedAirManager.cc" -Pattern "CalcPurchAirLoads" -Description "EnergyPlus IdealLoads node update routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PurchasedAirManager.cc" -Pattern "Node(InNodeNum).Temp" -Description "EnergyPlus IdealLoads supply temperature write"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PurchasedAirManager.cc" -Pattern "Node(InNodeNum).HumRat" -Description "EnergyPlus IdealLoads supply humidity write"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PurchasedAirManager.cc" -Pattern "Node(InNodeNum).MassFlowRate" -Description "EnergyPlus IdealLoads supply flow write"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\PurchasedAirManager.cc" -Pattern "Node(RecircNodeNum).MassFlowRate" -Description "EnergyPlus IdealLoads return flow write"

Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ZoneTempPredictorCorrector.cc" -Pattern "thisSystemNode.Temp" -Description "EnergyPlus zone node temperature write"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ZoneTempPredictorCorrector.cc" -Pattern "Node(ZoneNodeNum).HumRat" -Description "EnergyPlus zone node humidity write"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ZoneTempPredictorCorrector.cc" -Pattern "TempSetPoint = ZoneSetPoint" -Description "EnergyPlus zone node setpoint write"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ZoneEquipmentManager.cc" -Pattern "CalcZoneLeavingConditions" -Description "EnergyPlus return-node update routine"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ZoneEquipmentManager.cc" -Pattern "Node(ReturnNode).Temp" -Description "EnergyPlus return-node temperature write"
Assert-Contains -Path "$SourceRoot\src\EnergyPlus\ZoneEquipmentManager.cc" -Pattern "Node(ReturnNode).HumRat" -Description "EnergyPlus return-node humidity write"

Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "Reference version: EnergyPlus 26.1.0" -Description "node map version"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "AssignNodeNumber" -Description "node map allocation routine"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "CalcPurchAirLoads" -Description "node map IdealLoads routine"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "CalcZoneLeavingConditions" -Description "node map return routine"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "setTotalInletFlows" -Description "node map flow aggregation"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "TempSetPoint" -Description "node map setpoint boundary"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "NodeStateStore" -Description "node map Rust state store"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "Stop Rule" -Description "node map stop rule"

Assert-Contains -Path "docs\src\conformance\numeric-release-evidence.md" -Pattern "release PDF/HTML/JSON evidence pack" -Description "numeric evidence release policy"
Assert-Contains -Path "docs\src\conformance\numeric-release-evidence.md" -Pattern "v0.8/v0.9 cases only" -Description "numeric evidence claim boundary"
Assert-Contains -Path "docs\src\porting-map\output-variable-source-map.md" -Pattern "node-state-source-map.md" -Description "output variable node source map"
Assert-Contains -Path "docs\src\porting-map\hvac.md" -Pattern "v0.12 Node Source Map" -Description "HVAC node source map section"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "NodeStateProjectionEvidencePolicy" -Description "Rust node projection evidence policy"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "NODE_TEMPERATURE_SETPOINT_SENTINEL_C" -Description "Rust node setpoint sentinel"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "evidence_policy" -Description "node projection JSON evidence policy"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "timestamp_rule" -Description "node projection timestamp gate"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "sentinel_rule" -Description "node projection sentinel gate"

Write-Host "milestone: v0.12.0"
Write-Host "scope: source-function map for system-node output registration and update paths"
Write-Host "claim: planning guard only; no node, IdealLoads, or HVAC numerical conformance"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "air-side-node-diagnostic-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "conformance-evidence-report" -Arguments @("-Version", "0.12.0")

Assert-FileExists -Path ".runtime\release-evidence\v0.12.0\numeric-conformance-evidence.html" -Description "numeric conformance evidence HTML"
Assert-FileExists -Path ".runtime\release-evidence\v0.12.0\numeric-conformance-evidence.pdf" -Description "numeric conformance evidence PDF"
Assert-FileExists -Path ".runtime\release-evidence\v0.12.0\numeric-conformance-evidence.json" -Description "numeric conformance evidence JSON"

Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.12.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.12.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.12 release package"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.12.0.md" -Description "v0.12 packaged release note"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/porting-map/node-state-source-map.md" -Description "v0.12 packaged node source map"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/air_side_node_diagnostic_001/case.toml" -Description "v0.12 packaged node case manifest"

Write-Host "result: pass"
Write-Host "v0.12.0 node source mapping verification passed."
