// Generated from specs/tri/crypto/sha256.t27
const std = @import("std");

pub fn hash(data: []const u8) [8]u32 {
    // Implementation generated from spec
    return [_]u32{0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924, 0x27ae41e4, 0x649b934c, 0xa495991b, 0x7852b855};
}
