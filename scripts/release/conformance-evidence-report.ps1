[CmdletBinding()]
param(
    [string]$Version = "0.12.0",
    [switch]$SkipGateRun
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$EvidenceRoot = Join-Path $RepoRoot ".runtime\release-evidence\v$Version"
$HtmlPath = Join-Path $EvidenceRoot "numeric-conformance-evidence.html"
$PdfPath = Join-Path $EvidenceRoot "numeric-conformance-evidence.pdf"
$JsonPath = Join-Path $EvidenceRoot "numeric-conformance-evidence.json"

function Html-Escape {
    param([AllowNull()][string]$Value)
    if ($null -eq $Value) {
        return ""
    }
    return [System.Net.WebUtility]::HtmlEncode($Value)
}

function Number-Label {
    param([double]$Value, [int]$Digits = 6)
    return $Value.ToString("F$Digits", [System.Globalization.CultureInfo]::InvariantCulture)
}

function Elapsed-Seconds {
    param([string]$Path)
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        return $null
    }
    $text = Get-Content -LiteralPath $Path -Raw
    $match = [regex]::Match($text, "Elapsed Time=(?<hours>\d+)hr\s+(?<minutes>\d+)min\s+(?<seconds>[0-9.]+)sec")
    if (-not $match.Success) {
        return $null
    }
    return ([double]$match.Groups["hours"].Value * 3600.0) +
        ([double]$match.Groups["minutes"].Value * 60.0) +
        [double]$match.Groups["seconds"].Value
}

function Error-Summary {
    param([string]$Path)
    $warnings = $null
    $severes = $null
    if (Test-Path -LiteralPath $Path -PathType Leaf) {
        $text = Get-Content -LiteralPath $Path -Raw
        $match = [regex]::Match($text, "Completed Successfully--\s*(?<warnings>\d+) Warning;\s*(?<severes>\d+) Severe")
        if ($match.Success) {
            $warnings = [int]$match.Groups["warnings"].Value
            $severes = [int]$match.Groups["severes"].Value
        }
    }
    return [pscustomobject]@{
        warnings = $warnings
        severes = $severes
    }
}

function Browser-Executable {
    $candidates = @()
    if ($env:EDGE_BIN) { $candidates += $env:EDGE_BIN }
    foreach ($name in @("msedge.exe", "chrome.exe", "chromium.exe")) {
        $command = Get-Command $name -ErrorAction SilentlyContinue
        if ($null -ne $command) {
            $candidates += $command.Source
        }
    }
    $candidates += @(
        "$env:ProgramFiles\Microsoft\Edge\Application\msedge.exe",
        "${env:ProgramFiles(x86)}\Microsoft\Edge\Application\msedge.exe",
        "$env:ProgramFiles\Google\Chrome\Application\chrome.exe",
        "${env:ProgramFiles(x86)}\Google\Chrome\Application\chrome.exe"
    )

    foreach ($candidate in $candidates) {
        if ($candidate -and (Test-Path -LiteralPath $candidate -PathType Leaf)) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
    }
    throw "No headless browser found for PDF generation. Install Microsoft Edge/Chrome or set EDGE_BIN."
}

function Write-Pdf {
    param(
        [Parameter(Mandatory = $true)][string]$Html,
        [Parameter(Mandatory = $true)][string]$Pdf
    )
    $browser = Browser-Executable
    $htmlFull = (Resolve-Path -LiteralPath $Html).Path
    $htmlUri = [System.Uri]::new($htmlFull).AbsoluteUri
    $pdfFull = [System.IO.Path]::GetFullPath($Pdf)
    if (Test-Path -LiteralPath $pdfFull -PathType Leaf) {
        Remove-Item -LiteralPath $pdfFull -Force
    }
    & $browser --headless --disable-gpu "--print-to-pdf=$pdfFull" $htmlUri | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Headless browser PDF generation failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path -LiteralPath $pdfFull -PathType Leaf)) {
        throw "Headless browser did not create PDF: $pdfFull"
    }
    if ((Get-Item -LiteralPath $pdfFull).Length -lt 1024) {
        throw "Headless browser created an unexpectedly small PDF: $pdfFull"
    }
}

function Tolerance-For-Class {
    param(
        [Parameter(Mandatory = $true)]$Summary,
        [Parameter(Mandatory = $true)][string]$Class
    )
    foreach ($tolerance in @($Summary.tolerance_policy)) {
        if ($tolerance.variable_class -eq $Class) {
            return [double]$tolerance.max_abs_c
        }
    }
    return $null
}

function Build-Accuracy-Chart {
    param([array]$Cases)
    $rows = @()
    foreach ($case in $Cases) {
        foreach ($series in @($case.series)) {
            $rows += [pscustomobject]@{
                label = "$($case.milestone) $($series.key) $($series.variable)"
                delta = [double]$series.max_abs_delta_c
                tolerance = [double]$series.tolerance_max_abs_c
            }
        }
    }
    $width = 820
    $left = 250
    $right = 40
    $top = 28
    $rowHeight = 42
    $height = $top + ($rows.Count * $rowHeight) + 30
    $plotWidth = $width - $left - $right
    $maxValue = 0.0
    foreach ($row in $rows) {
        $maxValue = [Math]::Max($maxValue, [Math]::Max($row.delta, $row.tolerance))
    }
    if ($maxValue -le 0.0) {
        $maxValue = 1.0
    }

    $svg = New-Object System.Text.StringBuilder
    [void]$svg.AppendLine("<svg viewBox='0 0 $width $height' class='chart' role='img' aria-label='Accuracy chart'>")
    [void]$svg.AppendLine("<line x1='$left' y1='14' x2='$left' y2='$($height - 16)' class='axis' />")
    for ($i = 0; $i -lt $rows.Count; $i += 1) {
        $row = $rows[$i]
        $y = $top + ($i * $rowHeight)
        $tolWidth = [Math]::Max(1.0, $plotWidth * ($row.tolerance / $maxValue))
        $deltaWidth = if ($row.delta -eq 0.0) { 2.0 } else { [Math]::Max(2.0, $plotWidth * ($row.delta / $maxValue)) }
        [void]$svg.AppendLine("<text x='12' y='$($y + 15)' class='chart-label'>$(Html-Escape $row.label)</text>")
        [void]$svg.AppendLine("<rect x='$left' y='$($y + 4)' width='$tolWidth' height='12' class='tol-bar' />")
        [void]$svg.AppendLine("<rect x='$left' y='$($y + 22)' width='$deltaWidth' height='8' class='delta-bar' />")
        [void]$svg.AppendLine("<text x='$($left + $plotWidth + 8)' y='$($y + 15)' class='chart-value'>tol $(Number-Label $row.tolerance 12)</text>")
        [void]$svg.AppendLine("<text x='$($left + $plotWidth + 8)' y='$($y + 31)' class='chart-value'>delta $(Number-Label $row.delta 12)</text>")
    }
    [void]$svg.AppendLine("</svg>")
    return $svg.ToString()
}

function Build-Timing-Chart {
    param([array]$Cases)
    $width = 820
    $left = 190
    $right = 40
    $top = 28
    $rowHeight = 52
    $height = $top + ($Cases.Count * $rowHeight) + 30
    $plotWidth = $width - $left - $right
    $maxValue = 0.0
    foreach ($case in $Cases) {
        $maxValue = [Math]::Max($maxValue, [double]$case.gate_elapsed_seconds)
        if ($null -ne $case.energyplus_elapsed_seconds) {
            $maxValue = [Math]::Max($maxValue, [double]$case.energyplus_elapsed_seconds)
        }
    }
    if ($maxValue -le 0.0) {
        $maxValue = 1.0
    }

    $svg = New-Object System.Text.StringBuilder
    [void]$svg.AppendLine("<svg viewBox='0 0 $width $height' class='chart' role='img' aria-label='Timing chart'>")
    [void]$svg.AppendLine("<line x1='$left' y1='14' x2='$left' y2='$($height - 16)' class='axis' />")
    for ($i = 0; $i -lt $Cases.Count; $i += 1) {
        $case = $Cases[$i]
        $y = $top + ($i * $rowHeight)
        $gateWidth = [Math]::Max(2.0, $plotWidth * ([double]$case.gate_elapsed_seconds / $maxValue))
        $oracleSeconds = if ($null -eq $case.energyplus_elapsed_seconds) { 0.0 } else { [double]$case.energyplus_elapsed_seconds }
        $oracleWidth = [Math]::Max(2.0, $plotWidth * ($oracleSeconds / $maxValue))
        [void]$svg.AppendLine("<text x='12' y='$($y + 17)' class='chart-label'>$(Html-Escape "$($case.milestone) $($case.case_id)")</text>")
        [void]$svg.AppendLine("<rect x='$left' y='$($y + 5)' width='$gateWidth' height='12' class='gate-bar' />")
        [void]$svg.AppendLine("<rect x='$left' y='$($y + 27)' width='$oracleWidth' height='12' class='oracle-bar' />")
        [void]$svg.AppendLine("<text x='$($left + $plotWidth + 8)' y='$($y + 16)' class='chart-value'>gate $(Number-Label $case.gate_elapsed_seconds 3)s</text>")
        [void]$svg.AppendLine("<text x='$($left + $plotWidth + 8)' y='$($y + 38)' class='chart-value'>oracle $(Number-Label $oracleSeconds 3)s</text>")
    }
    [void]$svg.AppendLine("</svg>")
    return $svg.ToString()
}

function Render-Html {
    param(
        [Parameter(Mandatory = $true)]$Evidence
    )

    $cases = @($Evidence.cases)
    $accuracyChart = Build-Accuracy-Chart -Cases $cases
    $timingChart = Build-Timing-Chart -Cases $cases
    $caseRows = New-Object System.Text.StringBuilder
    foreach ($case in $cases) {
        $energyPlusElapsed = if ($null -eq $case.energyplus_elapsed_seconds) { "n/a" } else { "$(Number-Label $case.energyplus_elapsed_seconds 3)s" }
        [void]$caseRows.AppendLine("<tr><td>$(Html-Escape $case.milestone)</td><td>$(Html-Escape $case.case_id)</td><td>$(Html-Escape $case.status)</td><td>$($case.series_count)</td><td>$($case.samples)</td><td>$($case.heat_balance_timesteps)</td><td>$(Number-Label $case.max_abs_delta_c 12)</td><td>$(Number-Label $case.rmse_delta_c 12)</td><td>$(Number-Label $case.gate_elapsed_seconds 3)s</td><td>$energyPlusElapsed</td></tr>")
    }

    $seriesRows = New-Object System.Text.StringBuilder
    foreach ($case in $cases) {
        foreach ($series in @($case.series)) {
            [void]$seriesRows.AppendLine("<tr><td>$(Html-Escape $case.case_id)</td><td>$(Html-Escape $series.key)</td><td>$(Html-Escape $series.variable)</td><td>$(Html-Escape $series.class)</td><td>$($series.samples)</td><td>$(Number-Label $series.max_abs_delta_c 12)</td><td>$(Number-Label $series.rmse_delta_c 12)</td><td>$(Number-Label $series.tolerance_max_abs_c 12)</td><td>$(Html-Escape $series.status)</td></tr>")
        }
    }

    return @"
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>eplus-rs $($Evidence.version) Numeric Conformance Evidence</title>
<style>
@page { size: Letter; margin: 0.55in; }
* { box-sizing: border-box; }
body { font-family: Segoe UI, Arial, sans-serif; color: #18202a; line-height: 1.35; }
h1 { font-size: 28px; margin: 0 0 8px; }
h2 { font-size: 18px; margin: 28px 0 10px; border-bottom: 1px solid #d7dde5; padding-bottom: 5px; }
h3 { font-size: 14px; margin: 18px 0 8px; }
p { margin: 8px 0; }
.meta { color: #5b6775; font-size: 11px; }
.banner { border: 1px solid #b8c7d9; background: #f4f8fb; padding: 10px 12px; border-radius: 6px; margin: 14px 0; }
.cards { display: grid; grid-template-columns: repeat(4, 1fr); gap: 8px; margin: 14px 0; }
.card { border: 1px solid #d7dde5; border-radius: 6px; padding: 9px; background: #fff; }
.card .label { color: #5b6775; font-size: 10px; text-transform: uppercase; letter-spacing: 0.06em; }
.card .value { font-size: 17px; font-weight: 650; margin-top: 3px; }
table { width: 100%; border-collapse: collapse; margin: 8px 0 16px; font-size: 10px; }
th, td { border-bottom: 1px solid #e3e7ed; padding: 5px 6px; text-align: left; vertical-align: top; }
th { background: #eef3f7; color: #2d3742; font-weight: 650; }
.chart { width: 100%; height: auto; border: 1px solid #d7dde5; border-radius: 6px; background: #fff; margin: 8px 0 16px; }
.axis { stroke: #9aa7b5; stroke-width: 1; }
.tol-bar { fill: #c9d8e8; }
.delta-bar { fill: #1f7a5a; }
.gate-bar { fill: #3c6e9f; }
.oracle-bar { fill: #c77d1a; }
.chart-label { font-size: 10px; fill: #2d3742; }
.chart-value { font-size: 9px; fill: #5b6775; }
.legend { font-size: 10px; color: #5b6775; margin-top: -8px; }
.page-break { break-before: page; }
.mono { font-family: Consolas, Menlo, monospace; }
ul { margin: 6px 0 12px 20px; padding: 0; }
li { margin: 3px 0; }
</style>
</head>
<body>
<h1>eplus-rs $($Evidence.version) Numeric Conformance Evidence</h1>
<div class="meta">Generated UTC: $($Evidence.generated_at_utc) | EnergyPlus oracle: $($Evidence.oracle_version) | Report schema: $($Evidence.schema_version)</div>
<div class="banner">
This PDF is release evidence for currently promoted numerical conformance only. It covers the declared v0.8/v0.9 no-mass heat-balance cases and does not claim HVAC, node, plant, meter, fenestration, solar-radiation, warmup, sizing, or general ExampleFiles compatibility.
</div>

<div class="cards">
<div class="card"><div class="label">Cases</div><div class="value">$($Evidence.aggregate.case_count)</div></div>
<div class="card"><div class="label">Series</div><div class="value">$($Evidence.aggregate.series_count)</div></div>
<div class="card"><div class="label">Max Abs Delta C</div><div class="value">$(Number-Label $Evidence.aggregate.max_abs_delta_c 12)</div></div>
<div class="card"><div class="label">Gate Status</div><div class="value">$(Html-Escape $Evidence.aggregate.status)</div></div>
</div>

<h2>Claim Boundary</h2>
<p>The public numerical conformance claim is limited to the listed cases, variables, tolerance policies, and blocking gates. A passing diagnostic extraction, smoke comparison, or baseline-only artifact is excluded from this PDF unless it supports one of these promoted cases.</p>

<h2>Case Matrix</h2>
<table>
<thead><tr><th>Milestone</th><th>Case</th><th>Status</th><th>Series</th><th>Samples</th><th>Rust timesteps</th><th>Max abs C</th><th>RMSE C</th><th>Gate wall</th><th>E+ elapsed</th></tr></thead>
<tbody>
$caseRows
</tbody>
</table>

<h2>Accuracy Against Tolerance</h2>
<div class="legend">Light bars are declared max-absolute tolerances. Green bars are observed max-absolute deltas. Zero deltas are drawn as a visible hairline while labels preserve exact values.</div>
$accuracyChart

<h2>Execution Time Evidence</h2>
<div class="legend">Blue is release gate wall-clock for oracle generation, Rust comparison, and artifact writing. Orange is EnergyPlus self-reported oracle elapsed time from <span class="mono">eplusout.end</span>. These values are release-machine evidence, not a portable benchmark.</div>
$timingChart

<h2>Series Evidence</h2>
<table>
<thead><tr><th>Case</th><th>Key</th><th>Variable</th><th>Class</th><th>Samples</th><th>Max abs C</th><th>RMSE C</th><th>Max abs tolerance C</th><th>Status</th></tr></thead>
<tbody>
$seriesRows
</tbody>
</table>

<div class="page-break"></div>
<h2>Experiment Hygiene For Future Releases</h2>
<p>Future numerical conformance additions should enter this PDF only after they have a manifest, requested variables, tolerance policy, Rust result artifact, markdown/JSON report, and blocking gate. Low-level development checks should remain in smoke, diagnostic, or regression reports unless they explain a promoted claim.</p>
<ul>
<li>Keep static parser/intake checks out of this PDF unless they directly explain a promoted numerical result.</li>
<li>Summarize exploratory experiments as evidence groups, then retire duplicate low-level rows once a higher-level conformance case supersedes them.</li>
<li>Record first divergence, tolerance utilization, and runtime impact for every promoted output variable.</li>
<li>Preserve explicit non-claims for HVAC, node, plant, fenestration, solar, warmup, sizing, and meters until their own cases are promoted.</li>
</ul>

<h2>Artifact Paths</h2>
<table>
<thead><tr><th>Artifact</th><th>Path</th></tr></thead>
<tbody>
<tr><td>HTML evidence</td><td class="mono">$(Html-Escape $Evidence.artifacts.html)</td></tr>
<tr><td>PDF evidence</td><td class="mono">$(Html-Escape $Evidence.artifacts.pdf)</td></tr>
<tr><td>JSON evidence</td><td class="mono">$(Html-Escape $Evidence.artifacts.json)</td></tr>
</tbody>
</table>
</body>
</html>
"@
}

New-Item -ItemType Directory -Force -Path $EvidenceRoot | Out-Null

$caseSpecs = @(
    [pscustomobject]@{
        milestone = "v0.8"
        command = "compare-heat-balance-conformance"
        summary_path = ".runtime\heat-balance-conformance\26.1.0\heat_balance_nomass_001\compare\compare-summary.json"
        oracle_end_path = ".runtime\heat-balance-conformance\26.1.0\heat_balance_nomass_001\oracle\eplusout.end"
        oracle_err_path = ".runtime\heat-balance-conformance\26.1.0\heat_balance_nomass_001\oracle\eplusout.err"
    },
    [pscustomobject]@{
        milestone = "v0.9"
        command = "compare-surface-temperature-conformance"
        summary_path = ".runtime\surface-temperature-conformance\26.1.0\surface_temperature_nomass_001\compare\compare-summary.json"
        oracle_end_path = ".runtime\surface-temperature-conformance\26.1.0\surface_temperature_nomass_001\oracle\eplusout.end"
        oracle_err_path = ".runtime\surface-temperature-conformance\26.1.0\surface_temperature_nomass_001\oracle\eplusout.err"
    }
)

$caseReports = @()
foreach ($spec in $caseSpecs) {
    $gateElapsedSeconds = $null
    if (-not $SkipGateRun) {
        $watch = [System.Diagnostics.Stopwatch]::StartNew()
        Invoke-DevCommand -Command $spec.command
        $watch.Stop()
        $gateElapsedSeconds = $watch.Elapsed.TotalSeconds
    }

    if (-not (Test-Path -LiteralPath $spec.summary_path -PathType Leaf)) {
        throw "Missing conformance summary: $($spec.summary_path)"
    }
    $summary = Get-Content -LiteralPath $spec.summary_path -Raw | ConvertFrom-Json
    if ($summary.comparison_class -ne "conformance" -or $summary.conformance_claim -ne $true) {
        throw "Summary is not a promoted conformance claim: $($spec.summary_path)"
    }
    if ($summary.status -ne "pass") {
        throw "Conformance summary did not pass: $($summary.case_id)"
    }

    if ($null -eq $gateElapsedSeconds) {
        $gateElapsedSeconds = 0.0
    }
    $errorSummary = Error-Summary -Path $spec.oracle_err_path
    $seriesReports = @()
    foreach ($series in @($summary.series)) {
        $tolerance = Tolerance-For-Class -Summary $summary -Class $series.output.class
        $seriesReports += [pscustomobject]@{
            key = $series.output.key
            variable = $series.output.variable
            class = $series.output.class
            frequency = $series.output.frequency
            source = $series.output.source
            samples = [int]$series.samples
            status = $series.status
            max_abs_delta_c = [double]$series.max_abs_delta_c
            mean_abs_delta_c = [double]$series.mean_abs_delta_c
            rmse_delta_c = [double]$series.rmse_delta_c
            max_rel_delta = [double]$series.max_rel_delta
            tolerance_max_abs_c = [double]$tolerance
            first_delta_index = $series.first_delta_sample.index
            max_delta_index = $series.max_delta_sample.index
        }
    }

    $caseReports += [pscustomobject]@{
        milestone = $spec.milestone
        case_id = $summary.case_id
        oracle_version = $summary.oracle_version
        comparison_class = $summary.comparison_class
        conformance_claim = [bool]$summary.conformance_claim
        status = $summary.status
        runtime_class = $summary.runtime_class
        tolerance_policy_label = $summary.tolerance_policy_label
        samples = [int]$summary.samples
        heat_balance_timesteps = [int]$summary.heat_balance_timesteps
        zone_count = [int]$summary.zone_count
        surface_count = [int]$summary.surface_count
        series_count = [int]$summary.series_count
        max_abs_delta_c = [double]$summary.max_abs_delta_c
        rmse_delta_c = [double]$summary.rmse_delta_c
        max_rel_delta = [double]$summary.max_rel_delta
        gate_elapsed_seconds = [double]$gateElapsedSeconds
        energyplus_elapsed_seconds = Elapsed-Seconds -Path $spec.oracle_end_path
        energyplus_warnings = $errorSummary.warnings
        energyplus_severes = $errorSummary.severes
        gate_script = $summary.gate.script
        source_summary_json = $spec.summary_path.Replace("\", "/")
        source_report_md = $summary.report_contract.path
        series = $seriesReports
    }
}

$allSeries = @()
foreach ($case in $caseReports) {
    $allSeries += @($case.series)
}
$aggregateMaxAbs = 0.0
$aggregateRmse = 0.0
foreach ($case in $caseReports) {
    $aggregateMaxAbs = [Math]::Max($aggregateMaxAbs, [double]$case.max_abs_delta_c)
    $aggregateRmse = [Math]::Max($aggregateRmse, [double]$case.rmse_delta_c)
}
$failedCases = @($caseReports | Where-Object { $_.status -ne "pass" })
$aggregateStatus = if ($failedCases.Count -eq 0) { "pass" } else { "fail" }

$evidence = [pscustomobject]@{
    schema_version = 1
    version = $Version
    oracle_version = "26.1.0"
    generated_at_utc = (Get-Date).ToUniversalTime().ToString("o")
    claim_boundary = "Only declared v0.8/v0.9 no-mass heat-balance numerical conformance cases are promoted."
    aggregate = [pscustomobject]@{
        status = $aggregateStatus
        case_count = $caseReports.Count
        series_count = $allSeries.Count
        max_abs_delta_c = $aggregateMaxAbs
        rmse_delta_c = $aggregateRmse
    }
    cases = $caseReports
    artifacts = [pscustomobject]@{
        html = ".runtime/release-evidence/v$Version/numeric-conformance-evidence.html"
        pdf = ".runtime/release-evidence/v$Version/numeric-conformance-evidence.pdf"
        json = ".runtime/release-evidence/v$Version/numeric-conformance-evidence.json"
    }
}

$evidence | ConvertTo-Json -Depth 16 | Set-Content -LiteralPath $JsonPath -Encoding utf8
Render-Html -Evidence $evidence | Set-Content -LiteralPath $HtmlPath -Encoding utf8
Write-Pdf -Html $HtmlPath -Pdf $PdfPath

Write-Host "Numeric conformance evidence report"
Write-Host "  status: $aggregateStatus"
Write-Host "  cases: $($caseReports.Count)"
Write-Host "  series: $($allSeries.Count)"
Write-Host "  max_abs_delta_c: $(Number-Label $aggregateMaxAbs 12)"
Write-Host "  html: $HtmlPath"
Write-Host "  pdf: $PdfPath"
Write-Host "  json: $JsonPath"
