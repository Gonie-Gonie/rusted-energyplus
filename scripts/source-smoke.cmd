@echo off
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0source-smoke.ps1" %*

