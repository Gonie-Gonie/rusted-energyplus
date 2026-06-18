[CmdletBinding()]
param(
    [switch]$SelfTest
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptRoot = $PSScriptRoot
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptRoot "..")).Path
$AppRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptsRoot "..")).Path

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

function Resolve-EplusRsExe {
    $command = Get-Command eplus-rs -ErrorAction SilentlyContinue
    $commandPath = if ($null -ne $command) { $command.Source } else { $null }
    Resolve-FirstFile -Candidates @(
        (Join-Path $AppRoot "bin\eplus-rs.exe"),
        (Join-Path $AppRoot "target\release\eplus-rs.exe"),
        (Join-Path $AppRoot "target\debug\eplus-rs.exe"),
        $commandPath
    )
}

function Resolve-OracleRoot {
    Resolve-FirstDirectory -Candidates @(
        $env:RUSTED_ENERGYPLUS_ORACLE_ROOT,
        (Join-Path $AppRoot "oracle\energyplus\26.1.0"),
        (Join-Path $AppRoot ".runtime\energyplus\26.1.0")
    )
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

$DefaultOracleRoot = Resolve-OracleRoot
$DefaultIdf = if (-not [string]::IsNullOrWhiteSpace($DefaultOracleRoot)) {
    Resolve-FirstFile -Candidates @(
        (Join-Path $DefaultOracleRoot "ExampleFiles\1ZoneUncontrolled.idf")
    )
}
else {
    $null
}
$DefaultWeather = if (-not [string]::IsNullOrWhiteSpace($DefaultOracleRoot)) {
    Resolve-FirstFile -Candidates @(
        (Join-Path $DefaultOracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw")
    )
}
else {
    $null
}

if ($SelfTest) {
    [pscustomobject]@{
        app_root = $AppRoot
        eplus_rs = Resolve-EplusRsExe
        oracle_root = $DefaultOracleRoot
        oracle_ready = Test-OracleRoot -Path $DefaultOracleRoot
        default_idf = $DefaultIdf
        default_weather = $DefaultWeather
    } | ConvertTo-Json -Depth 3
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
$script:CompareOracle = Test-OracleRoot -Path $script:OracleRoot
$script:Overwrite = $true
$script:CurrentProcess = $null
$script:StdoutTask = $null
$script:StderrTask = $null
$script:StdoutPath = ""
$script:StderrPath = ""

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
    $box.Size = New-Object System.Drawing.Size(570, 24)
    $box.ReadOnly = $true
    return $box
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
    $inputBox.Text = $script:InputPath
    $weatherBox.Text = $script:WeatherPath
    $outputBox.Text = $script:OutputDir
    $oracleBox.Text = $script:OracleRoot
    $exeBox.Text = if ($null -ne $script:EplusRsExe) { $script:EplusRsExe } else { "eplus-rs.exe not found" }
    $compareButton.Text = if ($script:CompareOracle) { "Oracle Compare: ON" } else { "Oracle Compare: OFF" }
    $overwriteButton.Text = if ($script:Overwrite) { "Overwrite: ON" } else { "Overwrite: OFF" }

    $isRunning = $null -ne $script:CurrentProcess
    $canRun = (-not $isRunning) -and
        ($null -ne $script:EplusRsExe) -and
        (Test-LeafPath -Path $script:InputPath) -and
        (Test-LeafPath -Path $script:WeatherPath) -and
        (-not [string]::IsNullOrWhiteSpace($script:OutputDir))

    $runButton.Enabled = $canRun
    $inputButton.Enabled = -not $isRunning
    $weatherButton.Enabled = -not $isRunning
    $outputButton.Enabled = -not $isRunning
    $oracleButton.Enabled = -not $isRunning
    $compareButton.Enabled = -not $isRunning
    $overwriteButton.Enabled = -not $isRunning
    $openOutputButton.Enabled = Test-ContainerPath -Path $script:OutputDir
    $openRunReportButton.Enabled = Test-LeafPath -Path (Join-Path $script:OutputDir "reports\run-report.md")
    $openCompareButton.Enabled = Test-LeafPath -Path (Join-Path $script:OutputDir "compare\compare-report.md")
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
    if ($null -ne $summary) {
        $propertyNames = @($summary.PSObject.Properties.Name)
        $supportStatus = if ($propertyNames -contains "support") {
            $summary.support.status
        }
        elseif ($propertyNames -contains "support_status") {
            $summary.support_status
        }
        else {
            "unknown"
        }
        $compareStatus = if ($propertyNames -contains "compare_status") {
            $summary.compare_status
        }
        else {
            "not-run"
        }
        $statusLabel.Text = "Done: $($summary.status), support: $supportStatus, compare: $compareStatus"
    }
    elseif ($exitCode -eq 0) {
        $statusLabel.Text = "Done."
    }
    else {
        $statusLabel.Text = "Stopped with exit code $exitCode."
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
    New-Item -ItemType Directory -Force -Path $script:OutputDir | Out-Null
    $arguments = @(
        "run",
        $script:InputPath,
        "-w",
        $script:WeatherPath,
        "-d",
        $script:OutputDir
    )
    if ($script:Overwrite) {
        $arguments += "--overwrite"
    }
    if ($script:CompareOracle) {
        $arguments += "--compare-oracle"
    }
    if (-not [string]::IsNullOrWhiteSpace($script:OracleRoot)) {
        $arguments += @("--oracle-root", $script:OracleRoot)
    }

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
    Refresh-Ui
    $timer.Start()
}

$form = New-Object System.Windows.Forms.Form
$form.Text = "Rusted EnergyPlus Launch"
$form.StartPosition = [System.Windows.Forms.FormStartPosition]::CenterScreen
$form.Size = New-Object System.Drawing.Size(780, 420)
$form.MinimumSize = New-Object System.Drawing.Size(780, 420)

$statusLabel = New-Object System.Windows.Forms.Label
$statusLabel.Text = "Ready."
$statusLabel.Location = New-Object System.Drawing.Point(18, 18)
$statusLabel.Size = New-Object System.Drawing.Size(730, 24)
$form.Controls.Add($statusLabel)

$inputButton = New-Button "IDF / epJSON" 18 54 140 30
$inputBox = New-PathBox 58
$form.Controls.AddRange(@($inputButton, $inputBox))

$weatherButton = New-Button "Weather EPW" 18 94 140 30
$weatherBox = New-PathBox 98
$form.Controls.AddRange(@($weatherButton, $weatherBox))

$outputButton = New-Button "Output Folder" 18 134 140 30
$outputBox = New-PathBox 138
$form.Controls.AddRange(@($outputButton, $outputBox))

$oracleButton = New-Button "Oracle Folder" 18 174 140 30
$oracleBox = New-PathBox 178
$form.Controls.AddRange(@($oracleButton, $oracleBox))

$exeButton = New-Button "CLI Binary" 18 214 140 30
$exeBox = New-PathBox 218
$form.Controls.AddRange(@($exeButton, $exeBox))

$compareButton = New-Button "Oracle Compare: ON" 18 266 170 34
$overwriteButton = New-Button "Overwrite: ON" 204 266 150 34
$runButton = New-Button "Run" 374 266 120 34
$openOutputButton = New-Button "Open Output" 514 266 120 34
$exitButton = New-Button "Exit" 650 266 98 34
$form.Controls.AddRange(@($compareButton, $overwriteButton, $runButton, $openOutputButton, $exitButton))

$openRunReportButton = New-Button "Open Run Report" 18 318 170 34
$openCompareButton = New-Button "Open Compare Report" 204 318 190 34
$form.Controls.AddRange(@($openRunReportButton, $openCompareButton))

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
        Refresh-Ui
    }
})

$weatherButton.Add_Click({
    $dialog = New-Object System.Windows.Forms.OpenFileDialog
    $dialog.Filter = "Weather files (*.epw)|*.epw|All files (*.*)|*.*"
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $script:WeatherPath = $dialog.FileName
        Refresh-Ui
    }
})

$outputButton.Add_Click({
    $dialog = New-Object System.Windows.Forms.FolderBrowserDialog
    $dialog.SelectedPath = $script:OutputDir
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $script:OutputDir = $dialog.SelectedPath
        Refresh-Ui
    }
})

$oracleButton.Add_Click({
    $dialog = New-Object System.Windows.Forms.FolderBrowserDialog
    $dialog.SelectedPath = $script:OracleRoot
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $script:OracleRoot = $dialog.SelectedPath
        $script:CompareOracle = Test-OracleRoot -Path $script:OracleRoot
        Refresh-Ui
    }
})

$exeButton.Add_Click({
    $dialog = New-Object System.Windows.Forms.OpenFileDialog
    $dialog.Filter = "eplus-rs.exe|eplus-rs.exe|Executables (*.exe)|*.exe|All files (*.*)|*.*"
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $script:EplusRsExe = $dialog.FileName
        Refresh-Ui
    }
})

$compareButton.Add_Click({
    $script:CompareOracle = -not $script:CompareOracle
    Refresh-Ui
})

$overwriteButton.Add_Click({
    $script:Overwrite = -not $script:Overwrite
    Refresh-Ui
})

$runButton.Add_Click({ Start-Run })
$openOutputButton.Add_Click({ Open-Path -Path $script:OutputDir })
$openRunReportButton.Add_Click({ Open-Path -Path (Join-Path $script:OutputDir "reports\run-report.md") })
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
