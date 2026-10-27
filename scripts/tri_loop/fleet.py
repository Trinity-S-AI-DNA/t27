# tri fleet scan (#12170): prints the fleet law's live dated matrix and the pool's free capacity.
# One tiny probe per z.ai cell, one at a time, ~1 rps. The law is specs/queen/api_fleet.t27; the
# probe cell, the classifier and the seeded table are fleet_probe.py + generated code. Foreign:
# owner-approved 2026-10-25, session approval quoted in English in the PR body.
import os, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fleet_probe import classifier, pool, probe
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))) + "/gen/python/queen")
from api_fleet import ROUTES, PR_ZAI_API
PN = {0: "zai-coding-plan", 1: "z.ai", 2: "nvidia", 3: "free-claude-code", 4: "openrouter"}
MN = {0: "glm-5.3-flash", 1: "glm-5.3-highspeed", 2: "glm-4.5-flash", 3: "glm-4.7-flash", 4: "glm-5.3", 5: "glm-5.2:free"}
def main():
    cls, keys, alive, dmax, dn = classifier(), pool(), {}, 0, 0
    print("tri fleet scan " + time.strftime("%Y-%m-%dT%H:%MZ", time.gmtime()) + " | law specs/queen/api_fleet.t27 (#12170) | pool: " + (" ".join(k for k, _ in keys) or "none") + " | probe: claude -p x1 per cell, 1 at a time, ~1 rps")
    print("route | provider | key | model | verdict | ms | probe_date(UTC)")
    for r in ROUTES:
        mn = MN[r["model"]]
        for name, value in (keys if r["provider"] == PR_ZAI_API else [(None, "-")]):
            if r["provider"] != PR_ZAI_API:
                print("%d | %s | - | %s | %s (seeded, no host path) | - | %d" % (r["route_id"], PN[r["provider"]], mn, ("ALIVE", "REFUSED", "DEAD", "UNPROBED")[r["status"]], r["probe_date"]))
                continue
            code, mk, ms = probe(value, mn)
            v = cls(r["route_id"], code, mk)
            print("%d | %s | %s | %s | %s | %d | %s" % (r["route_id"], PN[r["provider"]], name, mn, v, ms, time.strftime("%Y%m%d", time.gmtime())))
            alive[mn] = alive.get(mn, 0) + (v == "ALIVE")
            dmax, dn = max(dmax, ms * (v in ("REFUSED", "DEAD"))), dn + (v in ("REFUSED", "DEAD"))
            time.sleep(1)
    print("pool free capacity estimate: %d alive key x model cells -- %s" % (sum(alive.values()), ", ".join("%s %d/5" % (m, n) for m, n in sorted(alive.items())) or "none alive"))
    print("timed note: " + ("no probe waited >1s on a dead route (max %d ms over %d refused/dead probe(s))" % (dmax, dn) if dn and dmax <= 1000 else ("A DEAD-ROUTE PROBE WAITED >1s: max %d ms over %d probe(s)" % (dmax, dn) if dn else "no refused/dead probe this date")))
    return 0
sys.exit(main())
