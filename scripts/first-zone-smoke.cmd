@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0first-zone-smoke.ps1" %*
exit /b %ERRORLEVEL%
