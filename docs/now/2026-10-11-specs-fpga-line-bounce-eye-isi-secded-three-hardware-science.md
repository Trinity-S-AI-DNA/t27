# NOW -- specs/fpga: line_bounce, eye_isi, secded -- three hardware-science widgets run from their own fn bodies (2026-10-11)

## specs/fpga: line_bounce, eye_isi, secded -- three hardware-science widgets run from their own fn bodies (Closes #8929)

- line_bounce: the lattice of a step down a lossless line; Johnson & Graham ch. 4 vectors 1.333, 0.889, 1.037, 0.988 V
- eye_isi: one RC pole per bit, eye 1 - 2a, shut at T = TAU ln 2; PRBS7 period 127
- secded: Hamming(12,8) + overall parity; every single flip corrected, every double caught
