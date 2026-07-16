[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-shade-equivalent-layer\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_shade_equivalent_layer_001"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_shade_equivalent_layer_001\window_material_shade_equivalent_layer.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:Shade:EquivalentLayer fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "window-material-shade-equivalent-layer.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus WindowMaterial:Shade:EquivalentLayer oracle case."
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

$materialDetailsHeader = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible"
$constructionHeader = "! <Construction:WindowEquivalentLayer>,Construction Name,Index,#Layers,U-factor {W/m2-K},SHGC, Solar Transmittance at Normal Incidence"
$shadeHeader = "! <WindowMaterial:Shade:EquivalentLayer>, Material Name, Front Side Beam-Beam Solar Transmittance, Back Side Beam-Beam Solar Transmittance, Front Side Beam-Diffuse Solar Transmittance, Back Side Beam-Diffuse Solar Transmittance, Front Side Beam-Diffuse Solar Reflectance, Back Side Beam-Diffuse Solar Reflectance, Infrared Transmittance, Front Side Infrared Emissivity, Back Side Infrared Emissivity"
$genericRows = @(
    "Material Details,Z HIGH PRECISION REUSED EQL SHADE,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,M DEFAULTED UNUSED EQL SHADE,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,A DEFAULTED USED EQL SHADE,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000"
)
$genericPrefixes = @(
    "Material Details,Z HIGH PRECISION REUSED EQL SHADE,",
    "Material Details,M DEFAULTED UNUSED EQL SHADE,",
    "Material Details,A DEFAULTED USED EQL SHADE,"
)
$orderedOccurrenceRows = @(
    "Construction:WindowEquivalentLayer,A DEFAULTED EQL SHADE WINDOW CONSTRUCTION,2,3,1.904,0.501,8.060E-002",
    "WindowMaterial:Shade:EquivalentLayer,A DEFAULTED USED EQL SHADE,0.0000,0.0000,0.1111,0.1222,0.2333,0.2444,5.0000E-002,0.9100,0.9100",
    "Construction:WindowEquivalentLayer,B HIGH PRECISION FIRST EQL SHADE WINDOW CONSTRUCTION,3,3,1.680,0.434,9.168E-002",
    "WindowMaterial:Shade:EquivalentLayer,Z HIGH PRECISION REUSED EQL SHADE,1.2346E-005,1.2346E-005,0.1235,0.2346,0.3457,0.4568,3.4568E-005,0.7654,0.6543",
    "Construction:WindowEquivalentLayer,C HIGH PRECISION SECOND EQL SHADE WINDOW CONSTRUCTION,4,3,1.680,0.434,9.168E-002",
    "WindowMaterial:Shade:EquivalentLayer,Z HIGH PRECISION REUSED EQL SHADE,1.2346E-005,1.2346E-005,0.1235,0.2346,0.3457,0.4568,3.4568E-005,0.7654,0.6543"
)
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE EQL SHADE WINDOW HOST WALL,Wall,,CTF - ConductionTransferFunction,D EQL SHADE OPAQUE HOST CONSTRUCTION,3.071,2.104,,9.60,12.00,9.60,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$defaultedWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE DEFAULTED EQL SHADE TEST WINDOW,Window,DISTINCTIVE EQL SHADE WINDOW HOST WALL,Window5 Detailed Fenestration,A DEFAULTED EQL SHADE WINDOW CONSTRUCTION,N/A,1.904,No,0.80,0.80,0.80,180.00,90.00,0.80,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$firstHighPrecisionWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE FIRST HIGH PRECISION EQL SHADE TEST WINDOW,Window,DISTINCTIVE EQL SHADE WINDOW HOST WALL,Window5 Detailed Fenestration,B HIGH PRECISION FIRST EQL SHADE WINDOW CONSTRUCTION,N/A,1.680,No,0.80,0.80,0.80,180.00,90.00,0.80,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$secondHighPrecisionWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE SECOND HIGH PRECISION EQL SHADE TEST WINDOW,Window,DISTINCTIVE EQL SHADE WINDOW HOST WALL,Window5 Detailed Fenestration,B HIGH PRECISION FIRST EQL SHADE WINDOW CONSTRUCTION,N/A,1.680,No,0.80,0.80,0.80,180.00,90.00,0.80,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

$eioLines = @(Get-Content -LiteralPath $eio)
$eioText = $eioLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "generic material-details header"
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes $genericPrefixes -Expected $genericRows -Description "equivalent-layer shade generic definition"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <Construction:WindowEquivalentLayer>," -Expected $constructionHeader -Description "equivalent-layer construction header"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <WindowMaterial:Shade:EquivalentLayer>," -Expected $shadeHeader -Description "specialized equivalent-layer shade header"
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes @("Construction:WindowEquivalentLayer,", "WindowMaterial:Shade:EquivalentLayer,") -Expected $orderedOccurrenceRows -Description "equivalent-layer construction and shade occurrence"
Assert-NotContains -Text $eioText -Pattern "WindowMaterial:Shade:EquivalentLayer,M DEFAULTED UNUSED EQL SHADE," -Description "unused equivalent-layer shade specialized occurrence"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE EQL SHADE WINDOW HOST WALL," -Expected $hostSurfaceRow -Description "opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE DEFAULTED EQL SHADE TEST WINDOW," -Expected $defaultedWindowSurfaceRow -Description "defaulted-shade fenestration heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE FIRST HIGH PRECISION EQL SHADE TEST WINDOW," -Expected $firstHighPrecisionWindowSurfaceRow -Description "first high-precision fenestration heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE SECOND HIGH PRECISION EQL SHADE TEST WINDOW," -Expected $secondHighPrecisionWindowSurfaceRow -Description "second high-precision fenestration heat-transfer surface"
Assert-NotContains -Text $eioText -Pattern "Window5 Detailed Fenestration,C HIGH PRECISION SECOND EQL SHADE WINDOW CONSTRUCTION," -Description "surface reference to deliberately unused second high-precision construction"

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-material-shade-equivalent-layer.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "window-material-shade-equivalent-layer.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-material-shade-equivalent-layer.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing bounded Rust WindowMaterial:Shade:EquivalentLayer inputs with EnergyPlus generic and specialized EIO evidence."
$output = & $cargo.Source run -p ep_cli --quiet -- compare window-material-shade-equivalent-layer $epjson $eio --tolerance exact 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "WindowMaterial:Shade:EquivalentLayer comparison smoke failed."
}

$text = ($output -join [Environment]::NewLine)
Assert-Contains -Text $text -Pattern "Window Material Shade EquivalentLayer Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "window_runtime_claim: false" -Description "window runtime boundary"
Assert-Contains -Text $text -Pattern "window_optics_claim: false" -Description "window optics boundary"
Assert-Contains -Text $text -Pattern "window_thermal_claim: false" -Description "window thermal boundary"
Assert-Contains -Text $text -Pattern "daylighting_claim: false" -Description "daylighting boundary"
Assert-Contains -Text $text -Pattern "equivalent_layer_construction_claim: false" -Description "equivalent-layer construction boundary"
Assert-Contains -Text $text -Pattern "complex_fenestration_construction_claim: false" -Description "complex-fenestration construction boundary"
Assert-Contains -Text $text -Pattern "fenestration_surface_claim: false" -Description "fenestration surface boundary"
Assert-Contains -Text $text -Pattern "construction_rating_claim: false" -Description "construction rating boundary"
Assert-Contains -Text $text -Pattern "visible_input_claim: false" -Description "unreported visible-input boundary"
Assert-Contains -Text $text -Pattern "nominal_resistance_claim: false" -Description "nominal resistance boundary"
Assert-Contains -Text $text -Pattern "broad_idf_declaration_order_claim: false" -Description "broad IDF declaration-order boundary"
Assert-Contains -Text $text -Pattern "arbitrary_idf_declaration_order_claim: false" -Description "arbitrary IDF declaration-order boundary"
Assert-Contains -Text $text -Pattern "tolerance_mode: exact" -Description "explicit exact comparison mode"
Assert-Contains -Text $text -Pattern "tolerance_policy: energyplus-26.1-material-details-zero-exact-shade-equivalent-layer-4R-normalized-exact" -Description "source-format policy"
Assert-Contains -Text $text -Pattern "material_objects: 3" -Description "Rust equivalent-layer shade definition count"
Assert-Contains -Text $text -Pattern "oracle_generic_material_rows: 3" -Description "matched oracle equivalent-layer shade definition count"
Assert-Contains -Text $text -Pattern "oracle_material_detail_rows: 6" -Description "all oracle generic material-detail row count"
Assert-Contains -Text $text -Pattern "shade_equivalent_layer_occurrences: 3" -Description "Rust equivalent-layer shade occurrence count"
Assert-Contains -Text $text -Pattern "oracle_shade_equivalent_layer_occurrence_rows: 3" -Description "oracle equivalent-layer shade occurrence count"
Assert-Contains -Text $text -Pattern "shade_equivalent_layer_header_present: true" -Description "specialized equivalent-layer shade header presence"
Assert-Contains -Text $text -Pattern "constructions_report_requested: true" -Description "source-required constructions report request"
Assert-Contains -Text $text -Pattern "shade_equivalent_layer_header_rows: 1" -Description "specialized equivalent-layer shade header count"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

$reportLines = @(
    "# Window material shade equivalent-layer smoke report",
    "",
    "- Case: window_material_shade_equivalent_layer_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Window runtime/optics/thermal claims: false",
    "- Daylighting claim: false",
    "- Equivalent-layer, complex-fenestration, and construction-rating claims: false",
    "- Fenestration surface claim: false",
    "- Unreported visible-input and nominal-resistance claims: false",
    "- Broad/arbitrary IDF declaration-order claims: false",
    "- Tolerance mode: exact",
    "",
    "## Exact oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader
) + $genericRows + @(
    $constructionHeader,
    $shadeHeader
) + $orderedOccurrenceRows + @(
    $hostSurfaceRow,
    $defaultedWindowSurfaceRow,
    $firstHighPrecisionWindowSurfaceRow,
    $secondHighPrecisionWindowSurfaceRow,
    "~~~",
    "",
    "The generic rows are one-per-definition echoes and include M DEFAULTED UNUSED EQL SHADE. The specialized rows are construction-layer occurrences: A appears once and Z appears once in each of two constructions, while the unused M definition is absent.",
    "The first Z construction is referenced by two surfaces but emits one Z row. The second Z construction is referenced by no surface but still emits one Z row; a fixture-only EMS construction-index variable keeps the oracle warning-free without adding a surface. Together these locks show that surface count does not control occurrence multiplicity and surface-unused constructions still participate.",
    "The visible beam-beam, beam-diffuse transmittance, and beam-diffuse reflectance inputs are not exposed by this specialized table. The construction and heat-transfer-surface rows are oracle-only fixture-integrity locks.",
    "",
    "## Bounded typed-input comparison",
    "",
    "~~~text",
    $text,
    "~~~",
    "",
    "The comparison checks all three typed definitions against exact generic zeros and the three equivalent-layer construction occurrences against EnergyPlus 26.1 {:.4R} fields in explicit exact mode.",
    "",
    "This report is non-blocking diagnostic-only static material evidence. It makes no window runtime, optics, thermal, daylighting, construction-rating, fenestration-surface, declaration-order, or conformance claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "WindowMaterial:Shade:EquivalentLayer report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window material shade equivalent-layer smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Tolerance mode: exact" -Description "report exact tolerance mode"
Assert-Contains -Text $reportText -Pattern ($genericRows -join [Environment]::NewLine) -Description "report exact generic equivalent-layer shade rows"
Assert-Contains -Text $reportText -Pattern ($orderedOccurrenceRows -join [Environment]::NewLine) -Description "report exact construction and shade occurrence rows"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
Assert-Contains -Text $reportText -Pattern "surface count does not control occurrence multiplicity" -Description "report nonvacuous occurrence proof"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"

Write-Host "WindowMaterial:Shade:EquivalentLayer comparison smoke passed."
Write-Host "Diagnostic-only, nonblocking evidence; no window runtime, optics, thermal, construction, surface, declaration-order, or conformance claim."
Write-Host "Report: $ReportPath"
