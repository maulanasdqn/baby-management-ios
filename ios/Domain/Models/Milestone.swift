import Foundation

struct Milestone: Identifiable, Sendable, Equatable {
    let id: String
    let title: String
    let description: String
    let occurredAt: Date
    let createdAt: Date
}
