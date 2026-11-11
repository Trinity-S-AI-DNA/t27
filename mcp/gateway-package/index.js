// T27 gateway Host half: serves the gateway audit tail (law-encoded lines,
// hashes only) as a JSON resource for the dashboard; reads the gateway's OWN
// audit log -- no arbitrary file access, no secrets.
import { readFileSync } from 'node:fs';

const AUDIT_LOG = '/Users/playra/t27/.claude/worktrees/mcp-m0/.trinity/gateway-audit.log';
const TAIL_BYTES = 4096;

export function apply() {
  return {
    inject: ['webServer'],
    apply(ctx) {
      const serveAudit = () => {
        let lines = [];
        try {
          const raw = readFileSync(AUDIT_LOG, 'utf8');
          lines = raw.split('\n').filter(Boolean).slice(-40);
        } catch {
          lines = [];
        }
        return new Response(JSON.stringify({ lines }), {
          headers: { 'content-type': 'application/json' },
        });
      };
      return ctx.webServer.register({
        resource: '/t27-gateway/audit',
        handler: (request) => serveAudit(request),
      });
    },
  };
}
