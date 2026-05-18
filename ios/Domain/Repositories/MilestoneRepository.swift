import Foundation

protocol MilestoneRepository: AnyObject, Sendable {
    func create(title: String, description: String, occurredAt: Date) async throws -> Milestone
    func list(limit: Int, offset: Int) async throws -> [Milestone]
    func getById(id: String) async throws -> Milestone
    func delete(id: String) async throws
}
