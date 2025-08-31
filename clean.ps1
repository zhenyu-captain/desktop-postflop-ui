# Windows PowerShell Cleanup Script
# Usage: Run .\clean.ps1 in PowerShell

Write-Host "Starting cleanup..." -ForegroundColor Green

# Clean Rust build artifacts
Write-Host "Cleaning Rust build files..." -ForegroundColor Yellow
if (Test-Path "src-tauri") {
    Set-Location "src-tauri"
    cargo clean
    Set-Location ".."
} else {
    Write-Host "src-tauri directory not found, skipping Rust cleanup" -ForegroundColor Yellow
}

# Clean frontend build artifacts
Write-Host "Cleaning frontend build files..." -ForegroundColor Yellow
if (Test-Path "dist") {
    Remove-Item -Recurse -Force "dist"
    Write-Host "Deleted dist directory" -ForegroundColor Green
}

if (Test-Path "node_modules") {
    Remove-Item -Recurse -Force "node_modules"
    Write-Host "Deleted node_modules directory" -ForegroundColor Green
}

# Clean other temporary files
Write-Host "Cleaning temporary files..." -ForegroundColor Yellow

# Clean Windows temporary files
Get-ChildItem -Path "." -Recurse -Include "*.tmp", "*.temp", "*.log", "Thumbs.db", "desktop.ini" | Remove-Item -Force
Write-Host "Cleaned temporary files" -ForegroundColor Green

# Clean possible cache files
if (Test-Path ".cache") {
    Remove-Item -Recurse -Force ".cache"
    Write-Host "Deleted .cache directory" -ForegroundColor Green
}

Write-Host "Cleanup completed!" -ForegroundColor Green
Write-Host "Now you can run npm install and npm run build" -ForegroundColor Cyan
