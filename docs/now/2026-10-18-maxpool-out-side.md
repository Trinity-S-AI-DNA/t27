# NOW -- maxpool2d_layer.t27: a lowerable out_side(kernel, stride) for the nn-maxpool widget (2026-10-18)

## specs/ml/layers/maxpool2d_layer.t27 (Closes #10664)

- Why: gHashTag/trinity#1696 (course 10) was refused because the nn-maxpool widget hand-holds `K_OUT_SIDES = [2, 2, 3, 1]`; `out_size` takes a `MaxPool2DConfig` struct and is in `__NOT_LOWERED__`, so the widget had no compiled output side.
- `out_side(kernel, stride)` = `out_size(4, cfg(kernel, stride))` on two u32s; `scripts/t27-logic.mjs` now lowers 3 fns (`outSide`, `poolAt`, `backwardAt`), 4 not (`out_size`, `forward`, `backward`, `cfg`).
- Tests: the widget's four configs (2,2)->2, (3,1)->2, (2,1)->3, (4,1)->1; edges (1,1)->4, (3,2)->1, (1,3)->2 (floor division).
- t27c (bootstrap unchanged since 239b4b059), zig 0.16.0: test-report 7 of 7 pass, 0 vacuous, invariant proved; the compiled WASM returns the same seven values in node.
- Negative controls: 4 mutants of the fn body (side 5, stride forced to 1, stride dropped, ceiling division), 4 killed; the ceiling mutant is killed only by the edge test.
- Seal saved (`Maxpool2d.json`, `layers_Maxpool2d.json`).
- Not here: the re-vendor into trinity, `outSide(k, st)` in `nn-maxpool/tool.js`, dropping `K_OUT_SIDES` and its hand formula test.
