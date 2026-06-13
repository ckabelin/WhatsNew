<#
.SYNOPSIS
    Verify prerequisites and install dependencies for WhatsNew development.
#>

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot

function Test-Command($name) {
    return [bool](Get-Command $name -ErrorAction SilentlyContinue)
}

Write-Host "Checking Rust toolchain..."
if (-not (Test-Command 'cargo')) {
    throw "cargo not found. Install Rust from https://rustup.rs/ before continuing."
}
cargo --version
rustc --version

Write-Host "Checking for MSVC build tools..."
if (-not (Test-Command 'link')) {
    Write-Warning "MSVC linker (link.exe) not found on PATH. Install 'Desktop development with C++' via Visual Studio Build Tools: https://visualstudio.microsoft.com/visual-cpp-build-tools/"
}

Write-Host "Checking for Tauri CLI..."
$tauriInstalled = cargo install --list | Select-String -Pattern '^tauri-cli\s'
if (-not $tauriInstalled) {
    Write-Host "Installing tauri-cli..."
    cargo install tauri-cli --version "^2"
} else {
    Write-Host "tauri-cli already installed."
}

Write-Host "Checking Node.js..."
if (-not (Test-Command 'npm')) {
    throw "npm not found. Install Node.js 20+ from https://nodejs.org/ before continuing."
}
node --version
npm --version

Write-Host "Installing frontend dependencies..."
Push-Location (Join-Path $root 'web')
try {
    npm install
} finally {
    Pop-Location
}

Write-Host "Installing git hook tooling (lefthook)..."
Push-Location $root
try {
    npm install
    npx lefthook install
} finally {
    Pop-Location
}

Write-Host "Setup complete."
