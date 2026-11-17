// T27 gateway Host half: serves the gateway audit tail (law-encoded lines,
// hashes only) as a JSON resource for the dashboard; reads the gateway's OWN
// audit log -- no arbitrary file access, no secrets, no absolute paths
// (the log is located by walking up from this package to the checkout's
// .trinity directory, so the package survives worktree relocation).
import { readFileSync, existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const PKG_DIR = path.dirname(fileURLToPath(import.meta.url));

function findAuditLog() {
  let dir = PKG_DIR;
  for (let i = 0; i < 8; i++) {
    const cand = path.join(dir, '.trinity', 'gateway-audit.log');
    if (existsSync(cand)) return cand;
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return path.join(PKG_DIR, '.trinity', 'gateway-audit.log');
}

export function apply() {
  return {
    inject: ['webServer'],
    apply(ctx) {
      const serveAudit = () => {
        let lines = [];
        try {
          const raw = readFileSync(findAuditLog(), 'utf8');
          lines = raw.split('\n').filter(Boolean).slice(-40);
        } catch {
          lines = [];
        }
        return new Response(JSON.stringify({ lines }), {
          headers: { 'content-type': 'application/json' },
        });
      };
      const serveHealth = () => {
        let auditOk = false;
        try { readFileSync(findAuditLog(), 'utf8'); auditOk = true; } catch { auditOk = false; }
        return new Response(JSON.stringify({
          host: 'local', idempotent: true, mutating: false,
          checks: [
            { name: 'gateway-audit-log', ok: auditOk },
            { name: 'dsh-web-3081', ok: null, note: 'probed by the tri-mcp tools' },
            { name: 'ladder-3131', ok: null, note: 'probed by the tri-mcp tools' },
            { name: 'caddy-route', ok: null, note: 'vps-only: not checkable from local' },
          ],
          anomalies: [!auditOk && 'audit log unreadable'].filter(Boolean),
        }), { headers: { 'content-type': 'application/json' } });
      };
      return ctx.webServer.register([
        { resource: '/api/t27-gateway/audit', handler: (request) => serveAudit(request) },
        { resource: '/api/t27-gateway/health', handler: () => serveHealth() },
      ]);
    },
  };
}
