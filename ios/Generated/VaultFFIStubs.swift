// VaultFFIStubs.swift
//
// Stub implementations of the types that UniFFI will generate from the Rust vault crate.
// These allow the project to compile before `build_ios.sh` has been run.
//
// HOW TO REPLACE WITH THE REAL FRAMEWORK:
//   1. Run `./build_ios.sh` to build VaultFFI.xcframework and generate vault.swift.
//   2. Delete this file (VaultFFIStubs.swift).
//   3. In Package.swift:
//      a. Uncomment the `.binaryTarget` block for "VaultFFI".
//      b. Add "VaultFFI" to the executable target's `dependencies`.
//      c. Uncomment `.define("VAULT_FFI_AVAILABLE")` in swiftSettings.

import Foundation

// MARK: - Enums

public enum FeedTypeDto: Sendable {
    case breast
    case bottle
    case solid
}

public enum DiaperTypeDto: Sendable {
    case wet
    case dirty
    case both
}

// MARK: - DTOs (UniFFI Record → Swift struct)

public struct MilestoneDto: Sendable {
    public var id: String
    public var title: String
    public var description: String
    public var occurredAtMillis: Int64
    public var createdAtMillis: Int64

    public init(id: String, title: String, description: String, occurredAtMillis: Int64, createdAtMillis: Int64) {
        self.id = id
        self.title = title
        self.description = description
        self.occurredAtMillis = occurredAtMillis
        self.createdAtMillis = createdAtMillis
    }
}

public struct GrowthLogDto: Sendable {
    public var id: String
    public var weightGrams: UInt32?
    public var heightMm: UInt32?
    public var notes: String
    public var loggedAtMillis: Int64

    public init(id: String, weightGrams: UInt32?, heightMm: UInt32?, notes: String, loggedAtMillis: Int64) {
        self.id = id
        self.weightGrams = weightGrams
        self.heightMm = heightMm
        self.notes = notes
        self.loggedAtMillis = loggedAtMillis
    }
}

public struct MediaItemDto: Sendable {
    public var id: String
    public var title: String
    public var encryptedPath: String
    public var sizeBytes: UInt64
    public var createdAtMillis: Int64

    public init(id: String, title: String, encryptedPath: String, sizeBytes: UInt64, createdAtMillis: Int64) {
        self.id = id
        self.title = title
        self.encryptedPath = encryptedPath
        self.sizeBytes = sizeBytes
        self.createdAtMillis = createdAtMillis
    }
}

public struct FeedLogDto: Sendable {
    public var id: String
    public var feedType: FeedTypeDto
    public var amountMl: UInt32?
    public var durationMinutes: UInt32?
    public var side: String?
    public var notes: String
    public var loggedAtMillis: Int64

    public init(id: String, feedType: FeedTypeDto, amountMl: UInt32?, durationMinutes: UInt32?, side: String?, notes: String, loggedAtMillis: Int64) {
        self.id = id
        self.feedType = feedType
        self.amountMl = amountMl
        self.durationMinutes = durationMinutes
        self.side = side
        self.notes = notes
        self.loggedAtMillis = loggedAtMillis
    }
}

public struct SleepLogDto: Sendable {
    public var id: String
    public var startTimeMillis: Int64
    public var endTimeMillis: Int64
    public var notes: String
    public var durationMinutes: Int64

    public init(id: String, startTimeMillis: Int64, endTimeMillis: Int64, notes: String, durationMinutes: Int64) {
        self.id = id
        self.startTimeMillis = startTimeMillis
        self.endTimeMillis = endTimeMillis
        self.notes = notes
        self.durationMinutes = durationMinutes
    }
}

public struct DiaperLogDto: Sendable {
    public var id: String
    public var diaperType: DiaperTypeDto
    public var notes: String
    public var loggedAtMillis: Int64

    public init(id: String, diaperType: DiaperTypeDto, notes: String, loggedAtMillis: Int64) {
        self.id = id
        self.diaperType = diaperType
        self.notes = notes
        self.loggedAtMillis = loggedAtMillis
    }
}

public struct SyncStatusDto: Sendable {
    public var pendingMilestones: UInt32
    public var pendingGrowthLogs: UInt32
    public var pendingMediaItems: UInt32
    public var lastSyncedAtMillis: Int64?
    public var isConfigured: Bool

    public init(pendingMilestones: UInt32, pendingGrowthLogs: UInt32, pendingMediaItems: UInt32, lastSyncedAtMillis: Int64?, isConfigured: Bool) {
        self.pendingMilestones = pendingMilestones
        self.pendingGrowthLogs = pendingGrowthLogs
        self.pendingMediaItems = pendingMediaItems
        self.lastSyncedAtMillis = lastSyncedAtMillis
        self.isConfigured = isConfigured
    }
}

// MARK: - VaultEngine stub

/// Stub VaultEngine that matches the interface UniFFI will generate from the Rust vault crate.
/// Replace this entire file with the real generated swift bindings once build_ios.sh has been run.
public final class VaultEngine: @unchecked Sendable {
    public init(dbPath: String, storageDir: String) throws {
        // Stub: no-op initializer
    }

    public func engineVersion() -> String { "stub-0.0.0" }

    public func generateMasterKey() throws -> Data {
        // Return 32 random bytes as a stand-in master key
        var bytes = [UInt8](repeating: 0, count: 32)
        _ = SecRandomCopyBytes(kSecRandomDefault, 32, &bytes)
        return Data(bytes)
    }

    public func unlock(rawKey: Data) throws {}

    public func createMilestone(title: String, description: String, occurredAtMillis: Int64) throws -> MilestoneDto {
        MilestoneDto(id: UUID().uuidString, title: title, description: description,
                     occurredAtMillis: occurredAtMillis, createdAtMillis: Int64(Date().timeIntervalSince1970 * 1000))
    }

    public func listMilestones(limit: UInt32, offset: UInt32) throws -> [MilestoneDto] { [] }

    public func getMilestone(id: String) throws -> MilestoneDto {
        throw VaultError.notFound("Milestone \(id) not found (stub)")
    }

    public func deleteMilestone(id: String) throws {}

    public func logGrowth(weightGrams: UInt32?, heightMm: UInt32?, notes: String, loggedAtMillis: Int64) throws -> GrowthLogDto {
        GrowthLogDto(id: UUID().uuidString, weightGrams: weightGrams, heightMm: heightMm,
                     notes: notes, loggedAtMillis: loggedAtMillis)
    }

    public func listGrowthByRange(fromMillis: Int64, toMillis: Int64) throws -> [GrowthLogDto] { [] }

    public func storeMedia(title: String, plaintextBytes: Data) throws -> MediaItemDto {
        MediaItemDto(id: UUID().uuidString, title: title, encryptedPath: "/stub",
                     sizeBytes: UInt64(plaintextBytes.count), createdAtMillis: Int64(Date().timeIntervalSince1970 * 1000))
    }

    public func readMedia(id: String) throws -> Data { Data() }

    public func listMedia(limit: UInt32, offset: UInt32) throws -> [MediaItemDto] { [] }

    public func logFeed(feedType: FeedTypeDto, amountMl: UInt32?, durationMinutes: UInt32?,
                        side: String?, notes: String, loggedAtMillis: Int64) throws -> FeedLogDto {
        FeedLogDto(id: UUID().uuidString, feedType: feedType, amountMl: amountMl,
                   durationMinutes: durationMinutes, side: side, notes: notes, loggedAtMillis: loggedAtMillis)
    }

    public func listFeedByRange(fromMillis: Int64, toMillis: Int64) throws -> [FeedLogDto] { [] }

    public func deleteFeed(id: String) throws {}

    public func logSleep(startTimeMillis: Int64, endTimeMillis: Int64, notes: String) throws -> SleepLogDto {
        let durationMinutes = (endTimeMillis - startTimeMillis) / 60_000
        return SleepLogDto(id: UUID().uuidString, startTimeMillis: startTimeMillis,
                           endTimeMillis: endTimeMillis, notes: notes, durationMinutes: durationMinutes)
    }

    public func listSleepByRange(fromMillis: Int64, toMillis: Int64) throws -> [SleepLogDto] { [] }

    public func deleteSleep(id: String) throws {}

    public func logDiaper(diaperType: DiaperTypeDto, notes: String, loggedAtMillis: Int64) throws -> DiaperLogDto {
        DiaperLogDto(id: UUID().uuidString, diaperType: diaperType, notes: notes, loggedAtMillis: loggedAtMillis)
    }

    public func listDiaperByRange(fromMillis: Int64, toMillis: Int64) throws -> [DiaperLogDto] { [] }

    public func deleteDiaper(id: String) throws {}

    public func configureSyncServer(serverUrl: String, apiKey: String) throws {}

    public func syncNow() throws -> SyncStatusDto {
        SyncStatusDto(pendingMilestones: 0, pendingGrowthLogs: 0, pendingMediaItems: 0,
                      lastSyncedAtMillis: nil, isConfigured: false)
    }

    public func getSyncStatus() throws -> SyncStatusDto {
        SyncStatusDto(pendingMilestones: 0, pendingGrowthLogs: 0, pendingMediaItems: 0,
                      lastSyncedAtMillis: nil, isConfigured: false)
    }
}

// MARK: - VaultError

public enum VaultError: Error, LocalizedError {
    case notFound(String)
    case engineError(String)
    case locked

    public var errorDescription: String? {
        switch self {
        case .notFound(let msg): return "Not found: \(msg)"
        case .engineError(let msg): return "Engine error: \(msg)"
        case .locked: return "Vault is locked"
        }
    }
}
