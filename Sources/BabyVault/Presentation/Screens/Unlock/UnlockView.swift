import SwiftUI

struct UnlockView: View {

    let onUnlocked: () -> Void

    @Environment(\.appContainer) private var container
    @State private var viewModel: UnlockViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                UnlockContent(viewModel: vm, onUnlocked: onUnlocked)
            } else {
                LinearGradient.tealSplash.ignoresSafeArea()
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = UnlockViewModel(
                    keychainStore: container.keychainStore,
                    engineProvider: container.engineProvider,
                    profileStore: container.profileStore
                )
            }
        }
    }
}

// MARK: - Content

private struct UnlockContent: View {

    @Bindable var viewModel: UnlockViewModel
    let onUnlocked: () -> Void

    var body: some View {
        ZStack {
            LinearGradient.tealSplash.ignoresSafeArea()

            VStack(spacing: 28) {
                // Icon
                ZStack {
                    Circle()
                        .fill(.white.opacity(0.2))
                        .frame(width: 100, height: 100)
                    Image(systemName: "faceid")
                        .font(.system(size: 52))
                        .foregroundStyle(.white)
                }

                VStack(spacing: 8) {
                    Text("Baby Vault")
                        .font(.system(size: 30, weight: .bold, design: .rounded))
                        .foregroundStyle(.white)
                    Text("Authenticate to continue")
                        .font(.subheadline)
                        .foregroundStyle(.white.opacity(0.8))
                }

                if case .error(let message) = viewModel.state {
                    HStack(spacing: 8) {
                        Image(systemName: "exclamationmark.triangle.fill")
                        Text(message)
                            .font(.footnote)
                    }
                    .foregroundStyle(.white)
                    .padding(12)
                    .background(.white.opacity(0.15))
                    .clipShape(RoundedRectangle(cornerRadius: 12))
                    .padding(.horizontal)
                }

                Button(action: { viewModel.authenticate() }) {
                    Group {
                        if case .authenticating = viewModel.state {
                            ProgressView().tint(Color.teal500)
                        } else if case .unlocking = viewModel.state {
                            ProgressView().tint(Color.teal500)
                        } else {
                            Label("Unlock with Biometrics", systemImage: "faceid")
                                .font(.headline)
                        }
                    }
                }
                .buttonStyle(WhiteButtonStyle())
                .disabled(viewModel.state.isLoading)
                .padding(.horizontal, 32)
            }
        }
        .onChange(of: viewModel.state) { _, newState in
            if case .success = newState { onUnlocked() }
            if case .needsProfile = newState { onUnlocked() }
        }
        .onAppear {
            // Auto-trigger biometric prompt on appearance.
            viewModel.authenticate()
        }
    }
}

// MARK: - White button style (for unlock screen)

private struct WhiteButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .frame(maxWidth: .infinity)
            .frame(height: 56)
            .background(.white.opacity(configuration.isPressed ? 0.85 : 1.0))
            .foregroundStyle(Color.teal500)
            .clipShape(RoundedRectangle(cornerRadius: 16))
            .animation(.easeInOut(duration: 0.1), value: configuration.isPressed)
    }
}

// MARK: - State helpers

private extension UnlockState {
    var isLoading: Bool {
        switch self {
        case .authenticating, .unlocking: return true
        default: return false
        }
    }
}
