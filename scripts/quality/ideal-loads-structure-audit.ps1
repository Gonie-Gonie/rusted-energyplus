[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Read-RepoText {
    param([Parameter(Mandatory = $true)][string]$Path)
    return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description missing: $Path"
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -notmatch $Pattern) {
        throw "$Description missing in $Path"
    }
}

function Assert-NotContains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -match $Pattern) {
        throw "$Description unexpectedly present in $Path"
    }
}

function Assert-LineLimit {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][int]$Limit,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $lineCount = (Get-Content -Encoding UTF8 -LiteralPath $Path | Measure-Object -Line).Lines
    if ($lineCount -gt $Limit) {
        throw "$Description exceeds $Limit LOC: $Path has $lineCount LOC"
    }
}

$calcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$noOaCalc = "crates\ep_runtime\src\ideal_loads\calc\no_oa.rs"
$noOaTests = "crates\ep_runtime\src\ideal_loads\calc\no_oa_tests.rs"
$outdoorAir = "crates\ep_runtime\src\ideal_loads\outdoor_air.rs"
$dispatch = "crates\ep_runtime\src\ideal_loads\dispatch.rs"
$runtime = "crates\ep_runtime\src\runtime.rs"
$node = "crates\ep_runtime\src\node\mod.rs"
$zoneEquipment = "crates\ep_runtime\src\zone_equipment\mod.rs"

Assert-FileExists -Path $calcRoot -Description "IdealLoads calc module root"
Assert-FileExists -Path $noOaCalc -Description "IdealLoads no-OA calc module"
Assert-FileExists -Path $noOaTests -Description "IdealLoads no-OA calc tests"
Assert-FileExists -Path $outdoorAir -Description "IdealLoads outdoor-air module"
Assert-FileExists -Path $dispatch -Description "IdealLoads source-order dispatch module"
Assert-FileExists -Path $runtime -Description "Runtime root"
Assert-FileExists -Path $node -Description "Node compatibility facade"
Assert-FileExists -Path $zoneEquipment -Description "Zone equipment compatibility facade"

Assert-LineLimit -Path $calcRoot -Limit 1200 -Description "IdealLoads calc module root"
Assert-LineLimit -Path $noOaCalc -Limit 1200 -Description "IdealLoads no-OA calc module"
Assert-LineLimit -Path $noOaTests -Limit 1200 -Description "IdealLoads no-OA calc tests"
Assert-LineLimit -Path $outdoorAir -Limit 1200 -Description "IdealLoads outdoor-air module"

Assert-Contains -Path $calcRoot -Pattern 'mod no_oa;' -Description "no-OA calc submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use no_oa::\*;' -Description "no-OA calc public re-export"
Assert-Contains -Path $calcRoot -Pattern 'mod no_oa_tests;' -Description "no-OA calc test module declaration"
Assert-NotContains -Path $calcRoot -Pattern 'calc_no_oa_no_limit_sensible_compat\s*\(' -Description "branch formula in calc module root"
Assert-NotContains -Path $calcRoot -Pattern 'fn heating_result_with_limits\s*\(' -Description "heating branch helper in calc module root"
Assert-NotContains -Path $calcRoot -Pattern 'fn cooling_result_with_limits\s*\(' -Description "cooling branch helper in calc module root"

Assert-Contains -Path $noOaCalc -Pattern 'pub fn calc_no_oa_no_limit_sensible_compat\s*\(' -Description "no-OA/no-limit sensible calc"
Assert-Contains -Path $noOaCalc -Pattern 'pub fn calc_no_oa_sensible_with_limits_compat\s*\(' -Description "finite-limit sensible calc"
Assert-Contains -Path $noOaCalc -Pattern 'fn heating_result_with_limits\s*\(' -Description "heating branch helper"
Assert-Contains -Path $noOaCalc -Pattern 'fn cooling_result_with_limits\s*\(' -Description "cooling branch helper"
Assert-Contains -Path $noOaCalc -Pattern 'fn humidistat_dehumidification_mass_flow_rate_kg_per_s\s*\(' -Description "dehumidification diagnostic branch helper"
Assert-Contains -Path $noOaCalc -Pattern 'fn humidistat_humidification_mass_flow_rate_kg_per_s\s*\(' -Description "humidification diagnostic branch helper"
Assert-NotContains -Path $noOaCalc -Pattern '#\[test\]' -Description "unit tests in no-OA implementation module"
Assert-Contains -Path $noOaTests -Pattern '#\[test\]' -Description "unit tests in no-OA test module"

Assert-Contains -Path $dispatch -Pattern 'pub fn sim_purchased_air_compat\s*\(' -Description "SimPurchasedAir source-order wrapper"
Assert-Contains -Path $dispatch -Pattern 'purchased_air_source_order_stages\s*\(' -Description "PurchasedAir source-order stage summary"
Assert-Contains -Path $dispatch -Pattern 'calc_no_oa_sensible_with_limits_and_recirculation_compat\s*\(' -Description "finite-limit branch dispatch"
Assert-Contains -Path $dispatch -Pattern 'calc_no_oa_no_limit_sensible_with_recirculation_context_compat\s*\(' -Description "no-limit branch dispatch"
Assert-NotContains -Path $dispatch -Pattern 'calc_outdoor_air_sensible_report_rates_compat\s*\(' -Description "outdoor-air diagnostic calculation in source-order conformance wrapper"

$runtimeFormulaPatterns = @(
    'fn calc_no_oa_',
    'fn calc_outdoor_air_',
    'fn heating_result_with_limits\s*\(',
    'fn cooling_result_with_limits\s*\(',
    'fn calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s\s*\('
)
foreach ($pattern in $runtimeFormulaPatterns) {
    Assert-NotContains -Path $runtime -Pattern $pattern -Description "IdealLoads branch formula in runtime root"
}

Assert-Contains -Path $node -Pattern 'NodeStateStore' -Description "NodeStateStore facade export"
Assert-Contains -Path $node -Pattern 'pub struct IdealLoadsSupplyNodeUpdate' -Description "IdealLoads node update transfer struct"
Assert-Contains -Path $node -Pattern 'IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE' -Description "IdealLoads node output store metadata"

Assert-Contains -Path $zoneEquipment -Pattern 'pub struct ZoneSysEnergyDemand' -Description "ZoneSysEnergyDemand source-order demand struct"
Assert-Contains -Path $zoneEquipment -Pattern 'pub const fn ideal_loads_zone_equipment_stages\s*\(' -Description "Zone equipment compatibility stages"
Assert-Contains -Path $zoneEquipment -Pattern 'pub fn validate_ideal_loads_zone_equipment_dispatch\s*\(' -Description "IdealLoads zone equipment dispatch validation"

Write-Host "IdealLoads structure audit complete."
