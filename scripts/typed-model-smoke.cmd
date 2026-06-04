@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0typed-model-smoke.ps1" %*
exit /b %ERRORLEVEL%
