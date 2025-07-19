# ------------------------------
# Mica Portable Installer Script (System-wide)
# ------------------------------

# Must run as Admin
$currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
$admin = $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $admin) {
    Write-Error "[mica] This script must be run as Administrator!"
    exit 1
}

Write-Host "[mica] Checking latest release on GitHub..."

# Get latest release metadata
$latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/lordaimer/mica/releases/latest"
$tag = $latestRelease.tag_name
Write-Host "[mica] Latest version: $tag"

# Find portable asset
$asset = $latestRelease.assets | Where-Object { $_.name -eq "mica-portable-x64.zip" }
if (-not $asset) {
    Write-Error "[mica] Could not find mica-portable-x64.zip in latest release!"
    exit 1
}

$downloadUrl = $asset.browser_download_url
Write-Host "[mica] Download URL: $downloadUrl"

# Download to temp
$tempZip = "$env:TEMP\mica-portable-x64.zip"
$tempExtract = "$env:TEMP\mica-portable"

Write-Host "[mica] Downloading to: $tempZip"
Invoke-WebRequest -Uri $downloadUrl -OutFile $tempZip

# Extract to temp
Write-Host "[mica] Extracting to: $tempExtract"
Expand-Archive -Path $tempZip -DestinationPath $tempExtract -Force

# Create system install folder
$micaRoot = "C:\Program Files\Mica"
$depsPath = "$micaRoot\Dependencies\gstreamer"

Write-Host "[mica] Creating folders at: $micaRoot"
New-Item -ItemType Directory -Path $micaRoot -Force | Out-Null
New-Item -ItemType Directory -Path $depsPath -Force | Out-Null

# Copy core binaries
Copy-Item "$tempExtract\mica.exe" $micaRoot -Force
Copy-Item "$tempExtract\libmica.dll" $micaRoot -Force

# Extract GStreamer
Write-Host "[mica] Extracting GStreamer..."
Expand-Archive -Path "$tempExtract\gstreamer.zip" -DestinationPath $depsPath -Force

# Add to PATH (system-wide)
$gstreamerBin = "$depsPath\bin"
$gstreamerLib = "$depsPath\lib"

Write-Host "[mica] Adding Mica, GStreamer bin & lib to system PATH..."
$envPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::Machine)

$newParts = @()
if (-not ($envPath -like "*$micaRoot*")) { $newParts += $micaRoot }
if (-not ($envPath -like "*$gstreamerBin*")) { $newParts += $gstreamerBin }
if (-not ($envPath -like "*$gstreamerLib*")) { $newParts += $gstreamerLib }

if ($newParts.Count -gt 0) {
    $newPath = ($newParts -join ";") + ";" + $envPath
    [Environment]::SetEnvironmentVariable("Path", $newPath, [EnvironmentVariableTarget]::Machine)
    Write-Host "[mica] PATH updated."
} else {
    Write-Host "[mica] PATH already up to date."
}

# Cleanup temp
Write-Host "[mica] Cleaning up temp files..."
Remove-Item $tempZip -Force
Remove-Item $tempExtract -Recurse -Force

Write-Host "[mica] System-wide Installation Complete! Restart your terminal or log out/in for PATH changes to apply."
