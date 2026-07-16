[CmdletBinding()]
param(
    [switch]$SkipRustComparison
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-blind\26.1.0"
$MaterialsOnlyOutputRoot = Join-Path $OutputRoot "materials-only"
$ConstructionsOnlyOutputRoot = Join-Path $OutputRoot "constructions-only"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_blind_001"
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

    Write-Host "Running EnergyPlus WindowMaterial:Blind $Description oracle case."
    Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputDirectory, $idfPath)

    $eioPath = Join-Path $OutputDirectory "eplusout.eio"
    $errPath = Join-Path $OutputDirectory "eplusout.err"
    if (-not (Test-Path -LiteralPath $eioPath -PathType Leaf)) {
        throw "EnergyPlus did not produce required $Description EIO: $eioPath"
    }
    Assert-CleanOracleLog -Path $errPath -Description $Description

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

function Invoke-BlindComparison {
    param(
        [Parameter(Mandatory = $true)][string]$CargoPath,
        [Parameter(Mandatory = $true)][string]$EpJsonPath,
        [Parameter(Mandatory = $true)][string]$EioPath,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Write-Host "Comparing bounded Rust WindowMaterial:Blind inputs with $Description EnergyPlus EIO evidence."
    $output = & $CargoPath run -p ep_cli --quiet -- compare window-material-blind $EpJsonPath $EioPath --tolerance exact 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "WindowMaterial:Blind $Description comparison smoke failed."
    }
    $text = $output -join [Environment]::NewLine
    Assert-Contains -Text $text -Pattern "Window Material Blind Comparison" -Description "$Description comparison header"
    Assert-Contains -Text $text -Pattern "evidence: diagnostic-only" -Description "$Description diagnostic boundary"
    Assert-Contains -Text $text -Pattern "blocking: false" -Description "$Description nonblocking boundary"
    Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "$Description conformance boundary"
    Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "$Description first divergence"
    Assert-Contains -Text $text -Pattern "status: pass" -Description "$Description comparison status"
    return $text
}

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
$weather = Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
foreach ($path in @($energyPlus, $converter, $weather)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_blind_001\window_material_blind.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:Blind fixture: $fixtureIdf"
}

Remove-RepoDirectory -Path $OutputRoot
Remove-RepoDirectory -Path $ReportRoot
New-Directory -Path $OutputRoot
New-Directory -Path $ReportRoot

$primary = Invoke-OracleCase -Description "primary Constructions-and-Materials" -OutputDirectory $OutputRoot -IdfName "window-material-blind.idf" -Selector "Constructions,Materials"
$materialsOnly = Invoke-OracleCase -Description "Materials-only" -OutputDirectory $MaterialsOnlyOutputRoot -IdfName "window-material-blind-materials-only.idf" -Selector "Materials"
$constructionsOnly = Invoke-OracleCase -Description "Constructions-only" -OutputDirectory $ConstructionsOnlyOutputRoot -IdfName "window-material-blind-constructions-only.idf" -Selector "Constructions"

$materialDetailsHeader = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible"
$windowConstructionHeader = "! <WindowConstruction>,Construction Name,Index,#Layers,Roughness,Conductance {W/m2-K},Conductance (Before Adjusted) {W/m2-K},Convection Coefficient Adjustment Ratio,SHGC,Solar Transmittance at Normal Incidence,Visible Transmittance at Normal Incidence"
$blindHeader = "! <WindowMaterial:Blind>,Material Name,Slat Width {m},Slat Separation {m},Slat Thickness {m},Slat Angle {deg},Slat Beam Solar Transmittance,Slat Beam Solar Front Reflectance,Blind To Glass Distance {m}"
$genericRows = @(
    "Material Details,Z HIGH PRECISION REUSED BLIND,0.0000,Rough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,M UNUSED BLIND,0.0000,Rough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,A DEFAULTED USED BLIND,0.0000,Rough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000"
)
$genericPrefixes = @(
    "Material Details,Z HIGH PRECISION REUSED BLIND,",
    "Material Details,M UNUSED BLIND,",
    "Material Details,A DEFAULTED USED BLIND,"
)
$orderedWindowRows = @(
    "WindowConstruction,A BASE BLIND TEST WINDOW CONSTRUCTION,1,1,VerySmooth,5.778,5.778,1.000,0.819,0.775,0.881",
    "WindowConstruction,B DEFAULTED EXTERIOR BLIND WINDOW CONSTRUCTION,2,2,Rough,4.913,4.913,1.000,6.218E-002,5.848E-002,4.993E-002",
    "WindowMaterial:Blind,A DEFAULTED USED BLIND,2.5000E-002,1.8750E-002,2.5000E-004,45.000,0.000,0.200,5.000E-002",
    "WindowConstruction,C HIGH PRECISION INTERIOR BLIND WINDOW CONSTRUCTION,3,2,VerySmooth,5.778,5.778,1.000,0.765,0.363,0.408",
    "WindowMaterial:Blind,Z HIGH PRECISION REUSED BLIND,2.4568E-002,1.8765E-002,6.5432E-004,63.457,0.123,0.235,3.457E-002",
    "WindowConstruction,D HIGH PRECISION UNUSED EXTERIOR BLIND CONSTRUCTION,4,2,Rough,4.913,4.913,1.000,0.380,0.358,0.403",
    "WindowMaterial:Blind,Z HIGH PRECISION REUSED BLIND,2.4568E-002,1.8765E-002,6.5432E-004,63.457,0.123,0.235,3.457E-002"
)
$orderedWindowPrefixes = @(
    "WindowConstruction,A BASE BLIND TEST WINDOW CONSTRUCTION,",
    "WindowConstruction,B DEFAULTED EXTERIOR BLIND WINDOW CONSTRUCTION,",
    "WindowConstruction,C HIGH PRECISION INTERIOR BLIND WINDOW CONSTRUCTION,",
    "WindowConstruction,D HIGH PRECISION UNUSED EXTERIOR BLIND CONSTRUCTION,",
    "WindowMaterial:Blind,"
)
$specializedRows = @($orderedWindowRows[2], $orderedWindowRows[4], $orderedWindowRows[6])
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE BLIND WINDOW HOST WALL,Wall,,CTF - ConductionTransferFunction,E BLIND OPAQUE HOST CONSTRUCTION,3.071,2.104,,9.60,12.00,9.60,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$defaultedWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE DEFAULTED BLIND TEST WINDOW,Window,DISTINCTIVE BLIND WINDOW HOST WALL,Window5 Detailed Fenestration,A BASE BLIND TEST WINDOW CONSTRUCTION,N/A,5.778,No,0.80,0.80,0.80,180.00,90.00,0.80,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$firstHighPrecisionWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE FIRST HIGH PRECISION BLIND TEST WINDOW,Window,DISTINCTIVE BLIND WINDOW HOST WALL,Window5 Detailed Fenestration,A BASE BLIND TEST WINDOW CONSTRUCTION,N/A,5.778,No,0.80,0.80,0.80,180.00,90.00,0.80,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$secondHighPrecisionWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE SECOND HIGH PRECISION BLIND TEST WINDOW,Window,DISTINCTIVE BLIND WINDOW HOST WALL,Window5 Detailed Fenestration,A BASE BLIND TEST WINDOW CONSTRUCTION,N/A,5.778,No,0.80,0.80,0.80,180.00,90.00,0.80,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

$primaryLines = @(Get-Content -LiteralPath $primary.Eio)
$primaryText = $primaryLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "primary generic material-details header"
Assert-ExactOrderedEioRows -Lines $primaryLines -Prefixes $genericPrefixes -Expected $genericRows -Description "primary Blind generic definition Z,M,A"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "! <WindowConstruction>," -Expected $windowConstructionHeader -Description "primary window-construction header"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "! <WindowMaterial:Blind>," -Expected $blindHeader -Description "primary specialized Blind header"
Assert-CommaSeparatedTokenCount -Row $blindHeader -Expected 9 -Description "primary specialized Blind header"
foreach ($row in $specializedRows) {
    Assert-CommaSeparatedTokenCount -Row $row -Expected 9 -Description "primary specialized Blind data row"
}
Assert-ExactOrderedEioRows -Lines $primaryLines -Prefixes $orderedWindowPrefixes -Expected $orderedWindowRows -Description "primary window construction and Blind occurrence A,Z,Z"
Assert-NotContains -Text $primaryText -Pattern "WindowMaterial:Blind,M UNUSED BLIND," -Description "primary unused Blind specialized occurrence"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "HeatTransfer Surface,DISTINCTIVE BLIND WINDOW HOST WALL," -Expected $hostSurfaceRow -Description "primary opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "HeatTransfer Surface,DISTINCTIVE DEFAULTED BLIND TEST WINDOW," -Expected $defaultedWindowSurfaceRow -Description "primary defaulted Blind host window"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "HeatTransfer Surface,DISTINCTIVE FIRST HIGH PRECISION BLIND TEST WINDOW," -Expected $firstHighPrecisionWindowSurfaceRow -Description "primary first high-precision Blind host window"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "HeatTransfer Surface,DISTINCTIVE SECOND HIGH PRECISION BLIND TEST WINDOW," -Expected $secondHighPrecisionWindowSurfaceRow -Description "primary second high-precision Blind host window"
foreach ($constructionName in @(
    "B DEFAULTED EXTERIOR BLIND WINDOW CONSTRUCTION",
    "C HIGH PRECISION INTERIOR BLIND WINDOW CONSTRUCTION",
    "D HIGH PRECISION UNUSED EXTERIOR BLIND CONSTRUCTION"
)) {
    Assert-NotContains -Text $primaryText -Pattern "Window5 Detailed Fenestration,$constructionName," -Description "surface reference to shading construction $constructionName"
}

$materialsOnlyLines = @(Get-Content -LiteralPath $materialsOnly.Eio)
$materialsOnlyText = $materialsOnlyLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $materialsOnlyLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "Materials-only generic material-details header"
Assert-ExactOrderedEioRows -Lines $materialsOnlyLines -Prefixes $genericPrefixes -Expected $genericRows -Description "Materials-only Blind generic definition Z,M,A"
Assert-NotContains -Text $materialsOnlyText -Pattern "! <WindowConstruction>," -Description "Materials-only window-construction header"
Assert-NotContains -Text $materialsOnlyText -Pattern "WindowConstruction," -Description "Materials-only window-construction row"
Assert-NotContains -Text $materialsOnlyText -Pattern "! <WindowMaterial:Blind>," -Description "Materials-only specialized Blind header"
Assert-NotContains -Text $materialsOnlyText -Pattern "WindowMaterial:Blind," -Description "Materials-only specialized Blind occurrence"

$constructionsOnlyLines = @(Get-Content -LiteralPath $constructionsOnly.Eio)
$constructionsOnlyText = $constructionsOnlyLines -join [Environment]::NewLine
Assert-NotContains -Text $constructionsOnlyText -Pattern "! <Material Details>," -Description "Constructions-only generic material-details header"
foreach ($prefix in $genericPrefixes) {
    Assert-NotContains -Text $constructionsOnlyText -Pattern $prefix -Description "Constructions-only generic Blind definition"
}
Assert-UniqueExactEioRow -Lines $constructionsOnlyLines -Prefix "! <WindowConstruction>," -Expected $windowConstructionHeader -Description "Constructions-only window-construction header"
Assert-UniqueExactEioRow -Lines $constructionsOnlyLines -Prefix "! <WindowMaterial:Blind>," -Expected $blindHeader -Description "Constructions-only specialized Blind header"
Assert-ExactOrderedEioRows -Lines $constructionsOnlyLines -Prefixes $orderedWindowPrefixes -Expected $orderedWindowRows -Description "Constructions-only window construction and Blind occurrence A,Z,Z"
Assert-NotContains -Text $constructionsOnlyText -Pattern "WindowMaterial:Blind,M UNUSED BLIND," -Description "Constructions-only unused Blind specialized occurrence"

$primaryComparison = "Skipped by -SkipRustComparison after exact oracle validation."
$materialsOnlyComparison = $primaryComparison
$constructionsOnlyComparison = $primaryComparison
if (-not $SkipRustComparison) {
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -eq $cargo) {
        throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
    }
    $primaryComparison = Invoke-BlindComparison -CargoPath $cargo.Source -EpJsonPath $primary.EpJson -EioPath $primary.Eio -Description "primary Constructions-and-Materials"
    $materialsOnlyComparison = Invoke-BlindComparison -CargoPath $cargo.Source -EpJsonPath $materialsOnly.EpJson -EioPath $materialsOnly.Eio -Description "Materials-only"
    $constructionsOnlyComparison = Invoke-BlindComparison -CargoPath $cargo.Source -EpJsonPath $constructionsOnly.EpJson -EioPath $constructionsOnly.Eio -Description "Constructions-only"

    Assert-Contains -Text $primaryComparison -Pattern "materials_report_requested: true" -Description "primary Rust Materials selector"
    Assert-Contains -Text $primaryComparison -Pattern "constructions_report_requested: true" -Description "primary Rust Constructions selector"
    Assert-Contains -Text $primaryComparison -Pattern "material_details_header_rows: 1" -Description "primary Rust generic header"
    Assert-Contains -Text $primaryComparison -Pattern "oracle_generic_blind_rows: 3" -Description "primary Rust generic Blind rows"
    Assert-Contains -Text $primaryComparison -Pattern "activated_blind_materials: 2" -Description "primary Rust activated Blind materials"
    Assert-Contains -Text $primaryComparison -Pattern "blind_occurrences: 3" -Description "primary Rust Blind occurrences"
    Assert-Contains -Text $primaryComparison -Pattern "oracle_blind_rows: 3" -Description "primary Rust oracle Blind rows"
    Assert-Contains -Text $primaryComparison -Pattern "blind_header_rows: 1" -Description "primary Rust specialized header"

    Assert-Contains -Text $materialsOnlyComparison -Pattern "materials_report_requested: true" -Description "Materials-only Rust Materials selector"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "constructions_report_requested: false" -Description "Materials-only Rust Constructions selector"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "material_details_header_rows: 1" -Description "Materials-only Rust generic header"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "oracle_generic_blind_rows: 3" -Description "Materials-only Rust generic Blind rows"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "activated_blind_materials: 0" -Description "Materials-only Rust activation boundary"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "blind_occurrences: 0" -Description "Materials-only Rust occurrence boundary"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "oracle_blind_rows: 0" -Description "Materials-only Rust oracle-row boundary"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "blind_header_rows: 0" -Description "Materials-only Rust specialized-header boundary"

    Assert-Contains -Text $constructionsOnlyComparison -Pattern "materials_report_requested: false" -Description "Constructions-only Rust Materials selector"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "constructions_report_requested: true" -Description "Constructions-only Rust Constructions selector"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "material_details_header_rows: 0" -Description "Constructions-only Rust generic-header boundary"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "oracle_generic_blind_rows: 0" -Description "Constructions-only Rust generic-row boundary"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "activated_blind_materials: 2" -Description "Constructions-only Rust activated Blind materials"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "blind_occurrences: 3" -Description "Constructions-only Rust Blind occurrences"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "oracle_blind_rows: 3" -Description "Constructions-only Rust oracle Blind rows"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "blind_header_rows: 1" -Description "Constructions-only Rust specialized header"
}

$reportLines = @(
    "# Window material blind smoke report",
    "",
    "- Case: window_material_blind_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Static typed-input/EIO definition and construction-occurrence evidence only: true",
    "- Blind TAR, angle interpolation, slat-angle control, window optics, and thermal claims: false",
    "- Shading-control, fenestration-surface, EMS, and between-glass claims: false",
    "- Construction-rating, runtime, and declaration-order claims: false",
    "- Tolerance mode: exact",
    "",
    "## Exact primary oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader
) + $genericRows + @(
    $windowConstructionHeader,
    $blindHeader
) + $orderedWindowRows + @(
    $hostSurfaceRow,
    $defaultedWindowSurfaceRow,
    $firstHighPrecisionWindowSurfaceRow,
    $secondHighPrecisionWindowSurfaceRow,
    "~~~",
    "",
    "The generic rows are one-per-definition echoes in source order Z, M, A and include M UNUSED BLIND. The specialized rows are construction-layer occurrences in order A, Z, Z; M is absent.",
    "Two surfaces share interior construction C but it emits one Z row. Exterior construction D has no surface but emits the second Z row; a fixture-only EMS construction-index variable keeps D warning-free.",
    "All three fenestration surfaces retain the bare A BASE BLIND TEST WINDOW CONSTRUCTION. WindowShadingControl links the defaulted surface to exterior construction B and the two high-precision surfaces to interior construction C while a constant-zero schedule keeps both controls inactive.",
    "The exact specialized header and data rows each contain nine comma-separated tokens. Slat width, separation, and thickness use source {:.4R}; the remaining specialized numeric fields use source {:.3R}.",
    "Controls, surfaces, and EMS are stable fixture locks only. The comparison makes no control, surface, EMS, or runtime claim and excludes between-glass blinds and unmatched bare stacks.",
    "",
    "## Materials-only reporting boundary",
    "",
    "The clean Materials-only run retains the exact generic Z, M, A rows and emits no WindowConstruction or specialized WindowMaterial:Blind header/data rows.",
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
    "This report is non-blocking diagnostic-only static material evidence. It makes no Blind TAR, angle interpolation, slat-angle control, window optics, thermal, daylighting, shading-control, surface, EMS, between-glass, construction-rating, runtime, declaration-order, or conformance claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "WindowMaterial:Blind report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window material blind smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Tolerance mode: exact" -Description "report exact tolerance mode"
Assert-Contains -Text $reportText -Pattern ($genericRows -join [Environment]::NewLine) -Description "report exact generic Blind rows"
Assert-Contains -Text $reportText -Pattern ($orderedWindowRows -join [Environment]::NewLine) -Description "report exact construction and Blind occurrence rows"
Assert-Contains -Text $reportText -Pattern "source order Z, M, A" -Description "report generic definition order"
Assert-Contains -Text $reportText -Pattern "order A, Z, Z" -Description "report specialized occurrence order"
Assert-Contains -Text $reportText -Pattern "nine comma-separated tokens" -Description "report specialized token count"
Assert-Contains -Text $reportText -Pattern "emits no WindowConstruction or specialized WindowMaterial:Blind header/data rows" -Description "report Materials-only activation boundary"
Assert-Contains -Text $reportText -Pattern "emits no generic Material Details header/data rows" -Description "report Constructions-only activation boundary"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
if (-not $SkipRustComparison) {
    Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
    Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"
}

Write-Host "WindowMaterial:Blind three-lane oracle and comparison smoke passed."
if ($SkipRustComparison) {
    Write-Host "Rust comparison was skipped explicitly; all three EnergyPlus activation and exact-row gates passed."
}
Write-Host "Diagnostic-only, nonblocking evidence; no Blind runtime, optics, thermal, control, surface, EMS, between-glass, declaration-order, or conformance claim."
Write-Host "Report: $ReportPath"
