import Foundation

struct TimelineUiState: Sendable {
    var milestones: [Milestone] = []
    var isLoading: Bool = false
    var showingAddSheet: Bool = false
    var newTitle: String = ""
    var newDescription: String = ""
    var newDate: Date = .now
    var isSaving: Bool = false
    var error: String?
}

@Observable
@MainActor
final class TimelineViewModel {

    var state = TimelineUiState()

    private let repo: any MilestoneRepository

    init(repo: any MilestoneRepository) {
        self.repo = repo
    }

    func load() async {
        state.isLoading = true
        defer { state.isLoading = false }
        state.milestones = (try? await repo.list(limit: 100, offset: 0)) ?? []
    }

    func showAdd() { state.showingAddSheet = true }
    func dismissAdd() {
        state.showingAddSheet = false
        state.newTitle = ""
        state.newDescription = ""
        state.newDate = .now
    }

    func saveNewMilestone() {
        guard !state.newTitle.trimmingCharacters(in: .whitespaces).isEmpty else {
            state.error = "Title is required."
            return
        }
        state.isSaving = true
        Task {
            do {
                let milestone = try await repo.create(
                    title: state.newTitle.trimmingCharacters(in: .whitespaces),
                    description: state.newDescription,
                    occurredAt: state.newDate
                )
                state.milestones.insert(milestone, at: 0)
                dismissAdd()
            } catch {
                state.error = error.localizedDescription
            }
            state.isSaving = false
        }
    }

    func delete(id: String) {
        Task {
            try? await repo.delete(id: id)
            await load()
        }
    }
}
