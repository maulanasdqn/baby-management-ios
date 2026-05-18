// VaultMapper.swift
// Extensions that convert UniFFI-generated DTOs into clean domain models.

import Foundation

// MARK: - Date helpers

private func toDate(_ millis: Int64) -> Date {
    Date(timeIntervalSince1970: TimeInterval(millis) / 1_000)
}

private func toMillis(_ date: Date) -> Int64 {
    Int64(date.timeIntervalSince1970 * 1_000)
}

// MARK: - MilestoneDto → Milestone

extension MilestoneDto {
    var toDomain: Milestone {
        Milestone(
            id: id,
            title: title,
            description: description,
            occurredAt: toDate(occurredAtMillis),
            createdAt: toDate(createdAtMillis)
        )
    }
}

// MARK: - GrowthLogDto → GrowthLog

extension GrowthLogDto {
    var toDomain: GrowthLog {
        GrowthLog(
            id: id,
            weightGrams: weightGrams.map(Int.init),
            heightMm: heightMm.map(Int.init),
            notes: notes,
            loggedAt: toDate(loggedAtMillis)
        )
    }
}

// MARK: - MediaItemDto → MediaItem

extension MediaItemDto {
    var toDomain: MediaItem {
        MediaItem(
            id: id,
            title: title,
            encryptedPath: encryptedPath,
            sizeBytes: sizeBytes,
            createdAt: toDate(createdAtMillis)
        )
    }
}

// MARK: - FeedLogDto → FeedLog

extension FeedTypeDto {
    var toDomain: FeedType {
        switch self {
        case .breast: return .breast
        case .bottle: return .bottle
        case .solid:  return .solid
        }
    }
}

extension FeedType {
    var toDto: FeedTypeDto {
        switch self {
        case .breast: return .breast
        case .bottle: return .bottle
        case .solid:  return .solid
        }
    }
}

extension FeedLogDto {
    var toDomain: FeedLog {
        FeedLog(
            id: id,
            feedType: feedType.toDomain,
            amountMl: amountMl.map(Int.init),
            durationMinutes: durationMinutes.map(Int.init),
            side: side,
            notes: notes,
            loggedAt: toDate(loggedAtMillis)
        )
    }
}

// MARK: - SleepLogDto → SleepLog

extension SleepLogDto {
    var toDomain: SleepLog {
        SleepLog(
            id: id,
            startTime: toDate(startTimeMillis),
            endTime: toDate(endTimeMillis),
            notes: notes,
            durationMinutes: Int(durationMinutes)
        )
    }
}

// MARK: - DiaperLogDto → DiaperLog

extension DiaperTypeDto {
    var toDomain: DiaperType {
        switch self {
        case .wet:   return .wet
        case .dirty: return .dirty
        case .both:  return .both
        }
    }
}

extension DiaperType {
    var toDto: DiaperTypeDto {
        switch self {
        case .wet:   return .wet
        case .dirty: return .dirty
        case .both:  return .both
        }
    }
}

extension DiaperLogDto {
    var toDomain: DiaperLog {
        DiaperLog(
            id: id,
            diaperType: diaperType.toDomain,
            notes: notes,
            loggedAt: toDate(loggedAtMillis)
        )
    }
}
