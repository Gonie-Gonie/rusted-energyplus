$ProjectPythonVersion = "3.11.9"
$ProjectPythonArchiveName = "python-$ProjectPythonVersion-nuget.zip"
$ProjectPythonArchiveUrl = "https://www.nuget.org/api/v2/package/python/$ProjectPythonVersion"
$ProjectPythonArchiveSha256 = "9283876d58c017e0e846f95b490da3bca0fc0a6ee1134b2870677cfb7eec3c67"
$ProjectOodocsVersion = "1.0.1"
$ProjectMatplotlibVersion = "3.10.9"

function Get-PortablePythonRoot {
    return (Join-Path (Get-RepoRoot) ".runtime\python\$ProjectPythonVersion")
}

function Get-PortablePythonExe {
    return (Join-Path (Get-PortablePythonRoot) "python.exe")
}

function Get-ReportVenvRoot {
    return (Join-Path (Get-RepoRoot) ".runtime\python-venvs\report")
}

function Get-ReportPythonExe {
    return (Join-Path (Get-ReportVenvRoot) "Scripts\python.exe")
}

function Get-ReportRequirementsPath {
    return (Join-Path (Get-RepoRoot) "tools\python\requirements-report.txt")
}
