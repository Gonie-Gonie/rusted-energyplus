function Add-CargoBinToPath {
    $cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    if (-not (Test-Path -LiteralPath $cargoBin)) {
        return
    }

    $parts = $env:Path -split ";"
    foreach ($part in $parts) {
        if ($part.Equals($cargoBin, [System.StringComparison]::OrdinalIgnoreCase)) {
            return
        }
    }

    $env:Path = "$cargoBin;$env:Path"
}

function Get-ScriptsRoot {
    return (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
}

function Get-RepoRoot {
    return (Resolve-Path -LiteralPath (Join-Path (Get-ScriptsRoot) "..")).Path
}

function Invoke-DevCommand {
    param(
        [Parameter(Mandatory = $true)][string]$Command,
        [string[]]$Arguments = @()
    )

    $runner = Join-Path (Get-ScriptsRoot) "dev.ps1"
    & $runner $Command @Arguments
    if (-not $?) {
        throw "Script command failed: $Command"
    }
}
