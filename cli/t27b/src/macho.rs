//! The object file `t27b build` writes: a Mach-O MH_OBJECT for arm64 (specs/tri/t27b/macho.t27) or an
//! ELF64 ET_REL for aarch64 Linux (specs/tri/t27b/elf.t27, #8947). Each plan decides the layout, the
//! symbols, the string table, the relocations and every byte; this file flattens a linked image into
//! the tables they read ("the image both object writers read" in macho.t27) and sizes the buffer.

use crate::codegen::Linked;

#[path = "../../../gen/rust/tri/t27b/macho.rs"]
#[allow(dead_code, unused_parens, unused_mut, unused_assignments, unused_variables, non_snake_case, non_upper_case_globals, clippy::all)]
mod spec; // t27c gen-rust of specs/tri/t27b/macho.t27

#[path = "../../../gen/rust/tri/t27b/elf.rs"]
#[allow(dead_code, unused_parens, unused_mut, unused_assignments, unused_variables, non_snake_case, non_upper_case_globals, clippy::all)]
mod elf; // t27c gen-rust of specs/tri/t27b/elf.t27

/// Each function's symbol ("" for none), and a note for each one refused: elf.t27's `export_kind` (#8948).
pub fn symbols(prog: &crate::ir::Program) -> (Vec<String>, Vec<String>) {
    let k = |i: usize| elf::export_kind(prog.funcs[i].name.as_bytes(), prog.internal_abi.contains(&(i as u32)));
    let notes = (0..prog.funcs.len()).filter(|&i| k(i) == elf::REFUSED).map(|i| format!("`{}` {}", prog.funcs[i].name, elf::REFUSED_NOTE)).collect();
    ((0..prog.funcs.len()).map(|i| if k(i) == elf::EXPORT { prog.funcs[i].name.clone() } else { String::new() }).collect(), notes)
}

/// Serialise a linked image (no prefix) with one global symbol per function.
/// `names[i]` is the t27 name of function i; the symbol is `_` + name. An
/// empty name exports nothing (a function only other functions call).
/// `data` is `Program::data`, referenced by `linked.data_refs`.
pub fn object(linked: &Linked, names: &[String], data: &[Vec<u8>]) -> Vec<u8> {
    write(linked, names, data, &[], false)
}

/// The same image as an ELF object: the symbol is the bare name, and `globals` (`Program::globals`) are in .data.
pub fn elf_object(linked: &Linked, names: &[String], data: &[Vec<u8>], globals: &[Vec<u8>]) -> Vec<u8> {
    write(linked, names, data, globals, true)
}

fn write(linked: &Linked, names: &[String], data: &[Vec<u8>], globals: &[Vec<u8>], as_elf: bool) -> Vec<u8> {
    let le = |v: Vec<u64>| -> Vec<u8> { v.iter().flat_map(|x| x.to_le_bytes()).collect() };
    let text: Vec<u8> = linked.code.iter().flat_map(|w| w.to_le_bytes()).collect();
    let nm: Vec<u8> = names.iter().flat_map(|n| n.bytes().chain([0])).collect();
    let fns = le(linked.offsets.iter().zip(&linked.sizes).flat_map(|(o, &s)| [o.map_or(0, |o| o as u64 + 1), s as u64]).collect());
    let refs = le(linked.data_refs.iter().flat_map(|&(p, k)| [p as u64, k as u64]).collect());
    let lens = le(data.iter().chain(globals).map(|b| b.len() as u64).collect());
    let blobs: Vec<u8> = data.iter().chain(globals).flatten().copied().collect();
    let n = data.len();
    if as_elf {
        let mut w = vec![0u8; elf::elf_size(&nm, &fns, &refs, &lens, text.len(), n)];
        elf::write_elf(&mut w, &text, &nm, &fns, &refs, &blobs, &lens, n);
        return w;
    }
    let mut w = vec![0u8; spec::object_len(&nm, &fns, &refs, &lens, text.len(), n)];
    spec::write_object(&mut w, &text, &nm, &fns, &refs, &blobs, &lens, n);
    w
}
