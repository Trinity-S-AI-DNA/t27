//! t27b: t27's own native backend (MVP).
//!
//! Pipeline: .t27 source -> t27c front-end (lexer, parser, typecheck_ast; the
//! frozen bootstrap/src/compiler.rs, mounted unmodified) -> AST ->
//! t27b IR (lower) -> AArch64 instruction selection and encoding (codegen, a64)
//! -> either an in-process JIT (jit) or a Mach-O MH_OBJECT file (macho).
//!
//! No LLVM, no zig, no clang, no rustc at run time; the only crate dependency is
//! serde, which compiler.rs needs for its derives.

// The frozen front-end. bootstrap/build.rs pins this file against
// bootstrap/stage0/FROZEN_HASH; it is included here byte-for-byte, never edited.
#[path = "../../../bootstrap/src/compiler.rs"]
#[allow(dead_code, unused, clippy::all)]
pub mod compiler;

// compiler.rs refers to crate::use_resolve (imported enums/structs for the
// Verilog path); the real module is mounted so the reference resolves and so
// `use` splicing behaves exactly as in t27c.
#[path = "../../../bootstrap/src/use_resolve.rs"]
#[allow(dead_code, unused, clippy::all)]
pub mod use_resolve;

pub mod a64;
pub mod blockers;
pub mod codegen;
pub mod eval;
pub mod front;
pub mod ir;
pub mod jit;
pub mod macho;
pub mod lower;
pub mod timing;

/// The wasm32 byte law (specs/tri/t27b/wasm.t27, floors 1-10): t27c gen-rust
/// output, mounted like check_budget in main.rs. Every emit decision is the
/// law's; #11840, port epic #6198.
#[path = "../../../gen/rust/tri/t27b/wasm.rs"]
#[allow(dead_code)]
mod wasm_law;

/// The wasm32 emit adapter (the emitter file deleted, owner rule 2026-10-24,
/// PR thread #11983): the IR half of the flatten is ir::wasm_facts; this
/// takes walk_refusal's verdict and composes the module through walk_prog +
/// compose_module. No emitter logic lives here.
pub mod wasm32_emitter {
    use super::ir::{Program, Ty};
    use super::wasm_law as w;

    /// Why a pub fn (or the module) is not emitable, shaped like
    /// codegen::CodegenError so the CLI prints it and `corpus` parses the
    /// construct out of it; message() is the law's refusal_message.
    #[derive(Debug)]
    pub struct EmitError {
        pub construct: String,
        pub func: String,
        pub line: u32,
        pub detail: String,
    }

    impl EmitError {
        pub fn message(&self) -> String {
            let mut b = [0u8; 512];
            let n = w::refusal_message(
                &mut b, 0, self.construct.as_bytes(), self.line,
                self.func.as_bytes(), self.detail.as_bytes(),
            );
            String::from_utf8_lossy(&b[..n]).into_owned()
        }
    }

    /// True when the wasm32 path carries this scalar (`t27b run`'s contract,
    /// #11824: the native value and the wasm value stay comparable).
    pub fn carries(ty: Ty) -> bool {
        w::carries(ty.name().as_bytes())
    }

    /// The EmitError of a refused walk: the verdict's row mapped back to its
    /// fn name and line, the law's construct and detail (the typed details
    /// read the refusing row's own type name or op byte).
    fn refusal_error(p: &Program, facts: &[u8], s: &[u8], ns: &[u8], verdict: u8) -> EmitError {
        let row = w::walk_refusal_row(facts, s, ns);
        let (func, line) = if row == w::fact_rows(facts) {
            (w::module_func().to_string(), w::module_line())
        } else {
            let (a, b) = (
                w::fact_name_off(facts, row) as usize,
                w::fact_name_off(facts, row) as usize + w::fact_name_len(facts, row) as usize,
            );
            (String::from_utf8_lossy(&s[a..b]).into_owned(), w::fact_line(facts, row))
        };
        // The typed details need the refusing row's own data: the type
        // name (first non-i32 parameter, or the return), the op byte.
        let mut ty: &[u8] = b"";
        let mut op = [0u8; 1];
        if let Some(fr) = p.funcs.get(row) {
            if verdict == w::REF_FN_PARAMETERS {
                ty = fr.vars[..fr.nparams]
                    .iter()
                    .find(|v| w::carrier(v.ty.name().as_bytes()) != w::VT_I32)
                    .map(|v| v.ty.name().as_bytes())
                    .unwrap_or(b"");
            } else if matches!(verdict, w::REF_PTR_RETURN | w::REF_I128_RETURN | w::REF_FN_RETURN_TYPE) {
                if let Some(t) = fr.ret {
                    ty = t.name().as_bytes();
                }
            } else if verdict == w::REF_ARITHMETIC_OP {
                w::postfix_refusal(
                    ns, w::fact_payload_off(facts, row), w::fact_payload_len(facts, row),
                    w::fact_nparams(facts, row), &mut op,
                );
            }
        }
        let mut d = [0u8; 192];
        let dn = w::refusal_detail_for(&mut d, 0, verdict, ty, op[0]);
        EmitError {
            construct: w::refusal_construct(verdict).to_string(),
            func,
            line,
            detail: String::from_utf8_lossy(&d[..dn]).into_owned(),
        }
    }

    /// The module with an optional data image mounted as linear memory
    /// (floor 12's emit_with_image contract): every pub fn in a supported
    /// shape exported under its t27 name, the image as one memory
    /// (min = max = pages_for) and one active data segment at offset 0.
    /// An empty image is the scalar cut, byte for byte.
    pub fn emit_with_image(p: &Program, image: &[u8]) -> Result<Vec<u8>, EmitError> {
        let (facts, s, ns) = super::ir::wasm_facts(p);
        let verdict = w::walk_refusal(&facts, &s, &ns);
        if verdict != w::REF_OK {
            return Err(refusal_error(p, &facts, &s, &ns, verdict));
        }
        let (k, bcap) = (w::fact_rows(&facts), w::bodies_cap(&facts, &s, &ns));
        let cap = w::scratch_cap(s.len(), bcap, k) + 64 + image.len() + 24;
        let (mut names, mut descs, mut bodies) =
            (vec![0u8; s.len()], vec![0u8; k * w::DESC_ROW], vec![0u8; bcap]);
        let (mut out, mut scratch) = (vec![0u8; cap], vec![0u8; cap]);
        let dn = w::walk_prog(&mut names, &mut descs, &mut bodies, &facts, &s, &ns);
        let used = &descs[..dn];
        let n = w::compose_module_with_image(
            &mut out,
            &names[..w::walk_names_len(used)],
            used,
            &bodies[..w::walk_bodies_len(used)],
            image,
            &mut scratch,
        );
        out.truncate(n);
        Ok(out)
    }

    /// The module: every pub fn in a supported shape exported under its t27
    /// name; anything else is refused by name, never silently dropped.
    pub fn emit(p: &Program) -> Result<Vec<u8>, EmitError> {
        emit_with_image(p, &[])
    }
}
