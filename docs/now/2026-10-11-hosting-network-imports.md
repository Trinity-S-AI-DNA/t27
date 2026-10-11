# NOW -- specs/hosting: the network MVP's quorum and pool imported, the TODOs closed (2026-10-11)

## specs/hosting/{proof,sybil,credit,statement}.t27 (gHashTag/trios#1761; network specs from #8788 and #8789)

- proof.t27 imports `safe_m` from network/quorum.t27 in place of its word-for-word copy of mvp_v2's formula; F stays 0 inside one job's replicas, because sybil.t27's anchor stands where the network's fault allowance does. Verdicts unchanged.
- sybil.t27 rule 6: an operator is credited once per job (network/quorum.t27 `operator_votes`, `OPERATOR_OWNER`); every owner-tier key is the owner. The anchored verdict keeps counting receipts, not operators: counted per operator, an owner-only anchor set cannot break a tie (owner's world, M = 20: jobs agreed 1897 -> 708, attacker keys suspended 160 -> 56). Lane B's tables (one key per operator) do not move; the attacker earns 0 with the defense on.
- credit.t27 keeps the per-receipt rate and shows, with network/pool.t27's own `part_of`, that the pool split pays a 2 mTRI shard nothing. statement.t27's TODO closed in comments; its wasm is unchanged.
