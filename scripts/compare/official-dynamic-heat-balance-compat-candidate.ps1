[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "official_1zone_uncontrolled_dynamic_conformance_candidate_001"
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\official-dynamic-compat-candidate\26.1.0"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$CompareRoot = Join-Path $CaseOutputRoot "compare"

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

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch [regex]::Escape($Pattern)) {
        Write-Host $Text
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required official dynamic conformance file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running official 1ZoneUncontrolled dynamic heat-balance conformance gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance heat-balance-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Official dynamic heat-balance conformance gate failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Conformance Heat Balance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "conformance claim"
Assert-Contains -Text $text -Pattern "zone_air_algorithm_lane: compatibility-candidate" -Description "compatibility lane"
Assert-Contains -Text $text -Pattern "conformance_promotion_allowed: true" -Description "promotion eligibility"
Assert-Contains -Text $text -Pattern "surface_iteration_count: 20" -Description "surface iteration count"
Assert-Contains -Text $text -Pattern "ctf_initial_history_policy: energyplus-surf-initial" -Description "CTF history policy"
Assert-Contains -Text $text -Pattern "status: pass" -Description "gate status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$digestPath = Join-Path $CompareRoot "compare-digest.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
Assert-FileExists -Path $summaryPath -Description "official dynamic summary"
Assert-FileExists -Path $digestPath -Description "official dynamic digest"
Assert-FileExists -Path $reportPath -Description "official dynamic report"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "Official dynamic candidate must retain conformance_claim=true"
}
if ($summary.gate.blocking -ne $true) {
    throw "Official dynamic candidate gate must be blocking"
}
if ($summary.status -ne "pass") {
    throw "Unexpected official dynamic conformance status: $($summary.status)"
}
if (@($summary.failure_reasons).Count -ne 0) {
    throw "Official dynamic conformance should not report failure reasons"
}
if ($summary.zone_air_algorithm -ne "energyplus-heat-balance-compat-candidate") {
    throw "Unexpected zone_air_algorithm: $($summary.zone_air_algorithm)"
}
if ($summary.zone_air_algorithm_lane -ne "compatibility-candidate") {
    throw "Unexpected algorithm lane: $($summary.zone_air_algorithm_lane)"
}
if ($summary.conformance_promotion_allowed -ne $true) {
    throw "Compatibility candidate must be promotion-eligible"
}
if ($summary.ctf_seed.policy -ne "all-eio") {
    throw "Expected all-EIO CTF seed policy, got $($summary.ctf_seed.policy)"
}
if ($summary.heat_balance_warmup.enabled -ne $true) {
    throw "Official dynamic candidate must run model warmup"
}
if ($summary.heat_balance_warmup.day_count -ne 20) {
    throw "Unexpected Rust warmup day count: $($summary.heat_balance_warmup.day_count)"
}
if ($summary.heat_balance_warmup.oracle_run_period_day_count -ne 20) {
    throw "Unexpected oracle warmup day count: $($summary.heat_balance_warmup.oracle_run_period_day_count)"
}
if ($summary.heat_balance_warmup.day_count_delta -ne 0) {
    throw "Warmup day count delta should be zero, got $($summary.heat_balance_warmup.day_count_delta)"
}
if ($summary.surface_iteration_count -ne 20) {
    throw "Unexpected surface_iteration_count: $($summary.surface_iteration_count)"
}
if ($summary.ctf_initial_history_policy -ne "energyplus-surf-initial") {
    throw "Unexpected CTF history policy: $($summary.ctf_initial_history_policy)"
}

$conformanceOutputs = @($summary.outputs | Where-Object { $_.level -eq "conformance" })
$diagnosticOutputs = @($summary.outputs | Where-Object { $_.level -eq "diagnostic" })
if ($conformanceOutputs.Count -ne 116) {
    throw "Expected 116 conformance-level outputs, got $($conformanceOutputs.Count)"
}
if ($diagnosticOutputs.Count -ne 0) {
    throw "Expected zero diagnostic outputs, got $($diagnosticOutputs.Count)"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq "Site Outdoor Air Drybulb Temperature" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })) {
    throw "Weather dry-bulb conformance series missing"
}
$wetbulbSeries = $summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq "Site Outdoor Air Wetbulb Temperature" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $wetbulbSeries) {
    throw "Weather wet-bulb conformance series missing"
}
if ([double]$wetbulbSeries.max_abs_delta_c -gt 0.00001) {
    throw "Weather wet-bulb max_abs_delta_c exceeds 1e-5 C: $($wetbulbSeries.max_abs_delta_c)"
}
if ([double]$wetbulbSeries.rmse_delta_c -gt 0.00001) {
    throw "Weather wet-bulb rmse_delta_c exceeds 1e-5 C: $($wetbulbSeries.rmse_delta_c)"
}
foreach ($weatherVariable in @("Site Sky Temperature", "Site Horizontal Infrared Radiation Rate per Area")) {
    $weatherSeries = $summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq $weatherVariable -and $_.output.class -eq "weather" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
    if (-not $weatherSeries) {
        throw "Weather conformance series missing: $weatherVariable"
    }
    if ([double]$weatherSeries.max_abs_delta_c -gt 0.00001) {
        throw "Weather max_abs_delta_c exceeds 1e-5 for ${weatherVariable}: $($weatherSeries.max_abs_delta_c)"
    }
    if ([double]$weatherSeries.rmse_delta_c -gt 0.00001) {
        throw "Weather rmse_delta_c exceeds 1e-5 for ${weatherVariable}: $($weatherSeries.rmse_delta_c)"
    }
}
$rainSeries = $summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq "Site Rain Status" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $rainSeries) {
    throw "Weather rain-status conformance series missing"
}
if ([double]$rainSeries.max_abs_delta_c -gt 0.000001) {
    throw "Weather rain-status max_abs_delta_c exceeds 1e-6: $($rainSeries.max_abs_delta_c)"
}
if ([double]$rainSeries.rmse_delta_c -gt 0.000001) {
    throw "Weather rain-status rmse_delta_c exceeds 1e-6: $($rainSeries.rmse_delta_c)"
}
foreach ($surfaceWeatherVariable in @(
    "Surface Outside Face Outdoor Air Drybulb Temperature",
    "Surface Outside Face Outdoor Air Wetbulb Temperature",
    "Surface Outside Face Outdoor Air Wind Speed",
    "Surface Outside Face Outdoor Air Wind Direction"
)) {
    $surfaceWeatherSeries = $summary.series | Where-Object { $_.output.key -eq "ZN001:ROOF001" -and $_.output.variable -eq $surfaceWeatherVariable -and $_.output.class -eq "weather" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
    if (-not $surfaceWeatherSeries) {
        throw "Roof surface-local weather conformance series missing: $surfaceWeatherVariable"
    }
    if ([double]$surfaceWeatherSeries.max_abs_delta_c -gt 0.00001) {
        throw "Roof surface-local weather max_abs_delta_c exceeds 1e-5 for ${surfaceWeatherVariable}: $($surfaceWeatherSeries.max_abs_delta_c)"
    }
    if ([double]$surfaceWeatherSeries.rmse_delta_c -gt 0.00001) {
        throw "Roof surface-local weather rmse_delta_c exceeds 1e-5 for ${surfaceWeatherVariable}: $($surfaceWeatherSeries.rmse_delta_c)"
    }
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZONE ONE" -and $_.output.variable -eq "Zone Mean Air Temperature" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })) {
    throw "Zone Mean Air Temperature conformance series missing"
}
$allSurfaceKeys = @("ZN001:WALL001", "ZN001:WALL002", "ZN001:WALL003", "ZN001:WALL004", "ZN001:FLR001", "ZN001:ROOF001")
$wallRoofSurfaceKeys = @("ZN001:WALL001", "ZN001:WALL002", "ZN001:WALL003", "ZN001:WALL004", "ZN001:ROOF001")
$surfaceFluxSeries = @($summary.series | Where-Object { $_.output.class -eq "surface-flux-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })
if ($surfaceFluxSeries.Count -ne 22) {
    throw "Expected 22 surface-flux-state conformance series, got $($surfaceFluxSeries.Count)"
}
foreach ($series in $surfaceFluxSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.005) {
        throw "Surface flux-state max_abs_delta_c exceeds 0.005 W/m2 for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.0015) {
        throw "Surface flux-state rmse_delta_c exceeds 0.0015 W/m2 for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$incidentSolarVariables = @(
    "Surface Outside Face Incident Solar Radiation Rate per Area",
    "Surface Outside Face Incident Beam Solar Radiation Rate per Area"
)
$incidentSolarSeries = @($summary.series | Where-Object {
        $wallRoofSurfaceKeys -contains $_.output.key `
            -and $incidentSolarVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-solar-flux-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($incidentSolarSeries.Count -ne 10) {
    throw "Expected 10 named wall/roof incident total/beam solar conformance series, got $($incidentSolarSeries.Count)"
}
foreach ($series in $incidentSolarSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.02) {
        throw "Incident total/beam solar max_abs_delta_c exceeds 0.02 W/m2 for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.003) {
        throw "Incident total/beam solar rmse_delta_c exceeds 0.003 W/m2 for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$absorbedSolarRateSeries = @($summary.series | Where-Object {
        $wallRoofSurfaceKeys -contains $_.output.key `
            -and $_.output.variable -eq "Surface Outside Face Solar Radiation Heat Gain Rate" `
            -and $_.output.class -eq "surface-solar-rate-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($absorbedSolarRateSeries.Count -ne 5) {
    throw "Expected five named wall/roof absorbed solar heat gain rate conformance series, got $($absorbedSolarRateSeries.Count)"
}
foreach ($series in $absorbedSolarRateSeries) {
    if ([double]$series.max_abs_delta_c -gt 2.5) {
        throw "Absorbed solar heat gain rate max_abs_delta_c exceeds 2.5 W for $($series.output.key): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.5) {
        throw "Absorbed solar heat gain rate rmse_delta_c exceeds 0.5 W for $($series.output.key): $($series.rmse_delta_c)"
    }
}
$absorbedSolarFluxSeries = @($summary.series | Where-Object {
        $_.output.key -eq "ZN001:ROOF001" `
            -and $_.output.variable -eq "Surface Outside Face Solar Radiation Heat Gain Rate per Area" `
            -and $_.output.class -eq "surface-solar-flux-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($absorbedSolarFluxSeries.Count -ne 1) {
    throw "Expected one roof absorbed solar heat gain per-area conformance series, got $($absorbedSolarFluxSeries.Count)"
}
foreach ($series in $absorbedSolarFluxSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.02) {
        throw "Roof absorbed solar heat gain per-area max_abs_delta_c exceeds 0.02 W/m2: $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.003) {
        throw "Roof absorbed solar heat gain per-area rmse_delta_c exceeds 0.003 W/m2: $($series.rmse_delta_c)"
    }
}
$incidentDiffuseVariables = @(
    "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area",
    "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area"
)
$incidentDiffuseSeries = @($summary.series | Where-Object {
        $wallRoofSurfaceKeys -contains $_.output.key `
            -and $incidentDiffuseVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-flux-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($incidentDiffuseSeries.Count -ne 10) {
    throw "Expected 10 named wall/roof incident sky/ground diffuse conformance series, got $($incidentDiffuseSeries.Count)"
}
foreach ($series in $incidentDiffuseSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.000001) {
        throw "Incident sky/ground diffuse max_abs_delta_c exceeds 1e-6 W/m2 for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.000001) {
        throw "Incident sky/ground diffuse rmse_delta_c exceeds 1e-6 W/m2 for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$surfaceCoefficientSeries = @($summary.series | Where-Object {
        (
            ($allSurfaceKeys -contains $_.output.key -and $_.output.variable -eq "Surface Inside Face Convection Heat Transfer Coefficient") `
                -or ($wallRoofSurfaceKeys -contains $_.output.key -and $_.output.variable -eq "Surface Outside Face Convection Heat Transfer Coefficient")
        ) `
            -and $_.output.class -eq "surface-coefficient-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($surfaceCoefficientSeries.Count -ne 11) {
    throw "Expected 11 inside/outside convection coefficient conformance series, got $($surfaceCoefficientSeries.Count)"
}
foreach ($series in $surfaceCoefficientSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.05) {
        throw "Surface convection coefficient max_abs_delta_c exceeds 0.05 W/m2-K for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.001) {
        throw "Surface convection coefficient rmse_delta_c exceeds 0.001 W/m2-K for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$exteriorSourceVariables = @(
    "Surface Outside Face Convection Heat Gain Rate",
    "Surface Outside Face Net Thermal Radiation Heat Gain Rate"
)
$exteriorSourceFluxVariables = @(
    "Surface Outside Face Convection Heat Gain Rate per Area",
    "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area"
)
$surfaceExteriorRateSeries = @($summary.series | Where-Object {
        $wallRoofSurfaceKeys -contains $_.output.key `
            -and $exteriorSourceVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-exterior-rate-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($surfaceExteriorRateSeries.Count -ne 10) {
    throw "Expected 10 wall/roof exterior convection/radiation rate conformance series, got $($surfaceExteriorRateSeries.Count)"
}
foreach ($series in $surfaceExteriorRateSeries) {
    if ([double]$series.max_abs_delta_c -gt 2.5) {
        throw "Exterior convection/radiation rate max_abs_delta_c exceeds 2.5 W for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.6) {
        throw "Exterior convection/radiation rate rmse_delta_c exceeds 0.6 W for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$surfaceExteriorFluxSeries = @($summary.series | Where-Object {
        $_.output.key -eq "ZN001:ROOF001" `
            -and $exteriorSourceFluxVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-exterior-flux-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($surfaceExteriorFluxSeries.Count -ne 2) {
    throw "Expected two roof exterior convection/radiation per-area conformance series, got $($surfaceExteriorFluxSeries.Count)"
}
foreach ($series in $surfaceExteriorFluxSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.011) {
        throw "Roof exterior convection/radiation per-area max_abs_delta_c exceeds 0.011 W/m2 for $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.002) {
        throw "Roof exterior convection/radiation per-area rmse_delta_c exceeds 0.002 W/m2 for $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$insideSourceVariables = @(
    "Surface Inside Face Convection Heat Gain Rate",
    "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate"
)
$insideSourceSeries = @($summary.series | Where-Object {
        $allSurfaceKeys -contains $_.output.key `
            -and $insideSourceVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($insideSourceSeries.Count -ne 12) {
    throw "Expected 12 inside convection/radiation source conformance series, got $($insideSourceSeries.Count)"
}
foreach ($series in $insideSourceSeries) {
    if ([double]$series.max_abs_delta_c -gt 1.0) {
        throw "Inside convection/radiation source max_abs_delta_c exceeds 1.0 W for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.35) {
        throw "Inside convection/radiation source rmse_delta_c exceeds 0.35 W for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$surfaceAggregateSeries = @($summary.series | Where-Object { $_.output.class -eq "surface-aggregate-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })
if ($surfaceAggregateSeries.Count -ne 4) {
    throw "Expected four surface-aggregate-state conformance series, got $($surfaceAggregateSeries.Count)"
}
foreach ($series in $surfaceAggregateSeries) {
    if ([double]$series.max_abs_delta_c -gt 1.2) {
        throw "Zone opaque aggregate conduction max_abs_delta_c exceeds 1.2 W for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.2) {
        throw "Zone opaque aggregate conduction rmse_delta_c exceeds 0.2 W for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$storageSeries = $summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Heat Storage Rate" -and $_.output.class -eq "surface-storage-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $storageSeries) {
    throw "Floor storage conformance series missing"
}
if ([double]$storageSeries.max_abs_delta_c -gt 1.2) {
    throw "Floor storage max_abs_delta_c exceeds 1.2 W: $($storageSeries.max_abs_delta_c)"
}
if ([double]$storageSeries.rmse_delta_c -gt 0.35) {
    throw "Floor storage rmse_delta_c exceeds 0.35 W: $($storageSeries.rmse_delta_c)"
}
$storageFluxSeries = $summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Heat Storage Rate per Area" -and $_.output.class -eq "surface-storage-flux-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $storageFluxSeries) {
    throw "Floor storage per-area conformance series missing"
}
if ([double]$storageFluxSeries.max_abs_delta_c -gt 0.005) {
    throw "Floor storage per-area max_abs_delta_c exceeds 0.005 W/m2: $($storageFluxSeries.max_abs_delta_c)"
}
if ([double]$storageFluxSeries.rmse_delta_c -gt 0.001) {
    throw "Floor storage per-area rmse_delta_c exceeds 0.001 W/m2: $($storageFluxSeries.rmse_delta_c)"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Heat Balance Conformance Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "comparison_class: conformance" -Description "markdown comparison class"
Assert-Contains -Text $reportText -Pattern "conformance_claim: true" -Description "markdown conformance claim"
Assert-Contains -Text $reportText -Pattern "gate_blocking: true" -Description "markdown blocking gate"
Assert-Contains -Text $reportText -Pattern "Site Outdoor Air Wetbulb Temperature / hourly / weather / eso / conformance" -Description "wet-bulb weather conformance output"
Assert-Contains -Text $reportText -Pattern "Site Rain Status / hourly / weather / eso / conformance" -Description "rain-status weather conformance output"
Assert-Contains -Text $reportText -Pattern "Site Sky Temperature / hourly / weather / eso / conformance" -Description "sky temperature weather conformance output"
Assert-Contains -Text $reportText -Pattern "Site Horizontal Infrared Radiation Rate per Area / hourly / weather / eso / conformance" -Description "horizontal infrared weather conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Outdoor Air Drybulb Temperature / hourly / weather / eso / conformance" -Description "roof local dry-bulb conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Outdoor Air Wetbulb Temperature / hourly / weather / eso / conformance" -Description "roof local wet-bulb conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Outdoor Air Wind Speed / hourly / weather / eso / conformance" -Description "roof local wind-speed conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Outdoor Air Wind Direction / hourly / weather / eso / conformance" -Description "roof local wind-direction conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Conduction Heat Transfer Rate per Area / hourly / surface-flux-state / eso / conformance" -Description "surface conduction per-area conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Solar Radiation Rate per Area / hourly / surface-solar-flux-state / eso / conformance" -Description "incident total solar conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Beam Solar Radiation Rate per Area / hourly / surface-solar-flux-state / eso / conformance" -Description "incident beam solar conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Solar Radiation Heat Gain Rate / hourly / surface-solar-rate-state / eso / conformance" -Description "absorbed solar heat gain rate conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Solar Radiation Heat Gain Rate per Area / hourly / surface-solar-flux-state / eso / conformance" -Description "absorbed solar heat gain per-area conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Convection Heat Transfer Coefficient / hourly / surface-coefficient-state / eso / conformance" -Description "inside convection coefficient conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Convection Heat Transfer Coefficient / hourly / surface-coefficient-state / eso / conformance" -Description "outside convection coefficient conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Convection Heat Gain Rate / hourly / surface-exterior-rate-state / eso / conformance" -Description "outside convection heat-gain rate conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Net Thermal Radiation Heat Gain Rate / hourly / surface-exterior-rate-state / eso / conformance" -Description "outside net thermal radiation heat-gain rate conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Convection Heat Gain Rate per Area / hourly / surface-exterior-flux-state / eso / conformance" -Description "outside convection heat-gain per-area conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area / hourly / surface-exterior-flux-state / eso / conformance" -Description "outside net thermal radiation heat-gain per-area conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Convection Heat Gain Rate / hourly / surface-state / eso / conformance" -Description "inside convection source conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate / hourly / surface-state / eso / conformance" -Description "inside radiation source conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area / hourly / surface-flux-state / eso / conformance" -Description "incident sky diffuse conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area / hourly / surface-flux-state / eso / conformance" -Description "incident ground diffuse conformance output"
Assert-Contains -Text $reportText -Pattern "Zone Opaque Surface Outside Faces Conduction Rate / hourly / surface-aggregate-state / eso / conformance" -Description "zone opaque aggregate conduction conformance output"
Assert-Contains -Text $reportText -Pattern "Zone Opaque Surface Outside Faces Conduction Heat Gain Rate / hourly / surface-aggregate-state / eso / conformance" -Description "zone opaque aggregate outside gain conformance output"
Assert-Contains -Text $reportText -Pattern "Zone Opaque Surface Outside Faces Conduction Heat Loss Rate / hourly / surface-aggregate-state / eso / conformance" -Description "zone opaque aggregate outside loss conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Heat Storage Rate / hourly / surface-storage-state / eso / conformance" -Description "surface storage conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Heat Storage Rate per Area / hourly / surface-storage-flux-state / eso / conformance" -Description "surface storage per-area conformance output"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "markdown status"

Write-Host "Official dynamic heat-balance conformance gate passed."
