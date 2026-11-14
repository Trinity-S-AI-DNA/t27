// T27 gateway Host half: serves the gateway audit tail (law-encoded lines,
// hashes only) as a JSON resource for the dashboard. Reads the gateway's OWN
// audit log -- no arbitrary file access, no secrets, no absolute paths (the
// log is located by walking up from this package to the checkout's .trinity
// directory). Route registration follows the verified working pattern from
// packages/experimental/inspector/src/host/plugin.ts:
// ctx.connection.fetch.register({path, methods, fetch}).
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

function auditResponse() {
  let raw = '';
  try { raw = readFileSync(findAuditLog(), 'utf8'); } catch { raw = ''; }
  const events = raw.split('\n').filter(Boolean).map(line => {
    const f = line.split('\t');
    return {
      timestamp: Number(f[0]) || 0,
      host: 'local',
      source: 'gateway-audit',
      component: f[2] || '',
      status: Number(f[4]) || 0,
      reason: f[5] || '',
      request_hash: f[6] || '',
      line,
    };
  });
  return new Response(JSON.stringify({ host: 'local', policy: 3, count: events.length, events }), {
    headers: { 'content-type': 'application/json' },
  });
}

function healthResponse() {
  let auditOk = false;
  try { readFileSync(findAuditLog(), 'utf8'); auditOk = true; } catch { auditOk = false; }
  return new Response(JSON.stringify({
    host: 'local', idempotent: true, mutating: false, policy: 3,
    checks: [
      { name: 'gateway-audit-log', ok: auditOk },
      { name: 'dsh-web-3081', ok: null, note: 'probed by the tri-mcp gateway.health tool' },
      { name: 'ladder-3131', ok: null, note: 'probed by the tri-mcp gateway.health tool' },
      { name: 'caddy-route', ok: null, note: 'vps-only: not checkable from local' },
    ],
    anomalies: [!auditOk && 'audit log unreadable'].filter(Boolean),
  }), { headers: { 'content-type': 'application/json' } });
}

export const inject = ['connection'];

export async function apply(ctx) {
  ctx.connection.fetch.register({
    path: '/api/t27-gateway/audit',
    methods: ['GET'],
    requestBody: 'buffered',
    fetch: async (request) => auditResponse(),
  });
  ctx.connection.fetch.register({
    path: '/api/t27-gateway/health',
    methods: ['GET'],
    requestBody: 'buffered',
    fetch: async (request) => healthResponse(),
  });
}
