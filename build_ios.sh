#!/usr/bin/env bash
# build_ios.sh — Build Rust vault from baby-management-core and generate Swift UniFFI bindings.
#
# Usage:
#   ./build_ios.sh [--core-dir /path/to/core]
#
# Prerequisites:
#   - Rust toolchain with iOS/macOS targets
#   - Xcode command-line tools (lipo, xcodebuild)
#
# By default, clones github.com/maulanasdqn/baby-management-core into a sibling
# directory (../baby-management-core). Pass --core-dir to use an existing checkout.
#
# Outputs:
#   build/VaultFFI.xcframework    — linked from Xcode / SPM
#   ios/Generated/                — UniFFI-generated Swift bindings

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IOS_OUT="$SCRIPT_DIR/build"
GENERATED_DIR="$SCRIPT_DIR/ios/Generated"
CORE_REPO="https://github.com/maulanasdqn/baby-management-core"
DEFAULT_CORE_DIR="$(dirname "$SCRIPT_DIR")/baby-management-core"

CORE_DIR=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --core-dir) CORE_DIR="$2"; shift 2 ;;
        *) echo "Unknown argument: $1"; exit 1 ;;
    esac
done

if [[ -z "$CORE_DIR" ]]; then
    CORE_DIR="$DEFAULT_CORE_DIR"
    if [[ ! -d "$CORE_DIR" ]]; then
        echo "==> Cloning $CORE_REPO into $CORE_DIR..."
        git clone "$CORE_REPO" "$CORE_DIR"
    else
        echo "==> Updating core at $CORE_DIR..."
        git -C "$CORE_DIR" pull --ff-only
    fi
fi

echo "==> Using core at $CORE_DIR"

TARGET_DEVICE="aarch64-apple-ios"
TARGET_SIM_ARM="aarch64-apple-ios-sim"
TARGET_SIM_X86="x86_64-apple-ios"
TARGET_MAC_ARM="aarch64-apple-darwin"
TARGET_MAC_X86="x86_64-apple-darwin"
LIB_NAME="libvault.a"

echo "==> Building iOS device ($TARGET_DEVICE)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_DEVICE" -p vault)

echo "==> Building iOS simulator arm64 ($TARGET_SIM_ARM)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_SIM_ARM" -p vault)

echo "==> Building iOS simulator x86_64 ($TARGET_SIM_X86)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_SIM_X86" -p vault)

echo "==> Building macOS arm64 ($TARGET_MAC_ARM)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_MAC_ARM" -p vault)

echo "==> Building macOS x86_64 ($TARGET_MAC_X86)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_MAC_X86" -p vault)

SIM_OUT="$CORE_DIR/target/ios-sim-fat"
MAC_OUT="$CORE_DIR/target/macos-fat"
mkdir -p "$SIM_OUT" "$MAC_OUT"

echo "==> Creating fat simulator library..."
lipo -create \
    "$CORE_DIR/target/$TARGET_SIM_ARM/release/$LIB_NAME" \
    "$CORE_DIR/target/$TARGET_SIM_X86/release/$LIB_NAME" \
    -output "$SIM_OUT/$LIB_NAME"

echo "==> Creating fat macOS library..."
lipo -create \
    "$CORE_DIR/target/$TARGET_MAC_ARM/release/$LIB_NAME" \
    "$CORE_DIR/target/$TARGET_MAC_X86/release/$LIB_NAME" \
    -output "$MAC_OUT/$LIB_NAME"

echo "==> Generating Swift bindings..."
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

echo ""
echo "Done!"
echo "  XCFramework: $XCFRAMEWORK"
echo "  Bindings:    $GENERATED_DIR"
