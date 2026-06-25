# Core helpers for eplus-rs-launch.ps1.

function Resolve-FirstFile {
    param([string[]]$Candidates)
    foreach ($candidate in $Candidates) {
        if ([string]::IsNullOrWhiteSpace($candidate)) {
            continue
        }
        if (Test-Path -LiteralPath $candidate -PathType Leaf) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
    }
    return $null
}

function Resolve-FirstDirectory {
    param([string[]]$Candidates)
    foreach ($candidate in $Candidates) {
        if ([string]::IsNullOrWhiteSpace($candidate)) {
            continue
        }
        if (Test-Path -LiteralPath $candidate -PathType Container) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
    }
    return $null
}

function Test-EplusRsRunCli {
    param([string]$Path)
    if ([string]::IsNullOrWhiteSpace($Path) -or -not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        return $false
    }
    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $Path
    $psi.Arguments = "run"
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $true
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true

    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $psi
    try {
        if (-not $process.Start()) {
            return $false
        }
        if (-not $process.WaitForExit(5000)) {
            $process.Kill()
            return $false
        }
        $usageText = $process.StandardOutput.ReadToEnd() + $process.StandardError.ReadToEnd()
        return ($usageText -match "--mode compatibility\|diagnostic") -and
            ($usageText -match "--partial deny\|allow")
    }
    catch {
        return $false
    }
    finally {
        $process.Dispose()
    }
}

function Resolve-EplusRsExe {
    $command = Get-Command eplus-rs -ErrorAction SilentlyContinue
    $commandPath = if ($null -ne $command) { $command.Source } else { $null }
    foreach ($candidate in @(
        (Join-Path $AppRoot "bin\eplus-rs.exe"),
        (Join-Path $AppRoot "target\debug\eplus-rs.exe"),
        (Join-Path $AppRoot "target\release\eplus-rs.exe"),
        $commandPath
    )) {
        if ([string]::IsNullOrWhiteSpace($candidate)) {
            continue
        }
        $resolved = Resolve-Path -LiteralPath $candidate -ErrorAction SilentlyContinue
        if ($null -eq $resolved) {
            continue
        }
        $resolvedPath = $resolved.Path
        if (Test-EplusRsRunCli -Path $resolvedPath) {
            return $resolvedPath
        }
    }
    return $null
}

function Resolve-OracleRoot {
    Resolve-FirstDirectory -Candidates @(
        $env:RUSTED_ENERGYPLUS_ORACLE_ROOT,
        (Join-Path $AppRoot "oracle\energyplus\26.1.0"),
        (Join-Path $AppRoot ".runtime\energyplus\26.1.0")
    )
}

function Get-LauncherDefaultPaths {
    $oracleRoot = Resolve-OracleRoot
    $idf = if (-not [string]::IsNullOrWhiteSpace($oracleRoot)) {
        Resolve-FirstFile -Candidates @(
            (Join-Path $oracleRoot "ExampleFiles\1ZoneUncontrolled.idf")
        )
    }
    else {
        $null
    }
    $weather = if (-not [string]::IsNullOrWhiteSpace($oracleRoot)) {
        Resolve-FirstFile -Candidates @(
            (Join-Path $oracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw")
        )
    }
    else {
        $null
    }

    [pscustomobject]@{
        OracleRoot = $oracleRoot
        Idf = $idf
        Weather = $weather
    }
}

function Test-OracleRoot {
    param([string]$Path)
    if ([string]::IsNullOrWhiteSpace($Path)) {
        return $false
    }
    return (Test-Path -LiteralPath (Join-Path $Path "energyplus.exe") -PathType Leaf) -and
        (Test-Path -LiteralPath (Join-Path $Path "ConvertInputFormat.exe") -PathType Leaf)
}

function Test-LeafPath {
    param([string]$Path)
    return (-not [string]::IsNullOrWhiteSpace($Path)) -and
        (Test-Path -LiteralPath $Path -PathType Leaf)
}

function Test-ContainerPath {
    param([string]$Path)
    return (-not [string]::IsNullOrWhiteSpace($Path)) -and
        (Test-Path -LiteralPath $Path -PathType Container)
}

function Get-LauncherSettingsPath {
    $settingsRoot = Join-Path ([Environment]::GetFolderPath("ApplicationData")) "RustedEnergyPlus"
    return Join-Path $settingsRoot "launcher-settings.json"
}

function Read-LauncherSettings {
    param([string]$Path)
    if ([string]::IsNullOrWhiteSpace($Path) -or -not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        return $null
    }
    try {
        return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path | ConvertFrom-Json
    }
    catch {
        return $null
    }
}

function Get-SettingValue {
    param(
        [object]$Settings,
        [string]$Name,
        [string]$Fallback
    )
    $value = Get-ObjectPropertyValue -Object $Settings -Name $Name -Default $null
    if ([string]::IsNullOrWhiteSpace([string]$value)) {
        return $Fallback
    }
    return [string]$value
}

function Get-SettingBool {
    param(
        [object]$Settings,
        [string]$Name,
        [bool]$Fallback
    )
    $value = Get-ObjectPropertyValue -Object $Settings -Name $Name -Default $null
    if ($null -eq $value) {
        return $Fallback
    }
    return [bool]$value
}

function Quote-ProcessArgument {
    param([string]$Value)
    if ($Value.Length -eq 0) {
        return '""'
    }
    if ($Value -notmatch '[\s"]') {
        return $Value
    }
    $builder = New-Object System.Text.StringBuilder
    [void]$builder.Append('"')
    $backslashes = 0
    foreach ($character in $Value.ToCharArray()) {
        if ($character -eq '\') {
            $backslashes += 1
            continue
        }
        if ($character -eq '"') {
            [void]$builder.Append('\' * (($backslashes * 2) + 1))
            [void]$builder.Append('"')
            $backslashes = 0
            continue
        }
        if ($backslashes -gt 0) {
            [void]$builder.Append('\' * $backslashes)
            $backslashes = 0
        }
        [void]$builder.Append($character)
    }
    if ($backslashes -gt 0) {
        [void]$builder.Append('\' * ($backslashes * 2))
    }
    [void]$builder.Append('"')
    return $builder.ToString()
}

function Get-ObjectPropertyValue {
    param(
        [object]$Object,
        [string]$Name,
        [object]$Default = $null
    )
    if ($null -eq $Object) {
        return $Default
    }
    $property = $Object.PSObject.Properties[$Name]
    if ($null -eq $property) {
        return $Default
    }
    return $property.Value
}

function Get-SummarySupportValue {
    param(
        [object]$Summary,
        [string]$Name,
        [object]$Default = $null
    )
    $support = Get-ObjectPropertyValue -Object $Summary -Name "support" -Default $null
    if ($null -ne $support) {
        $value = Get-ObjectPropertyValue -Object $support -Name $Name -Default $null
        if ($null -ne $value) {
            return $value
        }
    }
    if ($Name -eq "status") {
        return Get-ObjectPropertyValue -Object $Summary -Name "support_status" -Default $Default
    }
    return Get-ObjectPropertyValue -Object $Summary -Name $Name -Default $Default
}

function Test-LauncherWeatherRequired {
    param(
        [string]$Mode,
        [bool]$OracleBaseline,
        [bool]$CompareOracle
    )
    return ($Mode -eq "compatibility") -or $OracleBaseline -or $CompareOracle
}

function New-LauncherRunArguments {
    param(
        [string]$InputPath,
        [string]$WeatherPath,
        [string]$OutputDir,
        [string]$Mode,
        [string]$PartialPolicy,
        [string]$OutputFormat,
        [string]$TraceLevel,
        [bool]$FailOnWarning,
        [bool]$Overwrite,
        [bool]$OracleBaseline,
        [bool]$CompareOracle,
        [string]$OracleRoot
    )
    $arguments = @(
        "run",
        $InputPath,
        "-d",
        $OutputDir,
        "--mode",
        $Mode,
        "--partial",
        $PartialPolicy,
        "--format",
        $OutputFormat,
        "--trace-level",
        $TraceLevel
    )
    if (-not [string]::IsNullOrWhiteSpace($WeatherPath)) {
        $arguments += @("-w", $WeatherPath)
    }
    if ($FailOnWarning) {
        $arguments += "--fail-on-warning"
    }
    if ($Overwrite) {
        $arguments += "--overwrite"
    }
    if ($CompareOracle) {
        $arguments += "--compare-oracle"
    }
    elseif ($OracleBaseline) {
        $arguments += "--oracle-baseline"
    }
    if (-not [string]::IsNullOrWhiteSpace($OracleRoot)) {
        $arguments += @("--oracle-root", $OracleRoot)
    }
    return $arguments
}

function Get-RunResultPresentation {
    param([object]$Summary)
    $status = Get-ObjectPropertyValue -Object $Summary -Name "status" -Default "unknown"
    $exitCode = Get-ObjectPropertyValue -Object $Summary -Name "exit_code" -Default "unknown"
    $oracleStatus = Get-ObjectPropertyValue -Object $Summary -Name "oracle_status" -Default "not-run"
    $compareStatus = Get-ObjectPropertyValue -Object $Summary -Name "compare_status" -Default "not-run"
    $runState = Get-SummarySupportValue -Summary $Summary -Name "run_result_state" -Default "unknown"
    $supportStatus = Get-SummarySupportValue -Summary $Summary -Name "status" -Default "unknown"
    $runtimeClass = Get-SummarySupportValue -Summary $Summary -Name "runtime_class" -Default "unknown"
    $capabilityIds = Get-SummarySupportValue -Summary $Summary -Name "matched_capability_ids" -Default @()
    $capabilityText = if (@($capabilityIds).Count -gt 0) {
        (@($capabilityIds) -join ", ")
    }
    else {
        "none"
    }

    $title = "Run status unknown"
    $color = "DimGray"
    switch ($runState) {
        "run_blocked" {
            $title = "Cannot run"
            $color = "Firebrick"
        }
        "partial_supported_run" {
            $title = "Partial supported run"
            $color = "DarkGoldenrod"
        }
        "supported_compatibility_run" {
            $title = "Supported compatibility run"
            $color = "ForestGreen"
        }
    }

    [pscustomobject]@{
        state_id = $runState
        title = $title
        color = $color
        detail = "status=$status; exit_code=$exitCode; support=$supportStatus; runtime=$runtimeClass; oracle=$oracleStatus; compare=$compareStatus; capabilities=$capabilityText; conformance_claim=false"
    }
}

function Format-DiagnosticLine {
    param([object]$Diagnostic)
    $severity = Get-ObjectPropertyValue -Object $Diagnostic -Name "severity" -Default "unknown"
    $code = Get-ObjectPropertyValue -Object $Diagnostic -Name "code" -Default "Diagnostic"
    $stage = Get-ObjectPropertyValue -Object $Diagnostic -Name "stage" -Default "unknown"
    $message = Get-ObjectPropertyValue -Object $Diagnostic -Name "message" -Default ""
    return "$severity [$code] ${stage}: $message"
}

function Get-RunSummaryPhases {
    param([object]$Summary)
    $timing = Get-ObjectPropertyValue -Object $Summary -Name "timing" -Default $null
    if ($null -eq $timing) {
        return @()
    }
    return @(Get-ObjectPropertyValue -Object $timing -Name "phases" -Default @())
}

function Format-PhaseTimingLine {
    param([object]$Phase)
    $name = Get-ObjectPropertyValue -Object $Phase -Name "name" -Default "unknown"
    $engine = Get-ObjectPropertyValue -Object $Phase -Name "engine" -Default "unknown"
    $scope = Get-ObjectPropertyValue -Object $Phase -Name "scope" -Default ""
    $seconds = Get-ObjectPropertyValue -Object $Phase -Name "wall_seconds" -Default $null
    $secondsText = if ($null -ne $seconds) {
        "{0:N3}s" -f ([double]$seconds)
    }
    else {
        "n/a"
    }
    if ([string]::IsNullOrWhiteSpace($scope)) {
        return "$name [$engine] $secondsText"
    }
    return "$name [$engine] $secondsText - $scope"
}

function Save-LauncherSettings {
    param([string]$Path)
    if ([string]::IsNullOrWhiteSpace($Path)) {
        return
    }
    $settingsDir = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($settingsDir)) {
        New-Item -ItemType Directory -Force -Path $settingsDir | Out-Null
    }
    [pscustomobject]@{
        schema_version = 1
        input_path = $script:InputPath
        weather_path = $script:WeatherPath
        output_dir = $script:OutputDir
        oracle_root = $script:OracleRoot
        eplus_rs_exe = if ($null -ne $script:EplusRsExe) { $script:EplusRsExe } else { "" }
        mode = $script:Mode
        partial_policy = $script:PartialPolicy
        output_format = $script:OutputFormat
        trace_level = $script:TraceLevel
        fail_on_warning = $script:FailOnWarning
        oracle_baseline = $script:OracleBaseline
        compare_oracle = $script:CompareOracle
        overwrite = $script:Overwrite
    } | ConvertTo-Json -Depth 4 | Set-Content -Encoding UTF8 -LiteralPath $Path
}

