import Foundation

protocol FeedRepository: AnyObject, Sendable {
    func logFeed(feedType: FeedType, amountMl: Int?, durationMinutes: Int?, side: String?, notes: String, loggedAt: Date) async throws -> FeedLog
    func listByRange(from: Date, to: Date) async throws -> [FeedLog]
    func delete(id: String) async throws
}
