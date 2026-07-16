[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-glazing-equivalent-layer\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_glazing_equivalent_layer_001"
$ReportPath = Join-Path $ReportRoot "compare-report.md"

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

function Invoke-External {
    param(
        [Parameter(Mandatory = $true)][string]$FilePath,
        [Parameter(Mandatory = $true)][string[]]$Arguments
    )
    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed ($LASTEXITCODE): $FilePath $($Arguments -join ' ')"
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text.IndexOf($Pattern, [System.StringComparison]::Ordinal) -lt 0) {
        Write-Host $Text
        throw "Missing $($Description): $Pattern"
    }
    Write-Host "OK $($Description): $Pattern"
}

function Assert-UniqueExactEioRow {
    param(
        [Parameter(Mandatory = $true)][string[]]$Lines,
        [Parameter(Mandatory = $true)][string]$Prefix,
        [Parameter(Mandatory = $true)][string]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $rows = @(
        $Lines |
            ForEach-Object { $_.Trim() } |
            Where-Object { $_.StartsWith($Prefix, [System.StringComparison]::Ordinal) }
    )
    if ($rows.Count -ne 1) {
        $rows | ForEach-Object { Write-Host $_ }
        throw "Expected exactly one $Description row with prefix '$Prefix'; found $($rows.Count)."
    }
    if (-not $rows[0].Equals($Expected, [System.StringComparison]::Ordinal)) {
        Write-Host "Expected: $Expected"
        Write-Host "Actual:   $($rows[0])"
        throw "Exact $Description EIO row mismatch."
    }
    Write-Host "OK exact unique $($Description): $Expected"
}

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
$weather = Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
foreach ($path in @($energyPlus, $converter, $weather)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot
Remove-RepoDirectory -Path $ReportRoot
New-Directory -Path $OutputRoot
New-Directory -Path $ReportRoot

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_glazing_equivalent_layer_001\window_glazing_equivalent_layer.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing window glazing equivalent-layer fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "window-glazing-equivalent-layer.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus window glazing equivalent-layer oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$eio = Join-Path $OutputRoot "eplusout.eio"
$err = Join-Path $OutputRoot "eplusout.err"
foreach ($path in @($eio, $err)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "EnergyPlus did not produce required output: $path"
    }
}

$oracleLog = Get-Content -LiteralPath $err -Raw
Assert-Contains -Text $oracleLog -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "clean oracle completion"
if ($oracleLog -match "(?m)^\s*\*\* (?:Warning|Severe) \*\*") {
    Write-Host $oracleLog
    throw "EnergyPlus emitted a warning or severe diagnostic despite the required clean summary."
}

$eioLines = @(Get-Content -LiteralPath $eio)
$constructionRow = "Construction:WindowEquivalentLayer,DISTINCTIVE EQUIVALENT LAYER WINDOW CONSTRUCTION,2,1,5.905,0.661,0.643"
$materialRow = "WindowMaterial:Glazing:EquivalentLayer,DISTINCTIVE EQUIVALENT LAYER GLASS,SpectralAverage,,0.61200,0.61300,0.13700,0.14900,3.10000E-002,3.20000E-002,0.14100,0.14200,0.50100,0.20100,0.20200,1.10000E-002,0.82300,0.78600"
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE WINDOW HOST WALL,Wall,,CTF - ConductionTransferFunction,DISTINCTIVE OPAQUE HOST CONSTRUCTION,3.071,2.104,,8.00,12.00,8.00,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$windowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE EQUIVALENT LAYER WINDOW,Window,DISTINCTIVE WINDOW HOST WALL,Window5 Detailed Fenestration,DISTINCTIVE EQUIVALENT LAYER WINDOW CONSTRUCTION,N/A,5.905,No,4.00,4.00,4.00,180.00,90.00,2.00,2.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

Assert-UniqueExactEioRow -Lines $eioLines -Prefix "Construction:WindowEquivalentLayer," -Expected $constructionRow -Description "equivalent-layer window construction"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "WindowMaterial:Glazing:EquivalentLayer," -Expected $materialRow -Description "equivalent-layer window glazing material"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE WINDOW HOST WALL," -Expected $hostSurfaceRow -Description "opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE EQUIVALENT LAYER WINDOW," -Expected $windowSurfaceRow -Description "equivalent-layer fenestration heat-transfer surface"

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-glazing-equivalent-layer.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "window-glazing-equivalent-layer.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-glazing-equivalent-layer.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing bounded Rust equivalent-layer glazing inputs with EnergyPlus EIO."
$output = & $cargo.Source run -p ep_cli --quiet -- compare window-glazing-equivalent-layer $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Window glazing equivalent-layer comparison smoke failed."
}

$text = ($output -join [Environment]::NewLine)
Assert-Contains -Text $text -Pattern "Window Glazing EquivalentLayer Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "window_runtime_claim: false" -Description "window runtime boundary"
Assert-Contains -Text $text -Pattern "fenestration_surface_claim: false" -Description "fenestration surface boundary"
Assert-Contains -Text $text -Pattern "equivalent_layer_construction_claim: false" -Description "equivalent-layer construction boundary"
Assert-Contains -Text $text -Pattern "construction_occurrence_claim: false" -Description "construction occurrence boundary"
Assert-Contains -Text $text -Pattern "ashwat_runtime_claim: false" -Description "ASHWAT runtime boundary"
Assert-Contains -Text $text -Pattern "construction_rating_claim: false" -Description "construction rating boundary"
Assert-Contains -Text $text -Pattern "spectral_dataset_input_claim: false" -Description "spectral dataset input boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: absolute-0.00001" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "material_objects: 1" -Description "Rust material object count"
Assert-Contains -Text $text -Pattern "oracle_material_occurrences: 1" -Description "oracle material occurrence count"
Assert-Contains -Text $text -Pattern "material: DISTINCTIVE EQUIVALENT LAYER GLASS/DISTINCTIVE EQUIVALENT LAYER GLASS" -Description "distinctive equivalent-layer material detail"
Assert-Contains -Text $text -Pattern "typed_only: material: DISTINCTIVE EQUIVALENT LAYER GLASS visible_beam_beam_tf: 0.721000 visible_beam_beam_tb: 0.109000 visible_beam_beam_rf: 0.123000 visible_beam_beam_rb: 0.021000 visible_beam_diffuse_tf: 0.041000 visible_beam_diffuse_tb: 0.042000 visible_beam_diffuse_rf: 0.051000 visible_beam_diffuse_rb: 0.152000 visible_diffuse_diffuse_t: 0.601000 visible_diffuse_diffuse_rf: 0.101000 visible_diffuse_diffuse_rb: 0.102000 thermal_resistance_m2_k_per_w: 0.177000" -Description "typed-only visible and thermal-resistance inputs"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

$report = @(
    "# Window glazing equivalent-layer smoke report",
    "",
    "- Case: window_glazing_equivalent_layer_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Window runtime claim: false",
    "- Fenestration surface claim: false",
    "- Equivalent-layer construction claim: false",
    "- Construction occurrence claim: false",
    "- ASHWAT runtime claim: false",
    "- Construction rating claim: false",
    "- Spectral dataset input claim: false",
    "",
    "## Exact oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $constructionRow,
    $materialRow,
    $hostSurfaceRow,
    $windowSurfaceRow,
    "~~~",
    "",
    "The material row supplies the bounded EIO comparison. The construction and heat-transfer-surface rows are oracle-only fixture-integrity locks; their ratings, layer-occurrence semantics, and surface behavior are not Rust parity claims.",
    "",
    "## Bounded typed-input comparison",
    "",
    "~~~text",
    $text,
    "~~~",
    "",
    "The 11 visible optical inputs and thermal resistance are typed-only because this EIO row omits them. A blank EIO spectral dataset is not an input-preservation claim.",
    "The EIO -99999 sentinel for Autocalculate is locked by parser and CLI unit tests; this fixture uses explicit numbers and does not claim an ASHWAT-derived value.",
    "",
    "This report is non-blocking diagnostic-only static material evidence. It makes no equivalent-layer construction, ASHWAT, fenestration runtime, rating, or conformance claim.",
    ""
) -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "Window glazing equivalent-layer report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window glazing equivalent-layer smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Fenestration surface claim: false" -Description "report surface boundary"
Assert-Contains -Text $reportText -Pattern "Equivalent-layer construction claim: false" -Description "report construction boundary"
Assert-Contains -Text $reportText -Pattern "ASHWAT runtime claim: false" -Description "report ASHWAT boundary"
Assert-Contains -Text $reportText -Pattern "Construction rating claim: false" -Description "report rating boundary"
Assert-Contains -Text $reportText -Pattern "Spectral dataset input claim: false" -Description "report dataset boundary"
Assert-Contains -Text $reportText -Pattern "oracle-only fixture-integrity locks" -Description "report oracle-only boundary"
Assert-Contains -Text $reportText -Pattern "fixture uses explicit numbers" -Description "report Autocalculate sentinel boundary"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"

Write-Host "Window glazing equivalent-layer comparison smoke passed."
Write-Host "Diagnostic-only, nonblocking evidence; no construction, window runtime, rating, or conformance claim."
Write-Host "Report: $ReportPath"
