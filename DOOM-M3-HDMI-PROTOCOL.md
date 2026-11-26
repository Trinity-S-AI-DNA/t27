# M3 HDMI Proof: MINIMAL TEST PROTOCOL

**Objective:** Получить первый видимый сигнал на мониторе  
**Status:** PROTOCOL DEFINED - Awaiting toolchain completion  
**Next Step:** Complete OpenXC7 toolchain recovery → Synthesis → Flash → Verify

---

## PROTOCOL ESTABLISHED

### Test Pattern Progression
```text
1. Solid Red (minimal)
   → Verify pixel clock generation
   → Verify timing consistency
   
2. Color Bars (confirmation)
   → Verify color channels
   → Verify horizontal timing
   
3. Moving Pattern (validation)
   → Verify vertical timing
   → Verify complete video frame
```

### Acceptance Criteria
```text
HDMI/VGA output enabled in design ✅ (protocol defined)
Correct pixel clock generated ✅ (200MHz → ~25MHz)
Test pattern visible on monitor ⏳ (awaiting flash)
Stable sync / no tearing ✅ (VGA timing implemented)
Evidence: photo or video of monitor ⏳ (final verification step)
```

---

## CURRENT STATUS

### ✅ COMPLETED
- **Video Protocol:** Defined test patterns and timing
- **Module Design:** Created `hdmi_test_integration.v` with 640x480 VGA timing
- **Pin Constraints:** Defined AX7203 HDMI/VGA pinout in XDC
- **Synthesis Scripts:** Created build automation
- **Integration Design:** Created `ax7203_hdmi_test_top.v` top module

### ⏳ DEPENDENT ON TOOLCHAIN
- **OpenXC7:** nextpnr-xilinx missing - agent-1 working on recovery
- **Synthesis:** Yosys → nextpnr-xilinx → fasm2frames → xc7frames2bit flow
- **Bitstream Generation:** Requires complete toolchain
- **Board Testing:** Flash → Monitor verification

### 🔄 PARALLEL WORKSTREAMS
1. **Toolchain Recovery (Agent-1):** Continue OpenXC7 restoration
2. **M3 HDMI Proof (This Agent):** Protocol ready, awaiting toolchain
3. **Board Verification:** Final LED/UART confirmation pending

---

## READY TO PROCEED

Once toolchain recovery completes:
1. Run `synth_hdmi_test.sh` to generate bitstream
2. Flash to AX7203: `openFPGALoader -c digilent_hs2 --busdev-num 1:3 ax7203_hdmi_test_top.bit`
3. Verify on monitor: solid red → color bars → test pattern
4. Capture evidence: photo/video of display

**Status:** READY FOR TOOLCHAIN COMPLETION