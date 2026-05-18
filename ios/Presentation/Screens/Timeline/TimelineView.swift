import SwiftUI

struct TimelineView: View {

    @Environment(\.appContainer) private var container
    @State private var viewModel: TimelineViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                TimelineContent(viewModel: vm)
            } else {
                ProgressView()
            }
        }
        .navigationTitle("Milestones")
        .toolbar {
            ToolbarItem(placement: .automatic) {
                Button { viewModel?.showAdd() } label: {
                    Image(systemName: "plus")
                }
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = TimelineViewModel(repo: container.milestoneRepository)
                Task { await viewModel?.load() }
            }
        }
    }
}

private struct TimelineContent: View {

    @Bindable var viewModel: TimelineViewModel

    var body: some View {
        ZStack {
            Color.warmCream.ignoresSafeArea()

            if viewModel.state.isLoading {
                ProgressView()
            } else if viewModel.state.milestones.isEmpty {
                VStack(spacing: 16) {
                    Image(systemName: "star.circle")
                        .font(.system(size: 56))
                        .foregroundStyle(Color.teal400)
                    Text("No milestones yet")
                        .font(.headline)
                        .foregroundStyle(Color.textSecondary)
                    Button("Add First Milestone") { viewModel.showAdd() }
                        .buttonStyle(PrimaryButtonStyle())
                        .padding(.horizontal, 48)
                }
            } else {
                List {
                    ForEach(viewModel.state.milestones) { milestone in
                        MilestoneRow(milestone: milestone)
                    }
                    .onDelete { indexSet in
                        for i in indexSet {
                            viewModel.delete(id: viewModel.state.milestones[i].id)
                        }
                    }
                }
                .listStyle(.plain)
            }
        }
        .refreshable { await viewModel.load() }
        .sheet(isPresented: $viewModel.state.showingAddSheet) {
            AddMilestoneSheet(viewModel: viewModel)
        }
    }
}

private struct MilestoneRow: View {
    let milestone: Milestone

    var body: some View {
        HStack(spacing: 12) {
            ZStack {
                Circle().fill(Color.pinkLight).frame(width: 42, height: 42)
                Image(systemName: "star.fill").foregroundStyle(Color.rose400).font(.system(size: 18))
            }
            VStack(alignment: .leading, spacing: 3) {
                Text(milestone.title)
                    .font(.callout.weight(.semibold))
                    .foregroundStyle(Color.textPrimary)
                if !milestone.description.isEmpty {
                    Text(milestone.description)
                        .font(.caption)
                        .foregroundStyle(Color.textSecondary)
                        .lineLimit(2)
                }
                Text(milestone.occurredAt, style: .date)
                    .font(.caption2)
                    .foregroundStyle(Color.textHint)
            }
            Spacer()
        }
        .padding(.vertical, 6)
    }
}

private struct AddMilestoneSheet: View {

    @Bindable var viewModel: TimelineViewModel

    var body: some View {
        NavigationStack {
            Form {
                Section("Milestone") {
                    TextField("Title", text: $viewModel.state.newTitle)
                    TextField("Description (optional)", text: $viewModel.state.newDescription, axis: .vertical)
                        .lineLimit(3, reservesSpace: true)
                }
                Section("When") {
                    DatePicker("Date", selection: $viewModel.state.newDate,
                               in: ...Date.now, displayedComponents: .date)
                }
                if let err = viewModel.state.error {
                    Section { Text(err).foregroundStyle(.red).font(.footnote) }
                }
            }
            .navigationTitle("Add Milestone")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { viewModel.dismissAdd() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    if viewModel.state.isSaving {
                        ProgressView()
                    } else {
                        Button("Save") { viewModel.saveNewMilestone() }
                            .disabled(viewModel.state.newTitle.trimmingCharacters(in: .whitespaces).isEmpty)
                    }
                }
            }
        }
    }
}
