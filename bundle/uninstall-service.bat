@echo off
echo ========================================
echo Libre Hardware Monitor Service Uninstaller
echo ========================================
echo.

REM Check for admin privileges
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ERROR: This script requires Administrator privileges
    echo Please right-click and select "Run as Administrator"
    echo.
    pause
    exit /b 1
)

REM Check if service exists
sc query LibreHardwareMonitorService >nul 2>&1
if %errorLevel% neq 0 (
    echo Service is not installed
    echo.
    pause
    exit /b 0
)

REM Stop the service
echo Stopping service...
sc stop LibreHardwareMonitorService >nul 2>&1

if %errorLevel% neq 0 (
    echo Service may already be stopped or not responding
)

REM Wait a moment for service to stop
timeout /t 2 /nobreak >nul

REM Delete the service
echo Removing service...
sc delete LibreHardwareMonitorService

if %errorLevel% neq 0 (
    echo ERROR: Failed to remove service
    echo.
    pause
    exit /b 1
)

echo.
echo ========================================
echo SUCCESS: Service uninstalled!
echo ========================================
echo.
pause
