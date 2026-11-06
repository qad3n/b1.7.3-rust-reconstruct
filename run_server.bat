@echo off
rem builds and starts the beta173 server, extra arguments go to MinecraftServer.main
setlocal
cd /d "%~dp0"
set "ROOT=%CD%"

if not defined B173_PROFILE set "B173_PROFILE=release"
cargo build --profile %B173_PROFILE% --bin beta173_server || exit /b 1

rem the server keeps server.properties, worlds and logs in its working folder like the original jar
if not defined B173_SERVER_DIR set "B173_SERVER_DIR=%ROOT%\run\server"
if not exist "%B173_SERVER_DIR%" mkdir "%B173_SERVER_DIR%"
cd /d "%B173_SERVER_DIR%"
"%ROOT%\target\%B173_PROFILE%\beta173_server.exe" %*
