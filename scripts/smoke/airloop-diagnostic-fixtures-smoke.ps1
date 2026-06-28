[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot

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

function Invoke-CheckedCargo {
    param(
        [Parameter(Mandatory = $true)][string[]]$Arguments,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -eq $cargo) {
        throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
    }

    $output = & $cargo.Source @Arguments 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "$Description failed."
    }
    return ($output -join "`n")
}

$cases = @(
    @{
        Id = "airloop_coil_only_diagnostic_001"
        Fixture = "data\testcases\minimal\airloop-coil-only.epJSON"
        CompilePatterns = @("air_loops: 1", "fans: 0", "coils: 1", "setpoint_managers: 1", "availability_managers: 1")
        PlanPatterns = @("air_loop_execution_steps: 1", "component_registry_entries: 4", "node_graph_component_ownership_edges: 2")
        FixturePatterns = @('"AirLoopHVAC"', '"Coil:Heating:Electric"', '"Heating Coil Heating Rate"', '"System Node Temperature"')
    },
    @{
        Id = "ptac_diagnostic_001"
        Fixture = "data\testcases\minimal\ptac-diagnostic.epJSON"
        CompilePatterns = @("fans: 1", "coils: 1", "ZoneHVAC:PackagedTerminalAirConditioner: 1 [raw-only]")
        PlanPatterns = @("component_registry_entries: 2", "node_graph_component_ownership_edges: 2")
        FixturePatterns = @('"ZoneHVAC:PackagedTerminalAirConditioner"', '"Fan:OnOff"', '"Coil:Heating:Electric"', '"Zone Packaged Terminal Air Conditioner Total Heating Rate"')
    },
    @{
        Id = "airloop_5zone_aircooled_diagnostic_001"
        Fixture = "data\testcases\minimal\5zone-aircooled-diagnostic.epJSON"
        CompilePatterns = @("zones: 5", "air_loops: 1", "fans: 1", "coils: 1", "setpoint_managers: 1", "availability_managers: 1")
        PlanPatterns = @("air_loop_execution_steps: 2", "component_registry_entries: 6", "node_graph_component_ownership_edges: 4")
        FixturePatterns = @('"5Zone AirCooled Diagnostic Air Loop"', '"Fan:VariableVolume"', '"Coil:Cooling:DX:SingleSpeed"', '"Cooling Coil Total Cooling Rate"')
    }
)

foreach ($case in $cases) {
    $casePath = Join-Path $RepoRoot "data\conformance_cases\$($case.Id)\case.toml"
    $fixturePath = Join-Path $RepoRoot $case.Fixture

    foreach ($path in @($casePath, $fixturePath)) {
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Missing C3 airloop diagnostic input: $path"
        }
    }

    Write-Host "Validating C3 airloop diagnostic manifest $($case.Id)."
    $validateText = Invoke-CheckedCargo -Arguments @("run", "-p", "ep_cli", "--quiet", "--", "conformance", "validate-case-v2", $casePath) -Description "C3 airloop diagnostic manifest validation"
    Assert-Contains -Text $validateText -Pattern "comparison_class: diagnostic-only" -Description "$($case.Id) diagnostic class"
    Assert-Contains -Text $validateText -Pattern "conformance_claim: false" -Description "$($case.Id) claim boundary"
    Assert-Contains -Text $validateText -Pattern "source_kind: minimal-epjson" -Description "$($case.Id) source kind"
    Assert-Contains -Text $validateText -Pattern "v2: domain=hvac level=baseline" -Description "$($case.Id) hvac baseline-only level"

    Write-Host "Compiling C3 airloop diagnostic fixture $($case.Id)."
    $compileText = Invoke-CheckedCargo -Arguments @("run", "-p", "ep_cli", "--quiet", "--", "model", "compile", $fixturePath) -Description "C3 airloop diagnostic typed compile"
    Assert-Contains -Text $compileText -Pattern "TypedModel" -Description "$($case.Id) compile header"
    foreach ($pattern in $case.CompilePatterns) {
        Assert-Contains -Text $compileText -Pattern $pattern -Description "$($case.Id) compile pattern"
    }

    Write-Host "Planning C3 airloop diagnostic fixture $($case.Id)."
    $planText = Invoke-CheckedCargo -Arguments @("run", "-p", "ep_cli", "--quiet", "--", "model", "plan", $fixturePath) -Description "C3 airloop diagnostic graph planning"
    foreach ($pattern in $case.PlanPatterns) {
        Assert-Contains -Text $planText -Pattern $pattern -Description "$($case.Id) plan pattern"
    }

    $fixtureText = Get-Content -LiteralPath $fixturePath -Raw
    foreach ($pattern in $case.FixturePatterns) {
        Assert-Contains -Text $fixtureText -Pattern $pattern -Description "$($case.Id) fixture pattern"
    }
}

Write-Host "C3 airloop diagnostic fixtures smoke passed."
