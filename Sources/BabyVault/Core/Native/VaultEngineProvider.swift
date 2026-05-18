// VaultEngineProvider.swift
// Singleton actor that initialises VaultEngine with the app's documents-directory paths.

import Foundation

/// Provides the single shared VaultEngine instance for the whole app.
/// Call `initialise()` once at launch (from BabyVaultApp or AppContainer).
@MainActor
final class VaultEngineProvider {

    static let shared = VaultEngineProvider()

    private(set) var engine: VaultEngine?

    private init() {}

    /// Initialise the engine if not already done.
    /// - Returns: the engine, which callers may hold onto.
    @discardableResult
    func initialise() throws -> VaultEngine {
        if let existing = engine { return existing }

        let fm = FileManager.default
        guard let documents = fm.urls(for: .documentDirectory, in: .userDomainMask).first else {
            throw VaultProviderError.noDocumentsDirectory
        }

        let dbPath = documents.appendingPathComponent("vault.db").path
        let mediaDir = documents.appendingPathComponent("media")
        try fm.createDirectory(at: mediaDir, withIntermediateDirectories: true)

        let e = try VaultEngine(dbPath: dbPath, storageDir: mediaDir.path)
        self.engine = e
        return e
    }

    var isInitialised: Bool { engine != nil }

    /// Throws if the engine has not been initialised yet.
    func requireEngine() throws -> VaultEngine {
        guard let e = engine else { throw VaultProviderError.notInitialised }
        return e
    }
}

enum VaultProviderError: Error, LocalizedError {
    case noDocumentsDirectory
    case notInitialised

    var errorDescription: String? {
        switch self {
        case .noDocumentsDirectory: return "Could not locate the app's Documents directory."
        case .notInitialised: return "VaultEngine has not been initialised. Call initialise() first."
        }
    }
}
