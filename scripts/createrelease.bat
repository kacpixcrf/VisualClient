@echo off
setlocal
cd /d "%~dp0.."

node -e "const p=require('./package.json').version; const t=require('./src-tauri/tauri.conf.json').version; const c=require('fs').readFileSync('./src-tauri/Cargo.toml','utf8'); if(!c.includes('version = \"'+p+'\"') || p!==t) { process.exit(1); } console.log(p);" > temp_version.txt

if errorlevel 1 (
    echo Versions mismatch. Please use version-changer first.
    del temp_version.txt
    pause
    exit /b 1
)

set /p NEW_VERSION=<temp_version.txt
del temp_version.txt

if "%NEW_VERSION%"=="" (
    echo Versions mismatch. Please use version-changer first.
    pause
    exit /b 1
)

git add .
git commit -m "Release %NEW_VERSION%"
git push origin main
git tag v%NEW_VERSION%
git push origin v%NEW_VERSION%

echo Done.
pause
