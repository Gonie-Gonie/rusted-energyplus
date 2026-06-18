@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\gui\eplus-rs-launch.ps1"
exit /b %ERRORLEVEL%
