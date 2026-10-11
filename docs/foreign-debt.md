# Foreign Line Debt Report

## Current Status

| Language | Hand-written Lines |
|----------|-------------------|
| c | 142 |
| config | 3497 |
| javascript | 1572 |
| lean | 20742 |
| python | 126513 |
| rust | 216798 |
| shell | 3252 |
| typescript | 4028 |
| verilog | 29580 |
| zig | 2407 |
| **Total** | **408531** |

## Exclusions
- Generated code (gen/, build/, etc.)
- Vendored dependencies
- Build output
- Third-party libraries

## Goals
- Foreign lines should only decrease over time
- Each golden PR should be net-negative in foreign lines
- Eventually eliminate all hand-written foreign code
