import SwiftUI

struct SplashView: View {

    let onTransition: (RootAppState) -> Void
    @Environment(\.appContainer) private var container
    @State private var hasAppeared = false

    var body: some View {
        ZStack {
            LinearGradient.navyHeader.ignoresSafeArea()

            VStack(spacing: 20) {
                ZStack {
                    Circle()
                        .fill(.white.opacity(0.15))
                        .frame(width: 120, height: 120)
                    Image(systemName: "heart.fill")
                        .font(.system(size: 56))
                        .foregroundStyle(.white)
                }
                .scaleEffect(hasAppeared ? 1 : 0.5)
                .animation(.spring(response: 0.5, dampingFraction: 0.6), value: hasAppeared)

                Text("Baby Vault")
                    .font(.system(size: 34, weight: .bold, design: .rounded))
                    .foregroundStyle(.white)
                    .opacity(hasAppeared ? 1 : 0)
                    .animation(.easeIn(duration: 0.4).delay(0.15), value: hasAppeared)

                Text("Your baby's private diary")
                    .font(.subheadline)
                    .foregroundStyle(.white.opacity(0.7))
                    .opacity(hasAppeared ? 1 : 0)
                    .animation(.easeIn(duration: 0.4).delay(0.25), value: hasAppeared)

                ProgressView()
                    .tint(.white)
                    .scaleEffect(1.2)
                    .padding(.top, 8)
                    .opacity(hasAppeared ? 1 : 0)
                    .animation(.easeIn(duration: 0.3).delay(0.4), value: hasAppeared)
            }
        }
        .onAppear {
            hasAppeared = true
            Task {
                try? await Task.sleep(for: .milliseconds(1_500))
                await determineRoute()
            }
        }
    }

    // MARK: - Route logic

    private func determineRoute() async {
        // Ensure engine is initialised and key exists.
        do {
            let engine = try container.engineProvider.initialise()
            if !container.keychainStore.hasKey {
                let rawKey = try engine.generateMasterKey()
                try container.keychainStore.store(rawKey: rawKey)
                try engine.unlock(rawKey: rawKey)
            }
        } catch {
            // Non-fatal in stub mode — continue to the app.
        }

        let hasProfile = container.profileStore.hasProfile
        onTransition(hasProfile ? .unlocked : .needsProfile)
    }
}
