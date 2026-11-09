# NOW -- wasm/doom browser gate: 100/100 in Node V8, live run in Safari (2026-11-01)

## wasm/doom browser gate: 100/100 in Node V8, live run in Safari

Branch `claude/doom-wasm-chain` (off origin/master, cherry-pick of the doom commit).

- `cargo test --release -p t27b --test wasm32_equality`: `wasm32_equality_battery_100_of_100 ... ok` -- Node's WebAssembly instantiates all 100 emitted modules (Node v26.13.0).
- `cargo test --release -p t27b --test wasm32`: 12/12.
- Live browser run: `t27b build demo.t27 -o demo.wasm --target wasm32` (64 bytes, exports `the_answer` and `add`), served on 127.0.0.1:8137, opened in real Safari 26.5.2 (JavaScriptCore -- an engine independent of V8): the_answer()=42, add(20,22)=42, add(-5,7)=2, ok=true, reported back to the local server by the page itself. Scratch server and files removed afterwards.
- Seal re-minted by a master-built t27c (zig 0.16.0 on PATH): fpga_WasmInterpDoom 28/28.
- On this branch `git diff origin/master -- bootstrap/src/compiler.rs` is 0 lines: the broken branch copy never rides here.
- Chain totals on this tree: interp_doom 28/28, opcodes 18/18, interp 25/25, fb 8/8, module 21/21, wasm32_equality 100/100, wasm32 12/12.

Claim level: DOOM WASM chain complete; browser equality gate and live browser test passed on this host (macOS, Safari 26.5.2, Node 26.13.0). Not claimed: DOOM the game, FPGA synthesis, float arithmetic.
