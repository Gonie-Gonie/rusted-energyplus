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

function Get-ReleaseFileHash {
    param(
        [Parameter(Mandatory = $true)][string]$LiteralPath,
        [ValidateSet("SHA256")][string]$Algorithm = "SHA256"
    )
    $stream = [System.IO.File]::OpenRead($LiteralPath)
    $hasher = [System.Security.Cryptography.SHA256]::Create()
    try {
        $hash = [System.BitConverter]::ToString($hasher.ComputeHash($stream)).Replace("-", "")
        return [pscustomobject]@{ Algorithm = $Algorithm; Hash = $hash; Path = $LiteralPath }
    }
    finally {
        $hasher.Dispose()
        $stream.Dispose()
    }
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
        [string[]]$Arguments = @(),
        [switch]$Plan
    )

    if ($Plan) {
        Write-Host (".\scripts\dev.cmd " + $Command + " " + ($Arguments -join " ")).TrimEnd()
        return
    }
    $runner = Join-Path (Get-ScriptsRoot) "dev.ps1"
    & $runner $Command @Arguments
    if (-not $?) {
        throw "Script command failed: $Command"
    }
}
