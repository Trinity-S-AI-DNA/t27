# @local/t27-gateway — DSH local plugin package

Observation-only MCP gateway as a Cordis configuration-only bundle: it
inserts the installed `@deepseek-ai/dsh-mcp-client` pointed at the `tri-mcp`
binary (stdio). The law lives in `../gateway/policy.t27` (generated into the
binary); this package only wires the transport.

## Install

DSH Web UI: Add plugin -> Local directory path ->
`<repo>/mcp/gateway-package` (this directory), or from a session with
rights: `plugin_manager install_bundle` with this directory as target.

Requires the `tri-mcp` binary built at the absolute path pinned in
`cordis.patch.json` (`command:`), and `cwd:` set to a t27 checkout (the
server resolves the repo root from its working directory).

## Tools (ten, all read-only)

Core: `gateway.status`, `dsh.status`, `cdp.observe` -- loopback-only
endpoints, observation semantics, policy refusals recorded.
V12 introspection: `gateway.metrics`, `gateway.health`, `gateway.policy.audit`,
`gateway.config.get`, `gateway.routes.list`, `gateway.version`,
`gateway.logs.head` (tail of the gateway's own audit log).
No shell, no file writes, no credentials, no browser mutation, no policy
self-modification.

## Rollback

`plugin_manager remove_bundle @local/t27-gateway` removes the bundle and
its three tools from every session in the profile.
