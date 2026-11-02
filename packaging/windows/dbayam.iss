; DBayam's Windows installer (Inno Setup 6), built by scripts/bundle-windows.ps1:
;
;   ISCC.exe /DAppVersion=1.2.3 /DSourceExe=...\dbayam.exe /DOutputDir=...\dist ^
;            /DOutputBaseFilename=DBayam-1.2.3-windows-x64-setup packaging\windows\dbayam.iss
;
; Installs for the current user (no administrator rights) unless "all users"
; is picked, adds a Start menu shortcut (a desktop one when asked) and an
; uninstaller. Settings, connections and scripts (in %APPDATA%) stay when
; DBayam is uninstalled. Installing a new version over the old one updates it.

#ifndef AppVersion
  #error Define AppVersion (/DAppVersion=1.2.3)
#endif
#ifndef SourceExe
  #error Define SourceExe (/DSourceExe=path\to\dbayam.exe)
#endif
#ifndef OutputDir
  #define OutputDir "..\..\dist"
#endif
#ifndef OutputBaseFilename
  #define OutputBaseFilename "DBayam-" + AppVersion + "-windows-x64-setup"
#endif

[Setup]
AppId={{03F93606-1AE0-4EFD-A66F-1CC88238C476}
AppName=DBayam
AppVersion={#AppVersion}
AppVerName=DBayam {#AppVersion}
VersionInfoVersion={#AppVersion}
AppPublisher=DBayam
AppPublisherURL=https://github.com/rajebdev/dbayam
AppSupportURL=https://github.com/rajebdev/dbayam/issues
AppUpdatesURL=https://github.com/rajebdev/dbayam/releases
DefaultDirName={autopf}\DBayam
DefaultGroupName=DBayam
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
OutputDir={#OutputDir}
OutputBaseFilename={#OutputBaseFilename}
SetupIconFile=dbayam.ico
UninstallDisplayIcon={app}\dbayam.exe
UninstallDisplayName=DBayam
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
; A running DBayam is closed before its files are replaced.
CloseApplications=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#SourceExe}"; DestDir: "{app}"; DestName: "dbayam.exe"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\DBayam"; Filename: "{app}\dbayam.exe"
Name: "{autodesktop}\DBayam"; Filename: "{app}\dbayam.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\dbayam.exe"; Description: "{cm:LaunchProgram,DBayam}"; Flags: nowait postinstall skipifsilent
