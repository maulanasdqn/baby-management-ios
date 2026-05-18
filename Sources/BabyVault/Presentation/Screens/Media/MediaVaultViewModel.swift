import Foundation
import SwiftUI

struct MediaVaultUiState: Sendable {
    var items: [MediaItem] = []
    var isLoading: Bool = false
    var selectedData: Data? = nil
    var isShowingPicker: Bool = false
    var pickerTitle: String = ""
    var isSaving: Bool = false
    var error: String?
}

@Observable
@MainActor
final class MediaVaultViewModel {

    var state = MediaVaultUiState()

    private let repo: any MediaRepository

    init(repo: any MediaRepository) {
        self.repo = repo
    }

    func load() async {
        state.isLoading = true
        defer { state.isLoading = false }
        state.items = (try? await repo.list(limit: 100, offset: 0)) ?? []
    }

    func showPicker() {
        state.isShowingPicker = true
        state.pickerTitle = "Photo \(state.items.count + 1)"
    }

    func importPhoto(data: Data) {
        state.isShowingPicker = false
        state.isSaving = true
        Task {
            do {
                let item = try await repo.store(
                    title: state.pickerTitle.isEmpty ? "Photo" : state.pickerTitle,
                    plaintextBytes: data
                )
                state.items.insert(item, at: 0)
            } catch {
                state.error = error.localizedDescription
            }
            state.isSaving = false
        }
    }

    func readPhoto(id: String) async -> Data? {
        try? await repo.read(id: id)
    }
}
