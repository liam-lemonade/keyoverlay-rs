mkdir x86
cd x86
mkdir web
cd ../

xcopy D:\code\projects\keyoverlay\daemon\target\i686-pc-windows-msvc\release\keyoverlay_daemon.exe x86
xcopy D:\code\projects\keyoverlay\web x86\web /s
cd x86
ren keyoverlay_daemon.exe keyoverlay-windows-x86.exe
pause