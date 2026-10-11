//! `std.mem` calls on strings, as t27c's Zig backend prints them
//! (verbatim) and Zig 0.16 computes them:
//!
//! - `std.mem.eql(u8, a, b)`: equal lengths and equal bytes, exactly `a == b`
//!   on two `str` values (two literals fold; otherwise `__t27b_str_eql`);
//! - `std.mem.indexOf(u8, hay, needle)`: the `?usize` index of the first
//!   occurrence, null when there is none -- compared with `null` as any
//!   optional is;
//! - `std.mem.trim` / `std.mem.indexOfScalar` / `std.math.approxEqAbs` (with
//!   `std.mem.indexOf` above): each a call of its routine from
//!   specs/tri/t27b/stdmem.t27, spliced under t27b's reserved prefix, so the
//!   bytes are Zig's own std's; stdmem_plan.t27 decides what runs and what
//!   stays refused (#8965).
//!
//! The element type must be the identifier `u8`, and both slices `str`
//! (`[]const u8`), or bytes in a buffer that Zig coerces to one: a `[]u8`
//! (`buf[a:b]`) or a `*[N]u8` (`&buf`). Anything else stays refused.

use super::{is_agg, sm, LTy, Lower, Place, Val, R, STR_EQL};
use crate::compiler::{Node, NodeKind};
use crate::ir::*;

impl<'a> Lower<'a> {
    /// `std.mem.eql(u8, a, b)` as a value; None for any other call.
    pub(super) fn std_mem_call(&mut self, c: &Node) -> R<Option<Val>> {
        if c.name != "std.mem.eql" {
            return Ok(None);
        }
        let (a, b) = self.u8_slices(c)?;
        self.str_eq(false, a, b).map(Some)
    }

    /// A builtin of stdmem_plan.t27 (`std.mem.trim`, `std.mem.indexOfScalar`,
    /// `std.mem.indexOf`, `std.math.approxEqAbs`): a call of its routine from
    /// stdmem.t27, spliced under the reserved prefix, so the bytes are Zig
    /// 0.16's own std's (#8965). The plan weighs the arity and the element /
    /// type operand; the routine's own parameters coerce the value operands.
    pub(super) fn std_builtin_call(&mut self, n: &Node) -> R<Val> {
        self.see(n);
        let b = sm::BUILTINS.split(' ').position(|s| s == n.name).unwrap_or(0) as u8;
        let t0 = |c: &Node| if c.kind == NodeKind::ExprIdentifier { match c.name.as_str() { "u8" => sm::K_U8, "f64" => sm::K_TYPE_F64, "f32" => sm::K_TYPE_F32, _ => sm::K_OTHER } } else { sm::K_OTHER };
        let t = n.children.first().map_or(sm::K_OTHER, t0);
        let a = sm::plan(b, n.children.len(), t);
        if a != sm::CALL {
            return self.reject(sm::what(b), format!("`{}`: {}", n.name, sm::detail(b, a)));
        }
        // The call node under the routine's spliced name, the type operand dropped.
        let mut inner = n.clone();
        inner.name = sm::routine(b).to_string();
        inner.children.remove(0);
        match self.call(&inner, None)? {
            (call, Some(ty), temp) if is_agg(&ty) => Ok(Val::M(Place { addr: call, off: 0, ty, mutable: false, temp })),
            (call, Some(_), _) => Ok(Val::E(call)),
            _ => Err(()), // the routine returns a str, an optional or a bool by its own declaration
        }
    }

    /// The two `str` operands of `std.mem.<f>(u8, x, y)`.
    fn u8_slices(&mut self, c: &Node) -> R<(Val, Val)> {
        self.see(c);
        let a = &c.children;
        if a.len() != 3 || a[0].kind != NodeKind::ExprIdentifier || a[0].name != "u8" {
            return self.reject("ExprCall(std.*)", format!("call to `{}` other than (u8, str, str)", c.name));
        }
        let (x, y) = (self.expr(&a[1])?, self.expr(&a[2])?);
        let (x, y) = (self.bytes_as_str(x)?, self.bytes_as_str(y)?);
        for v in [&x, &y] {
            if !self.is_str(v) {
                let d = self.val_desc(v);
                return self.reject("ExprCall(std.*)", format!("`{}` on {}, not a string", c.name, d));
            }
        }
        Ok((x, y))
    }

    /// Bytes in a buffer read as a string, as Zig coerces them to
    /// `[]const u8`: a `[]u8` (`buf[a:b]` of a `var buf: [N]u8`) or a
    /// `*[N]u8` (`&buf`). Any other value is returned unchanged.
    fn bytes_as_str(&mut self, v: Val) -> R<Val> {
        let u8_ty = LTy::S(Ty::U8);
        let bytes = match &v {
            Val::M(p) => matches!(&p.ty, LTy::Slice(t, true) if **t == u8_ty),
            Val::P(_, LTy::Ptr(inner, _)) => matches!(&**inner, LTy::Arr(t, _) if **t == u8_ty),
            _ => false,
        };
        if bytes {
            self.coerce_to(v, &LTy::Str)
        } else {
            Ok(v)
        }
    }

    /// The fn lowering synthesizes, after the source fns (id `nfuncs`):
    /// `__t27b_str_eql` when either comparison uses it.
    pub(super) fn synthesized(&mut self, funcs: &mut Vec<Func>) {
        if self.eql_used {
            // Source fns are ids 0..nfuncs and come first in `funcs`, in order.
            let site = self.site(TrapKind::NoReturn, format!("end of fn {}", STR_EQL), Ty::Bool);
            funcs.insert(self.nfuncs as usize, super::str_eql_func(site));
            self.internal_abi.push(self.nfuncs);
        }
    }
}
