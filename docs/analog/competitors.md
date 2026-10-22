# t27 analog emulation v0.1 - competitors, prior art, weaknesses

Date of research: 2026-10-18. Method: web search plus page/PDF reads. English, ASCII only.
Section 11 (added 2026-10-19) revisits the weaknesses of section 7 against the measurements of M0 to M5 and adds a competitor delta; sections 0 to 8 are left as written on 2026-10-18.

## 0. How to read this file

Every factual claim carries a tag:

- [V] verified: I read the page text or PDF text myself (WebFetch or pdftotext) and the statement is in it.
- [S] search-only: the claim appears only in a web-search result summary; I did not confirm it on the page. Treat as a lead.
- [U] unknown: I looked and could not find it. "Not published" does not mean "does not exist".
- [I] inference or proposal by me, not stated by any source.

Source keys appear as (src:key). The full URL for each key is in section 9.

What the t27 side is (from the task brief): analog netlist (SPICE + IHP SG13G2 PDK, PSP103) becomes one .t27 spec that is both a fixed-point reference and the FPGA design. Linear part: MNA + backward Euler, fixed step, folded at compile time to x' = Mx + Nu. Nonlinear devices: per-corner I-V tables from ngspice, interpolated on the FPGA, one bitstream per corner. First result: RC low-pass bit-exact, <= 2.65 LSB from float64, 520 LUT / 0 DSP, 10 us of circuit time in 12 us wall time.

Headline result of the research (details below):

1. The t27 numerical core (MNA, fixed step, backward Euler, constant matrix, switches/devices as table or switch models) is the same family that OPAL-RT eHS, Simscape HDL, Typhoon, Plexim and a 2013 EPFL simulator already use. It is not novel by itself.
2. The genuinely different parts, as far as I could find, are: PDK-level transistor models (PSP103 tables, IHP SG13G2) feeding an FPGA fixed-step solver; fixed-point with a bit-exactness claim; spec-first single source (.t27) for both reference and hardware; ngspice and uberWAVE as the checking chain. I found no source that does all of these together. This is absence of evidence from a bounded search, not proof of novelty.
3. The closest neighbours are two open projects: Stanford msdsl/svreal/anasymod (FPGA mixed-signal emulation, netlist interface, fixed-point, published results) and cpphdl "analog" (C++ MNA + Newton + backward Euler, ngspice cross-check, IHP SG13G2 mentioned, no FPGA claim). Both must be named in any t27 positioning.

---

## 1. Overview table

| Competitor | Domain | Solver / numbers | Step | Nonlinear devices | Open? | t27 delta |
|---|---|---|---|---|---|---|
| Typhoon HIL (HIL404/606) | power electronics HIL | proprietary FPGA multi-core "HILCore"/SPC; format not published [U] | down to 25 ns, typical 250 ns [V] | LUT unit (PV, battery, saturable transformer), ideal and non-ideal switches [V] | closed | t27 wins on PDK transistors; loses on step, I/O, scale, ecosystem |
| OPAL-RT eHS | power electronics HIL | Gen4 MNA constant conductance matrix + Pejovic switch; Gen5 state-space; "floating-point solver" [V] | 70 ns - 4 us (Gen5, perf. license), 200 ns - 4 us (Gen4) [V] | saturable inductor, IGBT/diode/FET switch models, 64 non-linear/variable comps on x128 [V] | closed | same math family; t27 wins on device level; loses on everything operational |
| Plexim RT Box | power electronics HIL | FlexArray and Nanostep FPGA solvers; format not stated [V] | FlexArray "below 100 ns"; Nanostep 4 ns / 7.5 ns [V] | all switches become non-ideal switch model [V] | closed | general netlist vs power-module circuits; loses on speed |
| dSPACE XSG Electric Components | e-drive HIL | XSG library, "open architecture" [V] | "smaller than 200 ns" possible; pin-to-pin ~1 us [V] | PMSM saturation by 2D LUT; run-time changeable LUT data [V] | closed (open equations) | t27 wins: arbitrary netlist; loses: LUT reload without re-synthesis |
| RTDS | power systems | FPGA/processor hybrid (details [U]) | 10 us large step, 1.4 us small step [V] | n/a | closed | different domain |
| NI | platform only | sells OPAL-RT eHS add-on for LabVIEW/PXI/cRIO [V] | via eHS | via eHS | closed | n/a |
| MathWorks Simscape HDL | model-to-HDL | Backward Euler fixed Ts for linear/switched-linear; Partitioning solver for nonlinear [V]; single precision [V] | example 2 us on Kintex-7 [V] | switches as switched-linear; nonlinear via Partitioning [V] | closed | same numerics; t27 adds fixed-point + PDK devices |
| Stanford msdsl/svreal/anasymod | AMS chip emulation | switched LTI via matrix exponential, fixed-point auto-range [V] | 30-35 MHz emulator clock [V] | on/off switch models; memoryless LUTs [V] | open | closest open prior art |
| Cadence Palladium Z2 RNM app | AMS SoC verification | real-number-model emulation [V] | not published | behavioral RNM | closed | different layer (behavioral) |
| SPICE-on-FPGA (Kapre, DeHon) | general SPICE acceleration | double-precision float, adaptive SPICE [S] | adaptive | full SPICE models | papers | t27 is fixed-step emulation, not SPICE |
| cpphdl analog | C++ analog+digital HDL | MNA + damped Newton + fixed-step backward Euler [V] | user h | square-law MOS in tests [V] | open (Apache-2.0) | same numerics, no FPGA claim seen |
| ngspice / Xyce / OpenVAF | reference simulators | adaptive step, full PDK models | adaptive | full | open | t27 reference, not competitor |

---

## 2. Commercial FPGA real-time circuit solvers

### 2.1 Typhoon HIL (HIL404, HIL604, HIL606)

- What it does: controller hardware-in-the-loop for power electronics. The electrical model runs on a "specialized, proprietary FPGA-based multi-core processor" called HILCore; it "executes models at dedicated, nanosecond scale simulation steps" [V] (src:typhoon-arch). Sub-units named in the manual: SPC ("a flexible and programmable circuit solver capable of sub-microsecond time steps", ideal and non-ideal switch models), UltraCore (time step equal to the HIL clock period, down to 3.5 ns, suitable for switching frequencies up to 1 MHz), MachineCore (one electrical machine incl. mechanics), and a LUT unit "used to simulate behavior of nonlinear elements" such as PV panels, batteries, nonlinear passive components, saturable transformers [V] (src:typhoon-fpga-basics).
- Published numbers: "Down to 25ns simulation time step (typically 250ns), 3.5ns digital oversampling"; HIL404 has 4 FPGA processing cores, HIL606 has 8 [V] (src:typhoon-emr25). A brochure snippet says HIL404 has a 200 ns time step and 200 ns analog output update [S] (src:typhoon-hil404-brochure, src:typhoon-products). HIL606 "as fast as 200 ns", up to 16 units paralleled [S] (src:typhoon-hil606).
- Fixed step: Typhoon support FAQ says the step is fixed, cannot be changed dynamically, and all computation must finish before the next step [S] (src:typhoon-faq).
- Nonlinear inductor: implemented as a controlled current source i(flux), defined by the user and turned into a lookup table [S] (src:typhoon-nonlinear).
- How the model gets in: graphical Schematic Editor (details of file format and compile flow not found) [U].
- Numeric format of the solver: not published in anything I read [U]. A search result about an academic paper says fixed-point suits small-step real-time sims while floating-point is the common choice, but this is general context, not Typhoon's statement [S].
- Pricing: no public price; website directs to "Request a Demo" [S] (src:typhoon-blog-hil606).
- Open/closed: closed hardware and software.
- Does NOT (as far as published): transistor-level compact models from a foundry PDK; arbitrary SPICE netlists.
- How t27 differs or loses: t27 solves device-level (PSP103-derived) circuits from a netlist and is open and spec-first, but Typhoon already has 25-250 ns steps, 3.5 ns I/O, validated converter libraries and a product; t27 has none of that.

### 2.2 OPAL-RT eHS (FPGA electrical hardware solver)

- What it does: FPGA solver for power converters inside OPAL-RT simulators (RT-LAB, HYPERSIM) and, as an add-on, NI platforms [V] (src:opalrt-blog, src:ni-ehs).
- Method: Gen4 uses modified nodal analysis with a constant conductance matrix; switches use the Pejovic method (conducting switch = inductance, open switch = capacitance) so the matrix never changes [V] (src:opalrt-blog, src:opalrt-main-features). Gen5 "uses a new circuit analysis method based on State Space equations", with "Time Stamped Bridge technology" for switching events, resolving events to "625 ps of resolution" [V] (src:opalrt-main-features). Note the blog still labels it a "fixed matrix solver" and "a powerful floating-point solver" [V] (src:opalrt-blog); the documentation page does not state the number format [V]. A compute-power line of 102.4 GFLOPS for x128 appears on the documentation page, while the NI page says 51.2 GFLOPS and "two eHS cores" for the same product name; the sources disagree [V].
- Numbers: blog "sample time below 1 us"; "up to about 125 states in 1 us" for eHS x128 [V]. Documentation: Gen5 70 ns to 4 us, Gen4 200 ns to 4 us; footnote gives a solver limit of 4.8 us, which contradicts the 4 us upper bound on the same page [V]. License classes x128/x64/x32/x16: 144/72/48/12 total switches; 64/32/16/8 non-linear/variable components; max about 256 states "estimated" [V]. NI page: 144 switches, max 344 states, 128 I/O [V] (conflicts with 256 on the OPAL-RT page; sources disagree).
- Components: resistor, inductor, capacitor, ideal transformer, mutual inductance, RLC, PI line, surge arrester, saturable inductor; switches: IGBT/diode, diode, breaker, thyristor, ideal switch, FET, half-bridges, NPC [V]. Diodes and FETs appear under the switch types; no compact transistor model is listed on the page [V].
- Netlist in: OPAL-RT Schematic Editor; SimPowerSystems, PLECS blockset, NI VeriStand; PSIM and NI Multisim import; PSIM netlist files in RT-LAB [V] (src:opalrt-main-features). Older NI forum thread says a Multisim netlist export needs "special components" [S] (src:ni-forum).
- Pricing: license by class; no public price. The NI product page shows "Loading Pricing" and asks for a quote [V] (src:ni-ehs). One search snippet cited $14,400 for eHSx128 on NI; I could not confirm it on the page, treat as unverified [S].
- Open/closed: closed.
- Does NOT publish: arithmetic format in the docs I read; accuracy versus a reference simulator.
- How t27 differs or loses: same numerical family (MNA/state-space, fixed step, constant matrix), so the linear core is not a differentiator. t27 wins on device level and openness; loses on step (70 ns), scale (144 switches), I/O, imports from five tools.

### 2.3 Plexim RT Box (PLECS)

- What it does: PLECS models run on the RT Box; the electrical part can run on the FPGA with two solvers [V] (src:plexim-fpga).
- FlexArray: step "depends on model complexity and can be below 100 ns"; only the electrical domain runs on the FPGA, control always on the CPU; supports circuits built from power modules plus diode, thyristor, triac, switch, double/triple switch, breaker; "All switches automatically use a non-ideal switch model"; analog outputs update after each step but no faster than every 200 ns [V].
- Nanostep: 4 ns (RT Box 2, 3, 4) or 7.5 ns (RT Box 1, CE); only time-critical power modules with a weight limit of 6 (RT Box 2-4) or 3 (RT Box 1, CE) [V].
- Number format and FPGA device: not stated on that page [V]. Pricing: not found [U].
- Open/closed: closed.
- Does NOT: general analog netlists; transistor models; "code generation supports only power-module-based circuits" [V].
- How t27 differs or loses: t27 accepts general netlists, not templates; it loses badly on step size and I/O (200 ns analog-out floor still beats a UART).

### 2.4 dSPACE (SCALEXIO, XSG Electric Components Model)

- What it does: a Xilinx System Generator (XSG) library with "open models for 3-phase electric machines, power electronics, position sensors, and basic mechanics" for FPGA simulation [V] (src:dspace-ecm).
- Numbers: "Model execution step sizes smaller than 200ns are possible"; "pin-to-pin loop time of approximately 1us" [V]. A patent names DS5203 / DS6602 boards and XSG [S] (src:dspace-patent).
- Nonlinear: PMSM saturation by "inductances and flux values derived from 2D look-up tables"; "Parameters and look-up table data can be changed at run time without re-synthesizing the FPGA" [V].
- Netlist: none; parameterized library blocks, not a netlist solver.
- Pricing: not public [S].
- How t27 differs or loses: t27 solves arbitrary circuits; dSPACE shows the industry norm that table data is reloadable at run time, whereas t27 currently needs one bitstream per corner.

### 2.5 RTDS

- A 2020 paper states: "In the large-time step environment, the minimum time step allowed is 10 microseconds, while in the small-time step is 1.4 microsecond"; small-step costs more hardware and cannot include frequency-dependent transmission lines [V] (src:rtds-paper). Patent text says RTDS generally works with steps of about 50 us [S] (src:rtds-patent).
- The same paper reports that the RTDS/RSCAD VSC model shows "issues on stability, harmonic spectrum, ripple, and power quality" versus PSCAD [V]. This is a power-systems tool, not a circuit-level emulator; pricing not found [U].
- How t27 differs or loses: different domain; the paper is a useful reminder that fixed-step real-time models are compared against offline references and found different.

### 2.6 NI

- NI does not appear to ship its own FPGA circuit solver; the route is the OPAL-RT eHS add-on (LabVIEW, OP4510/OP5707, PXI/cRIO) [V] (src:ni-ehs). StarSim FPGA Express on CompactRIO is another vendor path [S] (src:ni-forum-search).
- Academic data point: a DFIG HIL emulator in LabVIEW with fixed-point VHDL on a PXIe-7858R, 5.0 us per iteration, compared against a Typhoon HIL602+ [S] (src:aau-dfig).
- How t27 differs or loses: nothing to compete with on NI itself; it is a channel for eHS.

---

## 3. Model-to-HDL flows

### 3.1 MathWorks Simscape HDL Workflow Advisor (HDL Coder + Simscape + Simulink Real-Time)

- What it does: `sschdladvisor` converts a Simscape plant model into a Simulink implementation model; HDL Coder generates HDL; Simulink Real-Time deploys to the FPGA [V] (src:mw-hdl-codegen). Target: "generic FPGAs, SoCs, or Speedgoat FPGA I/O modules" [V] (src:mw-hil-start).
- Numerics: Solver Configuration block, "Use local solver", Backward Euler, discrete sample time Ts; Simulink fixed-step discrete [V] (src:mw-linearize). "HDL code generation requires single-precision data" [V] (src:mw-advisor). Stiffness: "If your model is too stiff, you may encounter a validation mismatch" [V] (src:mw-advisor).
- Nonlinear: Backward Euler for linear and switched-linear blocks only; nonlinear blocks (nonlinear resistor, nonlinear inductor, DC motor, PMSM) need the Partitioning solver, which splits the equations into "several smaller sets of switched linear equations" linked by nonlinear functions; the PMSM example uses three partitions, one Forward Euler and two Backward Euler [V] (src:mw-hil-start, src:mw-partitioning). Switches can be replaced by "dynamic switch approximation" (Pejovic and Maksimovic method) [V] (src:mw-dynswitch).
- Numbers: the dynamic-switch example runs "at a time step of 2 us" with a 2 kHz switching frequency on a Kintex-7 xc7k325t [V]. No accuracy metric is given ("similar to the baseline results", tolerance via Absolute Tolerance) [V]. A search summary mentions a 1 us or greater sample time for linear models; I could not confirm that on the pages I read [S].
- Restrictions: no events, delays, run-time parameters, Multibody, time-varying sources like PS Random Number, nonlinear partitions [V] (src:mw-hil-start).
- Pricing: product licenses, no public price [U]. Closed.
- Does NOT: documented fixed-point flow in the pages I read; PDK compact models (a Simscape block library is used instead) [U for PDK].
- How t27 differs or loses: this is the closest commercial analogue of "fold to x' = Mx + Nu with backward Euler". The 2021 Stanford dissertation likewise describes Simscape Electrical HDL as linear/switched-linear only with fixed-step Backward Euler (src:herbst-diss, Section 2.3) [V]; that predates, or does not mention, the Partitioning solver now documented by MathWorks. t27 adds fixed-point, PSP103-based tables and an open spec; it loses on Partitioning-solver generality, library breadth, and bundled validation against a baseline.

---

## 4. Analog and mixed-signal emulation for chip verification, and academic FPGA prior art

### 4.1 Real-number modeling (RNM), wreal, SystemVerilog nettype

- What it is: analog blocks replaced by behavioral models whose wires carry real values inside a digital simulator, no analog solver [S] (src:tdf-wreal). SystemVerilog 1800-2012 added user-defined nettypes (UDN); structured UDN overcomes limits of scalar Verilog-AMS wreal for loading effects [S] (src:dvcon-udn). A search summary says wreal "does not perform accurately for models that involve significant feedback" [S] (src:tdf-wreal).
- Vendors: Cadence trains on wreal modeling with Spectre AMS Designer / Xcelium [S] (src:cadence-training); Siemens Questa ADMS "works exclusively with Eldo" and lists real-number modeling [S] (src:siemens-adms); I found no Synopsys-specific RNM page and no ZeBu analog source [U].
- Emulation of RNM: patents and DVCon papers note emulators run only synthesizable code; conversion of RNM to synthesizable RTL is manual or tool-assisted; one DVCon paper says fixed-point bit-model conversion and plain FPGA emulation "have not proven to be sufficient or easy to use" [S] (src:patent-rnm-rtl, src:dvcon-emulation).
- Cadence Palladium Z2 "Real Number Modeling App" (Jan 2024): "Industry's first real number model emulation capability"; no speedup number for this app on the page; NVIDIA quote about integrating RNM constructs [V] (src:designreuse-palladium). Palladium Z3 / Protium X3 announced April 2024; no source found for analog support in Protium [S] (src:businesswire-z3).
- Open/closed: closed.
- Does NOT: derive the analog model from a netlist; users write the behavioral model.
- How t27 differs or loses: t27 derives hardware from the netlist (no hand model), but RNM runs at SoC scale on vendor emulators where t27 has one RC low-pass.

### 4.2 Stanford msdsl / svreal / anasymod (Herbst, Horowitz et al.)

- What it is: open-source FPGA framework for mixed-signal emulation. msdsl: Python generator of synthesizable models; svreal: fixed- and floating-point SystemVerilog real library; anasymod: emulator control and FPGA build. TCAD vol. 41 no. 7, pp. 2223-2236, July 2022 [V] (src:ceda-herbst). Runs "2-3 orders of magnitude faster than CPU simulations with real-number functional models" on three applications: DragonPHY receiver, firmware-controlled flyback converter, NFC-powered chip [V].
- Netlist interface: dissertation shows resistor, capacitor, inductor, switch, diode, transformer in a Python netlist; flyback described in "about 200 lines of Python"; five diodes and one transistor give 2^6 = 64 linear modes analyzed at compile time using a matrix exponential per mode [V] (src:herbst-diss).
- Numbers: flyback emulator ran at 35 MHz on a ZC706 board, about 2,800x faster than Infineon's SIMetrix-based CPU simulation [V]; transistors and diodes were "simple on/off models" because coarse accuracy sufficed [V]. ICCAD 2018 paper: Zynq-7045, 30 MHz emulator clock, 1.25 ms of emulated time per second, "at least 875x faster" than a fast CPU functional model, relative error -0.7%/+1.1%, 11 min from synthesis to bitstream, 11,392 LUT / 141 DSP [V] (src:herbst-iccad).
- Fixed-point: svreal auto-formats ranges; user supplies voltage_range and current_range per element in the netlist. Dissertation example: without a snubber capacitance the computed switch-node range could exceed 100 kV, causing "a resolution reduction of 10,000x or more" [V].
- Nonlinear: memoryless nonlinearities by lookup tables outside the dynamics engine [V]; generalizing to piecewise-linear device I-V makes mode count the product of segments per device, so resources "grow quickly" [V].
- Warning on fixed step: dissertation says oversampling (fixed timestep, piecewise-constant) "is almost always the approach used" in prior work and is limited by a trade-off between time resolution and throughput; for a piecewise-constant approximation of a sine, "we must double the number of timesteps to gain one bit of fidelity" [V]. The 2018 paper says discrete-time oversampled models "link simulation accuracy to the time step" [V].
- Follow-up: VLSI-SoC 2023 paper added reconfigurable variables updated on the FPGA between test cases [S] (src:herbst-search).
- Open/closed: open source (PyPI package msdsl) [S] (src:msdsl-pypi).
- Does NOT: foundry PDK transistor models (PSP103/BSIM); ngspice-compared transistor-level corners.
- How t27 differs or loses: t27 targets transistor-level PDK devices and a bit-exact reference; msdsl already has published results on three real chips, auto-ranged fixed-point, exact zero-order-hold for LTI blocks (better than backward Euler, see weakness 1), and event-driven speedups.

### 4.3 Earlier AMS-emulation work (as reported in the Stanford dissertation, src:herbst-diss, chapter 2) [V unless noted]

- Bhattacharya et al., DC-DC buck emulator in Xilinx System Generator: hand-derived discrete equations at a 50 ns step for a 200 kHz converter (about 100 steps per switching period), a few percent RMS error versus SPICE; real-time operation ties emulator clock to the step, so bigger systems get fewer samples per period.
- Mishra et al., ICWET 2011: Simscape Electrical via HDL Coder, one CMOS inverter, not shown on an FPGA.
- Wang and Chiu: SAR ADC calibration emulator at 50 MHz, 3,000x versus MATLAB (discrete-time circuit, no oversampling).
- Nothaft et al., ICCAD 2014: cellular modem on Cadence Palladium, 1 MHz, 120x versus RTL simulation; floating-to-fixed pragma library; not released.
- Tertel and Hedrich: SPICE netlist to VHDL with a global fixed-point type of +-32 V and a few microvolts of resolution; small library; 88.2 kHz oversampling; not released.
- Wu et al., WaveACE, ISCAS 2016: wave digital filters, 512 kHz timestep rate, 38 adaptors for a seven-transistor differential amplifier; nonlinear devices only as small-signal lookup tables; not released.
- Fernandez-Alvarez et al., DCIS 2016: analog on the FPGA's hard processor, 12.6 switching cycles per second for a buck.
- Henkel and Ossoinig, CDNLive 2013: 1.6 ns ticks with 14-bit time offsets, 50x versus RTL on Palladium.
- A University of Twente thesis emulated a DC-DC boost converter on FPGA with average relative error 0.56%, max 1.89%, versus Simulink [S] (src:twente-thesis).

### 4.4 Academic "SPICE on FPGA" and real-time FPGA circuit solvers

- Kapre and DeHon, "SPICE2: A Spatial, Parallel Architecture for Accelerating the Spice Circuit Simulator", IEEE TCAD vol. 31 no. 1, 2012: single FPGA gives up to an order of magnitude, mean 2.8x, over a microprocessor on double-precision floating-point SPICE [S] (src:spice2-penn). Earlier: matrix solve 1.2-64x (geometric mean 8.8x) on Virtex-5 versus Core i7 965 (FPT 2009); model evaluation mean 6.5x, range 1.4-23x (thesis) [S] (src:spice2-fpt, src:spice2-caltech). A Southampton multi-FPGA model-evaluation accelerator reports 10x and 6x less power [S] (src:soton).
- Chen and Dinavahi, "An Iterative Real-Time Nonlinear Electromagnetic Transient Solver on FPGA", IEEE Trans. Ind. Electron. 58(6), June 2011: compensation method plus Newton-Raphson, sparse, floating-point, pipelined; targets power-system transients [S] (src:ualberta-chen).
- Myaing and Dinavahi, IEEE TIE, Jan 2011: FPGA simulator with device-level behavioral model, fixed 12.5 ns step [S]; Shen and Dinavahi, IEEE TPEL 2016: electrothermal MMC, 10 us system and 10 ns device-level steps [S] (src:ualberta-fpga-emtp).
- Razzaghi et al., ISGT 2013 (EPFL): general-purpose FPGA real-time simulator with MNA and fixed admittance matrix; search summary says authors chose backward Euler for its damping of switch-induced oscillations [S] (src:epfl-razzaghi). I could not open the PDF text.
- 2016-era H-bridge FPGA simulator: MNA + backward Euler, notes "inherent damping" [S] (src:rg-hbridge).
- A recent SiC MOSFET paper (ResearchGate title "An Accurate SiC MOSFET Transient Modeling Method for the FPGA-Based Real-Time Simulation of Power Electronic Converters"): search summary says backward Euler, 10 ns step, nonlinear characteristics by LUT, latency insertion to decouple circuits; I could not open it, venue and authors unverified [S] (src:rg-sic).
- Columbia University course project "CircuitSim" (2025): SPICE-like parser in software, Gaussian elimination in FPGA hardware (Quartus, Cyclone V), backward Euler companion models, Newton-Raphson for diodes and piecewise MOSFETs, floating point [V] (src:circuitsim). Its own limitations section lists: floating-point overflow (an initial 5 V diode guess escalates to about 1e68 V and gives NaN), roundoff about 1e-6 relative, singular matrices (CMOS inverter with lambda = 0), no convergence at non-smooth region boundaries, and Newton settling on an unstable equilibrium in a CMOS oscillator when it is initialized exactly there (it stayed stuck at both 1 ns and 0.1 ns steps) [V]. Not peer reviewed, no speed claim seen.
- DFG project on wave digital filters for real-time emulation of nonlinear analog circuits, verified on an emitter-follower [S] (src:dfg-wdf).

### 4.5 FPGA "table" device models in prior art

- dSPACE and Typhoon both use LUTs for saturation/nonlinear elements (see 2.1 and 2.4) [V].
- Classic MOS table-model literature: Bell Labs and others show terminal-charge tables satisfy charge conservation and use less memory than capacitance tables; a patent notes bulk models often use average capacitances instead of charge tables, and that conventional table lookup is quasi-static [S] (src:bell-charge, src:patent-7933747, src:patent-7127384, src:ucc-table). See weakness 3.

---

## 5. Open-source neighbours

### 5.1 ngspice and XSPICE

- Role for t27: reference simulator and I-V table generator. Release 46 and beyond items; supported PDKs: Skywater 130 nm, IHP SG13G2 130 nm SiGe, GlobalFoundries 0.18 um [V] (src:ngspice-fsic2026).
- Numbers from the 2026 status slides: a 512x32 dual-port SRAM, 140k devices, 37k nodes, 454,118 equations, 3 GByte program size; "Using non-nqs version of psp103 model cuts the sim time to halve" [V].
- Simulator-side PDK requirements: hierarchical library and self-including concepts demand a parser; "certain level of subckt and model parametrization"; corner/local/global/statistical simulation [V]. Relevant to weakness 8.
- Release 46 mentions an `m3avg` function "for filtering of trap ringing" [V], evidence that ngspice's trapezoidal rule rings. Secondary sources say trapezoidal is the SPICE default and Gear adds damping that can hide instability [S] (src:asilom88, src:qorvo-forum, src:multisim-transient). I could not read the ngspice manual itself [U].
- Mixed-signal: ngspice co-simulates analog and event-driven digital [V] (src:ngspice-home). License of the software not stated on the page I read [U].
- Open: yes. Does NOT: FPGA emulation or fixed-point.

### 5.2 Xyce

- Open-source SPICE-compatible simulator (Sandia), GPL, scales to large parallel platforms; "Xyce 7.10 has been released" with no date on the page; no GPU statement on the page [V] (src:xyce). The IHP PDK repository lists both ngspice and Xyce as supported tools [V] (src:ihp-pdk).

### 5.3 OpenVAF

- Verilog-A compiler by SemiMod, GPL, "compile models ten times faster" and faster simulation than other Verilog-A compilers per its own page [V] (src:openvaf). A search summary says it emits OSDI shared objects that ngspice loads, handles PSP/BSIM/HICUM, two forks exist (original and a community "Reloaded"), and lists unsupported Verilog-A features such as @cross and Laplace filters [S] (src:esserc25, src:monata-openvaf).
- Relevance: PSP103 in ngspice for IHP runs through OSDI/OpenVAF in practice [S]; t27 reference path depends on this toolchain.

### 5.4 cpphdl "analog" (mirekez/cpphdl)

- Apache-2.0 C++ HDL repository by Mike Reznikov; its README has an "Analog and mixed-signal schemas" section [V] (src:cpphdl-readme).
- The analog document says: header-only C++ library; one engine assembles modified nodal equations, solves them "with damped Newton iteration" and integrates continuous state with backward Euler; fixed-step, adaptive/sparse DAE integration is future work; outputs trace.csv, equations.txt, report.json; RC filter reaches 0.992396 V at 5 us [V] (src:cpphdl-analog). MOS devices are `pmos`/`nmos`; the Verilog-A tests use a square-law MOS model and say BSIM "is not substituted" [V] (src:cpphdl-va). IHP SG13G2 appears as a setup with ngspice compact models [V]. Export to Verilog-A with `cpphdl --verilog-a`; examples include LDO, PLL, op-amp, ring VCO, a 10GBASE-R path [V].
- Cross-check method worth copying: each circuit runs at native step h and h/2 against an independently simulated ngspice/OpenVAF network (adaptive second-order Gear); the comparison checks absolute error and error reduction under refinement; `difference.csv` and `comparison.json` record results [V] (src:cpphdl-va).
- It states "Simulated time does not imply real-time execution" and does not mention FPGA targeting [V] (src:cpphdl-analog).
- How t27 differs or loses: same numerical recipe (MNA, fixed-step backward Euler, ngspice check) in software; t27 adds FPGA execution, fixed-point, PDK-model tables. cpphdl has broader block catalogs and a refinement-based verification method; t27 has no published equivalent.

### 5.5 uberWAVE (Symbiotic EDA side)

- "Full featured live interactive waveform viewer", open source, analog and mixed-signal, displays NGSpice results; NLnet NGI0 Commons Fund, grant started August 2025; repository on a Symbiotic EDA GitLab server; license and feature list not stated on the NLnet page [V] (src:nlnet-uberwave). I found nothing public on the partner's "self-checking analog evaluator" [U].

### 5.6 IHP Open PDK (SG13G2) as reference data

- "SG13G2 is a high performance BiCMOS technology with a 0.13 um CMOS process", SiGe:C HBTs up to 350 GHz fT; models "for ngspice/Xyce"; Apache-2.0; "IHP is currently treating the existing content as a preview only" and "not intended to be used for production at this moment" [V] (src:ihp-pdk).
- Low-voltage MOS corner file: PSP 103.6 (revision 200310); sections mos_tt, mos_ss, mos_ff, mos_sf, mos_fs plus _mismatch and _stat variants; L 0.13-10 um, W 0.15-10 um; max Vds 1.5 V; TNOM = TEMP = 27 C; mismatch sections are the only ones that include mismatch model files [V] (src:ihp-corners). A high-voltage MOS corner file is not covered here [U].

### 5.7 Other open FPGA circuit-emulation projects

- Stanford msdsl/svreal/anasymod (4.2) is the substantive one. The Columbia CircuitSim project (4.4) is a course demo. I found no other open FPGA analog-circuit emulator in the searches [U].

---

## 6. AI-native analog design tools (brief)

- AnalogCoder (AAAI-25, vol. 39 no. 1, pp. 379-387): training-free LLM agent writing Python to design analog circuits; "successfully designed 20 circuits, 5 more than standard GPT-4o" [V] (src:analogcoder). Follow-ups from search results: AnalogCoder-Pro (topology + sizing, 28 circuits on a 13-type benchmark) [S]; AnalogAgent (92% Pass@1 Gemini, 97.4% GPT-5) [S]; AaLLM (end-to-end topology to sizing) [S] (src:analogcoder-pro, src:analogagent, src:aallm).
- AutoSizer (arXiv 2602.02849, listed at ICML 2026): LLM-agent sizing loop; AMS-SizingBench with 24 circuits in SKY130; page gives only qualitative gains [V] (src:autosizer).
- Analog-DB (arXiv 2609.01286, Sept 2026): open versioned database of analog designs in a DSL with testbenches, 68 circuits in sixteen classes; a gm/ID loop closed 17 of 23 imported sizings in one to three iterations; no mention of FPGA or emulation in the excerpt [V] (src:analogdb).
- Cadence Virtuoso Studio (April 2023): generative AI mainly for process migration; no 2026 vendor data found [S] (src:cadence-virtuoso).
- Relation to t27: these tools call SPICE-class simulators in their loops; none of the pages I read mention FPGA emulation. A fast fixed-step emulator could serve as an inner-loop evaluator [I], but only if accuracy gaps (section 7 weaknesses) are quantified, otherwise the agent optimizes against emulator artifacts [I].

---

## 7. Weaknesses and risks of the t27 approach

Thresholds below are my proposals [I] unless a source is cited; each needs owner approval before becoming a gate.

### W1. Backward Euler accuracy and numerical damping

- Evidence: backward Euler "has error of order one in time", is L-stable and suited to stiff problems [V] (src:wiki-be). Stanford: halving error costs double the steps for a piecewise-constant representation [V]; Bhattacharya needed about 100 steps per switching period for a few percent RMS error [V]. EPFL and an H-bridge paper pick backward Euler specifically because it damps switch ringing, i.e. it changes physics [S]. ngspice authors added a filter for trapezoidal ringing [V], so the reference itself has method-dependent artifacts.
- Risk: a high-Q resonance (LC tank, crystal, filter) will decay faster than the real circuit; the "bit-exact versus float64" claim does not reveal this because both use the same scheme.
- Mitigation: (a) for the linear part, offer exact zero-order-hold discretization (matrix exponential, as msdsl does) when inputs are piecewise constant; (b) offer trapezoidal or BDF2 as an option with its ringing trade-off documented; (c) compile-time report of steps per period of the fastest pole.
- Acceptance check: on an RLC with Q = 20 and an LC tank, run dt, dt/2, dt/4 against the analytic solution; measured convergence order p in [0.9, 1.1] for backward Euler (and [1.9, 2.1] for any second-order option); amplitude after 100 periods within 1% of analytic at the chosen dt, otherwise the compiler prints the minimum steps per period needed.

### W2. Fixed step on stiff or multi-rate circuits

- Evidence: Stanford: "it is not necessarily sufficient to simply set dt to a small fraction of RC" because input bandwidth can be anything [V]. CircuitSim notes companion resistance dt/C and conductance dt/L "should be less than 10" for accuracy and that large steps can diverge [V]. OPAL-RT offers decoupling blocks to lower the minimum step for large circuits [V]; Typhoon runs different cores at 25 ns (DAB, resonant) versus 250 ns typical [V]. Unlike SPICE, there is no adaptive step; a 10 us horizon with fF parasitics versus kHz poles forces a tiny step everywhere.
- Mitigation: compile-time stiffness analysis (ratio of fastest to slowest eigenvalue of M, steps per fastest relevant pole) with refusal or warning; optional multirate partitioning (fast sub-circuit with its own step) later.
- Acceptance check: for every example, the compiler emits "steps per fastest pole >= 20" or an explicit waiver; a step-input test of each example at dt/4 differs from dt by <= 1% of full scale or is listed as known-limited.

### W3. Table device models lose charge conservation and capacitances

- Evidence: literature says terminal-charge tables satisfy charge conservation with less memory than capacitance tables [S]; bulk models often substitute average capacitances [S]; conventional table lookup is quasi-static and ignores dynamic offsets from parasitic resistance [S]. t27's tables are I-V only ("per-corner I-V tables"), so PSP103 charge, capacitance and non-quasi-static behavior must come from elsewhere. ngspice notes the non-NQS PSP103 variant halves runtime in a large case [V], showing NQS is a real model feature.
- Mitigation: tabulate terminal charges Q(Vg, Vd, Vb) next to I, with Id/Ig/Is partition chosen so the fourth terminal is derived (sum of charges and currents is zero); derive companion conductances from Q and I tables with consistent derivatives; state clearly if v0.1 is DC-I-V plus lumped explicit capacitors only.
- Acceptance check: (a) cyclic gate-voltage sweep returns net charge within 1e-3 of the swing charge; (b) a CMOS inverter step response shows propagation delay within 5% of ngspice+PSP103 for all five corners; (c) KCL residual at every node <= 1 LSB of node current at every step.

### W4. Table size, geometry, derivative continuity, and one bitstream per corner

- Evidence: IHP PSP103 model validity covers L 0.13-10 um and W 0.15-10 um with five corner sections plus mismatch and stat variants and temperature [V]; a table is only valid for one (W, L, T, corner) unless normalized [I]. CircuitSim: piecewise MOSFET models "lack continuous derivatives at region boundaries" and prevent convergence [V]. Stanford and dSPACE allow lookup data to change at run time without re-synthesis [V], so "one bitstream per corner" is a regression against industry practice (Vivado build was 11 min in Stanford's small design [V]; Artix-7 200T builds will be longer [I]).
- Mitigation: store tables in BRAM and load them at run time (bitstream per topology, not per corner); normalize current by W/L only where PSP103 scaling holds, otherwise one table per distinct instance with a count budget; generate tables on an adaptive grid with an error bound; use smooth (C1) interpolation or tabulate g_m and g_ds consistently.
- Acceptance check: table versus ngspice+PSP103 relative error <= 1% (floor 1e-9 A) at 10,000 random bias points per corner; derivative error g_m, g_ds <= 5%; reloading a corner takes no re-synthesis; BRAM use reported per device instance.

### W5. Fixed-point dynamic range

- Evidence: Stanford: auto-computed range for a switch node exceeded 100 kV because of 10 kOhm off-resistance times 10 A, a "resolution reduction of 10,000x or more" [V]; Tertel/Hedrich used a global +-32 V type with a few microvolts of resolution, unsuitable for capacitances or large resistances [V]. CircuitSim shows float overflow to NaN from poor initial guesses [V]. The RC result (<= 2.65 LSB) is a single benign case.
- Mitigation: per-signal range analysis (interval arithmetic plus simulation-derived ranges), saturating arithmetic with a sticky overflow flag readable over the debug port, user range annotations as in msdsl, parasitic capacitance suggestions in the compiler output.
- Acceptance check: no overflow flag in any example at any corner; for each example, a table of (signal, analytic bound, used range, LSB) with ratio analytic-bound / used-range <= 4, otherwise flagged; error versus float64 reported in LSB per circuit, not just for the RC.

### W6. Nonlinear solution inside a fixed step (convergence, delay, spurious equilibria)

- Evidence: Chen and Dinavahi use Newton-Raphson iterations in floating point [S]; CircuitSim iterates Newton-Raphson per step and reports non-convergence at non-smooth regions and an oscillator stuck at an unstable equilibrium when initialized exactly there [V]; a SiC paper decouples the external circuit with latency insertion at 10 ns [S]. Simscape uses partitioning (Forward and Backward Euler mix) for nonlinear networks [V]. The t27 description says tables are interpolated but does not say how the implicit equation is solved per step [U].
- Risk: if the nonlinear current is taken from the previous step (explicit lag), loop delay can destabilize gain stages or latch circuits and produce a fixed-step dependent result; if Newton is used, iteration count must be bounded in hardware.
- Mitigation: use companion linearization with table-derived conductances and a fixed number of Newton iterations per step; report residual; keep a software model of the same iteration so bit-exactness remains meaningful.
- Acceptance check: bistable latch and 3-stage ring oscillator: oscillation frequency within 5% of ngspice+PSP103, correct state after a 1-LSB perturbation from the metastable point, residual after the last iteration below a stated LSB bound at every step of a 10 us run.

### W7. No wall-time comparison against ngspice, and "10 us in 12 us" is slower than real time

- Evidence: 10 us of circuit time in 12 us of wall time is 0.83x real-time, i.e. the board is 1.2x slower than the circuit [I, arithmetic from the brief]. Commercial HIL tools run exactly real time at 25 ns to 4 us steps [V]. Stanford publishes speed ratios against named CPU baselines: 875x (ICCAD 2018), 2,800x versus SIMetrix (flyback), and counts build time (11 min) [V]; SPICE2 reports mean 2.8x versus a CPU [S]. For tiny RC circuits ngspice wall time is probably dominated by process start-up, so a speedup claim would be misleading; the brief offers no ngspice measurement [U].
- Mitigation: benchmark table with at least five circuits of growing size, same stimuli, ngspice with fixed tstep=tmax and with default adaptive settings, median of 10 runs, t27 total including bitstream build and readout; compute the break-even run count.
- Acceptance check: publish wall(ngspice) / wall(t27 incl. I/O) for each circuit and the number of runs after which a bitstream build pays off; no speedup claim in any document unless the ratio is >= 1 on that circuit set.

### W8. Netlist subset: .subckt, .param, .lib corners, PDK preview status

- Evidence: ngspice developers state PDK support demands a hierarchical, self-including library parser, subckt and model parametrization, and corner/local/global/statistical simulation [V]. IHP corner files select by `.lib` section names mos_tt, mos_ss, mos_ff, mos_sf, mos_fs with mismatch variants [V]. The PDK content is "a preview only" and "not intended to be used for production" [V], so golden models can change. OpenVAF has unsupported Verilog-A features [S] and two forks [S].
- Mitigation: define the supported SPICE subset in a spec, with a parse-or-refuse gate that lists the first unsupported card; use ngspice itself to expand and flatten the netlist (check ngspice's listing/expansion option before relying on it [U]); pin the IHP PDK commit hash in every test.
- Acceptance check: the compiler either accepts a netlist or exits with a named unsupported feature; all five LV corners compile for the same netlist; the PDK commit is recorded in the receipt; a regression set of at least 20 IHP/ngspice example netlists is classified accepted / refused with reason.

### W9. Waveform I/O bandwidth and format

- Evidence: commercial analog outputs are limited to 200 ns updates (Typhoon HIL404 snippet [S]; Plexim "no faster than every 200 ns" [V]); Stanford captured waveforms with on-chip logic-analyzer blocks that consumed BRAM [V]. Data rate for t27 = nodes x bits x steps per second; the step is not given in the brief [U]. uberWAVE reads ngspice results [V], so output must be in a format it opens (ngspice raw or CSV) [I].
- Mitigation: on-chip capture buffer with trigger and decimation; burst readout; writer for the ngspice raw format so uberWAVE and ngspice overlays work.
- Acceptance check: capture N nodes x M samples with zero loss (checksum matches the software model); readout time reported; uberWAVE opens the file and overlays the ngspice trace without manual conversion.

### W10. Validation design: reference grid and what "error" means

- Evidence: ngspice uses adaptive steps; t27 uses a fixed grid, so comparison needs interpolation. cpphdl handles this by running native h and h/2 and comparing error reduction against an independent simulation with adaptive second-order Gear [V]. RTDS validation work found converter models differing from PSCAD in ripple and stability [V]. "<= 2.65 LSB from float64" measures arithmetic only [I].
- Mitigation: report three errors separately: (1) arithmetic (fixed-point versus float64, same scheme), (2) discretization (float64 scheme versus a fine-step or higher-order reference), (3) model (reference versus ngspice+PSP103). Adopt the h and h/2 refinement check from cpphdl.
- Acceptance check: every circuit report has the three numbers with thresholds; error(h/2) / error(h) in [0.4, 0.6] for backward Euler or the report marks the circuit as not in the asymptotic regime.

### Other risks worth one line each

- Scale: published solvers cap at roughly 144 switches / 256-344 states on large FPGAs [V]; t27's dense M on XC7A200T (740 DSP, 0 DSP used so far) will need a sparse or time-multiplexed MAC plan [I].
- Positioning: claiming novelty over msdsl or cpphdl is unsafe; they share much of the method [V].
- Partner dependency: public information on Symbiotic EDA's evaluator was not found [U]; do not cite capability that is not public.

---

## 8. What to borrow, in order of value [I]

1. Exact zero-order-hold (matrix exponential) for linear blocks, as msdsl.
2. Run-time loadable table data, as dSPACE and msdsl follow-up.
3. h and h/2 refinement versus ngspice, as cpphdl.
4. Per-element range annotations and a range report, as msdsl.
5. A bundled validation against a baseline with tolerance, as Simscape Advisor.
6. Time-stamped switching events (OPAL-RT Gen5) if switching circuits are in scope.

---

## 9. Sources (every URL used)

Commercial HIL
- typhoon-arch - https://www.typhoon-hil.com/documentation/typhoon-hil-software-manual/concepts/system_architecture_basics.html
- typhoon-fpga-basics - https://typhoon-hil.com/documentation/typhoon-hil-software-manual/concepts/fpga_solver_basics.html
- typhoon-emr25 - https://emr-website.univ-lille.fr/fileadmin/user_upload/emr-website/EMR25/Presentations/18-EMR-25-Typhoon-Arsenijevic.pdf
- typhoon-hil404-brochure - https://typhoon-hil.com/doc/brochures/Typhoon-HIL404-Brochure.pdf
- typhoon-hil606 - https://www.typhoon-hil.com/wp-content/uploads/2025/07/Brochure_A4-HIL606-Digital.pdf
- typhoon-hil606-webinar - https://typhoon-hil.com/doc/brochures/HIL606_Webinar.pdf
- typhoon-faq - https://ticket.typhoon-hil.com/kb/faq.php?id=184
- typhoon-nonlinear - https://www.typhoon-hil.com/documentation/typhoon-hil-schematic-editor-library/References/nonlinear_passive_components.html
- typhoon-products - https://www.typhoon-hil.com/products/
- typhoon-blog-hil606 - https://www.typhoon-hil.com/blog/meet-the-new-hil606-with-cto-dusan-majstorovic/
- opalrt-blog - https://www.opal-rt.com/blog/ehs-the-nanosecond-power-electronics-solver-whose-time-has-come/
- opalrt-main-features - https://opal-rt.atlassian.net/wiki/spaces/PFPET/pages/65672356/Main+features
- opalrt-solver-settings - https://opal-rt.atlassian.net/wiki/spaces/PSED/pages/68618522/Defining+your+solver+settings
- ni-ehs - https://www.ni.com/en-my/shop/product/ehsx128-real-time-power-electronics-solver.html
- ni-forum - https://decibel.ni.com/content/thread/46340
- ni-forum-search - https://sine.ni.com/nips/cds/view/p/lang/en/nid/217708
- aau-dfig - https://vbn.aau.dk/files/475858558/Real_Time_FPGA_based_HIL_Emulator_of_Power_Electronics_Controllers_using_NI_PXI_for_DFIG_Studies.pdf
- plexim-fpga - https://docs.plexim.com/tsp/rtbox/4.0/fpga/
- plexim-hil - https://www.plexim.com/node/970
- dspace-ecm - https://www.dspace.com/en/ltd/home/products/sw/automotive_simulation_models/produkte_asm/electric_system_models/electric_componen.cfm
- dspace-patent - https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/12216967
- rtds-paper - https://vbn.aau.dk/ws/files/329221832/Tinajero_2020_J._Phys._Conf._Ser._1457_012007.pdf
- rtds-patent - https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/9985439

MathWorks
- mw-hdl-codegen - https://www.mathworks.com/help/simscape/hdl-code-generation.html
- mw-advisor - https://www.mathworks.com/help/simscape/ug/generate-hdl-code-using-the-simscape-hdl-workflow-advisor.html
- mw-linearize - https://www.mathworks.com/help/simscape/ug/linearize-to-prepare-for-codegen.html
- mw-hil-start - https://www.mathworks.com/help/hdlcoder/ug/get-started-with-simscape-hardware-in-the-loop-workflow.html
- mw-partitioning - https://www.mathworks.com/help/hdlcoder/ug/Generate-HDL-Code-for-Nonlinear-Simscape-Models-Using-Partitioning-Solver.html
- mw-dynswitch - https://www.mathworks.com/help/hdlcoder/ug/generate-hdl-code-for-simscape-models-using-dynamic-switch-approximation.html
- mw-pmos-cap - https://www.mathworks.com/help/sps/ref/pmoscapacitor.html

AMS emulation and RNM
- ceda-herbst - https://ieee-ceda.org/media/open-source-framework-fpga-emulation-analogmixed-signal-integrated-circuit-designs
- herbst-iccad - https://arxiv.org/abs/2002.02072 and https://arxiv.org/pdf/2002.02072
- herbst-diss - https://purl.stanford.edu/gj828vr5382 and https://vlsiwiki.stanford.edu/people/alum/pdf/2106_Herbst_FPGAEmulationOfAnalogCircuits.pdf
- herbst-search - https://awesome.ecosyste.ms/projects/github.com%2Fsgherbst%2Fanasymod
- msdsl-pypi - https://pypi.org/project/msdsl/
- designreuse-palladium - https://www.design-reuse.com/news/55563/cadence-palladium-z2-apps-4-state-emulation-mixed-signal-modeling.html
- businesswire-z2 - https://www.businesswire.com/news/home/20240118083007/en/Cadence-Unveils-New-Palladium-Z2-Apps-with-Industrys-First-4-State-Emulation-and-Mixed-Signal-Modeling-to-Accelerate-SoC-Verification
- businesswire-z3 - https://www.businesswire.com/news/home/20240417697934/en
- tdf-wreal - https://www.techdesignforums.com/practice/?p=4447
- dvcon-udn - https://dvcon-proceedings.org/wp-content/uploads/enabling-digital-mixed-signal-verification-of-loading-effects-in-power-regulation-using-systemveriloguser-defined-nettyp.pdf
- dvcon-emulation - https://dvcon-proceedings.org/wp-content/uploads/157-Enabling-True-System-Level-Mixed-Signal-Emulation.pdf
- patent-rnm-rtl - https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/10133837
- cadence-training - https://newstaging.cadence.com/en_US/home/training/all-courses/85059.html
- siemens-adms - https://www.siemens.com/en-us/products/ic/questa-one/adms/
- twente-thesis - https://essay.utwente.nl/essays/87535

Academic FPGA circuit solvers
- spice2-penn - https://ic.ese.upenn.edu/abstracts/spice_trcad2012.html
- spice2-fpt - https://ic.ese.upenn.edu/pdf/matsolve_fpt2009.pdf
- spice2-caltech - https://thesis.caltech.edu/6159/
- soton - https://eprints.soton.ac.uk/173435
- ualberta-chen - https://era.library.ualberta.ca/items/5d2f2abb-3e06-47d1-b3a9-a3e5a392a6c8
- ualberta-fpga-emtp - https://era.library.ualberta.ca/items/310e2892-33d2-4d2f-8bd6-9eb0a3f40490
- epfl-razzaghi - https://infoscience.epfl.ch/server/api/core/bitstreams/55a9adc0-f7d5-49fa-bb0f-6cc589decf2e/content
- rg-hbridge - https://www.researchgate.net/publication/307593207_Design_and_Implementation_of_an_FPGA-based_Real-Time_Simulator_for_H-Bridge_Converter
- rg-sic - https://www.researchgate.net/publication/388807351_An_Accurate_SiC_MOSFET_Transient_Modeling_Method_for_the_FPGA-Based_Real-Time_Simulation_of_Power_Electronic_Converters
- circuitsim - https://www.cs.columbia.edu/~sedwards/classes/2025/4840-spring/reports/CircuitSim-report.pdf
- dfg-wdf - https://gepris.dfg.de/gepris/projekt/401215102/ergebnisse

Table models
- bell-charge - https://www.nokia.com/bell-labs/publications-and-media/publications/the-extraction-of-charge-matrices-from-two-dimensional-device-simulations-of-mos-transistors
- ucc-table - https://research.ucc.ie/en/publications/mos-table-models-for-circuit-simulation/
- patent-7933747 - https://patents.google.com/patent/US7933747
- patent-7127384 - https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/7127384

Open source and PDK
- ngspice-fsic2026 - https://wiki.f-si.org/images/9/94/FSiC2026_ngspice.pdf
- ngspice-home - https://ngspice.sourceforge.io/
- asilom88 - https://research.library.mun.ca/14689/1/ASILOM-88.pdf
- qorvo-forum - https://forum.qorvo.com/t/need-guidance-on-qspice-integration-method-and-this-feather-parameter/14393
- multisim-transient - https://www.ni.com/docs/ar-SO/bundle/multisim/page/topics/transient-tab.html
- xyce - https://xyce.sandia.gov/
- openvaf - https://openvaf.semimod.de/
- esserc25 - https://fides.fe.uni-lj.si/~arpadb/ESSERC25/ESSERC2025.pdf
- monata-openvaf - https://monata-docs.readthedocs.io/en/stable/toolchain/openvaf-osdi.html
- cpphdl-readme - https://github.com/mirekez/cpphdl and https://raw.githubusercontent.com/mirekez/cpphdl/main/README.md
- cpphdl-analog - https://raw.githubusercontent.com/mirekez/cpphdl/main/analog/README.md
- cpphdl-va - https://raw.githubusercontent.com/mirekez/cpphdl/main/analog/VerilogA/README.md
- nlnet-uberwave - https://nlnet.nl/project/uberWAVE/
- ihp-pdk - https://github.com/IHP-GmbH/IHP-Open-PDK
- ihp-corners - https://raw.githubusercontent.com/IHP-GmbH/IHP-Open-PDK/main/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib
- wiki-be - https://en.wikipedia.org/wiki/Backward_Euler_method

AI-native analog
- analogcoder - https://ojs.aaai.org/index.php/AAAI/article/view/32016
- analogcoder-pro - https://alphaxiv.org/abs/2508.02518
- analogagent - https://arxiv.org/pdf/2603.23910v1
- aallm - https://arxiv.org/pdf/2608.13472
- autosizer - https://arxiv.org/abs/2602.02849v1
- analogdb - https://arxiv.org/abs/2609.01286
- cadence-virtuoso - https://www.eejournal.com/industry_news/cadence-unleashes-the-future-of-analog-custom-and-rfic-design-with-pioneering-ai-powered-virtuoso-studio/

Repository evidence for section 11 (spec:<file> is specs/analog/<file>.t27, spec:sar_adc_ax7203 is specs/fpga/sar_adc_ax7203.t27, spec:spice_convert is specs/tools/t27o/spice_convert.t27, all at origin/master c33aec9c1)
- epic - https://github.com/gHashTag/t27/issues/10486
- ledger - https://github.com/gHashTag/t27/issues/10486#issuecomment-6251580882
- pr-N - https://github.com/gHashTag/t27/pull/N for N = 8844, 10517, 10528, 10530, 10578, 10604, 10779, 10781, 10845, 10854, 10884, 10893, 10898, 10919, 10940, 10953, 11055
- issues cited by number only - https://github.com/gHashTag/t27/issues/N for N = 10503, 10516, 10749, 10777, 10785, 10848, 10881, 10882, 10888, 10909, 10910, 10911, 10920, 10941, 10942, 11020, 11040

Competitor delta of section 11 (fetched 2026-10-19)
- ngspice-news - https://ngspice.sourceforge.io/news.html
- vacask-releases - https://codeberg.org/arpadbuermen/VACASK/releases
- vacask-nlnet - https://nlnet.nl/project/VACASK/
- vacask-pypi - https://pypi.org/project/vacask-bin/0.3.4.dev0/ (search result only, the page did not load)
- openvaf-gh-commits - https://api.github.com/repos/arpadbuermen/OpenVAF/commits?per_page=3
- openvaf-gh-releases - https://api.github.com/repos/arpadbuermen/OpenVAF/releases?per_page=3
- xyce-gh-releases - https://api.github.com/repos/Xyce/Xyce/releases?per_page=3
- cpphdl-gh-commits - https://api.github.com/repos/mirekez/cpphdl/commits?per_page=5
- anasymod-gh-commits - https://api.github.com/repos/sgherbst/anasymod/commits?per_page=3
- msdsl-gh-commits - https://api.github.com/repos/sgherbst/msdsl/commits?per_page=3
- ihp-gh-commits - https://api.github.com/repos/IHP-GmbH/IHP-Open-PDK/commits?per_page=5
- ihp-pin-commit - https://api.github.com/repos/IHP-GmbH/IHP-Open-PDK/commits/21133f219b1a6da10d803e345761e77415ceff40
- typhoon-blog - https://www.typhoon-hil.com/blog/
- opalrt-ehs-page - https://www.opal-rt.com/software/software-toolboxes/ehs/
- mdpi-emt - https://doi.org/10.3390/electronics15030606 and https://api.crossref.org/works/10.3390/electronics15030606
- e4ds-ni-its - https://www.e4ds.com/sub_view.asp?idx=23859&lang=en
- arxiv-sipm - https://arxiv.org/abs/2607.00841
- cadence-palladium-page - https://www.cadence.com/en_US/home/tools/system-design-and-verification/emulation-and-prototyping/palladium.html (search result only)
- synopsys-darpa - https://www.design-reuse.com/news/news44966.html (search result only)
- siemens-veloce-cs - https://www.design-reuse.com/news/55733/siemens-veloce-cs.html (search result only)

## 10. Gaps (what I could not establish)

- Number format inside Typhoon, Plexim, dSPACE FPGA solvers: not published in pages read [U].
- Any public price for Typhoon, OPAL-RT, Plexim, dSPACE, RTDS: none found [U]; NI shows "contact for quote".
- Synopsys and ZeBu analog or RNM emulation: no source found [U].
- Full text of EPFL (Razzaghi 2013), the 2025 SiC FPGA paper, and Typhoon HIL404 brochure: not readable; only summaries [S].
- ngspice official manual text on integration defaults: not read [U].
- The partner's self-checking analog evaluator: nothing public found [U].
- High-voltage MOS and HBT (VBIC) corner structure in IHP PDK: not read [U].
- Added 2026-10-19: wall time of one bitstream build (yosys, nextpnr, bitstream) and the break-even run count: in no receipt, so no speed claim is allowed [U].
- Added 2026-10-19: the ngspice options (klu, threads) of the 18 minute remote run of the golden deck: not recorded [U].
- Added 2026-10-19: full text of the 2026 FPGA electromagnetic-transient solver paper (publisher page returned 403; only the Crossref record and an abstract excerpt were read) and of the NI ITS item (excerpt only) [U].
- Added 2026-10-19: Synopsys and Siemens analog emulation in 2026: again no source found [U]. Vendor pages for Typhoon, Plexim and OPAL-RT carry no dates on their spec tables, so "new" cannot be decided from them [U].
- Added 2026-10-19: VACASK, Xyce and ngspice on the golden SAR deck side by side (issue #10911): not run [U].
- Added 2026-10-19: where the pinned IHP-Open-PDK commit 21133f21 lives: it resolves through the IHP-GmbH repository API to a merge dated 2026-09-08 of a pull request from a fork's upstream-sync branch, while the newest commit on the IHP-GmbH default branch list read the same day is dated 2026-09-01; whether the model files differ from upstream is not checked [U].

---

## 11. Status after M1-M5

Written 2026-10-19 against origin/master at c33aec9c1. Scope: the ten weaknesses of section 7, re-read against what this repository measured in M0 to M5 (epic gHashTag/t27#10486: M0, M1, M1b, M1c, the ngspice bench, M2a, M2b, M3a, M3b, M3c, M4a to M4d, M4e with its offset experiments and its board run, M5 first slice). It is the honest basis for the v0.1 release notes: it says what the work did not close.

### Tags and sources in this section

- Every number is copied from the source named next to it and is tagged [V] by default: I read it in the header or a test of a spec (written spec:<file>, see section 9), in a PR body (PR #N), in the epic (the body of gHashTag/t27#10486 as fetched on 2026-10-19) or in its ledger comment (ledger, comment 6251580882).
- [I] is arithmetic or a reading by me on top of those numbers. [U] is a number or a check I looked for in the repository and did not find; nothing is estimated. [S] appears only in the competitor delta at the end.
- CLOSED means the acceptance check of section 7 passes on the circuits that matter (the SAR blocks). REDUCED means some clauses of it are measured and pass and others fail or are not measured. OPEN means the measurements do not change what section 7 says. No weakness is CLOSED: each has at least one clause that is measured as failing or not measured.
- Tally: 0 CLOSED, 8 REDUCED, 2 OPEN (W2 and W7). W11 to W19 are weak points found while building.

### Status of W1 to W10

- W1: REDUCED. Exact zero-order-hold constants exist for the linear cell and are measured; the nonlinear path and the netlist path are still backward Euler.
  - Measured: spec:linear_zoh (M1c, #10503, PR #10528) puts the exact constants into the same fabric step, no hardware change. LC tank Q = 200, amplitude after 100 periods against the analytic one: exact scheme 1.0003 in Q.24 (1.00000000000006 in f64), backward Euler 1.78e-7. Backward Euler order measured at dt, dt/2, dt/4: 0.985 and 0.993 (RC), 0.975 and 0.988 (RLC Q 1), 0.915 and 0.957 (RLC Q 20); the exact scheme is within 3e-14 of the analytic solution at every grid point. For the RLC the backward Euler discretization error is 1.89e-2 V against an arithmetic error of 5.0e-7 V, some 37000 times larger: the number section 7 said a bit-exact comparison with float64 cannot show.
  - Not closed: the nonlinear path is backward Euler. spec:mos_psp103 (PR #10778): the inverter at dt = 2 ns differs from ngspice converged by 14.4 mV (242041 units) and its delays are 0.51 % and 0.66 % longer. spec:sar_switch (PR #10779): backward Euler at 250 ps against a grid five times finer moves the switch overdrive by 2.96 mV, the tracking error by 0.27 % and the injection by 4.5 %. The netlist path folds backward Euler only: spec:spice_convert refuses a deck without method=gear maxord=1 (PR #10781). No second-order option was added ("none is added here", spec:linear_zoh). A compile-time report of steps per period of the fastest pole: not found [U].
  - Next step: t27o emits the exact-ZOH constants for the linear part of a deck; the comparator's discretization at 0.1 ps is measured against a finer grid (spec:sar_comparator calls it "shared with ngspice by construction", that is, unmeasured).

- W2: OPEN. The step is chosen by hand per block, its effect is measured once, and no stiffness report or multirate coupling exists.
  - Evidence: a stiffness analysis, a steps-per-fastest-pole report or a multirate partition: not found in any spec [U] (the word "stiff" appears in specs/analog only in prose about the switch step and the DAC plates). Steps in use: comparator 0.1 ps, 140 steps (spec:sar_comparator); switch 250 ps, 416 steps, because 0.1 ps "would take 5 million steps for one 500 ns period" (spec:sar_switch); inverter 2 ns at 10 pF and 100 ps at 500 fF (spec:mos_psp103); gate logic as an event simulation with a 1 ps quantum (spec:sar_logic). The golden run is 16 us and ngspice's adaptive step needs 161633 time points for it (spec:sar_adc); one fixed 0.1 ps grid would need 1.6e8 steps, about 990 times as many [I]. The step matters: see the 2.96 mV overdrive shift under W1. The switch's damped Newton needs up to 120 iterations on the clock-fall step and 1.1 on average, so the work per step is data dependent exactly at the edges (spec:sar_switch).
  - Next step: a per-block step table with a steps-per-fastest-pole check in t27o for linear blocks; decide how a 0.1 ps block and a 250 ps block are coupled in M4e at device level (today the converter takes the held plates from the ngspice run, spec:sar_adc).

- W3: REDUCED. Drain and gate charge tables bring the 500 fF inverter within 0.02 % of ngspice; they exist for tt and ss only, and the comparator and the switch still carry no charge.
  - Measured: M3c (PR #10893, sub-issue #10880 of #10777) tabulates drain and gate charge for tt and ss from the terminal current of one backward Euler step (PSP103 reports no terminal charge). The 500 fF inverter at dt = 100 ps against ngspice backward Euler on the same grid: delays within 0.020 % (tpLH) and 0.0041 % (tpHL), where current-only tables were 3.09 % and 3.63 % fast; worst waveform sample 6693 units (0.40 mV, 0.027 % of Vdd) against 438317 (26 mV) without charge; table slopes within 0.16 %, 0.29 % and 0.19 % of the capacitances ngspice itself reports; Newton residual at most 2 units at every step; the ss charge used in a tt run is refused by 2.5 to 3.6 times (spec:mos_psp103).
  - Not closed: charge is tabulated for tt and ss only (ff, fs, sf: #10882, open), drain and gate only, for a 500 fF node and a 100 ps step; no circuit reads the gate charge; no source or bulk charge. The comparator is still current-only on PSP103: its decision at +2 mV is 5.35 ps in the model against 523.0 ps in ngspice with its own charge, 96 to 98 times too fast (resolution 155 to 183 times) (spec:sar_comparator_psp, PR #10884). The switch is square law with no gate capacitance, so no charge injection is modelled (spec:sar_switch). The transient is checked for tt only. Non-quasi-static behaviour, and the cyclic gate-sweep charge-conservation check of section 7: not found [U].
  - Next step: #10882; put the charge tables into the comparator and the switch and re-measure the 96 to 98 times.

- W4: REDUCED. Five corners are tabulated for two device sizes within 0.56 % where Id >= 100 uA; weak inversion, body effect, a table per temperature and the run-time load on a board remain.
  - Measured: spec:mos_psp103_data (M3b, #10516, PR #10778): sg13_lv_nmos W 1.0 um and sg13_lv_pmos W 3.0 um, L 0.45 um, five corners, 25 x 25 nodes of 62.5 mV (625 words each, under 2^21), IHP-Open-PDK commit 21133f21, 27 C. Against ngspice at held-out points (tt): absolute 6.51e-7 A (NMOS) and 4.39e-7 A (PMOS); relative 0.56 % and 0.41 % where Id >= 100 uA, 4.3 % and 3.2 % for Id >= 10 uA, 21 % and 17 % for Id >= 1 uA. The section 7 bound of 1 % with a 1e-9 A floor is NOT met (measured 0.55 to 0.73): in weak inversion Id grows 6.5 to 7 times per cell and a chord is more than 41 % above the exponential (follow-up #10881, open). Temperature is another table set: spec:mos_psp103_50c_data has the saturation current 5.8 % lower (NMOS) and 4.2 % lower (PMOS) at 50 C, and the 27 C tables in a 50 C comparator run decide 5.0 % early (resolution 6.0 %) (spec:sar_comparator_psp). The slopes gm and gds are those of the interpolant, constant per cell, so the 0.35 V threshold inside cell 5 gives a soft knee: 0.25 mV in the switch gate and 39.0 mV on one node (spec:sar_switch). The 2-D tables have no body effect: the golden comparator decides 6.2 to 6.5 % later than the bulk-tied one (spec:sar_comparator_psp). A multiplier m needs no table change (PSP103 multiplies the current by m).
  - Not closed: board_step takes the two tables as slices a board would load (spec:mos_psp103), but no board top reads a table, and both board tops use 0 BRAM (PR #10578, PR #10919): "one bitstream per topology, not per corner" is a design, not a receipt. Tables for another W, L, mismatch or the high-voltage devices: not made [U].
  - Next step: #10881 and #10882; a Vsb dimension or a threshold-shift term; a table-load top on the AX7203.

- W5: REDUCED. The arithmetic error is measured per circuit and every board receipt is bit-exact with its spec; there is no overflow flag and no range table.
  - Measured: arithmetic error against float64 of the same scheme, in LSB of 2^-24 V, asserted from both sides: M1 RC 2.65, RLC 8.44, ladder-3 9.45, ladder-5 40.03 (bounds 3, 9, 10, 41), growing with 1 / (1 - rho) of the slowest pole (spec:linear); exact scheme 4.84, 9.61, 34.71 and 120.50 for the tank (spec:linear_zoh); inverter 0.61 unit (spec:mos_table) and 4.78 units (spec:mos_psp103); DAC 1.74 units (spec:sar_cdac); comparator 68.05 units (4.1 uV) at 2 mV and 250.8 units (15 uV) at 0.5 mV, amplified by the regeneration (spec:sar_comparator); switch 237 units (14 uV) (spec:sar_switch). Board receipts are bit-exact with the spec: M1b 572 of 572 lines PASS and each control prints FAIL (286 lines) on its own circuit only (PR #10578); the SAR converter 550 of 550 lines PASS and its control fails exactly the 16 predicted conversions (PR #10919). Headroom: magnitudes below 2^26 in M1 (asserted), the widest word 4.95e14 (under 2^49) in the comparator and 7.2e16 (2^56) in the switch at the clock edges, in a u64 (spec:linear, spec:sar_comparator, spec:sar_switch). An unsigned word with ground at 0 cannot hold the switch's -0.17 V, so the switch counts from an offset of 0.5 V (spec:sar_switch).
  - Not closed: saturating arithmetic and a sticky overflow flag: not found in any spec or board top [U]; no table of analytic bound against used range per signal; ranges are asserted per run, not derived. The nonlinear solvers have not run in fixed point on a board (W16).
  - Next step: a sticky overflow bit and a range table in the receipt of the next board top.

- W6: REDUCED. A fixed number of Newton iterations with a bounded residual works on the inverter, the comparator and (damped) the switch; no oscillator or hold-state circuit has been run and no nonlinear step has been synthesised.
  - Measured: every nonlinear cell runs a fixed K of Newton iterations per step and computes the residual after the last one at every step. Inverter K = 3, last residual at most 1 unit, K = 1 leaves 8586 (spec:mos_psp103); with charge at most 2 units, K = 1 leaves 11636. Comparator K = 3: at most 2 units on the square law (K = 1 leaves 29361 units, 1.75 mV) and at most 3 units on PSP103 tables (K = 2 leaves 155, K = 1 leaves 20750) (spec:sar_comparator, spec:sar_comparator_psp). On PSP103 tables the comparator decides the right way in all 14 runs (7 inputs down to 0.12 uV, two orientations), its decision time within 0.66 % and its resolution time within 0.72 % of the bulk-tied PSP103 reference. At 0 V of input the circuit in real arithmetic stays exactly symmetric, and in fixed point one unit of rounding at step 60 grows to 2027 units at step 100 and 2659643 (0.158 V) at step 140: reported, not hidden (spec:sar_comparator). The switch uses a damped Newton with a stop test, at most 120 iterations and 1.1 on average (spec:sar_switch).
  - Not closed: the acceptance circuits of section 7 (a bistable latch held in a state, a three-stage ring oscillator within 5 % of ngspice) have no counterpart in specs/analog [U]; the SAR latch is the only regenerative circuit run. The switch's loop is a `while` with a flag that `t27b verilog` may refuse, and it has not been through it (spec:sar_switch).
  - Next step: take the comparator step through t27b verilog and the open flow; report LUT, DSP, Fmax and clocks per step with the iteration bound as a counter and the residual as a status bit.

- W7: OPEN. No speed claim is allowed: the only like-for-like row (M0) is a ratio of two fixed overheads, the build time is unmeasured, and the board runs a behavioural converter.
  - Measured: spec:bench (PR #10517), ngspice-47 on the bench laptop (8 GB), median of 10 runs: wall 14228 us for the RC and 14813, 14607, 14659 and 15021 us for the RLC and the 3-, 5- and 10-node ladders; the .tran itself takes 796 us for the RC, so more than 90 % is start-up. M0 on the board: 1200 clocks at 100 MHz is 12 us of compute for 10 us of circuit time (0.83 times real time [I]), and the first full 108-character line at 115200 baud arrives after 9387 us (computed, not a stopwatch reading); the wall ratio is 1.52, a ratio of two fixed overheads, and the compute-only ratio of 66 is not a claim about a run. may_claim_speedup says no for every row because BUILD_US is 0: the bitstream build time is in no receipt.
  - Since then: the golden SAR deck in ngspice-47 with PSP103 took 3894.9 s (65 minutes) of wall time, 3285.7 s of user time and 161633 time points, on the laptop while other jobs ran (load average 5 to 40) (spec:sar_adc, PR #10854); 18 minutes on a remote Docker host, 10 of 12 cores, the same 161633 points (ledger, tick 28), about 3.6 times shorter [I] but from two machines and not a controlled comparison; with option klu and 8 threads a slice of the deck took 153 s where the default took 275 s (epic, row of PR #10884). The board's SAR converter takes 75 clocks (0.75 us) per conversion and 63 characters (5.5 ms) per line (PR #10919), but it runs ROM samples and a sign comparator with two fitted numbers, so its spec calls a comparison with the 65 minutes "not a comparison of the same thing" (spec:sar_adc_ax7203). Bitstream build time and break-even run count: not measured [U].
  - Next step: put the wall time of one yosys, nextpnr and bitstream build in a receipt; until then no document states a speed claim. The only like-for-like row will be a nonlinear block solved on the fabric against ngspice on the same deck.

- W8: REDUCED. The SPICE reader parses, flattens and refuses with the line; the device tables are not yet generated from the netlist and no 20-netlist regression set exists.
  - Measured: spec:spice_convert (PR #10530, 22 tests; PR #10781, 39 tests) reads the whole golden SAR deck: 16 subcircuits, 57 MOS lines, 1265 MOS and 128 cap_cmim after flattening (epic); 113 gates for M4d: inverter 23, nor 44, nor3 1, tgate 44, delay cell 1 (spec:sar_logic). It refuses with the line and the reason: the element letters D Q J K M G F H S W T U O P Z Y, .measure, .func, .model, {expressions}, subcircuit parameters, instance parameters other than m (w l ng on PDK devices); .lib and .include are recorded and never followed, so no PDK file is read. Every table names the IHP-Open-PDK commit 21133f21 and ngspice-47 (spec:mos_psp103_data).
  - Not closed: the device tables come from hand-written ngspice decks (psp103_tab_*.cir) for one NMOS and one PMOS size, not from the netlist. A regression set of at least 20 netlists classified accepted or refused: not found [U] (the set is the decks in specs/analog/decks and the golden deck). The PDK commit is in the spec headers, not in the board's receipt line. The SAR runs are one corner (mos_tt, cap_typ) at 50 C. The PDK content is a preview by its own statement (section 5.6).
  - Next step: t27o emits the table deck for each distinct (model, W, L, m, temperature) of a netlist and refuses a device with no table; the PDK commit goes into the receipt.

- W9: REDUCED. The RAW writer is byte-equal to ngspice's and read by uberWAVE's parser; the boards stream receipts, not waveforms.
  - Measured: spec:raw_writer (M5, #10909, PR #10940, 18 tests): ASCII and binary files equal ngspice-47's own write byte for byte on the 15-point, 4-variable reference; %.15e equals Python's on 35 vectors; ngspice loads both files (load, print); uberWAVE's wp2-parser (commit c75a89c8f) parses both, the binary file bit-exact over 1005 values and the ASCII file within 4.4e-16 relative; `tri raw` writes the M1 RLC trace of 201 points; five writer mutants each fail. uberWAVE refuses a header of 202 points over 201 rows but accepts 200 (PR #10940).
  - Not closed: bandwidth. The boards print receipt lines: 108 characters per M0 line (9375 us at 115200 baud) and 63 per SAR line (5.5 ms); both tops use 0 BRAM, so there is no capture buffer, trigger or decimation. M4e's trace as RAW and the partner's per-block checks on it are not done (epic, item 4 of what is left). That uberWAVE's viewer overlays our trace on ngspice's without conversion is shown for the parser only; no screenshot found [U].
  - Next step: M5 second half; a BRAM capture buffer with burst readout and a checksum compared with the spec's fold.

- W10: REDUCED. Three errors are reported apart for the blocks up to M4c; M4b to M4d share the grid with ngspice by construction, and the converter is scored by code match.
  - Measured: arithmetic, table and discretization are reported apart in M1c, M3a, M3b, M3c, M4b and M4c. M3a 29 nV, 0.428 mV and 27.2 mV (spec:mos_table); M3b at 10 pF 0.28 uV, 1.28 mV and 14.4 mV (spec:mos_psp103); M4b arithmetic 4.1 uV, table 15.2 mV, model against ngspice 1.44 nV (spec:sar_comparator); M4c arithmetic 14 uV, table 39.0 mV, model 0.5 nV, discretization 2.96 mV in the overdrive (spec:sar_switch). The h and h/2 check: spec:linear_zoh measures the order at dt, dt/2, dt/4 against the analytic solution and marks the Q = 20 circuit as not in the asymptotic regime at dt = 5 ns (0.84 over 400 steps). ngspice is a fixed-step reference to between 1e-15 and 9.4e-15 V with gear order 1, a B source and a pre-roll, where a plain PWL source is 1.7e-5 V off the recurrence at 5 us, and ngspice's own adaptive run is 2.4e-4 to 3.4e-4 V from the continuous circuit (spec:bench, PR #10517).
  - Not closed: M4b and M4d state that the discretization is shared with ngspice by construction and is not measured (the comparator's times are the grid's); only M4c measured it, once. No error(h/2) / error(h) test exists for a nonlinear block. The converter is scored by code match, not by the three-error split, and the reference itself is not unique at the LSB (W15).
  - Next step: measure the comparator at 0.1 ps and 0.05 ps against ngspice on both grids; report the converter's error as a code distance (W15).

### Found while building (W11 onward)

### W11. A UART defect only the board showed

- Status: CLOSED for this defect; OPEN as a class (a behaviour that appears only at the real clock ratio).
- Evidence: the first capture of the SAR top on the AX7203 (PR #10919, spec:sar_adc_ax7203) had the right payload after K= on every line (top 551 lines, control 555), but every line after the first began with the same seven bytes 95 BA 02 9A 05 49 81 instead of "T27 SAR ". Cause: the transmitter set the restart flag at the shift that starts the LF's stop bit; the next conversion needs about 80 clocks, 0.09 of a bit time at 868 clocks a bit, so the next start bit cut the stop bit to a tenth of its time and a terminal lost frame alignment. A simulation at 8 clocks per bit did not show it (80 clocks were 10 bit times), and M1b is immune because its lines are tens of thousands of clocks apart. Fix: 8 flip-flops (a gap register, GAP_BITS = 2); 908 LUT (yosys), 538 FF, 0 DSP, 0 BRAM, 108.27 MHz after place and route (PR #10919, epic). Proof: a new test (13 in all, 28 asserts for it) runs the machine at the real divider 868 against a mid-bit receiver and counts 0 framing errors; reflashed: 550 of 550 lines PASS, 31 of 31 conversions, the control 282 FAIL and 268 PASS, exactly the 16 predicted conversions. A related instrument defect: an earlier "3252 of 3252" for M1b came from a capture split between files by stray readers and was retracted; only clean single-reader captures count (572 of 572) (epic, M1b row).
- Next step: every board top gets a test at its real bit divider with a mid-bit receiver that counts framing errors (the new test is the pattern), and a capture rule of one reader per port with the port free before each capture.

### W12. The comparator offset needed a fitted number until its source was found

- Status: REDUCED (the source is found and measured; it is not yet derived).
- Evidence: M4e (PR #10854) matched 30 of 31 conversions only with two fitted numbers, a top-plate load Cp of 0.63 units (77 fF) and an offset of +6.7 mV; as merged it matched 0 of 31 (model minus ngspice -21 to +12 codes, a straight line of gain 1.0199 and offset 4.60 codes) (spec:sar_adc). M4b-psp (PR #10884) showed the golden latch itself is not that offset: its flip point is 0 (model +0.12 uV, ngspice within 0.12 uV), and -0.153 mV with its loads. A first reading of the decision point (+2.5 to +3.1 mV) was taken inside the clock-edge kickback and had the wrong size; read before the edge it is 6.7 to 6.9 mV (spec:sar_adc). PR #10953 (spec:sar_kick, 10 tests) changes one thing at a time in ngspice with PSP103: base +9.42 mV at the first decision; ideal tracking switches +9.19 (moved by -0.23 +- 0.40); buffers between plates and comparator +0.03 (-9.38); both halves' bottom plates ideal +0.27 (-9.14); a resistor of one transmission gate's size (2592 ohm) added to each P plate -0.89 (-10.31). Mechanism: when the comparator clock rises, its input devices push charge into the two top plates, VDP falls by 22.7 mV and VDN by 30.2 mV, and the N half's bottom plates sit behind one more transmission gate than the P half's. Late decisions (the ones that set the codes): 6.66 mV (decision 9) and 6.90 mV (decision 10); the model takes their mean, 6.776 mV. With it, and Cp = 0.63: 29 of 31 conversions (misses: conversion 16, +1 code, and 25, -1 code); with Cp refitted alone, 30 of 31 at Cp = 0.634. Count by offset at Cp = 0.63: 6.6 mV 29, 6.7 mV 30, 6.9 mV 30, 7.0 mV 25, 7.1 mV 21.
- Next step: #10942: a circuit-level model of the plates' drive (gate and inverter resistances from the M3b tables, the DAC capacitors, the charge pulse from the M3c tables, solved by M1's step) with the six first-decision rows as targets.

### W13. The cost and the hosts of the reference runs

- Status: REDUCED.
- Evidence: the golden SAR deck, 16 us, 31 conversions, ngspice-47 with PSP103: 3894.9 s wall (65 minutes) on the shared laptop (spec:sar_adc); 18 minutes on a remote Docker host, 10 of 12 cores, with the same 161633 points, reproducing the pinned held plates to 0 uV from conversion 2 on (conversion 1: 7 and 12 uV; conversion 0, the start-up from .ic: 1.1 and 2.0 mV) (ledger, tick 28); on a slice, option klu with 8 threads 153 s against 275 s by default, 1.8 times [I] (epic). The toolchain cost something too: OpenVAF's bundled libraries had invalid code signatures on the laptop and the process ended with "Killed: 9" until they were re-signed (ledger, tick 8), and the laptop's disk reached 100 % with 251 MB free from other sessions' build folders, after which no build lane starts below 8 GB free (epic). `tri remote` (#11020, PR #11055) now runs a golden job on any ssh host from fixed command templates written in a spec, and #11040 plans one lab service; the host is never written in the repository. The ngspice options of the 18 minute run: not recorded [U].
- Next step: record host class, cores, ngspice options and wall time of every reference run in its receipt; measure the build time too (W7).

### W14. Fitted versus predicted

- Status: REDUCED (one of two fitted numbers is now measured; none is derived).
- Evidence: row 2 of spec:sar_adc (30 of 31) was fitted on all 31 conversions and "describes this run"; the held-out checks are 14 of 15 (fit on the 16 even conversions, scored on the 15 odd) and 15 of 16 (the reverse). The optimum is narrow: Cp moved by -0.02 or +0.02 units leaves 23 or 20 of 31 matches, the offset moved by -1 or +1 mV leaves 11 or 6 (spec:sar_adc). Cp (0.63 units of 122.94 fF, 77 fF) is still a fit: the first plate step gives it from 0.58 to 0.85 units depending on how the plates are driven (spec:sar_kick). The offset is measured on the netlist (6.776 mV), not derived. The board's 30 of 31 equals its spec's fitted model: "describes this run and predicts nothing" (spec:sar_adc_ax7203). Other fitted numbers: SL_TG_IN_LOAD = 0.43 (a transmission gate's input load, set so the CK edges come out right, which makes CK11 to CK2 not an independent check), the delay cell's input loads and the 4 fF and 8 fF equivalents "fitted, not derived", SL_C2_PM fitted at one point (spec:sar_logic). The gate arcs d0 and k are measured by least squares over three loads, not fitted to the result they are judged by (spec:sar_logic). All SAR evidence is one deck, one sine, one corner (mos_tt, cap_typ) and one temperature (50 C).
- Next step: after #10942 and Cp from the M3c charge tables, re-score with nothing fitted on conversions the fit never saw (another sine amplitude or phase); every spec lists its constants as fitted, measured or derived, and a claim of "matches" names which.

### W15. The reference is not unique at the LSB

- Status: OPEN.
- Evidence: conversion 0 gives code 1006 in our ngspice-47 run and 1007 in the published ngspice-46 RAW of the same deck; the wrapper decks of the offset experiments also give 1007. The last decision of that conversion sits 0.006 mV from its flip (D = -6.889 mV, flip -6.895 mV): "a 0.01 mV difference between two simulations of the same circuit decides it" (spec:sar_kick). Otherwise 30 of 31 codes are identical between the two runs, the held plates agree to 0.007 mV and 0.012 mV from conversion 1 on, and the whole traces differ by at most 1.1 mV (VDP) and 1.3 mV (VDN) from 600 ns on, in edge timing (spec:sar_adc). The offset's spread by decision (6.66, 6.90, 7.10 and 6.98 mV for decisions 9, 10, 8, 7) is the size of what separates 29 from 30 matches (spec:sar_kick). The M4 acceptance of the epic, "every conversion code ... equals ngspice's", is therefore at the mercy of 0.01 mV.
- Next step: state the acceptance as "within one code, and the distance of each miss from its threshold in LSB" (the spec already gives 0.09 LSB for conversion 25), with the distances of all 31 conversions tabulated.

### W16. The board runs the behavioural converter, not the circuit

- Status: OPEN.
- Evidence: in the SAR top on the AX7203 the held samples, the DAC words and the pinned codes are ROMs, and the machine is a sign comparator plus the MSB-first sequence: 75 clocks per conversion, 908 LUT, 538 FF, 0 DSP, 0 BRAM; "the board holds ROMs of the samples: it does not sample anything" (spec:sar_adc_ax7203, PR #10919). The nonlinear solvers (inverter with charge, comparator, switch) are specs run by t27c test and t27b test; no board top contains one, and the switch spec says that none of it has been through t27b verilog (spec:sar_switch, spec:mos_psp103). Timing margin is thin and not stable: the SAR top placed at 124.60 MHz and, after 8 more flip-flops, at 108.27 MHz against a 100 MHz constraint on the default seed (PR #10919); M1b's controls placed at 87.84 and 99.32 MHz against 106.30 for the committed top (PR #10578).
- Next step: tables in BRAM (after #10881 and #10882), then the inverter and the comparator step on the fabric with receipts that report LUT, DSP, BRAM, Fmax and clocks per step.

### W17. Gate-level logic is measured delays with fitted numbers

- Status: REDUCED.
- Evidence: spec:sar_logic (M4d, PR #10845; M4d-b, PR #10898, #10848): 113 gates, CK11 to CK1 in the same order as ngspice with PSP103. The first version used settled delays and the repeated comparator-clock falls were 83 to 127 % late, because the delay cell remembers its last edge (#10848); with age tables measured in ngspice the repeated falls are within 1.8 % and the rises within 0.6 % (bound 8 %), seven held-out pulse trains within a worst case of 5.1 %. The CK edges were first 17 % early (458 ps against 548 ps); two causes were found, a drive-strength fight in the skew window (+47 ps) and a transmission gate's input load, which is one fitted number (+41 ps). Not covered: pulses with a high time under 14 ns or a low time under 16 ns (the tables are flat there and a scratch prototype was 17 % off), trim codes other than 000 (scaled from 000), any corner but tt at 50 C, and the closed loop with the comparator (spec:sar_logic). This is not a transistor-level model.
- Next step: close the loop with the comparator's own Valid at device level (M4e) and calibrate another trim code.

### W18. No mismatch, noise or Monte Carlo

- Status: OPEN.
- Evidence: every spec header closes with a list of what it does not say, and noise, mismatch and Monte Carlo are in it (spec:sar_comparator_psp, spec:sar_adc, spec:mos_psp103, spec:sar_logic). The comparator's imbalance is a deterministic multiplier: m of 35 or 34 instead of 36 on one input device gives flip points of +4.2665 and +8.689 mV in ngspice, so a 4.4 % imbalance would give the converter's 6.7 mV (spec:sar_comparator_psp). The IHP corner files carry mismatch and statistical sections (section 5.6) that no spec uses. One conversion of the 31 is decided 0.133 mV (0.09 LSB) from a threshold (conversion 25, spec:sar_adc), so a mismatch of that size changes codes [I].
- Next step: a non-claim in the v0.1 release notes; later, a per-device multiplier as an input of the table solver and a distribution run on a remote host.

### W19. Toolchain debts found on the way

- Status: REDUCED.
- Evidence: the compiler PR #8844, "gen-verilog: an entry parameter named clk/rst_n/en is declared once; a void body's last call stays a statement", merged 2026-10-19 at 12:22 UTC; changes to the seals of analog_AnalogSar* were held until it landed, so PR #10893 and PR #10953 waited as drafts and merged at 13:43 and 13:48 UTC (ledger, ticks 21 and 27; GitHub close times). Still open: #10749 (frontier closure stale forever for specs two imports deep), #10785 (gen-rust lowers a local array passed to a mutable slice parameter to &mut &(w), which rustc refuses; one stamp in spec:spice_convert is written again because of it), #10920 (@bitCast between f64 and u64 is printed as written; spec:raw_writer builds the pair from arithmetic instead). t27b verilog refuses signed types, so every analog word is sign-magnitude (spec:linear), and array indexing, so the ROMs are if-chains (spec:sar_adc_ax7203). Two type-name collisions, Ring and Range, moved the types ratchet on master and were renamed (#10539, #10892).
- Next step: close #10749, #10785 and #10920; run the types ratchet in every review of an analog spec.

### Competitor delta since 2026-10-18

Method: searches run on 2026-10-19 (WebSearch, extended mode for FPGA solvers, AMS emulation, VACASK, ngspice and FPGA circuit emulation, standard mode for the vendor lookups), then WebFetch of the primary page or of the public GitHub API for each hit, so most items are [V]. The window is one day long. The search tool is US-only and vendor spec pages carry no dates.

Result: nothing new dated after 2026-10-18 was found for any of the items below. The positioning of sections 0 and 7 stands unchanged. What follows is the state of each tracked item on 2026-10-19, and three items the first pass missed that predate 2026-10-18.

- FPGA circuit solvers for power electronics. Typhoon HIL: newest blog post 2026-10-14 ("De-Risking BESS Projects"), none on the solver or on new hardware [V] (src:typhoon-blog). OPAL-RT: the eHS product page, undated, lists a 32 ns minimum time step, up to 500 kHz switching, 625 ps oversampled sampling and a "250-node grid @ 500 ns", and does not state the number format [V] (src:opalrt-ehs-page); this differs from the 70 ns Gen5 figure of the documentation page cited in section 2.2, so the two vendor pages still disagree and nothing in it is new. NI: an e4ds item dated 2026-10-16 about the NI ITS inverter test system says "FPGA-based real-time simulation" with no step or solver, full text paywalled [V, excerpt] (src:e4ds-ni-its). Plexim: nothing new; the Nanostep 4 ns figure is from a search summary [S].
- Academic FPGA solvers. Missed by the first pass, published 2026-01-29: Liang, Wu, Zhao and Huang, "A General FPGA-Based Accelerated Solver for Electromagnetic Transient Simulations", Electronics 15(3):606, an offline power-system accelerator checked against a CPU-only platform; Crossref metadata and abstract excerpt read, full text not (403) [V for metadata, U for numbers] (src:mdpi-emt). Also missed, submitted 2026-07-01: arXiv 2607.00841, an FPGA emulator of silicon-photomultiplier detector signals (one time bin per clock, one-pole recursive filters, DACs at 2.5 GS/s), not a circuit solver [V, abstract] (src:arxiv-sipm). Neither is IC-level.
- AMS emulation and real-number modelling. Cadence: no 2026 announcement for the Palladium real number modeling app; its product page still lists it [S] (src:cadence-palladium-page). Synopsys: only the 2018 to 2019 DARPA AMS emulation items on ZeBu, nothing from 2026 [S] (src:synopsys-darpa). Siemens: no Veloce analog or RNM source, only the Veloce CS launch article [S] (src:siemens-veloce-cs). Real-number modelling: no new source.
- VACASK (open Verilog-A simulator, University of Ljubljana, relevant to issue #10911). Newest release 0.3.4 on 2026-09-13, then 0.3.4.rc1 on 2026-07-02, 0.3.3 on 2026-06-10, 0.3.2 on 2026-01-21; no release after 2026-10-18 [V] (src:vacask-releases). NLnet grant started 2026-03; planned work: AC stability, S-parameters, transient noise, adjoint small-signal and noise analyses, PyOPUS integration [V] (src:vacask-nlnet). A pre-release 0.3.4.dev0 binary package on PyPI is a search result only [S] (src:vacask-pypi). 0.3.4 is the version a comparison run would pin [I].
- ngspice. Newest release 47 on 2026-08-11 (pyplot command, VDMOS soft recovery model, .wav support), before it 46 on 2026-03-29; no item after 2026-10-18 [V] (src:ngspice-news). The runs in this repository use ngspice-47; the golden deck's published RAW was made with 46 (spec:sar_adc).
- Xyce. The homepage says "Xyce 7.10 has been released" without a date; the GitHub release Release-7.10.0 is dated 2025-07-21 and no 2026 release exists [V] (src:xyce, src:xyce-gh-releases).
- OpenVAF (Reloaded fork). One release, tagged _20260610, published 2026-06-10; newest commits 2026-10-14 ("Updated readme") and 2026-10-13 ("Treat limexp as an ordinary builtin, not an analog operator"); nothing after 2026-10-18 [V] (src:openvaf-gh-releases, src:openvaf-gh-commits).
- Stanford msdsl and anasymod. The closest open prior art is dormant: the newest commit of anasymod is 2021-07-28 and of msdsl 2024-04-02 [V] (src:anasymod-gh-commits, src:msdsl-gh-commits).
- cpphdl analog. Active: five commits on 2026-10-17 and 2026-10-18 (Verilog-A export and generated model catalog, 10GBASE-R SerDes models with dual-solver regressions, a 1G PAM5 prototype with an IHP repeater flow, a README clarification at 2026-10-18 16:07 UTC), none after that [V] (src:cpphdl-gh-commits). The Verilog-A export and the 10GBASE-R path are already in section 5.4; no FPGA claim appears in what was read.
- IHP-Open-PDK. Newest commit on the IHP-GmbH default branch list is 2026-09-01 ("Dev -> Main", changelog entries since v0.3.0) [V] (src:ihp-gh-commits). Our pinned commit 21133f21 resolves to a merge dated 2026-09-08 from a fork's upstream-sync branch [V] (src:ihp-pin-commit); whether its model files equal upstream's is not checked [U].

Consequence for this file: none for the claims of sections 0 to 8. For the next refresh: re-read the VACASK release list, the ngspice news page and the cpphdl commit list; read the full text of the 2026 electromagnetic-transient paper if the publisher page opens.
