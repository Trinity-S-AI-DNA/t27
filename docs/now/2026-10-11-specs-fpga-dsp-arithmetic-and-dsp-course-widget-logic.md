# NOW -- specs/fpga/dsp: the logic of the arithmetic-and-dsp course widgets (2026-10-11)

## specs/fpga/dsp: adders, multipliers, fixed_point, cordic, signals -- 26 t27.ai widgets run from their own fn bodies (Closes #8985)

- adders: carry chain, four prefix networks replayed on spans (every carry computed, n = 2..32), Dadda = Wallace levels to k = 300
- multipliers: shift and add, radix-4 Booth exact for all 8-bit pairs, DSP48E1 25 x 18 tiling and 48-bit headroom
- fixed_point: Qm.n, truncate / round half up / convergent, wrap or saturate, the ramp bias of each
- cordic: rotation and vectoring step by step, K(n), a sine table nearest vs interpolated, its reference the same CORDIC
- signals: SQNR, aliasing, NCO, DFT leakage, windowed-sinc FIR, CIC, polyphase, an integer FFT, the capstone RTL FIR; sine and dB written once (DRY, #8963)
- 43 tests, none vacuous; 33 of 33 negative controls caught
