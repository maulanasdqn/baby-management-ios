import Foundation

struct SettingsUiState: Sendable {
    var profile: BabyProfile?
    var engineVersion: String = ""
    var syncStatus: String = "Not configured"
    var serverUrl: String = ""
    var apiKey: String = ""
    var isSyncing: Bool = false
    var syncMessage: String?
    var error: String?
}

@Observable
@MainActor
final class SettingsViewModel {

    var state = SettingsUiState()

    private let profileStore: BabyProfileStore
    private let engineProvider: VaultEngineProvider
    private let keychainStore: KeychainMasterKeyStore

    init(profileStore: BabyProfileStore,
         engineProvider: VaultEngineProvider,
         keychainStore: KeychainMasterKeyStore) {
        self.profileStore   = profileStore
        self.engineProvider = engineProvider
        self.keychainStore  = keychainStore
    }

    func load() {
        state.profile = profileStore.load()
        if let engine = engineProvider.engine {
            state.engineVersion = engine.engineVersion()
            if let status = try? engine.getSyncStatus() {
                state.syncStatus = status.isConfigured
                    ? "Configured (\(status.pendingMilestones) pending)"
                    : "Not configured"
            }
        }
    }

    func configureSyncServer() {
        guard !state.serverUrl.isEmpty, !state.apiKey.isEmpty else {
            state.error = "Server URL and API key are required."
            return
        }
        guard let engine = engineProvider.engine else {
            state.error = "Vault not initialised."
            return
        }
        do {
            try engine.configureSyncServer(serverUrl: state.serverUrl, apiKey: state.apiKey)
            state.syncMessage = "Sync server configured."
            load()
        } catch {
            state.error = error.localizedDescription
        }
    }

    func syncNow() {
        guard let engine = engineProvider.engine else { return }
        state.isSyncing = true
        Task {
            defer { state.isSyncing = false }
            do {
                let status = try engine.syncNow()
                state.syncMessage = "Synced! Pending: \(status.pendingMilestones)"
                load()
            } catch {
                state.error = error.localizedDescription
            }
        }
    }

    func deleteVaultKey() {
        keychainStore.deleteItem()
        state.syncMessage = "Vault key deleted. Restart the app to re-initialise."
    }
}
