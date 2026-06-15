[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

& (Join-Path $PSScriptRoot "official-dynamic-heat-balance-diagnostic.ps1") `
    -CtfSeedPolicy all-eio `
    -CtfInitialHistoryPolicy energyplus-surf-initial `
    -ZoneAirAlgorithm energyplus-heat-balance-compat-candidate `
    -WarmupMinimumDays 20 `
    -SurfaceIterations 20 `
    -CaseId official_1zone_uncontrolled_dynamic_conformance_candidate_001 `
    -OutputRootRelativeOverride ".runtime\official-dynamic-compat-candidate\26.1.0"
