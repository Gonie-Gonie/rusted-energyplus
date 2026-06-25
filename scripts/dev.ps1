param(
    [Parameter(Position = 0)][string]$Command = "list",
    [Parameter(Position = 1, ValueFromRemainingArguments = $true)][string[]]$CommandArgs = @()
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = $PSScriptRoot
$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptsRoot "..")).Path
$CatalogPath = Join-Path $ScriptsRoot "dev\commands.json"

if (-not (Test-Path -LiteralPath $CatalogPath -PathType Leaf)) {
    throw "Missing dev command catalog: $CatalogPath"
}

$Catalog = Get-Content -LiteralPath $CatalogPath -Encoding UTF8 -Raw | ConvertFrom-Json
$Groups = @($Catalog.groups | ForEach-Object { [string]$_ })

$Commands = [ordered]@{}
foreach ($entry in @($Catalog.commands)) {
    $name = [string]$entry.name
    if ([string]::IsNullOrWhiteSpace($name)) {
        throw "Dev command catalog contains an empty command name."
    }
    if ($Commands.Contains($name)) {
        throw "Duplicate dev command in catalog: $name"
    }

    $Commands[$name] = [pscustomobject]@{
        Path = [string]$entry.path
        Group = [string]$entry.group
        Help = [string]$entry.help
    }
}

$Aliases = @{}
foreach ($entry in $Catalog.aliases.PSObject.Properties) {
    $Aliases[$entry.Name] = [string]$entry.Value
}

function Show-Commands {
    Write-Host "Usage: .\scripts\dev.cmd <command> [args...]"
    Write-Host ""
    foreach ($group in $Groups) {
        Write-Host "[$group]"
        foreach ($name in $Commands.Keys) {
            $entry = $Commands[$name]
            if ($entry.Group -eq $group) {
                Write-Host ("  {0,-42} {1}" -f $name, $entry.Help)
            }
        }
        Write-Host ""
    }
}

function Convert-CommandArguments {
    param([string[]]$Values)

    $named = @{}
    $positional = @()
    for ($index = 0; $index -lt $Values.Count; $index += 1) {
        $value = $Values[$index]
        if ($value.StartsWith("-", [System.StringComparison]::Ordinal) -and $value.Length -gt 1) {
            $name = $value.TrimStart("-")
            $nextIndex = $index + 1
            if ($nextIndex -lt $Values.Count -and -not $Values[$nextIndex].StartsWith("-", [System.StringComparison]::Ordinal)) {
                $named[$name] = $Values[$nextIndex]
                $index += 1
            }
            else {
                $named[$name] = $true
            }
        }
        else {
            $positional += $value
        }
    }

    return [pscustomobject]@{
        Named = $named
        Positional = $positional
    }
}

if ($Command -in @("list", "help", "--help", "-h")) {
    Show-Commands
    return
}

$normalized = $Command
if ($normalized.EndsWith(".cmd", [System.StringComparison]::OrdinalIgnoreCase) -or
    $normalized.EndsWith(".ps1", [System.StringComparison]::OrdinalIgnoreCase)) {
    $normalized = [System.IO.Path]::GetFileNameWithoutExtension($normalized)
}

if ($Aliases.ContainsKey($normalized)) {
    $normalized = $Aliases[$normalized]
}

if (-not $Commands.Contains($normalized)) {
    Write-Error "Unknown script command: $Command"
    Show-Commands
    throw "Unknown script command: $Command"
}

$script = Join-Path $ScriptsRoot $Commands[$normalized].Path
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Command target is missing: $script"
}

Set-Location $RepoRoot
$bound = Convert-CommandArguments -Values $CommandArgs
$positionalArguments = $bound.Positional
$namedArguments = $bound.Named
& $script @positionalArguments @namedArguments
if (-not $?) {
    throw "Script command failed: $normalized"
}