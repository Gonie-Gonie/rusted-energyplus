@echo off
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0github-release.ps1" %*

