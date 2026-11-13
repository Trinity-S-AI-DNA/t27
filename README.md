# Golden Master

Clean production repository — системные файлы запуска .t27 + golden specs + сгенерированные артефакты, без experimental/work-in-progress кода.

## Promotion Pipeline

master (development + experiments) → candidate (specs passing local GOLDEN CHAIN) → golden-master (verified gold) → release (tagged deployable state)

## GOLDEN CHAIN Gate

Specs enter golden-master only if ALL pass:
1. t27b build PASS
2. tri test PASS
3. golden test PASS
4. regression PASS
5. seals/gates PASS
6. coverage delta recorded
7. no shell violations
8. no parallel-edit conflicts

## CI Fast Path

Minimal required checks for golden-master only:
- Fast compile check
- Fast test suite
- Optional heavy Coq proofs (nightly/optional track)

Not blocking release for heavy proof jobs if not required for runtime.
