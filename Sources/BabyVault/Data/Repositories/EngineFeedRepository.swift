import Foundation

final class EngineFeedRepository: FeedRepository, @unchecked Sendable {

    private let provider: VaultEngineProvider

    @MainActor
    init(provider: VaultEngineProvider) {
        self.provider = provider
    }

    func logFeed(feedType: FeedType, amountMl: Int?, durationMinutes: Int?, side: String?, notes: String, loggedAt: Date) async throws -> FeedLog {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.logFeed(
                feedType: feedType.toDto,
                amountMl: amountMl.map(UInt32.init),
                durationMinutes: durationMinutes.map(UInt32.init),
                side: side,
                notes: notes,
                loggedAtMillis: Int64(loggedAt.timeIntervalSince1970 * 1_000)
            ).toDomain
        }.value
    }

    func listByRange(from: Date, to: Date) async throws -> [FeedLog] {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.listFeedByRange(
                fromMillis: Int64(from.timeIntervalSince1970 * 1_000),
                toMillis: Int64(to.timeIntervalSince1970 * 1_000)
            ).map(\.toDomain)
        }.value
    }

    func delete(id: String) async throws {
        let engine = try await MainActor.run { try provider.requireEngine() }
        try await Task.detached(priority: .userInitiated) {
            try engine.deleteFeed(id: id)
        }.value
    }
}
