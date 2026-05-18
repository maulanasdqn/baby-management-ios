import Foundation

struct MediaItem: Identifiable, Sendable, Equatable {
    let id: String
    let title: String
    let encryptedPath: String
    let sizeBytes: UInt64
    let createdAt: Date
}
