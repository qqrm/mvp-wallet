@echo off
setlocal EnableExtensions EnableDelayedExpansion

rem ============================================================
rem bundle.cmd
rem Build bundle.zip from the current working tree (includes uncommitted + untracked),
rem respecting ALL nested .gitignore rules via "git check-ignore".
rem Smart recursion: if a directory is ignored, it is NOT traversed.
rem Hard excludes: .git/, target/, and bundle*.zip anywhere.
rem ============================================================

for /f "usebackq delims=" %%R in (`git rev-parse --show-toplevel 2^>nul`) do set "REPO_ROOT=%%R"
if "%REPO_ROOT%"=="" (
  echo ERROR: not a git repository
  exit /b 1
)

pushd "%REPO_ROOT%" >nul

set "OUT_NAME=bundle.zip"
if exist "%OUT_NAME%" del /f /q "%OUT_NAME%" >nul 2>&1

powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$ErrorActionPreference='Stop';" ^
  "" ^
  "$root = (Resolve-Path -LiteralPath '%REPO_ROOT%').Path;" ^
  "$out  = Join-Path $root '%OUT_NAME%';" ^
  "" ^
  "Add-Type -AssemblyName System.IO.Compression;" ^
  "Add-Type -AssemblyName System.IO.Compression.FileSystem;" ^
  "" ^
  "if (Test-Path -LiteralPath $out) { Remove-Item -LiteralPath $out -Force }" ^
  "" ^
  "function ToRel([string]$full) {" ^
  "  $rel = [System.IO.Path]::GetRelativePath($root, $full);" ^
  "  if ($rel -eq '.') { return '' }" ^
  "  return $rel -replace '/', '\';" ^
  "}" ^
  "" ^
  "function IsHardExcluded([string]$rel) {" ^
  "  if ([string]::IsNullOrWhiteSpace($rel)) { return $false }" ^
  "  $r = $rel -replace '/', '\'" ^
  "  $parts = $r -split '\\\\'" ^
  "  foreach ($p in $parts) {" ^
  "    if ($p -ieq '.git') { return $true }" ^
  "    if ($p -ieq 'target') { return $true }" ^
  "  }" ^
  "  $leaf = [System.IO.Path]::GetFileName($r)" ^
  "  if ($leaf -ilike 'bundle*.zip') { return $true }" ^
  "  return $false" ^
  "}" ^
  "" ^
  "function IsGitIgnored([string]$rel) {" ^
  "  if ([string]::IsNullOrWhiteSpace($rel)) { return $false }" ^
  "  & git -C $root check-ignore -q -- $rel" ^
  "  return ($LASTEXITCODE -eq 0)" ^
  "}" ^
  "" ^
  "function Walk([string]$dir) {" ^
  "  Get-ChildItem -LiteralPath $dir -Force -ErrorAction Stop | ForEach-Object {" ^
  "    $full = $_.FullName" ^
  "    $rel  = ToRel $full" ^
  "" ^
  "    if (IsHardExcluded $rel) { return }" ^
  "" ^
  "    # If gitignore says 'ignored', prune immediately (esp. directories)." ^
  "    if (IsGitIgnored $rel) { return }" ^
  "" ^
  "    if ($_.PSIsContainer) {" ^
  "      # Avoid walking into reparse points (junctions/symlinks) to prevent surprises." ^
  "      if (($_.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { return }" ^
  "      Walk $full" ^
  "    } else {" ^
  "      if (-not (Test-Path -LiteralPath $full -PathType Leaf)) { return }" ^
  "      [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile(" ^
  "        $zip, $full, $rel, [System.IO.Compression.CompressionLevel]::Optimal" ^
  "      ) | Out-Null" ^
  "    }" ^
  "  }" ^
  "}" ^
  "" ^
  "$zip = [System.IO.Compression.ZipFile]::Open($out, [System.IO.Compression.ZipArchiveMode]::Create);" ^
  "try { Walk $root } finally { $zip.Dispose() }"

if errorlevel 1 (
  echo ERROR: bundle build failed
  popd >nul
  exit /b 1
)

rem Compute a cheap content hash (MD5) and rename the bundle as: bundle_<hash>.zip
set "BUNDLE_HASH="
for /f "usebackq delims=" %%H in (`powershell -NoProfile -Command "(Get-FileHash -Algorithm MD5 -LiteralPath '%OUT_NAME%').Hash.Substring(0,8).ToLower()"`) do set "BUNDLE_HASH=%%H"

if "%BUNDLE_HASH%"=="" (
  echo ERROR: failed to compute bundle hash
  popd >nul
  exit /b 1
)

for %%A in ("%OUT_NAME%") do (
  set "BASE=%%~nA"
  set "EXT=%%~xA"
)

set "OUT_NAME_HASHED=%BASE%_%BUNDLE_HASH%%EXT%"

if exist "%OUT_NAME_HASHED%" del /f /q "%OUT_NAME_HASHED%" >nul 2>&1
move /y "%OUT_NAME%" "%OUT_NAME_HASHED%" >nul

if errorlevel 1 (
  echo ERROR: failed to rename bundle
  popd >nul
  exit /b 1
)

echo OK: %OUT_NAME_HASHED%

popd >nul
exit /b 0
