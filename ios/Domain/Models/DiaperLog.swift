import Foundation

enum DiaperType: String, Sendable, CaseIterable {
    case wet
    case dirty
    case both
}

struct DiaperLog: Identifiable, Sendable, Equatable {
    let id: String
    let diaperType: DiaperType
    let notes: String
    let loggedAt: Date
}
