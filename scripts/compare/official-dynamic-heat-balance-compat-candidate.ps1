[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "official_1zone_uncontrolled_dynamic_conformance_candidate_001"
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\official-dynamic-compat-candidate\26.1.0"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$CompareRoot = Join-Path $CaseOutputRoot "compare"

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot)
    if (-not $full.StartsWith($root, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to operate outside repository: $full"
    }
}

function Remove-RepoDirectory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (Test-Path -LiteralPath $Path) {
        Assert-RepoSubPath -Path $Path
        Remove-Item -LiteralPath $Path -Recurse -Force
    }
}

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

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required official dynamic conformance file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running official 1ZoneUncontrolled dynamic heat-balance conformance gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance heat-balance-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Official dynamic heat-balance conformance gate failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Conformance Heat Balance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "conformance claim"
Assert-Contains -Text $text -Pattern "zone_air_algorithm_lane: compatibility-candidate" -Description "compatibility lane"
Assert-Contains -Text $text -Pattern "conformance_promotion_allowed: true" -Description "promotion eligibility"
Assert-Contains -Text $text -Pattern "surface_iteration_count: 20" -Description "surface iteration count"
Assert-Contains -Text $text -Pattern "ctf_initial_history_policy: energyplus-surf-initial" -Description "CTF history policy"
Assert-Contains -Text $text -Pattern "status: pass" -Description "gate status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$digestPath = Join-Path $CompareRoot "compare-digest.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
Assert-FileExists -Path $summaryPath -Description "official dynamic summary"
Assert-FileExists -Path $digestPath -Description "official dynamic digest"
Assert-FileExists -Path $reportPath -Description "official dynamic report"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "Official dynamic candidate must retain conformance_claim=true"
}
if ($summary.gate.blocking -ne $true) {
    throw "Official dynamic candidate gate must be blocking"
}
if ($summary.status -ne "pass") {
    throw "Unexpected official dynamic conformance status: $($summary.status)"
}
if (@($summary.failure_reasons).Count -ne 0) {
    throw "Official dynamic conformance should not report failure reasons"
}
if ($summary.zone_air_algorithm -ne "energyplus-heat-balance-compat-candidate") {
    throw "Unexpected zone_air_algorithm: $($summary.zone_air_algorithm)"
}
if ($summary.zone_air_algorithm_lane -ne "compatibility-candidate") {
    throw "Unexpected algorithm lane: $($summary.zone_air_algorithm_lane)"
}
if ($summary.conformance_promotion_allowed -ne $true) {
    throw "Compatibility candidate must be promotion-eligible"
}
if ($summary.ctf_seed.policy -ne "all-eio") {
    throw "Expected all-EIO CTF seed policy, got $($summary.ctf_seed.policy)"
}
if ($summary.heat_balance_warmup.enabled -ne $true) {
    throw "Official dynamic candidate must run model warmup"
}
if ($summary.heat_balance_warmup.day_count -ne 20) {
    throw "Unexpected Rust warmup day count: $($summary.heat_balance_warmup.day_count)"
}
if ($summary.heat_balance_warmup.oracle_run_period_day_count -ne 20) {
    throw "Unexpected oracle warmup day count: $($summary.heat_balance_warmup.oracle_run_period_day_count)"
}
if ($summary.heat_balance_warmup.day_count_delta -ne 0) {
    throw "Warmup day count delta should be zero, got $($summary.heat_balance_warmup.day_count_delta)"
}
if ($summary.surface_iteration_count -ne 20) {
    throw "Unexpected surface_iteration_count: $($summary.surface_iteration_count)"
}
if ($summary.ctf_initial_history_policy -ne "energyplus-surf-initial") {
    throw "Unexpected CTF history policy: $($summary.ctf_initial_history_policy)"
}

$conformanceOutputs = @($summary.outputs | Where-Object { $_.level -eq "conformance" })
$diagnosticOutputs = @($summary.outputs | Where-Object { $_.level -eq "diagnostic" })
if ($conformanceOutputs.Count -ne 29) {
    throw "Expected 29 conformance-level outputs, got $($conformanceOutputs.Count)"
}
if ($diagnosticOutputs.Count -ne 1) {
    throw "Expected exactly one diagnostic output, got $($diagnosticOutputs.Count)"
}
if (-not ($diagnosticOutputs | Where-Object { $_.key -eq "ZN001:FLR001" -and $_.variable -eq "Surface Heat Storage Rate" })) {
    throw "Surface Heat Storage Rate must remain diagnostic-only"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq "Site Outdoor Air Drybulb Temperature" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })) {
    throw "Weather dry-bulb conformance series missing"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZONE ONE" -and $_.output.variable -eq "Zone Mean Air Temperature" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })) {
    throw "Zone Mean Air Temperature conformance series missing"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Heat Storage Rate" -and $_.output.level -eq "diagnostic" -and $_.status -eq "extracted" })) {
    throw "Floor storage diagnostic series missing"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Heat Balance Conformance Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "comparison_class: conformance" -Description "markdown comparison class"
Assert-Contains -Text $reportText -Pattern "conformance_claim: true" -Description "markdown conformance claim"
Assert-Contains -Text $reportText -Pattern "gate_blocking: true" -Description "markdown blocking gate"
Assert-Contains -Text $reportText -Pattern "Surface Heat Storage Rate / hourly / surface-state / eso / diagnostic" -Description "diagnostic storage output"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "markdown status"

Write-Host "Official dynamic heat-balance conformance gate passed."
