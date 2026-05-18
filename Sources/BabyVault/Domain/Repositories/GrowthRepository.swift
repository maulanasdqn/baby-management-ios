import Foundation

protocol GrowthRepository: AnyObject, Sendable {
    func log(weightGrams: Int?, heightMm: Int?, notes: String, loggedAt: Date) async throws -> GrowthLog
    func listByRange(from: Date, to: Date) async throws -> [GrowthLog]
}
