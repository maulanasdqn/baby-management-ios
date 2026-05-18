import Foundation

struct GrowthUiState: Sendable {
    var logs: [GrowthLog] = []
    var isLoading: Bool = false
    var showingAddSheet: Bool = false
    var weightText: String = ""
    var heightText: String = ""
    var notes: String = ""
    var loggedAt: Date = .now
    var isSaving: Bool = false
    var error: String?
}

@Observable
@MainActor
final class GrowthViewModel {

    var state = GrowthUiState()

    private let repo: any GrowthRepository

    init(repo: any GrowthRepository) {
        self.repo = repo
    }

    func load() async {
        state.isLoading = true
        defer { state.isLoading = false }
        let start = Calendar.current.date(byAdding: .month, value: -6, to: .now) ?? .now
        state.logs = (try? await repo.listByRange(from: start, to: .now)) ?? []
    }

    func showAdd() { state.showingAddSheet = true }
    func dismissAdd() {
        state.showingAddSheet = false
        state.weightText = ""
        state.heightText = ""
        state.notes = ""
        state.loggedAt = .now
        state.error = nil
    }

    func saveLog() {
        let weightGrams = Int(state.weightText)
        let heightMm    = Int(state.heightText)
        guard weightGrams != nil || heightMm != nil else {
            state.error = "Enter weight or height."
            return
        }
        state.isSaving = true
        Task {
            do {
                let log = try await repo.log(
                    weightGrams: weightGrams,
                    heightMm: heightMm,
                    notes: state.notes,
                    loggedAt: state.loggedAt
                )
                state.logs.insert(log, at: 0)
                dismissAdd()
            } catch {
                state.error = error.localizedDescription
            }
            state.isSaving = false
        }
    }
}
