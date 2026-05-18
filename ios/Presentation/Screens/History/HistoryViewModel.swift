import Foundation

enum HistoryTab: String, CaseIterable, Sendable {
    case feed, sleep, diaper
}

struct HistoryUiState: Sendable {
    var selectedTab: HistoryTab = .feed
    var feeds: [FeedLog] = []
    var sleeps: [SleepLog] = []
    var diapers: [DiaperLog] = []
    var isLoading: Bool = false
    var error: String?
    var rangeStart: Date = Calendar.current.date(byAdding: .day, value: -7, to: .now) ?? .now
    var rangeEnd: Date = .now
}

@Observable
@MainActor
final class HistoryViewModel {

    var state = HistoryUiState()

    private let feedRepo: any FeedRepository
    private let sleepRepo: any SleepRepository
    private let diaperRepo: any DiaperRepository

    init(feedRepo: any FeedRepository,
         sleepRepo: any SleepRepository,
         diaperRepo: any DiaperRepository) {
        self.feedRepo   = feedRepo
        self.sleepRepo  = sleepRepo
        self.diaperRepo = diaperRepo
    }

    func load() async {
        state.isLoading = true
        defer { state.isLoading = false }

        let from = state.rangeStart
        let to   = state.rangeEnd

        async let feeds   = (try? feedRepo.listByRange(from: from, to: to)) ?? []
        async let sleeps  = (try? sleepRepo.listByRange(from: from, to: to)) ?? []
        async let diapers = (try? diaperRepo.listByRange(from: from, to: to)) ?? []

        let (f, sl, d) = await (feeds, sleeps, diapers)
        state.feeds   = f
        state.sleeps  = sl
        state.diapers = d
    }

    func selectTab(_ tab: HistoryTab) {
        state.selectedTab = tab
    }

    func deleteFeed(id: String) {
        Task {
            try? await feedRepo.delete(id: id)
            await load()
        }
    }

    func deleteSleep(id: String) {
        Task {
            try? await sleepRepo.delete(id: id)
            await load()
        }
    }

    func deleteDiaper(id: String) {
        Task {
            try? await diaperRepo.delete(id: id)
            await load()
        }
    }
}
