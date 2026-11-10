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

## Tools (exactly three, read-only)

`gateway.status`, `dsh.status`, `cdp.observe` -- loopback-only endpoints,
dry-run/observation semantics, policy refusals recorded. No shell, no file
writes, no credentials, no browser mutation, no policy self-modification.

## Rollback

`plugin_manager remove_bundle @local/t27-gateway` removes the bundle and
its three tools from every session in the profile.
