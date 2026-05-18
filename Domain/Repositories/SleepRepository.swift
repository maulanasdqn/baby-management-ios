import Foundation

protocol SleepRepository: AnyObject, Sendable {
    func logSleep(startTime: Date, endTime: Date, notes: String) async throws -> SleepLog
    func listByRange(from: Date, to: Date) async throws -> [SleepLog]
    func delete(id: String) async throws
}
