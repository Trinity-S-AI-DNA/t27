# 2026-10-11 - Hot-Reload Steward Skill

Added the `t27/hot-reload-steward` skill card to monitor performance regressions in the hot-reload loop.

## What was added

- **Skill specification**: `specs/skills/t27-hot-reload-steward.t27` - defines the skill card constants and seven monitored specs
- **Skill implementation**: `.claude/skills/hot-reload-steward/SKILL.md` - contains the steward's workflow for running tests and monitoring budgets
- **Agent updates**: 
  - Agent T now holds the skill (28 total skills)
  - Agent M now holds the skill as domain lead (6 total skills)
- **Documentation update**: `docs/agents/AGENTS_ALPHABET.md` updated to reflect the new skill counts

## Monitored specs

The steward monitors seven specs for performance regressions:
- `specs/tri/hot/hot.t27` - the `tri hot` command specification
- `specs/verified/digest_cache.t27` - digest cache performance
- `specs/verified/suite_jobs.t27` - suite job performance  
- `specs/verified/gen_hash_cache.t27` - hash generation cache performance
- `specs/verified/bitstream_reuse.t27` - bitstream reuse performance
- `specs/verified/watch.t27` - watch performance
- `specs/verified/bench_agent.t27` - benchmark agent performance

## Budgets monitored

- Warm frontier: 5000 ms maximum
- Bitstream reuse hit: 2000 ms maximum

## Scheduler deployment

The scheduler sees the card only after gHashTag/BrowserOS `trios/agent-server/scripts/sync-t27-specs.sh` re-vendors `specs/skills` and the agent server deploys.