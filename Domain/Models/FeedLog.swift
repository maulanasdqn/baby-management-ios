import Foundation

enum FeedType: String, Sendable, CaseIterable {
    case breast
    case bottle
    case solid
}

struct FeedLog: Identifiable, Sendable, Equatable {
    let id: String
    let feedType: FeedType
    let amountMl: Int?
    let durationMinutes: Int?
    let side: String?
    let notes: String
    let loggedAt: Date
}
