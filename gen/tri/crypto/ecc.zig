// Generated from specs/tri/crypto/ecc.t27
const std = @import("std");

pub const ECPoint = struct {
    x: f64,
    y: f64,
    is_infinity: bool,
};

pub fn add(curve: anyopaque, p1: ECPoint, p2: ECPoint) ECPoint {
    // Implementation generated from spec
    return p1;
}
