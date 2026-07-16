[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-gap-equivalent-layer\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_gap_equivalent_layer_001"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_gap_equivalent_layer_001\window_material_gap_equivalent_layer.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:Gap:EquivalentLayer fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "window-material-gap-equivalent-layer.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus WindowMaterial:Gap:EquivalentLayer oracle case."
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

$constructionHeader = "! <Construction:WindowEquivalentLayer>,Construction Name,Index,#Layers,U-factor {W/m2-K},SHGC, Solar Transmittance at Normal Incidence"
$gapHeader = "! <WindowMaterial:Gap:EquivalentLayer>, Material Name, GasType, Gap Thickness {m}, Gap Vent Type"
$orderedRows = @(
    "Construction:WindowEquivalentLayer,A CUSTOM EQL WINDOW CONSTRUCTION,2,2,2.699,0.753,0.693",
    "WindowMaterial:Gap:EquivalentLayer,M CUSTOM SEALED EQL GAP,Custom,1.330E-002,Sealed",
    "Construction:WindowEquivalentLayer,B AIR OUTDOOR EQL WINDOW CONSTRUCTION,3,2,5.999,0.718,0.693",
    "WindowMaterial:Gap:EquivalentLayer,A AIR VENTED OUTDOOR EQL GAP,Air,1.220E-002,VentedOutdoor",
    "Construction:WindowEquivalentLayer,C KRYPTON EQL WINDOW CONSTRUCTION,4,2,2.313,0.755,0.693",
    "WindowMaterial:Gap:EquivalentLayer,Y KRYPTON SEALED EQL GAP,Krypton,1.440E-002,Sealed",
    "Construction:WindowEquivalentLayer,D AIR INDOOR EQL WINDOW CONSTRUCTION,5,2,3.990,0.762,0.693",
    "WindowMaterial:Gap:EquivalentLayer,B AIR VENTED INDOOR EQL GAP,Air,1.110E-002,VentedIndoor",
    "Construction:WindowEquivalentLayer,E XENON EQL WINDOW CONSTRUCTION,6,2,2.214,0.755,0.693",
    "WindowMaterial:Gap:EquivalentLayer,Z XENON SEALED EQL GAP,Xenon,1.550E-002,Sealed",
    "Construction:WindowEquivalentLayer,F ARGON EQL WINDOW CONSTRUCTION,7,2,2.424,0.754,0.693",
    "WindowMaterial:Gap:EquivalentLayer,C ARGON SEALED EQL GAP,Argon,1.770E-002,Sealed",
    "Construction:WindowEquivalentLayer,G REUSED ARGON EQL WINDOW CONSTRUCTION,8,2,2.424,0.754,0.693",
    "WindowMaterial:Gap:EquivalentLayer,C ARGON SEALED EQL GAP,Argon,1.770E-002,Sealed"
)
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE EQL GAP WINDOW HOST WALL,Wall,,CTF - ConductionTransferFunction,H EQL GAP OPAQUE HOST CONSTRUCTION,3.071,2.104,,6.75,12.00,6.75,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$windowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE EQL GAP TEST WINDOW,Window,DISTINCTIVE EQL GAP WINDOW HOST WALL,Window5 Detailed Fenestration,A CUSTOM EQL WINDOW CONSTRUCTION,N/A,2.699,No,0.75,0.75,0.75,180.00,90.00,0.75,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

$eioLines = @(Get-Content -LiteralPath $eio)
$eioText = $eioLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <Construction:WindowEquivalentLayer>," -Expected $constructionHeader -Description "equivalent-layer construction header"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <WindowMaterial:Gap:EquivalentLayer>," -Expected $gapHeader -Description "equivalent-layer gap header"
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes @("Construction:WindowEquivalentLayer,", "WindowMaterial:Gap:EquivalentLayer,") -Expected $orderedRows -Description "equivalent-layer construction and gap occurrence"
Assert-NotContains -Text $eioText -Pattern "WindowMaterial:Gap:EquivalentLayer,DISTINCTIVE UNUSED AIR SEALED EQL GAP," -Description "unused equivalent-layer gap definition EIO row"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE EQL GAP WINDOW HOST WALL," -Expected $hostSurfaceRow -Description "opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE EQL GAP TEST WINDOW," -Expected $windowSurfaceRow -Description "equivalent-layer fenestration heat-transfer surface"

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-material-gap-equivalent-layer.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "window-material-gap-equivalent-layer.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-material-gap-equivalent-layer.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing bounded Rust equivalent-layer gap inputs with EnergyPlus EIO occurrences."
$output = & $cargo.Source run -p ep_cli --quiet -- compare window-material-gap-equivalent-layer $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "WindowMaterial:Gap:EquivalentLayer comparison smoke failed."
}

$text = ($output -join [Environment]::NewLine)
Assert-Contains -Text $text -Pattern "Window Material Gap EquivalentLayer Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "window_runtime_claim: false" -Description "window runtime boundary"
Assert-Contains -Text $text -Pattern "fenestration_surface_claim: false" -Description "fenestration surface boundary"
Assert-Contains -Text $text -Pattern "equivalent_layer_construction_claim: false" -Description "equivalent-layer construction boundary"
Assert-Contains -Text $text -Pattern "construction_rating_claim: false" -Description "construction rating boundary"
Assert-Contains -Text $text -Pattern "broad_idf_declaration_order_claim: false" -Description "broad declaration-order boundary"
Assert-Contains -Text $text -Pattern "arbitrary_idf_declaration_order_claim: false" -Description "arbitrary declaration-order boundary"
Assert-Contains -Text $text -Pattern "occurrence_order_policy: epjson-canonical-construction-name-then-layer-order-exact" -Description "bounded occurrence-order policy"
Assert-Contains -Text $text -Pattern "tolerance_policy: energyplus-26.1-round-sig-digits-3-normalized-exact" -Description "source-format thickness policy"
Assert-Contains -Text $text -Pattern "material_occurrences: 7" -Description "Rust gap occurrence count"
Assert-Contains -Text $text -Pattern "oracle_material_rows: 7" -Description "oracle gap occurrence count"
Assert-Contains -Text $text -Pattern "gas_type: Custom/Custom" -Description "Custom gas occurrence"
Assert-Contains -Text $text -Pattern "gas_type: Krypton/Krypton" -Description "Krypton gas occurrence"
Assert-Contains -Text $text -Pattern "gas_type: Xenon/Xenon" -Description "Xenon gas occurrence"
Assert-Contains -Text $text -Pattern "gap_vent_type: VentedIndoor/VentedIndoor" -Description "indoor vent occurrence"
Assert-Contains -Text $text -Pattern "gap_vent_type: VentedOutdoor/VentedOutdoor" -Description "outdoor vent occurrence"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

$reportLines = @(
    "# Window material gap equivalent-layer smoke report",
    "",
    "- Case: window_material_gap_equivalent_layer_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Window runtime claim: false",
    "- Fenestration surface claim: false",
    "- Equivalent-layer construction claim: false",
    "- Construction rating claim: false",
    "- Broad IDF declaration-order claim: false",
    "- Arbitrary IDF declaration-order claim: false",
    "",
    "## Exact oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $constructionHeader,
    $gapHeader
) + $orderedRows + @(
    $hostSurfaceRow,
    $windowSurfaceRow,
    "~~~",
    "",
    "The seven gap rows are construction-layer occurrences in this fixture's canonical construction-name order and layer order. C ARGON SEALED EQL GAP occurs twice. The valid but unused DISTINCTIVE UNUSED AIR SEALED EQL GAP definition is intentionally absent from EIO.",
    "Gap definition order and material-name order intentionally differ from the occurrence order. Broader or arbitrary IDF construction declaration-order parity remains unclaimed because the Rust bridge has no IDF declaration-order overlay.",
    "The construction and heat-transfer-surface rows are oracle-only fixture-integrity locks; ratings, layer-count semantics, ASHWAT behavior, and surface behavior are not Rust parity claims.",
    "",
    "## Bounded typed-input comparison",
    "",
    "~~~text",
    $text,
    "~~~",
    "",
    "The Custom gas-property coefficients are typed-only because EnergyPlus EIO exposes only the occurrence material name, gas type, serialized thickness, and gap vent type.",
    "",
    "This report is non-blocking diagnostic-only static material evidence. It makes no equivalent-layer construction, ASHWAT, fenestration runtime, rating, or conformance claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "WindowMaterial:Gap:EquivalentLayer report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window material gap equivalent-layer smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Equivalent-layer construction claim: false" -Description "report construction boundary"
Assert-Contains -Text $reportText -Pattern "Broad IDF declaration-order claim: false" -Description "report declaration-order boundary"
Assert-Contains -Text $reportText -Pattern "tolerance_policy: energyplus-26.1-round-sig-digits-3-normalized-exact" -Description "report source-format thickness policy"
Assert-Contains -Text $reportText -Pattern ($orderedRows -join [Environment]::NewLine) -Description "report exact ordered construction and gap rows"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
Assert-Contains -Text $reportText -Pattern "valid but unused DISTINCTIVE UNUSED AIR SEALED EQL GAP" -Description "report unused-definition boundary"
Assert-Contains -Text $reportText -Pattern "oracle-only fixture-integrity locks" -Description "report oracle-only boundary"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"

Write-Host "WindowMaterial:Gap:EquivalentLayer comparison smoke passed."
Write-Host "Diagnostic-only, nonblocking evidence; no construction, window runtime, rating, or conformance claim."
Write-Host "Report: $ReportPath"
