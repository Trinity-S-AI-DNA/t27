---
name: ROADMAP (Pareto-scored task)
about: A task that earns the roadmap label under the Pareto rule in specs/queen/roadmap.t27
title: "[ROADMAP] "
labels: ["roadmap", "pareto"]
---

<!-- The rule, the score and the priority live in specs/queen/roadmap.t27. Do not restate them here. -->

## Boundary
<!-- One path per line, each holding a slash, e.g. specs/queen/x.t27 -->
- specs/

## User Scenarios
- **Given** the state before this task, **When** an agent or user does X, **Then** the observable outcome Y happens.

## Requirements
- FR-001: The change MUST ...

## Success Criteria
<!-- An outcome a machine can settle: a test count, an exit code, a file, a command. -->
- `t27c test-report specs/queen/x.t27` reports FAIL 0 and vacuous passes 0, with at least 3 tests.

## Pareto
title:
labels: roadmap, pareto
pareto_score: (impact x confidence / effort, 0.2 to 25.0)
impact: (1-5)
confidence: (1-5)
effort: (1-5)
priority: (P0-P3; P0 needs a hard gate or a recorded promotion)
owner_agent: (agent id or unassigned)
status: BACKLOG
depends_on: (issue numbers or none)
acceptance_criteria: (measurable, one per Success Criteria line)

## Non-goals
- What this task will not do.

## Risk
- What can go wrong and how it is caught.

## Evidence
- How it is verified: test, artifact, screenshot, log or metric.
