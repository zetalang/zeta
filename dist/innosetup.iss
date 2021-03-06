; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define MyAppName "Torq"
#define MyAppVersion "1.0.0-alpha"
#define MyAppPublisher "Torq"
#define MyAppURL "https://torqlang.sh"
#define MyAppExeName "torqc.exe"

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{6D78E4B8-B1CA-48D8-AA62-073C1D750C6F}
AppName={#MyAppName}
AppVersion={#MyAppVersion}

;AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
; Remove the following line to run in administrative install mode (install for all users.)
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=dialog
OutputDir=D:\Projects\torqc\dist
OutputBaseFilename=Torq Setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
LZMAUseSeparateProcess=yes
LZMANumBlockThreads=6

[Setup]
; Tell Windows Explorer to reload the environment
ChangesEnvironment=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "D:\Projects\torqc\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"

[Run]

filename: "{app}\{#MyAppExeName}"; Flags: runascurrentuser;Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}";
[Code]
const
  EnvironmentKey = 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
procedure RemovePath(Path: string);
var
  Paths: string;
  P: Integer;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths) then
  begin
    Log('PATH not found');
  end
    else
  begin
    Log(Format('PATH is [%s]', [Paths]));
    P := Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';');
    if P = 0 then
    begin
      Log(Format('Path [%s] not found in PATH', [Path]));
    end
      else
    begin
      if P > 1 then P := P - 1;
      Delete(Paths, P, Length(Path) + 1);
      Log(Format('Path [%s] removed from PATH => [%s]', [Path, Paths]));
      if RegWriteStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths) then
      begin
        Log('PATH written');
      end
        else
      begin
        Log('Error writing PATH');
      end;
    end;
  end;
end;
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usUninstall then
  begin
    RemovePath('');
  end;
end;
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  { look for the path with leading and trailing semicolon }
  { Pos() returns 0 if not found }
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;
[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};C:\Program Files (x86)\{#MyAppName}\bin"; \
    Check: NeedsAddPath('C:\Program Files (x86)\{#MyAppName}\bin')
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{%USERPROFILE}\electric\shims"; \
    Check: NeedsAddPath('{%USERPROFILE}\electric\shims')
