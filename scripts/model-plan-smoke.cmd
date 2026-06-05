@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0model-plan-smoke.ps1" %*
exit /b %ERRORLEVEL%
