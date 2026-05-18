#!/usr/bin/env bash
# build_ios.sh — Compile the Rust vault crate for iOS and generate Swift UniFFI bindings.
#
# Usage:
#   ./build_ios.sh
#
# Prerequisites:
#   - Rust toolchain with iOS targets (see core/rust-toolchain.toml)
#   - Xcode command-line tools (lipo, xcodebuild)
#
# Outputs:
#   ios/VaultFFI.xcframework         — linkable from Xcode / SPM
#   Sources/BabyVault/Generated/     — UniFFI-generated Swift bindings

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CORE_DIR="$SCRIPT_DIR/core"
IOS_OUT="$SCRIPT_DIR/ios"
GENERATED_DIR="$SCRIPT_DIR/Sources/BabyVault/Generated"

TARGET_DEVICE="aarch64-apple-ios"
TARGET_SIM_ARM="aarch64-apple-ios-sim"
TARGET_SIM_X86="x86_64-apple-ios"

# lib<name>.a is emitted because vault has crate-type = ["cdylib", "staticlib"]
LIB_NAME="libvault.a"

# All cargo commands run from the workspace root so workspace Cargo.toml is used.

echo "==> Building device target ($TARGET_DEVICE)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_DEVICE" -p vault)

echo "==> Building simulator arm64 target ($TARGET_SIM_ARM)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_SIM_ARM" -p vault)

echo "==> Building simulator x86_64 target ($TARGET_SIM_X86)..."
(cd "$CORE_DIR" && cargo build --release --target "$TARGET_SIM_X86" -p vault)

SIM_OUT="$CORE_DIR/target/ios-sim-fat"
mkdir -p "$SIM_OUT"
echo "==> Creating fat simulator library (lipo)..."
lipo -create \
    "$CORE_DIR/target/$TARGET_SIM_ARM/release/$LIB_NAME" \
    "$CORE_DIR/target/$TARGET_SIM_X86/release/$LIB_NAME" \
    -output "$SIM_OUT/$LIB_NAME"

mkdir -p "$IOS_OUT"
XCFRAMEWORK="$IOS_OUT/VaultFFI.xcframework"
rm -rf "$XCFRAMEWORK"
echo "==> Creating XCFramework..."
xcodebuild -create-xcframework \
    -library "$CORE_DIR/target/$TARGET_DEVICE/release/$LIB_NAME" \
    -library "$SIM_OUT/$LIB_NAME" \
    -output "$XCFRAMEWORK"

echo "==> XCFramework created at $XCFRAMEWORK"

# uniffi-bindgen is a [[bin]] inside the vault crate; run via cargo from workspace root.
echo "==> Generating Swift bindings via uniffi-bindgen..."
mkdir -p "$GENERATED_DIR"
(cd "$CORE_DIR" && cargo run --bin uniffi-bindgen -- generate \
    --library "target/$TARGET_DEVICE/release/$LIB_NAME" \
    --language swift \
    --out-dir "$GENERATED_DIR")

echo "==> Swift bindings written to $GENERATED_DIR"
echo "    NOTE: Remove VaultFFIStubs.swift and enable VAULT_FFI_AVAILABLE in Package.swift"

echo ""
echo "Done! Next steps:"
echo "  1. In Package.swift, uncomment the .binaryTarget block and add 'VaultFFI' to dependencies."
echo "  2. Delete Sources/BabyVault/Generated/VaultFFIStubs.swift."
echo "  3. Uncomment .define(\"VAULT_FFI_AVAILABLE\") in swiftSettings."
