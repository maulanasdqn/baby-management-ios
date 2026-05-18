// BabyVaultApp.swift
// @main entry point — wires up AppContainer and kicks off the vault engine.

import SwiftUI

@main
struct BabyVaultApp: App {

    @State private var container = AppContainer()
    @State private var navState  = AppNavigationState()
    @State private var appState  = RootAppState.loading

    var body: some Scene {
        WindowGroup {
            RootView(appState: $appState)
                .environment(\.appContainer, container)
                .environment(navState)
                .task {
                    await bootstrapEngine()
                }
        }
    }

    // MARK: - Bootstrap

    private func bootstrapEngine() async {
        do {
            try container.engineProvider.initialise()
        } catch {
            // Engine init failure is non-fatal for layout; VaultEngineProvider surfaces
            // the error when repositories are first accessed.
        }
    }
}

// MARK: - Root app state

enum RootAppState {
    case loading
    case needsSetup      // first launch — no master key in Keychain
    case needsUnlock     // master key exists but vault is locked
    case unlocked        // vault is open, profile may still need setup
    case needsProfile    // vault unlocked but no baby profile saved
}

// MARK: - RootView

struct RootView: View {

    @Binding var appState: RootAppState
    @Environment(\.appContainer) private var container

    var body: some View {
        Group {
            switch appState {
            case .loading:
                SplashView(onTransition: { newState in
                    withAnimation(.easeInOut(duration: 0.35)) {
                        appState = newState
                    }
                })

            case .needsSetup, .needsUnlock:
                UnlockView(onUnlocked: {
                    withAnimation(.easeInOut(duration: 0.35)) {
                        let hasProfile = container.profileStore.hasProfile
                        appState = hasProfile ? .unlocked : .needsProfile
                    }
                })

            case .needsProfile:
                ProfileSetupView(onDone: {
                    withAnimation(.easeInOut(duration: 0.35)) {
                        appState = .unlocked
                    }
                })

            case .unlocked:
                MainTabView()
            }
        }
        .background(Color.warmCream.ignoresSafeArea())
    }
}

// MARK: - MainTabView

struct MainTabView: View {

    @Environment(AppNavigationState.self) private var navState

    var body: some View {
        @Bindable var navState = navState

        TabView(selection: $navState.selectedTab) {
            NavigationStack(path: navState.path(for: .home)) {
                HomeView()
            }
            .withRouteDestinations()
            .tabItem { Label(Tab.home.label, systemImage: Tab.home.systemImage) }
            .tag(Tab.home)

            NavigationStack(path: navState.path(for: .history)) {
                HistoryView()
            }
            .withRouteDestinations()
            .tabItem { Label(Tab.history.label, systemImage: Tab.history.systemImage) }
            .tag(Tab.history)

            NavigationStack(path: navState.path(for: .insights)) {
                InsightsView()
            }
            .withRouteDestinations()
            .tabItem { Label(Tab.insights.label, systemImage: Tab.insights.systemImage) }
            .tag(Tab.insights)

            NavigationStack(path: navState.path(for: .chat)) {
                ChatView()
            }
            .withRouteDestinations()
            .tabItem { Label(Tab.chat.label, systemImage: Tab.chat.systemImage) }
            .tag(Tab.chat)

            NavigationStack(path: navState.path(for: .settings)) {
                SettingsView()
            }
            .withRouteDestinations()
            .tabItem { Label(Tab.settings.label, systemImage: Tab.settings.systemImage) }
            .tag(Tab.settings)
        }
        .tint(.navyPrimary)
    }
}
