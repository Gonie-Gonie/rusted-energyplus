param(
    [Parameter(Position = 0)][string]$Command = "list",
    [Parameter(Position = 1, ValueFromRemainingArguments = $true)][string[]]$CommandArgs = @()
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = $PSScriptRoot
$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptsRoot "..")).Path

$Commands = [ordered]@{
    "setup" = @{
        Path = "setup\setup.ps1"
        Group = "setup"
        Help = "Prepare Rust, docs tools, oracle/source assets, and report Python."
    }
    "oracle-smoke" = @{
        Path = "setup\oracle-smoke.ps1"
        Group = "setup"
        Help = "Run the EnergyPlus oracle example and IDF to epJSON conversion."
    }
    "source-smoke" = @{
        Path = "setup\source-smoke.ps1"
        Group = "setup"
        Help = "Verify the pinned EnergyPlus reference source checkout."
    }
    "python-smoke" = @{
        Path = "setup\python-smoke.ps1"
        Group = "setup"
        Help = "Verify portable Python and the report-generation venv."
    }
    "check" = @{
        Path = "quality\check.ps1"
        Group = "quality"
        Help = "Run fmt, clippy, tests, smoke gates, docs, and guards."
    }
    "test" = @{
        Path = "quality\test.ps1"
        Group = "quality"
        Help = "Run the Rust workspace tests."
    }
    "docs-check" = @{
        Path = "quality\docs-check.ps1"
        Group = "quality"
        Help = "Build or structurally check the mdBook docs."
    }
    "docs-generate" = @{
        Path = "quality\docs-generate.ps1"
        Group = "quality"
        Help = "Generate mdBook reference pages from specs and case manifests."
    }
    "file-size-check" = @{
        Path = "quality\file-size-check.ps1"
        Group = "quality"
        Help = "Warn or fail on oversized source files, with explicit legacy waivers."
    }
    "perf" = @{
        Path = "quality\perf.ps1"
        Group = "quality"
        Help = "Run local performance checks."
    }
    "strict-no-false-conformance" = @{
        Path = "quality\strict-no-false-conformance.ps1"
        Group = "quality"
        Help = "Guard against unsupported compatibility wording."
    }
    "algorithm-ledger-check" = @{
        Path = "quality\algorithm-ledger-check.ps1"
        Group = "quality"
        Help = "Validate source-map, EnergyPlus source, Rust target, and case evidence links."
    }
    "raw-model-smoke" = @{
        Path = "smoke\raw-model-smoke.ps1"
        Group = "smoke"
        Help = "Smoke test RawModel inspection."
    }
    "typed-model-smoke" = @{
        Path = "smoke\typed-model-smoke.ps1"
        Group = "smoke"
        Help = "Smoke test TypedModel compile preview."
    }
    "model-plan-smoke" = @{
        Path = "smoke\model-plan-smoke.ps1"
        Group = "smoke"
        Help = "Smoke test model graph and execution plan summaries."
    }
    "schedule-compact-smoke" = @{
        Path = "smoke\schedule-compact-smoke.ps1"
        Group = "smoke"
        Help = "Smoke test Schedule:Compact intake."
    }
    "geometry-smoke" = @{
        Path = "smoke\geometry-smoke.ps1"
        Group = "smoke"
        Help = "Smoke test Rust geometry summaries."
    }
    "first-zone-smoke" = @{
        Path = "smoke\first-zone-smoke.ps1"
        Group = "smoke"
        Help = "Run first-zone runtime plumbing diagnostics."
    }
    "runtime-registry-smoke" = @{
        Path = "smoke\runtime-registry-smoke.ps1"
        Group = "smoke"
        Help = "Gate v0.24 runtime output/meter registry and ResultStore diagnostics."
    }
    "heat-balance-generalization-smoke" = @{
        Path = "smoke\heat-balance-generalization-smoke.ps1"
        Group = "smoke"
        Help = "Gate v0.25 opaque no-mass heat-balance boundary generalization."
    }
    "ideal-loads-thermostat-smoke" = @{
        Path = "smoke\ideal-loads-thermostat-smoke.ps1"
        Group = "smoke"
        Help = "Gate the v0.10 thermostat, equipment, and IdealLoads typed graph."
    }
    "air-side-node-diagnostic-smoke" = @{
        Path = "smoke\air-side-node-diagnostic-smoke.ps1"
        Group = "smoke"
        Help = "Gate the v0.11 air-side node baseline evidence and Rust projection."
    }
    "plant-loop-skeleton-smoke" = @{
        Path = "smoke\plant-loop-skeleton-smoke.ps1"
        Group = "smoke"
        Help = "Gate the v0.13 PlantLoop typed graph skeleton."
    }
    "plant-loop-diagnostic-smoke" = @{
        Path = "smoke\plant-loop-diagnostic-smoke.ps1"
        Group = "smoke"
        Help = "Gate the v0.15 PlantLoadProfile baseline-only plant diagnostic."
    }
    "plant-loop-projection-smoke" = @{
        Path = "smoke\plant-loop-projection-smoke.ps1"
        Group = "smoke"
        Help = "Gate the diagnostic Rust plant-state projection addendum."
    }
    "compare-schedule-smoke" = @{
        Path = "compare\compare-schedule-smoke.ps1"
        Group = "compare"
        Help = "Compare schedule output with the EnergyPlus oracle."
    }
    "compare-weather-smoke" = @{
        Path = "compare\compare-weather-smoke.ps1"
        Group = "compare"
        Help = "Compare selected EPW weather fields with the EnergyPlus oracle."
    }
    "compare-schedule-conformance" = @{
        Path = "compare\compare-schedule-conformance.ps1"
        Group = "compare"
        Help = "Run the v0.22 tolerance-gated schedule conformance case."
    }
    "compare-weather-conformance" = @{
        Path = "compare\compare-weather-conformance.ps1"
        Group = "compare"
        Help = "Run the v0.22 tolerance-gated weather dry-bulb conformance case."
    }
    "compare-geometry-smoke" = @{
        Path = "compare\compare-geometry-smoke.ps1"
        Group = "compare"
        Help = "Compare Rust geometry summary with EnergyPlus EIO."
    }
    "compare-surface-geometry-smoke" = @{
        Path = "compare\compare-surface-geometry-smoke.ps1"
        Group = "compare"
        Help = "Compare Rust surface geometry with EnergyPlus EIO."
    }
    "compare-construction-materials-smoke" = @{
        Path = "compare\compare-construction-materials-smoke.ps1"
        Group = "compare"
        Help = "Compare construction/material thermal inputs with EnergyPlus EIO."
    }
    "compare-internal-gains-smoke" = @{
        Path = "compare\compare-internal-gains-smoke.ps1"
        Group = "compare"
        Help = "Compare OtherEquipment nominal gains with EnergyPlus EIO."
    }
    "compare-internal-convective-gain-smoke" = @{
        Path = "compare\compare-internal-convective-gain-smoke.ps1"
        Group = "compare"
        Help = "Compare internal convective gain trace with EnergyPlus ESO."
    }
    "compare-internal-convective-gain-conformance" = @{
        Path = "compare\compare-internal-convective-gain-conformance.ps1"
        Group = "compare"
        Help = "Run the v0.26 tolerance-gated internal convective gain conformance case."
    }
    "compare-zone-smoke" = @{
        Path = "compare\compare-zone-smoke.ps1"
        Group = "compare"
        Help = "Run diagnostic-only zone-temperature extraction comparison."
    }
    "compare-heat-balance-conformance" = @{
        Path = "compare\compare-heat-balance-conformance.ps1"
        Group = "compare"
        Help = "Run the v0.8 tolerance-gated heat-balance conformance case."
    }
    "compare-surface-temperature-conformance" = @{
        Path = "compare\compare-surface-temperature-conformance.ps1"
        Group = "compare"
        Help = "Run the v0.9 tolerance-gated surface-temperature conformance case."
    }
    "official-dynamic-heat-balance-diagnostic" = @{
        Path = "compare\official-dynamic-heat-balance-diagnostic.ps1"
        Group = "compare"
        Help = "Run the official 1ZoneUncontrolled dynamic heat-balance diagnostic case."
    }
    "official-dynamic-heat-balance-all-ctf-probe" = @{
        Path = "compare\official-dynamic-heat-balance-all-ctf-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with all EIO CTF rows enabled as a non-claim probe."
    }
    "official-dynamic-heat-balance-analytical-probe" = @{
        Path = "compare\official-dynamic-heat-balance-analytical-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with the EnergyPlus analytical zone-air probe enabled."
    }
    "official-dynamic-heat-balance-analytical-surface-first-probe" = @{
        Path = "compare\official-dynamic-heat-balance-analytical-surface-first-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with the EnergyPlus analytical surface-first zone-air probe enabled."
    }
    "official-dynamic-heat-balance-all-ctf-analytical-surface-first-probe" = @{
        Path = "compare\official-dynamic-heat-balance-all-ctf-analytical-surface-first-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with all EIO CTF rows and surface-first analytical zone-air correction."
    }
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-probe" = @{
        Path = "compare\official-dynamic-heat-balance-all-ctf-analytical-coupled-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with all EIO CTF rows and a same-timestep analytical surface rebalance."
    }
    "official-dynamic-heat-balance-all-ctf-analytical-surface-first-iter3-probe" = @{
        Path = "compare\official-dynamic-heat-balance-all-ctf-analytical-surface-first-iter3-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with all EIO CTF rows, surface-first analytical correction, and three surface passes."
    }
    "official-dynamic-heat-balance-third-order-probe" = @{
        Path = "compare\official-dynamic-heat-balance-third-order-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with the EnergyPlus third-order zone-air probe enabled."
    }
    "official-dynamic-heat-balance-warmup-20-probe" = @{
        Path = "compare\official-dynamic-heat-balance-warmup-20-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with Rust warmup minimum days raised to the EnergyPlus run-period count."
    }
    "official-dynamic-heat-balance-all-ctf-warmup-20-probe" = @{
        Path = "compare\official-dynamic-heat-balance-all-ctf-warmup-20-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with all EIO CTF rows and the EnergyPlus run-period warmup count."
    }
    "official-dynamic-heat-balance-all-ctf-surface-iter3-probe" = @{
        Path = "compare\official-dynamic-heat-balance-all-ctf-surface-iter3-probe.ps1"
        Group = "compare"
        Help = "Run the official dynamic diagnostic with all EIO CTF rows and three surface-balance passes per timestep."
    }
    "official-dynamic-heat-balance-probe-summary" = @{
        Path = "compare\official-dynamic-heat-balance-probe-summary.ps1"
        Group = "compare"
        Help = "Summarize existing official dynamic heat-balance probe lanes with the report Python."
    }
    "official-dynamic-heat-balance-probe-suite" = @{
        Path = "compare\official-dynamic-heat-balance-probe-suite.ps1"
        Group = "compare"
        Help = "Refresh all official dynamic heat-balance probe lanes and regenerate the probe summary."
    }
    "compare-static-model-conformance" = @{
        Path = "compare\compare-static-model-conformance.ps1"
        Group = "compare"
        Help = "Run the v0.23 official ExampleFile static model conformance case."
    }
    "compare-regression" = @{
        Path = "compare\compare-regression.ps1"
        Group = "compare"
        Help = "Run compare suite and write regression artifacts."
    }
    "compare-series-v2-smoke" = @{
        Path = "compare\compare-series-v2-smoke.ps1"
        Group = "compare"
        Help = "Gate timestamp-aware series reader and comparison metrics v2."
    }
    "conformance-schema-smoke" = @{
        Path = "conformance\conformance-schema-smoke.ps1"
        Group = "conformance"
        Help = "Validate conformance case and suite schema fixtures."
    }
    "manifest-validate-all" = @{
        Path = "conformance\manifest-validate-all.ps1"
        Group = "conformance"
        Help = "Validate all case manifests against the v0.17 schema v2 gate."
    }
    "conformance-baseline-smoke" = @{
        Path = "conformance\conformance-baseline-smoke.ps1"
        Group = "conformance"
        Help = "Generate EnergyPlus baseline artifacts for a fixture case."
    }
    "conformance-report-smoke" = @{
        Path = "conformance\conformance-report-smoke.ps1"
        Group = "conformance"
        Help = "Write baseline-only conformance report skeleton."
    }
    "official-baseline-smoke" = @{
        Path = "conformance\official-baseline-smoke.ps1"
        Group = "conformance"
        Help = "Gate v0.18 output injection and official ExampleFiles oracle baselines."
    }
    "conformance-diagnostic-report-smoke" = @{
        Path = "conformance\conformance-diagnostic-report-smoke.ps1"
        Group = "conformance"
        Help = "Generate diagnostic-only comparison artifacts from a case manifest."
    }
    "package" = @{
        Path = "release\package.ps1"
        Group = "release"
        Help = "Build the local release zip."
    }
    "conformance-evidence-report" = @{
        Path = "release\conformance-evidence-report.ps1"
        Group = "release"
        Help = "Generate oodocs PDF/HTML/JSON numerical conformance evidence."
    }
    "conformance-index-report" = @{
        Path = "release\conformance-index-report.ps1"
        Group = "release"
        Help = "Generate oodocs PDF/HTML/JSON/Markdown conformance index coverage."
    }
    "support-coverage-report" = @{
        Path = "release\support-coverage-report.ps1"
        Group = "release"
        Help = "Generate oodocs PDF/HTML/JSON/Markdown user-facing support coverage."
    }
    "user-coverage-handbook" = @{
        Path = "release\user-coverage-handbook.ps1"
        Group = "release"
        Help = "Generate oodocs PDF/HTML/JSON/Markdown user coverage handbook."
    }
    "release-evidence-manifest" = @{
        Path = "release\release-evidence-manifest.ps1"
        Group = "release"
        Help = "Generate oodocs PDF/HTML/JSON/Markdown release asset manifest."
    }
    "github-release" = @{
        Path = "release\github-release.ps1"
        Group = "release"
        Help = "Publish a release with GitHub CLI."
    }
    "v0.1-verify" = @{
        Path = "release\v0.1-verify.ps1"
        Group = "release"
        Help = "Verify the v0.1 model intake release contract."
    }
    "v0.2-verify" = @{
        Path = "release\v0.2-verify.ps1"
        Group = "release"
        Help = "Verify the v0.2 conformance harness contract."
    }
    "v0.3-verify" = @{
        Path = "release\v0.3-verify.ps1"
        Group = "release"
        Help = "Verify the v0.3 input interpretation contract."
    }
    "v0.4-verify" = @{
        Path = "release\v0.4-verify.ps1"
        Group = "release"
        Help = "Verify the v0.4 time/weather/schedule evidence contract."
    }
    "v0.5-verify" = @{
        Path = "release\v0.5-verify.ps1"
        Group = "release"
        Help = "Verify the v0.5 geometry/internal-variable evidence contract."
    }
    "v0.6-verify" = @{
        Path = "release\v0.6-verify.ps1"
        Group = "release"
        Help = "Verify the v0.6 output/trace/report diagnostic contract."
    }
    "v0.7-verify" = @{
        Path = "release\v0.7-verify.ps1"
        Group = "release"
        Help = "Verify the v0.7 EnergyPlus source mapping gate."
    }
    "v0.8-verify" = @{
        Path = "release\v0.8-verify.ps1"
        Group = "release"
        Help = "Verify the v0.8 heat-balance conformance gate."
    }
    "v0.9-verify" = @{
        Path = "release\v0.9-verify.ps1"
        Group = "release"
        Help = "Verify the v0.9 surface-temperature conformance gate."
    }
    "v0.10-verify" = @{
        Path = "release\v0.10-verify.ps1"
        Group = "release"
        Help = "Verify the v0.10 IdealLoads thermostat typed-graph gate."
    }
    "v0.11-verify" = @{
        Path = "release\v0.11-verify.ps1"
        Group = "release"
        Help = "Verify the v0.11 air-side node diagnostic gate."
    }
    "v0.12-verify" = @{
        Path = "release\v0.12-verify.ps1"
        Group = "release"
        Help = "Verify the v0.12 node source mapping gate."
    }
    "v0.13-verify" = @{
        Path = "release\v0.13-verify.ps1"
        Group = "release"
        Help = "Verify the v0.13 plant-loop skeleton gate."
    }
    "v0.14-verify" = @{
        Path = "release\v0.14-verify.ps1"
        Group = "release"
        Help = "Verify the v0.14 plant source mapping gate."
    }
    "v0.15-verify" = @{
        Path = "release\v0.15-verify.ps1"
        Group = "release"
        Help = "Verify the v0.15 plant-loop diagnostic gate."
    }
    "v0.16-verify" = @{
        Path = "release\v0.16-verify.ps1"
        Group = "release"
        Help = "Verify v0.16 versioning/evidence cleanup and diagnostic addendum gates."
    }
    "v0.17-verify" = @{
        Path = "release\v0.17-verify.ps1"
        Group = "release"
        Help = "Verify the v0.17 manifest and output request schema v2 gate."
    }
    "v0.18-verify" = @{
        Path = "release\v0.18-verify.ps1"
        Group = "release"
        Help = "Verify the v0.18 output injection and official baseline gate."
    }
    "v0.19-verify" = @{
        Path = "release\v0.19-verify.ps1"
        Group = "release"
        Help = "Verify the v0.19 series reader and compare engine v2 gate."
    }
    "v0.20-verify" = @{
        Path = "release\v0.20-verify.ps1"
        Group = "release"
        Help = "Verify the v0.20 conformance report generator gate."
    }
    "v0.21-verify" = @{
        Path = "release\v0.21-verify.ps1"
        Group = "release"
        Help = "Verify the v0.21 source map and algorithm ledger gate."
    }
    "v0.22-verify" = @{
        Path = "release\v0.22-verify.ps1"
        Group = "release"
        Help = "Verify the v0.22 time/weather/schedule conformance gate."
    }
    "v0.23-verify" = @{
        Path = "release\v0.23-verify.ps1"
        Group = "release"
        Help = "Verify the v0.23 static model evidence gate."
    }
    "v0.24-verify" = @{
        Path = "release\v0.24-verify.ps1"
        Group = "release"
        Help = "Verify the v0.24 runtime state and output registry hardening gate."
    }
    "v0.25-verify" = @{
        Path = "release\v0.25-verify.ps1"
        Group = "release"
        Help = "Verify the v0.25 opaque no-mass heat-balance generalization gate."
    }
    "v0.26-verify" = @{
        Path = "release\v0.26-verify.ps1"
        Group = "release"
        Help = "Verify the v0.26 internal convective gains conformance gate."
    }
    "v0.27-verify" = @{
        Path = "release\v0.27-verify.ps1"
        Group = "release"
        Help = "Verify the v0.27 user support coverage report gate."
    }
    "v0.28-verify" = @{
        Path = "release\v0.28-verify.ps1"
        Group = "release"
        Help = "Verify the v0.28 input object coverage metadata gate."
    }
    "v0.29-verify" = @{
        Path = "release\v0.29-verify.ps1"
        Group = "release"
        Help = "Verify the v0.29 output variable coverage metadata gate."
    }
    "v0.30-verify" = @{
        Path = "release\v0.30-verify.ps1"
        Group = "release"
        Help = "Verify the v0.30 algorithm coverage metadata gate."
    }
    "v0.31-verify" = @{
        Path = "release\v0.31-verify.ps1"
        Group = "release"
        Help = "Verify the v0.31 release evidence asset manifest gate."
    }
    "v0.32-verify" = @{
        Path = "release\v0.32-verify.ps1"
        Group = "release"
        Help = "Verify the v0.32 user coverage handbook gate."
    }
}

$Aliases = @{
    "docs" = "docs-check"
    "guard" = "strict-no-false-conformance"
    "verify-v0.1" = "v0.1-verify"
    "verify-v0.2" = "v0.2-verify"
    "verify-v0.3" = "v0.3-verify"
    "verify-v0.4" = "v0.4-verify"
    "verify-v0.5" = "v0.5-verify"
    "verify-v0.6" = "v0.6-verify"
    "verify-v0.7" = "v0.7-verify"
    "verify-v0.8" = "v0.8-verify"
    "verify-v0.9" = "v0.9-verify"
    "verify-v0.10" = "v0.10-verify"
    "verify-v0.11" = "v0.11-verify"
    "verify-v0.12" = "v0.12-verify"
    "verify-v0.13" = "v0.13-verify"
    "verify-v0.14" = "v0.14-verify"
    "verify-v0.15" = "v0.15-verify"
    "verify-v0.16" = "v0.16-verify"
    "verify-v0.17" = "v0.17-verify"
    "verify-v0.18" = "v0.18-verify"
    "verify-v0.19" = "v0.19-verify"
    "verify-v0.20" = "v0.20-verify"
    "verify-v0.21" = "v0.21-verify"
    "verify-v0.22" = "v0.22-verify"
    "verify-v0.23" = "v0.23-verify"
    "verify-v0.24" = "v0.24-verify"
    "verify-v0.25" = "v0.25-verify"
    "verify-v0.26" = "v0.26-verify"
    "verify-v0.27" = "v0.27-verify"
    "verify-v0.28" = "v0.28-verify"
    "verify-v0.29" = "v0.29-verify"
    "verify-v0.30" = "v0.30-verify"
    "verify-v0.31" = "v0.31-verify"
    "verify-v0.32" = "v0.32-verify"
}

function Show-Commands {
    Write-Host "Usage: .\scripts\dev.cmd <command> [args...]"
    Write-Host ""
    foreach ($group in @("setup", "quality", "smoke", "compare", "conformance", "release")) {
        Write-Host "[$group]"
        foreach ($name in $Commands.Keys) {
            $entry = $Commands[$name]
            if ($entry.Group -eq $group) {
                Write-Host ("  {0,-42} {1}" -f $name, $entry.Help)
            }
        }
        Write-Host ""
    }
}

function Convert-CommandArguments {
    param([string[]]$Values)

    $named = @{}
    $positional = @()
    for ($index = 0; $index -lt $Values.Count; $index += 1) {
        $value = $Values[$index]
        if ($value.StartsWith("-", [System.StringComparison]::Ordinal) -and $value.Length -gt 1) {
            $name = $value.TrimStart("-")
            $nextIndex = $index + 1
            if ($nextIndex -lt $Values.Count -and -not $Values[$nextIndex].StartsWith("-", [System.StringComparison]::Ordinal)) {
                $named[$name] = $Values[$nextIndex]
                $index += 1
            }
            else {
                $named[$name] = $true
            }
        }
        else {
            $positional += $value
        }
    }

    return [pscustomobject]@{
        Named = $named
        Positional = $positional
    }
}

if ($Command -in @("list", "help", "--help", "-h")) {
    Show-Commands
    return
}

$normalized = $Command
if ($normalized.EndsWith(".cmd", [System.StringComparison]::OrdinalIgnoreCase) -or
    $normalized.EndsWith(".ps1", [System.StringComparison]::OrdinalIgnoreCase)) {
    $normalized = [System.IO.Path]::GetFileNameWithoutExtension($normalized)
}

if ($Aliases.ContainsKey($normalized)) {
    $normalized = $Aliases[$normalized]
}

if (-not $Commands.Contains($normalized)) {
    Write-Error "Unknown script command: $Command"
    Show-Commands
    throw "Unknown script command: $Command"
}

$script = Join-Path $ScriptsRoot $Commands[$normalized].Path
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Command target is missing: $script"
}

Set-Location $RepoRoot
$bound = Convert-CommandArguments -Values $CommandArgs
$positionalArguments = $bound.Positional
$namedArguments = $bound.Named
& $script @positionalArguments @namedArguments
if (-not $?) {
    throw "Script command failed: $normalized"
}
