import Foundation

@Observable
@MainActor
final class LogFeedViewModel {

    var feedType: FeedType = .breast
    var amountText: String = ""
    var durationText: String = ""
    var side: String = ""
    var notes: String = ""
    var loggedAt: Date = .now
    var isSaving: Bool = false
    var savedSuccessfully: Bool = false
    var errorMessage: String?

    private let feedRepo: any FeedRepository

    init(feedRepo: any FeedRepository) {
        self.feedRepo = feedRepo
    }

    var amountMl: Int? { Int(amountText) }
    var durationMinutes: Int? { Int(durationText) }

    func save() {
        isSaving = true
        errorMessage = nil
        Task {
            do {
                _ = try await feedRepo.logFeed(
                    feedType: feedType,
                    amountMl: amountMl,
                    durationMinutes: durationMinutes,
                    side: side.isEmpty ? nil : side,
                    notes: notes,
                    loggedAt: loggedAt
                )
                savedSuccessfully = true
            } catch {
                errorMessage = error.localizedDescription
            }
            isSaving = false
        }
    }
}
