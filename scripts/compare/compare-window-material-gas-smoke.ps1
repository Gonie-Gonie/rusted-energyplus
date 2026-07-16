[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-gas\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_gas_001"
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

function Assert-NotContains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text.IndexOf($Pattern, [System.StringComparison]::Ordinal) -ge 0) {
        Write-Host $Text
        throw "Unexpected $($Description): $Pattern"
    }
    Write-Host "OK absent $($Description): $Pattern"
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

function Assert-ExactOrderedEioRows {
    param(
        [Parameter(Mandatory = $true)][string[]]$Lines,
        [Parameter(Mandatory = $true)][string[]]$Prefixes,
        [Parameter(Mandatory = $true)][string[]]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $rows = @(
        $Lines |
            ForEach-Object { $_.Trim() } |
            Where-Object {
                $line = $_
                @($Prefixes | Where-Object {
                    $line.StartsWith($_, [System.StringComparison]::Ordinal)
                }).Count -gt 0
            }
    )
    if ($rows.Count -ne $Expected.Count) {
        $rows | ForEach-Object { Write-Host $_ }
        throw "Expected $($Expected.Count) ordered $Description rows; found $($rows.Count)."
    }
    for ($index = 0; $index -lt $Expected.Count; $index++) {
        if (-not $rows[$index].Equals($Expected[$index], [System.StringComparison]::Ordinal)) {
            Write-Host "Expected[$index]: $($Expected[$index])"
            Write-Host "Actual[$index]:   $($rows[$index])"
            throw "Ordered $Description EIO row mismatch at index $index."
        }
    }
    Write-Host "OK exact ordered $Description rows: $($rows.Count)"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_gas_001\window_material_gas.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:Gas fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "window-material-gas.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus WindowMaterial:Gas oracle case."
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

$gasHeader = "! <WindowMaterial:Gas>,Material Name,GasType,Thickness {m}"
$orderedWindowRows = @(
    "WindowConstruction,A AIR WINDOW CONSTRUCTION,1,3,VerySmooth,2.632,2.632,1.000,0.496,0.358,0.491",
    "WindowMaterial:Gas,DISTINCTIVE AIR GAP,Air,1.110E-002",
    "WindowConstruction,B ARGON WINDOW CONSTRUCTION,2,3,VerySmooth,2.429,2.429,1.000,0.497,0.358,0.491",
    "WindowMaterial:Gas,DISTINCTIVE ARGON GAP,Argon,1.220E-002",
    "WindowConstruction,C CUSTOM WINDOW CONSTRUCTION,3,3,VerySmooth,2.676,2.676,1.000,0.496,0.358,0.491",
    "WindowMaterial:Gas,DISTINCTIVE CUSTOM GAP BLANK RATIO,Custom,1.330E-002",
    "WindowConstruction,D KRYPTON WINDOW CONSTRUCTION,4,3,VerySmooth,2.389,2.389,1.000,0.497,0.358,0.491",
    "WindowMaterial:Gas,DISTINCTIVE KRYPTON GAP,Krypton,1.440E-002",
    "WindowConstruction,E XENON WINDOW CONSTRUCTION,5,3,VerySmooth,2.336,2.336,1.000,0.497,0.358,0.491",
    "WindowMaterial:Gas,DISTINCTIVE XENON GAP,Xenon,1.550E-002",
    "WindowConstruction,F REUSED AIR WINDOW CONSTRUCTION,6,3,VerySmooth,2.632,2.632,1.000,0.496,0.358,0.491",
    "WindowMaterial:Gas,DISTINCTIVE AIR GAP,Air,1.110E-002"
)
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE GAS WINDOW HOST WALL,Wall,,CTF - ConductionTransferFunction,G OPAQUE HOST CONSTRUCTION,3.071,2.104,,6.06,12.00,6.06,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$windowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE GAS TEST WINDOW,Window,DISTINCTIVE GAS WINDOW HOST WALL,Window5 Detailed Fenestration,A AIR WINDOW CONSTRUCTION,N/A,2.632,No,0.99,0.99,0.99,180.00,90.00,0.90,1.10,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

$eioLines = @(Get-Content -LiteralPath $eio)
$eioText = $eioLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <WindowMaterial:Gas>," -Expected $gasHeader -Description "window gas header"
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes @("WindowConstruction,", "WindowMaterial:Gas,") -Expected $orderedWindowRows -Description "window construction and gas occurrence"
Assert-NotContains -Text $eioText -Pattern "WindowMaterial:Gas,DISTINCTIVE UNUSED AIR GAP," -Description "unused gas definition EIO row"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE GAS WINDOW HOST WALL," -Expected $hostSurfaceRow -Description "opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE GAS TEST WINDOW," -Expected $windowSurfaceRow -Description "fenestration heat-transfer surface"

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-material-gas.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "window-material-gas.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-material-gas.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing bounded Rust WindowMaterial:Gas inputs with EnergyPlus EIO occurrences."
$output = & $cargo.Source run -p ep_cli --quiet -- compare window-material-gas $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "WindowMaterial:Gas comparison smoke failed."
}

$text = ($output -join [Environment]::NewLine)
Assert-Contains -Text $text -Pattern "Window Material Gas Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "window_runtime_claim: false" -Description "window runtime boundary"
Assert-Contains -Text $text -Pattern "fenestration_surface_claim: false" -Description "fenestration surface boundary"
Assert-Contains -Text $text -Pattern "construction_rating_claim: false" -Description "construction rating boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: energyplus-26.1-round-sig-digits-3-normalized-exact" -Description "source-format thickness policy"
Assert-Contains -Text $text -Pattern "material_occurrences: 6" -Description "Rust gas-layer occurrence count"
Assert-Contains -Text $text -Pattern "oracle_material_rows: 6" -Description "oracle gas-layer occurrence count"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

$reportLines = @(
    "# Window material gas smoke report",
    "",
    "- Case: window_material_gas_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Window runtime claim: false",
    "- Fenestration surface claim: false",
    "- Construction rating claim: false",
    "",
    "## Exact oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $gasHeader
) + $orderedWindowRows + @(
    $hostSurfaceRow,
    $windowSurfaceRow,
    "~~~",
    "",
    "The six gas rows are construction-layer occurrences in alphabetic Construction declaration order. DISTINCTIVE AIR GAP occurs twice. The valid but unused DISTINCTIVE UNUSED AIR GAP definition is intentionally absent from the EIO.",
    "The construction and heat-transfer-surface rows are oracle-only fixture-integrity locks; their ratings, layer behavior, and surface behavior are not Rust parity claims.",
    "",
    "## Bounded typed-input comparison",
    "",
    "~~~text",
    $text,
    "~~~",
    "",
    "DISTINCTIVE CUSTOM GAP BLANK RATIO uses valid nondefault coefficients and omits its specific-heat ratio. EnergyPlus 26.1 stores that blank numeric input as zero and does not validate it in this source path; the Rust typed slice preserves this source-actual quirk.",
    "",
    "This report is non-blocking diagnostic-only static material evidence. It makes no construction-rating, fenestration-runtime, window-runtime, or conformance claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "WindowMaterial:Gas report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window material gas smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Window runtime claim: false" -Description "report runtime boundary"
Assert-Contains -Text $reportText -Pattern "Construction rating claim: false" -Description "report rating boundary"
Assert-Contains -Text $reportText -Pattern "tolerance_policy: energyplus-26.1-round-sig-digits-3-normalized-exact" -Description "report source-format thickness policy"
Assert-Contains -Text $reportText -Pattern ($orderedWindowRows -join [Environment]::NewLine) -Description "report exact ordered construction and gas rows"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
Assert-Contains -Text $reportText -Pattern "valid but unused DISTINCTIVE UNUSED AIR GAP" -Description "report unused-definition boundary"
Assert-Contains -Text $reportText -Pattern "source-actual quirk" -Description "report blank-ratio boundary"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"

Write-Host "WindowMaterial:Gas comparison smoke passed."
Write-Host "Diagnostic-only, nonblocking evidence; no construction-rating, window runtime, or conformance claim."
Write-Host "Report: $ReportPath"
