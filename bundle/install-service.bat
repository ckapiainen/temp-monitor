@echo off
echo ========================================
echo Libre Hardware Monitor Service Installer
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

REM Get the directory where this script is located
set "INSTALL_DIR=%~dp0"

REM Check if service executable exists
if not exist "%INSTALL_DIR%lhm-service.exe" (
    echo ERROR: lhm-service.exe not found in %INSTALL_DIR%
    echo Please ensure the service executable is in the same directory as this script
    echo.
    pause
    exit /b 1
)

echo Installing service...
echo Location: %INSTALL_DIR%lhm-service.exe
echo.

REM Stop service if it's already running
sc query LibreHardwareMonitorService >nul 2>&1
if %errorLevel% equ 0 (
    echo Stopping existing service...
    sc stop LibreHardwareMonitorService >nul 2>&1
    timeout /t 2 /nobreak >nul
)

REM Delete service if it already exists
sc query LibreHardwareMonitorService >nul 2>&1
if %errorLevel% equ 0 (
    echo Removing existing service...
    sc delete LibreHardwareMonitorService >nul 2>&1
    timeout /t 1 /nobreak >nul
)

REM Create the service
echo Creating service...
sc create LibreHardwareMonitorService binPath= "%INSTALL_DIR%lhm-service.exe" start= auto DisplayName= "Libre Hardware Monitor Service" type= own

if %errorLevel% neq 0 (
    echo ERROR: Failed to create service
    echo.
    pause
    exit /b 1
)

REM Start the service
echo Starting service...
sc start LibreHardwareMonitorService

if %errorLevel% neq 0 (
    echo WARNING: Service created but failed to start
    echo You can try starting it manually from Services (services.msc)
    echo.
    pause
    exit /b 1
)

echo.
echo ========================================
echo SUCCESS: Service installed and started!
echo ========================================
echo.
echo The Libre Hardware Monitor Service is now running
echo and will start automatically when Windows boots.
echo.
pause
