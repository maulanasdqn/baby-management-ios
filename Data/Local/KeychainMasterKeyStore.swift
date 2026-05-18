// KeychainMasterKeyStore.swift
// Stores and retrieves the vault master key in the iOS Keychain.
// The master key bytes are stored as a generic password protected by the app's entitlements.
// On devices with Secure Enclave, add kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly
// to further restrict access to unlocked+passcode states.

import Foundation
import Security

final class KeychainMasterKeyStore: Sendable {

    private static let service = "com.babyvault.ios"
    private static let account = "master_key"

    // MARK: - Store

    /// Persist the raw master key bytes in the Keychain.
    /// Overwrites any existing entry for the same service/account.
    func store(rawKey: Data) throws {
        // Delete any existing item first so SecItemAdd doesn't return errSecDuplicateItem.
        deleteItem()

        let query: [CFString: Any] = [
            kSecClass:           kSecClassGenericPassword,
            kSecAttrService:     Self.service,
            kSecAttrAccount:     Self.account,
            kSecValueData:       rawKey,
            kSecAttrAccessible:  kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
        ]

        let status = SecItemAdd(query as CFDictionary, nil)
        guard status == errSecSuccess else {
            throw KeychainError.unhandledError(status: status)
        }
    }

    // MARK: - Retrieve

    /// Retrieve the raw master key bytes from the Keychain.
    /// Returns `nil` if no key has been stored yet.
    func retrieve() throws -> Data? {
        let query: [CFString: Any] = [
            kSecClass:            kSecClassGenericPassword,
            kSecAttrService:      Self.service,
            kSecAttrAccount:      Self.account,
            kSecReturnData:       true,
            kSecMatchLimit:       kSecMatchLimitOne,
        ]

        var result: CFTypeRef?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        switch status {
        case errSecSuccess:
            guard let data = result as? Data else { throw KeychainError.unexpectedData }
            return data
        case errSecItemNotFound:
            return nil
        default:
            throw KeychainError.unhandledError(status: status)
        }
    }

    // MARK: - Delete

    func deleteItem() {
        let query: [CFString: Any] = [
            kSecClass:        kSecClassGenericPassword,
            kSecAttrService:  Self.service,
            kSecAttrAccount:  Self.account,
        ]
        SecItemDelete(query as CFDictionary)
    }

    /// Whether a master key has been persisted.
    var hasKey: Bool {
        (try? retrieve()) != nil
    }
}

// MARK: - Error

enum KeychainError: Error, LocalizedError {
    case unexpectedData
    case unhandledError(status: OSStatus)

    var errorDescription: String? {
        switch self {
        case .unexpectedData:
            return "Keychain returned data in an unexpected format."
        case .unhandledError(let status):
            return "Keychain error with status \(status): \(SecCopyErrorMessageString(status, nil) as String? ?? "unknown")"
        }
    }
}
