// AppNavigation.swift
// App-wide navigation routes and tab bar definition.

import SwiftUI

// MARK: - Routes

enum Route: Hashable {
    case splash
    case unlock
    case profileSetup
    case home
    case history
    case insights
    case chat
    case settings
    case logFeed
    case logSleep
    case logDiaper
    case logMilestone
    case logGrowth
    case media
    case profile
    case syncSettings
    case timeline
}

// MARK: - Tab definition

enum Tab: Int, CaseIterable {
    case home, history, insights, chat, settings

    var label: String {
        switch self {
        case .home:     return "Home"
        case .history:  return "History"
        case .insights: return "Insights"
        case .chat:     return "Chat"
        case .settings: return "Settings"
        }
    }

    var systemImage: String {
        switch self {
        case .home:     return "house.fill"
        case .history:  return "clock.fill"
        case .insights: return "chart.bar.fill"
        case .chat:     return "bubble.left.fill"
        case .settings: return "gearshape.fill"
        }
    }
}

// MARK: - AppNavigationState (Observable)

@Observable
final class AppNavigationState {
    var selectedTab: Tab = .home
    var homePath    = NavigationPath()
    var historyPath = NavigationPath()
    var insightsPath = NavigationPath()
    var chatPath    = NavigationPath()
    var settingsPath = NavigationPath()

    func path(for tab: Tab) -> Binding<NavigationPath> {
        switch tab {
        case .home:     return Binding { self.homePath }     set: { self.homePath = $0 }
        case .history:  return Binding { self.historyPath }  set: { self.historyPath = $0 }
        case .insights: return Binding { self.insightsPath } set: { self.insightsPath = $0 }
        case .chat:     return Binding { self.chatPath }     set: { self.chatPath = $0 }
        case .settings: return Binding { self.settingsPath } set: { self.settingsPath = $0 }
        }
    }
}
