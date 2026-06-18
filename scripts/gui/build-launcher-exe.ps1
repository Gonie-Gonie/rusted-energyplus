[CmdletBinding()]
param(
    [string]$OutputPath = "",
    [switch]$SelfTest
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptsRoot "..")).Path

if ([string]::IsNullOrWhiteSpace($OutputPath)) {
    $OutputPath = Join-Path $RepoRoot "target\launcher\eplus-rs-launch.exe"
}

if (-not [System.IO.Path]::IsPathRooted($OutputPath)) {
    $OutputPath = Join-Path $RepoRoot $OutputPath
}

$OutputPath = [System.IO.Path]::GetFullPath($OutputPath)
$parent = Split-Path -Parent $OutputPath
if (-not (Test-Path -LiteralPath $parent)) {
    New-Item -ItemType Directory -Force -Path $parent | Out-Null
}
if (Test-Path -LiteralPath $OutputPath -PathType Leaf) {
    Remove-Item -LiteralPath $OutputPath -Force
}

$source = @'
using System;
using System.Diagnostics;
using System.IO;
using System.Text;
using System.Windows.Forms;

internal static class Program
{
    [STAThread]
    private static int Main(string[] args)
    {
        string executablePath = Application.ExecutablePath;
        string appRoot = Path.GetDirectoryName(executablePath) ?? Environment.CurrentDirectory;
        string scriptPath = Path.Combine(appRoot, "scripts", "gui", "eplus-rs-launch.ps1");
        if (!File.Exists(scriptPath))
        {
            MessageBox.Show(
                "Missing launcher script:\n" + scriptPath,
                "Rusted EnergyPlus Launch",
                MessageBoxButtons.OK,
                MessageBoxIcon.Error);
            return 2;
        }

        string powershellPath = FindPowerShell();
        if (String.IsNullOrWhiteSpace(powershellPath))
        {
            MessageBox.Show(
                "PowerShell was not found.",
                "Rusted EnergyPlus Launch",
                MessageBoxButtons.OK,
                MessageBoxIcon.Error);
            return 3;
        }

        string argumentText = "-NoProfile -ExecutionPolicy Bypass -STA -File " + Quote(scriptPath);
        foreach (string argument in args)
        {
            argumentText += " " + Quote(argument);
        }

        ProcessStartInfo startInfo = new ProcessStartInfo();
        startInfo.FileName = powershellPath;
        startInfo.Arguments = argumentText;
        startInfo.UseShellExecute = false;
        startInfo.CreateNoWindow = true;
        startInfo.WindowStyle = ProcessWindowStyle.Hidden;

        try
        {
            Process.Start(startInfo);
            return 0;
        }
        catch (Exception error)
        {
            MessageBox.Show(
                "Failed to open the launcher:\n" + error.Message,
                "Rusted EnergyPlus Launch",
                MessageBoxButtons.OK,
                MessageBoxIcon.Error);
            return 4;
        }
    }

    private static string FindPowerShell()
    {
        string windows = Environment.GetFolderPath(Environment.SpecialFolder.Windows);
        string systemPowerShell = Path.Combine(windows, "System32", "WindowsPowerShell", "v1.0", "powershell.exe");
        if (File.Exists(systemPowerShell))
        {
            return systemPowerShell;
        }
        return "powershell.exe";
    }

    private static string Quote(string value)
    {
        if (value == null)
        {
            return "\"\"";
        }
        if (value.Length == 0)
        {
            return "\"\"";
        }
        bool needsQuotes = false;
        foreach (char character in value)
        {
            if (Char.IsWhiteSpace(character) || character == '"')
            {
                needsQuotes = true;
                break;
            }
        }
        if (!needsQuotes)
        {
            return value;
        }

        StringBuilder builder = new StringBuilder();
        builder.Append('"');
        int backslashes = 0;
        foreach (char character in value)
        {
            if (character == '\\')
            {
                backslashes += 1;
                continue;
            }
            if (character == '"')
            {
                builder.Append('\\', (backslashes * 2) + 1);
                builder.Append('"');
                backslashes = 0;
                continue;
            }
            if (backslashes > 0)
            {
                builder.Append('\\', backslashes);
                backslashes = 0;
            }
            builder.Append(character);
        }
        if (backslashes > 0)
        {
            builder.Append('\\', backslashes * 2);
        }
        builder.Append('"');
        return builder.ToString();
    }
}
'@

Add-Type `
    -TypeDefinition $source `
    -Language CSharp `
    -ReferencedAssemblies @("System.dll", "System.Windows.Forms.dll", "System.Drawing.dll") `
    -OutputAssembly $OutputPath `
    -OutputType WindowsApplication

if (-not (Test-Path -LiteralPath $OutputPath -PathType Leaf)) {
    throw "Launcher executable was not created: $OutputPath"
}

$item = Get-Item -LiteralPath $OutputPath
if ($SelfTest) {
    [pscustomobject]@{
        output_path = $item.FullName
        bytes = $item.Length
        output_type = "WindowsApplication"
        script_path = "scripts\gui\eplus-rs-launch.ps1"
    } | ConvertTo-Json -Depth 3
}
else {
    Write-Host "Launcher executable created: $($item.FullName)"
}
