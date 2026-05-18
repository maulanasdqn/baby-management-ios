// AppTheme.swift
// Design tokens mirroring the Android app's color scheme, adapted for SwiftUI.

import SwiftUI

// MARK: - Colors

extension Color {
    // Navy primary (matching Android NavyPrimary #3040B0)
    static let navy900       = Color(red: 0.082, green: 0.102, blue: 0.314)  // #151A50
    static let navy800       = Color(red: 0.118, green: 0.141, blue: 0.408)  // #1E2468
    static let navyPrimary   = Color(red: 0.188, green: 0.251, blue: 0.690)  // #3040B0
    static let navyLight     = Color(red: 0.302, green: 0.369, blue: 0.780)  // #4D5EC7

    // Pink / rose accent
    static let pinkBlob      = Color(red: 0.949, green: 0.769, blue: 0.816)  // #F2C4D0
    static let pinkAccent    = Color(red: 0.910, green: 0.475, blue: 0.627)  // #E879A0
    static let pinkLight     = Color(red: 0.984, green: 0.910, blue: 0.941)  // #FBE8F0
    static let rose100       = Color(red: 1.0,   green: 0.941, blue: 0.949)  // #FFF0F3
    static let rose300       = Color(red: 1.0,   green: 0.678, blue: 0.780)  // #FFADC7
    static let rose400       = Color(red: 0.957, green: 0.447, blue: 0.714)  // #F472B6

    // Warm background
    static let warmCream     = Color(red: 0.980, green: 0.961, blue: 0.945)  // #FAF5F2
    static let cardWhite     = Color.white

    // Pastel category tints
    static let amber100      = Color(red: 1.0,   green: 0.984, blue: 0.922)  // #FFFBEB
    static let amber400      = Color(red: 0.961, green: 0.620, blue: 0.043)  // #F59E0B
    static let skyBlue100    = Color(red: 0.937, green: 0.965, blue: 1.0)    // #EFF6FF
    static let skyBlue400    = Color(red: 0.376, green: 0.647, blue: 0.980)  // #60A5FA
    static let sage100       = Color(red: 0.941, green: 0.992, blue: 0.953)  // #F0FDF4
    static let sage400       = Color(red: 0.290, green: 0.871, blue: 0.502)  // #4ADE80
    static let lavender100   = Color(red: 0.961, green: 0.953, blue: 1.0)    // #F5F3FF
    static let lavender400   = Color(red: 0.655, green: 0.545, blue: 0.980)  // #A78BFA
    static let indigo100     = Color(red: 0.933, green: 0.949, blue: 1.0)    // #EEF2FF
    static let indigo400     = Color(red: 0.506, green: 0.549, blue: 0.973)  // #818CF8
    static let teal100       = Color(red: 0.910, green: 0.969, blue: 0.973)  // #E8F7F8
    static let teal400       = Color(red: 0.271, green: 0.722, blue: 0.753)  // #45B8C0
    static let teal500       = Color(red: 0.176, green: 0.651, blue: 0.690)  // #2DA6B0

    // Text
    static let textPrimary   = Color(red: 0.075, green: 0.082, blue: 0.290)  // #13154A
    static let textSecondary = Color(red: 0.541, green: 0.561, blue: 0.659)  // #8A8FA8
    static let textHint      = Color(red: 0.733, green: 0.741, blue: 0.816)  // #BBBDD0
}

// MARK: - Gradients

extension LinearGradient {
    static let navyHeader = LinearGradient(
        colors: [.navy900, .navyPrimary],
        startPoint: .top,
        endPoint: .bottom
    )
    static let tealSplash = LinearGradient(
        colors: [Color(red: 0.17, green: 0.42, blue: 0.58), Color(red: 0.27, green: 0.72, blue: 0.75)],
        startPoint: .top,
        endPoint: .bottom
    )
}

// MARK: - Button styles

struct PrimaryButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .frame(maxWidth: .infinity)
            .frame(height: 56)
            .background(Color.navyPrimary.opacity(configuration.isPressed ? 0.75 : 1.0))
            .foregroundStyle(.white)
            .clipShape(RoundedRectangle(cornerRadius: 16))
            .animation(.easeInOut(duration: 0.1), value: configuration.isPressed)
    }
}

struct SecondaryButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .frame(maxWidth: .infinity)
            .frame(height: 52)
            .background(Color.teal100.opacity(configuration.isPressed ? 0.5 : 1.0))
            .foregroundStyle(Color.navyPrimary)
            .clipShape(RoundedRectangle(cornerRadius: 14))
            .animation(.easeInOut(duration: 0.1), value: configuration.isPressed)
    }
}

// MARK: - Card modifier

struct CardStyle: ViewModifier {
    var cornerRadius: CGFloat = 20

    func body(content: Content) -> some View {
        content
            .background(Color.cardWhite)
            .clipShape(RoundedRectangle(cornerRadius: cornerRadius))
            .shadow(color: .black.opacity(0.06), radius: 6, x: 0, y: 2)
    }
}

extension View {
    func cardStyle(cornerRadius: CGFloat = 20) -> some View {
        modifier(CardStyle(cornerRadius: cornerRadius))
    }
}
