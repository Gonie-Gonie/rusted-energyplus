[CmdletBinding()]
param(
    [ValidateRange(1, 3600)]
    [int]$CliRunTimeoutSeconds = 120
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Assert-File {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [string]$Description = "artifact"
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing expected $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

function Assert-Directory {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [string]$Description = "directory"
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Container)) {
        throw "Missing expected $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

function Assert-Equal {
    param(
        [Parameter(Mandatory = $true)]$Actual,
        [Parameter(Mandatory = $true)]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Actual -ne $Expected) {
        throw "Unexpected $Description`: expected $Expected, got $Actual"
    }
    Write-Host "OK $Description`: $Actual"
}

function Assert-ContainsValue {
    param(
        [Parameter(Mandatory = $true)]$Values,
        [Parameter(Mandatory = $true)][string]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (@($Values) -notcontains $Expected) {
        throw "Missing $Description`: $Expected"
    }
    Write-Host "OK $Description`: $Expected"
}

function Assert-NotContainsValue {
    param(
        [Parameter(Mandatory = $true)]$Values,
        [Parameter(Mandatory = $true)][string]$Unexpected,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (@($Values) -contains $Unexpected) {
        throw "Unexpected $Description`: $Unexpected"
    }
    Write-Host "OK absent $Description`: $Unexpected"
}

function Assert-Matches {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch $Pattern) {
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

function Assert-NotMatches {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -match $Pattern) {
        throw "Unexpected $Description`: $Pattern"
    }
    Write-Host "OK absent $Description`: $Pattern"
}

function Read-JsonFile {
    param([Parameter(Mandatory = $true)][string]$Path)
    return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path | ConvertFrom-Json
}

function Write-Utf8NoBomFile {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Contents
    )
    $directory = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($directory)) {
        New-Item -ItemType Directory -Force -Path $directory | Out-Null
    }
    $encoding = New-Object System.Text.UTF8Encoding -ArgumentList $false
    [System.IO.File]::WriteAllText($Path, $Contents, $encoding)
}

function ConvertTo-ProcessArgument {
    param(
        [AllowEmptyString()]
        [Parameter(Mandatory = $true)][string]$Argument
    )

    if ($Argument.Length -gt 0 -and $Argument -notmatch '[\s"]') {
        return $Argument
    }

    $builder = New-Object System.Text.StringBuilder
    [void]$builder.Append('"')
    $backslashCount = 0
    foreach ($character in $Argument.ToCharArray()) {
        if ($character -eq '\') {
            $backslashCount += 1
            continue
        }
        if ($character -eq '"') {
            for ($index = 0; $index -lt ((2 * $backslashCount) + 1); $index += 1) {
                [void]$builder.Append('\')
            }
            [void]$builder.Append('"')
            $backslashCount = 0
            continue
        }
        for ($index = 0; $index -lt $backslashCount; $index += 1) {
            [void]$builder.Append('\')
        }
        $backslashCount = 0
        [void]$builder.Append($character)
    }
    for ($index = 0; $index -lt (2 * $backslashCount); $index += 1) {
        [void]$builder.Append('\')
    }
    [void]$builder.Append('"')
    return $builder.ToString()
}

function Invoke-LauncherCliRun {
    param(
        [Parameter(Mandatory = $true)][string]$Description,
        [Parameter(Mandatory = $true)][string[]]$Arguments,
        [Parameter(Mandatory = $true)][int]$ExpectedExitCode,
        [Parameter(Mandatory = $true)][string]$OutputDir
    )

    if (Test-Path -LiteralPath $OutputDir) {
        Remove-Item -Recurse -Force -LiteralPath $OutputDir
    }

    Write-Host "Running launcher-equivalent CLI case: $Description"
    $startInfo = New-Object System.Diagnostics.ProcessStartInfo
    $startInfo.FileName = $script:CliExe
    $startInfo.WorkingDirectory = $RepoRoot
    $startInfo.UseShellExecute = $false
    $startInfo.CreateNoWindow = $true
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    if (@($startInfo.PSObject.Properties.Name) -contains "ArgumentList") {
        foreach ($argument in $Arguments) {
            [void]$startInfo.ArgumentList.Add($argument)
        }
    } else {
        $startInfo.Arguments = (($Arguments | ForEach-Object {
                    ConvertTo-ProcessArgument -Argument $_
                }) -join " ")
    }

    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $startInfo
    $standardOutput = ""
    $standardError = ""
    $exitCode = $null
    $timedOut = $false
    try {
        if (-not $process.Start()) {
            throw "Failed to start launcher-equivalent CLI case: $Description"
        }
        $standardOutputTask = $process.StandardOutput.ReadToEndAsync()
        $standardErrorTask = $process.StandardError.ReadToEndAsync()
        $timeoutMilliseconds = [int]([Math]::Min(
                [int]::MaxValue,
                [int64]$CliRunTimeoutSeconds * 1000
            ))
        if (-not $process.WaitForExit($timeoutMilliseconds)) {
            $timedOut = $true
            $treeKillSucceeded = $false
            $taskkill = Join-Path $env:SystemRoot "System32\taskkill.exe"
            if (Test-Path -LiteralPath $taskkill -PathType Leaf) {
                & $taskkill /PID $process.Id /T /F *> $null
                $treeKillSucceeded = $LASTEXITCODE -eq 0
            }
            if (-not $treeKillSucceeded -and -not $process.HasExited) {
                try {
                    $process.Kill()
                } catch {
                    # A concurrent process exit is equivalent to a successful direct kill.
                }
            }
            if (-not $process.WaitForExit(5000) -and -not $process.HasExited) {
                try {
                    $process.Kill()
                    [void]$process.WaitForExit(1000)
                } catch {
                    # Output capture below is bounded even when final cleanup fails.
                }
            }
        } else {
            $process.WaitForExit()
        }
        if ($timedOut) {
            foreach ($outputTask in @($standardOutputTask, $standardErrorTask)) {
                try {
                    [void]$outputTask.Wait(1000)
                } catch {
                    # Timeout diagnostics are best-effort and must not extend the timeout.
                }
            }
            if ($standardOutputTask.Status -eq [System.Threading.Tasks.TaskStatus]::RanToCompletion) {
                $standardOutput = $standardOutputTask.GetAwaiter().GetResult()
            }
            if ($standardErrorTask.Status -eq [System.Threading.Tasks.TaskStatus]::RanToCompletion) {
                $standardError = $standardErrorTask.GetAwaiter().GetResult()
            }
        } else {
            $standardOutput = $standardOutputTask.GetAwaiter().GetResult()
            $standardError = $standardErrorTask.GetAwaiter().GetResult()
            $exitCode = $process.ExitCode
        }
    } finally {
        $process.Dispose()
    }

    $output = @($standardOutput, $standardError) | Where-Object {
        -not [string]::IsNullOrWhiteSpace($_)
    }
    if ($timedOut) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Timed out $Description after $CliRunTimeoutSeconds seconds"
    }
    if ($exitCode -ne $ExpectedExitCode) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Unexpected $Description exit code: expected $ExpectedExitCode, got $exitCode"
    }

    $summaryPath = Join-Path $OutputDir "run-summary.json"
    Assert-File -Path $summaryPath -Description "$Description run summary"
    $summary = Read-JsonFile -Path $summaryPath
    return $summary
}

function Assert-LauncherRunSummary {
    param(
        [Parameter(Mandatory = $true)]$Summary,
        [Parameter(Mandatory = $true)][string]$ExpectedStatus,
        [Parameter(Mandatory = $true)][int]$ExpectedExitCode,
        [Parameter(Mandatory = $true)][string]$ExpectedRunResultState,
        [Parameter(Mandatory = $true)][string]$Description
    )
    Assert-Equal -Actual $Summary.status -Expected $ExpectedStatus -Description "$Description status"
    Assert-Equal -Actual $Summary.exit_code -Expected $ExpectedExitCode -Description "$Description exit code"
    Assert-Equal -Actual $Summary.support.run_result_state -Expected $ExpectedRunResultState -Description "$Description run result state"
    Assert-Equal -Actual $Summary.support.conformance_claim -Expected $false -Description "$Description conformance claim"
}

function Assert-JsonFields {
    param(
        [Parameter(Mandatory = $true)]$Object,
        [Parameter(Mandatory = $true)][string[]]$Fields,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($null -eq $Object) {
        throw "Missing JSON object for $Description"
    }
    $names = @($Object.PSObject.Properties.Name)
    foreach ($field in $Fields) {
        if ($names -notcontains $field) {
            throw "Missing $Description field: $field"
        }
    }
    Write-Host "OK $Description fields: $($Fields -join ', ')"
}

function Assert-LauncherArtifactSchema {
    param(
        [Parameter(Mandatory = $true)]$Summary,
        [Parameter(Mandatory = $true)][string]$OutputDir,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-Equal -Actual $Summary.schema_version -Expected 1 -Description "$Description run-summary schema version"
    Assert-JsonFields -Object $Summary -Fields @(
        "schema_version",
        "status",
        "exit_code",
        "config",
        "support",
        "diagnostics",
        "artifacts"
    ) -Description "$Description run-summary schema"
    Assert-JsonFields -Object $Summary.config -Fields @("mode", "partial_policy", "output_format", "trace_level") -Description "$Description run-summary config schema"
    Assert-JsonFields -Object $Summary.support -Fields @(
        "status",
        "run_result_state",
        "runtime_class",
        "selected_algorithm_lane",
        "matched_capability_ids",
        "matched_capabilities",
        "conformance_claim"
    ) -Description "$Description run-summary support schema"
    Assert-JsonFields -Object $Summary.artifacts -Fields @(
        "diagnostics_json",
        "run_summary_json",
        "support_assessment_json",
        "support_report_md"
    ) -Description "$Description run-summary artifact schema"

    $supportPath = Join-Path $OutputDir "support-assessment.json"
    Assert-File -Path $supportPath -Description "$Description support assessment"
    $support = Read-JsonFile -Path $supportPath
    Assert-Equal -Actual $support.schema_version -Expected 1 -Description "$Description support-assessment schema version"
    Assert-JsonFields -Object $support -Fields @(
        "schema_version",
        "status",
        "run_result_state",
        "runtime_class",
        "selected_algorithm_lane",
        "matched_capability_ids",
        "matched_capabilities",
        "mode",
        "partial_policy",
        "output_format",
        "trace_level",
        "capability_registry",
        "capability_registry_loaded",
        "claim_boundary",
        "typed_objects",
        "unsupported_objects",
        "diagnostics"
    ) -Description "$Description support-assessment schema"
    Assert-JsonFields -Object $support.claim_boundary -Fields @(
        "conformance_claim",
        "release_evidence",
        "statement"
    ) -Description "$Description support-assessment claim boundary schema"
}

$launcherScript = Join-Path $RepoRoot "scripts\launcher\eplus-rs-launch.ps1"
if (-not (Test-Path -LiteralPath $launcherScript -PathType Leaf)) {
    throw "Missing launcher script: $launcherScript"
}
$launcherBuildScript = Join-Path $RepoRoot "scripts\launcher\build-launcher-exe.ps1"
if (-not (Test-Path -LiteralPath $launcherBuildScript -PathType Leaf)) {
    throw "Missing launcher build script: $launcherBuildScript"
}
$releasePackageScript = Join-Path $RepoRoot "scripts\release\package.ps1"
if (-not (Test-Path -LiteralPath $releasePackageScript -PathType Leaf)) {
    throw "Missing release package script: $releasePackageScript"
}
$launcherText = Get-Content -Encoding UTF8 -Raw -LiteralPath $launcherScript
$launcherCorePath = Join-Path $RepoRoot "scripts\launcher\eplus-rs-launch\core.ps1"
Assert-File -Path $launcherCorePath -Description "launcher PowerShell core"
$launcherCoreText = Get-Content -Encoding UTF8 -Raw -LiteralPath $launcherCorePath
$launcherCsPath = Join-Path $RepoRoot "scripts\launcher\eplus-rs-launcher.cs"
Assert-File -Path $launcherCsPath -Description "direct C# launcher source"
$launcherCsText = Get-Content -Encoding UTF8 -Raw -LiteralPath $launcherCsPath
Assert-Matches -Text $launcherText -Pattern "Resolve-EplusRsExe" -Description "launcher binary resolver"
Assert-Matches -Text $launcherCoreText -Pattern '(?s)bin\\eplus-rs\.exe.*?target\\release\\eplus-rs\.exe.*?target\\debug\\eplus-rs\.exe' -Description "PowerShell launcher packaged-bin then release-before-debug resolver order"
Assert-Matches -Text $launcherText -Pattern "New-LauncherRunArguments" -Description "launcher run command builder"
Assert-Matches -Text $launcherText -Pattern "Cancel-Run" -Description "launcher cancel command"
Assert-Matches -Text $launcherText -Pattern "Read-RunSummaryStatus" -Description "launcher run-summary reader"
Assert-Matches -Text $launcherText -Pattern "Read-RunDiagnostics" -Description "launcher diagnostics reader"
Assert-Matches -Text $launcherText -Pattern "support-report\.md" -Description "launcher support report link"
Assert-Matches -Text $launcherText -Pattern ([regex]::Escape('Open-Path -Path $script:OutputDir')) -Description "launcher open output handler"
Assert-Matches -Text $launcherText -Pattern "System\.Windows\.Forms\.OpenFileDialog" -Description "launcher input/weather file picker"
Assert-Matches -Text $launcherText -Pattern "System\.Windows\.Forms\.FolderBrowserDialog" -Description "launcher output/oracle directory picker"
Assert-Matches -Text $launcherText -Pattern "Claim Boundary" -Description "launcher claim boundary tab"
Assert-Matches -Text $launcherText -Pattern "Fast and experimental modes are never release conformance evidence" -Description "launcher fast/experimental claim boundary"
Assert-NotMatches -Text $launcherText -Pattern "full EnergyPlus compatible" -Description "launcher forbidden full compatibility wording"
Assert-Matches -Text $launcherCsText -Pattern "OpenFileDialog" -Description "direct launcher input/weather file picker"
Assert-Matches -Text $launcherCsText -Pattern "FolderBrowserDialog" -Description "direct launcher output/oracle directory picker"
Assert-Matches -Text $launcherCsText -Pattern "Claim Boundary" -Description "direct launcher claim boundary tab"
Assert-Matches -Text $launcherCsText -Pattern "Fast and experimental modes are never release conformance evidence" -Description "direct launcher fast/experimental claim boundary"
Assert-NotMatches -Text $launcherCsText -Pattern "full EnergyPlus compatible" -Description "direct launcher forbidden full compatibility wording"
Assert-Matches -Text $launcherCsText -Pattern '(?s)Path\.Combine\(appRoot, "bin", "eplus-rs\.exe"\).*?Path\.Combine\(appRoot, "target", "release", "eplus-rs\.exe"\).*?Path\.Combine\(appRoot, "target", "debug", "eplus-rs\.exe"\)' -Description "direct launcher packaged-bin then release-before-debug app-root order"
Assert-Matches -Text $launcherCsText -Pattern '(?s)Path\.Combine\(repoRoot, "target", "release", "eplus-rs\.exe"\).*?Path\.Combine\(repoRoot, "target", "debug", "eplus-rs\.exe"\)' -Description "direct launcher release-before-debug repo-root order"
foreach ($workflowText in @(
        "IDF / epJSON",
        "Weather EPW",
        "Output Folder",
        "Mode",
        "Partial",
        "Oracle Baseline",
        "Oracle Compare",
        "Trace",
        "Run",
        "Cancel",
        "Open Output",
        "Open Support Report",
        "Diagnostics",
        "Oracle Compare",
        "Claim Boundary"
    )) {
    Assert-Matches -Text $launcherCsText -Pattern ([regex]::Escape($workflowText)) -Description "direct launcher workflow control $workflowText"
}
Assert-Matches -Text $launcherText -Pattern "ScreenshotPath" -Description "launcher screenshot capture option"
Assert-NotMatches -Text $launcherText -Pattern "support-assessment\s" -Description "launcher support pre-step command"
foreach ($progressStage in @("Input", "Convert", "RawModel", "TypedModel", "Graph", "Support", "Plan", "Runtime", "Export", "Oracle", "Compare")) {
    Assert-Matches -Text $launcherText -Pattern ([regex]::Escape($progressStage)) -Description "launcher progress stage $progressStage"
}
$releasePackageText = Get-Content -Encoding UTF8 -Raw -LiteralPath $releasePackageScript
Assert-Matches -Text $releasePackageText -Pattern "build-launcher-exe\.ps1" -Description "release launcher build wiring"
Assert-Matches -Text $releasePackageText -Pattern "eplus-rs-launch\.exe" -Description "release launcher exe asset"

Write-Host "Running launcher script self-test."
$selfTestOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $launcherScript -SelfTest 2>&1
if ($LASTEXITCODE -ne 0) {
    $selfTestOutput | ForEach-Object { Write-Host $_ }
    throw "Launcher script self-test failed with exit code $LASTEXITCODE"
}
$selfTest = ($selfTestOutput -join "`n") | ConvertFrom-Json
Assert-Equal -Actual $selfTest.self_test -Expected "passed" -Description "launcher self-test status"
$candidatePaths = @($selfTest.eplus_rs_candidate_paths)
if ($candidatePaths.Count -lt 3) {
    throw "Launcher executable candidate list was shorter than the required packaged/release/debug paths."
}
Write-Host "OK launcher executable candidate count: $($candidatePaths.Count)"
Assert-Equal -Actual $candidatePaths[0] -Expected (Join-Path $RepoRoot "bin\eplus-rs.exe") -Description "launcher packaged binary priority"
Assert-Equal -Actual $candidatePaths[1] -Expected (Join-Path $RepoRoot "target\release\eplus-rs.exe") -Description "launcher release binary priority"
Assert-Equal -Actual $candidatePaths[2] -Expected (Join-Path $RepoRoot "target\debug\eplus-rs.exe") -Description "launcher debug binary fallback"
Assert-Equal -Actual $selfTest.rejects_stale_saved_exe -Expected $true -Description "launcher stale saved executable rejection"

$smokeRoot = Join-Path $RepoRoot ".runtime\launcher-smoke"
New-Item -ItemType Directory -Force -Path $smokeRoot | Out-Null
$screenshotPath = Join-Path $smokeRoot "launcher-ready.png"
if (Test-Path -LiteralPath $screenshotPath) {
    Remove-Item -Force -LiteralPath $screenshotPath
}
Write-Host "Capturing launcher screenshot evidence: $screenshotPath"
$screenshotOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $launcherScript -ScreenshotPath $screenshotPath 2>&1
if ($LASTEXITCODE -ne 0) {
    $screenshotOutput | ForEach-Object { Write-Host $_ }
    throw "Launcher screenshot capture failed with exit code $LASTEXITCODE"
}
Assert-File -Path $screenshotPath -Description "launcher screenshot evidence"
$screenshotInfo = Get-Item -LiteralPath $screenshotPath
if ($screenshotInfo.Length -le 1000) {
    throw "Launcher screenshot evidence is unexpectedly small: $($screenshotInfo.Length) bytes"
}
Write-Host "OK launcher screenshot bytes: $($screenshotInfo.Length)"

$diagnosticArgs = @($selfTest.diagnostic_arguments)
foreach ($required in @("run", "--mode", "diagnostic", "--partial", "allow", "--format", "rust-native", "--trace-level", "debug", "--fail-on-warning", "--overwrite")) {
    Assert-ContainsValue -Values $diagnosticArgs -Expected $required -Description "diagnostic command argument"
}
foreach ($unexpected in @("-w", "--oracle-baseline", "--compare-oracle")) {
    Assert-NotContainsValue -Values $diagnosticArgs -Unexpected $unexpected -Description "diagnostic command argument"
}

$baselineArgs = @($selfTest.baseline_arguments)
foreach ($required in @("run", "input.idf", "-w", "weather.epw", "--mode", "compatibility", "--partial", "deny", "--oracle-baseline", "--oracle-root", "oracle-root")) {
    Assert-ContainsValue -Values $baselineArgs -Expected $required -Description "baseline command argument"
}
Assert-NotContainsValue -Values $baselineArgs -Unexpected "--compare-oracle" -Description "baseline command argument"

$compareArgs = @($selfTest.compare_arguments)
foreach ($required in @("run", "input.idf", "-w", "weather.epw", "--mode", "compatibility", "--partial", "deny", "--format", "both", "--trace-level", "detailed", "--compare-oracle", "--oracle-root", "oracle-root")) {
    Assert-ContainsValue -Values $compareArgs -Expected $required -Description "compare command argument"
}
Assert-NotContainsValue -Values $compareArgs -Unexpected "--oracle-baseline" -Description "compare command argument"

$presentations = @($selfTest.state_presentations)
foreach ($state in @("run_blocked", "partial_supported_run", "supported_compatibility_run")) {
    $match = @($presentations | Where-Object { $_.state_id -eq $state })
    if ($match.Count -lt 1) {
        throw "Missing launcher state presentation: $state"
    }
    Assert-Matches -Text ([string]$match[0].detail) -Pattern "exit_code=" -Description "$state exit code detail"
    Assert-Matches -Text ([string]$match[0].detail) -Pattern "conformance_claim=false" -Description "$state claim boundary"
    Assert-NotMatches -Text ([string]$match[0].detail) -Pattern "full EnergyPlus compatible" -Description "$state forbidden full compatibility wording"
}
$blockedPresentation = @($presentations | Where-Object {
        $_.state_id -eq "run_blocked" -and
        $_.color -eq "Firebrick" -and
        $_.title -eq "Simulation was not run." -and
        $_.detail -match "Simulation was not run\."
    })
Assert-Equal -Actual $blockedPresentation.Count -Expected 2 -Description "blocked run red state wording"
$partialPresentation = @($presentations | Where-Object {
        $_.state_id -eq "partial_supported_run" -and
        $_.color -eq "DarkGoldenrod" -and
        $_.title -eq "Ad-hoc partial run, not conformance evidence." -and
        $_.detail -match "Ad-hoc partial run, not conformance evidence\."
    })
Assert-Equal -Actual $partialPresentation.Count -Expected 1 -Description "partial run yellow state wording"
$compatPresentation = @($presentations | Where-Object {
        $_.state_id -eq "supported_compatibility_run" -and
        $_.color -eq "ForestGreen" -and
        $_.detail -match "matched_capabilities="
    })
Assert-Equal -Actual $compatPresentation.Count -Expected 1 -Description "supported run green matched capabilities"
$fastPresentation = @($presentations | Where-Object {
        $_.detail -match "mode=fast" -and
        $_.detail -match "Fast and experimental modes are never release conformance evidence" -and
        $_.detail -match "conformance_claim=false"
    })
Assert-Equal -Actual $fastPresentation.Count -Expected 1 -Description "fast/experimental non-evidence boundary"
$blockedOracle = @($presentations | Where-Object {
        $_.state_id -eq "run_blocked" -and
        $_.detail -match "oracle=generated" -and
        $_.detail -match "compare=skipped-rust-unsupported-or-oracle-missing"
    })
Assert-Equal -Actual $blockedOracle.Count -Expected 1 -Description "blocked run oracle/compare presentation"
Assert-Matches -Text ([string]$selfTest.phase_line) -Pattern "support_assessment" -Description "phase timing support assessment"
Assert-Matches -Text ([string]$selfTest.phase_line) -Pattern "ep_run" -Description "phase timing engine"

$launcherExe = Join-Path $smokeRoot "eplus-rs-launch.exe"
Write-Host "Building launcher executable self-test: $launcherExe"
$buildOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $launcherBuildScript -OutputPath $launcherExe -SelfTest 2>&1
if ($LASTEXITCODE -ne 0) {
    $buildOutput | ForEach-Object { Write-Host $_ }
    throw "Launcher executable self-test failed with exit code $LASTEXITCODE"
}
$build = ($buildOutput -join "`n") | ConvertFrom-Json
Assert-Equal -Actual $build.output_type -Expected "WindowsApplication" -Description "launcher executable output type"
Assert-Equal -Actual $build.prefers_release_over_debug -Expected $true -Description "direct launcher release-before-debug self-test"
Assert-Equal -Actual $build.rejects_stale_saved_exe -Expected $true -Description "direct launcher stale saved executable guard"
Assert-Equal -Actual $build.migrates_legacy_debug_to_release -Expected $true -Description "direct launcher legacy debug migration guard"
if ([int64]$build.bytes -le 0) {
    throw "Launcher executable was empty: $($build.output_path)"
}
Write-Host "OK launcher executable bytes: $($build.bytes)"

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Building release eplus-rs CLI for launcher run validation with at most 12 jobs."
& $cargo.Source build -p ep_cli --release --jobs 12 --quiet
if ($LASTEXITCODE -ne 0) {
    throw "Failed to build ep_cli."
}

$script:CliExe = Join-Path $RepoRoot "target\release\eplus-rs.exe"
Assert-File -Path $script:CliExe -Description "launcher CLI binary"

$fixtureRoot = Join-Path $smokeRoot "fixtures"
New-Item -ItemType Directory -Force -Path $fixtureRoot | Out-Null

$weatherPath = Join-Path $fixtureRoot "one-day.epw"
$weatherLines = @(
@'
LOCATION,Example
DESIGN CONDITIONS,0
TYPICAL/EXTREME PERIODS,0
GROUND TEMPERATURES,0
HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0
COMMENTS 1
COMMENTS 2
DATA PERIODS,1,1,Data,Friday,1/1,1/1
'@
)
foreach ($hour in 1..24) {
    $weatherLines += "1999,1,1,$hour,60,Source,-3.0,-4.0,50,82000,0,0,0,0,0,0,0,0,0,0,180,2.5"
}
Write-Utf8NoBomFile -Path $weatherPath -Contents (($weatherLines -join "`n") + "`n")

$oneZonePath = Join-Path $fixtureRoot "one-zone.epJSON"
Write-Utf8NoBomFile -Path $oneZonePath -Contents @'
{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Building": {"Defaulted Building": {"terrain": "Suburbs"}},
  "Timestep": {"Timestep 1": {}},
  "Site:Location": {"Denver Site": {"latitude": 39.74, "longitude": -105.18}},
  "Material:NoMass": {"R13": {"roughness": "Rough", "thermal_resistance": 2.29}},
  "Construction": {"Wall Construction": {"outside_layer": "R13"}},
  "ScheduleTypeLimits": {
    "Fraction": {
      "lower_limit_value": 0.0,
      "numeric_type": "Continuous",
      "upper_limit_value": 1.0
    }
  },
  "Schedule:Constant": {
    "Always On": {"schedule_type_limits_name": "Fraction"}
  },
  "Zone": {"Zone One": {"volume": 100}},
  "BuildingSurface:Detailed": {
    "Wall One": {
      "construction_name": "Wall Construction",
      "outside_boundary_condition": "Outdoors",
      "surface_type": "Wall",
      "vertices": [
        {"vertex_x_coordinate": 0.0, "vertex_y_coordinate": 0.0, "vertex_z_coordinate": 0.0},
        {"vertex_x_coordinate": 1.0, "vertex_y_coordinate": 0.0, "vertex_z_coordinate": 0.0},
        {"vertex_x_coordinate": 1.0, "vertex_y_coordinate": 1.0, "vertex_z_coordinate": 0.0},
        {"vertex_x_coordinate": 0.0, "vertex_y_coordinate": 1.0, "vertex_z_coordinate": 0.0}
      ],
      "zone_name": "Zone One"
    }
  }
}
'@

$idealLoadsPath = Join-Path $fixtureRoot "ideal-loads.epJSON"
Write-Utf8NoBomFile -Path $idealLoadsPath -Contents @'
{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Timestep": {"Timestep 1": {"number_of_timesteps_per_hour": 1}},
  "Zone": {"Zone One": {"volume": 100}},
  "Schedule:Constant": {
    "Control Type": {"hourly_value": 4},
    "Heating Setpoint": {"hourly_value": 21},
    "Cooling Setpoint": {"hourly_value": 24}
  },
  "ThermostatSetpoint:DualSetpoint": {
    "Dual Setpoints": {
      "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
      "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
    }
  },
  "ZoneControl:Thermostat": {
    "Zone Thermostat": {
      "zone_or_zonelist_name": "Zone One",
      "control_type_schedule_name": "Control Type",
      "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
      "control_1_name": "Dual Setpoints"
    }
  },
  "NodeList": {
    "Zone Inlets": {
      "nodes": [{"node_name": "Zone One Inlet"}]
    }
  },
  "ZoneHVAC:IdealLoadsAirSystem": {
    "Zone Ideal Loads": {
      "zone_supply_air_node_name": "Zone Inlets",
      "dehumidification_control_type": "None",
      "humidification_control_type": "None"
    }
  },
  "ZoneHVAC:EquipmentList": {
    "Zone Equipment": {
      "equipment": [
        {
          "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
          "zone_equipment_name": "Zone Ideal Loads",
          "zone_equipment_cooling_sequence": 1,
          "zone_equipment_heating_or_no_load_sequence": 1
        }
      ]
    }
  },
  "ZoneHVAC:EquipmentConnections": {
    "Zone One": {
      "zone_name": "Zone One",
      "zone_conditioning_equipment_list_name": "Zone Equipment",
      "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
      "zone_air_node_name": "Zone One Air Node",
      "zone_return_air_node_or_nodelist_name": "Zone One Return"
    }
  }
}
'@

$airLoopPath = Join-Path $fixtureRoot "air-loop.epJSON"
Write-Utf8NoBomFile -Path $airLoopPath -Contents @'
{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Zone": {"Zone One": {"volume": 100}},
  "AirLoopHVAC": {"Main Air Loop": {}}
}
'@

$emsPath = Join-Path $fixtureRoot "ems.epJSON"
Write-Utf8NoBomFile -Path $emsPath -Contents @'
{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Zone": {"Zone One": {"volume": 100}},
  "EnergyManagementSystem:Program": {
    "Override Program": {
      "lines": [{"program_line": "SET X = 1"}]
    }
  }
}
'@

$oneZoneOutput = Join-Path $smokeRoot "one-zone-output"
$oneZoneSummary = Invoke-LauncherCliRun `
    -Description "supported 1Zone fixture" `
    -Arguments @("run", $oneZonePath, "-w", $weatherPath, "-d", $oneZoneOutput, "--mode", "compatibility", "--partial", "deny", "--format", "rust-native", "--trace-level", "normal", "--overwrite") `
    -ExpectedExitCode 0 `
    -OutputDir $oneZoneOutput
Assert-LauncherRunSummary -Summary $oneZoneSummary -ExpectedStatus "success" -ExpectedExitCode 0 -ExpectedRunResultState "supported_compatibility_run" -Description "supported 1Zone"
Assert-LauncherArtifactSchema -Summary $oneZoneSummary -OutputDir $oneZoneOutput -Description "supported 1Zone"
Assert-ContainsValue -Values @($oneZoneSummary.support.matched_capability_ids) -Expected "official_1zone_uncontrolled_declared_heat_balance" -Description "supported 1Zone matched capability"
Assert-Directory -Path $oneZoneOutput -Description "launcher output folder"
Assert-File -Path (Join-Path $oneZoneOutput "results\result-store.json") -Description "supported 1Zone result store"

$idealLoadsOutput = Join-Path $smokeRoot "ideal-loads-output"
$idealLoadsSummary = Invoke-LauncherCliRun `
    -Description "supported IdealLoads fixture" `
    -Arguments @("run", $idealLoadsPath, "-w", $weatherPath, "-d", $idealLoadsOutput, "--mode", "compatibility", "--partial", "deny", "--format", "rust-native", "--trace-level", "normal", "--overwrite") `
    -ExpectedExitCode 0 `
    -OutputDir $idealLoadsOutput
Assert-LauncherRunSummary -Summary $idealLoadsSummary -ExpectedStatus "success" -ExpectedExitCode 0 -ExpectedRunResultState "supported_compatibility_run" -Description "supported IdealLoads"
Assert-LauncherArtifactSchema -Summary $idealLoadsSummary -OutputDir $idealLoadsOutput -Description "supported IdealLoads"
Assert-ContainsValue -Values @($idealLoadsSummary.support.matched_capability_ids) -Expected "ideal_loads_no_oa_sensible" -Description "supported IdealLoads matched capability"
Assert-Equal -Actual $idealLoadsSummary.support.runtime_class -Expected "ideal-loads-direct-zone-coupled-compatibility" -Description "supported IdealLoads runtime class"
Assert-Equal -Actual $idealLoadsSummary.rust_runtime.zone_demand_source -Expected "rust-predictor-source-setpoint-thresholds" -Description "supported IdealLoads demand source"
Assert-Equal -Actual $idealLoadsSummary.rust_runtime.fixture_demand_injection_used -Expected $false -Description "supported IdealLoads fixture demand flag"

$airLoopOutput = Join-Path $smokeRoot "air-loop-output"
$airLoopSummary = Invoke-LauncherCliRun `
    -Description "AirLoop blocked fixture" `
    -Arguments @("run", $airLoopPath, "-d", $airLoopOutput, "--mode", "compatibility", "--partial", "deny", "--format", "rust-native", "--trace-level", "normal", "--overwrite") `
    -ExpectedExitCode 4 `
    -OutputDir $airLoopOutput
Assert-LauncherRunSummary -Summary $airLoopSummary -ExpectedStatus "unsupported" -ExpectedExitCode 4 -ExpectedRunResultState "run_blocked" -Description "AirLoop blocked"
Assert-LauncherArtifactSchema -Summary $airLoopSummary -OutputDir $airLoopOutput -Description "AirLoop blocked"
Assert-Matches -Text (Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $airLoopOutput "diagnostics.json")) -Pattern "UnsupportedHVACObject" -Description "AirLoop blocked diagnostics"

$plantLoopPath = Join-Path $RepoRoot "data\testcases\minimal\plant-loop-skeleton.epJSON"
Assert-File -Path $plantLoopPath -Description "PlantLoop fixture"
$plantLoopOutput = Join-Path $smokeRoot "plant-loop-output"
$plantLoopSummary = Invoke-LauncherCliRun `
    -Description "PlantLoop blocked fixture" `
    -Arguments @("run", $plantLoopPath, "-d", $plantLoopOutput, "--mode", "compatibility", "--partial", "deny", "--format", "rust-native", "--trace-level", "normal", "--overwrite") `
    -ExpectedExitCode 4 `
    -OutputDir $plantLoopOutput
Assert-LauncherRunSummary -Summary $plantLoopSummary -ExpectedStatus "unsupported" -ExpectedExitCode 4 -ExpectedRunResultState "run_blocked" -Description "PlantLoop blocked"
Assert-LauncherArtifactSchema -Summary $plantLoopSummary -OutputDir $plantLoopOutput -Description "PlantLoop blocked"
Assert-Matches -Text (Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $plantLoopOutput "diagnostics.json")) -Pattern "UnsupportedPlantObject" -Description "PlantLoop blocked diagnostics"

$emsOutput = Join-Path $smokeRoot "ems-output"
$emsSummary = Invoke-LauncherCliRun `
    -Description "EMS blocked fixture" `
    -Arguments @("run", $emsPath, "-d", $emsOutput, "--mode", "compatibility", "--partial", "deny", "--format", "rust-native", "--trace-level", "normal", "--overwrite") `
    -ExpectedExitCode 4 `
    -OutputDir $emsOutput
Assert-LauncherRunSummary -Summary $emsSummary -ExpectedStatus "unsupported" -ExpectedExitCode 4 -ExpectedRunResultState "run_blocked" -Description "EMS blocked"
Assert-LauncherArtifactSchema -Summary $emsSummary -OutputDir $emsOutput -Description "EMS blocked"
Assert-Matches -Text (Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $emsOutput "diagnostics.json")) -Pattern "UnsupportedEMS" -Description "EMS blocked diagnostics"

$missingWeatherOutput = Join-Path $smokeRoot "missing-weather-output"
$missingWeatherSummary = Invoke-LauncherCliRun `
    -Description "missing weather diagnostic fixture" `
    -Arguments @("run", $oneZonePath, "-d", $missingWeatherOutput, "--mode", "compatibility", "--partial", "deny", "--format", "rust-native", "--trace-level", "normal", "--overwrite") `
    -ExpectedExitCode 1 `
    -OutputDir $missingWeatherOutput
Assert-LauncherRunSummary -Summary $missingWeatherSummary -ExpectedStatus "args" -ExpectedExitCode 1 -ExpectedRunResultState "supported_compatibility_run" -Description "missing weather"
Assert-LauncherArtifactSchema -Summary $missingWeatherSummary -OutputDir $missingWeatherOutput -Description "missing weather"
Assert-Matches -Text (Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $missingWeatherOutput "diagnostics.json")) -Pattern "MissingWeatherFile" -Description "missing weather diagnostics"

$oracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$oracleIdf = Join-Path $oracleRoot "ExampleFiles\1ZoneUncontrolled.idf"
$oracleWeather = Join-Path $oracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
$energyplusExe = Join-Path $oracleRoot "energyplus.exe"
$convertExe = Join-Path $oracleRoot "ConvertInputFormat.exe"
foreach ($required in @($oracleIdf, $oracleWeather, $energyplusExe, $convertExe)) {
    Assert-File -Path $required -Description "launcher oracle prerequisite"
}

$baselineOutput = Join-Path $smokeRoot "baseline-output"
$baselineSummary = Invoke-LauncherCliRun `
    -Description "oracle baseline fixture" `
    -Arguments @("run", $oracleIdf, "-w", $oracleWeather, "-d", $baselineOutput, "--mode", "compatibility", "--partial", "deny", "--format", "rust-native", "--trace-level", "normal", "--overwrite", "--oracle-baseline", "--oracle-root", $oracleRoot) `
    -ExpectedExitCode 0 `
    -OutputDir $baselineOutput
Assert-LauncherRunSummary -Summary $baselineSummary -ExpectedStatus "success" -ExpectedExitCode 0 -ExpectedRunResultState "supported_compatibility_run" -Description "oracle baseline"
Assert-LauncherArtifactSchema -Summary $baselineSummary -OutputDir $baselineOutput -Description "oracle baseline"
Assert-Equal -Actual $baselineSummary.oracle_status -Expected "generated" -Description "oracle baseline oracle status"
Assert-File -Path (Join-Path $baselineOutput "oracle\eplusout.eso") -Description "launcher oracle baseline output"

$compareOutput = Join-Path $smokeRoot "compare-output"
$compareSummary = Invoke-LauncherCliRun `
    -Description "oracle compare fixture" `
    -Arguments @("run", $oracleIdf, "-w", $oracleWeather, "-d", $compareOutput, "--mode", "compatibility", "--partial", "deny", "--format", "both", "--trace-level", "normal", "--overwrite", "--compare-oracle", "--oracle-root", $oracleRoot) `
    -ExpectedExitCode 8 `
    -OutputDir $compareOutput
Assert-LauncherRunSummary -Summary $compareSummary -ExpectedStatus "oracle-compare" -ExpectedExitCode 8 -ExpectedRunResultState "supported_compatibility_run" -Description "oracle compare"
Assert-LauncherArtifactSchema -Summary $compareSummary -OutputDir $compareOutput -Description "oracle compare"
Assert-Equal -Actual $compareSummary.oracle_status -Expected "generated" -Description "oracle compare oracle status"
Assert-Equal -Actual $compareSummary.compare_status -Expected "fail" -Description "oracle compare status"
Assert-File -Path (Join-Path $compareOutput "compare\compare-report.md") -Description "launcher compare report"

Write-Host "Launcher smoke passed."
