# Builds NetLens for Windows and packages a portable zip in dist\
# Requirements: Rust (MSVC toolchain, https://rustup.rs) + Visual Studio Build Tools ("Desktop development with C++")
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

cargo build --release
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

$version = (Select-String -Path Cargo.toml -Pattern '^version\s*=\s*"(.+)"' | Select-Object -First 1).Matches[0].Groups[1].Value
$stage = "dist\netlens-$version-windows-x86_64"
Remove-Item -Recurse -Force $stage -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force $stage | Out-Null
Copy-Item target\release\netlens.exe $stage
Copy-Item README.md, LICENSE $stage
Compress-Archive -Path "$stage\*" -DestinationPath "dist\netlens-$version-windows-x86_64.zip" -Force
Write-Host "Done: dist\netlens-$version-windows-x86_64.zip"
