import Foundation

struct GrowthLog: Identifiable, Sendable, Equatable {
    let id: String
    /// Weight in grams, if recorded.
    let weightGrams: Int?
    /// Height in millimetres, if recorded.
    let heightMm: Int?
    let notes: String
    let loggedAt: Date
}
