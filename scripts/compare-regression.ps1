[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "common.ps1")
Add-CargoBinToPath

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-regression\26.1.0"

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

function New-Directory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        New-Item -ItemType Directory -Force -Path $Path | Out-Null
    }
}

function New-TraceEvent {
    param(
        [Parameter(Mandatory = $true)][string]$CaseId,
        [Parameter(Mandatory = $true)][string]$Phase,
        [Parameter(Mandatory = $true)][string]$Status,
        [int]$DurationMs = 0
    )

    [pscustomobject]@{
        timestamp_utc = (Get-Date).ToUniversalTime().ToString("o")
        case_id = $CaseId
        phase = $Phase
        status = $Status
        duration_ms = $DurationMs
    }
}

function Write-JsonFile {
    param(
        [Parameter(Mandatory = $true)]$Value,
        [Parameter(Mandatory = $true)][string]$Path
    )
    $Value | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath $Path -Encoding ASCII
}

function Get-OutputTail {
    param([Parameter(Mandatory = $true)][object[]]$Output)
    ($Output | Select-Object -Last 20) -join "`n"
}

function Write-CompareArtifacts {
    param(
        [Parameter(Mandatory = $true)][object[]]$Cases,
        [Parameter(Mandatory = $true)][object[]]$TraceEvents,
        [AllowNull()][object]$FirstDivergence,
        [Parameter(Mandatory = $true)][int]$TotalDurationMs
    )

    $status = if ($null -eq $FirstDivergence) { "pass" } else { "fail" }
    $summary = [pscustomobject]@{
        schema_version = 1
        suite = "compare-regression"
        oracle_version = "26.1.0"
        status = $status
        generated_utc = (Get-Date).ToUniversalTime().ToString("o")
        total_duration_ms = $TotalDurationMs
        cases = $Cases
        first_divergence = $FirstDivergence
        artifacts = [pscustomobject]@{
            trace_json = "trace.json"
            compare_report_md = "compare-report.md"
            compare_summary_json = "compare-summary.json"
            profile_summary_json = "profile-summary.json"
        }
    }

    $trace = [pscustomobject]@{
        schema_version = 1
        suite = "compare-regression"
        events = $TraceEvents
    }

    $profileCases = @(
        $Cases | ForEach-Object {
            [pscustomobject]@{
                id = $_.id
                duration_ms = $_.duration_ms
            }
        }
    )

    $profile = [pscustomobject]@{
        schema_version = 1
        suite = "compare-regression"
        profiled = $false
        note = "Profile counters are a v0.7 skeleton; case wall-clock durations are recorded."
        total_duration_ms = $TotalDurationMs
        cases = $profileCases
    }

    $report = @(
        "# Compare Regression Report",
        "",
        "- status: $status",
        "- oracle_version: 26.1.0",
        "- generated_utc: $($summary.generated_utc)",
        "- total_duration_ms: $TotalDurationMs",
        "",
        "## Cases",
        "",
        "| Case | Kind | Class | Status | Duration ms |",
        "|---|---|---|---:|---:|"
    )
    foreach ($case in $Cases) {
        $report += "| $($case.id) | $($case.kind) | $($case.comparison_class) | $($case.status) | $($case.duration_ms) |"
    }
    $report += ""
    $report += "## First Divergence"
    $report += ""
    if ($null -eq $FirstDivergence) {
        $report += "None."
    }
    else {
        $report += "- case: $($FirstDivergence.case_id)"
        $report += "- reason: $($FirstDivergence.reason)"
        $report += ""
        $report += '```text'
        $report += $FirstDivergence.output_tail
        $report += '```'
    }
    $report += ""
    $report += "## Artifacts"
    $report += ""
    $report += "- trace.json"
    $report += "- compare-summary.json"
    $report += "- profile-summary.json"

    Write-JsonFile -Value $summary -Path (Join-Path $OutputRoot "compare-summary.json")
    Write-JsonFile -Value $trace -Path (Join-Path $OutputRoot "trace.json")
    Write-JsonFile -Value $profile -Path (Join-Path $OutputRoot "profile-summary.json")
    $report | Set-Content -LiteralPath (Join-Path $OutputRoot "compare-report.md") -Encoding ASCII
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\setup.cmd -InstallRust first."
}

$suiteCases = @(
    [pscustomobject]@{
        id = "schedule-value"
        kind = "schedule"
        comparison_class = "conformance-smoke"
        script = "compare-schedule-smoke.cmd"
    },
    [pscustomobject]@{
        id = "weather-drybulb"
        kind = "weather"
        comparison_class = "conformance-smoke"
        script = "compare-weather-smoke.cmd"
    },
    [pscustomobject]@{
        id = "zone-temperature"
        kind = "zone"
        comparison_class = "diagnostic-only"
        script = "compare-zone-smoke.cmd"
    }
)

Remove-RepoDirectory -Path $OutputRoot
New-Directory -Path $OutputRoot
Set-Location $RepoRoot

$traceEvents = @()
$caseResults = @()
$firstDivergence = $null
$suiteTimer = [System.Diagnostics.Stopwatch]::StartNew()

foreach ($case in $suiteCases) {
    $scriptPath = Join-Path $PSScriptRoot $case.script
    if (-not (Test-Path -LiteralPath $scriptPath -PathType Leaf)) {
        throw "Missing compare script: $scriptPath"
    }

    Write-Host "Running compare regression case: $($case.id)"
    $traceEvents += New-TraceEvent -CaseId $case.id -Phase "start" -Status "running"
    $caseTimer = [System.Diagnostics.Stopwatch]::StartNew()
    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        $output = & $scriptPath 2>&1
        $exitCode = $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $previousErrorActionPreference
    }
    $caseTimer.Stop()
    $status = if ($exitCode -eq 0) {
        if ($case.comparison_class -eq "diagnostic-only") { "extracted" } else { "pass" }
    }
    else {
        "fail"
    }
    $traceEvents += New-TraceEvent -CaseId $case.id -Phase "finish" -Status $status -DurationMs ([int]$caseTimer.ElapsedMilliseconds)

    $result = [pscustomobject]@{
        id = $case.id
        kind = $case.kind
        comparison_class = $case.comparison_class
        command = $case.script
        status = $status
        exit_code = $exitCode
        duration_ms = [int]$caseTimer.ElapsedMilliseconds
    }
    $caseResults += $result

    if ($exitCode -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        $firstDivergence = [pscustomobject]@{
            case_id = $case.id
            reason = "case exited with $exitCode"
            output_tail = Get-OutputTail -Output $output
        }
        break
    }
}

$suiteTimer.Stop()
Write-CompareArtifacts -Cases $caseResults -TraceEvents $traceEvents -FirstDivergence $firstDivergence -TotalDurationMs ([int]$suiteTimer.ElapsedMilliseconds)

Write-Host "Compare regression artifacts:"
Write-Host "  $OutputRoot\trace.json"
Write-Host "  $OutputRoot\compare-summary.json"
Write-Host "  $OutputRoot\compare-report.md"
Write-Host "  $OutputRoot\profile-summary.json"

if ($null -ne $firstDivergence) {
    throw "Compare regression failed at $($firstDivergence.case_id)"
}

Write-Host "Compare regression passed."
