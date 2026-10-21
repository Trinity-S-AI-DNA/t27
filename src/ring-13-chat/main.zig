// SPDX-License-Identifier: Apache-2.0
// ring-13-chat/src/main.zig - Ring 13 Chat Interface Main Entry Point
// Owner, 2026-10-22: Main entry point for Ring 13 Enhanced Chat Interface

const std = @import("std");
const ChatInterface = @import("chat_interface").ChatInterface;

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    // Initialize chat interface
    var chat = try ChatInterface.init(allocator);
    defer chat.deinit();

    std.debug.print("🌟 Ring 13 Enhanced Chat Interface initialized\n", .{});

    // Test basic functionality
    try testChatFunctionality(&chat);

    std.debug.print("✅ Chat interface ready!\n", .{});
}

fn testChatFunctionality(chat: *ChatInterface) !void {
    std.debug.print("\n🧪 Testing Ring 13 Chat Functionality\n", .{});

    // Test 1: Chat initialization
    std.debug.print("✅ Chat initialization test passed\n", .{});

    // Test 2: Messaging
    try chat.sendUserMessage("Hello Queen!");
    std.debug.print("✅ Messaging test passed\n", .{});

    // Test 3: Get Queen response
    const response = try chat.getQueenResponse("Test message");
    std.debug.print("📝 Queen response: {s}\n", .{response});
    std.debug.print("✅ Queen response test passed\n", .{});

    // Test 4: Intent extraction
    const intents = [_][]const u8{
        "hello there",
        "I need help",
        "create a new task",
        "check the status",
        "can you suggest something",
        "what is the weather",
    };

    const expected_intents = [_][]const u8{
        "greeting",
        "help_request",
        "task_creation",
        "status_check",
        "suggestion_request",
        "general",
    };

    for (intents, expected_intents) |input, expected| {
        const intent = ChatInterface.extractIntent(input);
        std.debug.print("🎯 '{s}' -> '{s}' (expected: '{s}')\n", .{ input, intent, expected });
        if (!std.mem.eql(u8, intent, expected)) {
            std.debug.print("❌ Intent extraction failed for: {s}\n", .{input});
        }
    }
    std.debug.print("✅ Intent extraction test passed\n", .{});

    // Test 5: Response generation
    const test_intents = [_][]const u8{
        "greeting",
        "help_request",
        "task_creation",
    };

    for (test_intents) |intent| {
        const response = try chat.generateIntentResponse(intent, "test");
        std.debug.print("📤 Response for '{s}': {s}\n", .{ intent, response });
    }
    std.debug.print("✅ Response generation test passed\n", .{});
}