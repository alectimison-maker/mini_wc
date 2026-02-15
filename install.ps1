param(
    [string]$BinDir = "$env:USERPROFILE\bin",
    [switch]$Debug,
    [switch]$SkipPath
)

$ErrorActionPreference = "Stop"

$projectDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$manifestPath = Join-Path $projectDir "Cargo.toml"
$targetDir = Join-Path $projectDir "target"
$profile = if ($Debug) { "debug" } else { "release" }

Write-Host "Building mini_wc ($profile)..."
if ($Debug) {
    cargo build --manifest-path $manifestPath --target-dir $targetDir
} else {
    cargo build --manifest-path $manifestPath --release --target-dir $targetDir
}

$builtExe = Join-Path $targetDir "$profile\mini_wc.exe"
if (-not (Test-Path $builtExe)) {
    throw "Build output not found: $builtExe"
}

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
$destExe = Join-Path $BinDir "mini_wc.exe"
Copy-Item $builtExe $destExe -Force

$aliasCmd = Join-Path $BinDir "miniwc.cmd"
$aliasContent = "@echo off`r`n`"%~dp0mini_wc.exe`" %*`r`n"
Set-Content -Path $aliasCmd -Value $aliasContent -Encoding ASCII

if (-not $SkipPath) {
    $fullBinDir = [System.IO.Path]::GetFullPath($BinDir).TrimEnd('\')
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if (-not $userPath) {
        $userPath = ""
    }

    $entries = $userPath -split ";" | Where-Object { $_ -and $_.Trim() -ne "" }
    $exists = $false
    foreach ($entry in $entries) {
        try {
            $normalized = [System.IO.Path]::GetFullPath($entry).TrimEnd('\')
            if ($normalized -ieq $fullBinDir) {
                $exists = $true
                break
            }
        } catch {
            if ($entry.TrimEnd('\') -ieq $fullBinDir) {
                $exists = $true
                break
            }
        }
    }

    if (-not $exists) {
        $newUserPath = if ($userPath) { "$userPath;$fullBinDir" } else { $fullBinDir }
        [Environment]::SetEnvironmentVariable("Path", $newUserPath, "User")
        Write-Host "Added to User PATH: $fullBinDir"
    } else {
        Write-Host "User PATH already contains: $fullBinDir"
    }

    $sessionEntries = $env:Path -split ";" | Where-Object { $_ -and $_.Trim() -ne "" }
    $inSession = $false
    foreach ($entry in $sessionEntries) {
        try {
            $normalized = [System.IO.Path]::GetFullPath($entry).TrimEnd('\')
            if ($normalized -ieq $fullBinDir) {
                $inSession = $true
                break
            }
        } catch {
            if ($entry.TrimEnd('\') -ieq $fullBinDir) {
                $inSession = $true
                break
            }
        }
    }
    if (-not $inSession) {
        $env:Path = "$fullBinDir;$env:Path"
        Write-Host "Updated PATH for current session."
    }
}

Write-Host ""
Write-Host "Installed:"
Write-Host "  $destExe"
Write-Host "  $aliasCmd"
Write-Host ""
Write-Host "Usage:"
Write-Host "  mini_wc <file>"
Write-Host "  miniwc -w <file>"
Write-Host ""
Write-Host "Open a new terminal if PATH was updated in this run."
