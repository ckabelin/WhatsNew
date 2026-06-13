<#
.SYNOPSIS
    Build a release bundle of WhatsNew.
#>

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot

Push-Location (Join-Path $root 'src-tauri')
try {
    cargo tauri build
} finally {
    Pop-Location
}

$bundleDir = Join-Path $root 'target\release\bundle'
Write-Host ""
Write-Host "Build complete. Installer(s):"
Get-ChildItem -Recurse -Path $bundleDir -Include '*.exe', '*.msi' -ErrorAction SilentlyContinue |
    ForEach-Object { Write-Host "  $($_.FullName)" }
