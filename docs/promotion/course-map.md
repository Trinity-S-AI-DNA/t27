# Course map: 27 lessons

Part of #13473. Issue #13475. Status: DRAFT_UNSENT. Nothing here was posted.

## Source

Live lesson rows are `LESSON_IDS`, `LESSON_TITLES`, `LESSON_MODULES`, `LESSON_WIDGETS`, `LESSON_SPECS`, and `LESSON_GOALS` in `Trinity-S-AI-DNA/trinity` `apps/website/specs/course/course.t27`.

Fetched this run with `gh api repos/Trinity-S-AI-DNA/trinity/contents/apps/website/specs/course/course.t27`. Blob sha `3b3aa5d21a1dc86876c5b873bf6d6b8cf8d20b21`, size 28305. `LESSON_COUNT` is 27.

`specs/course/lesson_frames.t27` in this repo says `FRAME_NOW` is that `LESSON_SPECS` table as read on 2026-11-07, and `FRAME_NEXT` is the table a later course edit writes. `changed() == 8`. The column `frame_next` below is blank when the two paths are the same string.

Lesson body text was not copied. `specs/course/fpga_course_v2.t27` weak point W1 records that lesson 1 and lesson 3 still frame boards that are not the bench board `specs/boards/ax7203.t27`.

This repo's lesson 1 title in `specs/course/lesson_01_fpga_intro.t27` is `Lesson 1: What is an FPGA?`. That string is not the trinity lesson 1 title below.

## Asset fields, same for every lesson

| Field | Value |
| --- | --- |
| claim | N/A -- a lesson title is not one of the nine approved sentences in #13473 |
| visual proof | the widget id; screenshot N/A, none captured in this run |
| CTA | N/A -- not sent |
| metric | not measured |

`gallery.t27` is not in this checkout (`test -f specs/widgets/gallery.t27` exit 1). The course spec says every widget id is a gallery id. This map cannot check that membership from this tree.

## Lessons

| n | id | title | module | widget | spec now | frame_next | result named by LESSON_GOALS |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | fpga-is-a-grid | An FPGA is a grid you configure | chip | fasm-skyline | specs/boards/wukong_v1.t27 | specs/boards/ax7203.t27 | What an FPGA is made of, and why a design uses only part of the chip. |
| 2 | which-chip | Which chip is this? | chip | idcode | specs/tools/tri/fpga.t27 | | How a chip names itself in 32 bits, and how to read those bits. |
| 3 | pins-and-package | Pins, balls and the package | chip | pin-map | specs/boards/xc7a100t_minimal.t27 | specs/boards/ax7203.t27 | How the signals of your design reach the outside world. |
| 4 | bits-and-trits | Bits, trits and GF16 | numbers | gf16-calc | specs/tutorial/01_values_and_types.t27 | specs/ternary/packed_trit.t27 | How a number is stored in bits, and how balanced ternary stores it in trits. |
| 5 | what-arithmetic-costs | What arithmetic costs | numbers | cost-curves | specs/tutorial/05_widths_and_casts.t27 | specs/ternary/hybrid_arithmetic.t27 | Why a multiplier is expensive and an adder is cheap, measured on a real chip. |
| 6 | how-much-fits | How much fits on the chip | numbers | fits-on | specs/fpga/stdlib.t27 | | How to read a resource count and find which resource runs out first. |
| 7 | hello-t27 | Hello, t27 | program | play-hello-world | specs/tutorial/02_functions.t27 | | What a t27 spec looks like, and what the compiler does with it in your browser. |
| 8 | tests-are-the-spec | Tests are part of the spec | program | test-waves | specs/tutorial/07_tests_invariants_benches.t27 | | Why every t27 spec carries its own tests, and what a waveform shows. |
| 9 | compiles-is-not-correct | Compiles is not the same as correct | program | t27-tri-misread | specs/tutorial/03_operators.t27 | | Why green checks are not proof, and how to catch code that compiles and is wrong. |
| 10 | compiler-stages | The stages of t27c | compiler | t27c-stages | specs/tutorial/06_structs_enums_switch.t27 | | The path a spec takes inside t27c: tokens, syntax tree, types, HIR, code. |
| 11 | t27c-on-a-real-machine | t27c on a real machine | compiler | t27c-hello-world | specs/demos/hello_world.t27 | | The same compiler on a real machine, printing what it really prints. |
| 12 | seven-backends-and-t27b | Seven backends, and t27b | compiler | play-rust | specs/tutorial/08_modules_and_arrays.t27 | | One spec, many target languages, and a native compiler with no LLVM. |
| 13 | spec-to-verilog | From a spec to Verilog | hardware | play-verilog | specs/ternary/clocked_counter.t27 | | How a t27 function becomes logic, and how a clocked function becomes registers. |
| 14 | count-the-cells | Count the cells | hardware | stat-card | specs/tutorial/04_control_flow.t27 | specs/ternary/ternary_full_adder.t27 | How to count what a synthesizer built: LUTs, flip-flops, DSPs, block RAM. |
| 15 | one-lut-up-close | One LUT, up close | hardware | lut-explain | specs/ternary/ternary_full_adder.t27 | | What a 6-input lookup table is, and how 64 bits describe any gate. |
| 16 | synthesis-warnings | Synthesis warnings that matter | synthesis | synth-smells | specs/fpga/crossopt.t27 | | How to read a yosys log, and the warnings that become bugs on silicon. |
| 17 | state-machines | State machines | synthesis | fsm-sketch | specs/fpga/dft.t27 | specs/fpga/uart.t27 | How a synthesizer finds a state machine in your design, or refuses to. |
| 18 | where-the-luts-go | Where the LUTs go | synthesis | lut-treemap | specs/fpga/partition.t27 | | Which part of a design spends the LUTs, and why hierarchy matters. |
| 19 | placement | Placement: where the cells land | timing | placement-map | specs/fpga/placement.t27 | | What place and route does, and where it put a real design. |
| 20 | logic-depth | Logic depth | timing | lut-depth | specs/fpga/timing.t27 | | Why the longest chain of logic sets how fast a design can run. |
| 21 | timing-slack | Timing and slack | timing | slack-waterfall | specs/fpga/cts.t27 | specs/fpga/timing.t27 | What slack is, and how to read a timing report path by path. |
| 22 | fasm-to-frames | From FASM to frames | bitstream | tri-x7-fasm | specs/tools/trios/tri/x7-fasm.t27 | | What FASM is, and how features become configuration frames. |
| 23 | a-build-receipt | A build, itemised | bitstream | build-receipt | specs/fpga/adapter.t27 | specs/fpga/board_receipt.t27 | Every stage from a spec to a bitstream, with its time, tool and hash. |
| 24 | read-a-bitstream | Read a bitstream | bitstream | bit-autopsy | specs/tools/trios/tri/x7-writer.t27 | | What is inside a .bit file, and which chip it was built for. |
| 25 | jtag-asks-the-chip | JTAG asks the chip | board | tri-fpga-jtag | specs/tools/trios/tri/fpga-jtag.t27 | | How a cable finds the chip on a board and reads its IDCODE. |
| 26 | same-bits-on-the-board | The same bits, on the board | board | x7-board | specs/tools/trios/tri/x7-board.t27 | | How to show that a bitstream is byte-identical and that it runs. |
| 27 | capstone | Capstone: out-guess yosys | board | gatle | specs/fpga/e2e_demo.t27 | | Put it together: guess what a real t27 module costs in LUTs. |

Lesson 5's goal says "measured on a real chip" and does not give the number in `LESSON_GOALS`. The number is not measured in this run.

Nine module titles, from `MODULE_TITLES` in the same blob: The chip; Numbers in hardware; Your t27 program; Inside t27c; From spec to hardware; Reading synthesis; Place, route, timing; The bitstream; On the board.
