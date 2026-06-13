<#
.SYNOPSIS
    Run the full WhatsNew test/lint suite (mirrors CI).
#>

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot

Write-Host "==> cargo test --workspace"
cargo test --workspace

Write-Host "==> cargo clippy --workspace --all-targets -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings

Push-Location (Join-Path $root 'web')
try {
    Write-Host "==> npm run check"
    npm run check

    Write-Host "==> npm run lint"
    npm run lint
} finally {
    Pop-Location
}

Write-Host "All checks passed."
