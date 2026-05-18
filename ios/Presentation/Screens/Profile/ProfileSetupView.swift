import SwiftUI

struct ProfileSetupView: View {

    let onDone: () -> Void

    @Environment(\.appContainer) private var container
    @State private var viewModel: ProfileSetupViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                ProfileSetupContent(viewModel: vm, onDone: onDone)
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = ProfileSetupViewModel(profileStore: container.profileStore)
            }
        }
    }
}

// MARK: - Content

private struct ProfileSetupContent: View {

    @Bindable var viewModel: ProfileSetupViewModel
    let onDone: () -> Void

    var body: some View {
        ZStack {
            Color.warmCream.ignoresSafeArea()

            ScrollView {
                VStack(spacing: 32) {
                    // Header
                    VStack(spacing: 12) {
                        Text("👶")
                            .font(.system(size: 64))
                        Text("Welcome to Baby Vault")
                            .font(.system(size: 26, weight: .bold, design: .rounded))
                            .foregroundStyle(Color.textPrimary)
                        Text("Tell us about your little one")
                            .font(.subheadline)
                            .foregroundStyle(Color.textSecondary)
                    }
                    .padding(.top, 48)

                    // Form
                    VStack(spacing: 20) {
                        VStack(alignment: .leading, spacing: 6) {
                            Label("Baby's Name", systemImage: "person.fill")
                                .font(.caption)
                                .foregroundStyle(Color.textSecondary)
                            TextField("e.g. Emma", text: $viewModel.name)
                                .textFieldStyle(.plain)
                                .padding(14)
                                .background(Color.cardWhite)
                                .clipShape(RoundedRectangle(cornerRadius: 14))
                                .overlay(
                                    RoundedRectangle(cornerRadius: 14)
                                        .stroke(Color.navyPrimary.opacity(0.15), lineWidth: 1)
                                )
                                .font(.body)
                        }

                        VStack(alignment: .leading, spacing: 6) {
                            Label("Date of Birth", systemImage: "calendar")
                                .font(.caption)
                                .foregroundStyle(Color.textSecondary)
                            DatePicker("", selection: $viewModel.dateOfBirth,
                                       in: ...Date.now,
                                       displayedComponents: .date)
                                .datePickerStyle(.compact)
                                .labelsHidden()
                                .padding(14)
                                .frame(maxWidth: .infinity, alignment: .leading)
                                .background(Color.cardWhite)
                                .clipShape(RoundedRectangle(cornerRadius: 14))
                        }
                    }
                    .padding(.horizontal, 24)

                    if let err = viewModel.errorMessage {
                        Text(err)
                            .font(.footnote)
                            .foregroundStyle(.red)
                    }

                    Button("Get Started") {
                        if viewModel.save() != nil { onDone() }
                    }
                    .buttonStyle(PrimaryButtonStyle())
                    .disabled(!viewModel.canSave || viewModel.isSaving)
                    .padding(.horizontal, 24)

                    Spacer(minLength: 32)
                }
            }
        }
    }
}
