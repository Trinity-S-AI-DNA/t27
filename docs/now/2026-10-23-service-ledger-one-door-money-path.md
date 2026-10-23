# NOW -- service_ledger v3: one door for the money path (2026-10-23)

## service_ledger v3: settle_service, one door for the money path (Closes #11790)

- specs/automation/service_ledger.t27 (NEW, v1..v3): four services (course,
  operation, node-job, resale), one receipt gate, books derived from entries
  only, and `settle_service` running STOP -> receipt gate -> payout eligibility
  -> books arithmetic -> delivery in one fixed order; verdicts SETTLE_STOP /
  NONE / HOLD / PAY; 24 tests, 2 invariants, vacuous 0
- specs/automation/token-wallet.t27 v2: the three-unit trade (TRADE_UNITS 3;
  one unit is the price, one funds the bounty pool, one pays the referrer);
  the house's own price per operation does not move
- specs/verified/payout.t27: `trade_split_fits` -- refuses a client-side split
  exceeding the charge, subtracting so a share near the top of u64 cannot wrap
- all three seals regenerated and verified; L3 ASCII clean; cross-repo links
  checked live (trinity-fpga settlement_law.t27 + ledger.zig, 999-multibots
  host files); claim status stays DESIGN (no host, TRI_PAYABLE false)
