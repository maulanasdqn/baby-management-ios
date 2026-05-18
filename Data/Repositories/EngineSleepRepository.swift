import Foundation

final class EngineSleepRepository: SleepRepository, @unchecked Sendable {

    private let provider: VaultEngineProvider

    @MainActor
    init(provider: VaultEngineProvider) {
        self.provider = provider
    }

    func logSleep(startTime: Date, endTime: Date, notes: String) async throws -> SleepLog {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.logSleep(
                startTimeMillis: Int64(startTime.timeIntervalSince1970 * 1_000),
                endTimeMillis: Int64(endTime.timeIntervalSince1970 * 1_000),
                notes: notes
            ).toDomain
        }.value
    }

    func listByRange(from: Date, to: Date) async throws -> [SleepLog] {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.listSleepByRange(
                fromMillis: Int64(from.timeIntervalSince1970 * 1_000),
                toMillis: Int64(to.timeIntervalSince1970 * 1_000)
            ).map(\.toDomain)
        }.value
    }

    func delete(id: String) async throws {
        let engine = try await MainActor.run { try provider.requireEngine() }
        try await Task.detached(priority: .userInitiated) {
            try engine.deleteSleep(id: id)
        }.value
    }
}
