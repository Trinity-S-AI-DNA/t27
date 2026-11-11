// T27 gateway dashboard: unified stream + filters + live tail.
// Presentation only: the stream is already law-encoded at write time
// (specs/mcp policy + log contract); this panel filters and renders.
window.__ModuleLoader__.load({
  id: '@local/t27-gateway',
  factory(require) {
    const React = require('react');
    const h = React.createElement;

    const TOKEN = 'var(--dsw-alias-color-text-secondary, inherit)';
    const DENYC = 'var(--dsw-alias-color-status-danger, #c0392b)';
    const BOX = { border: '1px solid var(--dsw-alias-color-border, transparent)',
      borderRadius: 8, padding: '8px 10px', margin: '4px 0',
      background: 'var(--dsw-alias-color-background-secondary, transparent)',
      fontFamily: 'inherit' };

    function Dashboard() {
      const [events, setEvents] = React.useState([]);
      const [live, setLive] = React.useState(true);
      const [tool, setTool] = React.useState('');
      const [status, setStatus] = React.useState('');
      const [text, setText] = React.useState('');
      const [detail, setDetail] = React.useState(null);

      React.useEffect(() => {
        let alive = true;
        const tick = () => {
          fetch('/api/t27-gateway/audit').then(r => r.json()).then(d => {
            if (alive) setEvents(d.events || []);
          }).catch(() => {});
        };
        tick();
        const t = setInterval(tick, 10000);
        return () => { alive = false; clearInterval(t); };
      }, []);

      const filtered = events.filter(e =>
        (!tool || e.component === tool) &&
        (!status || String(e.status) === status) &&
        (!text || e.line.indexOf(text) !== -1));

      const sel = { fontSize: 11, padding: '2px 4px',
        background: 'var(--dsw-alias-color-background-primary, transparent)',
        color: 'inherit', border: '1px solid var(--dsw-alias-color-border, transparent)' };
      const inp = { ...sel, flex: 1, minWidth: 80 };

      return h('div', { role: 'region', 'aria-label': 'T27 gateway dashboard', style: BOX },
        h('div', { style: { display: 'flex', justifyContent: 'space-between', marginBottom: 4 } },
          h('strong', { style: { fontSize: 12 } }, 'T27 gateway'),
          h('span', { style: { fontSize: 11, opacity: 0.7 } },
            `${filtered.length}/${events.length} · host:local · policy v2`)),
        h('div', { style: { display: 'flex', gap: 6, marginBottom: 4, flexWrap: 'wrap' } },
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
          h('input', { value: text, placeholder: 'filter text…', onChange: e => setText(e.target.value), style: inp }),
          h('button', { onClick: () => setLive(!live), title: 'live tail refreshes every 10s',
            style: { ...sel, cursor: 'pointer' } }, live ? 'live ⏸' : 'paused ▶')),
        filtered.length === 0
          ? h('div', { style: { fontSize: 11, opacity: 0.6 } }, 'no matching gateway calls')
          : h('div', {}, filtered.slice(-12).map((e, k) => h('div', { key: (e.line || '') + k,
              onClick: () => setDetail(e), style: {
                display: 'flex', gap: 8, fontSize: 11, lineHeight: '16px', cursor: 'pointer',
                color: TOKEN, borderBottom: '1px dotted var(--dsw-alias-color-border, transparent)' } },
            h('span', { style: { flex: '0 0 auto', opacity: 0.7 } }, e.component),
            h('span', { style: { flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' } }, e.scope || ''),
            h('span', { style: { flex: '0 0 auto', fontWeight: e.status === 1 ? 600 : 400,
              color: e.status === 1 ? DENYC : 'inherit' } }, e.status === 1 ? 'DENY' : 'ALLOW')))),
        detail && h('div', { style: { marginTop: 6, fontSize: 11, whiteSpace: 'pre-wrap',
          background: 'var(--dsw-alias-color-background-primary, transparent)',
          padding: 6, borderRadius: 6 } },
          h('div', {}, 'detail:'),
          h('div', {}, detail.line)));
    }

    return {
      inject: ['slots'],
      apply(ctx) {
        ctx.slots.inject('conversation.composer.dock', () => ctx.slots.register({
          name: 'conversation.composer.dock', id: 't27-gateway-dashboard', order: 6,
        }, Dashboard));
      },
    };
  },
});
