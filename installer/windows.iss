#define AppVersion "0.2.0"

[Setup]
AppId=com.jeremdevgit.ljhub
AppName=LJ Hub
AppVersion={#AppVersion}
AppPublisher=jeremdevgit
AppPublisherURL=https://github.com/jeremdevgit/lj-hub
AppSupportURL=https://github.com/jeremdevgit/lj-hub/issues
DefaultDirName={localappdata}\Programs\LJ Hub
DefaultGroupName=LJ Hub
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
OutputDir=..\dist
OutputBaseFilename=LJ-Hub-{#AppVersion}-alpha-Windows-Setup
Compression=lzma2
SolidCompression=yes
UninstallDisplayName=LJ Hub
LicenseFile=..\LICENSE
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "french"; MessagesFile: "compiler:Languages\French.isl"
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "german"; MessagesFile: "compiler:Languages\German.isl"
Name: "italian"; MessagesFile: "compiler:Languages\Italian.isl"
Name: "portuguese"; MessagesFile: "compiler:Languages\Portuguese.isl"
Name: "chinese"; MessagesFile: "compiler:Languages\ChineseSimplified.isl"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\target\release\lj-hub.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.fr.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.es.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\LJ Hub"; Filename: "{app}\lj-hub.exe"; WorkingDir: "{app}"
Name: "{autodesktop}\LJ Hub"; Filename: "{app}\lj-hub.exe"; WorkingDir: "{app}"; Tasks: desktopicon

[Run]
Filename: "{app}\lj-hub.exe"; Description: "{cm:LaunchProgram,LJ Hub}"; Flags: nowait postinstall skipifsilent
