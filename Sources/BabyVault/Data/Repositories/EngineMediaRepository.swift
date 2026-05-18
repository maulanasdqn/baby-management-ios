import Foundation

final class EngineMediaRepository: MediaRepository, @unchecked Sendable {

    private let provider: VaultEngineProvider

    @MainActor
    init(provider: VaultEngineProvider) {
        self.provider = provider
    }

    func store(title: String, plaintextBytes: Data) async throws -> MediaItem {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.storeMedia(title: title, plaintextBytes: plaintextBytes).toDomain
        }.value
    }

    func read(id: String) async throws -> Data {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.readMedia(id: id)
        }.value
    }

    func list(limit: Int, offset: Int) async throws -> [MediaItem] {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.listMedia(limit: UInt32(limit), offset: UInt32(offset)).map(\.toDomain)
        }.value
    }
}
