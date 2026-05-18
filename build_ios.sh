#!/usr/bin/env bash
# build_ios.sh — Compile the Rust vault crate for iOS + macOS and generate Swift UniFFI bindings.
#
# Usage:
#   ./build_ios.sh
#
# Prerequisites:
#   - Rust toolchain with iOS targets (see core/rust-toolchain.toml)
#   - Xcode command-line tools (lipo, xcodebuild)
#
# Outputs:
#   build/VaultFFI.xcframework    — linkable from Xcode / SPM (iOS device, simulator, macOS)
#   ios/Generated/                — UniFFI-generated Swift bindings

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CORE_DIR="$SCRIPT_DIR/core"
IOS_OUT="$SCRIPT_DIR/build"
GENERATED_DIR="$SCRIPT_DIR/ios/Generated"

TARGET_DEVICE="aarch64-apple-ios"
TARGET_SIM_ARM="aarch64-apple-ios-sim"
TARGET_SIM_X86="x86_64-apple-ios"
TARGET_MAC_ARM="aarch64-apple-darwin"
TARGET_MAC_X86="x86_64-apple-darwin"

LIB_NAME="libvault.a"

echo "==> Building iOS device target ($TARGET_DEVICE)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_DEVICE" -p vault)

echo "==> Building iOS simulator arm64 target ($TARGET_SIM_ARM)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_SIM_ARM" -p vault)

echo "==> Building iOS simulator x86_64 target ($TARGET_SIM_X86)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_SIM_X86" -p vault)

echo "==> Building macOS arm64 target ($TARGET_MAC_ARM)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_MAC_ARM" -p vault)

echo "==> Building macOS x86_64 target ($TARGET_MAC_X86)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_MAC_X86" -p vault)

SIM_OUT="$CORE_DIR/target/ios-sim-fat"
MAC_OUT="$CORE_DIR/target/macos-fat"
mkdir -p "$SIM_OUT" "$MAC_OUT"

echo "==> Creating fat simulator library (lipo)..."
lipo -create \
    "$CORE_DIR/target/$TARGET_SIM_ARM/release/$LIB_NAME" \
    "$CORE_DIR/target/$TARGET_SIM_X86/release/$LIB_NAME" \
    -output "$SIM_OUT/$LIB_NAME"

echo "==> Creating fat macOS library (lipo)..."
lipo -create \
    "$CORE_DIR/target/$TARGET_MAC_ARM/release/$LIB_NAME" \
    "$CORE_DIR/target/$TARGET_MAC_X86/release/$LIB_NAME" \
    -output "$MAC_OUT/$LIB_NAME"

echo "==> Generating Swift bindings via uniffi-bindgen..."
mkdir -p "$GENERATED_DIR"
(cd "$CORE_DIR" && cargo run --bin uniffi-bindgen -- generate \
    --library "target/$TARGET_DEVICE/release/$LIB_NAME" \
    --language swift \
    --out-dir "$GENERATED_DIR")

HEADERS_DIR="$IOS_OUT/Headers"
rm -rf "$HEADERS_DIR"
mkdir -p "$HEADERS_DIR"
cp "$GENERATED_DIR/vaultFFI.h" "$HEADERS_DIR/"
cp "$GENERATED_DIR/vaultFFI.modulemap" "$HEADERS_DIR/module.modulemap"

mkdir -p "$IOS_OUT"
XCFRAMEWORK="$IOS_OUT/VaultFFI.xcframework"
rm -rf "$XCFRAMEWORK"
echo "==> Creating XCFramework (iOS device + simulator + macOS)..."
xcodebuild -create-xcframework \
    -library "$CORE_DIR/target/$TARGET_DEVICE/release/$LIB_NAME" \
    -headers "$HEADERS_DIR" \
    -library "$SIM_OUT/$LIB_NAME" \
    -headers "$HEADERS_DIR" \
    -library "$MAC_OUT/$LIB_NAME" \
    -headers "$HEADERS_DIR" \
    -output "$XCFRAMEWORK"

echo "==> XCFramework created at $XCFRAMEWORK"
echo "==> Swift bindings written to $GENERATED_DIR"
echo ""
echo "Done!"
