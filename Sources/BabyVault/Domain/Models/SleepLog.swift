import Foundation

struct SleepLog: Identifiable, Sendable, Equatable {
    let id: String
    let startTime: Date
    let endTime: Date
    let notes: String
    let durationMinutes: Int
}
