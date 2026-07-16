[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-glazing-refraction-extinction\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_glazing_refraction_extinction_001"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_glazing_refraction_extinction_001\window_glazing_refraction_extinction.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing window glazing refraction-extinction fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "window-glazing-refraction-extinction.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus window glazing refraction-extinction oracle case."
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
$materialRow = "WindowMaterial:Glazing,DISTINCTIVE REFRACTION EXTINCTION GLASS,SpectralAverage,,6.70000E-003,0.81705,6.22029E-002,6.22029E-002,0.84538,9.81859E-002,6.22029E-002,1.70000E-002,0.81100,0.81100,1.23000,0.91000,Yes"
$constructionRow = "WindowConstruction,DISTINCTIVE REFRACTION EXTINCTION WINDOW CONSTRUCTION,2,1,VerySmooth,5.731,5.731,1.000,0.799,0.744,0.770"
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE REFRACTION EXTINCTION HOST WALL,Wall,,CTF - ConductionTransferFunction,REFRACTION EXTINCTION OPAQUE HOST CONSTRUCTION,3.071,2.104,,8.00,12.00,8.00,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$windowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE REFRACTION EXTINCTION WINDOW,Window,DISTINCTIVE REFRACTION EXTINCTION HOST WALL,Window5 Detailed Fenestration,DISTINCTIVE REFRACTION EXTINCTION WINDOW CONSTRUCTION,N/A,5.731,Yes,4.00,4.00,4.00,180.00,90.00,2.00,2.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

Assert-UniqueExactEioRow -Lines $eioLines -Prefix "WindowMaterial:Glazing," -Expected $materialRow -Description "normalized window glazing material"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "WindowConstruction," -Expected $constructionRow -Description "window construction"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE REFRACTION EXTINCTION HOST WALL," -Expected $hostSurfaceRow -Description "opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE REFRACTION EXTINCTION WINDOW," -Expected $windowSurfaceRow -Description "fenestration heat-transfer surface"

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-glazing-refraction-extinction.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "window-glazing-refraction-extinction.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-glazing-refraction-extinction.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing bounded Rust refraction-extinction glazing inputs with normalized EnergyPlus EIO."
$output = & $cargo.Source run -p ep_cli --quiet -- compare window-glazing-refraction-extinction $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Window glazing refraction-extinction comparison smoke failed."
}

$text = ($output -join [Environment]::NewLine)
Assert-Contains -Text $text -Pattern "Window Glazing RefractionExtinction Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "window_runtime_claim: false" -Description "window runtime boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: absolute-0.00001" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "material_occurrences: 1" -Description "Rust material occurrence count"
Assert-Contains -Text $text -Pattern "oracle_material_rows: 1" -Description "oracle material row count"
Assert-Contains -Text $text -Pattern "material: DISTINCTIVE REFRACTION EXTINCTION GLASS" -Description "distinctive material detail"
Assert-Contains -Text $text -Pattern "typed_only: material: DISTINCTIVE REFRACTION EXTINCTION GLASS solar_index: 1.470000 solar_extinction_per_m: 19.300000 visible_index: 1.610000 visible_extinction_per_m: 8.700000" -Description "typed-only refraction-extinction inputs"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

$report = @(
    "# Window glazing refraction-extinction smoke report",
    "",
    "- Case: window_glazing_refraction_extinction_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Window runtime claim: false",
    "",
    "## Exact oracle evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialRow,
    $constructionRow,
    $hostSurfaceRow,
    $windowSurfaceRow,
    "~~~",
    "",
    "The normalized EnergyPlus 26.1 row repeats solar front reflectance in the visible-back slot. This is source-actual diagnostic evidence, not an intended visible-back optical-physics claim.",
    "",
    "## Bounded typed-input comparison",
    "",
    "~~~text",
    $text,
    "~~~",
    "",
    "This report is non-blocking diagnostic-only static input evidence. It makes no window runtime or conformance claim.",
    ""
) -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "Window glazing refraction-extinction report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window glazing refraction-extinction smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Window runtime claim: false" -Description "report runtime boundary"
Assert-Contains -Text $reportText -Pattern "source-actual diagnostic evidence" -Description "report normalized-row boundary"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"

Write-Host "Window glazing refraction-extinction comparison smoke passed."
Write-Host "Diagnostic-only, nonblocking evidence; no window runtime or conformance claim."
Write-Host "Report: $ReportPath"
