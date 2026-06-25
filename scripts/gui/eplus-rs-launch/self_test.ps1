# Self-test helpers for eplus-rs-launch.ps1.

function Invoke-LauncherSelfTest {
    param(
        [string]$AppRoot,
        [string]$DefaultOracleRoot,
        [string]$DefaultIdf,
        [string]$DefaultWeather,
        [string]$LauncherScriptPath
    )
    $diagnosticArgs = New-LauncherRunArguments `
        -InputPath "input.epJSON" `
        -WeatherPath "" `
        -OutputDir "out" `
        -Mode "diagnostic" `
        -PartialPolicy "allow" `
        -OutputFormat "rust-native" `
        -TraceLevel "debug" `
        -FailOnWarning $true `
        -Overwrite $true `
        -OracleBaseline $false `
        -CompareOracle $false `
        -OracleRoot ""
    $baselineArgs = New-LauncherRunArguments `
        -InputPath "input.idf" `
        -WeatherPath "weather.epw" `
        -OutputDir "out" `
        -Mode "compatibility" `
        -PartialPolicy "deny" `
        -OutputFormat "rust-native" `
        -TraceLevel "normal" `
        -FailOnWarning $false `
        -Overwrite $true `
        -OracleBaseline $true `
        -CompareOracle $false `
        -OracleRoot "oracle-root"
    $compareArgs = New-LauncherRunArguments `
        -InputPath "input.idf" `
        -WeatherPath "weather.epw" `
        -OutputDir "out" `
        -Mode "compatibility" `
        -PartialPolicy "deny" `
        -OutputFormat "both" `
        -TraceLevel "detailed" `
        -FailOnWarning $false `
        -Overwrite $true `
        -OracleBaseline $true `
        -CompareOracle $true `
        -OracleRoot "oracle-root"
    foreach ($required in @("--mode", "diagnostic", "--partial", "allow", "--format", "rust-native", "--trace-level", "debug", "--fail-on-warning")) {
        if ($diagnosticArgs -notcontains $required) {
            throw "launcher self-test command builder missed $required"
        }
    }
    if (($diagnosticArgs -contains "-w") -or ($diagnosticArgs -contains "--oracle-baseline")) {
        throw "launcher self-test diagnostic command unexpectedly required weather or oracle baseline"
    }
    if (($baselineArgs -notcontains "-w") -or ($baselineArgs -notcontains "--oracle-baseline")) {
        throw "launcher self-test command builder missed oracle baseline weather/options"
    }
    foreach ($required in @("--compare-oracle", "--format", "both", "--trace-level", "detailed")) {
        if ($compareArgs -notcontains $required) {
            throw "launcher self-test command builder missed $required"
        }
    }

    $stateSamples = @(
        [pscustomobject]@{
            support = [pscustomobject]@{
                run_result_state = "run_blocked"
                status = "unsupported"
                runtime_class = "none"
                matched_capability_ids = @()
            }
            status = "unsupported"
            exit_code = 4
            oracle_status = "not-requested"
            compare_status = "not-requested"
        },
        [pscustomobject]@{
            support = [pscustomobject]@{
                run_result_state = "partial_supported_run"
                status = "supported-diagnostic-only"
                runtime_class = "ideal-loads-node-state-projection"
                matched_capability_ids = @("ideal_loads_no_oa_sensible")
            }
            status = "success"
            exit_code = 0
            oracle_status = "not-requested"
            compare_status = "not-requested"
        },
        [pscustomobject]@{
            support = [pscustomobject]@{
                run_result_state = "supported_compatibility_run"
                status = "supported-compatibility"
                runtime_class = "one-zone-heat-balance-compatibility"
                matched_capability_ids = @("official_1zone_uncontrolled_declared_heat_balance")
            }
            status = "success"
            exit_code = 0
            oracle_status = "generated"
            compare_status = "not-requested"
        },
        [pscustomobject]@{
            support = [pscustomobject]@{
                run_result_state = "run_blocked"
                status = "unsupported"
                runtime_class = "none"
                matched_capability_ids = @()
            }
            status = "unsupported"
            exit_code = 4
            oracle_status = "generated"
            compare_status = "skipped-rust-unsupported-or-oracle-missing"
        }
    )
    $presentations = @($stateSamples | ForEach-Object { Get-RunResultPresentation -Summary $_ })
    $expectedStates = @("run_blocked", "partial_supported_run", "supported_compatibility_run")
    foreach ($expected in $expectedStates) {
        if (@($presentations | Where-Object { $_.state_id -eq $expected }).Count -lt 1) {
            throw "launcher self-test missed state presentation $expected"
        }
    }
    $blockedOraclePresentation = @($presentations | Where-Object {
            $_.state_id -eq "run_blocked" -and
            $_.detail -match "oracle=generated" -and
            $_.detail -match "compare=skipped-rust-unsupported-or-oracle-missing"
        })
    if ($blockedOraclePresentation.Count -ne 1) {
        throw "launcher self-test missed blocked run oracle/compare presentation"
    }
    foreach ($presentation in $presentations) {
        if ($presentation.detail -notmatch "exit_code=") {
            throw "launcher self-test missed exit code presentation for $($presentation.state_id)"
        }
    }
    $phaseLine = Format-PhaseTimingLine -Phase ([pscustomobject]@{
            name = "support_assessment"
            engine = "ep_run"
            wall_seconds = 0.1234
            scope = "capability registry"
        })
    foreach ($required in @("support_assessment", "ep_run", "0.123s", "capability registry")) {
        if ($phaseLine -notmatch [regex]::Escape($required)) {
            throw "launcher self-test missed phase timing token $required"
        }
    }
    $settingsPath = Join-Path ([System.IO.Path]::GetTempPath()) ("eplus-rs-launch-settings-{0}.json" -f ([guid]::NewGuid()))
    $script:InputPath = "remembered-input.idf"
    $script:WeatherPath = "remembered-weather.epw"
    $script:OutputDir = "remembered-output"
    $script:OracleRoot = "remembered-oracle"
    $script:EplusRsExe = "remembered-eplus-rs.exe"
    $script:Mode = "diagnostic"
    $script:PartialPolicy = "allow"
    $script:OutputFormat = "both"
    $script:TraceLevel = "debug"
    $script:FailOnWarning = $true
    $script:OracleBaseline = $true
    $script:CompareOracle = $true
    $script:Overwrite = $false
    Save-LauncherSettings -Path $settingsPath
    $settings = Read-LauncherSettings -Path $settingsPath
    if ((Get-SettingValue -Settings $settings -Name "input_path" -Fallback "") -ne "remembered-input.idf") {
        throw "launcher self-test failed to save input path"
    }
    if (-not (Get-SettingBool -Settings $settings -Name "compare_oracle" -Fallback $false)) {
        throw "launcher self-test failed to save oracle compare option"
    }
    Remove-Item -LiteralPath $settingsPath -Force
    $scriptText = Get-Content -Encoding UTF8 -Raw -LiteralPath $LauncherScriptPath
    foreach ($required in @("Summary", "Diagnostics", "Support Report", "Results", "Oracle Compare", "Plots", "Logs", "Open Diagnostics", "Plot artifacts", "not a drop-in replacement")) {
        if ($scriptText -notmatch [regex]::Escape($required)) {
            throw "launcher self-test missed UI boundary token $required"
        }
    }
    foreach ($helper in @("Read-ArtifactPreview", "Read-PlotArtifactPreview")) {
        if ($null -eq (Get-Command $helper -ErrorAction SilentlyContinue)) {
            throw "launcher self-test missed artifact helper $helper"
        }
    }

    [pscustomobject]@{
        self_test = "passed"
        app_root = $AppRoot
        eplus_rs = Resolve-EplusRsExe
        oracle_root = $DefaultOracleRoot
        oracle_ready = Test-OracleRoot -Path $DefaultOracleRoot
        default_idf = $DefaultIdf
        default_weather = $DefaultWeather
        diagnostic_arguments = $diagnosticArgs
        baseline_arguments = $baselineArgs
        compare_arguments = $compareArgs
        state_presentations = $presentations
        phase_line = $phaseLine
    } | ConvertTo-Json -Depth 6
}
