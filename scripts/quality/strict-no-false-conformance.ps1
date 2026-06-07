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

Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "comparison_class: diagnostic-only" -Description "zone diagnostic CLI"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "conformance_claim: false" -Description "diagnostic CLI"
Assert-Contains -Path "scripts\compare\compare-zone-smoke.ps1" -Pattern "status: extracted" -Description "zone diagnostic smoke"
Assert-Contains -Path "docs\src\operations\full-compatibility-reset.md" -Pattern "No conformance claim without case + variable list + tolerance + report + gate." -Description "reset policy"

Write-Host "False-conformance guard passed."
