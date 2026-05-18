import SwiftUI

struct HomeView: View {

    @Environment(\.appContainer) private var container
    @Environment(AppNavigationState.self) private var navState
    @State private var viewModel: HomeViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                HomeContent(viewModel: vm, navigate: navigate)
            } else {
                Color.warmCream.ignoresSafeArea()
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = HomeViewModel(
                    feedRepo:      container.feedRepository,
                    sleepRepo:     container.sleepRepository,
                    diaperRepo:    container.diaperRepository,
                    milestoneRepo: container.milestoneRepository,
                    profileStore:  container.profileStore,
                    engineProvider: container.engineProvider
                )
                Task { await viewModel?.load() }
            }
        }
        #if os(iOS)
        .navigationBarHidden(true)
        #endif
    }

    private func navigate(to route: Route) {
        navState.homePath.append(route)
    }
}

// MARK: - Content

private struct HomeContent: View {

    let viewModel: HomeViewModel
    let navigate: (Route) -> Void

    fileprivate struct QuickAction {
        let label: String
        let icon: String
        let bg: Color
        let fg: Color
        let route: Route
    }

    private let quickActions: [QuickAction] = [
        QuickAction(label: "Feed",      icon: "drop.fill",           bg: .rose100,     fg: .rose400,     route: .logFeed),
        QuickAction(label: "Sleep",     icon: "moon.fill",           bg: .skyBlue100,  fg: .skyBlue400,  route: .logSleep),
        QuickAction(label: "Diaper",    icon: "heart.fill",          bg: .sage100,     fg: .sage400,     route: .logDiaper),
        QuickAction(label: "Milestone", icon: "star.fill",           bg: .teal100,     fg: .teal500,     route: .logMilestone),
        QuickAction(label: "Growth",    icon: "chart.line.uptrend.xyaxis", bg: .lavender100, fg: .lavender400, route: .logGrowth),
        QuickAction(label: "Media",     icon: "camera.fill",         bg: .indigo100,   fg: .indigo400,   route: .media),
    ]

    var body: some View {
        ScrollView {
            VStack(spacing: 0) {
                // Header
                HomeHeaderView(
                    name: viewModel.state.babyName,
                    ageLabel: viewModel.state.babyAgeLabel
                )

                VStack(spacing: 24) {
                    // Today stats strip
                    HStack(spacing: 12) {
                        TodayStatCard(
                            icon: "drop.fill", bg: .rose100, fg: .rose400,
                            value: "\(viewModel.state.todayFeeds)", label: "Feeds"
                        )
                        TodayStatCard(
                            icon: "moon.fill", bg: .skyBlue100, fg: .skyBlue400,
                            value: formatSleep(viewModel.state.todaySleepMinutes), label: "Sleep"
                        )
                        TodayStatCard(
                            icon: "heart.fill", bg: .sage100, fg: .sage400,
                            value: "\(viewModel.state.todayDiapers)", label: "Diapers"
                        )
                    }
                    .padding(.horizontal, 20)

                    // Quick log grid
                    SectionHeader("Quick Log")
                        .padding(.horizontal, 20)

                    LazyVGrid(
                        columns: [GridItem(.flexible(), spacing: 12), GridItem(.flexible(), spacing: 12)],
                        spacing: 12
                    ) {
                        ForEach(quickActions, id: \.label) { action in
                            Button { navigate(action.route) } label: {
                                QuickActionCardView(action: action)
                            }
                            .buttonStyle(.plain)
                        }
                    }
                    .padding(.horizontal, 20)

                    // Recent milestones
                    if !viewModel.state.recentMilestones.isEmpty {
                        SectionHeader("Recent Milestones")
                            .padding(.horizontal, 20)
                        MilestonesListView(milestones: viewModel.state.recentMilestones)
                            .padding(.horizontal, 20)
                    }

                    Spacer(minLength: 100)
                }
                .padding(.top, 20)
            }
        }
        .background(Color.warmCream.ignoresSafeArea())
        .refreshable { await viewModel.load() }
    }

    private func formatSleep(_ minutes: Int) -> String {
        if minutes < 60 { return "\(minutes)m" }
        return "\(minutes / 60)h \(minutes % 60)m"
    }
}

// MARK: - Sub-views

private struct HomeHeaderView: View {
    let name: String
    let ageLabel: String

    var body: some View {
        ZStack {
            LinearGradient.navyHeader.ignoresSafeArea(edges: .top)

            // Decorative blobs
            Circle()
                .fill(Color.pinkBlob.opacity(0.45))
                .frame(width: 160, height: 160)
                .offset(x: 100, y: -40)
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topTrailing)

            Circle()
                .fill(Color.pinkBlob.opacity(0.25))
                .frame(width: 90, height: 90)
                .offset(x: -30, y: 30)
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .bottomLeading)

            VStack(alignment: .leading, spacing: 6) {
                Text("Good morning 👋")
                    .font(.subheadline)
                    .foregroundStyle(.white.opacity(0.72))

                HStack(spacing: 10) {
                    ZStack {
                        Circle()
                            .fill(Color.pinkBlob.opacity(0.35))
                            .frame(width: 38, height: 38)
                        Image(systemName: "heart.fill")
                            .font(.body)
                            .foregroundStyle(.white)
                    }

                    VStack(alignment: .leading, spacing: 2) {
                        Text(name.isEmpty ? "Baby Vault" : name)
                            .font(.title2.bold())
                            .foregroundStyle(.white)
                        if !ageLabel.isEmpty {
                            Text(ageLabel)
                                .font(.caption)
                                .foregroundStyle(.white.opacity(0.68))
                        }
                    }
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.horizontal, 20)
            .padding(.vertical, 24)
            .padding(.top, 44) // safe area top approximate
        }
        .frame(height: 200)
    }
}

private struct TodayStatCard: View {
    let icon: String
    let bg: Color
    let fg: Color
    let value: String
    let label: String

    var body: some View {
        VStack(spacing: 6) {
            ZStack {
                Circle().fill(bg).frame(width: 36, height: 36)
                Image(systemName: icon).font(.system(size: 16)).foregroundStyle(fg)
            }
            Text(value)
                .font(.system(size: 18, weight: .bold))
                .foregroundStyle(Color.textPrimary)
            Text(label)
                .font(.caption2)
                .foregroundStyle(Color.textSecondary)
        }
        .padding(14)
        .frame(maxWidth: .infinity)
        .cardStyle()
    }
}

private struct QuickActionCardView: View {
    let action: HomeContent.QuickAction

    var body: some View {
        HStack(spacing: 12) {
            ZStack {
                RoundedRectangle(cornerRadius: 12)
                    .fill(action.bg)
                    .frame(width: 42, height: 42)
                Image(systemName: action.icon)
                    .font(.system(size: 18))
                    .foregroundStyle(action.fg)
            }
            Text(action.label)
                .font(.callout.weight(.semibold))
                .foregroundStyle(Color.textPrimary)
            Spacer()
        }
        .padding(.horizontal, 16)
        .frame(height: 88)
        .cardStyle()
    }
}

private struct SectionHeader: View {
    let title: String
    init(_ title: String) { self.title = title }

    var body: some View {
        Text(title)
            .font(.headline.weight(.bold))
            .foregroundStyle(Color.textPrimary)
            .frame(maxWidth: .infinity, alignment: .leading)
    }
}

private struct MilestonesListView: View {
    let milestones: [Milestone]

    var body: some View {
        VStack(spacing: 8) {
            ForEach(milestones) { milestone in
                HStack(spacing: 12) {
                    ZStack {
                        Circle().fill(Color.pinkLight).frame(width: 36, height: 36)
                        Image(systemName: "star.fill")
                            .font(.system(size: 16))
                            .foregroundStyle(Color.rose400)
                    }
                    VStack(alignment: .leading, spacing: 2) {
                        Text(milestone.title)
                            .font(.callout.weight(.semibold))
                            .foregroundStyle(Color.textPrimary)
                        if !milestone.description.isEmpty {
                            Text(milestone.description)
                                .font(.caption)
                                .foregroundStyle(Color.textSecondary)
                                .lineLimit(1)
                        }
                    }
                    Spacer()
                }
                .padding(12)
                .cardStyle(cornerRadius: 16)
            }
        }
    }
}
