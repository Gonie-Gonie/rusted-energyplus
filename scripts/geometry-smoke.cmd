@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0geometry-smoke.ps1" %*
exit /b %ERRORLEVEL%
