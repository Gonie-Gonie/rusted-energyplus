[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$Fixture = Join-Path $RepoRoot "data\testcases\minimal\plant-loop-skeleton.epJSON"

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

if (-not (Test-Path -LiteralPath $Fixture -PathType Leaf)) {
    throw "Missing v0.13 plant-loop skeleton fixture: $Fixture"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Compiling v0.13 plant-loop skeleton fixture."
$compileOutput = & $cargo.Source run -p ep_cli --quiet -- compile $Fixture 2>&1
if ($LASTEXITCODE -ne 0) {
    $compileOutput | ForEach-Object { Write-Host $_ }
    throw "Plant-loop skeleton typed compile failed."
}
$compileText = ($compileOutput -join "`n")
Assert-Contains -Text $compileText -Pattern "TypedModel" -Description "compile header"
Assert-Contains -Text $compileText -Pattern "plant_loops: 1" -Description "PlantLoop typed count"
Assert-Contains -Text $compileText -Pattern "plant_branches: 4" -Description "Branch typed count"
Assert-Contains -Text $compileText -Pattern "plant_branch_lists: 3" -Description "BranchList typed count"
Assert-Contains -Text $compileText -Pattern "plant_connectors: 2" -Description "connector typed count"
Assert-Contains -Text $compileText -Pattern "plant_connector_lists: 1" -Description "ConnectorList typed count"
Assert-Contains -Text $compileText -Pattern "pumps_constant_speed: 1" -Description "Pump typed count"
Assert-Contains -Text $compileText -Pattern "boilers_hot_water: 1" -Description "Boiler typed count"
Assert-Contains -Text $compileText -Pattern "chillers_electric_eir: 1" -Description "Chiller typed count"
Assert-Contains -Text $compileText -Pattern "nodes: 9" -Description "plant node registry count"
Assert-Contains -Text $compileText -Pattern "PlantLoop: 1 [typed]" -Description "PlantLoop coverage"
Assert-Contains -Text $compileText -Pattern "Branch: 4 [typed]" -Description "Branch coverage"
Assert-Contains -Text $compileText -Pattern "BranchList: 3 [typed]" -Description "BranchList coverage"
Assert-Contains -Text $compileText -Pattern "Connector:Splitter: 1 [typed]" -Description "splitter coverage"
Assert-Contains -Text $compileText -Pattern "Connector:Mixer: 1 [typed]" -Description "mixer coverage"
Assert-Contains -Text $compileText -Pattern "ConnectorList: 1 [typed]" -Description "ConnectorList coverage"
Assert-Contains -Text $compileText -Pattern "Pump:ConstantSpeed: 1 [typed]" -Description "pump coverage"
Assert-Contains -Text $compileText -Pattern "Boiler:HotWater: 1 [typed]" -Description "boiler coverage"
Assert-Contains -Text $compileText -Pattern "Chiller:Electric:EIR: 1 [typed]" -Description "chiller coverage"

Write-Host "Planning v0.13 plant-loop skeleton graph."
$planOutput = & $cargo.Source run -p ep_cli --quiet -- model plan $Fixture 2>&1
if ($LASTEXITCODE -ne 0) {
    $planOutput | ForEach-Object { Write-Host $_ }
    throw "Plant-loop skeleton plan smoke failed."
}
$planText = ($planOutput -join "`n")
Assert-Contains -Text $planText -Pattern "ExecutionPlan" -Description "plan header"
Assert-Contains -Text $planText -Pattern "plant_loop_branch_list_edges: 2" -Description "plant loop branch-list graph edges"
Assert-Contains -Text $planText -Pattern "plant_branch_list_member_edges: 4" -Description "plant branch-list member graph edges"
Assert-Contains -Text $planText -Pattern "plant_connector_list_member_edges: 2" -Description "plant connector-list member graph edges"
Assert-Contains -Text $planText -Pattern "plant_branch_component_edges: 4" -Description "plant branch component graph edges"

Write-Host "Plant-loop skeleton smoke passed."
