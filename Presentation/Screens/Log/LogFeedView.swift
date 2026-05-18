import SwiftUI

struct LogFeedView: View {

    @Environment(\.appContainer) private var container
    @Environment(\.dismiss) private var dismiss
    @State private var viewModel: LogFeedViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                LogFeedContent(viewModel: vm, dismiss: { dismiss() })
            } else {
                Color.warmCream.ignoresSafeArea()
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = LogFeedViewModel(feedRepo: container.feedRepository)
            }
        }
        .navigationTitle("Log Feed")
        #if os(iOS)
        .navigationBarTitleDisplayMode(.inline)
        #endif
    }
}

private struct LogFeedContent: View {

    @Bindable var viewModel: LogFeedViewModel
    let dismiss: () -> Void

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Feed type picker
                VStack(alignment: .leading, spacing: 8) {
                    Label("Feed Type", systemImage: "drop.fill").formLabel()
                    Picker("Type", selection: $viewModel.feedType) {
                        ForEach(FeedType.allCases, id: \.self) { type in
                            Text(type.rawValue.capitalized).tag(type)
                        }
                    }
                    .pickerStyle(.segmented)
                }
                .formSection()

                // Breast fields
                if viewModel.feedType == .breast {
                    VStack(alignment: .leading, spacing: 8) {
                        Label("Side (optional)", systemImage: "arrow.left.arrow.right").formLabel()
                        TextField("Left / Right / Both", text: $viewModel.side)
                            .textFieldStyle(.plain)
                            .formInput()
                    }
                    .formSection()

                    VStack(alignment: .leading, spacing: 8) {
                        Label("Duration (minutes)", systemImage: "timer").formLabel()
                        TextField("e.g. 15", text: $viewModel.durationText)
                            .textFieldStyle(.plain)
                            #if os(iOS)
                            .keyboardType(.numberPad)
                            #endif
                            .formInput()
                    }
                    .formSection()
                }

                // Bottle amount
                if viewModel.feedType == .bottle {
                    VStack(alignment: .leading, spacing: 8) {
                        Label("Amount (ml)", systemImage: "flask.fill").formLabel()
                        TextField("e.g. 120", text: $viewModel.amountText)
                            .textFieldStyle(.plain)
                            #if os(iOS)
                            .keyboardType(.numberPad)
                            #endif
                            .formInput()
                    }
                    .formSection()
                }

                // Time
                VStack(alignment: .leading, spacing: 8) {
                    Label("Time", systemImage: "clock").formLabel()
                    DatePicker("", selection: $viewModel.loggedAt)
                        .datePickerStyle(.compact)
                        .labelsHidden()
                }
                .formSection()

                // Notes
                VStack(alignment: .leading, spacing: 8) {
                    Label("Notes", systemImage: "note.text").formLabel()
                    TextField("Optional notes", text: $viewModel.notes, axis: .vertical)
                        .textFieldStyle(.plain)
                        .lineLimit(3, reservesSpace: true)
                        .formInput()
                }
                .formSection()

                if let err = viewModel.errorMessage {
                    Text(err).font(.footnote).foregroundStyle(.red)
                }

                Button("Save Feed") { viewModel.save() }
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

// MARK: - Form styling helpers

private extension View {
    func formLabel() -> some View {
        self.font(.caption).foregroundStyle(Color.textSecondary)
    }
    func formInput() -> some View {
        self.padding(12)
            .background(Color.cardWhite)
            .clipShape(RoundedRectangle(cornerRadius: 12))
    }
    func formSection() -> some View {
        self.padding(.horizontal, 20)
    }
}
