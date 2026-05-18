import Foundation

struct HomeUiState: Sendable {
    var babyName: String = ""
    var babyAgeLabel: String = ""
    var todayFeeds: Int = 0
    var todaySleepMinutes: Int = 0
    var todayDiapers: Int = 0
    var recentMilestones: [Milestone] = []
    var engineVersion: String = ""
    var isLoading: Bool = true
    var error: String?
}

@Observable
@MainActor
final class HomeViewModel {

    var state = HomeUiState()

    private let feedRepo: any FeedRepository
    private let sleepRepo: any SleepRepository
    private let diaperRepo: any DiaperRepository
    private let milestoneRepo: any MilestoneRepository
    private let profileStore: BabyProfileStore
    private let engineProvider: VaultEngineProvider

    init(feedRepo: any FeedRepository,
         sleepRepo: any SleepRepository,
         diaperRepo: any DiaperRepository,
         milestoneRepo: any MilestoneRepository,
         profileStore: BabyProfileStore,
         engineProvider: VaultEngineProvider) {
        self.feedRepo      = feedRepo
        self.sleepRepo     = sleepRepo
        self.diaperRepo    = diaperRepo
        self.milestoneRepo = milestoneRepo
        self.profileStore  = profileStore
        self.engineProvider = engineProvider
    }

    func load() async {
        state.isLoading = true
        defer { state.isLoading = false }

        // Baby profile
        if let profile = profileStore.load() {
            state.babyName     = profile.name
            state.babyAgeLabel = ageLabel(for: profile.dateOfBirth)
        }

        // Engine version (best-effort)
        if let engine = engineProvider.engine {
            state.engineVersion = engine.engineVersion()
        }

        // Today's range
        let calendar = Calendar.current
        let todayStart = calendar.startOfDay(for: .now)
        let now = Date.now

        async let feeds     = (try? feedRepo.listByRange(from: todayStart, to: now)) ?? []
        async let sleeps    = (try? sleepRepo.listByRange(from: todayStart, to: now)) ?? []
        async let diapers   = (try? diaperRepo.listByRange(from: todayStart, to: now)) ?? []
        async let milestones = (try? milestoneRepo.list(limit: 5, offset: 0)) ?? []

        let (f, sl, d, m) = await (feeds, sleeps, diapers, milestones)

        state.todayFeeds         = f.count
        state.todaySleepMinutes  = sl.reduce(0) { $0 + $1.durationMinutes }
        state.todayDiapers       = d.count
        state.recentMilestones   = m
    }

    func refresh() {
        Task { await load() }
    }

    // MARK: - Helpers

    private func ageLabel(for dob: Date) -> String {
        let calendar = Calendar.current
        let components = calendar.dateComponents([.year, .month], from: dob, to: .now)
        let totalMonths = (components.year ?? 0) * 12 + (components.month ?? 0)

        switch totalMonths {
        case 0:       return "newborn"
        case 1..<12:  return "\(totalMonths) month\(totalMonths == 1 ? "" : "s") old"
        default:
            let years = totalMonths / 12
            let rem   = totalMonths % 12
            if rem == 0 { return "\(years) year\(years == 1 ? "" : "s") old" }
            return "\(years) yr \(rem) mo old"
        }
    }
}
