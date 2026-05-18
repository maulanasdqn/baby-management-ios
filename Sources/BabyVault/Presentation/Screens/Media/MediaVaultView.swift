import SwiftUI
import PhotosUI

struct MediaVaultView: View {

    @Environment(\.appContainer) private var container
    @State private var viewModel: MediaVaultViewModel?
    @State private var selectedItem: PhotosPickerItem?

    var body: some View {
        Group {
            if let vm = viewModel {
                MediaVaultContent(viewModel: vm, selectedItem: $selectedItem)
            } else {
                ProgressView()
            }
        }
        .navigationTitle("Media Vault")
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                PhotosPicker(selection: $selectedItem, matching: .images) {
                    Image(systemName: "plus")
                }
            }
        }
        .onChange(of: selectedItem) { _, newItem in
            guard let item = newItem else { return }
            Task {
                if let data = try? await item.loadTransferable(type: Data.self) {
                    viewModel?.importPhoto(data: data)
                }
                selectedItem = nil
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = MediaVaultViewModel(repo: container.mediaRepository)
                Task { await viewModel?.load() }
            }
        }
    }
}

private struct MediaVaultContent: View {

    @Bindable var viewModel: MediaVaultViewModel
    @Binding var selectedItem: PhotosPickerItem?

    private let columns = [
        GridItem(.flexible(), spacing: 4),
        GridItem(.flexible(), spacing: 4),
        GridItem(.flexible(), spacing: 4),
    ]

    var body: some View {
        ScrollView {
            if viewModel.state.isLoading {
                ProgressView().padding(.top, 60)
            } else if viewModel.state.items.isEmpty {
                VStack(spacing: 16) {
                    Image(systemName: "photo.on.rectangle.angled")
                        .font(.system(size: 56))
                        .foregroundStyle(Color.indigo400)
                    Text("No media yet")
                        .font(.headline)
                        .foregroundStyle(Color.textSecondary)
                    Text("Tap + to add your first encrypted photo")
                        .font(.caption)
                        .foregroundStyle(Color.textHint)
                        .multilineTextAlignment(.center)
                }
                .padding(.top, 80)
            } else {
                LazyVGrid(columns: columns, spacing: 4) {
                    ForEach(viewModel.state.items) { item in
                        MediaThumbView(item: item, viewModel: viewModel)
                    }
                }
                .padding(4)
            }
        }
        .background(Color.warmCream.ignoresSafeArea())
        .refreshable { await viewModel.load() }
        .overlay {
            if viewModel.state.isSaving {
                ZStack {
                    Color.black.opacity(0.3).ignoresSafeArea()
                    VStack(spacing: 12) {
                        ProgressView().tint(.white).scaleEffect(1.5)
                        Text("Encrypting…").foregroundStyle(.white).font(.caption)
                    }
                }
            }
        }
        .alert("Error", isPresented: .constant(viewModel.state.error != nil), actions: {
            Button("OK") { viewModel.state.error = nil }
        }, message: {
            Text(viewModel.state.error ?? "")
        })
    }
}

private struct MediaThumbView: View {

    let item: MediaItem
    let viewModel: MediaVaultViewModel

    @State private var imageData: Data?
    @State private var isLoading = false

    var body: some View {
        ZStack {
            Color.indigo100
            if isLoading {
                ProgressView()
            } else if let data = imageData, let uiImage = UIImage(data: data) {
                Image(uiImage: uiImage)
                    .resizable()
                    .scaledToFill()
                    .clipped()
            } else {
                Image(systemName: "lock.fill")
                    .foregroundStyle(Color.indigo400)
            }
        }
        .aspectRatio(1, contentMode: .fit)
        .clipShape(RoundedRectangle(cornerRadius: 8))
        .overlay(alignment: .bottomLeading) {
            Text(item.title)
                .font(.caption2)
                .foregroundStyle(.white)
                .padding(4)
                .background(.black.opacity(0.4))
                .clipShape(RoundedRectangle(cornerRadius: 4))
                .padding(4)
        }
        .task {
            isLoading = true
            imageData = await viewModel.readPhoto(id: item.id)
            isLoading = false
        }
    }
}
