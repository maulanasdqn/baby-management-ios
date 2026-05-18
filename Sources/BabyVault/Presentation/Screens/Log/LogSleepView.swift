import SwiftUI

struct LogSleepView: View {

    @Environment(\.appContainer) private var container
    @Environment(\.dismiss) private var dismiss
    @State private var viewModel: LogSleepViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                LogSleepContent(viewModel: vm, dismiss: { dismiss() })
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = LogSleepViewModel(sleepRepo: container.sleepRepository)
            }
        }
        .navigationTitle("Log Sleep")
        .navigationBarTitleDisplayMode(.inline)
    }
}

private struct LogSleepContent: View {

    @Bindable var viewModel: LogSleepViewModel
    let dismiss: () -> Void

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Duration display
                VStack(spacing: 6) {
                    Image(systemName: "moon.fill")
                        .font(.system(size: 40))
                        .foregroundStyle(Color.skyBlue400)
                    Text("\(viewModel.durationMinutes / 60)h \(viewModel.durationMinutes % 60)m")
                        .font(.system(size: 36, weight: .bold, design: .rounded))
                        .foregroundStyle(Color.textPrimary)
                    Text("Duration")
                        .font(.caption)
                        .foregroundStyle(Color.textSecondary)
                }
                .padding()
                .frame(maxWidth: .infinity)
                .cardStyle()
                .padding(.horizontal, 20)

                VStack(alignment: .leading, spacing: 8) {
                    Label("Start Time", systemImage: "play.fill")
                        .font(.caption).foregroundStyle(Color.textSecondary)
                    DatePicker("", selection: $viewModel.startTime)
                        .datePickerStyle(.compact).labelsHidden()
                }
                .padding(.horizontal, 20)

                VStack(alignment: .leading, spacing: 8) {
                    Label("End Time", systemImage: "stop.fill")
                        .font(.caption).foregroundStyle(Color.textSecondary)
                    DatePicker("", selection: $viewModel.endTime, in: viewModel.startTime...)
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

                Button("Save Sleep") { viewModel.save() }
                    .buttonStyle(PrimaryButtonStyle())
                    .disabled(viewModel.isSaving || !viewModel.isValidRange)
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
