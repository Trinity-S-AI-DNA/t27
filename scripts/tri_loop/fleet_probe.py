# The probe cell of tri fleet scan (#12170): one tiny claude -p call per route cell, the answer
# wired to the fleet law's (http_code, marker) vocabulary and classified by probe_result from
# gen/c/queen/api_fleet.c (generated; decisions never live here). Foreign: owner-approved
# 2026-10-25, quoted in the PR body (English there).
import ctypes, hashlib, os, subprocess, sys, tempfile, time
R = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, R + "/gen/python/queen")
def _bm_consts():
    import re
    src = open(os.path.join(os.path.dirname(__file__), "..", "gen/c/queen/api_fleet.c")).read()
    vals = dict((m.group(1), int(m.group(2), 0)) for m in re.finditer(r"#define\s+(BM_[A-Z_]+)\s+(\d+)", src))
    return vals
_BM = _bm_consts()
BM_NONE, BM_QUOTA, BM_OVERLOAD, BM_PLAN_EXCLUDES = (_BM["BM_NONE"], _BM["BM_QUOTA"], _BM["BM_OVERLOAD"], _BM["BM_PLAN_EXCLUDES"])
# z.ai body fragments -> (http_code, marker) of specs/queen/api_fleet.t27; 1113 spent, "token
# expired" the dead key, 1305 overload, 1302/1303 rate, then bare codes. A body matching nothing
# is (0, BM_NONE): the law reads that as UNPROBED, never as a verdict.
WIRE = (("1113", 402, BM_QUOTA), ("token expired or incorrect", 401, BM_NONE), ("1305", 0, BM_OVERLOAD), ("1302", 429, BM_NONE), ("1303", 429, BM_NONE), ("no access", 403, BM_PLAN_EXCLUDES), ('"type":"result"', 200, BM_NONE), ("401", 401, BM_NONE), ("403", 403, BM_NONE), ("429", 429, BM_NONE), ("529", 529, BM_NONE))
def classifier():
    src = open(R + "/gen/c/queen/api_fleet.c", "rb").read()
    ext = ".dylib" if sys.platform == "darwin" else ".so"
    base = tempfile.gettempdir() + "/fleet-" + hashlib.sha256(src).hexdigest()[:16]
    if not os.path.exists(base + ext):
        open(base + ".c", "wb").write(src)
        subprocess.run(["cc", "-shared", "-fPIC", "-O2", "-w", "-o", base + ext, base + ".c"], check=True)
    f = ctypes.CDLL(base + ext).probe_result
    f.argtypes, f.restype = [ctypes.c_uint8, ctypes.c_uint32, ctypes.c_uint8], ctypes.c_uint8
    return lambda route_id, http, marker: ("ALIVE", "REFUSED", "DEAD", "UNPROBED")[f(route_id, http, marker)]
def pool():
    pairs = (l.strip().partition("=") for l in open(os.path.expanduser("~/.claude/.env")))
    return sorted((n, v) for n, _, v in pairs if n.startswith("ZAI_KEY_") and v)
def probe(key, model):
    env = {k: v for k, v in os.environ.items() if not k.startswith(("ANTHROPIC_", "CLAUDE_CODE_"))}
    env.update(ANTHROPIC_BASE_URL="https://api.z.ai/api/anthropic", ANTHROPIC_AUTH_TOKEN=key, ANTHROPIC_MODEL=model, ANTHROPIC_SMALL_FAST_MODEL=model, ANTHROPIC_DEFAULT_HAIKU_MODEL=model, API_TIMEOUT_MS="30000", CLAUDE_CODE_MAX_RETRIES="0", CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC="1")
    t, body = time.monotonic(), ""
    try:
        p = subprocess.run(["claude", "-p", "Reply with exactly: OK", "--max-turns", "1", "--output-format", "json", "--model", model], env=env, capture_output=True, text=True, timeout=45)
        body = (p.stdout or "") + (p.stderr or "")
    except subprocess.TimeoutExpired:
        pass
    ms = int((time.monotonic() - t) * 1000)
    for frag, code, mk in WIRE:
        if frag in body:
            return code, mk, ms
    return 0, BM_NONE, ms
