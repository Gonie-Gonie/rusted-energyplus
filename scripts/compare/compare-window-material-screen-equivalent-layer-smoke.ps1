[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..\..")).Path
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-screen-equivalent-layer\26.1.0"
$MaterialsOnlyOutputRoot = Join-Path $OutputRoot "materials-only"
$ConstructionsOnlyOutputRoot = Join-Path $OutputRoot "constructions-only"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_screen_equivalent_layer_001"
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

    & $FilePath @Arguments | Out-Host
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

function Assert-CleanOracleArtifacts {
    param(
        [Parameter(Mandatory = $true)][string]$OutputDirectory,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $errPath = Join-Path $OutputDirectory "eplusout.err"
    $endPath = Join-Path $OutputDirectory "eplusout.end"
    foreach ($path in @($errPath, $endPath)) {
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "EnergyPlus did not produce required $Description oracle artifact: $path"
        }
    }

    $cleanSummary = "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;"
    $errText = Get-Content -LiteralPath $errPath -Raw
    $endText = Get-Content -LiteralPath $endPath -Raw
    Assert-Contains -Text $errText -Pattern $cleanSummary -Description "$Description clean ERR summary"
    Assert-Contains -Text $endText -Pattern $cleanSummary -Description "$Description clean END summary"
    if ($errText -match "(?m)^\s*\*\* (?:Warning|Severe) \*\*") {
        Write-Host $errText
        throw "EnergyPlus emitted a warning or severe diagnostic in the $Description run."
    }
}

function Invoke-OracleCase {
    param(
        [Parameter(Mandatory = $true)][string]$Description,
        [Parameter(Mandatory = $true)][string]$OutputDirectory,
        [Parameter(Mandatory = $true)][string]$IdfName,
        [Parameter(Mandatory = $true)][string]$Selector
    )

    New-Directory -Path $OutputDirectory
    $idfPath = Join-Path $OutputDirectory $IdfName
    if ($Selector -eq "Constructions,Materials") {
        Copy-Item -LiteralPath $fixtureIdf -Destination $idfPath -Force
    }
    else {
        $fixtureText = [System.IO.File]::ReadAllText($fixtureIdf)
        $derivedText = $fixtureText.Replace(
            "Output:Constructions,Constructions,Materials;",
            "Output:Constructions,$Selector;"
        )
        if ($derivedText.Equals($fixtureText, [System.StringComparison]::Ordinal)) {
            throw "Could not derive $Description fixture because the exact Output:Constructions selector was not found."
        }
        $utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText($idfPath, $derivedText, $utf8WithoutBom)
    }

    Write-Host "Running EnergyPlus WindowMaterial:Screen:EquivalentLayer $Description oracle case."
    Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputDirectory, $idfPath)

    $eioPath = Join-Path $OutputDirectory "eplusout.eio"
    if (-not (Test-Path -LiteralPath $eioPath -PathType Leaf)) {
        throw "EnergyPlus did not produce required $Description EIO: $eioPath"
    }
    Assert-CleanOracleArtifacts -OutputDirectory $OutputDirectory -Description $Description

    Push-Location $OutputDirectory
    try {
        Invoke-External -FilePath $converter -Arguments @($IdfName)
    }
    finally {
        Pop-Location
    }

    $epJsonName = [System.IO.Path]::GetFileNameWithoutExtension($IdfName) + ".epJSON"
    $epJsonPath = Join-Path $OutputDirectory $epJsonName
    if (-not (Test-Path -LiteralPath $epJsonPath -PathType Leaf)) {
        throw "ConvertInputFormat did not produce $epJsonName"
    }

    return [pscustomobject]@{
        Eio = $eioPath
        EpJson = $epJsonPath
    }
}

function Invoke-ScreenEquivalentLayerComparison {
    param(
        [Parameter(Mandatory = $true)][string]$CargoPath,
        [Parameter(Mandatory = $true)][string]$EpJsonPath,
        [Parameter(Mandatory = $true)][string]$EioPath,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Write-Host "Comparing bounded Rust WindowMaterial:Screen:EquivalentLayer inputs with $Description EnergyPlus EIO evidence."
    $output = & $CargoPath run -p ep_cli --quiet -- compare window-material-screen-equivalent-layer $EpJsonPath $EioPath --tolerance exact 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "WindowMaterial:Screen:EquivalentLayer $Description comparison smoke failed."
    }
    return ($output -join [Environment]::NewLine)
}

function Assert-ComparisonBoundaries {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-Contains -Text $Text -Pattern "Window Material Screen EquivalentLayer Comparison" -Description "$Description comparison header"
    Assert-Contains -Text $Text -Pattern "case_id: window_material_screen_equivalent_layer_001" -Description "$Description comparison case identity"
    Assert-Contains -Text $Text -Pattern "comparison_class: smoke" -Description "$Description comparison class"
    Assert-Contains -Text $Text -Pattern "conformance_claim: false" -Description "$Description conformance boundary"
    Assert-Contains -Text $Text -Pattern "window_runtime_claim: false" -Description "$Description window runtime boundary"
    Assert-Contains -Text $Text -Pattern "window_optics_claim: false" -Description "$Description window optics boundary"
    Assert-Contains -Text $Text -Pattern "window_thermal_claim: false" -Description "$Description window thermal boundary"
    Assert-Contains -Text $Text -Pattern "daylighting_claim: false" -Description "$Description daylighting boundary"
    Assert-Contains -Text $Text -Pattern "equivalent_layer_construction_claim: false" -Description "$Description equivalent-layer construction boundary"
    Assert-Contains -Text $Text -Pattern "equivalent_layer_construction_typing_claim: false" -Description "$Description equivalent-layer construction typing boundary"
    Assert-Contains -Text $Text -Pattern "complex_fenestration_construction_claim: false" -Description "$Description complex-fenestration boundary"
    Assert-Contains -Text $Text -Pattern "fenestration_surface_claim: false" -Description "$Description fenestration surface boundary"
    Assert-Contains -Text $Text -Pattern "construction_rating_claim: false" -Description "$Description construction rating boundary"
    Assert-Contains -Text $Text -Pattern "visible_input_claim: false" -Description "$Description unreported visible-input boundary"
    Assert-Contains -Text $Text -Pattern "nominal_resistance_claim: false" -Description "$Description nominal resistance boundary"
    Assert-Contains -Text $Text -Pattern "occurrence_bridge: fixture-declared-raw-construction-window-equivalent-layer-metadata" -Description "$Description bounded occurrence bridge"
    Assert-Contains -Text $Text -Pattern "occurrence_order_policy: epjson-canonical-construction-name-then-layer-order-exact" -Description "$Description occurrence order policy"
    Assert-Contains -Text $Text -Pattern "tolerance_mode: exact" -Description "$Description explicit exact mode"
    Assert-Contains -Text $Text -Pattern "tolerance_policy: energyplus-26.1-material-details-zero-exact-screen-equivalent-layer-4R-geometry-5R-auto-sentinel-normalized-exact" -Description "$Description source-format policy"
    Assert-Contains -Text $Text -Pattern "material_objects: 3" -Description "$Description typed material count"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_screen_equivalent_layer_001\window_material_screen_equivalent_layer.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:Screen:EquivalentLayer fixture: $fixtureIdf"
}

Remove-RepoDirectory -Path $OutputRoot
Remove-RepoDirectory -Path $ReportRoot
New-Directory -Path $OutputRoot
New-Directory -Path $ReportRoot

$primary = Invoke-OracleCase -Description "primary Constructions-and-Materials" -OutputDirectory $OutputRoot -IdfName "window-material-screen-equivalent-layer.idf" -Selector "Constructions,Materials"
$materialsOnly = Invoke-OracleCase -Description "Materials-only" -OutputDirectory $MaterialsOnlyOutputRoot -IdfName "window-material-screen-equivalent-layer-materials-only.idf" -Selector "Materials"
$constructionsOnly = Invoke-OracleCase -Description "Constructions-only" -OutputDirectory $ConstructionsOnlyOutputRoot -IdfName "window-material-screen-equivalent-layer-constructions-only.idf" -Selector "Constructions"
$expectedPrimaryEpJson = Join-Path $OutputRoot "window-material-screen-equivalent-layer.epJSON"
if (-not $primary.EpJson.Equals($expectedPrimaryEpJson, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "Primary converted epJSON was not retained at the OutputRoot: $($primary.EpJson)"
}

$materialDetailsHeader = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible"
$constructionHeader = "! <Construction:WindowEquivalentLayer>,Construction Name,Index,#Layers,U-factor {W/m2-K},SHGC, Solar Transmittance at Normal Incidence"
$screenHeader = "! <WindowMaterial:Screen:EquivalentLayer>, Material Name, Screen Beam-Beam Solar Transmittance, Screen Beam-Diffuse Solar Transmittance, Screen Beam-Diffuse Solar Reflectance, Screen Infrared Transmittance, Screen Infrared Emissivity, Screen Wire Spacing, Screen Wire Diameter"
$genericRows = @(
    "Material Details,Z HIGH PRECISION REUSED EQL SCREEN,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,M UNUSED EQL SCREEN,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,A DEFAULT AUTO BLANK GEOMETRY EQL SCREEN,0.0000,MediumRough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000"
)
$genericPrefixes = @(
    "Material Details,Z HIGH PRECISION REUSED EQL SCREEN,",
    "Material Details,M UNUSED EQL SCREEN,",
    "Material Details,A DEFAULT AUTO BLANK GEOMETRY EQL SCREEN,"
)
$defaultScreenRow = "WindowMaterial:Screen:EquivalentLayer,A DEFAULT AUTO BLANK GEOMETRY EQL SCREEN,-99999.0000,0.1111,0.1111,0.1222,0.1222,2.0000E-002,0.9300,0.9300,0.00000,0.00000"
$highPrecisionScreenRow = "WindowMaterial:Screen:EquivalentLayer,Z HIGH PRECISION REUSED EQL SCREEN,0.7270,1.2346E-002,1.2346E-002,0.1235,0.1235,2.3457E-002,0.7654,0.7654,2.34567E-002,3.45678E-003"
$orderedRows = @(
    "Construction:WindowEquivalentLayer,A DEFAULT AUTO EQL SCREEN WINDOW CONSTRUCTION,2,2,2.832,0.230,8.605E-002",
    $defaultScreenRow,
    "Construction:WindowEquivalentLayer,B HIGH PRECISION FIRST EQL SCREEN WINDOW CONSTRUCTION,3,2,3.008,0.670,0.619",
    $highPrecisionScreenRow,
    "Construction:WindowEquivalentLayer,C HIGH PRECISION SECOND EQL SCREEN WINDOW CONSTRUCTION,4,2,3.008,0.670,0.619",
    $highPrecisionScreenRow
)
$orderedPrefixes = @(
    "Construction:WindowEquivalentLayer,A DEFAULT AUTO EQL SCREEN WINDOW CONSTRUCTION,",
    "Construction:WindowEquivalentLayer,B HIGH PRECISION FIRST EQL SCREEN WINDOW CONSTRUCTION,",
    "Construction:WindowEquivalentLayer,C HIGH PRECISION SECOND EQL SCREEN WINDOW CONSTRUCTION,",
    "WindowMaterial:Screen:EquivalentLayer,"
)
$firstHighPrecisionSurfaceRow = "HeatTransfer Surface,DISTINCTIVE FIRST HIGH PRECISION EQL SCREEN TEST WINDOW,Window,DISTINCTIVE EQL SCREEN WINDOW HOST WALL,Window5 Detailed Fenestration,B HIGH PRECISION FIRST EQL SCREEN WINDOW CONSTRUCTION,N/A,3.008,No,1.00,1.00,1.00,180.00,90.00,1.00,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$secondHighPrecisionSurfaceRow = "HeatTransfer Surface,DISTINCTIVE SECOND HIGH PRECISION EQL SCREEN TEST WINDOW,Window,DISTINCTIVE EQL SCREEN WINDOW HOST WALL,Window5 Detailed Fenestration,B HIGH PRECISION FIRST EQL SCREEN WINDOW CONSTRUCTION,N/A,3.008,No,1.00,1.00,1.00,180.00,90.00,1.00,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

$primaryLines = @(Get-Content -LiteralPath $primary.Eio)
$primaryText = $primaryLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "primary generic material-details header"
Assert-ExactOrderedEioRows -Lines $primaryLines -Prefixes $genericPrefixes -Expected $genericRows -Description "primary Screen:EquivalentLayer generic definition Z,M,A"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "! <Construction:WindowEquivalentLayer>," -Expected $constructionHeader -Description "primary equivalent-layer construction header"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "! <WindowMaterial:Screen:EquivalentLayer>," -Expected $screenHeader -Description "primary specialized Screen:EquivalentLayer header"
Assert-CommaSeparatedTokenCount -Row $screenHeader -Expected 9 -Description "primary malformed specialized header"
foreach ($row in @($defaultScreenRow, $highPrecisionScreenRow)) {
    Assert-CommaSeparatedTokenCount -Row $row -Expected 12 -Description "primary specialized data row"
}
Assert-ExactOrderedEioRows -Lines $primaryLines -Prefixes $orderedPrefixes -Expected $orderedRows -Description "primary construction and Screen:EquivalentLayer occurrence A,Z,Z"
Assert-NotContains -Text $primaryText -Pattern "WindowMaterial:Screen:EquivalentLayer,M UNUSED EQL SCREEN," -Description "primary unused specialized occurrence"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "HeatTransfer Surface,DISTINCTIVE FIRST HIGH PRECISION EQL SCREEN TEST WINDOW," -Expected $firstHighPrecisionSurfaceRow -Description "primary first high-precision host window"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "HeatTransfer Surface,DISTINCTIVE SECOND HIGH PRECISION EQL SCREEN TEST WINDOW," -Expected $secondHighPrecisionSurfaceRow -Description "primary second high-precision host window"
foreach ($constructionName in @(
    "A DEFAULT AUTO EQL SCREEN WINDOW CONSTRUCTION",
    "C HIGH PRECISION SECOND EQL SCREEN WINDOW CONSTRUCTION"
)) {
    Assert-NotContains -Text $primaryText -Pattern "Window5 Detailed Fenestration,$constructionName," -Description "surface reference to deliberately surface-unused $constructionName"
}

$materialsOnlyLines = @(Get-Content -LiteralPath $materialsOnly.Eio)
$materialsOnlyText = $materialsOnlyLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $materialsOnlyLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "Materials-only generic material-details header"
Assert-ExactOrderedEioRows -Lines $materialsOnlyLines -Prefixes $genericPrefixes -Expected $genericRows -Description "Materials-only Screen:EquivalentLayer generic definition Z,M,A"
Assert-NotContains -Text $materialsOnlyText -Pattern "! <Construction:WindowEquivalentLayer>," -Description "Materials-only equivalent-layer construction header"
Assert-NotContains -Text $materialsOnlyText -Pattern "Construction:WindowEquivalentLayer," -Description "Materials-only equivalent-layer construction row"
Assert-NotContains -Text $materialsOnlyText -Pattern "! <WindowMaterial:Screen:EquivalentLayer>," -Description "Materials-only specialized Screen:EquivalentLayer header"
Assert-NotContains -Text $materialsOnlyText -Pattern "WindowMaterial:Screen:EquivalentLayer," -Description "Materials-only specialized Screen:EquivalentLayer occurrence"

$constructionsOnlyLines = @(Get-Content -LiteralPath $constructionsOnly.Eio)
$constructionsOnlyText = $constructionsOnlyLines -join [Environment]::NewLine
Assert-NotContains -Text $constructionsOnlyText -Pattern "! <Material Details>," -Description "Constructions-only generic material-details header"
foreach ($prefix in $genericPrefixes) {
    Assert-NotContains -Text $constructionsOnlyText -Pattern $prefix -Description "Constructions-only generic Screen:EquivalentLayer definition"
}
Assert-UniqueExactEioRow -Lines $constructionsOnlyLines -Prefix "! <Construction:WindowEquivalentLayer>," -Expected $constructionHeader -Description "Constructions-only equivalent-layer construction header"
Assert-UniqueExactEioRow -Lines $constructionsOnlyLines -Prefix "! <WindowMaterial:Screen:EquivalentLayer>," -Expected $screenHeader -Description "Constructions-only specialized Screen:EquivalentLayer header"
Assert-ExactOrderedEioRows -Lines $constructionsOnlyLines -Prefixes $orderedPrefixes -Expected $orderedRows -Description "Constructions-only construction and Screen:EquivalentLayer occurrence A,Z,Z"
Assert-NotContains -Text $constructionsOnlyText -Pattern "WindowMaterial:Screen:EquivalentLayer,M UNUSED EQL SCREEN," -Description "Constructions-only unused specialized occurrence"

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$primaryComparison = Invoke-ScreenEquivalentLayerComparison -CargoPath $cargo.Source -EpJsonPath $primary.EpJson -EioPath $primary.Eio -Description "primary Constructions-and-Materials"
$materialsOnlyComparison = Invoke-ScreenEquivalentLayerComparison -CargoPath $cargo.Source -EpJsonPath $materialsOnly.EpJson -EioPath $materialsOnly.Eio -Description "Materials-only"
$constructionsOnlyComparison = Invoke-ScreenEquivalentLayerComparison -CargoPath $cargo.Source -EpJsonPath $constructionsOnly.EpJson -EioPath $constructionsOnly.Eio -Description "Constructions-only"

Assert-ComparisonBoundaries -Text $primaryComparison -Description "primary"
Assert-Contains -Text $primaryComparison -Pattern "oracle_generic_material_rows: 3" -Description "primary generic Screen:EquivalentLayer rows"
Assert-Contains -Text $primaryComparison -Pattern "oracle_material_detail_rows: 6" -Description "primary all generic material rows"
Assert-Contains -Text $primaryComparison -Pattern "screen_equivalent_layer_occurrences: 3" -Description "primary typed Screen:EquivalentLayer occurrences"
Assert-Contains -Text $primaryComparison -Pattern "oracle_screen_equivalent_layer_occurrence_rows: 3" -Description "primary oracle Screen:EquivalentLayer occurrences"
Assert-Contains -Text $primaryComparison -Pattern "screen_equivalent_layer_header_present: true" -Description "primary specialized header presence"
Assert-Contains -Text $primaryComparison -Pattern "constructions_report_requested: true" -Description "primary Constructions selector"
Assert-Contains -Text $primaryComparison -Pattern "materials_report_requested: true" -Description "primary Materials selector"
Assert-Contains -Text $primaryComparison -Pattern "screen_equivalent_layer_header_rows: 1" -Description "primary specialized header count"
Assert-Contains -Text $primaryComparison -Pattern "definition: 1 material: A DEFAULT AUTO BLANK GEOMETRY EQL SCREEN oracle_matches: 1 generic_fixed_zero_fields: 8 status: pass" -Description "primary canonical A definition"
Assert-Contains -Text $primaryComparison -Pattern "definition: 2 material: M UNUSED EQL SCREEN oracle_matches: 1 generic_fixed_zero_fields: 8 status: pass" -Description "primary canonical M definition"
Assert-Contains -Text $primaryComparison -Pattern "definition: 3 material: Z HIGH PRECISION REUSED EQL SCREEN oracle_matches: 1 generic_fixed_zero_fields: 8 status: pass" -Description "primary canonical Z definition"
Assert-Contains -Text $primaryComparison -Pattern "occurrence: 1 construction: A DEFAULT AUTO EQL SCREEN WINDOW CONSTRUCTION layer: 1 material: A DEFAULT AUTO BLANK GEOMETRY EQL SCREEN status: pass" -Description "primary A occurrence"
Assert-Contains -Text $primaryComparison -Pattern "occurrence: 2 construction: B HIGH PRECISION FIRST EQL SCREEN WINDOW CONSTRUCTION layer: 1 material: Z HIGH PRECISION REUSED EQL SCREEN status: pass" -Description "primary first Z occurrence"
Assert-Contains -Text $primaryComparison -Pattern "occurrence: 3 construction: C HIGH PRECISION SECOND EQL SCREEN WINDOW CONSTRUCTION layer: 1 material: Z HIGH PRECISION REUSED EQL SCREEN status: pass" -Description "primary second Z occurrence"

Assert-ComparisonBoundaries -Text $materialsOnlyComparison -Description "Materials-only"
Assert-Contains -Text $materialsOnlyComparison -Pattern "oracle_generic_material_rows: 3" -Description "Materials-only generic Screen:EquivalentLayer rows"
Assert-Contains -Text $materialsOnlyComparison -Pattern "oracle_material_detail_rows: 6" -Description "Materials-only all generic material rows"
Assert-Contains -Text $materialsOnlyComparison -Pattern "screen_equivalent_layer_occurrences: 0" -Description "Materials-only suppressed typed occurrences"
Assert-Contains -Text $materialsOnlyComparison -Pattern "oracle_screen_equivalent_layer_occurrence_rows: 0" -Description "Materials-only absent oracle occurrences"
Assert-Contains -Text $materialsOnlyComparison -Pattern "screen_equivalent_layer_header_present: false" -Description "Materials-only specialized header absence"
Assert-Contains -Text $materialsOnlyComparison -Pattern "constructions_report_requested: false" -Description "Materials-only absent Constructions selector"
Assert-Contains -Text $materialsOnlyComparison -Pattern "materials_report_requested: true" -Description "Materials-only Materials selector"
Assert-Contains -Text $materialsOnlyComparison -Pattern "screen_equivalent_layer_header_rows: 0" -Description "Materials-only specialized header count"
Assert-NotContains -Text $materialsOnlyComparison -Pattern "occurrence: 1 construction:" -Description "Materials-only rendered occurrence"

Assert-ComparisonBoundaries -Text $constructionsOnlyComparison -Description "Constructions-only"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "oracle_generic_material_rows: 0" -Description "Constructions-only generic Screen:EquivalentLayer rows"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "oracle_material_detail_rows: 0" -Description "Constructions-only all generic material rows"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "screen_equivalent_layer_occurrences: 3" -Description "Constructions-only typed occurrences"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "oracle_screen_equivalent_layer_occurrence_rows: 3" -Description "Constructions-only oracle occurrences"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "screen_equivalent_layer_header_present: true" -Description "Constructions-only specialized header presence"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "constructions_report_requested: true" -Description "Constructions-only Constructions selector"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "materials_report_requested: false" -Description "Constructions-only absent Materials selector"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "screen_equivalent_layer_header_rows: 1" -Description "Constructions-only specialized header count"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "definition: 1 material: A DEFAULT AUTO BLANK GEOMETRY EQL SCREEN oracle_matches: 0 generic_fixed_zero_fields: 8 status: pass" -Description "Constructions-only canonical A definition"
Assert-Contains -Text $constructionsOnlyComparison -Pattern "occurrence: 3 construction: C HIGH PRECISION SECOND EQL SCREEN WINDOW CONSTRUCTION layer: 1 material: Z HIGH PRECISION REUSED EQL SCREEN status: pass" -Description "Constructions-only final Z occurrence"

$reportLines = @(
    "# Window material screen equivalent-layer smoke report",
    "",
    "- Case: window_material_screen_equivalent_layer_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Bounded Rust typed-input/EIO comparator evidence: true",
    "- Rust EIO serializer claim: false",
    "- Equivalent-layer runtime/optics/thermal claims: false",
    "- Tolerance mode: exact strings",
    "",
    "## Exact primary oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader
) + $genericRows + @(
    $constructionHeader,
    $screenHeader
) + $orderedRows + @(
    $firstHighPrecisionSurfaceRow,
    $secondHighPrecisionSurfaceRow,
    "~~~",
    "",
    "The generic rows are exact one-per-definition echoes in source order Z, M, A and include M UNUSED EQL SCREEN. The specialized construction-layer rows are in canonical source order A, Z, Z; M is absent.",
    "Two surfaces share construction B but it emits one Z row. Surface-unused construction C emits the second Z row, while fixture-only EMS construction-index variables keep A and C warning-free.",
    "The malformed specialized header contains nine comma-separated tokens while every specialized row contains twelve. Required visible inputs N4 through N6 are absent from the EIO shape.",
    "The A row preserves raw Autocalculate as -99999.0000 and blank source-effective spacing/diameter as 0.00000/0.00000. The high-precision Z row occurs twice byte-for-byte.",
    "",
    "## Materials-only reporting boundary",
    "",
    "The clean Materials-only run retains the exact generic Z, M, A rows and emits no Construction:WindowEquivalentLayer or specialized WindowMaterial:Screen:EquivalentLayer header/data rows.",
    "",
    "## Constructions-only reporting boundary",
    "",
    "The clean Constructions-only run retains the exact construction and specialized A, Z, Z rows and emits no generic Material Details header/data rows.",
    "",
    "## Bounded typed-input comparisons",
    "",
    "### Primary",
    "~~~text",
    $primaryComparison,
    "~~~",
    "",
    "### Materials-only",
    "~~~text",
    $materialsOnlyComparison,
    "~~~",
    "",
    "### Constructions-only",
    "~~~text",
    $constructionsOnlyComparison,
    "~~~",
    "",
    "All three lane directories retain the generated IDF, converted epJSON, EIO baseline, ERR log, and END completion summary under the comparison OutputRoot.",
    "",
    "This report is non-blocking diagnostic-only static typed-input/EIO comparison evidence. It adds no Rust EIO serializer, equivalent-layer construction packing, CheckAndFixCFSLayer, ASHWAT optics/thermal, rating, surface behavior, runtime, broad diagnostic-text, or conformance claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "WindowMaterial:Screen:EquivalentLayer report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window material screen equivalent-layer smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Tolerance mode: exact strings" -Description "report exact mode"
Assert-Contains -Text $reportText -Pattern ($genericRows -join [Environment]::NewLine) -Description "report exact generic Screen:EquivalentLayer rows"
Assert-Contains -Text $reportText -Pattern ($orderedRows -join [Environment]::NewLine) -Description "report exact construction and Screen:EquivalentLayer occurrence rows"
Assert-Contains -Text $reportText -Pattern ($firstHighPrecisionSurfaceRow + [Environment]::NewLine + $secondHighPrecisionSurfaceRow) -Description "report two shared-construction surface rows"
Assert-Contains -Text $reportText -Pattern "source order Z, M, A" -Description "report generic source order"
Assert-Contains -Text $reportText -Pattern "canonical source order A, Z, Z" -Description "report specialized occurrence order"
Assert-Contains -Text $reportText -Pattern "nine comma-separated tokens" -Description "report malformed specialized header"
Assert-Contains -Text $reportText -Pattern "-99999.0000" -Description "report raw Autocalculate sentinel"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report comparison pass marker"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"

Write-Host "WindowMaterial:Screen:EquivalentLayer three-lane oracle smoke passed."
Write-Host "Diagnostic-only, nonblocking bounded Rust typed-input/EIO evidence; no serializer or runtime claim."
Write-Host "Report: $ReportPath"
