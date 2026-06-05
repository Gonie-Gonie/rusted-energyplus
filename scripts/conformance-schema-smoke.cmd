@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0conformance-schema-smoke.ps1" %*
exit /b %ERRORLEVEL%
