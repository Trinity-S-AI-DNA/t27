#!/bin/zsh
# Bee load + token stock, no LLM. Writes measures.json = live pulse rows + static rows. Owner 2026-10-20.
D=${0:A:h}; R=gHashTag/t27; now=$(date -u +%H:%M)
day=$(date -u -v-1d +%Y-%m-%d)
run=$(gh api "repos/$R/actions/runs?status=in_progress&per_page=1" --jq .total_count 2>/dev/null || echo -1)
que=$(gh api "repos/$R/actions/runs?status=queued&per_page=1" --jq .total_count 2>/dev/null || echo -1)
opened=$(gh pr list --repo $R --state all --search "created:>=$day" --limit 300 --json number --jq length 2>/dev/null || echo -1)
merged=$(gh pr list --repo $R --state merged --search "merged:>=$day" --limit 300 --json number --jq length 2>/dev/null || echo -1)
swarm=$(gh issue list --repo $R --state open --search "in:body \"Part of epic #11275\"" --limit 200 --json number --jq length 2>/dev/null || echo -1)
keys() { local n=0 a; for a in "$@"; do security find-generic-password -s ai.t27.os -a "$a" >/dev/null 2>&1 && n=$((n+1)); done; echo $n; }
zai=$(keys zai zai#2 zai#3 zai#4 zai#5 zai#6 zai#7)
orr=$(keys openrouter openrouter#2 openrouter#3)
paid=$(keys anthropic openai gemini)
agents=$(pgrep -f "claude|codex|opencode|crush|gemini|cursor-agent" | wc -l | tr -d ' ')
disk=$(df -g / | awk 'NR==2{print $4}')
state=BUSY; [ "$run" = 0 ] && [ "$que" = 0 ] && state=IDLE
python3 - "$D" <<PY
import json,sys
d=sys.argv[1]
live=[
 {"metric":"Bees running now ($state)","value":$run,"unit":"CI jobs","kind":"live $now","how":"actions runs in_progress, t27"},
 {"metric":"Bees queued","value":$que,"unit":"CI jobs","kind":"live $now","how":"actions runs queued, t27"},
 {"metric":"PRs opened 24 h","value":$opened,"unit":"PR","kind":"live $now","how":"gh pr list created >= $day"},
 {"metric":"PRs merged 24 h","value":$merged,"unit":"PR","kind":"live $now","how":"gh pr list merged >= $day"},
 {"metric":"Swarm issues waiting","value":$swarm,"unit":"issues","kind":"live $now","how":"open, Part of epic 11275"},
 {"metric":"z.ai free keys","value":$zai,"unit":"of 7","kind":"live $now","how":"Keychain items present, ai.t27.os"},
 {"metric":"OpenRouter keys","value":$orr,"unit":"keys","kind":"live $now","how":"Keychain items present"},
 {"metric":"Paid keys","value":$paid,"unit":"keys","kind":"live $now","how":"anthropic/openai/gemini items"},
 {"metric":"Agent processes","value":$agents,"unit":"proc","kind":"live $now","how":"pgrep agent CLIs on this Mac"},
 {"metric":"Disk free","value":$disk,"unit":"GB","kind":"live $now","how":"df; no builds below 8"},
]
st=json.load(open(d+"/measures_static.json"))
json.dump(live+st,open(d+"/measures.json","w"),indent=0)
print("state=$state run=$run que=$que opened=$opened merged=$merged swarm=$swarm zai=$zai or=$orr paid=$paid agents=$agents disk=$disk")
PY
