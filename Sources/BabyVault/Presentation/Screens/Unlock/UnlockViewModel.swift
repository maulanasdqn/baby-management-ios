import Foundation
import LocalAuthentication

enum UnlockState {
    case idle
    case authenticating
    case unlocking
    case success
    case needsProfile
    case error(String)
}

@Observable
@MainActor
final class UnlockViewModel {

    var state: UnlockState = .idle

    private let keychainStore: KeychainMasterKeyStore
    private let engineProvider: VaultEngineProvider
    private let profileStore: BabyProfileStore

    init(keychainStore: KeychainMasterKeyStore,
         engineProvider: VaultEngineProvider,
         profileStore: BabyProfileStore) {
        self.keychainStore  = keychainStore
        self.engineProvider = engineProvider
        self.profileStore   = profileStore
    }

    // MARK: - Biometric unlock

    func authenticate() {
        guard case .idle = state else { return }
        state = .authenticating

        Task {
            let context = LAContext()
            var error: NSError?

            guard context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) else {
                // Fall back to device passcode
                await evaluatePasscode(context: context)
                return
            }

            do {
                let success = try await context.evaluatePolicy(
                    .deviceOwnerAuthenticationWithBiometrics,
                    localizedReason: "Unlock Baby Vault"
                )
                if success {
                    await unlockVault()
                } else {
                    state = .error("Authentication was not successful.")
                }
            } catch {
                state = .error(error.localizedDescription)
            }
        }
    }

    private func evaluatePasscode(context: LAContext) async {
        do {
            let success = try await context.evaluatePolicy(
                .deviceOwnerAuthentication,
                localizedReason: "Unlock Baby Vault"
            )
            if success {
                await unlockVault()
            } else {
                state = .error("Authentication cancelled.")
            }
        } catch {
            state = .error(error.localizedDescription)
        }
    }

    // MARK: - Vault unlock

    private func unlockVault() async {
        state = .unlocking
        do {
            guard let rawKey = try keychainStore.retrieve() else {
                state = .error("No vault key found. Please re-initialise the app.")
                return
            }
            let engine = try engineProvider.initialise()
            try engine.unlock(rawKey: rawKey)
            let hasProfile = profileStore.hasProfile
            state = hasProfile ? .success : .needsProfile
        } catch {
            state = .error(error.localizedDescription)
        }
    }

    func resetError() {
        state = .idle
    }
}
