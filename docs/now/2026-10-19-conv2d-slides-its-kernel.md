# NOW -- conv2d_layer.t27 slides its kernel, plus scalar fns for the nn-conv widget (2026-10-19)

## specs/ml/layers/conv2d_layer.t27 (Closes #10612)

- Why: `forward` multiplied one kernel-sized patch elementwise by the weights and added the bias, so the 2x2 input with the 2x2 kernel gave four numbers where a convolution gives one (3.25), and its test checked the four. No fn lowered to WASM, so course 10 lesson 10 (trinity `widgets/nn-conv`) falls back to average pooling.
- `forward` is now the deep-learning convolution (2-D cross-correlation, kernel not flipped) over one H x W channel with stride and zero padding; output side `(n + 2p - k) / s + 1`; bias once per output cell. `backward` is the sliding gradient (grad_input, grad_weights; the bias gradient is its return value, which keeps it at 8 parameters and the site compiler warning-free). Both now take `height, width`.
- Scalar entry points: `out_size(n, kernel, stride, padding)`, `conv_at(kernel_id, stride, padding, index)` on the fixed 5x5 square image with 4 fixed 3x3 kernels (identity, Sobel x, Sobel y, box blur), `kernel_at`, `image_at`, and `default_at(index)` for the old 2x2 case. `scripts/t27-logic.mjs` lowers 5 fns (`outSize`, `convAt`, `kernelAt`, `imageAt`, `defaultAt`); in node `defaultAt(0)` is 3.25 and every row of the issue's table comes back exact.
- VERSION 2 with a dated header note and a test that VERSION is 2.
- t27c (bootstrap unchanged since 239b4b059), zig 0.16.0: test-report 14 of 14 pass, 0 vacuous, 5 invariants proved. Each row of the issue's table has its own test; a 3x3 input with the 2x2 kernel pins the slide (3.6, 5.45, 9.15, 11.0).
- Negative controls: 9 mutants (elementwise fallback, stride ignored, padding ignored, kernel flipped, bias k*k times, and in backward padding ignored, kernel flipped, has_bias ignored, stride ignored), 9 killed.
- Site compiler (trinity `public/t27/t27_compiler.wasm`): typecheck ok, 0 errors, 0 discarded, all seven backends print.
- `t27c dupes --bodies` names no group in this spec (master had one: three unused `default_*` helpers with one body; removed).
- Seal saved (`Conv2d.json`, `layers_Conv2d.json`). No spec `use`s Conv2d.
- Not here: re-vendoring into trinity and switching `nn-conv/tool.js` from `pool4At` to `convAt`.
