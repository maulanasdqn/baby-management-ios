import Foundation

protocol MediaRepository: AnyObject, Sendable {
    func store(title: String, plaintextBytes: Data) async throws -> MediaItem
    func read(id: String) async throws -> Data
    func list(limit: Int, offset: Int) async throws -> [MediaItem]
}
