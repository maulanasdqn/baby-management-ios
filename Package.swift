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
        .binaryTarget(
            name: "VaultFFI",
            path: "build/VaultFFI.xcframework"
        ),
        .executableTarget(
            name: "BabyVault",
            dependencies: ["VaultFFI"],
            path: ".",
            swiftSettings: [
                .define("VAULT_FFI_AVAILABLE"),
            ]
        ),
    ]
)
