import Foundation

final class EngineDiaperRepository: DiaperRepository, @unchecked Sendable {

    private let provider: VaultEngineProvider

    @MainActor
    init(provider: VaultEngineProvider) {
        self.provider = provider
    }

    func logDiaper(diaperType: DiaperType, notes: String, loggedAt: Date) async throws -> DiaperLog {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.logDiaper(
                diaperType: diaperType.toDto,
                notes: notes,
                loggedAtMillis: Int64(loggedAt.timeIntervalSince1970 * 1_000)
            ).toDomain
        }.value
    }

    func listByRange(from: Date, to: Date) async throws -> [DiaperLog] {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.listDiaperByRange(
                fromMillis: Int64(from.timeIntervalSince1970 * 1_000),
                toMillis: Int64(to.timeIntervalSince1970 * 1_000)
            ).map(\.toDomain)
        }.value
    }

    func delete(id: String) async throws {
        let engine = try await MainActor.run { try provider.requireEngine() }
        try await Task.detached(priority: .userInitiated) {
            try engine.deleteDiaper(id: id)
        }.value
    }
}
