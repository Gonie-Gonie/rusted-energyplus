[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Assert-DoesNotContain {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for false-conformance guard: $Path"
    }

    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -ne $match) {
        $match | ForEach-Object { Write-Host "$($_.Path):$($_.LineNumber): $($_.Line)" }
        throw "Forbidden false-conformance wording found for $Description`: $Pattern"
    }
    Write-Host "OK no false-conformance wording for $Description"
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for false-conformance guard: $Path"
    }

    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -eq $match) {
        throw "Missing required compatibility boundary for $Description`: $Pattern"
    }
    Write-Host "OK compatibility boundary for $Description`: $Pattern"
}

Assert-DoesNotContain -Path "README.md" -Pattern "first runtime path for an uncontrolled one-zone building subset" -Description "README scope"
Assert-DoesNotContain -Path "README.md" -Pattern "ResultStore output from the first uncontrolled one-zone simulation subset" -Description "README scope"
Assert-DoesNotContain -Path "README.md" -Pattern "zone temperature comparison passes" -Description "README scope"
Assert-DoesNotContain -Path "README.md" -Pattern "EnergyPlus simulation works" -Description "README scope"

Assert-DoesNotContain -Path "crates\ep_cli\src\main.rs" -Pattern "Zone Temperature Smoke Comparison" -Description "zone diagnostic CLI"
Assert-DoesNotContain -Path "crates\ep_cli\src\main.rs" -Pattern "exact_match: future" -Description "zone diagnostic CLI"
Assert-DoesNotContain -Path "scripts\compare\compare-zone-smoke.ps1" -Pattern "status: pass" -Description "zone diagnostic smoke"
Assert-DoesNotContain -Path "scripts\compare\compare-zone-smoke.ps1" -Pattern "exact_match: future" -Description "zone diagnostic smoke"
Assert-DoesNotContain -Path "scripts\compare\compare-regression.ps1" -Pattern "conformance-smoke" -Description "compare regression class names"
Assert-DoesNotContain -Path "docs\src\archive\old-readiness-notes\v0.6.0-diagnostic-runtime-note.md" -Pattern "first executable building simulation subset" -Description "v0.6 scope"
Assert-Contains -Path "docs\src\archive\old-readiness-notes\v0.6.0-diagnostic-runtime-note.md" -Pattern "Historical diagnostic note" -Description "v0.6 archive boundary"
Assert-Contains -Path "docs\src\archive\old-readiness-notes\v0.7.0-compare-diagnostic-note.md" -Pattern "Historical diagnostic note" -Description "v0.7 archive boundary"
Assert-Contains -Path "docs\src\operations\v0.6.0-readiness.md" -Pattern "diagnostic-ready" -Description "v0.6 active readiness boundary"
Assert-Contains -Path "docs\src\operations\v0.6.0-readiness.md" -Pattern "tolerance_policy: none" -Description "v0.6 tolerance boundary"
Assert-Contains -Path "docs\src\operations\v0.7.0-readiness.md" -Pattern "planning-ready" -Description "v0.7 active readiness boundary"
Assert-Contains -Path "docs\src\porting-map\algorithm-porting-readiness.md" -Pattern "zone-temperature pass wording" -Description "v0.7 algorithm boundary"
Assert-Contains -Path "docs\src\operations\v0.8.0-readiness.md" -Pattern "conformance-ready" -Description "v0.8 active readiness boundary"
Assert-Contains -Path "docs\src\operations\v0.8.0-readiness.md" -Pattern "not a dynamic exterior heat-balance claim" -Description "v0.8 claim boundary"
Assert-Contains -Path "docs\src\operations\v0.9.0-readiness.md" -Pattern "conformance-ready" -Description "v0.9 active readiness boundary"
Assert-Contains -Path "docs\src\operations\v0.9.0-readiness.md" -Pattern "not a fenestration or solar-radiation claim" -Description "v0.9 claim boundary"
Assert-Contains -Path "docs\src\operations\v0.10.0-readiness.md" -Pattern "typed-graph-ready" -Description "v0.10 active readiness boundary"
Assert-Contains -Path "docs\src\operations\v0.10.0-readiness.md" -Pattern "not an IdealLoads load-conformance claim" -Description "v0.10 claim boundary"
Assert-Contains -Path "docs\src\operations\v0.10.0-readiness.md" -Pattern "v0.11 may begin only after v0.10 hardening has landed" -Description "v0.11 hardening boundary"
Assert-Contains -Path "docs\src\operations\v0.11.0-readiness.md" -Pattern "diagnostic-ready" -Description "v0.11 active readiness boundary"
Assert-Contains -Path "docs\src\operations\v0.11.0-readiness.md" -Pattern "not a node or HVAC numerical conformance claim" -Description "v0.11 claim boundary"
Assert-Contains -Path "docs\src\operations\v0.12.0-readiness.md" -Pattern "planning-ready" -Description "v0.12 active readiness boundary"
Assert-Contains -Path "docs\src\operations\v0.12.0-readiness.md" -Pattern "not a node or HVAC numerical conformance claim" -Description "v0.12 claim boundary"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "planning guard" -Description "v0.12 source-map boundary"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "remain baseline-only or diagnostic-only evidence" -Description "v0.12 node non-claim boundary"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "v0.8 case class"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "conformance_claim = true" -Description "v0.8 manifest claim"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "max_abs = 0.000001" -Description "v0.8 tolerance"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "blocking = true" -Description "v0.8 blocking gate"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "v0.9 case class"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern "conformance_claim = true" -Description "v0.9 manifest claim"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern 'variable_class = "surface-state"' -Description "v0.9 surface tolerance"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern "blocking = true" -Description "v0.9 blocking gate"
Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern 'comparison_class = "smoke"' -Description "v0.10 case class"
Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern "conformance_claim = false" -Description "v0.10 manifest claim boundary"
Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern 'class = "hvac-state"' -Description "v0.10 HVAC output class"
Assert-Contains -Path "scripts\smoke\ideal-loads-thermostat-smoke.ps1" -Pattern "status: baseline-only" -Description "v0.10 smoke gate baseline-only status"
Assert-Contains -Path "scripts\smoke\ideal-loads-thermostat-smoke.ps1" -Pattern "baseline_nonzero_count" -Description "v0.10 nonzero baseline gate"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern 'comparison_class = "diagnostic-only"' -Description "v0.11 case class"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern "conformance_claim = false" -Description "v0.11 manifest claim boundary"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern 'class = "node-state"' -Description "v0.11 node output class"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "status: baseline-only" -Description "v0.11 smoke gate baseline-only status"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "baseline_nonzero_count" -Description "v0.11 nonzero node baseline gate"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "algorithm_parity: false" -Description "v0.11 node projection algorithm boundary"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "state_nodes: 3" -Description "v0.11 node projection state count"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "status: projected" -Description "v0.11 node projection status"

Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "comparison_class: diagnostic-only" -Description "zone diagnostic CLI"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "conformance_claim: false" -Description "diagnostic CLI"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "Conformance Heat Balance Report" -Description "v0.8 conformance CLI"
Assert-Contains -Path "scripts\compare\compare-zone-smoke.ps1" -Pattern "status: extracted" -Description "zone diagnostic smoke"
Assert-Contains -Path "scripts\compare\compare-heat-balance-conformance.ps1" -Pattern "status: pass" -Description "v0.8 conformance gate"
Assert-Contains -Path "scripts\compare\compare-surface-temperature-conformance.ps1" -Pattern "status: pass" -Description "v0.9 conformance gate"
Assert-Contains -Path "docs\src\operations\full-compatibility-reset.md" -Pattern "No conformance claim without case + variable list + tolerance + report + gate." -Description "reset policy"

Write-Host "False-conformance guard passed."
