import SwiftUI
import Charts

struct GrowthView: View {

    @Environment(\.appContainer) private var container
    @State private var viewModel: GrowthViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                GrowthContent(viewModel: vm)
            } else {
                ProgressView()
            }
        }
        .navigationTitle("Growth")
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button { viewModel?.showAdd() } label: {
                    Image(systemName: "plus")
                }
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = GrowthViewModel(repo: container.growthRepository)
                Task { await viewModel?.load() }
            }
        }
    }
}

private struct GrowthContent: View {

    @Bindable var viewModel: GrowthViewModel

    private var weightData: [(Date, Double)] {
        viewModel.state.logs
            .compactMap { log -> (Date, Double)? in
                guard let w = log.weightGrams else { return nil }
                return (log.loggedAt, Double(w) / 1000.0) // kg
            }
            .sorted { $0.0 < $1.0 }
    }

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Weight chart
                if !weightData.isEmpty {
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Weight (kg)")
                            .font(.headline.weight(.bold))
                            .foregroundStyle(Color.textPrimary)
                        Chart(weightData, id: \.0) { point in
                            LineMark(
                                x: .value("Date", point.0),
                                y: .value("kg", point.1)
                            )
                            .foregroundStyle(Color.navyPrimary)
                            PointMark(
                                x: .value("Date", point.0),
                                y: .value("kg", point.1)
                            )
                            .foregroundStyle(Color.navyPrimary)
                        }
                        .frame(height: 180)
                    }
                    .padding(16)
                    .cardStyle()
                    .padding(.horizontal, 20)
                }

                // Logs list
                VStack(alignment: .leading, spacing: 8) {
                    Text("Records")
                        .font(.headline.weight(.bold))
                        .foregroundStyle(Color.textPrimary)
                        .padding(.horizontal, 20)

                    ForEach(viewModel.state.logs) { log in
                        GrowthLogRow(log: log)
                            .padding(.horizontal, 20)
                    }

                    if viewModel.state.logs.isEmpty {
                        Text("No growth records yet")
                            .font(.callout)
                            .foregroundStyle(Color.textSecondary)
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 40)
                    }
                }

                Spacer(minLength: 40)
            }
            .padding(.top, 20)
        }
        .background(Color.warmCream.ignoresSafeArea())
        .refreshable { await viewModel.load() }
        .sheet(isPresented: $viewModel.state.showingAddSheet) {
            AddGrowthSheet(viewModel: viewModel)
        }
    }
}

private struct GrowthLogRow: View {
    let log: GrowthLog

    var body: some View {
        HStack(spacing: 12) {
            ZStack {
                Circle().fill(Color.lavender100).frame(width: 42, height: 42)
                Image(systemName: "chart.line.uptrend.xyaxis")
                    .foregroundStyle(Color.lavender400)
            }
            VStack(alignment: .leading, spacing: 2) {
                if let w = log.weightGrams {
                    Text(String(format: "%.2f kg", Double(w) / 1000)).font(.callout.weight(.semibold))
                }
                if let h = log.heightMm {
                    Text(String(format: "%.1f cm", Double(h) / 10)).font(.caption).foregroundStyle(Color.textSecondary)
                }
            }
            Spacer()
            Text(log.loggedAt, style: .date)
                .font(.caption2).foregroundStyle(Color.textHint)
        }
        .padding(12)
        .cardStyle(cornerRadius: 14)
    }
}

private struct AddGrowthSheet: View {

    @Bindable var viewModel: GrowthViewModel

    var body: some View {
        NavigationStack {
            Form {
                Section("Measurements") {
                    HStack {
                        Text("Weight (g)")
                        Spacer()
                        TextField("e.g. 5400", text: $viewModel.state.weightText)
                            .keyboardType(.numberPad)
                            .multilineTextAlignment(.trailing)
                    }
                    HStack {
                        Text("Height (mm)")
                        Spacer()
                        TextField("e.g. 560", text: $viewModel.state.heightText)
                            .keyboardType(.numberPad)
                            .multilineTextAlignment(.trailing)
                    }
                }
                Section("When") {
                    DatePicker("Date", selection: $viewModel.state.loggedAt,
                               in: ...Date.now, displayedComponents: .date)
                }
                Section("Notes") {
                    TextField("Optional", text: $viewModel.state.notes, axis: .vertical)
                        .lineLimit(3, reservesSpace: true)
                }
                if let err = viewModel.state.error {
                    Section { Text(err).foregroundStyle(.red).font(.footnote) }
                }
            }
            .navigationTitle("Log Growth")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { viewModel.dismissAdd() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    if viewModel.state.isSaving {
                        ProgressView()
                    } else {
                        Button("Save") { viewModel.saveLog() }
                    }
                }
            }
        }
    }
}
