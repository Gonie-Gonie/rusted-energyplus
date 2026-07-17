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
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-complex-shade\26.1.0"
$MaterialsOnlyOutputRoot = Join-Path $OutputRoot "materials-only"
$ConstructionsOnlyOutputRoot = Join-Path $OutputRoot "constructions-only"
$DefaultOutputRoot = Join-Path $OutputRoot "default"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_complex_shade_001"
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

function Assert-EioRowCount {
    param(
        [Parameter(Mandatory = $true)][string[]]$Lines,
        [Parameter(Mandatory = $true)][string]$Prefix,
        [Parameter(Mandatory = $true)][int]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $rows = @(
        $Lines |
            ForEach-Object { $_.Trim() } |
            Where-Object { $_.StartsWith($Prefix, [System.StringComparison]::Ordinal) }
    )
    if ($rows.Count -ne $Expected) {
        $rows | ForEach-Object { Write-Host $_ }
        throw "Expected $Expected $Description rows with prefix '$Prefix'; found $($rows.Count)."
    }
    Write-Host "OK $Description row count: $($rows.Count)"
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
        throw "EnergyPlus emitted a warning or severe diagnostic in the $Description run."
    }
}

function Assert-NoSpecializedWindowEvidence {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Description
    )
    foreach ($pattern in @(
        "! <WindowMaterial:ComplexShade>,",
        "WindowMaterial:ComplexShade,",
        "! <WindowMaterial:Glazing>",
        "WindowMaterial:Glazing,",
        "! <WindowConstruction>",
        "WindowConstruction,"
    )) {
        Assert-NotContains -Text $Text -Pattern $pattern -Description "$Description specialized/window evidence"
    }
}

function Assert-ConvertedComplexShadeArtifact {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $converted = Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json
    $shadeObjects = $converted.'WindowMaterial:ComplexShade'
    if ($null -eq $shadeObjects) {
        throw "Converted $Description epJSON has no WindowMaterial:ComplexShade object."
    }
    $actualNames = @($shadeObjects.PSObject.Properties | ForEach-Object { $_.Name })
    $expectedNames = @(
        "T WOVEN DEFAULTS",
        "U VENETIAN V FLAT",
        "V VENETIAN H EQUAL HALF",
        "W PERFORATED DEFAULTS",
        "X OTHER NONVENETIAN SUBHALF",
        "Y BSDF CUSTOM BASE",
        "Z FULL DEFAULTS"
    )
    if (($actualNames -join [char]31) -cne ($expectedNames -join [char]31)) {
        Write-Host "Expected converted names: $($expectedNames -join ', ')"
        Write-Host "Actual converted names:   $($actualNames -join ', ')"
        throw "Converted $Description ComplexShade key-order mismatch."
    }

    $zDefaults = $shadeObjects.'Z FULL DEFAULTS'
    $zProperties = @($zDefaults.PSObject.Properties | ForEach-Object { $_.Name })
    if ($zProperties.Count -ne 0) {
        throw "Converted $Description fully defaulted Z object unexpectedly materialized fields: $($zProperties -join ', ')"
    }
    foreach ($typeOnly in @(
        [pscustomobject]@{ Name = "T WOVEN DEFAULTS"; Type = "Woven" },
        [pscustomobject]@{ Name = "W PERFORATED DEFAULTS"; Type = "Perforated" }
    )) {
        $object = $shadeObjects.($typeOnly.Name)
        $properties = @($object.PSObject.Properties | ForEach-Object { $_.Name })
        if (($properties -join [char]31) -cne "layer_type") {
            throw "Converted $Description $($typeOnly.Name) should contain only layer_type; found $($properties -join ', ')."
        }
        if (-not $object.layer_type.Equals($typeOnly.Type, [System.StringComparison]::Ordinal)) {
            throw "Converted $Description $($typeOnly.Name) layer_type mismatch."
        }
    }
    Write-Host "OK converted $Description ComplexShade keys T,U,V,W,X,Y,Z; Z remains empty."
}

function Invoke-OracleCase {
    param(
        [Parameter(Mandatory = $true)][string]$Description,
        [Parameter(Mandatory = $true)][string]$OutputDirectory,
        [Parameter(Mandatory = $true)][string]$IdfName,
        [Parameter(Mandatory = $true)][string]$SelectorObject
    )
    New-Directory -Path $OutputDirectory
    $idfPath = Join-Path $OutputDirectory $IdfName
    if ($SelectorObject -eq "Output:Constructions,Constructions,Materials;") {
        Copy-Item -LiteralPath $fixtureIdf -Destination $idfPath -Force
    }
    else {
        $fixtureText = [System.IO.File]::ReadAllText($fixtureIdf)
        $derivedText = $fixtureText.Replace(
            "Output:Constructions,Constructions,Materials;",
            $SelectorObject
        )
        if ($derivedText.Equals($fixtureText, [System.StringComparison]::Ordinal)) {
            throw "Could not derive $Description fixture because the primary selector was not found."
        }
        $utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText($idfPath, $derivedText, $utf8WithoutBom)
    }

    Write-Host "Running EnergyPlus WindowMaterial:ComplexShade $Description oracle case."
    Invoke-External -FilePath $energyPlus -Arguments @("-D", "-d", $OutputDirectory, $idfPath)
    $eioPath = Join-Path $OutputDirectory "eplusout.eio"
    if (-not (Test-Path -LiteralPath $eioPath -PathType Leaf)) {
        throw "EnergyPlus did not produce required $Description EIO: $eioPath"
    }
    Assert-CleanOracleLog -Path (Join-Path $OutputDirectory "eplusout.err") -Description $Description

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
    return [pscustomobject]@{ Eio = $eioPath; EpJson = $epJsonPath }
}

function Invoke-ComplexShadeComparison {
    param(
        [Parameter(Mandatory = $true)][string]$CargoPath,
        [Parameter(Mandatory = $true)][string]$EpJsonPath,
        [Parameter(Mandatory = $true)][string]$EioPath,
        [Parameter(Mandatory = $true)][string]$Description
    )
    Write-Host "Comparing bounded Rust WindowMaterial:ComplexShade inputs with $Description EIO evidence."
    $output = & $CargoPath run -p ep_cli --quiet -- compare window-material-complex-shade $EpJsonPath $EioPath --tolerance exact 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "WindowMaterial:ComplexShade $Description comparison smoke failed."
    }
    $text = $output -join [Environment]::NewLine
    foreach ($pattern in @(
        "Window Material Complex Shade Comparison",
        "case_id: window_material_complex_shade_001",
        "comparison_class: smoke",
        "evidence: diagnostic-only",
        "blocking: false",
        "conformance_claim: false",
        "runtime_claim: false",
        "layer_type_reporting_claim: false",
        "infrared_reporting_claim: false",
        "front_emissivity_reporting_claim: false",
        "opening_reporting_claim: false",
        "slat_reporting_claim: false",
        "complex_fenestration_construction_claim: false",
        "specialized_complex_shade_claim: false",
        "specialized_glazing_claim: false",
        "window_construction_claim: false",
        "window_thermal_claim: false",
        "rust_eio_serialization_claim: false",
        "broad_idf_declaration_order_claim: false",
        "tolerance_mode: exact",
        "tolerance_policy: energyplus-26.1-window-material-complex-shade-material-details-source-format-normalized-exact",
        "material_objects: 7",
        "specialized_complex_shade_header_rows: 0",
        "specialized_complex_shade_rows: 0",
        "specialized_glazing_header_rows: 0",
        "specialized_glazing_rows: 0",
        "window_construction_header_rows: 0",
        "window_construction_rows: 0",
        "first_divergence: none",
        "status: pass"
    )) {
        Assert-Contains -Text $text -Pattern $pattern -Description "$Description comparison contract"
    }
    return $text
}

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
foreach ($path in @($energyPlus, $converter)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}
$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_complex_shade_001\window_material_complex_shade.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:ComplexShade fixture: $fixtureIdf"
}

Remove-RepoDirectory -Path $OutputRoot
Remove-RepoDirectory -Path $ReportRoot
New-Directory -Path $OutputRoot
New-Directory -Path $ReportRoot

$primary = Invoke-OracleCase -Description "primary Constructions-and-Materials" -OutputDirectory $OutputRoot -IdfName "window-material-complex-shade.idf" -SelectorObject "Output:Constructions,Constructions,Materials;"
$materialsOnly = Invoke-OracleCase -Description "Materials-only" -OutputDirectory $MaterialsOnlyOutputRoot -IdfName "window-material-complex-shade-materials-only.idf" -SelectorObject "Output:Constructions,Materials;"
$constructionsOnly = Invoke-OracleCase -Description "Constructions-only" -OutputDirectory $ConstructionsOnlyOutputRoot -IdfName "window-material-complex-shade-constructions-only.idf" -SelectorObject "Output:Constructions,Constructions;"
$default = Invoke-OracleCase -Description "blank/default-selector" -OutputDirectory $DefaultOutputRoot -IdfName "window-material-complex-shade-default.idf" -SelectorObject "Output:Constructions,;"

$materialDetailsHeader = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible"
$materialAirHeader = "! <Material:Air>,Material Name,ThermalResistance {m2-K/w}"
$ctfHeaders = @(
    "! <Construction CTF>,Construction Name,Index,#Layers,#CTFs,Time Step {hours},ThermalConductance {w/m2-K},OuterThermalAbsorptance,InnerThermalAbsorptance,OuterSolarAbsorptance,InnerSolarAbsorptance,Roughness",
    "! <Material CTF Summary>,Material Name,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},ThermalResistance {m2-K/w}",
    "! <Material:Air CTF Summary>,Material Name,ThermalResistance {m2-K/w}",
    "! <CTF>,Time,Outside,Cross,Inside,Flux (except final one)"
)
$ctfHeaderPrefixes = @(
    "! <Construction CTF>,",
    "! <Material CTF Summary>,",
    "! <Material:Air CTF Summary>,",
    "! <CTF>,"
)
$ctfDataPrefixes = @(
    "Construction CTF,",
    "Material CTF Summary,",
    "Material:Air CTF Summary,",
    "CTF,"
)
$complexShadeRows = @(
    "Material Details,Z FULL DEFAULTS,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000",
    "Material Details,Y BSDF CUSTOM BASE,0.0000,Rough,3.0000E-003,2.000,0.000,0.000,0.8000,0.0000,0.0000",
    "Material Details,X OTHER NONVENETIAN SUBHALF,0.0000,Rough,4.0000E-003,3.000,0.000,0.000,0.9000,0.0000,0.0000",
    "Material Details,W PERFORATED DEFAULTS,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000",
    "Material Details,V VENETIAN H EQUAL HALF,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000",
    "Material Details,U VENETIAN V FLAT,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000",
    "Material Details,T WOVEN DEFAULTS,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000"
)
$complexShadePrefixes = @(
    "Material Details,Z FULL DEFAULTS,",
    "Material Details,Y BSDF CUSTOM BASE,",
    "Material Details,X OTHER NONVENETIAN SUBHALF,",
    "Material Details,W PERFORATED DEFAULTS,",
    "Material Details,V VENETIAN H EQUAL HALF,",
    "Material Details,U VENETIAN V FLAT,",
    "Material Details,T WOVEN DEFAULTS,"
)
Assert-CommaSeparatedTokenCount -Row $materialDetailsHeader -Expected 11 -Description "generic material-details header"
foreach ($row in $complexShadeRows) {
    Assert-CommaSeparatedTokenCount -Row $row -Expected 11 -Description "WindowMaterial:ComplexShade generic definition row"
}

foreach ($lane in @(
    [pscustomobject]@{ Name = "primary"; Result = $primary; Materials = $true; Constructions = $true },
    [pscustomobject]@{ Name = "Materials-only"; Result = $materialsOnly; Materials = $true; Constructions = $false },
    [pscustomobject]@{ Name = "Constructions-only"; Result = $constructionsOnly; Materials = $false; Constructions = $true },
    [pscustomobject]@{ Name = "blank/default-selector"; Result = $default; Materials = $false; Constructions = $false }
)) {
    $lines = @(Get-Content -LiteralPath $lane.Result.Eio)
    $text = $lines -join [Environment]::NewLine
    if ($lane.Materials) {
        Assert-UniqueExactEioRow -Lines $lines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "$($lane.Name) generic material-details header"
        Assert-UniqueExactEioRow -Lines $lines -Prefix "! <Material:Air>," -Expected $materialAirHeader -Description "$($lane.Name) shared material-air header"
        Assert-EioRowCount -Lines $lines -Prefix "Material Details," -Expected 7 -Description "$($lane.Name) complete generic material-details data"
        Assert-EioRowCount -Lines $lines -Prefix "Material:Air," -Expected 0 -Description "$($lane.Name) material-air data"
        Assert-ExactOrderedEioRows -Lines $lines -Prefixes $complexShadePrefixes -Expected $complexShadeRows -Description "$($lane.Name) WindowMaterial:ComplexShade Z,Y,X,W,V,U,T"
    }
    else {
        Assert-NotContains -Text $text -Pattern "! <Material Details>," -Description "$($lane.Name) generic material-details header"
        Assert-EioRowCount -Lines $lines -Prefix "Material Details," -Expected 0 -Description "$($lane.Name) generic material-details data"
        Assert-NotContains -Text $text -Pattern "! <Material:Air>," -Description "$($lane.Name) shared material-air header"
        Assert-EioRowCount -Lines $lines -Prefix "Material:Air," -Expected 0 -Description "$($lane.Name) material-air data"
    }

    for ($index = 0; $index -lt $ctfHeaders.Count; $index++) {
        if ($lane.Constructions) {
            Assert-UniqueExactEioRow -Lines $lines -Prefix $ctfHeaderPrefixes[$index] -Expected $ctfHeaders[$index] -Description "$($lane.Name) empty CTF header"
        }
        else {
            Assert-NotContains -Text $text -Pattern $ctfHeaderPrefixes[$index] -Description "$($lane.Name) CTF header"
        }
        Assert-EioRowCount -Lines $lines -Prefix $ctfDataPrefixes[$index] -Expected 0 -Description "$($lane.Name) CTF data"
    }

    Assert-NoSpecializedWindowEvidence -Text $text -Description $lane.Name
    Assert-ConvertedComplexShadeArtifact -Path $lane.Result.EpJson -Description $lane.Name
}

$skippedComparison = "Skipped by -SkipRustComparison after exact oracle validation."
$comparisons = @($skippedComparison, $skippedComparison, $skippedComparison, $skippedComparison)
if (-not $SkipRustComparison) {
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -eq $cargo) {
        throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
    }
    $comparisons = @(
        (Invoke-ComplexShadeComparison -CargoPath $cargo.Source -EpJsonPath $primary.EpJson -EioPath $primary.Eio -Description "primary Constructions-and-Materials"),
        (Invoke-ComplexShadeComparison -CargoPath $cargo.Source -EpJsonPath $materialsOnly.EpJson -EioPath $materialsOnly.Eio -Description "Materials-only"),
        (Invoke-ComplexShadeComparison -CargoPath $cargo.Source -EpJsonPath $constructionsOnly.EpJson -EioPath $constructionsOnly.Eio -Description "Constructions-only"),
        (Invoke-ComplexShadeComparison -CargoPath $cargo.Source -EpJsonPath $default.EpJson -EioPath $default.Eio -Description "blank/default-selector")
    )
    foreach ($comparison in $comparisons[0..1]) {
        foreach ($pattern in @(
            "materials_report_requested: true",
            "oracle_window_material_complex_shade_rows: 7",
            "oracle_material_detail_rows: 7",
            "material_details_header_rows: 1"
        )) {
            Assert-Contains -Text $comparison -Pattern $pattern -Description "Materials-enabled comparison"
        }
    }
    Assert-Contains -Text $comparisons[0] -Pattern "constructions_report_requested: true" -Description "primary Constructions selector"
    Assert-Contains -Text $comparisons[1] -Pattern "constructions_report_requested: false" -Description "Materials-only Constructions selector"
    foreach ($comparison in $comparisons[2..3]) {
        foreach ($pattern in @(
            "materials_report_requested: false",
            "oracle_window_material_complex_shade_rows: 0",
            "oracle_material_detail_rows: 0",
            "material_details_header_rows: 0"
        )) {
            Assert-Contains -Text $comparison -Pattern $pattern -Description "Materials-disabled comparison"
        }
    }
    Assert-Contains -Text $comparisons[2] -Pattern "constructions_report_requested: true" -Description "Constructions-only selector"
    Assert-Contains -Text $comparisons[3] -Pattern "constructions_report_requested: false" -Description "default Constructions selector"
}

$reportLines = @(
    "# Window material complex shade smoke report",
    "",
    "- Case: window_material_complex_shade_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Runtime claim: false",
    "- Complex-fenestration construction claim: false",
    "- Tolerance mode: exact",
    "",
    "## Exact Materials-enabled target evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader,
    $materialAirHeader
) + $complexShadeRows + @(
    "~~~",
    "",
    "Both Materials-enabled lanes emit exactly the generic header and seven rows in fixture-local source IDF order Z, Y, X, W, V, U, T.",
    "Every row has eleven tokens and proves only normalized identity, Rough, resistance, thickness, conductivity, density, specific heat, and the three generic absorptance projections.",
    "Constructions-only and blank/default-selector lanes emit no Material Details or Material:Air header or data.",
    "Both Constructions-enabled lanes emit exactly four empty CTF headers and no CTF data; both Constructions-disabled lanes omit those headers.",
    "All four lanes omit dedicated WindowMaterial:ComplexShade, WindowMaterial:Glazing, and WindowConstruction headers and data rows.",
    "Every oracle lane completes with zero warnings and zero severe errors and produces a converted epJSON artifact.",
    "ConvertInputFormat orders ComplexShade keys T, U, V, W, X, Y, Z and keeps fully defaulted Z as an empty object, so broad source-IDF/converted-epJSON order parity is explicitly unclaimed.",
    "",
    "## Bounded typed-input comparisons",
    "",
    "### Primary Constructions-and-Materials",
    "~~~text", $comparisons[0], "~~~", "",
    "### Materials-only",
    "~~~text", $comparisons[1], "~~~", "",
    "### Constructions-only",
    "~~~text", $comparisons[2], "~~~", "",
    "### Blank/default selector",
    "~~~text", $comparisons[3], "~~~", "",
    "Layer type, infrared transmittance, front emissivity, openings, slat state, complex-fenestration construction use, heat transfer, surfaces, runtime, diagnostics, Rust EIO serialization, and broad IDF/epJSON declaration order remain explicit nonclaims.",
    "",
    "This report is non-blocking diagnostic-only static material evidence and makes no WindowMaterial:ComplexShade conformance claim.",
    ""
)
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, ($reportLines -join [Environment]::NewLine), $utf8WithoutBom)
$reportText = Get-Content -LiteralPath $ReportPath -Raw
foreach ($pattern in @(
    "# Window material complex shade smoke report",
    "Evidence level: diagnostic-only",
    "Blocking: false",
    "Conformance claim: false",
    "Tolerance mode: exact",
    ($complexShadeRows -join [Environment]::NewLine),
    "fixture-local source IDF order Z, Y, X, W, V, U, T",
    "emit no Material Details or Material:Air header or data",
    "emit exactly four empty CTF headers and no CTF data",
    "omit dedicated WindowMaterial:ComplexShade, WindowMaterial:Glazing, and WindowConstruction",
    "orders ComplexShade keys T, U, V, W, X, Y, Z",
    "broad source-IDF/converted-epJSON order parity is explicitly unclaimed"
)) {
    Assert-Contains -Text $reportText -Pattern $pattern -Description "report contract"
}
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
if (-not $SkipRustComparison) {
    Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
    Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"
}

Write-Host "WindowMaterial:ComplexShade oracle and comparison smoke passed."
if ($SkipRustComparison) {
    Write-Host "Rust comparison was skipped explicitly; all four exact oracle selector, row, conversion, and absence gates passed."
}
Write-Host "Diagnostic-only, nonblocking evidence; no complex-construction, runtime, or conformance claim."
Write-Host "Report: $ReportPath"
