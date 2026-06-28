[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "airloop_fan_only_diagnostic_001"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$Fixture = Join-Path $RepoRoot "data\testcases\minimal\airloop-fan-only.epJSON"

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

foreach ($path in @($CasePath, $Fixture)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing C3 airloop fan-only diagnostic input: $path"
    }
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Validating C3 airloop fan-only diagnostic manifest."
$validateOutput = & $cargo.Source run -p ep_cli --quiet -- conformance validate-case-v2 $CasePath 2>&1
if ($LASTEXITCODE -ne 0) {
    $validateOutput | ForEach-Object { Write-Host $_ }
    throw "C3 airloop fan-only manifest validation failed."
}
$validateText = ($validateOutput -join "`n")
Assert-Contains -Text $validateText -Pattern "comparison_class: diagnostic-only" -Description "manifest diagnostic class"
Assert-Contains -Text $validateText -Pattern "conformance_claim: false" -Description "manifest claim boundary"
Assert-Contains -Text $validateText -Pattern "source_kind: minimal-epjson" -Description "manifest source kind"
Assert-Contains -Text $validateText -Pattern "has_air_loop: true" -Description "manifest airloop scope"
Assert-Contains -Text $validateText -Pattern "Fan Electricity Rate / hourly / hvac-state / eso" -Description "manifest fan output gate"
Assert-Contains -Text $validateText -Pattern "System Node Temperature / hourly / node-state / eso" -Description "manifest node output gate"
Assert-Contains -Text $validateText -Pattern "v2: domain=hvac level=baseline" -Description "fan baseline-only level"
Assert-Contains -Text $validateText -Pattern "v2: domain=node level=baseline" -Description "node baseline-only level"

Write-Host "Compiling C3 airloop fan-only typed model."
$compileOutput = & $cargo.Source run -p ep_cli --quiet -- model compile $Fixture 2>&1
if ($LASTEXITCODE -ne 0) {
    $compileOutput | ForEach-Object { Write-Host $_ }
    throw "C3 airloop fan-only typed compile failed."
}
$compileText = ($compileOutput -join "`n")
Assert-Contains -Text $compileText -Pattern "TypedModel" -Description "compile header"
Assert-Contains -Text $compileText -Pattern "air_loops: 1" -Description "AirLoopHVAC typed count"
Assert-Contains -Text $compileText -Pattern "fans: 1" -Description "fan typed count"
Assert-Contains -Text $compileText -Pattern "coils: 0" -Description "coil skeleton count"
Assert-Contains -Text $compileText -Pattern "setpoint_managers: 1" -Description "setpoint manager typed count"
Assert-Contains -Text $compileText -Pattern "availability_managers: 1" -Description "availability manager typed count"
Assert-Contains -Text $compileText -Pattern "plant_branches: 1" -Description "Branch typed count"
Assert-Contains -Text $compileText -Pattern "plant_branch_lists: 1" -Description "BranchList typed count"
Assert-Contains -Text $compileText -Pattern "AirLoopHVAC: 1 [typed]" -Description "AirLoopHVAC typed coverage"
Assert-Contains -Text $compileText -Pattern "Fan:ConstantVolume: 1 [typed]" -Description "Fan typed coverage"
Assert-Contains -Text $compileText -Pattern "SetpointManager:Scheduled: 1 [typed]" -Description "SetpointManager typed coverage"
Assert-Contains -Text $compileText -Pattern "AvailabilityManager:Scheduled: 1 [typed]" -Description "AvailabilityManager typed coverage"

Write-Host "Planning C3 airloop fan-only graph."
$planOutput = & $cargo.Source run -p ep_cli --quiet -- model plan $Fixture 2>&1
if ($LASTEXITCODE -ne 0) {
    $planOutput | ForEach-Object { Write-Host $_ }
    throw "C3 airloop fan-only graph planning failed."
}
$planText = ($planOutput -join "`n")
Assert-Contains -Text $planText -Pattern "air_loop_branch_list_edges: 1" -Description "airloop BranchList edge"
Assert-Contains -Text $planText -Pattern "air_loop_branch_list_member_edges: 1" -Description "airloop BranchList member edge"
Assert-Contains -Text $planText -Pattern "air_loop_execution_steps: 1" -Description "airloop execution order"
Assert-Contains -Text $planText -Pattern "component_registry_entries: 4" -Description "component registry entries"
Assert-Contains -Text $planText -Pattern "node_graph_component_ownership_edges: 2" -Description "node ownership edges"

$fixtureText = Get-Content -LiteralPath $Fixture -Raw
Assert-Contains -Text $fixtureText -Pattern '"AirLoopHVAC"' -Description "fan-only AirLoopHVAC fixture"
Assert-Contains -Text $fixtureText -Pattern '"Fan:ConstantVolume"' -Description "fan-only Fan fixture"
Assert-Contains -Text $fixtureText -Pattern '"BranchList"' -Description "fan-only BranchList fixture"
Assert-Contains -Text $fixtureText -Pattern '"Fan Electricity Rate"' -Description "fan output gate variable"
Assert-Contains -Text $fixtureText -Pattern '"System Node Temperature"' -Description "node output gate variable"

Write-Host "C3 airloop fan-only diagnostic smoke passed."
