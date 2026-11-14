// T27 gateway dashboard: embedded in conversation.input.dock (full-width entries
// above the composer card). Presentation only: the stream is law-encoded at
// write time (specs/mcp policy + log contract); this panel filters and renders.
// Host adapter: React rendering + slot registration. Pure logic (status labels)
// is generated from specs/gateway/gateway_logic.t27 via t27c gen-js.
window.__ModuleLoader__.load({
  id: '@local/t27-gateway',
  factory(require) {
    const React = require('react');
    const h = React.createElement;
    // GENERATED FROM specs/gateway/gateway_logic.t27 via `t27c gen-js`.
    // Do not edit by hand; regenerate and replace this block.
    const STATUS_ALLOW = 0;
    const STATUS_DENY = 1;
    const DEFAULT_POLICY = 3;
    function status_label(status) {
      if (!(Number.isInteger(status) && status >= -2147483648 && status <= 2147483647)) throw new RangeError("status out of i32");
      if ((status === STATUS_DENY)) { return "DENY"; }
      return "ALLOW";
    }
    function event_matches(tool, status, text_found, event_tool, event_status) {
      if (!(Number.isInteger(event_status) && event_status >= -2147483648 && event_status <= 2147483647)) throw new RangeError("event_status out of i32");
      if (((tool !== "") && (tool !== event_tool))) { return false; }
      if (((status !== "") && (status !== status_label(event_status)))) { return false; }
      if ((!text_found)) { return false; }
      return true;
    }
    const statusLabel = status_label;
    // generated event_matches (from .t27)
    const eventMatches = event_matches;

    const TOKEN = 'var(--dsw-alias-color-text-secondary, inherit)';
    const DENYC = 'var(--dsw-alias-color-status-danger, #c0392b)';
    const CTRL = { fontSize: 11, padding: '2px 6px',
      background: 'var(--dsw-alias-color-background-primary, transparent)',
      color: 'inherit', border: '1px solid var(--dsw-alias-color-border, transparent)',
      borderRadius: 4, cursor: 'pointer' };
    const PANEL = { boxSizing: 'border-box', width: '100%', maxWidth: 712, margin: '0 auto',
      border: '0px none rgb(249, 250, 251)',
      borderRadius: 28, padding: '8px 12px',
      background: 'rgb(44, 44, 46)',
      fontFamily: 'inherit',
      boxShadow: 'rgba(255, 255, 255, 0.12) 0px 0px 0px 0.5px, rgba(0, 0, 0, 0.03) 0px 4px 16px 0px, rgba(0, 0, 0, 0.03) 0px 0px 24px 0px',
      maxHeight: 240, overflowY: 'auto' };

    function GatewayDashboard() {
      const [collapsed, setCollapsed] = React.useState(true);
      const [events, setEvents] = React.useState([]);
      const [meta, setMeta] = React.useState({ host: 'local', policy: DEFAULT_POLICY });
      const [live, setLive] = React.useState(true);
      const [tool, setTool] = React.useState('');
      const [status, setStatus] = React.useState('');
      const [text, setText] = React.useState('');
      const [detail, setDetail] = React.useState(null);
      const [error, setError] = React.useState('');

      React.useEffect(() => {
        if (collapsed || !live) return undefined;
        const tick = async () => {
          try {
            const r = await fetch('/api/t27-gateway/audit');
            const status = r.status;
            if (!r.ok) {
              if (status === 401) {
                setError('unauthorized — valid session identity required');
              } else if (status === 404) {
                setError('not found — route /api/t27-gateway/audit not mounted');
              } else {
                setError('HTTP ' + status);
              }
              return;
            }
            const d = await r.json();
            setEvents(d.events || []);
            setMeta({ host: d.host || 'local', policy: d.policy || DEFAULT_POLICY });
            setError('');
          } catch (e) {
            setError(String(e.message || e));
          }
        };
        tick();
        const t = setInterval(tick, 10000);
        return () => clearInterval(t);
      }, [collapsed, live]);

      const filtered = events.filter(e => {
        const textFound = text === '' ? true : (e.line || '').includes(text);
        return eventMatches(tool, status, textFound, e.component, e.status);
      });

      if (collapsed) {
        return h('button', { onClick: () => setCollapsed(false),
          title: 'Open T27 gateway dashboard',
          style: { ...CTRL } },
          'T27 gateway');
      }

      const sel = { ...CTRL };
      const inp = { ...CTRL, flex: '1 1 120px', minWidth: 110 };

      return h('div', { role: 'region', 'aria-label': 'T27 gateway dashboard', style: PANEL },
        h('div', { style: { display: 'flex', justifyContent: 'space-between',
          alignItems: 'baseline', marginBottom: 6 } },
          h('strong', { style: { fontSize: 12 } }, 'T27 gateway'),
          h('span', { style: { fontSize: 11, opacity: 0.7 } },
            `${filtered.length}/${events.length} · host:${meta.host} · policy v${meta.policy}`),
          h('button', { onClick: () => setCollapsed(true), title: 'Collapse',
            style: { ...CTRL, marginLeft: 6 } }, '–')),
        error && h('div', { style: { fontSize: 11, color: DENYC, marginBottom: 4 } },
          'fetch error: ' + error),
        h('div', { style: { display: 'flex', gap: 6, marginBottom: 6,
          flexWrap: 'wrap', alignItems: 'center' } },
          h('select', { value: tool, onChange: e => setTool(e.target.value), style: sel },
            h('option', { value: '' }, 'all tools'),
            ['gateway.status', 'dsh.status', 'cdp.observe', 'gateway.metrics',
             'gateway.health', 'gateway.policy.audit', 'gateway.config.get',
             'gateway.routes.list', 'gateway.version', 'gateway.logs.head']
              .map(n => h('option', { value: n }, n))),
          h('select', { value: status, onChange: e => setStatus(e.target.value), style: sel },
            h('option', { value: '' }, 'all statuses'),
            h('option', { value: '0' }, 'ALLOW'),
            h('option', { value: '1' }, 'DENY')),
          h('input', { value: text, placeholder: 'filter text…',
            onChange: e => setText(e.target.value), style: inp }),
          h('button', { onClick: () => setLive(!live), title: 'live tail refreshes every 10s',
            style: { ...CTRL } }, live ? 'live ⏸' : 'paused ▶')),
        h('div', { style: { maxHeight: 168, overflowY: 'auto' } },
          filtered.length === 0
            ? h('div', { style: { fontSize: 11, opacity: 0.6, padding: '4px 0' } },
                error ? 'no events (fetch failed — see error above)'
                      : 'no matching gateway calls')
            : filtered.slice(-40).map((e, k) => h('div', { key: (e.line || '') + k,
                onClick: () => setDetail(e), style: {
                  display: 'flex', gap: 8, fontSize: 11, lineHeight: '16px', cursor: 'pointer',
                  color: TOKEN, borderBottom: '1px dotted var(--dsw-alias-color-border, transparent)' } },
              h('span', { style: { flex: '0 0 auto', opacity: 0.7 } }, e.component),
              h('span', { style: { flex: '0 0 auto', opacity: 0.7 } }, e.scope || ''),
              h('span', { style: { flex: '0 0 auto', fontWeight: e.status === 1 ? 600 : 400,
                color: e.status === 1 ? DENYC : 'inherit' } }, statusLabel(e.status))))),
        detail && h('div', { style: { marginTop: 6, fontSize: 11, whiteSpace: 'pre-wrap',
          background: 'var(--dsw-alias-color-background-primary, transparent)',
          padding: 6, borderRadius: 6 } },
          h('div', {}, 'detail:'),
          h('div', {}, detail.line)));
    }

    return {
      inject: ['slots'],
      apply(ctx) {
        ctx.slots.inject('conversation.input.dock', () => ctx.slots.register({
          name: 'conversation.input.dock', id: 't27-gateway-dashboard', order: 100, label: 'T27 gateway',
        }, GatewayDashboard));
      },
    };
  },
});