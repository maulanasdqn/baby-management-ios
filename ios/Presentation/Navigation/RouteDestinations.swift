// RouteDestinations.swift
// Centralised NavigationDestination handler applied to each NavigationStack.

import SwiftUI

extension View {
    /// Attach all route destinations used throughout the app.
    func withRouteDestinations() -> some View {
        self
            .navigationDestination(for: Route.self) { route in
                RouteDestinationView(route: route)
            }
    }
}

private struct RouteDestinationView: View {

    let route: Route

    var body: some View {
        switch route {
        case .logFeed:       LogFeedView()
        case .logSleep:      LogSleepView()
        case .logDiaper:     LogDiaperView()
        case .logMilestone:  TimelineView()
        case .logGrowth:     GrowthView()
        case .media:         MediaVaultView()
        case .profileSetup:  ProfileSetupView(onDone: {})
        case .timeline:      TimelineView()
        default:
            Text("Screen not found").foregroundStyle(Color.textSecondary)
        }
    }
}
