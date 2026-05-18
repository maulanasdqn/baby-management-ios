// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "BabyVault",
    platforms: [
        .iOS(.v17),
        .macOS(.v14),
    ],
    products: [
        .executable(name: "BabyVault", targets: ["BabyVault"]),
    ],
    targets: [
        // Real XCFramework built by build_ios.sh — path-based local binary target.
        // Run `./build_ios.sh` first to generate build/VaultFFI.xcframework
        // then remove ios/Generated/VaultFFIStubs.swift and uncomment VAULT_FFI_AVAILABLE below.
        //
        // .binaryTarget(
        //     name: "VaultFFI",
        //     path: "build/VaultFFI.xcframework"
        // ),
        .executableTarget(
            name: "BabyVault",
            // Once the xcframework is built:
            //   1. Uncomment the binaryTarget above
            //   2. Add "VaultFFI" to dependencies below
            //   3. Delete ios/Generated/VaultFFIStubs.swift
            //   4. Uncomment .define("VAULT_FFI_AVAILABLE") in swiftSettings
            dependencies: [],
            path: "ios",
            swiftSettings: [
                // .define("VAULT_FFI_AVAILABLE"),
            ]
        ),
    ]
)
