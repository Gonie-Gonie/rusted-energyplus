[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-gas-mixture\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_gas_mixture_001"
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

function Assert-ZeroEioRows {
    param(
        [Parameter(Mandatory = $true)][string[]]$Lines,
        [Parameter(Mandatory = $true)][string]$Prefix,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $rows = @(
        $Lines |
            ForEach-Object { $_.Trim() } |
            Where-Object { $_.StartsWith($Prefix, [System.StringComparison]::Ordinal) }
    )
    if ($rows.Count -ne 0) {
        $rows | ForEach-Object { Write-Host $_ }
        throw "Expected zero $Description rows with prefix '$Prefix'; found $($rows.Count)."
    }
    Write-Host "OK zero $Description rows"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_gas_mixture_001\window_material_gas_mixture.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:GasMixture fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "window-material-gas-mixture.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus WindowMaterial:GasMixture oracle case."
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
$gasHeader = "! <WindowMaterial:Gas>,Material Name,GasType,Thickness {m}"
$materialRows = @(
    "Material Details,A COUNT1 XENON MIX,0.0000,MediumRough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,B ORDER AIR ARGON MIX,0.0000,MediumRough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,C ORDER ARGON AIR MIX,0.0000,MediumRough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,D COUNT3 ZERO THIRD MIX,0.0000,MediumRough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,E COUNT4 DUPLICATE MIX,0.0000,MediumRough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,Z UNUSED KRYPTON XENON MIX,0.0000,MediumRough,1.5900E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000"
)
$materialPrefixes = @(
    "Material Details,A COUNT1 XENON MIX,",
    "Material Details,B ORDER AIR ARGON MIX,",
    "Material Details,C ORDER ARGON AIR MIX,",
    "Material Details,D COUNT3 ZERO THIRD MIX,",
    "Material Details,E COUNT4 DUPLICATE MIX,",
    "Material Details,Z UNUSED KRYPTON XENON MIX,"
)
$constructionRows = @(
    "WindowConstruction,A AIR WINDOW CONSTRUCTION,1,7,VerySmooth,1.127,1.127,1.000,0.330,0.143,0.267",
    "WindowConstruction,B ARGON WINDOW CONSTRUCTION,2,7,VerySmooth,1.103,1.103,1.000,0.320,0.143,0.267"
)
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE GAS WINDOW HOST WALL,Wall,,CTF - ConductionTransferFunction,G OPAQUE HOST CONSTRUCTION,3.071,2.104,,10.02,12.00,10.02,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$airWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE GAS TEST WINDOW,Window,DISTINCTIVE GAS WINDOW HOST WALL,Window5 Detailed Fenestration,A AIR WINDOW CONSTRUCTION,N/A,1.127,No,0.99,0.99,0.99,180.00,90.00,0.90,1.10,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$argonWindowSurfaceRow = "HeatTransfer Surface,GAS TEST ARGON WINDOW,Window,DISTINCTIVE GAS WINDOW HOST WALL,Window5 Detailed Fenestration,B ARGON WINDOW CONSTRUCTION,N/A,1.103,No,0.99,0.99,0.99,180.00,90.00,0.90,1.10,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

$eioLines = @(Get-Content -LiteralPath $eio)
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "generic material-details header"
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes $materialPrefixes -Expected $materialRows -Description "gas-mixture generic definition"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <WindowMaterial:Gas>," -Expected $gasHeader -Description "shared window gas header"
Assert-ZeroEioRows -Lines $eioLines -Prefix "WindowMaterial:Gas," -Description "single-gas material data"
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes @("WindowConstruction,A AIR WINDOW CONSTRUCTION,", "WindowConstruction,B ARGON WINDOW CONSTRUCTION,") -Expected $constructionRows -Description "gas-mixture window construction"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE GAS WINDOW HOST WALL," -Expected $hostSurfaceRow -Description "opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE GAS TEST WINDOW," -Expected $airWindowSurfaceRow -Description "first gas-mixture fenestration heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,GAS TEST ARGON WINDOW," -Expected $argonWindowSurfaceRow -Description "second gas-mixture fenestration heat-transfer surface"

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-material-gas-mixture.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "window-material-gas-mixture.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-material-gas-mixture.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing bounded Rust WindowMaterial:GasMixture definitions with generic EnergyPlus EIO details."
$output = & $cargo.Source run -p ep_cli --quiet -- compare window-material-gas-mixture $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "WindowMaterial:GasMixture comparison smoke failed."
}

$text = ($output -join [Environment]::NewLine)
Assert-Contains -Text $text -Pattern "Window Material Gas Mixture Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "window_runtime_claim: false" -Description "window runtime boundary"
Assert-Contains -Text $text -Pattern "fenestration_surface_claim: false" -Description "fenestration surface boundary"
Assert-Contains -Text $text -Pattern "construction_rating_claim: false" -Description "construction rating boundary"
Assert-Contains -Text $text -Pattern "component_count_claim: false" -Description "component count boundary"
Assert-Contains -Text $text -Pattern "component_species_claim: false" -Description "component species boundary"
Assert-Contains -Text $text -Pattern "component_fraction_claim: false" -Description "component fraction boundary"
Assert-Contains -Text $text -Pattern "component_order_claim: false" -Description "component order boundary"
Assert-Contains -Text $text -Pattern "mixture_occurrence_claim: false" -Description "mixture occurrence boundary"
Assert-Contains -Text $text -Pattern "mixture_reuse_claim: false" -Description "mixture reuse boundary"
Assert-Contains -Text $text -Pattern "mixture_unused_definition_claim: false" -Description "mixture unused-definition boundary"
Assert-Contains -Text $text -Pattern "nominal_resistance_claim: false" -Description "first-gas nominal-resistance boundary"
Assert-Contains -Text $text -Pattern "broad_idf_declaration_order_claim: false" -Description "broad IDF declaration-order boundary"
Assert-Contains -Text $text -Pattern "arbitrary_idf_declaration_order_claim: false" -Description "arbitrary IDF declaration-order boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: energyplus-26.1-round-sig-digits-4-normalized-exact" -Description "source-format thickness policy"
Assert-Contains -Text $text -Pattern "material_objects: 6" -Description "Rust gas-mixture definition count"
Assert-Contains -Text $text -Pattern "oracle_material_rows: 6" -Description "matched oracle gas-mixture definition count"
Assert-Contains -Text $text -Pattern "oracle_material_detail_rows: 8" -Description "all oracle generic material-detail row count"
Assert-Contains -Text $text -Pattern "gas_header_present: true" -Description "shared gas header presence"
Assert-Contains -Text $text -Pattern "gas_header_rows: 1" -Description "shared gas header count"
Assert-Contains -Text $text -Pattern "gas_data_rows: 0" -Description "single-gas data exclusion"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

$reportLines = @(
    "# Window material gas-mixture smoke report",
    "",
    "- Case: window_material_gas_mixture_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Window runtime claim: false",
    "- Fenestration surface claim: false",
    "- Construction rating claim: false",
    "- Component count/species/fraction/order claims: false",
    "- Mixture occurrence/reuse/unused-use claims: false",
    "- First-gas nominal resistance claim: false",
    "- Broad/arbitrary IDF declaration-order claims: false",
    "",
    "## Exact oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader
) + $materialRows + @(
    $gasHeader
) + $constructionRows + @(
    $hostSurfaceRow,
    $airWindowSurfaceRow,
    $argonWindowSurfaceRow,
    "~~~",
    "",
    "The six gas-mixture Material Details rows are generic definition echoes. Each definition appears once, including the unused Z definition; the rows cannot establish occurrence, reuse, or unused-use behavior.",
    "The EIO contains the shared WindowMaterial:Gas header but zero WindowMaterial:Gas data rows. Gas-mixture component count, species, fractions, order, and first-gas nominal resistance are not exposed.",
    "The two construction and three heat-transfer-surface rows are oracle-only fixture-integrity locks; their ratings, layers, runtime, and surface behavior are not Rust parity claims.",
    "",
    "## Bounded typed-input comparison",
    "",
    "~~~text",
    $text,
    "~~~",
    "",
    "The comparison matches the six typed gas-mixture definitions by canonical name and checks only the generic MediumRough, {:.4R} thickness, and fixed-zero numeric fields exposed by EnergyPlus 26.1.",
    "",
    "This report is non-blocking diagnostic-only static definition evidence. It makes no gas-mixture component, occurrence, construction-rating, fenestration-runtime, window-runtime, declaration-order, or conformance claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "WindowMaterial:GasMixture report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window material gas-mixture smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Component count/species/fraction/order claims: false" -Description "report component boundary"
Assert-Contains -Text $reportText -Pattern "Mixture occurrence/reuse/unused-use claims: false" -Description "report occurrence boundary"
Assert-Contains -Text $reportText -Pattern "First-gas nominal resistance claim: false" -Description "report nominal-resistance boundary"
Assert-Contains -Text $reportText -Pattern "Broad/arbitrary IDF declaration-order claims: false" -Description "report declaration-order boundary"
Assert-Contains -Text $reportText -Pattern ($materialRows -join [Environment]::NewLine) -Description "report exact gas-mixture generic rows"
Assert-Contains -Text $reportText -Pattern ($constructionRows -join [Environment]::NewLine) -Description "report exact construction rows"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"

Write-Host "WindowMaterial:GasMixture comparison smoke passed."
Write-Host "Diagnostic-only, nonblocking evidence; no gas-mixture component, occurrence, runtime, rating, declaration-order, or conformance claim."
Write-Host "Report: $ReportPath"
