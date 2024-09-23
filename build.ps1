# Parameters for the script
param (
    [switch]$Clean,
    [switch]$Build,
    [switch]$Publish
)

# Variables
$PUBLISH_DIR = "./publish"
$CARGO_TARGET_DIR = "./target/release"
$TAURI_APP_DIR = "./app/search-files-app"
$TAURI_TARGET_DIR = "$TAURI_APP_DIR/src-tauri/target/release"
$RUST_BINARY = "file_elf.exe"
$TAURI_BINARY = "search-files-app.exe"
$TAURI_INSTALL_MSI = "bundle/msi/search-files-app_0.11.0_x64_en-US.msi"
$TAURI_INSTALL_EXE = "bundle/nsis/search-files-app_0.11.0_x64-setup.exe"
$ZIP_FILE = "release_package_windows.zip"

# Ensure the publish directory exists
function Ensure-PublishDir {
    if (-Not (Test-Path -Path $PUBLISH_DIR)) {
        Write-Host "Creating publish directory..."
        New-Item -Path $PUBLISH_DIR -ItemType Directory
    }
}

# Build the Rust project
function Build-Elf {
    Write-Host "Building Rust file_elf project..."
    cargo build --features webserver --release
}

# Build the Tauri app
function Build-Tauri {
    Write-Host "Building Tauri app..."
    Push-Location $TAURI_APP_DIR
    cargo tauri build
    Pop-Location
}

# Copy files to the publish directory
function Publish-Files {
    Ensure-PublishDir
    Write-Host "Copying files to the publish directory..."
    Copy-Item "$CARGO_TARGET_DIR/$RUST_BINARY" "$PUBLISH_DIR/"
    Copy-Item "$TAURI_TARGET_DIR/$TAURI_BINARY" "$PUBLISH_DIR/"
    Copy-Item "$TAURI_TARGET_DIR/$TAURI_INSTALL_MSI" "$PUBLISH_DIR/"
    Copy-Item "$TAURI_TARGET_DIR/$TAURI_INSTALL_EXE" "$PUBLISH_DIR/"
}

# Package the build artifacts into a zip file
function Package-Files {
    Write-Host "Packaging files into $ZIP_FILE..."
    Compress-Archive -Path "$PUBLISH_DIR/*" -DestinationPath "./$ZIP_FILE" -Force
}

# Clean build artifacts
function Clean-Build {
    Write-Host "Cleaning build artifacts..."
    cargo clean
    Remove-Item -Recurse -Force $PUBLISH_DIR
    Remove-Item -Recurse -Force "$TAURI_APP_DIR/build/"
    Remove-Item -Recurse -Force "$TAURI_TARGET_DIR/"
}

# Clean only the publish directory
function Clean-Publish {
    Write-Host "Cleaning the publish directory..."
    Remove-Item -Recurse -Force $PUBLISH_DIR
}

# Clean only the Tauri build artifacts
function Clean-Tauri {
    Write-Host "Cleaning Tauri build artifacts..."
    Remove-Item -Recurse -Force "$TAURI_APP_DIR/build/"
    Remove-Item -Recurse -Force "$TAURI_TARGET_DIR/"
}

# Clean only the Rust project artifacts
function Clean-Elf {
    Write-Host "Cleaning Rust file_elf project artifacts..."
    cargo clean
}

# Main logic for the build script
if ($Clean) {
    Clean-Build
}
elseif ($Build) {
    Build-Elf
    Build-Tauri
}
elseif ($Publish) {
    Publish-Files
    Package-Files
}
else {
    Write-Host "Usage: ./build.ps1 -Clean | -Build | -Publish"
}
