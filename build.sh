#!/bin/bash

APP_NAME="scholar_scraper"
LINUX_TARGET="x86_64-unknown-linux-gnu"

WINDOWS_TARGET="x86_64-pc-windows-gnu"

MACOS_INTEL_TARGET="x86_64-apple-darwin"
MACOS_ARM_TARGET="aarch64-apple-darwin"

echo "* Creating dist directory..."
mkdir -p dist


# --------------------------------------------------------


# Linux Build

echo "* Building for Linux..."
cargo build --release --target=$LINUX_TARGET
strip target/$LINUX_TARGET/release/$APP_NAME || echo "Skipping strip, not available"
tar -czvf dist/${APP_NAME}_linux.tar.gz -C target/$LINUX_TARGET/release $APP_NAME


# --------------------------------------------------------


# Windows Build

echo "* Building for Windows..."
echo "* Building for Windows..."
cargo build --release --target=$WINDOWS_TARGET
strip target/$WINDOWS_TARGET/release/$APP_NAME.exe || echo "Skipping strip, not available"
zip -j dist/${APP_NAME}_windows.zip target/$WINDOWS_TARGET/release/$APP_NAME.exe


# --------------------------------------------------------


# macOS Intel Build

echo "* Building for macOS (Intel)..."
cargo build --release --target=$MACOS_INTEL_TARGET
strip target/$MACOS_INTEL_TARGET/release/$APP_NAME || echo "Skipping strip, not available"
tar -czvf dist/${APP_NAME}_macos_intel.tar.gz -C target/$MACOS_INTEL_TARGET/release $APP_NAME


# macOS ARM (M1/MX) Build

echo "* Building for macOS (Apple Silicon)..."
cargo build --release --target=$MACOS_ARM_TARGET
strip target/$MACOS_ARM_TARGET/release/$APP_NAME || echo "Skipping strip, not available"
tar -czvf dist/${APP_NAME}_macos_arm.tar.gz -C target/$MACOS_ARM_TARGET/release $APP_NAME

echo "Build completed. Check the 'dist' directory."