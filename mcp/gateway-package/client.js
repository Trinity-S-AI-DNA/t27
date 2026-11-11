// T27 gateway dashboard: audit tail + law summary in the composer dock.
// Read-only; styles via --dsw-alias-* tokens; data via the Host resource.
window.__ModuleLoader__.load({
  id: '@local/t27-gateway',
  factory(require) {
    const React = require('react');
    const h = React.createElement;

    function Dashboard() {
      const [lines, setLines] = React.useState([]);
      React.useEffect(() => {
        let alive = true;
        const tick = () => {
          fetch('/api/t27-gateway/audit').then(r => r.json()).then(d => {
            if (alive) setLines(d.lines || []);
          }).catch(() => {});
        };
        tick();
        const t = setInterval(tick, 10000);
        return () => { alive = false; clearInterval(t); };
      }, []);
      const denied = lines.filter(l => l.split('\t')[4] === '1').length;
      const row = (l, k) => {
        const f = l.split('\t');
        return h('div', { key: k, style: {
          display: 'flex', gap: 8, fontSize: 11, lineHeight: '16px',
          color: 'var(--dsw-alias-color-text-secondary, inherit)',
        } },
          h('span', { style: { flex: '0 0 auto', opacity: 0.7 } }, f[2] || ''),
          h('span', { style: { flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' } }, f[3] || ''),
          h('span', { style: { flex: '0 0 auto', fontWeight: f[4] === '1' ? 600 : 400, color: f[4] === '1' ? 'var(--dsw-alias-color-status-danger, #c0392b)' : 'inherit' } }, f[4] === '1' ? 'DENY' : 'ALLOW'));
      };
      return h('div', { role: 'region', 'aria-label': 'T27 gateway dashboard', style: {
        border: '1px solid var(--dsw-alias-color-border, transparent)',
        borderRadius: 8, padding: '8px 10px', margin: '4px 0',
        background: 'var(--dsw-alias-color-background-secondary, transparent)',
        fontFamily: 'inherit',
      } },
        h('div', { style: { display: 'flex', justifyContent: 'space-between', marginBottom: 4 } },
          h('strong', { style: { fontSize: 12 } }, 'T27 gateway'),
          h('span', { style: { fontSize: 11, opacity: 0.7 } },
            `${lines.length} calls · ${denied} denied · policy v2`)),
        lines.length === 0
          ? h('div', { style: { fontSize: 11, opacity: 0.6 } }, 'no gateway calls yet')
          : h('div', {}, lines.slice(-12).map(row).reverse()));
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
