import SwiftUI

struct ChatView: View {

    @Environment(\.appContainer) private var container
    @State private var viewModel: ChatViewModel?

    var body: some View {
        Group {
            if let vm = viewModel {
                ChatContent(viewModel: vm)
            } else {
                ProgressView()
            }
        }
        .navigationTitle("AI Assistant")
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button("Clear") { viewModel?.clearHistory() }
                    .font(.caption)
            }
        }
        .onAppear {
            if viewModel == nil {
                viewModel = ChatViewModel(
                    feedRepo:      container.feedRepository,
                    sleepRepo:     container.sleepRepository,
                    milestoneRepo: container.milestoneRepository
                )
            }
        }
    }
}

private struct ChatContent: View {

    @Bindable var viewModel: ChatViewModel

    var body: some View {
        VStack(spacing: 0) {
            // Messages
            ScrollViewReader { proxy in
                ScrollView {
                    LazyVStack(spacing: 12) {
                        if viewModel.state.messages.isEmpty {
                            WelcomeBanner()
                        }
                        ForEach(viewModel.state.messages) { msg in
                            MessageBubble(message: msg)
                                .id(msg.id)
                        }
                        if viewModel.state.isLoading && viewModel.state.messages.last?.role == .user {
                            TypingIndicator()
                        }
                    }
                    .padding(.horizontal, 16)
                    .padding(.vertical, 12)
                }
                .onChange(of: viewModel.state.messages.count) { _, _ in
                    if let last = viewModel.state.messages.last {
                        withAnimation { proxy.scrollTo(last.id, anchor: .bottom) }
                    }
                }
            }

            Divider()

            // Input bar
            HStack(spacing: 10) {
                TextField("Ask about feeds, sleep, milestones…", text: $viewModel.state.inputText, axis: .vertical)
                    .textFieldStyle(.plain)
                    .lineLimit(1...4)
                    .padding(.horizontal, 14)
                    .padding(.vertical, 10)
                    .background(Color.warmCream)
                    .clipShape(RoundedRectangle(cornerRadius: 20))

                Button {
                    viewModel.sendMessage()
                } label: {
                    Image(systemName: "arrow.up.circle.fill")
                        .font(.system(size: 32))
                        .foregroundStyle(
                            viewModel.state.inputText.trimmingCharacters(in: .whitespaces).isEmpty
                            ? Color.textHint : Color.navyPrimary
                        )
                }
                .disabled(viewModel.state.inputText.trimmingCharacters(in: .whitespaces).isEmpty || viewModel.state.isLoading)
            }
            .padding(.horizontal, 16)
            .padding(.vertical, 12)
            .background(Color.cardWhite)
        }
        .background(Color.warmCream.ignoresSafeArea())
    }
}

private struct WelcomeBanner: View {
    var body: some View {
        VStack(spacing: 12) {
            Text("🤖")
                .font(.system(size: 48))
            Text("Baby Care AI")
                .font(.title2.weight(.bold))
                .foregroundStyle(Color.textPrimary)
            Text("Ask me anything about your baby's feeds, sleep, growth, and milestones.")
                .font(.callout)
                .foregroundStyle(Color.textSecondary)
                .multilineTextAlignment(.center)
        }
        .padding(24)
        .padding(.top, 40)
    }
}

private struct MessageBubble: View {

    let message: ChatMessage

    var body: some View {
        HStack {
            if message.role == .user { Spacer(minLength: 60) }

            Text(message.content + (message.isStreaming ? "▊" : ""))
                .font(.callout)
                .foregroundStyle(message.role == .user ? .white : Color.textPrimary)
                .padding(.horizontal, 14)
                .padding(.vertical, 10)
                .background(
                    message.role == .user
                    ? Color.navyPrimary
                    : Color.cardWhite
                )
                .clipShape(
                    message.role == .user
                    ? RoundedCorner(radius: 18, corners: [.topLeft, .topRight, .bottomLeft])
                    : RoundedCorner(radius: 18, corners: [.topLeft, .topRight, .bottomRight])
                )
                .shadow(color: .black.opacity(0.06), radius: 4, x: 0, y: 2)

            if message.role == .assistant { Spacer(minLength: 60) }
        }
    }
}

private struct TypingIndicator: View {
    @State private var dotPhase = 0

    var body: some View {
        HStack {
            HStack(spacing: 4) {
                ForEach(0..<3) { i in
                    Circle()
                        .fill(Color.textSecondary)
                        .frame(width: 7, height: 7)
                        .opacity(dotPhase == i ? 1 : 0.3)
                }
            }
            .padding(.horizontal, 14)
            .padding(.vertical, 12)
            .background(Color.cardWhite)
            .clipShape(RoundedRectangle(cornerRadius: 18))
            .shadow(color: .black.opacity(0.06), radius: 4, x: 0, y: 2)
            Spacer(minLength: 60)
        }
        .task {
            while true {
                try? await Task.sleep(for: .milliseconds(400))
                dotPhase = (dotPhase + 1) % 3
            }
        }
    }
}

// MARK: - Rounded corner helper

private struct RoundedCorner: Shape {
    var radius: CGFloat
    var corners: UIRectCorner

    func path(in rect: CGRect) -> Path {
        let path = UIBezierPath(
            roundedRect: rect,
            byRoundingCorners: corners,
            cornerRadii: CGSize(width: radius, height: radius)
        )
        return Path(path.cgPath)
    }
}
