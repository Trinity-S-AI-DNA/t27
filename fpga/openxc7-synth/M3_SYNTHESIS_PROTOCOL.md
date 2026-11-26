# M3 HDMI Proof: SYNTHESIS CHECKLIST & PROTOCOL

**Status:** READY FOR SYNTHESIS PENDING VALIDATION  
**Protocol:** Strict validation → Synthesis → Flash → Visual Evidence

---

## 🎯 ACCEPTANCE CRITERIA

**M3 COMPLETE WHEN:**
1. ✅ **Synthesis successful** - bitstream generated without timing violations
2. ✅ **Board flashed** - AX7203 programmed with HDMI test bitstream
3. ✅ **Visual evidence** - monitor shows stable test pattern (photo/video)
4. ✅ **Protocol followed** - solid colors → color bars → moving patterns

**DO NOT CLAIM COMPLETE WITHOUT:**
- Bitstream generation proof
- Flashing success confirmation  
- Monitor display evidence
- Photo/video documentation

---

## 🔍 PRE-SYNTHESIS VALIDATION CHECKLIST

### Critical Issues (MUST RESOLVE)

#### 1. Hardware Documentation Priority
- [ ] **Locate AX7203 schematic/user manual**
- [ ] **Verify video output capability** (HDMI TMDS vs VGA RGB)
- [ ] **Confirm actual pin availability**
- [ ] **Check level shifter requirements**

#### 2. Pin Conflict Analysis  
- [ ] **Cross-check against KSZ9031 PHY pins**
- [ ] **Verify no JTAG header conflicts**
- [ ] **Check DDR3 and Ethernet signal overlaps**
- [ ] **Confirm clock signal assignments**

#### 3. Interface Design Verification
- [ ] **Correct interface type** (VGA vs HDMI TMDS)
- [ ] **Level shifter requirements**
- [ ] **Pull-up/down resistor needs**
- [ ] **DDC/HPD configuration**

### Completed Items ✅

- [ ] **640x480 @ 60Hz timing implementation**
- [ ] **200MHz → 25MHz pixel clock generation**
- [ ] **Test pattern generation** (solid → bars → moving)
- [ ] **UART control interface**
- [ ] **Synthesis automation scripts**
- [ ] **XDC constraint file**
- [ ] **RTL module integration**

---

## 📋 SYNTHESIS PROTOCOL

### Phase 1: Final Validation
```bash
# 1. Complete pin conflict resolution
# 2. Verify hardware documentation  
# 3. Update XDC based on actual capabilities
# 4. Run static timing analysis
```

### Phase 2: Synthesis
```bash
# 1. Run synthesis script
./synth_hdmi_test.sh

# 2. Review timing reports
# 3. Check for placement violations
# 4. Verify bitstream generation
```

### Phase 3: Board Test
```bash
# 1. Flash to AX7203
openFPGALoader -c digilent_hs2 --busdev-num 1:3 ax7203_hdmi_test_top.bit

# 2. Monitor connection
# 3. Capture test patterns
# 4. Document results
```

### Phase 4: Evidence Collection
```bash
# Required evidence:
# - Photo of monitor showing test pattern
# - Video of stable display
# - UART control demonstration
# - JTAG programming confirmation
```

---

## 🚫 STRICT PROTOCOL ENFORCEMENT

### NO EARLY COMPLETION
**DO NOT** claim M3 complete until:
- Bitstream synthesis succeeds
- Board flashes successfully  
- Monitor displays test pattern
- Visual evidence captured

### EVIDENCE REQUIREMENTS
- **Photo evidence:** Clear monitor display with test pattern
- **Video evidence:** Stable display for at least 10 seconds
- **Control evidence:** UART pattern selection demonstration
- **Status evidence:** JTAG programming confirmation

### FAILURE MODES
- **Pin conflicts** → Redesign XDC
- **Timing violations** → Optimize placement/routing
- **No display** → Verify interface type and connections
- **Unstable sync** → Check clock generation and timing

---

## 📊 STATUS TRACKING

| Task | Status | Next Step |
|------|--------|-----------|
| Design Implementation | ✅ COMPLETE | Validation |
| Static Timing Analysis | ✅ COMPLETE | Pin verification |
| Toolchain Preparation | ⏳ WAITING | Agent-1 completion |
| Documentation Review | ❌ PENDING | Locate schematic |
| Pin Conflict Resolution | ❌ BLOCKED | Need hardware docs |
| Synthesis Run | ❌ BLOCKED | Wait validation |
| Board Testing | ❌ FUTURE | Post-synthesis |
| Evidence Collection | ❌ FUTURE | Post-test |

---

## 🎯 PROTOCOL SUMMARY

**M3 HDMI Proof Status:** `DESIGN COMPLETE, VALIDATION PENDING`

**Next Critical Step:** Resolve pin documentation conflicts before synthesis

**Gatekeeper:** Must see actual AX7203 schematic to validate pin assignments

**Risk Level:** HIGH (potential pin conflicts without documentation)

**Completion Criterion:** Visual confirmation of stable test pattern on monitor

---

*Protocol Version: 1.0*  
*Created: 2026-11-26*  
*Enforcement: Strict - no milestone completion without evidence*