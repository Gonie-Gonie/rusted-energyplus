[CmdletBinding()]
param(
    [switch]$SelfTest
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptRoot = $PSScriptRoot
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptRoot "..")).Path
$AppRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptsRoot "..")).Path

. (Join-Path $ScriptRoot "eplus-rs-launch\core.ps1")
. (Join-Path $ScriptRoot "eplus-rs-launch\ui.ps1")

$LauncherDefaults = Get-LauncherDefaultPaths
$DefaultOracleRoot = $LauncherDefaults.OracleRoot
$DefaultIdf = $LauncherDefaults.Idf
$DefaultWeather = $LauncherDefaults.Weather

if ($SelfTest) {
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
    $scriptText = Get-Content -Encoding UTF8 -Raw -LiteralPath $PSCommandPath
    foreach ($required in @("Summary", "Diagnostics", "Support Report", "Results", "Oracle Compare", "Logs", "Open Diagnostics", "not a drop-in replacement")) {
        if ($scriptText -notmatch [regex]::Escape($required)) {
            throw "launcher self-test missed UI boundary token $required"
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
    exit 0
}

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
[System.Windows.Forms.Application]::EnableVisualStyles()

$script:EplusRsExe = Resolve-EplusRsExe
$script:InputPath = if ($null -ne $DefaultIdf) { $DefaultIdf } else { "" }
$script:WeatherPath = if ($null -ne $DefaultWeather) { $DefaultWeather } else { "" }
$script:OutputDir = Join-Path $AppRoot ".runtime\ep-launch-output"
$script:OracleRoot = if ($null -ne $DefaultOracleRoot) { $DefaultOracleRoot } else { "" }
$script:Mode = "compatibility"
$script:PartialPolicy = "deny"
$script:OutputFormat = "rust-native"
$script:TraceLevel = "normal"
$script:FailOnWarning = $false
$script:OracleBaseline = $false
$script:CompareOracle = Test-OracleRoot -Path $script:OracleRoot
$script:Overwrite = $true
$script:RefreshingUi = $false
$script:CurrentProcess = $null
$script:StdoutTask = $null
$script:StderrTask = $null
$script:StdoutPath = ""
$script:StderrPath = ""
$script:LauncherSettingsPath = Get-LauncherSettingsPath

$settings = Read-LauncherSettings -Path $script:LauncherSettingsPath
if ($null -ne $settings) {
    $script:InputPath = Get-SettingValue -Settings $settings -Name "input_path" -Fallback $script:InputPath
    $script:WeatherPath = Get-SettingValue -Settings $settings -Name "weather_path" -Fallback $script:WeatherPath
    $script:OutputDir = Get-SettingValue -Settings $settings -Name "output_dir" -Fallback $script:OutputDir
    $script:OracleRoot = Get-SettingValue -Settings $settings -Name "oracle_root" -Fallback $script:OracleRoot
    $savedExe = Get-SettingValue -Settings $settings -Name "eplus_rs_exe" -Fallback ""
    if (-not [string]::IsNullOrWhiteSpace($savedExe)) {
        $script:EplusRsExe = $savedExe
    }
    $script:Mode = Get-SettingValue -Settings $settings -Name "mode" -Fallback $script:Mode
    $script:PartialPolicy = Get-SettingValue -Settings $settings -Name "partial_policy" -Fallback $script:PartialPolicy
    $script:OutputFormat = Get-SettingValue -Settings $settings -Name "output_format" -Fallback $script:OutputFormat
    $script:TraceLevel = Get-SettingValue -Settings $settings -Name "trace_level" -Fallback $script:TraceLevel
    $script:FailOnWarning = Get-SettingBool -Settings $settings -Name "fail_on_warning" -Fallback $script:FailOnWarning
    $script:OracleBaseline = Get-SettingBool -Settings $settings -Name "oracle_baseline" -Fallback $script:OracleBaseline
    $script:CompareOracle = Get-SettingBool -Settings $settings -Name "compare_oracle" -Fallback $script:CompareOracle
    $script:Overwrite = Get-SettingBool -Settings $settings -Name "overwrite" -Fallback $script:Overwrite
}

function Show-Error {
    param([string]$Message)
    [System.Windows.Forms.MessageBox]::Show(
        $Message,
        "Rusted EnergyPlus Launch",
        [System.Windows.Forms.MessageBoxButtons]::OK,
        [System.Windows.Forms.MessageBoxIcon]::Error
    ) | Out-Null
}

function New-Button {
    param(
        [string]$Text,
        [int]$X,
        [int]$Y,
        [int]$Width,
        [int]$Height
    )
    $button = New-Object System.Windows.Forms.Button
    $button.Text = $Text
    $button.Location = New-Object System.Drawing.Point($X, $Y)
    $button.Size = New-Object System.Drawing.Size($Width, $Height)
    $button.FlatStyle = [System.Windows.Forms.FlatStyle]::System
    return $button
}

function New-PathBox {
    param([int]$Y)
    $box = New-Object System.Windows.Forms.TextBox
    $box.Location = New-Object System.Drawing.Point(178, $Y)
    $box.Size = New-Object System.Drawing.Size(650, 24)
    $box.ReadOnly = $true
    return $box
}

function New-Label {
    param(
        [string]$Text,
        [int]$X,
        [int]$Y,
        [int]$Width,
        [int]$Height
    )
    $label = New-Object System.Windows.Forms.Label
    $label.Text = $Text
    $label.Location = New-Object System.Drawing.Point($X, $Y)
    $label.Size = New-Object System.Drawing.Size($Width, $Height)
    $label.TextAlign = [System.Drawing.ContentAlignment]::MiddleLeft
    return $label
}

function New-ComboBox {
    param(
        [string[]]$Items,
        [string]$Selected,
        [int]$X,
        [int]$Y,
        [int]$Width,
        [int]$Height
    )
    $combo = New-Object System.Windows.Forms.ComboBox
    $combo.DropDownStyle = [System.Windows.Forms.ComboBoxStyle]::DropDownList
    $combo.Location = New-Object System.Drawing.Point($X, $Y)
    $combo.Size = New-Object System.Drawing.Size($Width, $Height)
    [void]$combo.Items.AddRange([object[]]$Items)
    $combo.SelectedItem = $Selected
    return $combo
}

function Open-Path {
    param([string]$Path)
    if ([string]::IsNullOrWhiteSpace($Path) -or -not (Test-Path -LiteralPath $Path)) {
        Show-Error "File or folder is not available yet."
        return
    }
    Start-Process -FilePath $Path | Out-Null
}

function Refresh-Ui {
    $script:RefreshingUi = $true
    try {
        $inputBox.Text = $script:InputPath
        $weatherBox.Text = $script:WeatherPath
        $outputBox.Text = $script:OutputDir
        $oracleBox.Text = $script:OracleRoot
        $exeBox.Text = if ($null -ne $script:EplusRsExe) { $script:EplusRsExe } else { "eplus-rs.exe not found" }
        $modeCombo.SelectedItem = $script:Mode
        $partialCombo.SelectedItem = $script:PartialPolicy
        $formatCombo.SelectedItem = $script:OutputFormat
        $traceCombo.SelectedItem = $script:TraceLevel
        $failOnWarningButton.Text = if ($script:FailOnWarning) { "Strict Warnings: ON" } else { "Strict Warnings: OFF" }
        $oracleBaselineButton.Text = if ($script:OracleBaseline -or $script:CompareOracle) { "Oracle Baseline: ON" } else { "Oracle Baseline: OFF" }
        $compareButton.Text = if ($script:CompareOracle) { "Oracle Compare: ON" } else { "Oracle Compare: OFF" }
        $overwriteButton.Text = if ($script:Overwrite) { "Overwrite: ON" } else { "Overwrite: OFF" }

        $isRunning = $null -ne $script:CurrentProcess
        $weatherRequired = Test-LauncherWeatherRequired `
            -Mode $script:Mode `
            -OracleBaseline $script:OracleBaseline `
            -CompareOracle $script:CompareOracle
        $weatherReady = (-not $weatherRequired) -or (Test-LeafPath -Path $script:WeatherPath)
        $canRun = (-not $isRunning) -and
            ($null -ne $script:EplusRsExe) -and
            (Test-LeafPath -Path $script:InputPath) -and
            $weatherReady -and
            (-not [string]::IsNullOrWhiteSpace($script:OutputDir))

        $runButton.Enabled = $canRun
        $inputButton.Enabled = -not $isRunning
        $weatherButton.Enabled = -not $isRunning
        $outputButton.Enabled = -not $isRunning
        $oracleButton.Enabled = -not $isRunning
        $modeCombo.Enabled = -not $isRunning
        $partialCombo.Enabled = -not $isRunning
        $formatCombo.Enabled = -not $isRunning
        $traceCombo.Enabled = -not $isRunning
        $failOnWarningButton.Enabled = -not $isRunning
        $oracleBaselineButton.Enabled = -not $isRunning
        $compareButton.Enabled = -not $isRunning
        $overwriteButton.Enabled = -not $isRunning
        $openOutputButton.Enabled = Test-ContainerPath -Path $script:OutputDir
        $openRunReportButton.Enabled = Test-LeafPath -Path (Join-Path $script:OutputDir "reports\run-report.md")
        $openDiagnosticsButton.Enabled = Test-LeafPath -Path (Join-Path $script:OutputDir "diagnostics.json")
        $openSupportReportButton.Enabled = Test-LeafPath -Path (Join-Path $script:OutputDir "support-report.md")
        $openCompareButton.Enabled = Test-LeafPath -Path (Join-Path $script:OutputDir "compare\compare-report.md")
    }
    finally {
        $script:RefreshingUi = $false
    }
}

function Read-RunSummaryStatus {
    $summaryPath = Join-Path $script:OutputDir "run-summary.json"
    if (-not (Test-Path -LiteralPath $summaryPath -PathType Leaf)) {
        return $null
    }
    try {
        return Get-Content -Encoding UTF8 -Raw -LiteralPath $summaryPath | ConvertFrom-Json
    }
    catch {
        return $null
    }
}

function Read-RunDiagnostics {
    $diagnosticsPath = Join-Path $script:OutputDir "diagnostics.json"
    if (-not (Test-Path -LiteralPath $diagnosticsPath -PathType Leaf)) {
        return @()
    }
    try {
        $payload = Get-Content -Encoding UTF8 -Raw -LiteralPath $diagnosticsPath | ConvertFrom-Json
        return @($payload.diagnostics)
    }
    catch {
        return @()
    }
}

function Read-ArtifactPreview {
    param(
        [string]$Path,
        [string]$MissingText,
        [int]$MaxCharacters = 12000
    )
    if (-not (Test-LeafPath -Path $Path)) {
        return $MissingText
    }
    try {
        $text = Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
        if ($text.Length -le $MaxCharacters) {
            return $text
        }
        return $text.Substring(0, $MaxCharacters) + "`r`n... truncated in launcher preview; open the artifact for the full file."
    }
    catch {
        return "Failed to read artifact: $Path"
    }
}

function Finish-Run {
    $timer.Stop()
    $exitCode = $script:CurrentProcess.ExitCode
    $stdout = $script:StdoutTask.Result
    $stderr = $script:StderrTask.Result
    $script:CurrentProcess.Dispose()
    $script:CurrentProcess = $null

    $logsDir = Join-Path $script:OutputDir "logs"
    if (Test-Path -LiteralPath $script:OutputDir -PathType Container) {
        New-Item -ItemType Directory -Force -Path $logsDir | Out-Null
        Set-Content -Encoding UTF8 -LiteralPath (Join-Path $logsDir "gui-stdout.log") -Value $stdout
        Set-Content -Encoding UTF8 -LiteralPath (Join-Path $logsDir "gui-stderr.log") -Value $stderr
    }

    $summary = Read-RunSummaryStatus
    $phaseList.Items.Clear()
    $phases = if ($null -ne $summary) { Get-RunSummaryPhases -Summary $summary } else { @() }
    if (@($phases).Count -eq 0) {
        [void]$phaseList.Items.Add("No phase timing.")
    }
    else {
        foreach ($phase in @($phases | Select-Object -First 12)) {
            [void]$phaseList.Items.Add((Format-PhaseTimingLine -Phase $phase))
        }
        $timing = Get-ObjectPropertyValue -Object $summary -Name "timing" -Default $null
        $totalSeconds = Get-ObjectPropertyValue -Object $timing -Name "total_wall_seconds" -Default $null
        if ($null -ne $totalSeconds) {
            [void]$phaseList.Items.Add(("total [{0:N3}s]" -f ([double]$totalSeconds)))
        }
    }

    $diagnostics = Read-RunDiagnostics
    $diagnosticsList.Items.Clear()
    if (@($diagnostics).Count -eq 0) {
        [void]$diagnosticsList.Items.Add("No diagnostics.")
    }
    else {
        foreach ($diagnostic in @($diagnostics | Select-Object -First 8)) {
            [void]$diagnosticsList.Items.Add((Format-DiagnosticLine -Diagnostic $diagnostic))
        }
    }

    $supportTextBox.Text = Read-ArtifactPreview `
        -Path (Join-Path $script:OutputDir "support-report.md") `
        -MissingText "Support report is not available for this run."
    $selectedOutputsPath = Join-Path $script:OutputDir "results\selected-outputs.csv"
    $resultStorePath = Join-Path $script:OutputDir "results\result-store.json"
    $resultPreviewPath = if (Test-LeafPath -Path $selectedOutputsPath) { $selectedOutputsPath } else { $resultStorePath }
    $resultsTextBox.Text = Read-ArtifactPreview `
        -Path $resultPreviewPath `
        -MissingText "Rust result artifacts are not available for this run."
    $compareTextBox.Text = Read-ArtifactPreview `
        -Path (Join-Path $script:OutputDir "compare\compare-report.md") `
        -MissingText "Oracle compare report is not available for this run."
    $logsTextBox.Text = "exit_code=$exitCode`r`n`r`nstdout:`r`n$stdout`r`n`r`nstderr:`r`n$stderr"

    if ($null -ne $summary) {
        $presentation = Get-RunResultPresentation -Summary $summary
        $statusLabel.Text = $presentation.title
        $stateDetailLabel.Text = $presentation.detail
        $stateDetailLabel.ForeColor = [System.Drawing.Color]::FromName($presentation.color)
    }
    elseif ($exitCode -eq 0) {
        $statusLabel.Text = "Done."
        $stateDetailLabel.Text = "No run-summary.json was written."
        $stateDetailLabel.ForeColor = [System.Drawing.Color]::DimGray
    }
    else {
        $statusLabel.Text = "Stopped with exit code $exitCode."
        $stateDetailLabel.Text = "No run-summary.json was written."
        $stateDetailLabel.ForeColor = [System.Drawing.Color]::Firebrick
    }
    Refresh-Ui
}

function Start-Run {
    if ($null -ne $script:CurrentProcess) {
        return
    }
    if ($script:CompareOracle -and -not (Test-OracleRoot -Path $script:OracleRoot)) {
        Show-Error "Oracle compare needs an EnergyPlus 26.1.0 folder with energyplus.exe and ConvertInputFormat.exe."
        return
    }
    if ($script:OracleBaseline -and -not (Test-OracleRoot -Path $script:OracleRoot)) {
        Show-Error "Oracle baseline needs an EnergyPlus 26.1.0 folder with energyplus.exe and ConvertInputFormat.exe."
        return
    }
    $weatherRequired = Test-LauncherWeatherRequired `
        -Mode $script:Mode `
        -OracleBaseline $script:OracleBaseline `
        -CompareOracle $script:CompareOracle
    if ($weatherRequired -and -not (Test-LeafPath -Path $script:WeatherPath)) {
        Show-Error "The selected mode or oracle option needs a weather EPW file."
        return
    }
    if ($script:CompareOracle) {
        $script:OracleBaseline = $true
        $script:OutputFormat = "both"
    }
    Save-LauncherSettings -Path $script:LauncherSettingsPath
    New-Item -ItemType Directory -Force -Path $script:OutputDir | Out-Null
    $arguments = New-LauncherRunArguments `
        -InputPath $script:InputPath `
        -WeatherPath $script:WeatherPath `
        -OutputDir $script:OutputDir `
        -Mode $script:Mode `
        -PartialPolicy $script:PartialPolicy `
        -OutputFormat $script:OutputFormat `
        -TraceLevel $script:TraceLevel `
        -FailOnWarning $script:FailOnWarning `
        -Overwrite $script:Overwrite `
        -OracleBaseline $script:OracleBaseline `
        -CompareOracle $script:CompareOracle `
        -OracleRoot $script:OracleRoot

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $script:EplusRsExe
    $psi.Arguments = ($arguments | ForEach-Object { Quote-ProcessArgument $_ }) -join " "
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $true
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true

    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $psi
    if (-not $process.Start()) {
        Show-Error "Failed to start eplus-rs.exe."
        return
    }
    $script:CurrentProcess = $process
    $script:StdoutTask = $process.StandardOutput.ReadToEndAsync()
    $script:StderrTask = $process.StandardError.ReadToEndAsync()
    $statusLabel.Text = "Running..."
    $stateDetailLabel.Text = (($arguments | ForEach-Object { Quote-ProcessArgument $_ }) -join " ")
    $stateDetailLabel.ForeColor = [System.Drawing.Color]::DimGray
    $phaseList.Items.Clear()
    [void]$phaseList.Items.Add("queued: input_resolver -> typed_compile -> support_assessment -> execution_plan -> runtime/output")
    $diagnosticsList.Items.Clear()
    [void]$diagnosticsList.Items.Add("Waiting for diagnostics.json.")
    Refresh-Ui
    $timer.Start()
}

$form = New-Object System.Windows.Forms.Form
$form.Text = "Rusted EnergyPlus Launch"
$form.StartPosition = [System.Windows.Forms.FormStartPosition]::CenterScreen
$form.Size = New-Object System.Drawing.Size(880, 700)
$form.MinimumSize = New-Object System.Drawing.Size(880, 700)

$statusLabel = New-Object System.Windows.Forms.Label
$statusLabel.Text = "Ready."
$statusLabel.Location = New-Object System.Drawing.Point(18, 18)
$statusLabel.Size = New-Object System.Drawing.Size(830, 24)
$statusLabel.Font = New-Object System.Drawing.Font($statusLabel.Font, [System.Drawing.FontStyle]::Bold)
$form.Controls.Add($statusLabel)

$stateDetailLabel = New-Object System.Windows.Forms.Label
$stateDetailLabel.Text = "conformance_claim=false for launcher and arbitrary runs."
$stateDetailLabel.Location = New-Object System.Drawing.Point(18, 42)
$stateDetailLabel.Size = New-Object System.Drawing.Size(830, 36)
$stateDetailLabel.ForeColor = [System.Drawing.Color]::DimGray
$form.Controls.Add($stateDetailLabel)

$inputButton = New-Button "IDF / epJSON" 18 54 140 30
$inputButton.Location = New-Object System.Drawing.Point(18, 88)
$inputBox = New-PathBox 92
$form.Controls.AddRange(@($inputButton, $inputBox))

$weatherButton = New-Button "Weather EPW" 18 128 140 30
$weatherBox = New-PathBox 132
$form.Controls.AddRange(@($weatherButton, $weatherBox))

$outputButton = New-Button "Output Folder" 18 168 140 30
$outputBox = New-PathBox 172
$form.Controls.AddRange(@($outputButton, $outputBox))

$oracleButton = New-Button "Oracle Folder" 18 208 140 30
$oracleBox = New-PathBox 212
$form.Controls.AddRange(@($oracleButton, $oracleBox))

$exeButton = New-Button "CLI Binary" 18 248 140 30
$exeBox = New-PathBox 252
$form.Controls.AddRange(@($exeButton, $exeBox))

$modeLabel = New-Label "Mode" 18 300 50 26
$modeCombo = New-ComboBox @("compatibility", "diagnostic", "fast", "experimental") $script:Mode 70 300 132 26
$partialLabel = New-Label "Partial" 214 300 52 26
$partialCombo = New-ComboBox @("deny", "allow") $script:PartialPolicy 266 300 90 26
$formatLabel = New-Label "Format" 370 300 56 26
$formatCombo = New-ComboBox @("rust-native", "both") $script:OutputFormat 428 300 120 26
$traceLabel = New-Label "Trace" 562 300 46 26
$traceCombo = New-ComboBox @("normal", "detailed", "debug") $script:TraceLevel 610 300 84 26
$failOnWarningButton = New-Button "Strict Warnings: OFF" 706 298 122 30
$form.Controls.AddRange(@($modeLabel, $modeCombo, $partialLabel, $partialCombo, $formatLabel, $formatCombo, $traceLabel, $traceCombo, $failOnWarningButton))

$oracleBaselineButton = New-Button "Oracle Baseline: OFF" 18 342 180 34
$compareButton = New-Button "Oracle Compare: ON" 214 342 170 34
$overwriteButton = New-Button "Overwrite: ON" 400 342 150 34
$runButton = New-Button "Run" 570 342 120 34
$openOutputButton = New-Button "Open Output" 708 342 120 34
$form.Controls.AddRange(@($oracleBaselineButton, $compareButton, $overwriteButton, $runButton, $openOutputButton))

$openRunReportButton = New-Button "Open Run Report" 18 390 170 34
$openDiagnosticsButton = New-Button "Open Diagnostics" 204 390 170 34
$openSupportReportButton = New-Button "Open Support Report" 390 390 178 34
$openCompareButton = New-Button "Open Compare Report" 584 390 170 34
$exitButton = New-Button "Exit" 766 390 62 34
$form.Controls.AddRange(@($openRunReportButton, $openDiagnosticsButton, $openSupportReportButton, $openCompareButton, $exitButton))

$resultTabs = New-Object System.Windows.Forms.TabControl
$resultTabs.Location = New-Object System.Drawing.Point(18, 432)
$resultTabs.Size = New-Object System.Drawing.Size(810, 188)
$resultTabs.Anchor = [System.Windows.Forms.AnchorStyles]::Left -bor [System.Windows.Forms.AnchorStyles]::Right -bor [System.Windows.Forms.AnchorStyles]::Bottom -bor [System.Windows.Forms.AnchorStyles]::Top

$summaryTab = New-Object System.Windows.Forms.TabPage
$summaryTab.Text = "Summary"
$diagnosticsTab = New-Object System.Windows.Forms.TabPage
$diagnosticsTab.Text = "Diagnostics"
$supportTab = New-Object System.Windows.Forms.TabPage
$supportTab.Text = "Support Report"
$resultsTab = New-Object System.Windows.Forms.TabPage
$resultsTab.Text = "Results"
$compareTab = New-Object System.Windows.Forms.TabPage
$compareTab.Text = "Oracle Compare"
$logsTab = New-Object System.Windows.Forms.TabPage
$logsTab.Text = "Logs"

$phaseList = New-Object System.Windows.Forms.ListBox
$phaseList.Dock = [System.Windows.Forms.DockStyle]::Fill
$phaseList.HorizontalScrollbar = $true
[void]$phaseList.Items.Add("No phase timing.")
$summaryTab.Controls.Add($phaseList)

$diagnosticsList = New-Object System.Windows.Forms.ListBox
$diagnosticsList.Dock = [System.Windows.Forms.DockStyle]::Fill
$diagnosticsList.HorizontalScrollbar = $true
[void]$diagnosticsList.Items.Add("No diagnostics.")
$diagnosticsTab.Controls.Add($diagnosticsList)

$supportTextBox = New-ReadOnlyMultilineBox
$supportTextBox.Text = "Support report will appear after a run."
$supportTab.Controls.Add($supportTextBox)

$resultsTextBox = New-ReadOnlyMultilineBox
$resultsTextBox.Text = "Result artifacts will appear after a supported Rust run."
$resultsTab.Controls.Add($resultsTextBox)

$compareTextBox = New-ReadOnlyMultilineBox
$compareTextBox.Text = "Oracle comparison artifacts will appear when compare is enabled."
$compareTab.Controls.Add($compareTextBox)

$logsTextBox = New-ReadOnlyMultilineBox
$logsTextBox.Text = "Launcher stdout/stderr logs will appear after a run."
$logsTab.Controls.Add($logsTextBox)

foreach ($tab in @($summaryTab, $diagnosticsTab, $supportTab, $resultsTab, $compareTab, $logsTab)) {
    [void]$resultTabs.TabPages.Add($tab)
}
$form.Controls.Add($resultTabs)

$footerLabel = New-Object System.Windows.Forms.Label
$footerLabel.Text = "Rusted EnergyPlus is not a drop-in replacement for EnergyPlus; SupportAssessment controls Rust execution, and oracle output is never shown as Rust success."
$footerLabel.Location = New-Object System.Drawing.Point(18, 632)
$footerLabel.Size = New-Object System.Drawing.Size(810, 30)
$footerLabel.ForeColor = [System.Drawing.Color]::DimGray
$form.Controls.Add($footerLabel)

$timer = New-Object System.Windows.Forms.Timer
$timer.Interval = 500
$timer.Add_Tick({
    if ($null -ne $script:CurrentProcess -and $script:CurrentProcess.HasExited) {
        Finish-Run
    }
})

$inputButton.Add_Click({
    $dialog = New-Object System.Windows.Forms.OpenFileDialog
    $dialog.Filter = "EnergyPlus Inputs (*.idf;*.epJSON)|*.idf;*.epJSON|All files (*.*)|*.*"
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $script:InputPath = $dialog.FileName
        Save-LauncherSettings -Path $script:LauncherSettingsPath
        Refresh-Ui
    }
})

$weatherButton.Add_Click({
    $dialog = New-Object System.Windows.Forms.OpenFileDialog
    $dialog.Filter = "Weather files (*.epw)|*.epw|All files (*.*)|*.*"
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $script:WeatherPath = $dialog.FileName
        Save-LauncherSettings -Path $script:LauncherSettingsPath
        Refresh-Ui
    }
})

$outputButton.Add_Click({
    $dialog = New-Object System.Windows.Forms.FolderBrowserDialog
    $dialog.SelectedPath = $script:OutputDir
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $script:OutputDir = $dialog.SelectedPath
        Save-LauncherSettings -Path $script:LauncherSettingsPath
        Refresh-Ui
    }
})

$oracleButton.Add_Click({
    $dialog = New-Object System.Windows.Forms.FolderBrowserDialog
    $dialog.SelectedPath = $script:OracleRoot
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $script:OracleRoot = $dialog.SelectedPath
        if (Test-OracleRoot -Path $script:OracleRoot) {
            $script:OracleBaseline = $true
        }
        Save-LauncherSettings -Path $script:LauncherSettingsPath
        Refresh-Ui
    }
})

$exeButton.Add_Click({
    $dialog = New-Object System.Windows.Forms.OpenFileDialog
    $dialog.Filter = "eplus-rs.exe|eplus-rs.exe|Executables (*.exe)|*.exe|All files (*.*)|*.*"
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $script:EplusRsExe = $dialog.FileName
        Save-LauncherSettings -Path $script:LauncherSettingsPath
        Refresh-Ui
    }
})

$compareButton.Add_Click({
    $script:CompareOracle = -not $script:CompareOracle
    if ($script:CompareOracle) {
        $script:OracleBaseline = $true
        $script:OutputFormat = "both"
    }
    Save-LauncherSettings -Path $script:LauncherSettingsPath
    Refresh-Ui
})

$oracleBaselineButton.Add_Click({
    if ($script:CompareOracle) {
        $script:CompareOracle = $false
        $script:OracleBaseline = $false
    }
    else {
        $script:OracleBaseline = -not $script:OracleBaseline
    }
    Save-LauncherSettings -Path $script:LauncherSettingsPath
    Refresh-Ui
})

$overwriteButton.Add_Click({
    $script:Overwrite = -not $script:Overwrite
    Save-LauncherSettings -Path $script:LauncherSettingsPath
    Refresh-Ui
})

$failOnWarningButton.Add_Click({
    $script:FailOnWarning = -not $script:FailOnWarning
    Save-LauncherSettings -Path $script:LauncherSettingsPath
    Refresh-Ui
})

$modeCombo.Add_SelectedIndexChanged({
    if ($script:RefreshingUi) {
        return
    }
    if ($null -ne $modeCombo.SelectedItem) {
        $script:Mode = [string]$modeCombo.SelectedItem
        Save-LauncherSettings -Path $script:LauncherSettingsPath
        Refresh-Ui
    }
})

$partialCombo.Add_SelectedIndexChanged({
    if ($script:RefreshingUi) {
        return
    }
    if ($null -ne $partialCombo.SelectedItem) {
        $script:PartialPolicy = [string]$partialCombo.SelectedItem
        Save-LauncherSettings -Path $script:LauncherSettingsPath
        Refresh-Ui
    }
})

$formatCombo.Add_SelectedIndexChanged({
    if ($script:RefreshingUi) {
        return
    }
    if ($null -ne $formatCombo.SelectedItem) {
        $script:OutputFormat = [string]$formatCombo.SelectedItem
        Save-LauncherSettings -Path $script:LauncherSettingsPath
        Refresh-Ui
    }
})

$traceCombo.Add_SelectedIndexChanged({
    if ($script:RefreshingUi) {
        return
    }
    if ($null -ne $traceCombo.SelectedItem) {
        $script:TraceLevel = [string]$traceCombo.SelectedItem
        Save-LauncherSettings -Path $script:LauncherSettingsPath
        Refresh-Ui
    }
})

$runButton.Add_Click({ Start-Run })
$openOutputButton.Add_Click({ Open-Path -Path $script:OutputDir })
$openRunReportButton.Add_Click({ Open-Path -Path (Join-Path $script:OutputDir "reports\run-report.md") })
$openDiagnosticsButton.Add_Click({ Open-Path -Path (Join-Path $script:OutputDir "diagnostics.json") })
$openSupportReportButton.Add_Click({ Open-Path -Path (Join-Path $script:OutputDir "support-report.md") })
$openCompareButton.Add_Click({ Open-Path -Path (Join-Path $script:OutputDir "compare\compare-report.md") })
$exitButton.Add_Click({ $form.Close() })

$form.Add_FormClosing({
    param($Sender, $EventArgs)
    if ($null -ne $script:CurrentProcess -and -not $script:CurrentProcess.HasExited) {
        $answer = [System.Windows.Forms.MessageBox]::Show(
            "A run is still active. Stop it and close?",
            "Rusted EnergyPlus Launch",
            [System.Windows.Forms.MessageBoxButtons]::YesNo,
            [System.Windows.Forms.MessageBoxIcon]::Warning
        )
        if ($answer -ne [System.Windows.Forms.DialogResult]::Yes) {
            $EventArgs.Cancel = $true
            return
        }
        $script:CurrentProcess.Kill()
    }
})

Refresh-Ui
[void]$form.ShowDialog()
