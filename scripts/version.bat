@echo off
setlocal
cd /d "%~dp0.."

node -e "const fs=require('fs'); const p=require('./package.json'); const t=require('./src-tauri/tauri.conf.json'); const c=fs.readFileSync('./src-tauri/Cargo.toml','utf8'); const cv=c.match(/^version = \"(.*)\"/m)[1]; console.log('Current versions:'); console.log('package.json:    ' + p.version); console.log('tauri.conf.json: ' + t.version); console.log('Cargo.toml:      ' + cv);"

echo.
set /p NEW_VERSION="Enter new version: "

if "%NEW_VERSION%"=="" (
    echo Version cannot be empty.
    pause
    exit /b 1
)

node -e "const fs=require('fs'); let p=fs.readFileSync('package.json','utf8'); p=p.replace(/\"version\": \".*\"/, '\"version\": \"%NEW_VERSION%\"'); fs.writeFileSync('package.json',p); let t=fs.readFileSync('src-tauri/tauri.conf.json','utf8'); t=t.replace(/\"version\": \".*\"/, '\"version\": \"%NEW_VERSION%\"'); fs.writeFileSync('src-tauri/tauri.conf.json',t); let c=fs.readFileSync('src-tauri/Cargo.toml','utf8'); c=c.replace(/^version = \".*\"/m, 'version = \"%NEW_VERSION%\"'); fs.writeFileSync('src-tauri/Cargo.toml',c);"

echo Done.
pause
