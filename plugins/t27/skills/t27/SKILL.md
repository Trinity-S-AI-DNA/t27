---
name: t27
description: Work in a t27 checkout. Use when the tree has specs or the user names t27, t27c, t27b, or t27.ai.
when-to-use: t27, t27c, t27b, .t27, t27.ai, golden chain
paths:
  - "**/*.t27"
  - "AGENTS.md"
user-invocable: true
---

# T27

This plugin is the Grok package for a t27 checkout. Identity strings live in
`specs/tools/grok_plugin.t27`. The checkout's `AGENTS.md` is the law. This file
does not restate it.

## When it applies

Use it when the workspace contains `AGENTS.md` and `specs/`, or the user names
t27, t27c, t27b, t27.ai, or a `.t27` file.

If those files are absent, say this workspace is not a t27 checkout and stop.

## What to read

1. `AGENTS.md` at the checkout root.
2. `.claude/skills/tri-pipeline/SKILL.md` for the commands that exist.

## What a change is

- Behavior goes in a `.t27` spec that has a `test` block.
- Do not hand-edit `gen/`.
- Do not add a new hand-written file in another language. `.md`, `.txt`, and
  `.json` are prose and data.
- First-party source is English and ASCII.

## Checks

From the repository root, use `target/release` when that binary exists, else
`target/debug`.

- `t27c check <spec>` parses and typechecks.
- `t27b test <spec>` runs tests when `t27b` is built. Exit 0 passes. Exit 1
  fails. Any other exit is not a verdict.
- `t27c test-report <spec>` prints the per-test table. `t27c test` only lists
  tests.
- `t27c seal <spec> --save` then `--verify`. Verification must say the hashes
  match.

Before `test-report` or `seal`, set `ZIG_GLOBAL_CACHE_DIR` and
`ZIG_LOCAL_CACHE_DIR` to directories under the temp directory.

A missing binary is a missing build. Say so. Do not invent a pass or a count.
A pass with zero runtime asserts is vacuous. Say so.

## What this plugin does not do

It calls no network endpoint. It reads no token. It ships no compiler. The
toolchain is built in the checkout. There is no MCP server and no hook.

## Dashboard

Grok loads the package from `.grok-plugin/marketplace.json`. The plugins page
is https://grok.com/dashboard/plugins. Add `Trinity-S-AI-DNA/t27`, then install
`t27`.
