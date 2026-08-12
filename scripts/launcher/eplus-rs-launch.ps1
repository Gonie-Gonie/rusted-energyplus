[CmdletBinding()]
param(
    [switch]$SelfTest,
    [string]$ScreenshotPath = ""
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptRoot = $PSScriptRoot
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptRoot "..")).Path
$AppRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptsRoot "..")).Path

. (Join-Path $ScriptRoot "eplus-rs-launch\core.ps1")
. (Join-Path $ScriptRoot "eplus-rs-launch\artifacts.ps1")
. (Join-Path $ScriptRoot "eplus-rs-launch\ui.ps1")
. (Join-Path $ScriptRoot "eplus-rs-launch\self_test.ps1")

$LauncherDefaults = Get-LauncherDefaultPaths
$DefaultOracleRoot = $LauncherDefaults.OracleRoot
$DefaultIdf = $LauncherDefaults.Idf
$DefaultWeather = $LauncherDefaults.Weather


if ($SelfTest) {
    Invoke-LauncherSelfTest `
        -AppRoot $AppRoot `
        -DefaultOracleRoot $DefaultOracleRoot `
        -DefaultIdf $DefaultIdf `
        -DefaultWeather $DefaultWeather `
        -LauncherScriptPath $PSCommandPath
    exit 0
}

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
[System.Windows.Forms.Application]::EnableVisualStyles()

$script:EplusRsExe = Resolve-EplusRsExe
$script:EplusRsExeSelection = "auto"
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
$script:CancelRequested = $false

$settings = Read-LauncherSettings -Path $script:LauncherSettingsPath
if ($null -ne $settings) {
    $script:InputPath = Get-SettingValue -Settings $settings -Name "input_path" -Fallback $script:InputPath
    $script:WeatherPath = Get-SettingValue -Settings $settings -Name "weather_path" -Fallback $script:WeatherPath
    $script:OutputDir = Get-SettingValue -Settings $settings -Name "output_dir" -Fallback $script:OutputDir
    $script:OracleRoot = Get-SettingValue -Settings $settings -Name "oracle_root" -Fallback $script:OracleRoot
    $savedExe = Get-SettingValue -Settings $settings -Name "eplus_rs_exe" -Fallback ""
    $savedExeSelection = Get-SettingValue -Settings $settings -Name "eplus_rs_exe_selection" -Fallback ""
    $resolvedExeSetting = Resolve-EplusRsExeSetting `
        -SavedPath $savedExe `
        -SelectionSource $savedExeSelection `
        -AutoResolvedPath $script:EplusRsExe `
        -AppRoot $AppRoot
    $script:EplusRsExe = $resolvedExeSetting.path
    $script:EplusRsExeSelection = $resolvedExeSetting.selection_source
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
        $cancelButton.Enabled = $isRunning
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
        $openEvidenceButton.Enabled = Test-LeafPath -Path (Find-EvidenceArtifactPath -OutputDir $script:OutputDir)
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
    $claimBoundaryTextBox.Text = "Claim boundary is not available until run-summary.json is written."
    $selectedOutputsPath = Join-Path $script:OutputDir "results\selected-outputs.csv"
    $resultStorePath = Join-Path $script:OutputDir "results\result-store.json"
    $resultPreviewPath = if (Test-LeafPath -Path $selectedOutputsPath) { $selectedOutputsPath } else { $resultStorePath }
    $resultsTextBox.Text = Read-ArtifactPreview `
        -Path $resultPreviewPath `
        -MissingText "Rust result artifacts are not available for this run."
    $compareTextBox.Text = Read-ArtifactPreview `
        -Path (Join-Path $script:OutputDir "compare\compare-report.md") `
        -MissingText "Oracle compare report is not available for this run."
    $plotsTextBox.Text = Read-PlotArtifactPreview -OutputDir $script:OutputDir
    $evidenceTextBox.Text = Read-EvidenceArtifactPreview -OutputDir $script:OutputDir
    $logsTextBox.Text = "exit_code=$exitCode`r`n`r`nstdout:`r`n$stdout`r`n`r`nstderr:`r`n$stderr"

    if ($null -ne $summary) {
        $presentation = Get-RunResultPresentation -Summary $summary
        $statusLabel.Text = $presentation.title
        $stateDetailLabel.Text = $presentation.detail
        $stateDetailLabel.ForeColor = [System.Drawing.Color]::FromName($presentation.color)
        $claimBoundaryTextBox.Text = "Claim Boundary`r`n`r`n$($presentation.detail)`r`n`r`nFast and experimental modes are never release conformance evidence."
    }
    elseif ($script:CancelRequested) {
        $statusLabel.Text = "Cancelled."
        $stateDetailLabel.Text = "Run process was cancelled before run-summary.json was written."
        $stateDetailLabel.ForeColor = [System.Drawing.Color]::DarkGoldenrod
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
    $script:CancelRequested = $false
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
    $script:CancelRequested = $false
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
    foreach ($stage in @("Input", "Convert", "RawModel", "TypedModel", "Graph", "Support", "Plan", "Runtime", "Export", "Oracle", "Compare")) {
        [void]$phaseList.Items.Add("queued: $stage")
    }
    $diagnosticsList.Items.Clear()
    [void]$diagnosticsList.Items.Add("Waiting for diagnostics.json.")
    Refresh-Ui
    $timer.Start()
}

function Cancel-Run {
    if ($null -eq $script:CurrentProcess) {
        return
    }
    $script:CancelRequested = $true
    $statusLabel.Text = "Cancelling..."
    $stateDetailLabel.Text = "Stopping eplus-rs run process."
    $stateDetailLabel.ForeColor = [System.Drawing.Color]::DarkGoldenrod
    try {
        if (-not $script:CurrentProcess.HasExited) {
            $script:CurrentProcess.Kill()
        }
    }
    catch {
        Show-Error "Failed to cancel eplus-rs.exe: $_"
    }
    Refresh-Ui
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

$oracleBaselineButton = New-Button "Oracle Baseline: OFF" 18 342 170 34
$compareButton = New-Button "Oracle Compare: ON" 198 342 160 34
$overwriteButton = New-Button "Overwrite: ON" 368 342 140 34
$runButton = New-Button "Run" 520 342 90 34
$cancelButton = New-Button "Cancel" 620 342 90 34
$openOutputButton = New-Button "Open Output" 720 342 108 34
$form.Controls.AddRange(@($oracleBaselineButton, $compareButton, $overwriteButton, $runButton, $cancelButton, $openOutputButton))

$openRunReportButton = New-Button "Open Run Report" 18 390 150 34
$openDiagnosticsButton = New-Button "Open Diagnostics" 180 390 150 34
$openSupportReportButton = New-Button "Open Support Report" 342 390 166 34
$openCompareButton = New-Button "Open Compare Report" 520 390 160 34
$openEvidenceButton = New-Button "Open Evidence" 692 390 136 34
$form.Controls.AddRange(@($openRunReportButton, $openDiagnosticsButton, $openSupportReportButton, $openCompareButton, $openEvidenceButton))

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
$claimBoundaryTab = New-Object System.Windows.Forms.TabPage
$claimBoundaryTab.Text = "Claim Boundary"
$resultsTab = New-Object System.Windows.Forms.TabPage
$resultsTab.Text = "Results"
$compareTab = New-Object System.Windows.Forms.TabPage
$compareTab.Text = "Oracle Compare"
$plotsTab = New-Object System.Windows.Forms.TabPage
$plotsTab.Text = "Plots"
$evidenceTab = New-Object System.Windows.Forms.TabPage
$evidenceTab.Text = "Evidence"
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

$claimBoundaryTextBox = New-ReadOnlyMultilineBox
$claimBoundaryTextBox.Text = "Claim boundary will appear after a run."
$claimBoundaryTab.Controls.Add($claimBoundaryTextBox)

$resultsTextBox = New-ReadOnlyMultilineBox
$resultsTextBox.Text = "Result artifacts will appear after a supported Rust run."
$resultsTab.Controls.Add($resultsTextBox)

$compareTextBox = New-ReadOnlyMultilineBox
$compareTextBox.Text = "Oracle comparison artifacts will appear when compare is enabled."
$compareTab.Controls.Add($compareTextBox)

$plotsTextBox = New-ReadOnlyMultilineBox
$plotsTextBox.Text = "Plot artifacts will appear after a run writes reports\plots, plots, or compare\plots."
$plotsTab.Controls.Add($plotsTextBox)

$evidenceTextBox = New-ReadOnlyMultilineBox
$evidenceTextBox.Text = "Evidence summary/PDF artifacts will appear after a run writes reports\evidence-summary.md or evidence PDFs."
$evidenceTab.Controls.Add($evidenceTextBox)

$logsTextBox = New-ReadOnlyMultilineBox
$logsTextBox.Text = "Launcher stdout/stderr logs will appear after a run."
$logsTab.Controls.Add($logsTextBox)

foreach ($tab in @($summaryTab, $diagnosticsTab, $supportTab, $claimBoundaryTab, $resultsTab, $compareTab, $plotsTab, $evidenceTab, $logsTab)) {
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
        $script:EplusRsExeSelection = "user"
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
$cancelButton.Add_Click({ Cancel-Run })
$openOutputButton.Add_Click({ Open-Path -Path $script:OutputDir })
$openRunReportButton.Add_Click({ Open-Path -Path (Join-Path $script:OutputDir "reports\run-report.md") })
$openDiagnosticsButton.Add_Click({ Open-Path -Path (Join-Path $script:OutputDir "diagnostics.json") })
$openSupportReportButton.Add_Click({ Open-Path -Path (Join-Path $script:OutputDir "support-report.md") })
$openCompareButton.Add_Click({ Open-Path -Path (Join-Path $script:OutputDir "compare\compare-report.md") })
$openEvidenceButton.Add_Click({ Open-Path -Path (Find-EvidenceArtifactPath -OutputDir $script:OutputDir) })

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

if (-not [string]::IsNullOrWhiteSpace($ScreenshotPath)) {
    $screenshotDirectory = Split-Path -Parent $ScreenshotPath
    if (-not [string]::IsNullOrWhiteSpace($screenshotDirectory)) {
        New-Item -ItemType Directory -Force -Path $screenshotDirectory | Out-Null
    }
    $form.CreateControl()
    $bitmap = New-Object System.Drawing.Bitmap($form.Width, $form.Height)
    try {
        $bounds = New-Object System.Drawing.Rectangle(0, 0, $form.Width, $form.Height)
        $form.DrawToBitmap($bitmap, $bounds)
        $bitmap.Save($ScreenshotPath, [System.Drawing.Imaging.ImageFormat]::Png)
    }
    finally {
        $bitmap.Dispose()
        $form.Dispose()
    }
    exit 0
}

[void]$form.ShowDialog()
