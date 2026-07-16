[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-shade\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_shade_001"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_shade_001\window_material_shade.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:Shade fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "window-material-shade.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus WindowMaterial:Shade oracle case."
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
$shadeHeader = "! <WindowMaterial:Shade>,Material Name,Thickness {m},Conductivity {W/m-K},Thermal Absorptance,Transmittance,Visible Transmittance,Shade Reflectance"
$genericRows = @(
    "Material Details,A DEFAULTED UNUSED SHADE,0.0000,MediumRough,4.0000E-004,0.100,0.000,0.000,0.1500,0.2250,0.0000",
    "Material Details,B REUSED HIGH PRECISION SHADE,0.0000,MediumRough,4.5670E-004,0.123,0.000,0.000,0.5679,0.6420,0.0000"
)
$genericPrefixes = @(
    "Material Details,A DEFAULTED UNUSED SHADE,",
    "Material Details,B REUSED HIGH PRECISION SHADE,"
)
$orderedWindowRows = @(
    "WindowConstruction,A BARE SINGLE SHADE WINDOW CONSTRUCTION,1,1,VerySmooth,5.778,5.778,1.000,0.819,0.775,0.881",
    "WindowConstruction,B EXTERIOR SHADE WINDOW CONSTRUCTION,2,2,MediumRough,4.913,4.913,1.000,9.443E-002,8.793E-002,0.296",
    "WindowMaterial:Shade,B REUSED HIGH PRECISION SHADE,4.567E-004,0.123,0.568,0.123,0.346,0.235",
    "WindowConstruction,C INTERIOR SHADE WINDOW CONSTRUCTION,3,2,VerySmooth,5.778,5.778,1.000,0.665,9.868E-002,0.326",
    "WindowMaterial:Shade,B REUSED HIGH PRECISION SHADE,4.567E-004,0.123,0.568,0.123,0.346,0.235"
)
$hostSurfaceRow = "HeatTransfer Surface,DISTINCTIVE SHADE WINDOW HOST WALL,Wall,,CTF - ConductionTransferFunction,G SHADE OPAQUE HOST CONSTRUCTION,3.071,2.104,,10.02,12.00,10.02,180.00,90.00,4.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$exteriorWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE EXTERIOR SHADE TEST WINDOW,Window,DISTINCTIVE SHADE WINDOW HOST WALL,Window5 Detailed Fenestration,A BARE SINGLE SHADE WINDOW CONSTRUCTION,N/A,5.778,No,0.99,0.99,0.99,180.00,90.00,0.90,1.10,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"
$interiorWindowSurfaceRow = "HeatTransfer Surface,DISTINCTIVE INTERIOR SHADE TEST WINDOW,Window,DISTINCTIVE SHADE WINDOW HOST WALL,Window5 Detailed Fenestration,A BARE SINGLE SHADE WINDOW CONSTRUCTION,N/A,5.778,No,0.99,0.99,0.99,180.00,90.00,0.90,1.10,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4"

$eioLines = @(Get-Content -LiteralPath $eio)
$eioText = $eioLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "generic material-details header"
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes $genericPrefixes -Expected $genericRows -Description "shade generic definition"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "! <WindowMaterial:Shade>," -Expected $shadeHeader -Description "specialized shade header"
Assert-ExactOrderedEioRows -Lines $eioLines -Prefixes @("WindowConstruction,A BARE SINGLE SHADE WINDOW CONSTRUCTION,", "WindowConstruction,B EXTERIOR SHADE WINDOW CONSTRUCTION,", "WindowConstruction,C INTERIOR SHADE WINDOW CONSTRUCTION,", "WindowMaterial:Shade,") -Expected $orderedWindowRows -Description "window construction and shade occurrence"
Assert-NotContains -Text $eioText -Pattern "WindowMaterial:Shade,A DEFAULTED UNUSED SHADE," -Description "unused shade specialized occurrence"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE SHADE WINDOW HOST WALL," -Expected $hostSurfaceRow -Description "opaque host heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE EXTERIOR SHADE TEST WINDOW," -Expected $exteriorWindowSurfaceRow -Description "exterior-controlled fenestration heat-transfer surface"
Assert-UniqueExactEioRow -Lines $eioLines -Prefix "HeatTransfer Surface,DISTINCTIVE INTERIOR SHADE TEST WINDOW," -Expected $interiorWindowSurfaceRow -Description "interior-controlled fenestration heat-transfer surface"

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("window-material-shade.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "window-material-shade.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce window-material-shade.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing bounded Rust WindowMaterial:Shade inputs with EnergyPlus generic and specialized EIO evidence."
$output = & $cargo.Source run -p ep_cli --quiet -- compare window-material-shade $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "WindowMaterial:Shade comparison smoke failed."
}

$text = ($output -join [Environment]::NewLine)
Assert-Contains -Text $text -Pattern "Window Material Shade Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "window_runtime_claim: false" -Description "window runtime boundary"
Assert-Contains -Text $text -Pattern "window_optics_claim: false" -Description "window optics boundary"
Assert-Contains -Text $text -Pattern "window_thermal_claim: false" -Description "window thermal boundary"
Assert-Contains -Text $text -Pattern "daylighting_claim: false" -Description "daylighting boundary"
Assert-Contains -Text $text -Pattern "shading_control_claim: false" -Description "shading-control boundary"
Assert-Contains -Text $text -Pattern "fenestration_surface_claim: false" -Description "fenestration surface boundary"
Assert-Contains -Text $text -Pattern "construction_rating_claim: false" -Description "construction rating boundary"
Assert-Contains -Text $text -Pattern "shade_to_glass_distance_claim: false" -Description "shade-to-glass distance boundary"
Assert-Contains -Text $text -Pattern "opening_multiplier_claim: false" -Description "opening multiplier boundary"
Assert-Contains -Text $text -Pattern "airflow_permeability_claim: false" -Description "airflow permeability boundary"
Assert-Contains -Text $text -Pattern "visible_reflectance_claim: false" -Description "visible reflectance boundary"
Assert-Contains -Text $text -Pattern "infrared_transmittance_claim: false" -Description "infrared transmittance boundary"
Assert-Contains -Text $text -Pattern "nominal_resistance_claim: false" -Description "nominal resistance boundary"
Assert-Contains -Text $text -Pattern "broad_idf_declaration_order_claim: false" -Description "broad IDF declaration-order boundary"
Assert-Contains -Text $text -Pattern "arbitrary_idf_declaration_order_claim: false" -Description "arbitrary IDF declaration-order boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: energyplus-26.1-material-details-4R-shade-occurrence-3R-normalized-exact" -Description "source-format policy"
Assert-Contains -Text $text -Pattern "material_objects: 2" -Description "Rust shade definition count"
Assert-Contains -Text $text -Pattern "oracle_generic_material_rows: 2" -Description "matched oracle shade definition count"
Assert-Contains -Text $text -Pattern "oracle_material_detail_rows: 4" -Description "all oracle generic material-detail row count"
Assert-Contains -Text $text -Pattern "shade_occurrences: 2" -Description "Rust shade-layer occurrence count"
Assert-Contains -Text $text -Pattern "oracle_shade_occurrence_rows: 2" -Description "oracle shade-layer occurrence count"
Assert-Contains -Text $text -Pattern "shade_header_present: true" -Description "specialized shade header presence"
Assert-Contains -Text $text -Pattern "shade_header_rows: 1" -Description "specialized shade header count"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

$reportLines = @(
    "# Window material shade smoke report",
    "",
    "- Case: window_material_shade_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Window runtime/optics/thermal claims: false",
    "- Daylighting and shading-control claims: false",
    "- Fenestration surface and construction rating claims: false",
    "- Shade distance/opening/airflow claims: false",
    "- Visible-reflectance and infrared-transmittance claims: false",
    "- Nominal resistance claim: false",
    "- Broad/arbitrary IDF declaration-order claims: false",
    "",
    "## Exact oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader
) + $genericRows + @(
    $shadeHeader
) + $orderedWindowRows + @(
    $hostSurfaceRow,
    $exteriorWindowSurfaceRow,
    $interiorWindowSurfaceRow,
    "~~~",
    "",
    "The generic rows are one-per-definition echoes and include A DEFAULTED UNUSED SHADE. The specialized rows are construction-layer occurrences: B REUSED HIGH PRECISION SHADE appears exactly twice in exterior then interior construction order, while the unused A definition is absent.",
    "The generic and specialized tables do not expose visible reflectance, infrared transmittance, shade-to-glass distance, opening multipliers, airflow permeability, nominal resistance, or shading-control behavior.",
    "The three construction and three heat-transfer-surface rows are oracle-only fixture-integrity locks; their ratings, optics, runtime, and surface behavior are not Rust parity claims.",
    "",
    "## Bounded typed-input comparison",
    "",
    "~~~text",
    $text,
    "~~~",
    "",
    "The comparison checks the two typed shade definitions against generic {:.4R}/{:.3R} fields and the two exterior/interior shade-layer occurrences against specialized {:.3R} fields exposed by EnergyPlus 26.1.",
    "",
    "This report is non-blocking diagnostic-only static material evidence. It makes no shaded-window runtime, optics, thermal, daylighting, shading-control, construction-rating, surface, declaration-order, or conformance claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "WindowMaterial:Shade report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window material shade smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Window runtime/optics/thermal claims: false" -Description "report window boundary"
Assert-Contains -Text $reportText -Pattern "Daylighting and shading-control claims: false" -Description "report control boundary"
Assert-Contains -Text $reportText -Pattern "Shade distance/opening/airflow claims: false" -Description "report unexposed shade boundary"
Assert-Contains -Text $reportText -Pattern "Broad/arbitrary IDF declaration-order claims: false" -Description "report declaration-order boundary"
Assert-Contains -Text $reportText -Pattern ($genericRows -join [Environment]::NewLine) -Description "report exact generic shade rows"
Assert-Contains -Text $reportText -Pattern ($orderedWindowRows -join [Environment]::NewLine) -Description "report exact construction and shade occurrence rows"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"

Write-Host "WindowMaterial:Shade comparison smoke passed."
Write-Host "Diagnostic-only, nonblocking evidence; no window runtime, optics, thermal, control, rating, surface, declaration-order, or conformance claim."
Write-Host "Report: $ReportPath"
