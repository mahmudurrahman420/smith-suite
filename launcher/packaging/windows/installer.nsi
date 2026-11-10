; Per-user installer for Smith Creative Cloud. No administrator rights.
; Built by CI: makensis /DSOURCE=path\to\SmithCreativeCloud.exe /DOUTFILE=path\to\Setup.exe installer.nsi

Unicode True
!include "MUI2.nsh"

!ifndef SOURCE
  !error "Pass /DSOURCE=path\\to\\SmithCreativeCloud.exe"
!endif
!ifndef OUTFILE
  !error "Pass /DOUTFILE=path\\to\\SmithCreativeCloud-Setup.exe"
!endif

Name "Smith Creative Cloud"
OutFile "${OUTFILE}"
InstallDir "$LOCALAPPDATA\SmithSuite\launcher"
RequestExecutionLevel user
ShowInstDetails show

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_LANGUAGE "English"

Section "Install"
  SetOutPath "$INSTDIR"
  File "${SOURCE}"
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  CreateDirectory "$SMPROGRAMS\Smith Suite"
  CreateShortcut "$SMPROGRAMS\Smith Suite\Smith Creative Cloud.lnk" "$INSTDIR\SmithCreativeCloud.exe"
  CreateShortcut "$SMPROGRAMS\Smith Suite\Uninstall Smith Creative Cloud.lnk" "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\SmithCreativeCloud.exe"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"
  Delete "$SMPROGRAMS\Smith Suite\Smith Creative Cloud.lnk"
  Delete "$SMPROGRAMS\Smith Suite\Uninstall Smith Creative Cloud.lnk"
  RMDir "$SMPROGRAMS\Smith Suite"
SectionEnd
