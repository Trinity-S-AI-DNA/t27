# Limitations

- Backend agreement is self-consistency behind one shared front end, not independent verification.
- Code quality is measured by executed guest instructions, emulated run time and code size, on 20 tier-1 specifications.
- All run times are qemu-aarch64 user-mode emulation on x86_64, not native arm64 speed.
- Compile latency was measured on a shared cloud host (load 27-103 on 48 threads); millisecond samples are noisy.
- The 4.8 ms latency is a geometric mean over 18 specifications, not over the corpus.
- Coverage is file-level success relative to the reference: 1,296 of 1,311 files. It is not full language coverage.
- 0 of 8,344 disagreements covers tests both pipelines ran. Files blocked in either pipeline are not compared.
- Tier 1 excludes every specification gen-c cannot compile.
- Clang -O2 folded the test at compile time on 11 of 20 specifications; its ratios use the remaining 9.
- At the measured revision `t27b build` wrote Mach-O objects only and did not link.
- Post-snapshot changes (ELF64 objects, `t27b run`, `t27b verilog`, the 2,991-spec corpus) were not re-measured.
- The measurement harness is not committed; the report lists its commands.
- t27b is not an optimising compiler and not a replacement for LLVM.
- No front end for C, Rust, Zig, Go or Python exists. Cross-language lowering is not claimed.
- t27b reads no serialised IR. The external `t27_ir_v1` contract has no t27b reader.
