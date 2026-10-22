# T27F Live: a generative UI for agent work, in our own format

Research and design proposal, 2026-10-18. Status: proposal, nothing here is built yet.
Epic: gHashTag/t27#10266 (milestones M0-M6 are #10259-#10265). Definition of T27F: `.claude/skills/t27f/SKILL.md`,
section "What T27F is" (gHashTag/trinity#1648, RFC gHashTag/trinity#1649). This document does
not restate it.

Placement note: the brief asked for `docs/t27f/`. Article DOCS-TREE of
`docs/T27-CONSTITUTION.md` allows new first-party Markdown only under its buckets, so this file
lives in `docs/nona-02-organism/t27f/` (language and organism themes, thematic subfolder).

## 0. The question, and the short answer

The owner asked (translated): T27F was planned from the start to become a generative UI of agent
work in our own format, not HTML and not web JS. Is that impossible because the browser does not
support our format? What is the best way to build such an interface with real-time video and
streaming, in the visual style of the reference post, with the Queen Trinity as a face (a
different face, blue eyes), driven by graph relationships?

Short answer:

1. **Yes, a browser can run our format.** A browser executes exactly two kinds of program:
   JavaScript and WebAssembly. t27 already reaches the browser as WebAssembly: the page
   `t27.ai/t27f/graph/` (gHashTag/trinity#1679) runs the T27F exporter and layout, both written
   in `.t27`, as a 39 KB wasm module, and the only hand-written JavaScript is 39 lines that move
   bytes in and draw what comes back. The same split scales to a live, animated interface: the
   scene, layout, animation, state machine and the face are `.t27`; a loader of at most 40 lines
   hands a flat list of draw commands to the browser's canvas. That loader is plumbing, like a
   sitemap: it is what the browser parses, not what the project says about itself.
2. **The reference post is not a real-time render.** It is a 3-minute music video produced
   offline ("Made in 44 hours by 116 agents. 2,423,515,148 tokens, 409 images, 78 clips"; the
   author's own reply links it as an "Official Music Video"). What is reproducible live is its
   visual grammar: generated footage of a character, overlaid with an agent-state HUD (a status
   bar reading TASK COMPLETE / AGENT RUNNING / TASKS n / a clock, task cards such as "TASK 09
   ACTIVE", chart overlays, a window labelled `cloud.mp4`, a log ticker). The HUD is exactly what
   T27F can generate from real agent events; the footage is an asset layer.
3. **The best build is client-side and state-driven**: agent events (already published by the
   Queen actor feed and the t27-mcp SSE stream) feed a `.t27` work graph (spec -> agent ->
   action); a `.t27` scene function turns the graph into a HUD and a vector Queen face; the
   browser only paints. Video is an optional slot that plays allowlisted clips; no GPU server
   streams pixels. Section 6 gives three options with costs.

## 1. What already exists (measured, not planned)

| Piece | Where | Number |
|---|---|---|
| Spec -> graph IR | `specs/t27f/spec_graph.t27`, `spec_graph_export.t27`, `spec_graph_derive.t27` | export p50 0.09 ms, p99 2.1 ms, max 18.8 ms over 2227 specs (Railway lab, 2026-10-18) |
| Graph layout, fold, hit-test | `specs/t27f/spec_graph_view.t27`, `spec_graph_fold.t27` | caps 1024 nodes / 4096 edges, u16 indices |
| Hot reload record | `specs/t27f/hot.t27`; host in gHashTag/t27-mcp `--watch`, `GET /events` (SSE) | save -> SSE event p50 ~25 ms, p90 ~35 ms (lab) |
| Wasm build rule | `specs/t27f/build.t27` | cold rebuild 1.1 s, reuse check 0.2 s, byte-identical output |
| Browser renderer | gHashTag/trinity `apps/website/public/t27f/graph/` | 39 lines JS glue, 39 KB wasm, SVG output |
| UI plan allowlist | `specs/t27f/ui_plan.t27` | 5 trusted widgets, `FORBIDDEN_NAMES` = script, eval, html, iframe, raw_js, function |
| Agent event feed | `specs/queen/actor_events.t27`, `actor_view.t27`, `tasks.t27` | spawn / exit / DOWN / restart / deliver; task states backlog, blocked, running, review, done, dropped, failed |
| Queen art | gHashTag/trinity `apps/website/public/queen/portrait-queen-256.png` and siblings | static 256 px portraits |

So the hard parts of the chain (our language in the browser, a deterministic exporter, an SSE
stream with a measured latency, a typed allowlist, an agent event feed) are already on master.
What is missing is a scene layer that animates, a work graph that joins agents to specs, and a
face.

## 2. Can a browser run our own format? The honest map

Read on 2026-10-18. Support versions come from the webstatus.dev API
(`api.webstatus.dev/v1/features`), MDN and vendor engineering posts.

### 2.1 What a browser executes, and where our language stops

A browser runs JavaScript and WebAssembly, and nothing else. WebAssembly is enough for every
piece of logic: the Wasm 3.0 feature set (GC, tail calls, exceptions, 128-bit SIMD, 64-bit
memory) was completed on 2025-09-17 and ships in Chrome, Firefox and Safari
(webassembly.org/news/2025-09-17-wasm-3.0/). t27 needs none of the newer features: it lowers to
`wasm32-freestanding` through Zig, with no garbage collector and no threads, which is already the
page `/t27f/graph/`.

What wasm cannot do is touch a Web API. WebGPU, Canvas, `fetch`, `EventSource`, WebSocket,
WebCodecs, input events, IME, clipboard and the DOM are all JavaScript interfaces; wasm sees only
the functions its host imports. Even Figma's C++ renderer reaches WebGPU "ultimately" through the
browser's JavaScript API (figma.com/blog/figma-rendering-powered-by-webgpu/), and Dawn's
Emdawnwebgpu ships JS that "must be provided at the final link step"
(dawn.googlesource.com, src/emdawnwebgpu/pkg/README.md).

So "zero JavaScript" is not reachable for anyone. The accurate claim, and the one T27F Live makes,
is: **zero logic in JavaScript, one shim whose only job is to call the browser.** The measured
spread of that shim across the field:

| Project | Logic in wasm | Glue | Accessibility |
|---|---|---|---|
| Zig `wasm32-freestanding`, compute only | everything | about 10 lines (0xkiire.com/wasm-with-zig/) | none |
| t27 `/t27f/graph/` (ours) | export + layout, 39 KB wasm | 39 lines | SVG text |
| Rive web runtime | C++ animation runtime | small wrapper; 222-648 KB compressed total (rive.app/docs/runtimes/runtime-sizes) | not documented |
| Makepad (Rust UI) | the whole UI | about 300 KB of unminified hand-written JS (web.js 141,778 chars, web_gl.js 131,716; github.com/makepad/makepad, platform/src/os/web) | not documented |
| Flutter web (skwasm) | Dart via WasmGC + Skia | loader + DOM overlays; engine 1.21 MB brotli | opt-in hidden button, then a DOM mirror of the Semantics tree (docs.flutter.dev/ui/accessibility/web-accessibility) |
| Slint web | Rust | wasm-bindgen glue | "not available" (docs.slint.dev, platforms/web) |
| egui web | Rust | eframe web integration | experimental built-in screen reader only |

Everything with a full UI pays tens to hundreds of kilobytes of glue, because its wasm calls the
browser in hundreds of places. T27F Live keeps the glue small by design: the wasm returns a flat
display list of about 12 opcodes, so the loader calls about 12 canvas methods and nothing else.

### 2.2 The layer map

| Layer | Must stay plumbing (we call it, we cannot replace it) | Is `.t27` |
|---|---|---|
| Loading | instantiate wasm, pass imports, `requestAnimationFrame` | -- |
| Drawing | Canvas2D context (later WebGPU device, swap chain) | the display list, every coordinate, colour token and glyph position |
| Network | `EventSource` (SSE) down, `fetch` POST up | event names, sequence numbers, keyframes, coalescing, resume (`live_stream.t27`) |
| Video | `<video>` element or WebCodecs `VideoDecoder`; `drawImage` / `importExternalTexture` | which clip id plays for which state, and when (`clip_lane.t27`) |
| Input, IME | pointer and key events; a hidden `<input>` if text entry is ever needed | hit-testing, focus order, the action allowlist (`ui_plan.t27`) |
| Accessibility, SEO | DOM text nodes, `aria-live`, `<title>` and og tags | their content (`a11y_mirror.t27`); a static HTML shell generated from the spec |
| Model | none | work graph, scene, HUD, face, motion: all of it |

The loader is plumbing in exactly the sense of the own-language-first rule: it is a shape the
browser parses, not a description of the project. Under the only-t27 rule it is thin glue (at most
40 lines per file). The longer-term fix is to stop writing it by hand: a `.t27` host-interface spec
that lists the imports, and a t27c backend that emits the shim from it. Until that backend exists,
the shim is the one hand-written file, and it holds no logic.

### 2.3 Renderer options

| Renderer | Where it runs today | Verdict |
|---|---|---|
| Canvas2D | every browser and webview | **first target**: paths, text, images and video frames, about 12 calls |
| SVG (what `/t27f/graph/` uses) | everywhere | fine for static graphs; too many DOM nodes for 60 fps animation |
| WebGL2 | baseline since 2021 (Chrome 56, Firefox 51, Safari 15) | a fallback, needs shaders |
| WebGPU | Chrome/Edge desktop since 113, Android since 121; Safari 26 (macOS, iOS) since 2025-09-15; Firefox on Windows since 141 and Apple-silicon macOS since 145, Linux and Android not yet (web.dev/blog/webgpu-supported-major-browsers, gpuweb wiki) | **later**: only when t27c can emit WGSL, because a hand-written shader is foreign code; and a Canvas2D fallback stays for Firefox Linux and unknown Telegram webviews |

Telegram Mini Apps run in the platform webview (Chrome-based on Android, system WebKit on iOS,
core.telegram.org/bots/webapps); WebGPU inside them is UNVERIFIED, which is one more reason to
start on Canvas2D.

### 2.4 Transport

| Transport | Status | Use |
|---|---|---|
| SSE (`EventSource`) | universal; auto-reconnect with `Last-Event-ID`; the 6-connection limit applies only on HTTP/1.1 (MDN) | **agent events down** (already in t27-mcp `--watch`) |
| `fetch` POST | universal | **actions up** (allowlisted ids) |
| WebSocket | universal | not needed: our traffic is one way |
| WebTransport | Chrome 97, Firefox 114, Safari 26.4; moq.dev still falls back to WebSocket on Safari because of long-session stalls (doc.moq.dev/concept/transport) | not needed for events; a candidate for video later |
| WebRTC / WHEP | universal | only for server-rendered video (option C) |
| MoQ (Media over QUIC) | IETF draft, `draft-ietf-moq-transport-22`, 2026-10-01, not an RFC (datatracker.ietf.org/wg/moq/documents/); Cloudflare runs a public relay | watch, do not build on |

### 2.5 Video, and why the pixels should be made in the browser

WebCodecs ships in Chrome 94, Firefox 130 and Safari 26 (webstatus.dev), and a decoded
`VideoFrame` can go straight to the GPU through `GPUDevice.importExternalTexture` (MDN), so a
clip lane composites without copying. Measured glass-to-glass latency for live camera video over
WebRTC is about 130-170 ms on a local or TURN path (transitiverobotics.com/blog/webrtc-latency-breakdown/)
and about 265-340 ms through WHEP relays (aircast.one/blog/glass-to-glass-latency-test). Those are
camera numbers, not UI numbers, but they bound the server-rendered path from below: it pays render,
encode, network, jitter buffer and decode for every frame, and it needs one GPU per viewer.
Client-side rendering pays one network hop per *event*, not per frame, and our measured hop is
about 25 ms p50 (section 1). That arithmetic, not taste, is why the design renders in the browser
and keeps server video for the one thing that needs it: a photoreal face, if anyone ever wants one.

### 2.6 The honest costs of drawing on a canvas

- **Accessibility**: a canvas has no semantics. The only working pattern is Flutter's: a parallel
  DOM mirror. We generate ours from the scene (`a11y_mirror.t27`) and test that every text node
  has a mirror line; it still has to be tried with VoiceOver and NVDA by a person.
- **Text input**: needs a hidden `<input>`; T27F Live is a viewer with buttons, so the first
  milestones need none.
- **SEO and link previews**: crawlers and the X / Telegram preview bots read static HTML. The page
  ships a static shell with title, og tags and a text summary generated from the same spec, as the
  `/learn/` pages already do.
- **Text shaping**: a full shaper and fonts are the heaviest part of any canvas UI. The first
  version draws with the browser's own `fillText` and system fonts, and measures text through the
  same call; shaping in wasm is not on this roadmap.

## 3. Competitors and prior art

### 3.1 Generative-UI protocols

All rows were read on 2026-10-18 from the primary page named in the Source column. The axis that
matters most for us is the second column: does executable code reach the client?

| Stack | Payload | Code reaches client? | Transport | Licence, price | Status | Source |
|---|---|---|---|---|---|---|
| Vercel AI SDK UI / RSC | tool call + result -> developer React component; RSC streams a React tree | no model code | UI message stream (wire format UNVERIFIED) | Apache-2.0, free | `ai` 7.0.143; RSC `streamUI` "experimental", the vendor recommends AI SDK UI for production | ai-sdk.dev/docs/ai-sdk-ui/generative-user-interfaces, ai-sdk.dev/docs/ai-sdk-rsc/migrating-to-ui |
| Google A2UI | declarative JSON component tree + data model, JSONL; client keeps a catalog of trusted components | no ("a declarative data format, not executable code") | transport-agnostic; A2A and AG-UI stable, SSE and WebSockets proposed | Apache-2.0, free | v0.9.1, v1.0 release candidate, "early stage public preview" | github.com/a2ui-project/a2ui README, a2ui.org/concepts/transports/ |
| CopilotKit + AG-UI | AG-UI: about 16 event types; CopilotKit `useComponent`: tool args -> React props | no for `useComponent`; AG-UI is a pipe and also carries MCP-UI iframes | SSE, WebSockets, webhooks | MIT; cloud Free / $39 per month / $100 per seat per month / custom | `@ag-ui/core` 1.0.3 | github.com/ag-ui-protocol/ag-ui README, docs.ag-ui.com/concepts/generative-ui-specs, copilotkit.ai/pricing |
| MCP Apps (SEP-1865), mcp-ui | `ui://` resource, MIME `text/html;profile=mcp-app` | **yes**: server HTML/JS in a mandatory sandboxed iframe, CSP built from declared domains | JSON-RPC 2.0 over `postMessage` | Apache-2.0, free | spec "Stable (2026-01-26)"; ext-apps 2.0.3 | github.com/modelcontextprotocol/ext-apps, specification/2026-01-26/apps.mdx |
| Claude Artifacts | one model-written HTML page (or Markdown) | **yes**: model-written HTML/CSS/JS under strict CSP, 5 CDN hosts, no backend, at most 16 MiB | published to claude.ai (wire UNVERIFIED) | closed; included in all plans | legacy artifacts cannot be created after 2026-09-16 | code.claude.com/docs/en/artifacts, support.claude.com article 17153992 |
| OpenAI Apps SDK, now "Plugins" | MCP tool result + HTML widget | **yes**: developer widget in an iframe with strict CSP | MCP Apps bridge + `window.openai` extensions | examples MIT, platform closed; distribution pricing UNVERIFIED | ChatGPT implements MCP Apps; changelog 2026-08-21 | developers.openai.com/plugins/build/chatgpt-ui, /plugins/guides/security-privacy |
| tldraw make real | canvas screenshot -> vision model -> HTML in an iframe on the canvas | **yes**: model-written HTML | HTTPS to the model API | SDK custom tldraw licence, 100-day trial then commercial | make-real repo archived 2026-02-20; tldraw computer "experimental"; the agent kit emits typed canvas actions | github.com/tldraw/make-real, computer.tldraw.com, tldraw.dev/starter-kits/agent |
| json-render (Vercel Labs) | JSON spec constrained by a zod catalog; JSONL patch ops set/add/replace/remove | no ("AI can only use components in your catalog") | chunked HTTP JSONL | Apache-2.0, free | core 0.21.1 | github.com/vercel-labs/json-render, json-render.org/docs/streaming |
| OpenUI (Thesys, C1 line) | "OpenUI Lang", a compact streaming language parsed by the renderer | no | streaming text | MIT; cloud Free / $49 / $499 per month / custom | react-lang 1.0.1; "up to 67% fewer tokens than JSON" is a vendor claim | github.com/thesysdev/openui, thesys.dev/pricing |
| LangGraph generative UI | `push_ui_message(name, props)`; components bundled by the agent server | no model code, but developer JS is loaded from the server at run time | LangSmith streaming API | MIT; LangSmith pricing UNVERIFIED | version UNVERIFIED | docs.langchain.com/langsmith/generative-ui-react |

**Latency.** None of these ten stacks publishes a time-to-first-UI or end-to-end latency number.
The only quantified statements found are OpenUI's token claim and the AI SDK's own warning that
`createStreamableUI` "can cause quadratic data transfer". Our save -> SSE p50 of about 25 ms
(section 1) is, as far as this survey found, the only measured number in the field.

**What the field looks like.** Two camps. Camp one ships code and contains it: MCP Apps, the
OpenAI plugin UI, Claude Artifacts, make real (iframe plus CSP). Camp two ships only data that
names allowlisted components: A2UI, json-render, OpenUI, AI SDK UI, CopilotKit `useComponent`.
ChatGPT and Claude both host MCP Apps, and AG-UI and A2UI present themselves as pipe and payload.

**Where T27F Live wins.** It is in camp two, and goes one step further: the catalog, the plan and
the tests are one language checked by one compiler before anything renders (A2UI and json-render
validate at run time with JSON Schema or zod), the renderer has no widget code at all (it executes
about 12 draw opcodes, not React components), and the output is deterministic, so a frame can be
hash-checked and replayed. Nobody in the table draws agent state continuously; they all render a
UI once per tool call.

**Where it loses.** Renderer breadth (A2UI and json-render target many frameworks), distribution
(ChatGPT and Claude already host MCP Apps), open-ended UIs the allowlist does not cover, and
ecosystem. The cheapest bridge is composition: a T27F Live event stream can ride inside AG-UI as
its transport, and a T27F view can be offered later as an MCP App whose HTML is the 40-line
loader plus our wasm.

### 3.2 Live avatars and real-time visual runtimes

Read on 2026-10-18 from vendor docs, pricing pages, GitHub and arXiv. "n/p" means the vendor
publishes no number. A latency marked UNVERIFIED appears only in press, a tweet or a
competitor-run aggregator, never on the vendor's own page.

**A. Server-rendered live avatars** (audio or text in, video out over WebRTC)

| Vendor | Render, transport | Latency | Price (primary page) | Face | Source |
|---|---|---|---|---|---|
| HeyGen LiveAvatar | server GPU, LiveKit WebRTC; LITE mode audio in, video out | n/p ("depends on your pipeline") | LITE 1 credit/min, FULL 2; Essential $99 = 1,100 credits; "$0.01/min at scale" | a real person (2 min footage + consent recording); some stylised catalogue faces | docs.liveavatar.com (lite-mode, faq/credits, core-concepts/avatars) |
| Synthesia Interactive Avatars | server GPU, LiveKit plugin | n/p | $0.12/min pay as you go | photoreal presenters, "or cartoon based" | synthesia.io/features/avatars/interactive-avatars |
| D-ID Agents | server video, WebRTC; V4 over LiveKit | n/p | Build $14.40/mo for 32 min, Launch $35 for 90 min (about $0.39-0.45/min, computed) | any uploaded photo | docs.d-id.com/reference/agents-sdk-overview, d-id.com/pricing/api/ |
| Tavus CVI (Phoenix-4) | server video, WebRTC via Daily; 1080p 40 fps | "low latency"; sub-600 ms is press only, UNVERIFIED | Builder $59 for 175 min, overage $0.35-0.26/min; at most 15 concurrent | one image or 2 min video of a person, or a character | docs.tavus.io, tavus.io/pricing |
| Hedra Live Avatars | server video, LiveKit | "under 100 ms" UNVERIFIED (tweet) | $0.05/min UNVERIFIED | any style from one image (secondary) | hedra.com/developers (empty SPA), press |
| Simli | server video, WebRTC; PCM16 16 kHz in | "under 300 ms" UNVERIFIED (aggregator) | UNVERIFIED | faces from an image | docs.simli.com |
| Anam | server video, managed STT -> LLM -> TTS -> face | 180 ms UNVERIFIED (aggregator); docs report p50/p95 per session | Starter $12/mo; overage $0.16-0.11/min | from an image or a text prompt | docs.anam.ai, anam.ai/pricing |
| bitHuman Expression 2 | **on device** neural audio-to-video: iPhone, Mac, Linux CPU, WebGPU browser | measured by the vendor (2026-10-08): first speech 0.05-0.89 s by device; Chrome on M5 at 2.4x real time (49 fps) | own hardware about $0.02/min, cloud $0.04, voice $0.10 | any character from one portrait | docs.bithuman.ai/performance.md, bithuman.ai/pricing |
| NVIDIA ACE Audio2Face-3D | not video: ARKit blendshape weights computed on an NVIDIA GPU; the client draws the mesh | "faster than 60 FPS"; a 1 s window with 0.5 s stride implies at least about 0.5 s buffering (derived from arXiv 2508.16401) | SDK MIT, weights NVIDIA Open Model License; needs CUDA, Windows/Linux | any blendshape rig, stylised allowed | github.com/NVIDIA/Audio2Face-3D, huggingface.co/nvidia/Audio2Face-3D-v3.0 |
| Lemon Slice | page did not render | UNVERIFIED | UNVERIFIED | UNVERIFIED | lemonslice.com |

**B. Client-side and engine runtimes**

| Runtime | How it renders | Licence, price | Fit for "expression = f(agent state)" | Source |
|---|---|---|---|---|
| Rive | client vector runtime, state machines, data binding; WASM, WebGL | runtime MIT, editor proprietary ($0-$120 per seat/mo); web runtime 222-648 KB compressed | the closest existing match; but `.riv` is a binary authored in a GUI editor, not a diffable spec | rive.app/docs/runtimes/data-binding, /runtime-sizes, rive.app/pricing |
| Live2D Cubism | client 2D mesh deformation by parameters | free to develop; publication licence, and "Expandable Applications" (avatar systems) excluded from the small-business exemption | same philosophy, licence trap | live2d.com/en/sdk/license/ |
| VRM + three-vrm | client three.js WebGL/WebGPU humanoid | MIT runtime; model licences per model | named expression presets (happy, angry, sad, relaxed, surprised, visemes, blink, gaze) are a direct precedent for our table | github.com/pixiv/three-vrm, vrm-specification expressions.md |
| VTube Studio | desktop app renders Live2D; WebSocket API feeds parameters | API MIT, app proprietary | protocol precedent (state -> parameters over a socket); not embeddable | github.com/DenchiSoft/VTubeStudio |
| Spline | real-time 3D on the web | runtime licence UNVERIFIED; $0-$60 per seat/mo | rich 3D, heavy | spline.design/pricing |
| Gaussian-splat heads | GPU rendering of rigged 3D Gaussians; SplattingAvatar "over 300 FPS" on a desktop GPU, 30 fps mobile | GaussianAvatars CC-BY-NC-SA-4.0 (non-commercial) | trained from video of a real subject: the opposite of our likeness rule; no browser-native audio-driven splat avatar found | github.com/ShenhanQian/GaussianAvatars, initialneil.github.io/SplattingAvatar |
| Unreal Pixel Streaming | server renders, WebRTC to the browser; NVENC/AMF/VideoToolbox | engine EULA; one GPU per viewer | cinematic (MetaHuman), cost per concurrent viewer | dev.epicgames.com pixel-streaming docs |
| Unity Render Streaming | as above, Unity | Unity Companion License | as above | docs.unity3d.com renderstreaming manual |

**What the market looks like.** Every live-avatar vendor sells the same pipeline: a GPU server
renders neural video of a (usually real) face from audio and streams it over WebRTC through
LiveKit or Daily, priced per minute, $0.01 to $0.45. None takes agent state as input; they all
take audio or text, so the face shows speech, never work. Only bitHuman publishes a measured
latency table; everyone else publishes an adjective.

**What T27F Live beats.** Zero cost per minute and no GPU, because a vector face is about 300
control points per frame, not a neural net per frame. No bandwidth beyond kilobytes of events. No
consent paperwork, because no real face is an input. Deterministic: same state and frame, same
points, so the face is testable, which no vendor can offer. And it is the only design in which
the face is driven by the work graph rather than by a voice.

**What it loses.** Photoreal presence, lip sync from arbitrary audio (we need a viseme layer,
section 4.5), reading the user's face (Tavus Raven), and instant avatars from a single portrait.
Rive and VRM are the nearest open precedents and are worth reading for their expression model;
we do not adopt their binary or 3D formats, because the face must be a `.t27` table to be ours.

## 4. The Queen Trinity avatar

### 4.1 Rules for the character

- **An original character, never a real person's likeness.** No photograph, video or scan of
  any real person is an input to the rig, the assets or any model prompt. The eye colour (blue)
  is a style token, `IRIS_BLUE`, and nothing more. The face is a new design in the line of the
  existing Queen art in `apps/website/public/queen/`, not a copy of the reference video's actor
  and not a portrait of anyone.
- **Stylised, not photoreal.** A stylised face is honest about being a machine, avoids the
  uncanny valley at low frame rates, and is drawable as vectors at 60 fps on a phone.
- **Every expression is a pure function of agent state.** The face shows what the agents are
  doing; it never pretends to feel. A suggested (model-written) caption may appear beside it,
  drawn with the SUGGESTED style, never as the face's own speech without that label.

### 4.2 The rig: a parametric vector face

The face is a small set of named parameters, each a fixed-point number in [0, 1] or [-1, 1]:
head yaw and pitch, gaze x and y, left and right lid, brow height and tilt, mouth open, mouth
smile, cheek, a glow level for the collar light (the reference's green collar LED becomes a
state lamp), and hair sway. Around 16 parameters, against the 52 blendshapes of ARKit or the
hundreds of a Live2D model: enough for six readable expressions, small enough to keep in a
`.t27` table with tests.

Drawing: the face is a stack of layers (back hair, face, eyes, iris, lids, brows, mouth, front
hair, collar), each a closed path of cubic Bezier segments. A parameter moves control points by
a weight table (linear blend of deltas, the same idea as Live2D deformers and blendshapes).
`face.t27` computes the deformed points; the loader only fills paths. Roughly 300 control points
per frame, which is under one millisecond of wasm work (target, to be measured).

### 4.3 State -> expression

`tasks.t27` already names seven task states; the actor feed names spawn, exit, DOWN and restart.
The face reads an aggregate of the focused agent's state:

| Agent state (source) | Face | Collar lamp | Gaze |
|---|---|---|---|
| idle, no running task | slow breathing, blink every 3-6 s | dim blue | camera |
| running (`tasks.t27` running) | lids 80%, slight brow focus | pulsing blue | the active node in the work graph |
| review | one brow up | amber | the PR / test node |
| blocked (`blocked`, or a DOWN) | brows in, mouth flat | red, steady | the blocking edge |
| done | small smile, eyes close briefly | green flash, then dim | camera |
| failed / restart | quick look away, then back | red blink per restart_count | the failing actor |

Blink timing, breathing and hair sway come from a seeded schedule keyed by frame number, so the
same event log always renders the same frames (determinism is a test, not a hope).

### 4.4 Graph edges drive the picture

The HUD is the work graph drawn in the style of the reference: every on-screen element is a node
or an edge, so the picture is a view of the graph, not decoration.

- **Nodes**: SPEC (a `.t27` file), AGENT (an actor pid or a Bee), TASK (an issue or a queue
  card), ACTION (a tool call, a test run, a PR), ARTIFACT (a file, a wasm, a clip).
- **Edges**: READS (agent -> spec), ASSIGNED (task -> agent), PERFORMS (agent -> action),
  PRODUCES (action -> artifact), VERIFIES (action -> spec, a test), BLOCKS.
- **Labels**, as in `spec_graph.t27`: EXACT when the edge is an event in the feed (a deliver that
  names a task, a test run that names a spec), DERIVED when it follows from a rule (an issue's
  `## Boundary` path lines say which specs a task reads), SUGGESTED for a model's narration,
  UNKNOWN otherwise. The HUD draws each label with its own stroke, as the graph page does.
- **Motion follows edges**: when an ACTION node appears, a pulse travels the PERFORMS edge from
  the agent to it; the face's gaze moves to the newest EXACT node; a task card slides into the
  "TASK COMPLETE" bar when its VERIFIES edge lands. The pulse is a tween on the edge's own path,
  so every animation is anchored to a row of the graph.

### 4.5 Speech (later, optional)

A mouth that moves only when text is shown keeps the first version silent and honest. Lip sync
is a later milestone: text -> phonemes -> visemes is a table (`face.t27` maps about 12 visemes to
mouth parameters); the audio itself would come from a TTS vendor or an open model, and that
voice is plumbing, not our format.

## 5. Architecture and decomposed plan

### 5.1 The pipeline

```
agent events  -->  work graph  -->  scene  -->  display list  -->  paint
(actor feed,      (work_graph     (hud, face,   (display_list    (loader: <= 40 lines,
 t27-mcp SSE,      .t27, labels)   motion .t27)  .t27, flat ops)   Canvas2D, later WebGPU)
 tasks route)                                                    + a11y text mirror
```

Everything left of "paint" is `.t27`, compiled once by t27c to Zig and wasm32 (as the graph page
is today, under the `build.t27` seal). The model is never in this loop. A model may write the
SUGGESTED captions and may propose a `ui_plan` for a panel, and both pass `validate_plan` or a
new `validate_scene` before anything is drawn. No field anywhere carries code (`ui_plan.t27`
`FORBIDDEN_NAMES` extends to the scene).

### 5.2 Spec files (all new unless marked)

| File | Decides | Tests that settle it |
|---|---|---|
| `specs/t27f/work_graph.t27` | node and edge kinds of spec -> agent -> task -> action -> artifact; labels; `apply_event(graph, event)`; caps | a recorded feed of N events gives the same graph twice; every edge has a label; an event naming an unknown pid makes an UNKNOWN node, never a guess |
| `specs/t27f/scene.t27` | retained scene IR: allowlisted primitives (group, rect, text, path, image_ref, clip_ref, face), style tokens, `validate_scene`, delta ops (add / set / remove / keyframe) with sequence numbers | a forbidden primitive name is refused; `apply_delta` of a keyframe equals the keyframe; deltas out of order are refused |
| `specs/t27f/motion.t27` | deterministic tweens and springs in fixed point, indexed by frame number; reduced-motion switch | frame k of a tween is the same on every run; reduced motion jumps to the end state |
| `specs/t27f/hud.t27` | the HUD layout in the reference's grammar: status bar (state, task count, clock), task cards, log ticker, metric chart, graph inset | a fixture snapshot gives a fixed scene; text never overflows its box; 375 px and 1920 px layouts both validate |
| `specs/t27f/face.t27` | the parametric Queen face: parameters, layers, weight tables, state -> expression table, blink and breath schedule, viseme table | each of the six expressions differs from every other by at least a stated parameter distance; same state and frame give the same points; `IRIS_BLUE` is the only colour token for the iris |
| `specs/t27f/display_list.t27` | flattening a scene at frame k into a flat array of draw ops (about 12 opcodes: save, restore, transform, fill_rect, stroke_path, fill_path, text, image, clip_frame, alpha); a per-frame op budget | the op stream of a fixture has a fixed sha256; a scene over the budget is refused, never truncated |
| `specs/t27f/live_stream.t27` | the wire record: SSE event names, seq, keyframe every K deltas, resume from Last-Event-ID, coalescing under backpressure, max event rate | a client that misses deltas recovers from the next keyframe to the same scene as one that did not |
| `specs/t27f/a11y_mirror.t27` | the text mirror of a scene: one line per visible node, an aria-live summary per state change, rate-limited | every TEXT and every focusable node has a mirror line; the summary of a state change names the agent and the task |
| `specs/t27f/clip_lane.t27` | allowlisted clip ids per state (option B), how a clip slot is timed against the scene clock | a state with no clip falls back to the vector face; an id not in the table is refused |
| `specs/t27f/ui_plan.t27` (exists) | the widget allowlist; gains a `live_panel` widget bound to a work graph source | `validate_plan` still refuses every forbidden name |
| `specs/t27f/hot.t27` (exists) | the SSE record for spec saves; the live stream reuses its event discipline | unchanged |

### 5.3 Transport

- **Agent state to the browser: SSE.** It is what t27-mcp `--watch` already serves (measured
  save -> event p50 ~25 ms), it rides plain HTTP through every proxy and webview (including
  Telegram Mini Apps), and it reconnects with `Last-Event-ID` for free. The stream carries scene
  deltas or, simpler and preferred, the raw events: the browser runs `work_graph` + `hud` + `face`
  itself in wasm, so the server never renders and the wire carries kilobytes, not video.
- **User actions back: POST** of an allowlisted action id from `ui_plan.t27` (open spec, run
  tests on the lab). No WebSocket is needed for an interface whose traffic is one way.
- **Video: a clip slot, not a stream.** Option B plays pre-rendered clips (files on the CDN)
  through the browser's own media pipeline and paints the current frame into the scene slot;
  Canvas2D `drawImage` takes a video element directly. Server-rendered live video (WebRTC pixel
  streaming or a vendor avatar) is option C and is the only path that needs a GPU server.
- **Export for X: the same display list.** The lab renders the display list of a recorded event
  log frame by frame to images and encodes an MP4 (ffmpeg as plumbing). The live page and the
  posted video are the same frames, which is the claim the reference post cannot make.

### 5.4 Renderer

- **Canvas2D first.** It is in every browser and every webview, it draws paths, text and video
  frames, and a loader for about 12 opcodes fits the 40-line glue cap. WebGPU comes later and
  only through a t27c backend that emits WGSL: a hand-written shader is foreign code, so the
  WebGPU path waits for the compiler rather than for a hand-written exception.
- **The loader** (trinity, `apps/website/public/t27f/live/`): instantiate wasm, open the SSE
  stream, push each event into wasm memory, and on each animation frame call `frame(k)` and
  execute the returned ops. It contains no layout, no colours and no state.
- **Accessibility**: a canvas is invisible to screen readers. The scene's text mirror
  (`a11y_mirror.t27`) is written by the loader into a visually hidden list and an aria-live
  region, as plain text nodes. This is the same trade Flutter web and Figma make (section 2).

### 5.5 Milestones

Each milestone ends with a number a machine can check. Runs happen on the Railway lab, never on
the Mac.

| # | Milestone | Done when |
|---|---|---|
| M0 | This research doc and the epic | `docs/nona-02-organism/t27f/genui-research.md` on master; epic and sub-issues pass the issue-shape gate |
| M1 | `work_graph.t27` from a recorded Queen feed | a recorded feed of at least 1000 events replays to the same graph twice (equal sha256); 0 edges without a label; tests pass with 0 vacuous |
| M2 | `scene.t27` + `display_list.t27` + `motion.t27` | a HUD fixture flattens to a fixed op stream (sha256 recorded); per-frame wasm time p99 under 4 ms on the lab for the fixture |
| M3 | `hud.t27` and the loader page `/t27f/live/` in replay mode | the page replays a recorded log at 60 fps with 0 dropped frames over 60 s in a headless browser on the lab; loader at most 40 lines |
| M4 | live mode over SSE | event -> painted frame p50 under 50 ms, p99 under 150 ms, measured in the browser against t27-mcp `--watch` on the lab |
| M5 | `face.t27`, the vector Queen | six expressions pass the pairwise-distance test; same state and frame give the same points (hash); the face adds under 1 ms p99 per frame |
| M6 | export to MP4 | frames rendered on the lab from a log equal the browser's op stream for the same frames (hash per frame); one 30 s clip for X |
| M7 | clip lane (option B) | a state change swaps the clip at the next clip boundary within 1 frame of the scene clock; a missing clip falls back to the vector face |
| M8 | WebGPU path | t27c emits WGSL from a `.t27` shader spec; same frames as Canvas2D within a stated pixel tolerance |

## 6. Three options

| | A. Pure t27, vector Queen (recommended) | B. A + pre-rendered Queen clips | C. A + a vendor live avatar |
|---|---|---|---|
| What the viewer sees | the HUD in the reference's grammar, live from real agent events, with a stylised vector Queen whose face follows agent state | A, plus a cinematic Queen in a clip slot: a library of short loops (idle, focus, review, blocked, done, failed) generated once, picked per state by a `.t27` table | A, plus a photoreal talking head streamed from a vendor over WebRTC into a scene slot |
| Our format covers | everything except the 40-line loader | everything except the loader and the clip files (assets, like images) | the HUD; the face is the vendor's model and protocol |
| Running cost | $0 per minute; static hosting | one-time generation of about 20-40 clips; then $0 per minute | per-minute vendor billing for every viewer-minute, $0.01 to $0.45 per minute on primary pages (section 3.2) |
| Latency, event to face | one frame (target under 50 ms p50) | the next clip boundary (0.5-2 s by design) | unpublished by most vendors; bitHuman measures 0.05-0.89 s to first speech (section 3.2); plus our event path |
| Deterministic, testable | yes: same log, same frames, hash-checked | the scene yes; the clips are fixed files with checksums | no: the vendor's frames are not ours to hash |
| Same frames to MP4 for X | yes (M6) | yes, the clips composite on the lab | only by screen recording |
| Risk | the vector look must be good enough: an art task, not an engineering one | clip style drifts between generations; needs a style sheet and a review | vendor lock-in, a foreign format at the centre, likeness and consent terms of the vendor |
| Milestones | M0-M6 | M0-M7 | M0-M4, then a vendor slot |

**Recommendation: A now, B as the first upgrade.** A is the only option in which the picture is
true by construction: every pixel comes from a row of the work graph, the same log always gives
the same frames, and the face is ours. B adds the cinematic look of the reference without giving
that up, because the clip ids still come from a `.t27` table and the scene, not the clip, says
what the agents did. C is the right call only if a photoreal talking head is a product
requirement; it is a slot in the same scene, so choosing A first does not close it.

## 7. What this document does not claim

- No part of T27F Live is built. Every latency in sections 5 and 6 that is not marked
  "measured" is a target.
- Vendor prices, latencies and licences in section 3 were read from vendor pages on 2026-10-18
  through the BrowserOS neo browser; vendors change them without notice. Rows marked UNVERIFIED
  could not be confirmed from a primary source.
- The reference post was read on x.com on 2026-10-18; its production method is inferred from the
  post text and its reply, not from the author's tooling.

phi^2 + 1/phi^2 = 3 | TRINITY
