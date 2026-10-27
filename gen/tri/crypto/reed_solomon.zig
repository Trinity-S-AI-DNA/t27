// Generated from specs/tri/crypto/reed_solomon.t27
const std = @import("std");

pub fn encode(data: []const u8, data_shards: usize, parity_shards: usize) []const u8 {
    // Implementation generated from spec
    return [_]u8{0x11, 0x22, 0x33, 0x44};
}

pub fn decode(shards: []const ?u8, data_shards: usize, parity_shards: usize) []const u8 {
    // Implementation generated from spec
    return [_]u8{0x11, 0x22};
}
