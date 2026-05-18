# Baby Vault iOS

A privacy-first iOS application for tracking baby activities, milestones, and growth — **entirely offline, entirely encrypted, zero cloud dependency**.

All sensitive data is encrypted at rest using ChaCha20-Poly1305 inside a Rust native library before it ever touches the filesystem. The iOS app never sees plaintext on disk.

---

## How It Works

```
┌─────────────────────────────────────────────────────┐
│                 iOS App (Swift)                     │
│                                                     │
│  LocalAuthentication → Keychain → Master Key        │
│                                                     │
│  SwiftUI → ViewModel (@Observable) → Repository    │
└──────────────────────┬──────────────────────────────┘
                       │  UniFFI (auto-generated Swift bindings)
           ┌───────────┴────────────┐
           ▼                        ▼
┌──────────────────┐   ┌────────────────────────────┐
│   Rust: vault    │   │   Rust: inference          │
│                  │   │                            │
│  VaultEngine     │   │  InferenceEngine           │
│  ├── rusqlite    │   │  ├── Burn NdArray (CPU)    │
│  ├── ChaCha20    │   │  └── on-device GPT-2       │
│  └── Use-cases   │   │                            │
└──────────────────┘   └────────────────────────────┘
```

### Security Flow

1. **First launch** — a 32-byte random master key is generated inside Rust. The raw key is wrapped by the iOS Secure Enclave via the Keychain (`kSecAttrAccessibleWhenUnlockedThisDeviceOnly`) and only the wrapped blob is persisted.
2. **Every subsequent launch** — `LocalAuthentication` triggers biometric/passcode auth. The raw key is retrieved from Keychain, passed to `VaultEngine.unlock()` in Rust, held in a `Mutex<Zeroizing<Vec<u8>>>`, and wiped from memory on drop.
3. **All writes** — media bytes are encrypted by the Rust crypto engine before being written to the filesystem. Text fields are stored in an encrypted SQLite database (row-level ChaCha20-Poly1305).
4. **All reads** — decryption happens inside Rust; plaintext is returned only to the in-memory SwiftUI layer.

---

## Features

| Feature | Description |
|---|---|
| **Baby Profile** | Set baby's name and date of birth on first launch; age label auto-calculated |
| **Feed Tracking** | Log breast / bottle / solid feeds with duration, side, amount, and notes |
| **Feed Timer** | Live stopwatch for timed breast-feeding sessions |
| **Sleep Tracking** | Log sleep sessions with start/end times |
| **Diaper Tracking** | Log diaper changes (Wet / Dirty / Both) with optional notes |
| **Milestone Vault** | Record developmental milestones with title, description, and date |
| **Growth Log** | Track height, weight over time with Charts visualisation |
| **Media Vault** | Store photos/videos encrypted at rest via PhotosPicker |
| **Today Summary** | Home screen strip showing today's feed count, total sleep, and diaper changes |
| **History** | Filterable activity log across all event types |
| **Insights** | Weekly stats — avg sleep, feed breakdown, diaper totals |
| **Baby AI Chat** | On-device GPT-2 assistant; runs fully offline |
| **Self-hosted Sync** | Optional: push encrypted blobs to a self-hosted server |
| **Biometric Auth** | Face ID / Touch ID unlock; vault is inaccessible without enrolled biometric |

---

## Repository Layout

```
baby-management-ios/
├── core/                   Rust workspace (shared logic)
│   ├── Cargo.toml          Workspace root
│   ├── rust-toolchain.toml Pins stable + iOS/Android targets
│   ├── apps/
│   │   ├── config/         Shared error types + logger
│   │   ├── vault/          Main cdylib — encryption, SQLite, all use-cases
│   │   └── inference/      GPT-2 inference cdylib
│   └── crates/
│       └── nnapi/          Android NNAPI bindings (Android only, unused on iOS)
│
├── ios/                    Swift source (Package.swift path: "ios")
│   ├── BabyVaultApp.swift  @main entry point
│   ├── Generated/          UniFFI-generated Swift bindings (git-ignored output)
│   │   └── VaultFFIStubs.swift  Compile stubs — replace after build_ios.sh
│   ├── Core/Native/        VaultEngineProvider singleton
│   ├── Domain/
│   │   ├── Models/         Pure Swift structs (Milestone, FeedLog, SleepLog…)
│   │   └── Repositories/   Repository protocols (AnyObject + Sendable)
│   ├── Data/
│   │   ├── Local/          KeychainMasterKeyStore · BabyProfileStore
│   │   ├── Mappers/        DTO → domain model conversions
│   │   └── Repositories/   Engine*Repository implementations
│   ├── DI/                 AppContainer (@Environment injection)
│   └── Presentation/
│       ├── Theme/          Teal pastel colors, button/card styles
│       ├── Navigation/     Route enum + NavigationStack setup
│       └── Screens/        One directory per screen (View + ViewModel)
│           ├── Splash/
│           ├── Unlock/
│           ├── Profile/
│           ├── Home/
│           ├── Log/        Feed · Sleep · Diaper
│           ├── History/
│           ├── Insights/
│           ├── Timeline/
│           ├── Growth/
│           ├── Media/
│           ├── Chat/
│           └── Settings/
│
├── build/                  Gitignored — XCFramework output from build_ios.sh
├── build_ios.sh            One-shot: cargo → lipo → xcframework → uniffi-bindgen
└── Package.swift           SPM manifest (iOS 17+)
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| UI | Swift + SwiftUI + Charts |
| State | `@Observable` view models (iOS 17) |
| Navigation | `NavigationStack` + typed `Route` enum |
| Auth | `LocalAuthentication` (Face ID / Touch ID) |
| Key storage | Keychain (`kSecAttrAccessibleWhenUnlockedThisDeviceOnly`) |
| Core logic | Rust (staticlib, compiled for iOS) |
| FFI bridge | UniFFI 0.28 — auto-generates type-safe Swift bindings from Rust |
| Database | SQLite via `rusqlite` (bundled, runs inside Rust) |
| Encryption | `chacha20poly1305` crate — authenticated encryption |
| Dependency injection | Manual DI via `AppContainer` + `@Environment` |

---

## Getting Started

### Prerequisites

- Xcode 16+
- Rust stable toolchain — [rustup.rs](https://rustup.rs)
- iOS deployment target: iOS 17

### 1. Install iOS Rust targets

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```

### 2. Build the Rust core

```bash
./build_ios.sh
```

This will:
1. Compile `vault` for `aarch64-apple-ios` (device), `aarch64-apple-ios-sim`, and `x86_64-apple-ios` (simulator)
2. Merge the two simulator slices into a fat library with `lipo`
3. Package into `build/VaultFFI.xcframework` via `xcodebuild -create-xcframework`
4. Run `uniffi-bindgen` to emit `ios/Generated/vault.swift`

### 3. Activate the real framework in Package.swift

After the build completes, make three edits in `Package.swift`:

```swift
// 1. Uncomment the binary target
.binaryTarget(
    name: "VaultFFI",
    path: "build/VaultFFI.xcframework"
),

// 2. Add "VaultFFI" to the executable target's dependencies
dependencies: ["VaultFFI"],

// 3. Uncomment the Swift define
.define("VAULT_FFI_AVAILABLE"),
```

Then delete the compile stubs:

```bash
rm ios/Generated/VaultFFIStubs.swift
```

### 4. Open in Xcode

```bash
open Package.swift
```

---

## Architecture

### Clean Architecture layers

```
Domain          Pure Swift structs + AnyObject & Sendable protocols
   ↑
Data            Engine*Repository impls — wrap VaultEngine FFI calls
                FFI calls run on Task.detached (never blocks main thread)
   ↑
Presentation    @Observable ViewModels + SwiftUI Views
                All VMs are @MainActor isolated
```

### Dependency injection

`AppContainer` is a `@MainActor` class that constructs all singletons and repository instances at launch. It is injected into the SwiftUI environment via a custom `EnvironmentKey`:

```swift
@Environment(\.appContainer) private var container
```

### Navigation

A typed `Route` enum drives `NavigationStack`. Each tab has its own `NavigationPath` inside `AppNavigationState` (`@Observable`). Route destinations are registered centrally with `.withRouteDestinations()`.

### Biometric unlock flow

```
SplashView
  ├── No key in Keychain → generate key → ProfileSetupView
  └── Key exists → UnlockView
        └── LAContext.evaluatePolicy(.deviceOwnerAuthentication)
              └── success → retrieve key from Keychain
                         → VaultEngine.unlock(rawKey:)
                         → HomeView
```

---

## Relationship to the Android version

This repo shares the `core/` Rust workspace with the Android app. The only difference is the FFI target:

| | iOS | Android |
|---|---|---|
| Build tool | `cargo build --target aarch64-apple-ios` | `cargo ndk -t aarch64-linux-android` |
| Library type | `staticlib` → XCFramework | `cdylib` → `.so` |
| Bindings | UniFFI → Swift | UniFFI → Kotlin |
| Auth | LocalAuthentication + Keychain | BiometricPrompt + Android Keystore |
| UI | SwiftUI | Jetpack Compose |

---

## Contributing

1. Fork and clone
2. Create a feature branch off `develop`
3. Build the Rust core (`./build_ios.sh`) before opening Xcode
4. Open `Package.swift` in Xcode
5. Submit a PR against `develop`
