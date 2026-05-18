import Foundation

@Observable
@MainActor
final class ProfileSetupViewModel {

    var name: String = ""
    var dateOfBirth: Date = Calendar.current.date(byAdding: .month, value: -3, to: .now) ?? .now
    var isSaving: Bool = false
    var errorMessage: String?

    private let profileStore: BabyProfileStore

    init(profileStore: BabyProfileStore) {
        self.profileStore = profileStore
    }

    var canSave: Bool {
        !name.trimmingCharacters(in: .whitespaces).isEmpty
    }

    func save() -> BabyProfile? {
        let trimmed = name.trimmingCharacters(in: .whitespaces)
        guard !trimmed.isEmpty else {
            errorMessage = "Please enter a name."
            return nil
        }
        isSaving = true
        defer { isSaving = false }
        let profile = BabyProfile(name: trimmed, dateOfBirth: dateOfBirth)
        profileStore.save(profile: profile)
        return profile
    }
}
