[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-drape-equivalent-layer\26.1.0"
$MaterialsOnlyOutputRoot = Join-Path $OutputRoot "materials-only"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_drape_equivalent_layer_001"
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

function Assert-CommaSeparatedTokenCount {
    param(
        [Parameter(Mandatory = $true)][string]$Row,
        [Parameter(Mandatory = $true)][int]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $actual = $Row.Split([char]',').Count
    if ($actual -ne $Expected) {
        throw "Expected $Expected comma-separated tokens for $Description; found $actual in: $Row"
    }
    Write-Host "OK $Description token count: $actual"
}

function Assert-CleanOracleLog {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "EnergyPlus did not produce required $Description log: $Path"
    }
    $text = Get-Content -LiteralPath $Path -Raw
    Assert-Contains -Text $text -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "$Description clean oracle completion"
    if ($text -match "(?m)^\s*\*\* (?:Warning|Severe) \*\*") {
        Write-Host $text
        throw "EnergyPlus emitted a warning or severe diagnostic in the $Description run despite the required clean summary."
    }
}

function Invoke-DrapeComparison {
    param(
        [Parameter(Mandatory = $true)][string]$CargoPath,
        [Parameter(Mandatory = $true)][string]$EpJsonPath,
        [Parameter(Mandatory = $true)][string]$EioPath,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Write-Host "Comparing bounded Rust WindowMaterial:Drape:EquivalentLayer inputs with $Description EnergyPlus EIO evidence."
    $output = & $CargoPath run -p ep_cli --quiet -- compare window-material-drape-equivalent-layer $EpJsonPath $EioPath --tolerance exact 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "WindowMaterial:Drape:EquivalentLayer $Description comparison smoke failed."
    }
    return ($output -join [Environment]::NewLine)
}

function Assert-ComparisonBoundaries {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-Contains -Text $Text -Pattern "Window Material Drape EquivalentLayer Comparison" -Description "$Description comparison header"
    Assert-Contains -Text $Text -Pattern "comparison_class: smoke" -Description "$Description comparison class"
    Assert-Contains -Text $Text -Pattern "conformance_claim: false" -Description "$Description conformance boundary"
    Assert-Contains -Text $Text -Pattern "window_runtime_claim: false" -Description "$Description window runtime boundary"
    Assert-Contains -Text $Text -Pattern "window_optics_claim: false" -Description "$Description window optics boundary"
    Assert-Contains -Text $Text -Pattern "window_thermal_claim: false" -Description "$Description window thermal boundary"
    Assert-Contains -Text $Text -Pattern "daylighting_claim: false" -Description "$Description daylighting boundary"
    Assert-Contains -Text $Text -Pattern "equivalent_layer_construction_claim: false" -Description "$Description equivalent-layer construction boundary"
    Assert-Contains -Text $Text -Pattern "complex_fenestration_construction_claim: false" -Description "$Description complex-fenestration construction boundary"
    Assert-Contains -Text $Text -Pattern "fenestration_surface_claim: false" -Description "$Description fenestration surface boundary"
    Assert-Contains -Text $Text -Pattern "construction_rating_claim: false" -Description "$Description construction rating boundary"
    Assert-Contains -Text $Text -Pattern "visible_input_claim: false" -Description "$Description unreported visible-input boundary"
    Assert-Contains -Text $Text -Pattern "nominal_resistance_claim: false" -Description "$Description nominal resistance boundary"
    Assert-Contains -Text $Text -Pattern "broad_idf_declaration_order_claim: false" -Description "$Description broad IDF declaration-order boundary"
    Assert-Contains -Text $Text -Pattern "arbitrary_idf_declaration_order_claim: false" -Description "$Description arbitrary IDF declaration-order boundary"
    Assert-Contains -Text $Text -Pattern "tolerance_mode: exact" -Description "$Description explicit exact comparison mode"
    Assert-Contains -Text $Text -Pattern "tolerance_policy: energyplus-26.1-material-details-zero-exact-drape-equivalent-layer-4R-pleat-5R-normalized-exact" -Description "$Description source-format policy"
    Assert-Contains -Text $Text -Pattern "material_objects: 5" -Description "$Description Rust equivalent-layer drape definition count"
    Assert-Contains -Text $Text -Pattern "oracle_generic_material_rows: 5" -Description "$Description matched oracle equivalent-layer drape definition count"
    Assert-Contains -Text $Text -Pattern "oracle_material_detail_rows: 8" -Description "$Description all oracle generic material-detail row count"
    Assert-Contains -Text $Text -Pattern "first_divergence: none" -Description "$Description first divergence"
    Assert-Contains -Text $Text -Pattern "status: pass" -Description "$Description comparison status"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_drape_equivalent_layer_001\window_material_drape_equivalent_layer.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:Drape:EquivalentLayer fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "window-material-drape-equivalent-layer.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus WindowMaterial:Drape:EquivalentLayer oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$eio = Join-Path $OutputRoot "eplusout.eio"
$err = Join-Path $OutputRoot "eplusout.err"
if (-not (Test-Path -LiteralPath $eio -PathType Leaf)) {
    throw "EnergyPlus did not produce required primary EIO: $eio"
}
Assert-CleanOracleLog -Path $err -Description "primary Constructions-and-Materials"

$materialDetailsHeader = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible"
$constructionHeader = "! <Construction:WindowEquivalentLayer>,Construction Name,Index,#Layers,U-factor {W/m2-K},SHGC, Solar Transmittance at Normal Incidence"
$drapeHeader = "! <WindowMaterial:Drape:EquivalentLayer>, Material Name, Front Side Beam-Beam Solar Transmittance, Back Side Beam-Beam Solar Transmittance, Front Side Beam-Diffuse Solar Transmittance, Back Side Beam-Diffuse Solar Transmittance, , Front Side Beam-Diffuse Solar Reflectance, Back Side Beam-Diffuse Solar Reflectance, Infrared Transmittance, Front Side Infrared Emissivity, Back Side Infrared Emissivity, Width of Pleated Fabric, Length of Pleated Fabric"
$genericRows = @(
    "Material Details,Z HIGH PRECISION REUSED EQL DRAPE,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,M DEFAULTED UNUSED EQL DRAPE,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,A DEFAULTED USED EQL DRAPE,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,P WIDTH ONLY EXPLICIT ZERO LENGTH EQL DRAPE,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,Q EXPLICIT ZERO WIDTH LENGTH ONLY EQL DRAPE,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000"
)
$genericPrefixes = @(
    "Material Details,Z HIGH PRECISION REUSED EQL DRAPE,",
    "Material Details,M DEFAULTED UNUSED EQL DRAPE,",
    "Material Details,A DEFAULTED USED EQL DRAPE,",
    "Material Details,P WIDTH ONLY EXPLICIT ZERO LENGTH EQL DRAPE,",
    "Material Details,Q EXPLICIT ZERO WIDTH LENGTH ONLY EQL DRAPE,"
)
$orderedOccurrenceRows = @(
    "Construction:WindowEquivalentLayer,A DEFAULTED EQL DRAPE WINDOW CONSTRUCTION,2,3,1.897,-N0AN,0.000",
    "WindowMaterial:Drape:EquivalentLayer,A DEFAULTED USED EQL DRAPE,0.0000,0.1111,0.1222,0.2333,0.2444,5.0000E-002,0.8700,0.8700,0.00000,0.00000",
    "Construction:WindowEquivalentLayer,B HIGH PRECISION FIRST EQL DRAPE WINDOW CONSTRUCTION,3,3,1.728,0.456,8.047E-002",
    "WindowMaterial:Drape:EquivalentLayer,Z HIGH PRECISION REUSED EQL DRAPE,1.2346E-005,0.1235,0.2346,0.3457,0.4568,3.4568E-005,0.7654,0.6543,1.23456E-002,2.34567E-002",
    "Construction:WindowEquivalentLayer,C HIGH PRECISION SECOND EQL DRAPE WINDOW CONSTRUCTION,4,3,1.728,0.456,8.047E-002",
    "WindowMaterial:Drape:EquivalentLayer,Z HIGH PRECISION REUSED EQL DRAPE,1.2346E-005,0.1235,0.2346,0.3457,0.4568,3.4568E-005,0.7654,0.6543,1.23456E-002,2.34567E-002",
    "Construction:WindowEquivalentLayer,D WIDTH ONLY EQL DRAPE WINDOW CONSTRUCTION,5,3,1.897,-N0AN,7.700E-012",
    "WindowMaterial:Drape:EquivalentLayer,P WIDTH ONLY EXPLICIT ZERO LENGTH EQL DRAPE,1.0000E-002,0.1100,0.1200,0.2100,0.2200,4.0000E-002,0.8000,0.7000,0.00000,0.00000",
    "Construction:WindowEquivalentLayer,E LENGTH ONLY EQL DRAPE WINDOW CONSTRUCTION,6,3,1.897,-N0AN,3.090E-010",
    "WindowMaterial:Drape:EquivalentLayer,Q EXPLICIT ZERO WIDTH LENGTH ONLY EQL DRAPE,2.0000E-002,0.1200,0.1300,0.2200,0.2300,5.0000E-002,0.7500,0.6500,0.00000,0.00000"
)
$specializedRows = @(
    $orderedOccurrenceRows[1],
    $orderedOccurrenceRows[3],
    $orderedOccurrenceRows[5],
    $orderedOccurrenceRows[7],
    $orderedOccurrenceRows[9]
)
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE EQL DRAPE WINDOW HOST WALL,Wall,,CTF - ConductionTransferFunction,F EQL DRAPE OPAQUE HOST CONSTRUCTION,3.071,2.104,,10.00,12.00,10.00,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$firstHighPrecisionWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE FIRST HIGH PRECISION EQL DRAPE TEST WINDOW,Window,DISTINCTIVE EQL DRAPE WINDOW HOST WALL,Window5 Detailed Fenestration,B HIGH PRECISION FIRST EQL DRAPE WINDOW CONSTRUCTION,N/A,1.728,No,1.00,1.00,1.00,180.00,90.00,1.00,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$secondHighPrecisionWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE SECOND HIGH PRECISION EQL DRAPE TEST WINDOW,Window,DISTINCTIVE EQL DRAPE WINDOW HOST WALL,Window5 Detailed Fenestration,B HIGH PRECISION FIRST EQL DRAPE WINDOW CONSTRUCTION,N/A,1.728,No,1.00,1.00,1.00,180.00,90.00,1.00,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

$eioLines = @(Get-Content -LiteralPath $eio)
$eioText = $eioLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "generic material-details header"
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes $genericPrefixes -Expected $genericRows -Description "equivalent-layer drape generic definition"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <Construction:WindowEquivalentLayer>," -Expected $constructionHeader -Description "equivalent-layer construction header"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <WindowMaterial:Drape:EquivalentLayer>," -Expected $drapeHeader -Description "specialized equivalent-layer drape header"
Assert-CommaSeparatedTokenCount -Row $drapeHeader -Expected 14 -Description "source-malformed specialized equivalent-layer drape header"
foreach ($row in $specializedRows) {
    Assert-CommaSeparatedTokenCount -Row $row -Expected 12 -Description "specialized equivalent-layer drape data row"
}
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes @("Construction:WindowEquivalentLayer,", "WindowMaterial:Drape:EquivalentLayer,") -Expected $orderedOccurrenceRows -Description "equivalent-layer construction and drape occurrence"
Assert-NotContains -Text $eioText -Pattern "WindowMaterial:Drape:EquivalentLayer,M DEFAULTED UNUSED EQL DRAPE," -Description "unused equivalent-layer drape specialized occurrence"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE EQL DRAPE WINDOW HOST WALL," -Expected $hostSurfaceRow -Description "opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE FIRST HIGH PRECISION EQL DRAPE TEST WINDOW," -Expected $firstHighPrecisionWindowSurfaceRow -Description "first high-precision fenestration heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE SECOND HIGH PRECISION EQL DRAPE TEST WINDOW," -Expected $secondHighPrecisionWindowSurfaceRow -Description "second high-precision fenestration heat-transfer surface"
foreach ($constructionName in @(
    "A DEFAULTED EQL DRAPE WINDOW CONSTRUCTION",
    "C HIGH PRECISION SECOND EQL DRAPE WINDOW CONSTRUCTION",
    "D WIDTH ONLY EQL DRAPE WINDOW CONSTRUCTION",
    "E LENGTH ONLY EQL DRAPE WINDOW CONSTRUCTION"
)) {
    Assert-NotContains -Text $eioText -Pattern "Window5 Detailed Fenestration,$constructionName," -Description "surface reference to deliberately surface-unused $constructionName"
}

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-material-drape-equivalent-layer.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "window-material-drape-equivalent-layer.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-material-drape-equivalent-layer.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$primaryComparison = Invoke-DrapeComparison -CargoPath $cargo.Source -EpJsonPath $epjson -EioPath $eio -Description "primary Constructions-and-Materials"
Assert-ComparisonBoundaries -Text $primaryComparison -Description "primary"
Assert-Contains -Text $primaryComparison -Pattern "drape_equivalent_layer_occurrences: 5" -Description "primary Rust equivalent-layer drape occurrence count"
Assert-Contains -Text $primaryComparison -Pattern "oracle_drape_equivalent_layer_occurrence_rows: 5" -Description "primary oracle equivalent-layer drape occurrence count"
Assert-Contains -Text $primaryComparison -Pattern "drape_equivalent_layer_header_present: true" -Description "primary specialized equivalent-layer drape header presence"
Assert-Contains -Text $primaryComparison -Pattern "constructions_report_requested: true" -Description "primary source-required constructions report request"
Assert-Contains -Text $primaryComparison -Pattern "drape_equivalent_layer_header_rows: 1" -Description "primary specialized equivalent-layer drape header count"

New-Directory -Path $MaterialsOnlyOutputRoot
$materialsOnlyIdf = Join-Path $MaterialsOnlyOutputRoot "window-material-drape-equivalent-layer-materials-only.idf"
$fixtureText = [System.IO.File]::ReadAllText($fixtureIdf)
$materialsOnlyText = $fixtureText.Replace(
    "Output:Constructions,Constructions,Materials;",
    "Output:Constructions,Materials;"
)
if ($materialsOnlyText.Equals($fixtureText, [System.StringComparison]::Ordinal)) {
    throw "Could not derive Materials-only fixture because the exact Output:Constructions selector was not found."
}
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($materialsOnlyIdf, $materialsOnlyText, $utf8WithoutBom)

Write-Host "Running EnergyPlus WindowMaterial:Drape:EquivalentLayer Materials-only oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $MaterialsOnlyOutputRoot, $materialsOnlyIdf)

$materialsOnlyEio = Join-Path $MaterialsOnlyOutputRoot "eplusout.eio"
$materialsOnlyErr = Join-Path $MaterialsOnlyOutputRoot "eplusout.err"
if (-not (Test-Path -LiteralPath $materialsOnlyEio -PathType Leaf)) {
    throw "EnergyPlus did not produce required Materials-only EIO: $materialsOnlyEio"
}
Assert-CleanOracleLog -Path $materialsOnlyErr -Description "Materials-only"

$materialsOnlyEioLines = @(Get-Content -LiteralPath $materialsOnlyEio)
$materialsOnlyEioText = $materialsOnlyEioLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $materialsOnlyEioLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "Materials-only generic material-details header"
Assert-ExactOrderedEioRows -Lines $materialsOnlyEioLines -Prefixes $genericPrefixes -Expected $genericRows -Description "Materials-only equivalent-layer drape generic definition"
Assert-NotContains -Text $materialsOnlyEioText -Pattern "! <Construction:WindowEquivalentLayer>," -Description "Materials-only equivalent-layer construction header"
Assert-NotContains -Text $materialsOnlyEioText -Pattern "Construction:WindowEquivalentLayer," -Description "Materials-only equivalent-layer construction row"
Assert-NotContains -Text $materialsOnlyEioText -Pattern "! <WindowMaterial:Drape:EquivalentLayer>," -Description "Materials-only specialized equivalent-layer drape header"
Assert-NotContains -Text $materialsOnlyEioText -Pattern "WindowMaterial:Drape:EquivalentLayer," -Description "Materials-only specialized equivalent-layer drape occurrence"

Push-Location $MaterialsOnlyOutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-material-drape-equivalent-layer-materials-only.idf")
}
finally {
    Pop-Location
}

$materialsOnlyEpjson = Join-Path $MaterialsOnlyOutputRoot "window-material-drape-equivalent-layer-materials-only.epJSON"
if (-not (Test-Path -LiteralPath $materialsOnlyEpjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-material-drape-equivalent-layer-materials-only.epJSON"
}

$materialsOnlyComparison = Invoke-DrapeComparison -CargoPath $cargo.Source -EpJsonPath $materialsOnlyEpjson -EioPath $materialsOnlyEio -Description "Materials-only"
Assert-ComparisonBoundaries -Text $materialsOnlyComparison -Description "Materials-only"
Assert-Contains -Text $materialsOnlyComparison -Pattern "drape_equivalent_layer_occurrences: 0" -Description "Materials-only suppressed Rust equivalent-layer drape occurrence count"
Assert-Contains -Text $materialsOnlyComparison -Pattern "oracle_drape_equivalent_layer_occurrence_rows: 0" -Description "Materials-only absent oracle equivalent-layer drape occurrence count"
Assert-Contains -Text $materialsOnlyComparison -Pattern "drape_equivalent_layer_header_present: false" -Description "Materials-only specialized equivalent-layer drape header absence"
Assert-Contains -Text $materialsOnlyComparison -Pattern "constructions_report_requested: false" -Description "Materials-only absent constructions report request"
Assert-Contains -Text $materialsOnlyComparison -Pattern "drape_equivalent_layer_header_rows: 0" -Description "Materials-only specialized equivalent-layer drape header count"

$reportLines = @(
    "# Window material drape equivalent-layer smoke report",
    "",
    "- Case: window_material_drape_equivalent_layer_001",
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
    "## Exact primary oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader
) + $genericRows + @(
    $constructionHeader,
    $drapeHeader
) + $orderedOccurrenceRows + @(
    $hostSurfaceRow,
    $firstHighPrecisionWindowSurfaceRow,
    $secondHighPrecisionWindowSurfaceRow,
    "~~~",
    "",
    "The generic rows are one-per-definition echoes and include M DEFAULTED UNUSED EQL DRAPE. The specialized rows are construction-layer occurrences: A, P, and Q each appear once and Z appears once in each of two constructions, while unused M is absent.",
    "Two surfaces share the first Z construction but it emits one Z row. The second Z construction has no surface but still emits one Z row; fixture-only EMS construction-index variables keep all surface-unused constructions warning-free. Surface count therefore does not control occurrence multiplicity.",
    "The literal specialized header has fourteen comma-separated tokens because EnergyPlus emits an empty column, while each data row has twelve. N1 appears only once despite front/back header labels, N6-N8 are omitted, N1-N11 report with {:.4R}, and pleat dimensions report with {:.5R}.",
    "The P and Q rows prove source normalization of either one-zero pleat pair to exact zero/zero output. Construction and heat-transfer-surface rows are oracle-only fixture-integrity locks.",
    "",
    "## Primary bounded typed-input comparison",
    "",
    "~~~text",
    $primaryComparison,
    "~~~",
    "",
    "## Materials-only reporting boundary",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader
) + $genericRows + @(
    "No Construction:WindowEquivalentLayer or WindowMaterial:Drape:EquivalentLayer header/data rows were emitted.",
    "~~~",
    "",
    "~~~text",
    $materialsOnlyComparison,
    "~~~",
    "",
    "The Materials-only comparison retains all five generic definitions and requires zero specialized headers and occurrences.",
    "",
    "This report is non-blocking diagnostic-only static material evidence. It makes no window runtime, optics, thermal, daylighting, construction-rating, fenestration-surface, declaration-order, or conformance claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "WindowMaterial:Drape:EquivalentLayer report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window material drape equivalent-layer smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Tolerance mode: exact" -Description "report exact tolerance mode"
Assert-Contains -Text $reportText -Pattern ($genericRows -join [Environment]::NewLine) -Description "report exact generic equivalent-layer drape rows"
Assert-Contains -Text $reportText -Pattern ($orderedOccurrenceRows -join [Environment]::NewLine) -Description "report exact construction and drape occurrence rows"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
Assert-Contains -Text $reportText -Pattern "Surface count therefore does not control occurrence multiplicity" -Description "report nonvacuous occurrence proof"
Assert-Contains -Text $reportText -Pattern "requires zero specialized headers and occurrences" -Description "report Materials-only boundary"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"

Write-Host "WindowMaterial:Drape:EquivalentLayer comparison smoke passed."
Write-Host "Diagnostic-only, nonblocking evidence; no window runtime, optics, thermal, construction, surface, declaration-order, or conformance claim."
Write-Host "Report: $ReportPath"
