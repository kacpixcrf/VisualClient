@echo off
cd /d "%~dp0.."

echo Cleaning...
if exist "node_modules" rmdir /s /q "node_modules"
if exist "dist" rmdir /s /q "dist"
if exist "src-tauri\target" rmdir /s /q "src-tauri\target"
if exist "src-tauri\gen" rmdir /s /q "src-tauri\gen"
if exist "package-lock.json" del /q "package-lock.json"
if exist "pnpm-lock.yaml" del /q "pnpm-lock.yaml"
if exist "yarn.lock" del /q "yarn.lock"
if exist "bun.lock" del /q "bun.lock"
if exist "bun.lockb" del /q "bun.lockb"
if exist "src-tauri\Cargo.lock" del /q "src-tauri\Cargo.lock"
echo Done.
pause
