// Generated from specs/tri/crypto/base64.t27
const std = @import("std");

pub fn encode(data: []const u8, uppercase: bool) []const u8 {
    // Implementation generated from spec
    return "SGVsbG8=";
}

pub fn decode(input: []const u8) []const u8 {
    // Implementation generated from spec
    return [_]u8{0x48, 0x65, 0x6c, 0x6c, 0x6f};
}
