import Foundation

final class EngineMilestoneRepository: MilestoneRepository, @unchecked Sendable {

    private let provider: VaultEngineProvider

    @MainActor
    init(provider: VaultEngineProvider) {
        self.provider = provider
    }

    func create(title: String, description: String, occurredAt: Date) async throws -> Milestone {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.createMilestone(
                title: title,
                description: description,
                occurredAtMillis: Int64(occurredAt.timeIntervalSince1970 * 1_000)
            ).toDomain
        }.value
    }

    func list(limit: Int, offset: Int) async throws -> [Milestone] {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.listMilestones(limit: UInt32(limit), offset: UInt32(offset)).map(\.toDomain)
        }.value
    }

    func getById(id: String) async throws -> Milestone {
        let engine = try await MainActor.run { try provider.requireEngine() }
        return try await Task.detached(priority: .userInitiated) {
            try engine.getMilestone(id: id).toDomain
        }.value
    }

    func delete(id: String) async throws {
        let engine = try await MainActor.run { try provider.requireEngine() }
        try await Task.detached(priority: .userInitiated) {
            try engine.deleteMilestone(id: id)
        }.value
    }
}
