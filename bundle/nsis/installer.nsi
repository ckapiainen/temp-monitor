!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "LogicLib.nsh"

; ===== Basic Definitions =====
!define PRODUCT_NAME "{{product_name}}"
!define PRODUCT_VERSION "{{version}}"
!define PRODUCT_PUBLISHER "{{publisher}}"
!define PRODUCT_INSTALL_DIR "$PROGRAMFILES64\${PRODUCT_NAME}"

; ===== Installer Attributes =====
Name "${PRODUCT_NAME}"
OutFile "{{out_file}}"
InstallDir "${PRODUCT_INSTALL_DIR}"
RequestExecutionLevel admin
SetCompressor /SOLID lzma

; ===== UI Configuration =====
!define MUI_ABORTWARNING
!define INSTALLERICON "{{installer_icon}}"
!if "${INSTALLERICON}" != ""
 !define MUI_ICON "${INSTALLERICON}"
 !define MUI_UNICON "${INSTALLERICON}"
!endif

; ===== Pages =====
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

; ===== Installation Section =====
Section "MainSection" SEC01
    SetOutPath "$INSTDIR"

    ; Install main binary
    File "{{main_binary_path}}"

    ; Install all resources
    {{#each resources}}
    File /a "/oname={{this}}" "{{@key}}"
    {{/each}}

    ; Check and install PawnIO driver
    DetailPrint "Checking for PawnIO driver..."
    ReadRegStr $0 HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\PawnIO" "DisplayName"
    ${If} $0 == ""
        DetailPrint "PawnIO not found, installing..."
        ExecWait '"$INSTDIR\PawnIO_setup.exe"' $1
        ${If} $1 == 0
            DetailPrint "PawnIO installed successfully"
        ${Else}
            DetailPrint "Warning: PawnIO installation returned code $1 (may require reboot)"
        ${EndIf}
    ${Else}
        DetailPrint "PawnIO already installed: $0"
    ${EndIf}

    ; Check and install LibreHardwareMonitor service
    DetailPrint "Checking for LibreHardwareMonitor service..."
    ; Try to query the service (sc query returns error code if not found)
    ExecWait 'sc query LibreHardwareMonitorService' $0
    ${If} $0 != 0
        DetailPrint "LHM service not found, installing..."
        ; Create service with sc create
        ExecWait 'sc create LibreHardwareMonitorService binPath= "$INSTDIR\lhm-service.exe" start= auto DisplayName= "LibreHardwareMonitor Service" type= own' $0
        ${If} $0 == 0
            DetailPrint "Service created successfully"
            ; Start the service
            ExecWait 'sc start LibreHardwareMonitorService' $0
            ${If} $0 == 0
                DetailPrint "Service started successfully"
            ${Else}
                DetailPrint "Warning: Failed to start service (error code: $0)"
            ${EndIf}
        ${Else}
            DetailPrint "Warning: Failed to create service (error code: $0)"
        ${EndIf}
    ${Else}
        DetailPrint "LHM service already installed"
    ${EndIf}

    ; Create Start Menu shortcuts
    CreateDirectory "$SMPROGRAMS\${PRODUCT_NAME}"
    CreateShortcut "$SMPROGRAMS\${PRODUCT_NAME}\${PRODUCT_NAME}.lnk" \
        "$INSTDIR\temp-monitor.exe" "" "$INSTDIR\logo.ico" 0
    CreateShortcut "$SMPROGRAMS\${PRODUCT_NAME}\Uninstall.lnk" \
        "$INSTDIR\uninstall.exe"

    ; Create desktop shortcut
    CreateShortcut "$DESKTOP\${PRODUCT_NAME}.lnk" "$INSTDIR\temp-monitor.exe" "" "$INSTDIR\logo.ico" 0

    ; Write uninstaller
    WriteUninstaller "$INSTDIR\uninstall.exe"

    ; Write registry keys for Add/Remove Programs
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" \
        "DisplayName" "${PRODUCT_NAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" \
        "UninstallString" "$INSTDIR\uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" \
        "DisplayIcon" "$INSTDIR\temp-monitor.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" \
        "Publisher" "${PRODUCT_PUBLISHER}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" \
        "DisplayVersion" "${PRODUCT_VERSION}"
SectionEnd

; ===== Uninstallation Section =====
Section "Uninstall"
    ; Stop and remove service (only if no other apps use it)
    MessageBox MB_YESNO "Do you want to remove the LibreHardwareMonitor service? (Choose No if other apps use it)" \
        IDNO skip_service

    ExecWait 'sc stop LibreHardwareMonitorService' $0
    ExecWait 'sc delete LibreHardwareMonitorService' $0

skip_service:
    ; Remove files
    Delete "$INSTDIR\temp-monitor.exe"
    Delete "$INSTDIR\lhm-service.exe"
    Delete "$INSTDIR\uninstall.exe"
    RMDir /r "$INSTDIR"

    ; Remove shortcuts
    Delete "$SMPROGRAMS\${PRODUCT_NAME}\*.*"
    RMDir "$SMPROGRAMS\${PRODUCT_NAME}"
    Delete "$DESKTOP\${PRODUCT_NAME}.lnk"

    ; Remove registry keys
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
SectionEnd