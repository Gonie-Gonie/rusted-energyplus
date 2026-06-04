@echo off
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0docs-check.ps1" %*

