@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0compare-internal-gains-smoke.ps1" %*
exit /b %ERRORLEVEL%
