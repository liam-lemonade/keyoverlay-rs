mkdir package
cd package
mkdir web
cd ../

xcopy D:\code\projects\keyoverlay\daemon\target\release\keyoverlay_daemon.exe package
xcopy D:\code\projects\keyoverlay\web package\web /s
pause