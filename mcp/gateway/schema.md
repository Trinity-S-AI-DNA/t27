# M1 Gateway — audit event schema (V1)

One line per decision, tab-separated, seven fields, fixed order. Field codes
are decimal; hashes are decimal FNV-1a 32 computed in u64 lanes (the dialect
has no wrapping arithmetic; see `policy.t27` `request_hash`).

```
0        1      2     3      4         5       6
ts       actor  tool  scope  decision  reason  request_hash
u64      str    str   str    u8        u8      u64
```

| # | Field | Meaning |
|---|---|---|
| 0 | `ts` | epoch seconds of the decision |
| 1 | `actor` | caller identity ("queen", "bee-<id>", "claude-<session>") |
| 2 | `tool` | exactly a row of the spec's `TOOLS` table: `gateway.status`, `dsh.status` or `cdp.observe` |
| 3 | `scope` | the loopback endpoint served, e.g. `3081` or `9222` |
| 4 | `decision` | `0` = `ALLOW`, `1` = `DENY`, `2` = `DENY_RATE` |
| 5 | `reason` | `0` when allowed; otherwise the denied op code (`10` spawn, `11` file write, `12` credential, `13` browser act, `14` self-modify, `255` unknown) |
| 6 | `request_hash` | fingerprint of the raw request bytes |

Line terminator: `\n` (byte 10). The record always has 7 fields.

## Secrecy rules

- The schema has **no field a credential could hide in**: field 6 is a hash;
  fields 1–3 are structural. A request that carries a secret never reaches
  the log with the secret — only its hash does.
- No token, cookie, key or endpoint credential value may be written into any
  field by the gateway; the gateway does not hold credentials (they stay
  with the human-side session outside gateway state).

## Example

```
1762798000	queen	cdp.observe	9222	0	0	2854501129
1762798005	bee-7	dsh.status	3081	1	12	193827144
```

Line 1: allowed observe call. Line 2: credential access attempted through
`dsh.status` — denied with reason 12, audited.
