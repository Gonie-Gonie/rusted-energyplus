@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0compare-zone-smoke.ps1" %*
exit /b %ERRORLEVEL%
