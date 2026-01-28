Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$webDir = Join-Path $PSScriptRoot "..\wallet-web"
$force = ($env:WALLET_WEB_INSTALL -eq "1")

Push-Location $webDir
try {
  $hasNodeModules = Test-Path "node_modules"
  $hasLock = Test-Path "package-lock.json"

  if ($force -or -not $hasNodeModules) {
    if ($hasLock) {
      Write-Host "web-install: npm ci (fallback to npm install)" -ForegroundColor DarkGray
      npm ci --no-audit --no-fund
      if ($LASTEXITCODE -ne 0) {
        Write-Host "npm ci failed; falling back to npm install" -ForegroundColor Yellow
        npm install --no-audit --no-fund
        exit $LASTEXITCODE
      }
    } else {
      Write-Host "web-install: no package-lock.json; using npm install" -ForegroundColor Yellow
      npm install --no-audit --no-fund
      exit $LASTEXITCODE
    }
  } else {
    Write-Host "web-install: node_modules present; skipping (set WALLET_WEB_INSTALL=1 to force)" -ForegroundColor DarkGray
  }
}
finally {
  Pop-Location
}
