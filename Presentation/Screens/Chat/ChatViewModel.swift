import Foundation

struct ChatMessage: Identifiable, Sendable {
    enum Role: Sendable { case user, assistant }
    let id = UUID()
    let role: Role
    var content: String
    var isStreaming: Bool = false
}

struct ChatUiState: Sendable {
    var messages: [ChatMessage] = []
    var inputText: String = ""
    var isLoading: Bool = false
    var error: String?
}

@Observable
@MainActor
final class ChatViewModel {

    var state = ChatUiState()

    // Context from repositories for AI prompt enrichment
    private let feedRepo: any FeedRepository
    private let sleepRepo: any SleepRepository
    private let milestoneRepo: any MilestoneRepository

    init(feedRepo: any FeedRepository,
         sleepRepo: any SleepRepository,
         milestoneRepo: any MilestoneRepository) {
        self.feedRepo      = feedRepo
        self.sleepRepo     = sleepRepo
        self.milestoneRepo = milestoneRepo
    }

    func sendMessage() {
        let text = state.inputText.trimmingCharacters(in: .whitespaces)
        guard !text.isEmpty, !state.isLoading else { return }

        state.inputText = ""
        let userMsg = ChatMessage(role: .user, content: text)
        state.messages.append(userMsg)
        state.isLoading = true

        Task {
            await generateReply(to: text)
            state.isLoading = false
        }
    }

    // MARK: - Local stub response (replace with real LLM call)

    private func generateReply(to userText: String) async {
        // Build context summary
        let now = Date.now
        let dayStart = Calendar.current.startOfDay(for: now)

        let todayFeeds   = (try? await feedRepo.listByRange(from: dayStart, to: now))?.count ?? 0
        let todaySleeps  = (try? await sleepRepo.listByRange(from: dayStart, to: now)) ?? []
        let totalSleepMins = todaySleeps.reduce(0) { $0 + $1.durationMinutes }
        let milestones   = (try? await milestoneRepo.list(limit: 3, offset: 0)) ?? []

        let contextSnippet = """
        Today: \(todayFeeds) feeds, \(totalSleepMins / 60)h \(totalSleepMins % 60)m sleep.
        Recent milestones: \(milestones.map(\.title).joined(separator: ", ")).
        """

        // Placeholder response — swap with a real inference call when available.
        let response = """
        I'm your baby care assistant! Here's what I know so far:
        \(contextSnippet)

        You asked: "\(userText)"

        (Connect a local LLM model in Settings > AI Model to get intelligent responses.)
        """

        // Simulate streaming
        var assistantMsg = ChatMessage(role: .assistant, content: "", isStreaming: true)
        state.messages.append(assistantMsg)
        let idx = state.messages.count - 1

        for character in response {
            try? await Task.sleep(for: .milliseconds(12))
            state.messages[idx].content.append(character)
        }
        state.messages[idx].isStreaming = false
    }

    func clearHistory() {
        state.messages = []
    }
}
