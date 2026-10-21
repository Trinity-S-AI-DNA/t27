#!/usr/bin/env python3
"""Verify the ONE exact Trinity identity this file defines, at 50 digits.

NAME. The filename says 152 and the docstring used to say "all 152 Trinity
formulas". `ALL_FORMULAS` has one entry, `S3_L5_TRINITY`, and had one entry on
BOTH sides of every conflict below -- so there is no lost commit in which it
held 152. The name is kept because renaming a file is a separate decision with
its own reference-checking; the claim is corrected here instead, where a reader
arrives before they arrive at the count.

MERGE RESOLUTION. This file was committed with SIX unresolved conflict markers,
two of them nested, and had been at HEAD in that state long enough to be
unparseable in every clean clone: `python3 -m py_compile scripts/verify_all_152.py`
exited 1 at line 4 with "SyntaxError: invalid syntax". No workflow, doc, or
source file references it, which is why nothing went red.

Neither side ran on its own, so the resolution is not "pick a side":

  * "Stashed changes" imported only `mp`, while line 15 below needs `mpf` --
    NameError on import-time evaluation of PHI. The `mp, mpf` import is taken
    from "Updated upstream" because it is the only one that works.
  * "Updated upstream" never defined `validated_deviations`, which the last
    line of the loop appends to -- NameError on the first PASS. That binding is
    taken from "Stashed changes" for the same reason.

Everything else is "Updated upstream" (f-strings, the fuller per-formula
print), chosen for being the more informative of two behaviourally equal sides.

WHAT WAS DROPPED, and why it is not a silent deletion. The upstream side ended
by writing a seal file:

    seal_dir = "/Users/playra/t27/research/seals"     # not this machine
    seal_file = os.path.join(seal_dir, "all_152_v0.2.sha")

That is removed rather than resolved. Two independent reasons: the path is
hardcoded to a user (`playra`) that does not exist here, so `os.makedirs` would
have manufactured a home directory for a nonexistent account; and the artifact
it wrote is named `all_152_v0.2.sha` while hashing `str(ALL_FORMULAS)` -- a
dict of ONE formula. A seal whose filename asserts 152 over a hash of 1 is
worse than no seal, because it is quotable. The digest is still PRINTED below,
so nothing is lost that a caller could not capture itself.
"""
import hashlib

from mpmath import mp, mpf

mp.dps = 50

PHI = (1 + mpf(5).sqrt()) / 2
E = mp.e
PI = mp.pi
GAMMA_PHI = PHI ** -3

# Exact formulas
EXACT = {
    "S3_L5_TRINITY": PHI**2 + PHI**(-2),
}

# Expected values
EXPECTED = {
    "S3_L5_TRINITY": (mpf("3"), mpf("0.0")),
}

ALL_FORMULAS = {**EXACT}

all_pass = True
deviations = []
validated_deviations = []
formula_dict = {}

for name, value in ALL_FORMULAS.items():
    print(f"[{name}]")
    print(f" Calculated: {value}")
    formula_dict[name] = str(value)
    # Was `expected, tolerance = EXPECTED.get(name, (None, None))`, which bound
    # `expected` to the FIRST ELEMENT of the pair, so `target, tol = expected`
    # below raised "TypeError: cannot unpack non-iterable mpf object" on the
    # first formula. That line sat OUTSIDE every conflict marker and is
    # character-identical on both sides, so it is not merge damage: the file has
    # never reached its own comparison. `tolerance` was read nowhere.
    expected = EXPECTED.get(name, (None, None))

    # EXACT identity check
    if name in EXACT:
        target, tol = expected
        exact_target = target[0] if isinstance(target, tuple) else target
        # Computed by the upstream side and read by neither side. Kept rather
        # than deleted so that the resolution stays reviewable against the
        # conflict it came from; the comparison below uses the literal 1e-40 /
        # 1e-45 thresholds, not this.
        exact_tol = tol if tol is not None else mpf("0")
        value_mp = value if isinstance(value, mpf) else mpf(value)
        diff_mp = value_mp - exact_target if isinstance(exact_target, mpf) else mpf(value - exact_target)
        diff = abs(diff_mp)
        if diff > mpf("1e-40"):
            print(f" FAIL: EXACT identity deviation {diff:.2e}")
            all_pass = False
        elif diff > mpf("1e-45"):
            print(f" WARNING: Small deviation {diff:.2e}")
        else:
            print(f" PASS: Exact identity (Delta = 0)")
            validated_deviations.append(0.0)

print("=" * 70)
print("=== SUMMARY ===")
print("=" * 70)
print(f"Total formulas: {len(ALL_FORMULAS)}")
print(f"Validated against experiment: {len(deviations)}")

seal_str = str(ALL_FORMULAS)
sha256_seal = hashlib.sha256(seal_str.encode()).hexdigest()
print(f"SHA256 seal: {sha256_seal}")
print("Seal NOT written to disk -- see the module docstring.")
