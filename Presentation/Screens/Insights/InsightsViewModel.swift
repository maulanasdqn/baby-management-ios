import Foundation

struct InsightsUiState: Sendable {
    var avgFeedsPerDay: Double = 0
    var avgSleepHoursPerDay: Double = 0
    var avgDiapersPerDay: Double = 0
    var feedsByType: [FeedType: Int] = [:]
    var weeklyFeeds: [Int] = Array(repeating: 0, count: 7)
    var weeklySleepMinutes: [Int] = Array(repeating: 0, count: 7)
    var isLoading: Bool = true
}

@Observable
@MainActor
final class InsightsViewModel {

    var state = InsightsUiState()

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

        let calendar = Calendar.current
        let now = Date.now
        let weekStart = calendar.date(byAdding: .day, value: -6, to: calendar.startOfDay(for: now)) ?? now

        async let feeds   = (try? feedRepo.listByRange(from: weekStart, to: now)) ?? []
        async let sleeps  = (try? sleepRepo.listByRange(from: weekStart, to: now)) ?? []
        async let diapers = (try? diaperRepo.listByRange(from: weekStart, to: now)) ?? []

        let (f, sl, d) = await (feeds, sleeps, diapers)

        let days = 7.0
        state.avgFeedsPerDay     = Double(f.count) / days
        state.avgSleepHoursPerDay = Double(sl.reduce(0) { $0 + $1.durationMinutes }) / 60.0 / days
        state.avgDiapersPerDay   = Double(d.count) / days

        // Feeds by type
        var byType: [FeedType: Int] = [:]
        for feed in f { byType[feed.feedType, default: 0] += 1 }
        state.feedsByType = byType

        // Weekly breakdown (index 0 = 6 days ago, index 6 = today)
        var weeklyFeeds = Array(repeating: 0, count: 7)
        var weeklySleep = Array(repeating: 0, count: 7)
        for feed in f {
            let dayOffset = calendar.dateComponents([.day], from: weekStart, to: feed.loggedAt).day ?? 0
            if dayOffset >= 0 && dayOffset < 7 { weeklyFeeds[dayOffset] += 1 }
        }
        for sleep in sl {
            let dayOffset = calendar.dateComponents([.day], from: weekStart, to: sleep.startTime).day ?? 0
            if dayOffset >= 0 && dayOffset < 7 { weeklySleep[dayOffset] += sleep.durationMinutes }
        }
        state.weeklyFeeds       = weeklyFeeds
        state.weeklySleepMinutes = weeklySleep
    }
}
