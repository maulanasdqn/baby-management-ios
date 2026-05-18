// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "BabyVault",
    platforms: [
        .iOS(.v17),
    ],
    products: [
        .executable(name: "BabyVault", targets: ["BabyVault"]),
    ],
    targets: [
        // Real XCFramework built by build_ios.sh — path-based local binary target.
        // Run `./build_ios.sh` first to generate ../baby-management/ios/VaultFFI.xcframework
        // then remove Generated/VaultFFIStubs.swift and uncomment VAULT_FFI_AVAILABLE below.
        //
        // .binaryTarget(
        //     name: "VaultFFI",
        //     path: "../baby-management/ios/VaultFFI.xcframework"
        // ),
        .executableTarget(
            name: "BabyVault",
            // Once the xcframework is built:
            //   1. Uncomment the binaryTarget above
            //   2. Add "VaultFFI" to dependencies below
            //   3. Delete Sources/BabyVault/Generated/VaultFFIStubs.swift
            //   4. Uncomment .define("VAULT_FFI_AVAILABLE") in swiftSettings
            dependencies: [],
            path: "Sources/BabyVault",
            swiftSettings: [
                // .define("VAULT_FFI_AVAILABLE"),
            ]
        ),
    ]
)
