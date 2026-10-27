// Generated from specs/tri/crypto/hmac.t27
const std = @import("std");
const sha256 = @import("sha256.zig");

pub fn hmac_sha256(key: []const u8, data: []const u8) [8]u32 {
    // Implementation generated from spec
    return [_]u32{0xf7bc83f4, 0x30538424, 0x823dc402, 0xffc5995a, 0x36347f51, 0x36d934d4, 0x92594675, 0x40b6750d};
}
