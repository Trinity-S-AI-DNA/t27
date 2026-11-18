# T27 Chimera Metaprogramming Research

## Research Objective
Investigate whether DSH can treat every .t27 specification as a composable program unit and whether valid compositions of functions from multiple .t27 specs can generate new algorithms, workflows, and invariants for metaprogramming.

## Research Directory Structure
- `00_executive_summary.md` - High-level findings and recommendations
- `01_existing_t27_inventory.json` - Inventory of existing specs and their composability
- `02_spec_as_program_unit.md` - Analysis of spec components as program units
- `03_type_compatibility_analysis.md` - Type and contract compatibility rules
- `04_invariant_composition_rules.md` - Invariant composition and conflict resolution
- `05_effect_and_security_model.md` - Effect and security composition rules
- `06_actor_execution_architecture.md` - Actor-based execution architecture
- `07_chimera_examples.md` - End-to-end chimera examples
- `08_project_impact_assessment.md` - Impact on developer workflow and project
- `09_risks_and_failure_modes.md` - Risks and failure mode analysis
- `10_mvp_implementation_plan.md` - Minimum viable product implementation
- `11_decision_record.md` - Final decision and rationale

## Definition of Done
CHIMERA_RESEARCH_COMPLETE WHEN:
- every existing .t27 spec is classified as composable, partially composable, or non-composable;
- input/output compatibility rules are defined;
- invariant merge and conflict rules are defined;
- effect and approval composition rules are defined;
- actor-based execution architecture is proposed;
- at least three end-to-end Chimera examples are modeled;
- security, replayability, and evidence requirements are preserved;
- project impact, risks, and MVP scope are explicit;
- recommendation states build, do-not-build, or build-in-shadow-mode.

## Guardrails
- FORBIDDEN: implementing a new runtime, modifying existing .t27 specs, enabling autonomous destructive actions, weakening invariants during composition, silent provider fallback, execution without evidence and approval gates
- REQUIRED: read-only code and spec analysis, typed composition model, explicit invariant conflict detection, actor/supervisor integration plan, shadow-mode rollout recommendation