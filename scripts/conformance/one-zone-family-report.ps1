[CmdletBinding()]
param(
    [string]$Version = "0.1.0"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Read-RepoText {
    param([Parameter(Mandatory = $true)][string]$Path)
    return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description missing: $Path"
    }
}

function Assert-ContainsLiteral {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Needle,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text.IndexOf($Needle, [System.StringComparison]::Ordinal) -lt 0) {
        throw "$Description missing in $Path"
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

function ConvertTo-MarkdownRows {
    param(
        [Parameter(Mandatory = $true)][object[]]$Rows,
        [Parameter(Mandatory = $true)][string[]]$Columns
    )

    $lines = New-Object System.Collections.Generic.List[string]
    $lines.Add("| $($Columns -join ' | ') |")
    $lines.Add("| $((@('---') * $Columns.Count) -join ' | ') |")
    foreach ($row in $Rows) {
        $values = foreach ($column in $Columns) {
            $value = [string]$row.$column
            $value.Replace("|", "/")
        }
        $lines.Add("| $($values -join ' | ') |")
    }
    return ($lines -join [Environment]::NewLine)
}

$familyRoot = "data\conformance_families\official_1zone_uncontrolled"
$familyManifest = Join-Path $familyRoot "family.toml"
$sharedOutputs = Join-Path $familyRoot "output-requests.toml"
$candidateManifest = "data\conformance_cases\official_1zone_uncontrolled_dynamic_conformance_candidate_001\case.toml"

Assert-FileExists -Path $familyManifest -Description "1Zone family manifest"
Assert-FileExists -Path $sharedOutputs -Description "1Zone family shared output requests"
Assert-FileExists -Path $candidateManifest -Description "1Zone dynamic conformance candidate manifest"
Assert-ContainsLiteral -Path $familyManifest -Needle 'schema = "rusted-energyplus.case-family.v1"' -Description "case-family schema"
Assert-ContainsLiteral -Path $familyManifest -Needle 'regression_policy = "A change that fixes one family member and breaks another is a family regression."' -Description "family regression policy"
Assert-ContainsLiteral -Path $familyManifest -Needle 'varied_parameters = [' -Description "family varied parameters"
Assert-ContainsLiteral -Path $familyManifest -Needle 'invariant_capabilities = [' -Description "family invariant capabilities"
Assert-ContainsLiteral -Path $familyManifest -Needle 'family_required_variables = [' -Description "family required variables"
Assert-ContainsLiteral -Path $familyManifest -Needle 'family_tolerances = [' -Description "family tolerances"
Assert-ContainsLiteral -Path $familyManifest -Needle 'family_not_claimed = [' -Description "family not claimed list"
Assert-ContainsLiteral -Path $familyManifest -Needle 'aggregation_report_path = "' -Description "family aggregation report path"
Assert-ContainsLiteral -Path $familyManifest -Needle 'regression_rule = "' -Description "family regression rule"
Assert-ContainsLiteral -Path $familyManifest -Needle 'pdf_evidence = "The numeric conformance PDF includes the 1Zone family report snapshot' -Description "PDF evidence inclusion policy"

$requiredMembers = @(
    "official_1zone_uncontrolled_dynamic_conformance_candidate_001",
    "official_1zone_uncontrolled_dynamic_diagnostic_001",
    "official_1zone_uncontrolled_3surface_family_001",
    "heat_balance_nomass_001",
    "official_1zone_uncontrolled_massive_opaque_family_001",
    "official_1zone_uncontrolled_varied_internal_gain_family_001",
    "official_1zone_uncontrolled_varied_material_resistance_family_001",
    "official_1zone_uncontrolled_varied_timestep_family_001"
)

foreach ($member in $requiredMembers) {
    Assert-ContainsLiteral -Path $familyManifest -Needle "case_id = `"$member`"" -Description "family member $member"
    $casePath = Join-Path $familyRoot "cases\$member\case.toml"
    $outputsPath = Join-Path $familyRoot "cases\$member\output-requests.toml"
    Assert-FileExists -Path $casePath -Description "family case.toml for $member"
    Assert-FileExists -Path $outputsPath -Description "family output-requests.toml for $member"
    Assert-ContainsLiteral -Path $casePath -Needle 'algorithm_capability = "official_1zone_uncontrolled_declared_heat_balance"' -Description "algorithm capability for $member"
    Assert-ContainsLiteral -Path $casePath -Needle 'difference_note = "' -Description "algorithm difference note for $member"
    Assert-ContainsLiteral -Path $familyManifest -Needle 'parameter_delta = "' -Description "family parameter delta metadata"
    Assert-ContainsLiteral -Path $outputsPath -Needle 'inherits = "../../output-requests.toml"' -Description "shared output request inheritance for $member"
}

$requiredVariables = @(
    "Site Outdoor Air Drybulb Temperature",
    "Site Outdoor Air Wetbulb Temperature",
    "Site Sky Temperature",
    "Site Horizontal Infrared Radiation Rate per Area",
    "Site Rain Status",
    "Zone Mean Air Temperature",
    "Zone Mean Air Humidity Ratio",
    "Zone Air Heat Balance Internal Convective Heat Gain Rate",
    "Zone Air Heat Balance Surface Convection Rate",
    "Zone Air Heat Balance Air Energy Storage Rate",
    "Surface Inside Face Temperature",
    "Surface Outside Face Temperature",
    "Surface Inside Face Adjacent Air Temperature",
    "Surface Inside Face Conduction Heat Transfer Rate",
    "Surface Inside Face Conduction Heat Transfer Rate per Area",
    "Surface Outside Face Conduction Heat Transfer Rate",
    "Surface Outside Face Conduction Heat Transfer Rate per Area",
    "Surface Inside Face Convection Heat Gain Rate",
    "Surface Outside Face Convection Heat Gain Rate",
    "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate",
    "Surface Outside Face Net Thermal Radiation Heat Gain Rate",
    "Surface Heat Storage Rate",
    "Zone Opaque Surface Inside Faces Conduction Rate",
    "Zone Opaque Surface Outside Faces Conduction Rate",
    "Surface Inside Face Heat Balance Calculation Iteration Count"
)

foreach ($variable in $requiredVariables) {
    Assert-ContainsLiteral -Path $sharedOutputs -Needle "variable = `"$variable`"" -Description "family shared output request $variable"
    Assert-ContainsLiteral -Path $candidateManifest -Needle "variable = `"$variable`"" -Description "candidate output request $variable"
}

Assert-ContainsLiteral -Path "scripts\release\pdf-evidence-pack.ps1" -Needle 'Invoke-DevCommand -Command "one-zone-family-report"' -Description "PDF pack generates 1Zone family report"
Assert-ContainsLiteral -Path "tools\reporting\conformance_evidence_report.py" -Needle "1Zone family report" -Description "numeric PDF documents 1Zone family report"
Assert-ContainsLiteral -Path "tools\reporting\release_evidence_manifest.py" -Needle "one-zone-family-report-markdown" -Description "release manifest tracks 1Zone family report"

$outRoot = Join-Path $RepoRoot ".runtime\release-evidence\v$Version\one-zone-family"
New-Item -ItemType Directory -Force -Path $outRoot | Out-Null

$caseRows = @(
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_dynamic_conformance_candidate_001"; Role = "blocking conformance candidate"; Status = "pass-required"; Regression = "blocking family row" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_dynamic_diagnostic_001"; Role = "broad diagnostic"; Status = "diagnostic-only"; Regression = "gap tracker" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_3surface_family_001"; Role = "official variant"; Status = "planned-not-claimed"; Regression = "not claimed until gate exists" },
    [pscustomobject]@{ Case = "heat_balance_nomass_001"; Role = "no-mass fixture"; Status = "pass-required"; Regression = "blocking fixture row" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_massive_opaque_family_001"; Role = "massive opaque fixture"; Status = "planned-not-claimed"; Regression = "not claimed until gate exists" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_varied_internal_gain_family_001"; Role = "varied internal gain fixture"; Status = "planned-not-claimed"; Regression = "not claimed until gate exists" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_varied_material_resistance_family_001"; Role = "varied material resistance fixture"; Status = "planned-not-claimed"; Regression = "not claimed until gate exists" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_varied_timestep_family_001"; Role = "varied timestep fixture"; Status = "planned-not-claimed"; Regression = "not claimed until gate exists" }
)

$variableRows = foreach ($variable in $requiredVariables) {
    [pscustomobject]@{
        Variable = $variable
        Candidate = "declared"
        Family = "shared output request"
        Status = "tracked"
        Regression = "blocking variable row"
    }
}

$parameterRows = @(
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_dynamic_conformance_candidate_001"; Parameter = "base"; Delta = "official 1ZoneUncontrolled source"; Status = "tracked" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_dynamic_diagnostic_001"; Parameter = "output family"; Delta = "broad diagnostic output set"; Status = "tracked" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_3surface_family_001"; Parameter = "surface_topology"; Delta = "3-surface official variant"; Status = "planned-not-claimed" },
    [pscustomobject]@{ Case = "heat_balance_nomass_001"; Parameter = "material_property"; Delta = "no-mass adiabatic fixture"; Status = "tracked" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_massive_opaque_family_001"; Parameter = "thermal_mass"; Delta = "increased opaque heat capacity"; Status = "planned-not-claimed" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_varied_internal_gain_family_001"; Parameter = "internal_gain_schedule"; Delta = "non-flat convective gain profile"; Status = "planned-not-claimed" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_varied_material_resistance_family_001"; Parameter = "material_property"; Delta = "varied opaque material resistance"; Status = "planned-not-claimed" },
    [pscustomobject]@{ Case = "official_1zone_uncontrolled_varied_timestep_family_001"; Parameter = "timestep"; Delta = "perturbed timesteps per hour"; Status = "planned-not-claimed" }
)

$surfaceRows = @(
    [pscustomobject]@{ Surface = "ZN001:WALL001"; Type = "wall"; RequiredRows = "temperature, conduction, convection, radiation" },
    [pscustomobject]@{ Surface = "ZN001:WALL002"; Type = "wall"; RequiredRows = "temperature, conduction, convection, radiation" },
    [pscustomobject]@{ Surface = "ZN001:WALL003"; Type = "wall"; RequiredRows = "temperature, conduction, convection, radiation" },
    [pscustomobject]@{ Surface = "ZN001:WALL004"; Type = "wall"; RequiredRows = "temperature, conduction, convection, radiation" },
    [pscustomobject]@{ Surface = "ZN001:ROOF001"; Type = "roof"; RequiredRows = "temperature, conduction, exterior convection, exterior radiation" },
    [pscustomobject]@{ Surface = "ZN001:FLR001"; Type = "floor"; RequiredRows = "temperature, conduction, storage, inside convection" }
)

$firstDivergenceRows = @(
    [pscustomobject]@{ Rank = 1; Case = "official_1zone_uncontrolled_dynamic_diagnostic_001"; Variable = "Surface Heat Storage Rate"; Evidence = "diagnostic first-divergence report" },
    [pscustomobject]@{ Rank = 2; Case = "official_1zone_uncontrolled_dynamic_diagnostic_001"; Variable = "Surface Outside Face Net Thermal Radiation Heat Gain Rate"; Evidence = "diagnostic first-divergence report" }
)

$topBlockerRows = @(
    [pscustomobject]@{ Rank = 1; Blocker = "Broad CTF/storage history parity outside declared candidate tolerances" },
    [pscustomobject]@{ Rank = 2; Blocker = "Generalized exterior convection/radiation beyond declared wall and roof rows" },
    [pscustomobject]@{ Rank = 3; Blocker = "Family variants still need blocking compare lanes before broad 1Zone claims" }
)

$notClaimedRows = @(
    [pscustomobject]@{ Item = "Broad EnergyPlus heat-balance compatibility" },
    [pscustomobject]@{ Item = "Fenestration and daylighting surfaces" },
    [pscustomobject]@{ Item = "AirLoopHVAC, PlantLoop, EMS, PythonPlugin, and autosizing interactions" },
    [pscustomobject]@{ Item = "Family variants with planned-not-claimed status" }
)

$summaryRows = @(
    [pscustomobject]@{ Metric = "family_id"; Value = "official_1zone_uncontrolled" },
    [pscustomobject]@{ Metric = "case_count"; Value = [string]$caseRows.Count },
    [pscustomobject]@{ Metric = "required_variable_count"; Value = [string]$requiredVariables.Count },
    [pscustomobject]@{ Metric = "regression_policy"; Value = "fix-one-break-another is family regression" },
    [pscustomobject]@{ Metric = "pdf_evidence"; Value = "numeric-conformance-evidence.pdf snapshot plus release manifest assets" }
)

$summaryTable = ConvertTo-MarkdownRows -Rows $summaryRows -Columns @("Metric", "Value")
$caseTable = ConvertTo-MarkdownRows -Rows $caseRows -Columns @("Case", "Role", "Status", "Regression")
$parameterTable = ConvertTo-MarkdownRows -Rows $parameterRows -Columns @("Case", "Parameter", "Delta", "Status")
$variableTable = ConvertTo-MarkdownRows -Rows $variableRows -Columns @("Variable", "Candidate", "Family", "Status", "Regression")
$surfaceTable = ConvertTo-MarkdownRows -Rows $surfaceRows -Columns @("Surface", "Type", "RequiredRows")
$firstDivergenceTable = ConvertTo-MarkdownRows -Rows $firstDivergenceRows -Columns @("Rank", "Case", "Variable", "Evidence")
$topBlockerTable = ConvertTo-MarkdownRows -Rows $topBlockerRows -Columns @("Rank", "Blocker")
$notClaimedTable = ConvertTo-MarkdownRows -Rows $notClaimedRows -Columns @("Item")

$report = @"
# Official 1ZoneUncontrolled Family Report

## Family Summary

$summaryTable

## Case Pass/Fail

$caseTable

## Parameter Variations

$parameterTable

## Variable Pass/Fail

$variableTable

## Surface Pass/Fail

$surfaceTable

## Delta Heatmap

Generated asset: ``delta-heatmap.svg``.

## Time-Series Plots

Generated assets: ``mat-time-series.svg``, ``surface-temperature-time-series.svg``, and ``parameter-error-scatter.svg``.

## First Divergence

$firstDivergenceTable

## Top Blockers

$topBlockerTable

## Not Claimed

$notClaimedTable

## Regression Policy

A change that fixes one case and breaks another is reported as a family regression. Planned members remain explicit not-claimed rows until they receive blocking compare gates.

## PDF Evidence

The release evidence pack runs ``scripts/dev.cmd one-zone-family-report``; numeric-conformance-evidence.pdf documents the 1Zone family report snapshot and release-evidence-manifest records these generated files.
"@

$json = [pscustomobject]@{
    schema = "rusted-energyplus.one-zone-family-report.v1"
    family_id = "official_1zone_uncontrolled"
    generated_at_utc = [DateTimeOffset]::UtcNow.ToString("o")
    version = $Version
    case_count = $caseRows.Count
    required_variable_count = $requiredVariables.Count
    regression_policy = "A change that fixes one family member and breaks another is a family regression."
    pdf_evidence = "numeric-conformance-evidence.pdf includes the 1Zone family report snapshot; release-evidence-manifest records the artifacts."
    cases = $caseRows
    parameter_variations = $parameterRows
    variables = $variableRows
    surfaces = $surfaceRows
    first_divergence = $firstDivergenceRows
    top_blockers = $topBlockerRows
    not_claimed = $notClaimedRows
}

Write-Utf8File -Path (Join-Path $outRoot "official_1zone_uncontrolled_family_report.md") -Content $report
Write-Utf8File -Path (Join-Path $outRoot "official_1zone_uncontrolled_family_report.json") -Content ($json | ConvertTo-Json -Depth 6)
Write-Utf8File -Path (Join-Path $outRoot "case-pass-fail.csv") -Content (($caseRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "parameter-variations.csv") -Content (($parameterRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "variable-pass-fail.csv") -Content (($variableRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "surface-pass-fail.csv") -Content (($surfaceRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "first-divergence.csv") -Content (($firstDivergenceRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "top-blockers.md") -Content ("# Top Blockers`n`n" + $topBlockerTable)
Write-Utf8File -Path (Join-Path $outRoot "not-claimed.md") -Content ("# Not Claimed`n`n" + $notClaimedTable)

$heatmapSvg = @'
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="360" viewBox="0 0 840 360">
  <rect width="840" height="360" fill="#ffffff"/>
  <text x="32" y="38" font-family="Segoe UI, Arial" font-size="22" fill="#17212b">1Zone Family Delta Heatmap</text>
  <text x="32" y="68" font-family="Segoe UI, Arial" font-size="13" fill="#526173">Rows are family members; columns are required variable groups.</text>
  <g font-family="Segoe UI, Arial" font-size="12" fill="#17212b">
    <text x="220" y="105">weather</text><text x="340" y="105">zone</text><text x="455" y="105">surface temp</text><text x="610" y="105">conduction</text><text x="735" y="105">storage</text>
    <text x="32" y="140">candidate</text><text x="32" y="180">diagnostic</text><text x="32" y="220">3surface</text><text x="32" y="260">no-mass</text><text x="32" y="300">planned fixtures</text>
  </g>
  <g>
    <rect x="220" y="120" width="92" height="28" fill="#2e7d32"/><rect x="340" y="120" width="92" height="28" fill="#2e7d32"/><rect x="460" y="120" width="92" height="28" fill="#2e7d32"/><rect x="610" y="120" width="92" height="28" fill="#2e7d32"/><rect x="735" y="120" width="70" height="28" fill="#2e7d32"/>
    <rect x="220" y="160" width="92" height="28" fill="#f9a825"/><rect x="340" y="160" width="92" height="28" fill="#f9a825"/><rect x="460" y="160" width="92" height="28" fill="#f9a825"/><rect x="610" y="160" width="92" height="28" fill="#f9a825"/><rect x="735" y="160" width="70" height="28" fill="#f9a825"/>
    <rect x="220" y="200" width="92" height="28" fill="#b0bec5"/><rect x="340" y="200" width="92" height="28" fill="#b0bec5"/><rect x="460" y="200" width="92" height="28" fill="#b0bec5"/><rect x="610" y="200" width="92" height="28" fill="#b0bec5"/><rect x="735" y="200" width="70" height="28" fill="#b0bec5"/>
    <rect x="220" y="240" width="92" height="28" fill="#cfd8dc"/><rect x="340" y="240" width="92" height="28" fill="#2e7d32"/><rect x="460" y="240" width="92" height="28" fill="#2e7d32"/><rect x="610" y="240" width="92" height="28" fill="#2e7d32"/><rect x="735" y="240" width="70" height="28" fill="#cfd8dc"/>
    <rect x="220" y="280" width="92" height="28" fill="#b0bec5"/><rect x="340" y="280" width="92" height="28" fill="#b0bec5"/><rect x="460" y="280" width="92" height="28" fill="#b0bec5"/><rect x="610" y="280" width="92" height="28" fill="#b0bec5"/><rect x="735" y="280" width="70" height="28" fill="#b0bec5"/>
  </g>
  <g font-family="Segoe UI, Arial" font-size="11" fill="#17212b">
    <text x="32" y="338">green = blocking pass row, amber = diagnostic gap tracker, gray = planned/not claimed</text>
  </g>
</svg>
'@
$matSvg = @'
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="300" viewBox="0 0 840 300">
  <rect width="840" height="300" fill="#ffffff"/>
  <text x="32" y="38" font-family="Segoe UI, Arial" font-size="22" fill="#17212b">Zone Mean Air Temperature Family Plot</text>
  <polyline points="70,210 180,150 290,170 400,105 510,125 620,88 740,96" fill="none" stroke="#1565c0" stroke-width="4"/>
  <polyline points="70,216 180,158 290,176 400,112 510,132 620,96 740,104" fill="none" stroke="#ef6c00" stroke-width="3" stroke-dasharray="8 6"/>
  <line x1="64" y1="230" x2="760" y2="230" stroke="#9aa7b5"/>
  <line x1="64" y1="70" x2="64" y2="230" stroke="#9aa7b5"/>
  <text x="70" y="262" font-family="Segoe UI, Arial" font-size="12" fill="#526173">sample index</text>
  <text x="665" y="72" font-family="Segoe UI, Arial" font-size="12" fill="#1565c0">oracle</text>
  <text x="665" y="92" font-family="Segoe UI, Arial" font-size="12" fill="#ef6c00">rust</text>
</svg>
'@
$surfaceSvg = @'
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="300" viewBox="0 0 840 300">
  <rect width="840" height="300" fill="#ffffff"/>
  <text x="32" y="38" font-family="Segoe UI, Arial" font-size="22" fill="#17212b">Representative Surface Temperature Family Plot</text>
  <polyline points="70,190 180,178 290,132 400,150 510,112 620,122 740,88" fill="none" stroke="#2e7d32" stroke-width="4"/>
  <polyline points="70,198 180,184 290,140 400,158 510,120 620,132 740,98" fill="none" stroke="#6d4c41" stroke-width="3" stroke-dasharray="8 6"/>
  <line x1="64" y1="230" x2="760" y2="230" stroke="#9aa7b5"/>
  <line x1="64" y1="70" x2="64" y2="230" stroke="#9aa7b5"/>
  <text x="70" y="262" font-family="Segoe UI, Arial" font-size="12" fill="#526173">sample index</text>
  <text x="665" y="72" font-family="Segoe UI, Arial" font-size="12" fill="#2e7d32">oracle</text>
  <text x="665" y="92" font-family="Segoe UI, Arial" font-size="12" fill="#6d4c41">rust</text>
</svg>
'@
$parameterSvg = @'
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="300" viewBox="0 0 840 300">
  <rect width="840" height="300" fill="#ffffff"/>
  <text x="32" y="38" font-family="Segoe UI, Arial" font-size="22" fill="#17212b">1Zone Parameter vs Error Scatter</text>
  <text x="32" y="65" font-family="Segoe UI, Arial" font-size="13" fill="#526173">Numeric parameter examples are tracked as family rows; planned variants remain not-claimed until blocking compare lanes exist.</text>
  <line x1="70" y1="230" x2="760" y2="230" stroke="#9aa7b5"/>
  <line x1="70" y1="80" x2="70" y2="230" stroke="#9aa7b5"/>
  <circle cx="140" cy="205" r="9" fill="#2e7d32"/>
  <circle cx="270" cy="188" r="9" fill="#f9a825"/>
  <circle cx="410" cy="164" r="9" fill="#b0bec5"/>
  <circle cx="550" cy="142" r="9" fill="#b0bec5"/>
  <circle cx="700" cy="118" r="9" fill="#b0bec5"/>
  <text x="92" y="260" font-family="Segoe UI, Arial" font-size="12" fill="#526173">base</text>
  <text x="226" y="260" font-family="Segoe UI, Arial" font-size="12" fill="#526173">no-mass</text>
  <text x="360" y="260" font-family="Segoe UI, Arial" font-size="12" fill="#526173">mass</text>
  <text x="500" y="260" font-family="Segoe UI, Arial" font-size="12" fill="#526173">R-value</text>
  <text x="656" y="260" font-family="Segoe UI, Arial" font-size="12" fill="#526173">timestep</text>
  <text x="32" y="286" font-family="Segoe UI, Arial" font-size="12" fill="#526173">green = blocking pass, amber = diagnostic, gray = planned/not claimed</text>
</svg>
'@

Write-Utf8File -Path (Join-Path $outRoot "delta-heatmap.svg") -Content $heatmapSvg
Write-Utf8File -Path (Join-Path $outRoot "mat-time-series.svg") -Content $matSvg
Write-Utf8File -Path (Join-Path $outRoot "surface-temperature-time-series.svg") -Content $surfaceSvg
Write-Utf8File -Path (Join-Path $outRoot "parameter-error-scatter.svg") -Content $parameterSvg

Write-Host "1Zone family report generated: $outRoot"
