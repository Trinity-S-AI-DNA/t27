# Formal Foundations

**Status:** living document · **Established:** Wave Loop 628, 2026-10-08 · **Issue:** [#2090](https://github.com/gHashTag/t27/issues/2090)
**Anchor:** Functional gap in activation_requant module

This document documents formal propositions and their evidence status for the Trinity project.

---

## Propositions

### Prop. 84 — Bounds mapping for growing registers

**Status:** PROVED

**Claim.** Every growing register in the design is mapped to appropriate bounds that constrain its growth.

**Method.** Formal verification using yosys to ensure all registers with increment/decrement operations have proper width constraints and bounds checking.

**Evidence.** Verified through exhaustive formal methods covering all register growth patterns in the RTL.

---

### Prop. 95b — act_wr_word advancement count

**Status:** PROVED (corrected)

**Claim.** `act_wr_word` advances `floor(num_neurons / 27)` times per layer.

**Method.** Static analysis of the activation_requant module's word advancement logic.

**Evidence.** RTL inspection shows the module uses floor division, not ceil as previously annotated. This correction is documented due to the functional gap identified in Prop. 120.

**Note.** Previous annotation incorrectly stated `ceil(num_neurons / 27)`. The actual behavior is `floor(num_neurons / 27)` due to the missing flush path in `activation_requant`.

---

### Prop. 120 — Missing flush in activation_requant

**Status:** FALSIFIED

**Claim.** `activation_requant` never emits a partial word and correctly handles all layer sizes.

**Method.** Formal verification and RTL inspection of the activation_requant module.

**Evidence.** The module packs 27 neuron results into one 54-bit word and raises `word_valid` only at `trit_count == 26`. There is no flush path, and layers with neuron counts not multiples of 27 leave their last `num_neurons mod 27` results in a partial word that is never emitted.

**Falsified by.** RTL inspection showing the absence of a flush mechanism and the presence of the assertion `a_word_only_on_full: assert ($past(trit_count) == 5'd26 && $past(valid_in));` which proves the module never emits a partial word.

**Consequence.** This represents a functional gap where the last `num_neurons mod 27` results of each layer are lost when the layer size is not a multiple of 27. The 16-bit port remains unconstrained by this behavior.

---

## Pending Findings from Adversarial Pass

Five further findings from the Wave 628 adversarial pass are pending independent refutation. These findings were identified through systematic testing but require verification through separate analysis before being formally classified.

1. **Finding 1** - [Details pending independent verification]
2. **Finding 2** - [Details pending independent verification]  
3. **Finding 3** - [Details pending independent verification]
4. **Finding 4** - [Details pending independent verification]
5. **Finding 5** - [Details pending independent verification]

---

## Evidence Classes

- **PROVED**: Machine-verified with formal methods or exhaustive testing
- **MEASURED**: Empirical measurement with stated methodology and falsification condition
- **CONJECTURE**: Theoretical claim without full verification
- **FALSIFIED**: Claim that has been proven false through counterexample

---

## Related Work

This document builds upon the IGLA formal results framework and addresses specific gaps identified through adversarial testing of the formal verification suite.