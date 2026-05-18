import Foundation

@Observable
@MainActor
final class LogDiaperViewModel {

    var diaperType: DiaperType = .wet
    var notes: String = ""
    var loggedAt: Date = .now
    var isSaving: Bool = false
    var savedSuccessfully: Bool = false
    var errorMessage: String?

    private let diaperRepo: any DiaperRepository

    init(diaperRepo: any DiaperRepository) {
        self.diaperRepo = diaperRepo
    }

    func save() {
        isSaving = true
        errorMessage = nil
        Task {
            do {
                _ = try await diaperRepo.logDiaper(diaperType: diaperType, notes: notes, loggedAt: loggedAt)
                savedSuccessfully = true
            } catch {
                errorMessage = error.localizedDescription
            }
            isSaving = false
        }
    }
}
