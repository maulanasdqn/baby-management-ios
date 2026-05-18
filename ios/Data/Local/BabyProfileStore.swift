// BabyProfileStore.swift
// Persists the baby profile using UserDefaults.

import Foundation

final class BabyProfileStore: Sendable {

    private enum Keys {
        static let name = "baby_profile_name"
        static let dob  = "baby_profile_dob"
    }

    private let defaults: UserDefaults

    init(defaults: UserDefaults = .standard) {
        self.defaults = defaults
    }

    // MARK: - Save

    func save(profile: BabyProfile) {
        defaults.set(profile.name, forKey: Keys.name)
        defaults.set(profile.dateOfBirth.timeIntervalSince1970, forKey: Keys.dob)
    }

    // MARK: - Load

    func load() -> BabyProfile? {
        guard
            let name = defaults.string(forKey: Keys.name),
            !name.isEmpty,
            defaults.object(forKey: Keys.dob) != nil
        else { return nil }

        let dobInterval = defaults.double(forKey: Keys.dob)
        let dob = Date(timeIntervalSince1970: dobInterval)
        return BabyProfile(name: name, dateOfBirth: dob)
    }

    // MARK: - Convenience

    var hasProfile: Bool { load() != nil }
}
