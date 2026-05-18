// AppContainer.swift
// Central dependency container wired at app launch.
// Passed through the environment via .environment(container) and accessed with @Environment(\.appContainer).

import Foundation
import SwiftUI

@MainActor
final class AppContainer: ObservableObject {

    // MARK: - Singletons

    let engineProvider: VaultEngineProvider
    let keychainStore: KeychainMasterKeyStore
    let profileStore: BabyProfileStore

    // MARK: - Repositories

    let milestoneRepository: any MilestoneRepository
    let growthRepository: any GrowthRepository
    let feedRepository: any FeedRepository
    let sleepRepository: any SleepRepository
    let diaperRepository: any DiaperRepository
    let mediaRepository: any MediaRepository

    // MARK: - Init

    init() {
        let provider = VaultEngineProvider.shared
        self.engineProvider = provider

        self.keychainStore = KeychainMasterKeyStore()
        self.profileStore  = BabyProfileStore()

        self.milestoneRepository = EngineMilestoneRepository(provider: provider)
        self.growthRepository    = EngineGrowthRepository(provider: provider)
        self.feedRepository      = EngineFeedRepository(provider: provider)
        self.sleepRepository     = EngineSleepRepository(provider: provider)
        self.diaperRepository    = EngineDiaperRepository(provider: provider)
        self.mediaRepository     = EngineMediaRepository(provider: provider)
    }
}

// MARK: - Environment key

struct AppContainerKey: EnvironmentKey {
    // nonisolated default — views always receive the real container injected at root.
    nonisolated(unsafe) static var defaultValue: AppContainer = {
        // This is constructed once on first access; safe because AppContainer's
        // mutable state is guarded by @MainActor at the call sites.
        MainActor.assumeIsolated { AppContainer() }
    }()
}

extension EnvironmentValues {
    var appContainer: AppContainer {
        get { self[AppContainerKey.self] }
        set { self[AppContainerKey.self] = newValue }
    }
}
