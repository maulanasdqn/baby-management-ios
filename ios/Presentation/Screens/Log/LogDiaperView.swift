import SwiftUI

struct LogDiaperView: View {

    @Environment(\.appContainer) private var container
    @Environment(\.dismiss) private var dismiss
    @State private var viewModel: LogDiaperViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                LogDiaperContent(viewModel: vm, dismiss: { dismiss() })
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = LogDiaperViewModel(diaperRepo: container.diaperRepository)
            }
        }
        .navigationTitle("Log Diaper")
        .navigationBarTitleDisplayMode(.inline)
    }
}

private struct LogDiaperContent: View {

    @Bindable var viewModel: LogDiaperViewModel
    let dismiss: () -> Void

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Type selector
                VStack(alignment: .leading, spacing: 8) {
                    Label("Diaper Type", systemImage: "heart.fill")
                        .font(.caption).foregroundStyle(Color.textSecondary)

                    HStack(spacing: 12) {
                        ForEach(DiaperType.allCases, id: \.self) { type in
                            DiaperTypeChip(
                                label: type.rawValue.capitalized,
                                isSelected: viewModel.diaperType == type
                            ) {
                                viewModel.diaperType = type
                            }
                        }
                    }
                }
                .padding(.horizontal, 20)

                VStack(alignment: .leading, spacing: 8) {
                    Label("Time", systemImage: "clock")
                        .font(.caption).foregroundStyle(Color.textSecondary)
                    DatePicker("", selection: $viewModel.loggedAt)
                        .datePickerStyle(.compact).labelsHidden()
                }
                .padding(.horizontal, 20)

                VStack(alignment: .leading, spacing: 8) {
                    Label("Notes", systemImage: "note.text")
                        .font(.caption).foregroundStyle(Color.textSecondary)
                    TextField("Optional notes", text: $viewModel.notes, axis: .vertical)
                        .textFieldStyle(.plain)
                        .lineLimit(3, reservesSpace: true)
                        .padding(12)
                        .background(Color.cardWhite)
                        .clipShape(RoundedRectangle(cornerRadius: 12))
                }
                .padding(.horizontal, 20)

                if let err = viewModel.errorMessage {
                    Text(err).font(.footnote).foregroundStyle(.red).padding(.horizontal)
                }

                Button("Save Diaper") { viewModel.save() }
                    .buttonStyle(PrimaryButtonStyle())
                    .disabled(viewModel.isSaving)
                    .padding(.horizontal)

                Spacer(minLength: 40)
            }
            .padding(.vertical, 20)
        }
        .background(Color.warmCream.ignoresSafeArea())
        .onChange(of: viewModel.savedSuccessfully) { _, saved in
            if saved { dismiss() }
        }
    }
}

private struct DiaperTypeChip: View {
    let label: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(label)
                .font(.callout.weight(.semibold))
                .padding(.horizontal, 16)
                .padding(.vertical, 10)
                .background(isSelected ? Color.sage400 : Color.cardWhite)
                .foregroundStyle(isSelected ? .white : Color.textPrimary)
                .clipShape(RoundedRectangle(cornerRadius: 12))
                .shadow(color: .black.opacity(isSelected ? 0 : 0.06), radius: 4, x: 0, y: 2)
        }
        .buttonStyle(.plain)
    }
}
