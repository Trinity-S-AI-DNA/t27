// T27 gateway Host half: serves the gateway audit tail as a STRUCTURED JSON
// resource for the dashboard. Lines are already law-encoded at write time
// (redaction/allowlist ran in the generated .t27 law); this handler only
// formats tab-separated fields -- no policy decisions here. Reads the
// gateway's OWN audit log; no arbitrary file access, no secrets.
import { readFileSync } from 'node:fs';

const AUDIT_LOG = '/Users/playra/t27/.claude/worktrees/mcp-m0/.trinity/gateway-audit.log';
const TAIL_BYTES = 65536;
const HOST_LABEL = 'local';

export function apply() {
  return {
    inject: ['webServer'],
    apply(ctx) {
      const serveAudit = () => {
        let raw = '';
        try { raw = readFileSync(AUDIT_LOG, 'utf8'); } catch { raw = ''; }
        const events = raw.split('\n').filter(Boolean).map(line => {
          const f = line.split('\t');
          return {
            timestamp: Number(f[0]) || 0,
            host: HOST_LABEL,
            source: 'gateway-audit',
            component: f[2] || '',
            status: Number(f[4]) || 0,
            reason: f[5] || '',
            request_hash: f[6] || '',
            line,
          };
        });
        return new Response(JSON.stringify({ host: HOST_LABEL, count: events.length, events }), {
          headers: { 'content-type': 'application/json' },
        });
      };
      return ctx.webServer.register({
        resource: '/api/t27-gateway/audit',
        handler: (request) => serveAudit(request),
      });
    },
  };
}
