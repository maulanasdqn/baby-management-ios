import SwiftUI
import Charts

struct InsightsView: View {

    @Environment(\.appContainer) private var container
    @State private var viewModel: InsightsViewModel?

    var body: some View {
        ScrollView {
            if let vm = viewModel {
                InsightsContent(viewModel: vm)
            } else {
                ProgressView().padding(.top, 60)
            }
        }
        .background(Color.warmCream.ignoresSafeArea())
        .navigationTitle("Insights")
        .refreshable { await viewModel?.load() }
        .onAppear {
            if viewModel == nil {
                viewModel = InsightsViewModel(
                    feedRepo:   container.feedRepository,
                    sleepRepo:  container.sleepRepository,
                    diaperRepo: container.diaperRepository
                )
                Task { await viewModel?.load() }
            }
        }
    }
}

private struct InsightsContent: View {

    let viewModel: InsightsViewModel

    private let dayLabels = ["6d", "5d", "4d", "3d", "2d", "1d", "Today"]

    var body: some View {
        VStack(spacing: 20) {
            // Summary cards
            HStack(spacing: 12) {
                InsightCard(
                    icon: "drop.fill", bg: .rose100, fg: .rose400,
                    value: String(format: "%.1f", viewModel.state.avgFeedsPerDay),
                    label: "Avg feeds/day"
                )
                InsightCard(
                    icon: "moon.fill", bg: .skyBlue100, fg: .skyBlue400,
                    value: String(format: "%.1f h", viewModel.state.avgSleepHoursPerDay),
                    label: "Avg sleep/day"
                )
                InsightCard(
                    icon: "heart.fill", bg: .sage100, fg: .sage400,
                    value: String(format: "%.1f", viewModel.state.avgDiapersPerDay),
                    label: "Avg diapers/day"
                )
            }
            .padding(.horizontal, 20)
            .padding(.top, 16)

            // Weekly feeds bar chart
            VStack(alignment: .leading, spacing: 12) {
                Text("Feeds — Last 7 Days")
                    .font(.headline.weight(.bold))
                    .foregroundStyle(Color.textPrimary)

                Chart {
                    ForEach(Array(viewModel.state.weeklyFeeds.enumerated()), id: \.offset) { index, count in
                        BarMark(
                            x: .value("Day", dayLabels[index]),
                            y: .value("Feeds", count)
                        )
                        .foregroundStyle(Color.rose400)
                        .cornerRadius(4)
                    }
                }
                .frame(height: 160)
            }
            .padding(16)
            .cardStyle()
            .padding(.horizontal, 20)

            // Feed type breakdown
            if !viewModel.state.feedsByType.isEmpty {
                VStack(alignment: .leading, spacing: 12) {
                    Text("Feed Type Breakdown")
                        .font(.headline.weight(.bold))
                        .foregroundStyle(Color.textPrimary)

                    ForEach(FeedType.allCases, id: \.self) { type in
                        let count = viewModel.state.feedsByType[type] ?? 0
                        let total = viewModel.state.feedsByType.values.reduce(0, +)
                        let fraction = total > 0 ? Double(count) / Double(total) : 0

                        HStack {
                            Text(type.rawValue.capitalized)
                                .font(.callout)
                                .foregroundStyle(Color.textPrimary)
                                .frame(width: 70, alignment: .leading)
                            GeometryReader { geo in
                                RoundedRectangle(cornerRadius: 4)
                                    .fill(Color.navyPrimary.opacity(0.2))
                                    .frame(height: 10)
                                    .overlay(alignment: .leading) {
                                        RoundedRectangle(cornerRadius: 4)
                                            .fill(Color.navyPrimary)
                                            .frame(width: geo.size.width * fraction, height: 10)
                                    }
                            }
                            .frame(height: 10)
                            Text("\(count)")
                                .font(.callout.weight(.semibold))
                                .foregroundStyle(Color.textPrimary)
                                .frame(width: 30, alignment: .trailing)
                        }
                    }
                }
                .padding(16)
                .cardStyle()
                .padding(.horizontal, 20)
            }

            Spacer(minLength: 40)
        }
    }
}

private struct InsightCard: View {
    let icon: String
    let bg: Color
    let fg: Color
    let value: String
    let label: String

    var body: some View {
        VStack(spacing: 6) {
            ZStack {
                Circle().fill(bg).frame(width: 36, height: 36)
                Image(systemName: icon).foregroundStyle(fg).font(.system(size: 16))
            }
            Text(value)
                .font(.system(size: 16, weight: .bold))
                .foregroundStyle(Color.textPrimary)
            Text(label)
                .font(.caption2)
                .foregroundStyle(Color.textSecondary)
                .multilineTextAlignment(.center)
        }
        .padding(12)
        .frame(maxWidth: .infinity)
        .cardStyle()
    }
}
