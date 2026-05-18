import SwiftUI

struct HistoryView: View {

    @Environment(\.appContainer) private var container
    @State private var viewModel: HistoryViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                HistoryContent(viewModel: vm)
            } else {
                ProgressView()
            }
        }
        .navigationTitle("History")
        .onAppear {
            if viewModel == nil {
                viewModel = HistoryViewModel(
                    feedRepo:   container.feedRepository,
                    sleepRepo:  container.sleepRepository,
                    diaperRepo: container.diaperRepository
                )
                Task { await viewModel?.load() }
            }
        }
    }
}

private struct HistoryContent: View {

    @Bindable var viewModel: HistoryViewModel

    private static let dateFormatter: DateFormatter = {
        let f = DateFormatter()
        f.dateStyle = .short
        f.timeStyle = .short
        return f
    }()

    var body: some View {
        VStack(spacing: 0) {
            // Tab picker
            Picker("Category", selection: $viewModel.state.selectedTab) {
                ForEach(HistoryTab.allCases, id: \.self) { tab in
                    Text(tab.rawValue.capitalized).tag(tab)
                }
            }
            .pickerStyle(.segmented)
            .padding()

            if viewModel.state.isLoading {
                ProgressView().frame(maxWidth: .infinity, maxHeight: .infinity)
            } else {
                List {
                    switch viewModel.state.selectedTab {
                    case .feed:
                        ForEach(viewModel.state.feeds) { log in
                            FeedLogRow(log: log)
                        }
                        .onDelete { indexSet in
                            for i in indexSet {
                                viewModel.deleteFeed(id: viewModel.state.feeds[i].id)
                            }
                        }
                        if viewModel.state.feeds.isEmpty {
                            EmptyHistoryRow(label: "No feed logs in range")
                        }

                    case .sleep:
                        ForEach(viewModel.state.sleeps) { log in
                            SleepLogRow(log: log)
                        }
                        .onDelete { indexSet in
                            for i in indexSet {
                                viewModel.deleteSleep(id: viewModel.state.sleeps[i].id)
                            }
                        }
                        if viewModel.state.sleeps.isEmpty {
                            EmptyHistoryRow(label: "No sleep logs in range")
                        }

                    case .diaper:
                        ForEach(viewModel.state.diapers) { log in
                            DiaperLogRow(log: log)
                        }
                        .onDelete { indexSet in
                            for i in indexSet {
                                viewModel.deleteDiaper(id: viewModel.state.diapers[i].id)
                            }
                        }
                        if viewModel.state.diapers.isEmpty {
                            EmptyHistoryRow(label: "No diaper logs in range")
                        }
                    }
                }
                .listStyle(.plain)
            }
        }
        .background(Color.warmCream.ignoresSafeArea())
        .refreshable { await viewModel.load() }
    }
}

private struct FeedLogRow: View {
    let log: FeedLog
    var body: some View {
        HStack(spacing: 12) {
            ZStack {
                Circle().fill(Color.rose100).frame(width: 38, height: 38)
                Image(systemName: "drop.fill").foregroundStyle(Color.rose400)
            }
            VStack(alignment: .leading, spacing: 2) {
                Text(log.feedType.rawValue.capitalized).font(.callout.weight(.semibold))
                if let ml = log.amountMl { Text("\(ml) ml").font(.caption).foregroundStyle(Color.textSecondary) }
                if let dur = log.durationMinutes { Text("\(dur) min").font(.caption).foregroundStyle(Color.textSecondary) }
            }
            Spacer()
            Text(log.loggedAt, style: .time).font(.caption2).foregroundStyle(Color.textHint)
        }
        .padding(.vertical, 4)
    }
}

private struct SleepLogRow: View {
    let log: SleepLog
    var body: some View {
        HStack(spacing: 12) {
            ZStack {
                Circle().fill(Color.skyBlue100).frame(width: 38, height: 38)
                Image(systemName: "moon.fill").foregroundStyle(Color.skyBlue400)
            }
            VStack(alignment: .leading, spacing: 2) {
                Text("\(log.durationMinutes / 60)h \(log.durationMinutes % 60)m").font(.callout.weight(.semibold))
                Text(log.startTime, style: .time).font(.caption).foregroundStyle(Color.textSecondary)
            }
            Spacer()
            Text(log.endTime, style: .time).font(.caption2).foregroundStyle(Color.textHint)
        }
        .padding(.vertical, 4)
    }
}

private struct DiaperLogRow: View {
    let log: DiaperLog
    var body: some View {
        HStack(spacing: 12) {
            ZStack {
                Circle().fill(Color.sage100).frame(width: 38, height: 38)
                Image(systemName: "heart.fill").foregroundStyle(Color.sage400)
            }
            VStack(alignment: .leading, spacing: 2) {
                Text(log.diaperType.rawValue.capitalized).font(.callout.weight(.semibold))
                if !log.notes.isEmpty {
                    Text(log.notes).font(.caption).foregroundStyle(Color.textSecondary).lineLimit(1)
                }
            }
            Spacer()
            Text(log.loggedAt, style: .time).font(.caption2).foregroundStyle(Color.textHint)
        }
        .padding(.vertical, 4)
    }
}

private struct EmptyHistoryRow: View {
    let label: String
    var body: some View {
        Text(label)
            .font(.callout)
            .foregroundStyle(Color.textSecondary)
            .frame(maxWidth: .infinity)
            .padding()
    }
}
