@echo off
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0compare-regression.ps1" %*

