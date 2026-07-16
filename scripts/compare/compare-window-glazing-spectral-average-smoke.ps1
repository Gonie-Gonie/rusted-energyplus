[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-glazing-spectral-average\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_glazing_spectral_average_001"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_glazing_spectral_average_001\window_glazing_spectral_average.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing window glazing SpectralAverage fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "window-glazing-spectral-average.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus window glazing SpectralAverage oracle case."
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
$materialRow = "WindowMaterial:Glazing,DISTINCTIVE SPECTRAL AVERAGE GLASS,SpectralAverage,,7.30000E-003,0.61200,0.13700,0.14900,0.72100,0.10900,0.12300,2.10000E-002,0.82300,0.78600,1.17000,0.93000,Yes"
$constructionRow = "WindowConstruction,DISTINCTIVE SPECTRAL AVERAGE WINDOW CONSTRUCTION,2,1,VerySmooth,5.633,5.633,1.000,0.654,0.571,0.671"
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE WINDOW HOST WALL,Wall,,CTF - ConductionTransferFunction,DISTINCTIVE OPAQUE HOST CONSTRUCTION,3.071,2.104,,8.00,12.00,8.00,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$windowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE SPECTRAL AVERAGE WINDOW,Window,DISTINCTIVE WINDOW HOST WALL,Window5 Detailed Fenestration,DISTINCTIVE SPECTRAL AVERAGE WINDOW CONSTRUCTION,N/A,5.633,Yes,4.00,4.00,4.00,180.00,90.00,2.00,2.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

Assert-UniqueExactEioRow -Lines $eioLines -Prefix "WindowMaterial:Glazing," -Expected $materialRow -Description "window glazing material"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "WindowConstruction," -Expected $constructionRow -Description "window construction"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE WINDOW HOST WALL," -Expected $hostSurfaceRow -Description "opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE SPECTRAL AVERAGE WINDOW," -Expected $windowSurfaceRow -Description "fenestration heat-transfer surface"

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-glazing-spectral-average.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "window-glazing-spectral-average.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-glazing-spectral-average.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing bounded Rust SpectralAverage glazing inputs with EnergyPlus EIO."
$output = & $cargo.Source run -p ep_cli --quiet -- compare window-glazing-spectral-average $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Window glazing SpectralAverage comparison smoke failed."
}

$text = ($output -join [Environment]::NewLine)
Assert-Contains -Text $text -Pattern "Window Glazing SpectralAverage Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "window_runtime_claim: false" -Description "window runtime boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: absolute-0.00001" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "material_occurrences: 1" -Description "Rust material occurrence count"
Assert-Contains -Text $text -Pattern "oracle_material_rows: 1" -Description "oracle material row count"
Assert-Contains -Text $text -Pattern "material: DISTINCTIVE SPECTRAL AVERAGE GLASS" -Description "distinctive material detail"
Assert-Contains -Text $text -Pattern "typed_only: material: DISTINCTIVE SPECTRAL AVERAGE GLASS youngs_modulus_pa: 68000000000.000000 poissons_ratio: 0.240000" -Description "typed-only mechanical properties"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

$report = @(
    "# Window glazing SpectralAverage smoke report",
    "",
    "- Case: window_glazing_spectral_average_001",
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
    throw "Window glazing SpectralAverage report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window glazing SpectralAverage smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Window runtime claim: false" -Description "report runtime boundary"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"

Write-Host "Window glazing SpectralAverage comparison smoke passed."
Write-Host "Diagnostic-only, nonblocking evidence; no window runtime or conformance claim."
Write-Host "Report: $ReportPath"
