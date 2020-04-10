call .\scripts\build.bat
if %errorlevel% neq 0 exit /b %errorlevel%

set TO=\\192.168.122.1\smb\bot.exe
set FROM=.\target\release\bot.exe
copy /Y %FROM% %TO% 
if %errorlevel% neq 0 exit /b %errorlevel%
md5sum %FROM%
md5sum %TO%