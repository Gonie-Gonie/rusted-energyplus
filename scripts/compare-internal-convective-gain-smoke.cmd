@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0compare-internal-convective-gain-smoke.ps1" %*
exit /b %ERRORLEVEL%
