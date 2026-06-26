[CmdletBinding()]
param(
    [string]$Version = "26.1.0"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\$Version"
$WeatherPath = Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
$OutputRoot = Join-Path $RepoRoot ".runtime\one-zone-family-runs\$Version"

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

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
}

function Write-Utf8File {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Content
    )

    $parent = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Force -Path $parent | Out-Null
    }
    $encoding = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

Assert-FileExists -Path (Join-Path $OracleRoot "energyplus.exe") -Description "EnergyPlus oracle executable"
Assert-FileExists -Path (Join-Path $OracleRoot "ConvertInputFormat.exe") -Description "EnergyPlus input converter"
Assert-FileExists -Path $WeatherPath -Description "EnergyPlus oracle weather"

$familyMembers = @(
    [pscustomobject]@{ Id = "official_1zone_uncontrolled_dynamic_conformance_candidate_001"; SourceIdf = ".runtime\energyplus\26.1.0\ExampleFiles\1ZoneUncontrolled.idf"; ExpectedStatus = "pass" },
    [pscustomobject]@{ Id = "official_1zone_uncontrolled_dynamic_diagnostic_001"; SourceIdf = ".runtime\energyplus\26.1.0\ExampleFiles\1ZoneUncontrolled.idf"; ExpectedStatus = "diagnostic-only" },
    [pscustomobject]@{ Id = "official_1zone_uncontrolled_3surface_family_001"; SourceIdf = ".runtime\energyplus\26.1.0\ExampleFiles\1ZoneUncontrolled3SurfaceZone.idf"; ExpectedStatus = "planned-not-claimed" },
    [pscustomobject]@{ Id = "heat_balance_nomass_001"; SourceIdf = "data\conformance_cases\heat_balance_nomass_001\heat_balance_nomass.idf"; ExpectedStatus = "pass" },
    [pscustomobject]@{ Id = "official_1zone_uncontrolled_massive_opaque_family_001"; SourceIdf = "data\conformance_families\official_1zone_uncontrolled\fixtures\massive_opaque.idf"; ExpectedStatus = "planned-not-claimed" },
    [pscustomobject]@{ Id = "official_1zone_uncontrolled_varied_internal_gain_family_001"; SourceIdf = "data\conformance_families\official_1zone_uncontrolled\fixtures\varied_internal_gain.idf"; ExpectedStatus = "planned-not-claimed" },
    [pscustomobject]@{ Id = "official_1zone_uncontrolled_varied_material_resistance_family_001"; SourceIdf = "data\conformance_families\official_1zone_uncontrolled\fixtures\varied_material_resistance.idf"; ExpectedStatus = "planned-not-claimed" },
    [pscustomobject]@{ Id = "official_1zone_uncontrolled_varied_timestep_family_001"; SourceIdf = "data\conformance_families\official_1zone_uncontrolled\fixtures\varied_timestep.idf"; ExpectedStatus = "planned-not-claimed" }
)

foreach ($member in $familyMembers) {
    Assert-FileExists -Path (Join-Path $RepoRoot $member.SourceIdf) -Description "family source IDF for $($member.Id)"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Building eplus-rs CLI for 1Zone family runs."
& $cargo.Source build -p ep_cli --quiet
if ($LASTEXITCODE -ne 0) {
    throw "Failed to build ep_cli."
}

$exe = Join-Path $RepoRoot "target\debug\eplus-rs.exe"
Assert-FileExists -Path $exe -Description "built CLI binary"

Remove-RepoDirectory -Path $OutputRoot
New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null

$results = @()
foreach ($member in $familyMembers) {
    $caseOutputRoot = Join-Path $OutputRoot $member.Id
    $sourceIdf = Join-Path $RepoRoot $member.SourceIdf
    Write-Host "Running 1Zone family member: $($member.Id)"
    $output = & $exe run $sourceIdf -w $WeatherPath -d $caseOutputRoot --overwrite --compare-oracle --oracle-root $OracleRoot 2>&1
    $processExitCode = $LASTEXITCODE

    $runSummaryPath = Join-Path $caseOutputRoot "run-summary.json"
    $compareSummaryPath = Join-Path $caseOutputRoot "compare\compare-summary.json"
    $compareReportPath = Join-Path $caseOutputRoot "compare\compare-report.md"
    if (-not (Test-Path -LiteralPath $runSummaryPath -PathType Leaf) -or -not (Test-Path -LiteralPath $compareSummaryPath -PathType Leaf)) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Family member did not produce run-summary.json and compare-summary.json: $($member.Id)"
    }
    Assert-FileExists -Path $compareReportPath -Description "compare report for $($member.Id)"

    $runSummary = Get-Content -Encoding UTF8 -Raw -LiteralPath $runSummaryPath | ConvertFrom-Json
    $compareSummary = Get-Content -Encoding UTF8 -Raw -LiteralPath $compareSummaryPath | ConvertFrom-Json
    if ($runSummary.oracle_status -ne "generated") {
        throw "Family member did not generate oracle baseline: $($member.Id)"
    }
    if ($runSummary.support.run_result_state -ne "supported_compatibility_run") {
        throw "Family member did not execute in supported compatibility runtime: $($member.Id) -> $($runSummary.support.run_result_state)"
    }
    if ($compareSummary.conformance_claim -ne $false) {
        throw "Arbitrary family run must not set conformance_claim=true: $($member.Id)"
    }

    $results += [pscustomobject]@{
        case_id = $member.Id
        expected_status = $member.ExpectedStatus
        process_exit_code = $processExitCode
        run_summary_exit_code = $runSummary.exit_code
        run_status = $runSummary.status
        support_status = $runSummary.support.status
        run_result_state = $runSummary.support.run_result_state
        oracle_status = $runSummary.oracle_status
        compare_status = $compareSummary.status
        compare_summary_json = $compareSummaryPath.Replace($RepoRoot + "\", "")
        compare_report_md = $compareReportPath.Replace($RepoRoot + "\", "")
    }
}

$missingCompareSummaries = @($results | Where-Object { -not (Test-Path -LiteralPath (Join-Path $RepoRoot $_.compare_summary_json) -PathType Leaf) })
$summary = [pscustomobject]@{
    schema = "rusted-energyplus.one-zone-family-run-summary.v1"
    oracle_version = $Version
    generated_at_utc = [DateTimeOffset]::UtcNow.ToString("o")
    member_count = $familyMembers.Count
    members_run = $results.Count
    all_members_run = ($results.Count -eq $familyMembers.Count)
    all_members_have_compare_summary = ($missingCompareSummaries.Count -eq 0)
    members = $results
}

$markdown = @(
    "# Official 1ZoneUncontrolled Family Run Summary",
    "",
    "- oracle_version: $Version",
    "- members_run: $($summary.members_run)",
    "- all_members_run: $($summary.all_members_run)",
    "- all_members_have_compare_summary: $($summary.all_members_have_compare_summary)",
    "",
    "| Case | Expected status | Run status | Compare status | Compare summary |",
    "|---|---|---|---|---|"
)
foreach ($result in $results) {
    $markdown += "| $($result.case_id) | $($result.expected_status) | $($result.run_status) | $($result.compare_status) | $($result.compare_summary_json) |"
}

Write-Utf8File -Path (Join-Path $OutputRoot "family-run-summary.json") -Content ($summary | ConvertTo-Json -Depth 8)
Write-Utf8File -Path (Join-Path $OutputRoot "family-run-summary.md") -Content ($markdown -join [Environment]::NewLine)

if (-not $summary.all_members_run -or -not $summary.all_members_have_compare_summary) {
    throw "1Zone family run evidence is incomplete."
}

Write-Host "1Zone family runs completed: $OutputRoot"
