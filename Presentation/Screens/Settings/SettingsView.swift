import SwiftUI

struct SettingsView: View {

    @Environment(\.appContainer) private var container
    @State private var viewModel: SettingsViewModel?
    @State private var showDeleteConfirmation = false

    var body: some View {
        Group {
            if let vm = viewModel {
                SettingsContent(
                    viewModel: vm,
                    showDeleteConfirmation: $showDeleteConfirmation
                )
            } else {
                ProgressView()
            }
        }
        .navigationTitle("Settings")
        .onAppear {
            if viewModel == nil {
                viewModel = SettingsViewModel(
                    profileStore:   container.profileStore,
                    engineProvider: container.engineProvider,
                    keychainStore:  container.keychainStore
                )
                viewModel?.load()
            }
        }
    }
}

private struct SettingsContent: View {

    @Bindable var viewModel: SettingsViewModel
    @Binding var showDeleteConfirmation: Bool
    @Environment(AppNavigationState.self) private var navState

    var body: some View {
        List {
            // Profile section
            Section("Profile") {
                if let profile = viewModel.state.profile {
                    LabeledContent("Name", value: profile.name)
                    LabeledContent("Date of Birth", value: profile.dateOfBirth.formatted(date: .abbreviated, time: .omitted))
                } else {
                    Text("No profile saved")
                        .foregroundStyle(Color.textSecondary)
                }
                NavigationLink(value: Route.profileSetup) {
                    Label("Edit Profile", systemImage: "person.fill")
                }
            }

            // Sync section
            Section("Sync") {
                LabeledContent("Status", value: viewModel.state.syncStatus)
                TextField("Server URL", text: $viewModel.state.serverUrl)
                    #if os(iOS)
                    .textInputAutocapitalization(.never)
                    #endif
                    .autocorrectionDisabled()
                SecureField("API Key", text: $viewModel.state.apiKey)
                Button("Save Sync Config") { viewModel.configureSyncServer() }
                    .foregroundStyle(Color.navyPrimary)
                Button {
                    viewModel.syncNow()
                } label: {
                    if viewModel.state.isSyncing {
                        ProgressView()
                    } else {
                        Label("Sync Now", systemImage: "arrow.clockwise")
                    }
                }
                .disabled(viewModel.state.isSyncing)
                if let msg = viewModel.state.syncMessage {
                    Text(msg).font(.footnote).foregroundStyle(Color.textSecondary)
                }
            }

            // Quick access links
            Section("Features") {
                NavigationLink(value: Route.logGrowth) {
                    Label("Growth Records", systemImage: "chart.line.uptrend.xyaxis")
                }
                NavigationLink(value: Route.media) {
                    Label("Media Vault", systemImage: "photo.on.rectangle.angled")
                }
                NavigationLink(value: Route.logMilestone) {
                    Label("Milestones", systemImage: "star.fill")
                }
            }

            // About section
            Section("About") {
                if !viewModel.state.engineVersion.isEmpty {
                    LabeledContent("Engine Version", value: viewModel.state.engineVersion)
                }
                LabeledContent("App Version", value: appVersion())
            }

            // Danger zone
            Section("Danger Zone") {
                Button(role: .destructive) {
                    showDeleteConfirmation = true
                } label: {
                    Label("Delete Vault Key", systemImage: "trash.fill")
                }
            }
        }
        #if os(iOS)
        .listStyle(.insetGrouped)
        #endif
        .background(Color.warmCream.ignoresSafeArea())
        .alert("Error", isPresented: .constant(viewModel.state.error != nil), actions: {
            Button("OK") { viewModel.state.error = nil }
        }, message: {
            Text(viewModel.state.error ?? "")
        })
        .confirmationDialog("Delete Vault Key?", isPresented: $showDeleteConfirmation, titleVisibility: .visible) {
            Button("Delete", role: .destructive) { viewModel.deleteVaultKey() }
            Button("Cancel", role: .cancel) {}
        } message: {
            Text("This will remove the encryption key from Keychain. You will not be able to access any stored data without the key.")
        }
    }

    private func appVersion() -> String {
        Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "1.0"
    }
}
