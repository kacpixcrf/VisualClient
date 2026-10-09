@echo off
setlocal
cd /d "%~dp0.."

set VERSION=%~1
if "%VERSION%"=="" (
    set /p VERSION="Enter version to remove: "
)

if "%VERSION%"=="" (
    echo Error: No version specified.
    pause
    exit /b 1
)

if /I "%VERSION:~0,1%"=="v" set VERSION=%VERSION:~1%

git tag -d v%VERSION%
git push origin :refs/tags/v%VERSION%

echo Done.
pause
