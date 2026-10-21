import os, re, json, hashlib, glob, collections

ROOTS = {'t27':'/Users/playom/t27','trinity':'/Users/playom/trinity','trios':'/Users/playom/trios'}
MIRROR_PREFIX = 'apps/website/public/t27/files/'
DIST_MIRROR   = 'apps/website/dist/'
OUT = '/Users/playom/t27/datasets/lora-v1'
stats = collections.OrderedDict()

# ---------- 1. enumerate ----------
allfiles = []
symlinks = []
for repo, root in ROOTS.items():
    for dp, dn, fn in os.walk(root):
        for f in fn:
            if f.endswith('.t27'):
                fp = os.path.join(dp, f)
                if os.path.islink(fp) or not os.path.isfile(fp):
                    symlinks.append((repo, fp)); continue
                allfiles.append((repo, fp))
allfiles.sort(key=lambda x: x[1])
stats['skipped_symlink_or_nonregular'] = len(symlinks)
stats['skipped_symlink_paths'] = [p for _,p in symlinks]
stats['enumerated_total'] = len(allfiles)
stats['enumerated_by_repo'] = dict(collections.Counter(r for r,_ in allfiles))

# ---------- 2. exclusions (ordered, first match wins) ----------
BUILD_DIRS = {'target','dist','build','node_modules','generated','out','.next','.cache','coverage','zig-out','zig-cache','.zig-cache'}
def rel(repo, p): return os.path.relpath(p, ROOTS[repo])

def exclusion(repo, p):
    r = rel(repo, p); segs = r.split('/')
    if '.claude/worktrees/' in r + '/': 
        if 'worktrees' in segs and '.claude' in segs: return 'worktrees'
    for i in range(len(segs)-1):
        if segs[i]=='specs' and segs[i+1]=='scratch': return 'specs_scratch'
    if r.startswith('var/trinity/output/'): return 'build_output'
    if r.startswith(DIST_MIRROR): return 'website_dist_mirror'
    for s in segs[:-1]:
        if s in BUILD_DIRS: return 'build_output'
    if r.startswith('public/') or '/public/' in r: return 'website_public_mirror'
    return None

kept, exc_counts = [], collections.Counter()
for repo, p in allfiles:
    e = exclusion(repo, p)
    if e: exc_counts[e] += 1
    else: kept.append((repo, p))
stats['excluded_by_rule'] = dict(exc_counts)
stats['excluded_total'] = sum(exc_counts.values())
stats['after_exclusions_strict'] = len(kept)

# mirror candidates rescued back in as low-priority variants
mirror = [(repo,p) for repo,p in allfiles
          if repo=='trinity' and rel(repo,p).startswith(MIRROR_PREFIX)]
stats['website_public_mirror_files'] = len(mirror)

# ---------- helpers ----------
def logical(repo, p):
    r = rel(repo, p)
    if r.startswith(MIRROR_PREFIX): r = r[len(MIRROR_PREFIX):]
    if r.startswith('t27/'): r = r[4:]
    return r

REPO_RANK = {'t27':0,'trinity':1,'trios':2}

# ---------- seals ----------
seals = {}
for f in glob.glob('/Users/playom/t27/.trinity/seals/*.json'):
    d = json.load(open(f)); seals[d['spec_path']] = d['spec_hash']
stats['seal_records'] = len(seals)
seal_hashes = set(seals.values())

# ---------- dialect ----------
RE_MOD  = re.compile(r'^[ \t]*(pub[ \t]+)?module[ \t]+[^\n;{]{1,120}?[ \t]*[;{]', re.M)
RE_ASM  = re.compile(r'^[ \t]*\.(const|data|code|text|word|dword|tryte|section|org|global)\b', re.M)
RE_SPEC = re.compile(r'^[ \t]*spec[ \t]+[A-Za-z_][\w.]*[ \t]*(version[ \t]+\S+)?[ \t]*\{?[ \t]*$', re.M)
RE_ALGO = re.compile(r'^[ \t]*algorithm[ \t]+[A-Za-z_]\w*[ \t]*\{', re.M)
RE_MD   = re.compile(r'^#{1,6}[ \t]\S', re.M)
def dialect(t):
    if RE_MOD.search(t):  return 'module-code'
    if RE_ASM.search(t):  return 'tri27-asm'
    if RE_SPEC.search(t): return 'spec-DSL'
    if RE_ALGO.search(t): return 'algorithm-DSL'
    if RE_MD.search(t):   return 'markdown-prose'
    return 'unknown'

# ---------- stub ratio ----------
RE_FN = re.compile(r'^[ \t]*(?:pub[ \t]+)?fn[ \t]+[A-Za-z_]\w*[ \t]*\(', re.M)
RE_BLOCK_C = re.compile(r'/\*.*?\*/', re.S)
RE_LINE_C  = re.compile(r'//[^\n]*')
RE_STUBWORD= re.compile(r'TODO|unimplemented|not[ _]implemented|FIXME|stub', re.I)
def stub_ratio(t):
    total = stubs = 0
    marked = [0]
    for m in RE_FN.finditer(t):
        i = t.find('{', m.end()-1)
        if i < 0: continue
        # a ';' before the '{' means a bodyless declaration
        if ';' in t[m.end():i]: continue
        d, j = 0, i
        while j < len(t):
            if t[j]=='{': d+=1
            elif t[j]=='}':
                d-=1
                if d==0: break
            j+=1
        if d!=0: continue
        body = t[i+1:j]; total += 1
        had_stubword = bool(RE_STUBWORD.search(body))
        code = RE_LINE_C.sub('', RE_BLOCK_C.sub('', body)).strip()
        if code=='':
            stubs += 1
            if had_stubword: marked[0] += 1
    return (round(stubs/total, 6) if total else 0.0), total, stubs, marked[0]

# ---------- build variant records ----------
def make(repo, p, is_mirror):
    b = open(p,'rb').read()
    t = b.decode('utf-8', errors='replace')
    sr, nfn, nstub, nmark = stub_ratio(t)
    sh = hashlib.sha256(b).hexdigest()
    return {'path': p, 'repo': ('trinity-website-mirror' if is_mirror else repo),
            'bytes': len(b), 'chars': len(t), 'dialect': dialect(t),
            'sealed': None, 'sealExact': ('sha256:'+sh) in seal_hashes,
            'stubRatio': sr, 'sha256': sh,
            '_logical': logical(repo,p), '_mtime': os.path.getmtime(p),
            '_mirror': is_mirror, '_rank': REPO_RANK[repo], '_fns': nfn, '_stubfns': nstub, '_markedstubs': nmark}

variants = [make(r,p,False) for r,p in kept] + [make(r,p,True) for r,p in mirror]

# sealed = logical path (relative to t27 root convention) appears in seal index
seal_paths = set(seals.keys())
for v in variants:
    v['sealed'] = v['_logical'] in seal_paths or v['sealExact']

def prio(v):  # lower is better
    return (0 if v['sealExact'] else 1, v['_mirror'], -v['bytes'], -v['_mtime'], v['_rank'], v['path'])

# ---------- 3a. exact-content dedup ----------
byhash = collections.defaultdict(list)
for v in variants: byhash[v['sha256']].append(v)
stats['variants_in_dedup_pool'] = len(variants)
stats['distinct_content_hashes'] = len(byhash)
stats['removed_by_exact_hash_dedup'] = len(variants) - len(byhash)
stats['hashes_with_multiple_copies'] = sum(1 for g in byhash.values() if len(g)>1)
reps = [sorted(g, key=prio)[0] for g in byhash.values()]

# ---------- 3b. logical-path dedup ----------
bylog = collections.defaultdict(list)
for v in reps: bylog[v['_logical']].append(v)
stats['distinct_logical_paths'] = len(bylog)
stats['logical_paths_with_multiple_variants'] = sum(1 for g in bylog.values() if len(g)>1)
stats['removed_by_logical_path_dedup'] = len(reps) - len(bylog)
collisions=[]
for k,g in sorted(bylog.items()):
    if len(g)>1:
        w=sorted(g,key=prio)[0]
        collisions.append({'logical':k,'kept':w['path'],'dropped':[x['path'] for x in g if x is not w]})
stats['logical_collision_detail']=collisions
final = [sorted(g, key=prio)[0] for g in bylog.values()]
final.sort(key=lambda v: v['path'])

stats['final_kept'] = len(final)
stats['final_mirror_orphans_rescued'] = sum(1 for v in final if v['_mirror'])
stats['zero_byte_files']=[v['path'] for v in final if v['bytes']==0]
stats['final_bytes'] = sum(v['bytes'] for v in final)
stats['final_chars'] = sum(v['chars'] for v in final)
stats['dialect_counts'] = dict(collections.Counter(v['dialect'] for v in final))
stats['sealed_count'] = sum(1 for v in final if v['sealed'])
stats['sealExact_count'] = sum(1 for v in final if v['sealExact'])
stats['files_with_zero_fn_decls'] = sum(1 for v in final if v['_fns']==0)
stats['files_stubRatio_gt_0'] = sum(1 for v in final if v['stubRatio']>0)
stats['files_stubRatio_eq_1'] = sum(1 for v in final if v['stubRatio']>=1.0)
stats['total_fn_decls'] = sum(v['_fns'] for v in final)
stats['total_stub_fns'] = sum(v['_stubfns'] for v in final)
stats['total_stub_fns_with_explicit_TODO_marker'] = sum(v['_markedstubs'] for v in final)
sx=[v for v in final if v['sealExact']]; nsx=[v for v in final if not v['sealExact']]
stats['sealExact_files_with_stubs'] = sum(1 for v in sx if v['stubRatio']>0)
stats['non_sealExact_files_with_stubs'] = sum(1 for v in nsx if v['stubRatio']>0)
stats['sealExact_bytes'] = sum(v['bytes'] for v in sx)
stats['sealExact_chars'] = sum(v['chars'] for v in sx)
stats['dialect_counts_sealExact'] = dict(collections.Counter(v['dialect'] for v in sx))

FIELDS = ['path','repo','bytes','chars','dialect','sealed','sealExact','stubRatio','sha256']
manifest = [{k:v[k] for k in FIELDS} for v in final]
os.makedirs(OUT, exist_ok=True)
json.dump(manifest, open(OUT+'/manifest.json','w'), indent=1)
json.dump(stats, open(OUT+'/stats.json','w'), indent=1)
print(json.dumps(stats, indent=1))
