@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0strict-no-false-conformance.ps1" %*
exit /b %ERRORLEVEL%
