import Foundation

final class EngineGrowthRepository: GrowthRepository, @unchecked Sendable {

    private let provider: VaultEngineProvider

    @MainActor
    init(provider: VaultEngineProvider) {
        self.provider = provider
    }

    func log(weightGrams: Int?, heightMm: Int?, notes: String, loggedAt: Date) async throws -> GrowthLog {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.logGrowth(
                weightGrams: weightGrams.map(UInt32.init),
                heightMm: heightMm.map(UInt32.init),
                notes: notes,
                loggedAtMillis: Int64(loggedAt.timeIntervalSince1970 * 1_000)
            ).toDomain
        }.value
    }

    func listByRange(from: Date, to: Date) async throws -> [GrowthLog] {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.listGrowthByRange(
                fromMillis: Int64(from.timeIntervalSince1970 * 1_000),
                toMillis: Int64(to.timeIntervalSince1970 * 1_000)
            ).map(\.toDomain)
        }.value
    }
}
