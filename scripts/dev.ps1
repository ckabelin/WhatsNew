<#
.SYNOPSIS
    Run WhatsNew in development mode with hot reload.
#>

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot

Push-Location (Join-Path $root 'src-tauri')
try {
    cargo tauri dev
} finally {
    Pop-Location
}
