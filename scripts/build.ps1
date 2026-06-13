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
$bundlePatterns = @(
    '*.exe',
    '*.msi',
    '*.dmg',
    '*.app.tar.gz',
    '*.AppImage',
    '*.deb',
    '*.rpm'
)

Write-Host ""
Write-Host "Build complete. Bundle artifact(s):"
Get-ChildItem -Recurse -Path $bundleDir -Include $bundlePatterns -ErrorAction SilentlyContinue |
    ForEach-Object { Write-Host "  $($_.FullName)" }
