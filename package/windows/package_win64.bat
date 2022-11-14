mkdir x64
cd x64
mkdir web
cd ../

xcopy D:\code\projects\keyoverlay\daemon\target\x86_64-pc-windows-msvc\release\keyoverlay_daemon.exe x64
xcopy D:\code\projects\keyoverlay\web x64\web /s
cd x64
ren keyoverlay_daemon.exe keyoverlay-windows-x64.exe
pause