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
    "compare-regression" = @{
        Path = "compare\compare-regression.ps1"
        Group = "compare"
        Help = "Run compare suite and write regression artifacts."
    }
    "conformance-schema-smoke" = @{
        Path = "conformance\conformance-schema-smoke.ps1"
        Group = "conformance"
        Help = "Validate conformance case and suite schema fixtures."
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
