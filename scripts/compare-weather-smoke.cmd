@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0compare-weather-smoke.ps1" %*
exit /b %ERRORLEVEL%
