@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0schedule-compact-smoke.ps1" %*
exit /b %ERRORLEVEL%
