import SwiftUI

struct SplashView: View {

    let onTransition: (RootAppState) -> Void
    @Environment(\.appContainer) private var container
    @State private var hasAppeared = false

    var body: some View {
        ZStack {
            LinearGradient.tealSplash.ignoresSafeArea()

            VStack(spacing: 20) {
                Text("👶")
                    .font(.system(size: 72))
                    .scaleEffect(hasAppeared ? 1 : 0.6)
                    .animation(.spring(response: 0.5, dampingFraction: 0.6), value: hasAppeared)

                Text("Baby Vault")
                    .font(.system(size: 34, weight: .bold, design: .rounded))
                    .foregroundStyle(.white)
                    .opacity(hasAppeared ? 1 : 0)
                    .animation(.easeIn(duration: 0.4).delay(0.15), value: hasAppeared)

                ProgressView()
                    .tint(.white)
                    .scaleEffect(1.2)
                    .opacity(hasAppeared ? 1 : 0)
                    .animation(.easeIn(duration: 0.3).delay(0.3), value: hasAppeared)
            }
        }
        .onAppear {
            hasAppeared = true
            Task {
                // Small delay so the animation plays before we navigate.
                try? await Task.sleep(for: .milliseconds(800))
                await determineRoute()
            }
        }
    }

    // MARK: - Route logic

    private func determineRoute() async {
        let hasKey     = container.keychainStore.hasKey
        let hasProfile = container.profileStore.hasProfile

        if hasKey {
            // Key exists — go straight to biometric unlock.
            onTransition(.needsUnlock)
        } else {
            // First launch: generate & store master key, then unlock automatically.
            do {
                let engine = try container.engineProvider.initialise()
                let rawKey = try engine.generateMasterKey()
                try container.keychainStore.store(rawKey: rawKey)
                try engine.unlock(rawKey: rawKey)
                onTransition(hasProfile ? .unlocked : .needsProfile)
            } catch {
                // Fall back to unlock screen where user can retry.
                onTransition(.needsUnlock)
            }
        }
    }
}
