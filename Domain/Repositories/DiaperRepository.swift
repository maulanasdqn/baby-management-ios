import Foundation

protocol DiaperRepository: AnyObject, Sendable {
    func logDiaper(diaperType: DiaperType, notes: String, loggedAt: Date) async throws -> DiaperLog
    func listByRange(from: Date, to: Date) async throws -> [DiaperLog]
    func delete(id: String) async throws
}
