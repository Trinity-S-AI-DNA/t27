# NOW -- wasm/doom: 23 of 23 named opcodes; the framebuffer link paints (2026-11-01)

## wasm/doom: 23 of 23 named opcodes; the framebuffer link paints

- specs/fpga/wasm_interp_doom.t27 gains call (0x10, return-frame call stack with a fn table) and f32/f64 load/store (0x2A/0x2B/0x38/0x39) as IEEE-754 bit-pattern movement through the integer carriers -- no float datapath claimed, only the bits move. 28/28 tests.
- specs/fpga/wasm_opcodes.t27's audit rises 18 -> 23 of the 23 named DOOM rows; the_doom_interpreter_covers_twentythree_of_twentythree pins it.
- The splice in bootstrap/src/use_resolve.rs carried fns but not the `pub var` state they read (FB, PAL, pc, err): decl_name now knows `var` (31/31 unit tests). wasm_doom_fb_link.t27 -- sealed blocked "does not compile" -- compiles and passes 2/2: a real wasm loop paints palette indices into interpreter memory and the presenter turns them into framebuffer pixels, verified per pixel with a black negative control.
- bootstrap/src/compiler.rs taken from master: this branch's committed copy had a mismatched delimiter and 35 type errors (half-finished merge). With master's copy t27b builds clean again; LANG-EN fixes in queen-brief-shape.t27 and the two docs files.
- Full wasm chain on this tree: interp_doom 28/28, fb_link 2/2, opcodes 18/18, interp 25/25, fb 8/8, module 21/21.
- Known, pre-existing, not touched here: wasm_board_ax7203.t27's duplicate `const OP_I64_ADD` (fixed on claude/wizardly-easley-fb851e, #12964) and wasm.t27's new param-count lints on this branch's t27b.
