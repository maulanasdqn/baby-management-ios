import Foundation

@Observable
@MainActor
final class LogSleepViewModel {

    var startTime: Date = Calendar.current.date(byAdding: .hour, value: -1, to: .now) ?? .now
    var endTime: Date = .now
    var notes: String = ""
    var isSaving: Bool = false
    var savedSuccessfully: Bool = false
    var errorMessage: String?

    private let sleepRepo: any SleepRepository

    init(sleepRepo: any SleepRepository) {
        self.sleepRepo = sleepRepo
    }

    var durationMinutes: Int {
        max(0, Int(endTime.timeIntervalSince(startTime) / 60))
    }

    var isValidRange: Bool { endTime > startTime }

    func save() {
        guard isValidRange else {
            errorMessage = "End time must be after start time."
            return
        }
        isSaving = true
        errorMessage = nil
        Task {
            do {
                _ = try await sleepRepo.logSleep(startTime: startTime, endTime: endTime, notes: notes)
                savedSuccessfully = true
            } catch {
                errorMessage = error.localizedDescription
            }
            isSaving = false
        }
    }
}
