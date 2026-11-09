# t27

Grok plugin for a t27 checkout. Specs are the source. `t27c` checks them.
`t27b` runs them. The identity strings are `specs/tools/grok_plugin.t27`.

## Install

The plugins page is https://grok.com/dashboard/plugins. Sign in, add the
marketplace `Trinity-S-AI-DNA/t27`, then install `t27`.

Grok reads `.grok-plugin/marketplace.json` at the repository root. The package
is `plugins/t27`.

From the Grok Build CLI:

```
grok plugin marketplace add Trinity-S-AI-DNA/t27
grok plugin install t27 --trust
```

Install this directory without the marketplace:

```
grok plugin install Trinity-S-AI-DNA/t27#plugins/t27 --trust
```

## What it adds

- Skill `t27`, used when the workspace is a t27 checkout.
- Slash commands `/t27-check`, `/t27-test`, and `/t27-seal`.

The component list the dashboard shows before install is
`.grok-plugin/plugin-index.json`.

## Network and credentials

None. This plugin ships no MCP server, no hook, and no remote URL. The
commands run `t27c` and `t27b` in the checkout. Those binaries are not bundled.
No token is read.

Listing the same package in the public xAI catalog
(`xai-org/plugin-marketplace`) is a separate pull request that pins a commit
SHA. This repository is the source that request would point at. It is not
that submission.

## License

Apache-2.0, the same license as the repository.
