// SPDX-License-Identifier: Apache-2.0
// PROPOSED regression tests for specs/policy/vf_write_gate.t27 + specs/t27f/vf_guard.t27
// (PR #14083), prepared 2026-11-17 by the P0 audit. NOT applied to the lane; paste after
// the owner-approved fixes (landing -> GR_ENGINE ceiling; bootstrap self-write removed).
// Both specs already carry the negative cases; these pin the ones that were RED.

module vf_gate_regression_tests;

// ---- into specs/policy/vf_write_gate.t27 (after the landing fix, 13/13 measured) ----

// landing must never exceed the engine ceiling, whatever the diff touches
test landing_is_capped_at_engine_even_for_owner_only_paths {
    var out : [256]u8 = undefined;
    let inp : str = "landing\n--\nM\tspecs/t27f/visual_factory_contract.t27\n";
    assert effective_role(inp, 52, 8) == GR_ENGINE;
    assert check_vf_writes(inp, 52, &out, 256) == 1;
}

// a landing PR may still write the gate-bootstrap check paths (engine layer)
test landing_allows_gate_bootstrap_check {
    var out : [256]u8 = undefined;
    let inp : str = "landing\n--\nM\tspecs/policy/vf_write_gate.t27\n";
    assert check_vf_writes(inp, 42, &out, 256) == 0;
}

// no role header at all falls back to agent: owner-only refuses
test missing_role_header_defaults_to_agent_and_refuses_owner_paths {
    var out : [256]u8 = undefined;
    let inp : str = "--\nM\tspecs/t27f/visual_factory_contract.t27\n";
    assert parse_role(inp, 41, 2) == GR_AGENT;
    assert check_vf_writes(inp, 41, &out, 256) == 1;
}

// ---- into specs/t27f/vf_guard.t27 (after the bootstrap fix) ----

// the permission table itself is never writable through the bootstrap shortcut
test the_gate_never_rewrites_its_own_law {
    assert gd_may_write(GR_AGENT, "specs/t27f/vf_guard.t27") == false;
    assert gd_may_write(GR_ENGINE, "specs/t27f/vf_guard.t27") == false;
    assert gd_may_write(GR_OWNER, "specs/t27f/vf_guard.t27");
    // the bootstrap exemption covers only the NEW policy file
    assert gd_may_write(GR_AGENT, "specs/policy/vf_write_gate.t27");
    assert gd_may_write(GR_ENGINE, "specs/policy/vf_write_gate.t27");
}

// ---- process rule (owner decision material, not a spec test) ----
// The two branch tests gate_bootstrap_check_paths and contract_is_owner_only are
// mutually exclusive as written. The owner picks: (1) owner-only vf_guard.t27 with a
// closed bootstrap phase (recommended; matches "cannot assign yourself owner"), or
// (2) time-limited bootstrap with an owner commit closing it.
