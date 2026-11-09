# Electronics 101: architecture teardown, content tree and learning model

Subject: <https://electronics-101-six.vercel.app/> ("Learn Electronics", by Dilum Sanjaya).
Read on 2026-11-07 from the shipped production bundles. There are no source maps (they return 403),
so every fact below comes from the minified code, read as data with `grep`, `sed` and `dd`. Nothing
from the site was executed outside the browser pane. Identifiers such as `mr`, `Oi` and `Xa` are
the minifier's names in this build and will change with the next deploy.

The lesson prose belongs to the author. This document gives structure, numbers and formulas, which are
facts, and paraphrases the text instead of copying it.

---

## 0. Verdict in five lines

1. **The lesson is data, not code.** 48 lesson records live in one array. Each record carries its
   own closed-form `solve()`, its own Maths text and its own schematic. One generic shell runs all
   of them.
2. **One solve result drives five views:** the 3D bench, the callouts, the schematic, the Maths tab
   and the chart.
3. **The 3D bench is fully procedural.** Three.js builds every part from cylinders, tubes and
   extrudes, with no glTF files. Each lesson's scene spec is diffed by part `key`, so a changed
   part animates in and out instead of the whole scene reloading.
4. **The hand-drawn look comes from a post-process outline pass,** not from textures. It finds
   edges from depth, normals and alpha, in a single full-screen shader.
5. **The pedagogy is "touch, see the number move, read why".** A lesson counts as done on the first
   interaction. There is no quiz or other assessment.

---

## 1. Stack and module tree

| Layer | Choice | Evidence |
|---|---|---|
| Build | Vite, hashed chunks, lazy `import()`; CSS through Lightning CSS | asset names, `__vitePreload` |
| UI | Vanilla JS and DOM, no framework | no React/Vue/Svelte runtime in any chunk |
| 3D | Three.js r17x, bundled | `three-*.js`, 584 KB |
| Hosting | Vercel; the SPA keeps `/<lesson-id>` in the URL through `history.replaceState` | there is no back-stack: Back leaves the site |
| i18n | 21 lazily loaded dictionary chunks, keyed by the full English sentence | `G("English text")` -> Map lookup |
| SEO | `<noscript>` lesson list, plus JSON-LD `WebApplication` + `LearningResource` covering all 48 lessons | `index.html` |
| Storage | `localStorage` only; no backend, no account | keys listed in section 7 |

```
index.html  (artboard markup, JSON-LD, noscript list, design tokens via CSS)
|
+-- index-*.js        294 KB  BOOT + DATA + PHYSICS + UI SHELL
|   +-- lesson data       mr[] : 7 tracks x experiments[]  (48 lessons)
|   +-- physics helpers   bulb, LED diode, transistor bisection, NTC, LDR, power/burn
|   +-- UI shell          sidebar, info card (4 tabs), lesson strip, chart card,
|   |                     circuit dialog, part inspector, callout layer, progress, loader
|   +-- schematic builder j.loop({...}) -> SVG
|   +-- live clock        Xa(): wall-clock dt -> exp.step() -> throttled panel refresh
|   +-- persistence       Pa (done set), saved params, settings, language
|   +-- i18n loader       Lr(lang): fetch chunk, swap [data-i18n] text and attributes
|
+-- stage-*.js         39 KB  BoardStage
|   +-- WebGLRenderer + OrthographicCamera + lights + VSM shadow light
|   +-- camera controller (pan, fling, snap-rotate, cursor-anchored zoom, pinch/twist)
|   +-- tween engine (7 easings) + part diff/animate (drop-in, fall-out)
|   +-- flow dots (current made visible) + hover/press picking
|   +-- on-demand render loop (dirty flag) + frame callback for the live clock
|
+-- scenes-*.js        84 KB  per-lesson scene specs   id -> (values) -> {place, wires, flows}
+-- waterScenes-*.js   88 KB  shared parts library + materials k(color,rough,metal,opts)
|                             + the Basics scenes (Oi) + water-analogy scenes (pipe, tank)
+-- boardview-*.js     11 KB  breadboard SVG generated from the same scene specs
+-- thumbs-*.js        11 KB  offscreen render of every lesson card -> PNG blobs
+-- outline-*.js        5 KB  ink-outline post-process pass
+-- index-*.css        80 KB  layout + tokens (one light scheme; reduced-motion, <=600 px, landscape-phone)
+-- <lang>-*.js        21 dictionaries (es pt fr it de nl pl tr ru uk el ar he id vi ja ko zh zh-TW ...)
```

Boot sequence, from the loader labels in order:

1. Load the language dictionary unless the language is `en`.
2. "Loading fonts": Barlow, Barlow Semi Condensed and JetBrains Mono, plus a glyph subset for
   non-Latin scripts.
3. "Setting out the bench": build `BoardStage`, the lights and the first scene.
4. "Drawing the lessons": render every lesson card thumbnail offscreen.
5. Close the loader. It stays up until both the bench and all thumbnails are ready, so the first
   frame the user sees is complete.

---

## 2. The lesson record (the whole design hangs on this)

Each track is `{id, title, icon, blurb, experiments[]}`. Each experiment (lesson) is a plain object:

```
{
  id, title, cardTitle, thumb,                 // routing + lesson-strip card
  heading, lede, features: [[icon, text]...],  // Overview tab
  components(values) -> [[icon, name, value]], // Components tab (values are live)
  params: [{ key, part, label,                 // the controls; `part` binds a control to a 3D part
             type: toggle | choice | range,
             value, options | min/max/step/unit }],
  power: "none" | "meter" | (default: a power switch),
  live: bool,  initial() -> state,  step(state, values, dt, {on, pressed}),
  chart: bool,
  solve(values, ui, live) -> { amps, leds:{part: level}, flows:{name: speed},
                               label:{part, text}, chart?, burnt?, ... },
  maths(values, solved) -> HTML,               // Maths tab, printed with the user's numbers
  schematic(values, solved) -> j.loop({...}),  // Circuit tab, SVG
  callouts: [{ anchor: "part:<id>", icon, title, body, x, y, w, side, at, leg, ring }],
  goal?, constrain?, watch?                    // challenge / value clamps / live redraw trigger
}
```

Counts over all 48 lessons:

- Every lesson has `callouts`, `schematic` and `maths`.
- 23 are `live` (they keep time-stepped state) and 27 have a `chart`.
- 12 have neither: breadboard, kirchhoff, colour-code, led-polarity, diode, cap-code, slide-switch,
  and-or, two-way, ldr-meter, reed and hall.
- Control types: 35 choice params (plus the `F()` helper that builds choice lists of resistor
  values), 10 range and 5 toggle.
- Most lessons expose one or two controls. `colour-code` is the outlier, with bands, goal, brush,
  three band digits, multiplier and tolerance.
- `power: "none"` appears in 8 lessons, where the circuit is always on. `power: "meter"` appears
  only in ldr-meter.

The scene side mirrors the lesson record. A registry maps each `id` to a function of the current
param values:

```
id -> (values) -> {
  bench: bool,
  place: { partId: { key, build, at, rot,
                     interact: { type, param },   // makes the 3D part itself the control
                     apply(mesh, values) } },     // in-place update, no rebuild
  wires: [...colored tube paths],
  flows: [{ current: "<flow name>", points: [...] }]   // polyline the current dots ride
}
```

Two design rules sit in this shape:

- **A part's `key` is its identity.** When the key changes (the `conductors` sample is
  `big-sample|<material>`, for example), the old mesh falls out and a new one drops in. When the key
  stays the same, `apply()` mutates the mesh: the switch blade angle, the pipe width, the gauge
  needle.
- **`interact.param` closes the loop between the 3D part and the data.** Clicking the knife switch
  writes `values.closed`. The params panel and the 3D part are two views of the same field.

---

## 3. Vertical slice: `/closed-loop` from URL to pixels to progress

The traced lesson is Basics #1, "Open and closed circuits": a battery, a knife switch and a bulb,
with one toggle.

```
URL /closed-loop
 |  boot: find lesson by id in mr[], restore saved values (localStorage), else param defaults
 v
values = { closed: true }
 |
 |  solve(values)                       physics (closed form, no netlist solver)
 v                                      I = closed ? Bn : 0,   Bn = 3 V / V(=10 Ohm) = 0.3 A
solved = { amps: 0.3,
           leds:  { bulb: Un(0.3) },    Un(I) = 0.018 * b^2,  b = min(1.1, (I/0.3)^1.7), 0 below 0.08
           flows: { main: H(0.3) },     H(I) = 0.04 * I   -> dot speed along the flow polyline
           label: { part: "bulb", text: "0.30 A" or "off" } }
 |
 +--> scene spec Oi["closed-loop"](values)
 |      = jr({ rightKind: "switch",            jr = shared "battery + bulb + right slot" loop builder
 |             right: { switch: { key: "switch", build, at,
 |                      interact: { type: "toggle", param: "closed" },
 |                      apply: (mesh, v) => set blade open/closed } },
 |             rightPath, current: "main" })
 |      -> BoardStage diff: same key "switch" -> apply() only; nothing rebuilt
 |      -> flows.main drives dots on the loop polyline; leds.bulb drives emissive glow
 |      -> camera home frame = fit of ki["closed-loop"] (8 corners of a box, Z(-38,41,-18,24,22))
 |
 +--> render (on demand, only when dirty)
 |      normals pass (MeshNormalMaterial override + depth texture)
 |      colour pass (4x MSAA)
 |      outline shader: depth Laplacian + normal crease (0.22) + alpha silhouette -> ink #141a2c
 |
 +--> callout layer: 3 cards (battery, switch, bulb); each SVG leader line ends at the
 |    projected screen position of "part:<id>", re-projected as the camera moves
 |
 +--> info card tabs, all fed by the same (values, solved):
 |      Overview    heading + lede + 3 feature rows
 |      Components  battery 3 V | bulb 10 Ohm | knife switch open/closed | 3 wires
 |      Circuit     j.loop({ parts:[switch(closed)], led:{lit}, current }) -> SVG schematic,
 |                  Diagram/Breadboard toggle (boardview chunk), x-ray, zoom
 |      Maths       I = V / R printed with this lesson's numbers; the open case explains R -> inf, I = 0
 |
 v
user clicks the 3D switch  (or the toggle in the Circuit tab: same field)
 |  values.closed = false -> save to localStorage -> solve() -> every view above refreshes
 |  Qa(): Pa.complete("closed-loop") -> add to the "done" set -> progress "BASICS 1/9"
 v
next lesson: "]" key or lesson strip; "[" goes back
```

The slice shows four things:

- **No view owns any state.** The 3D bench, the panels and the schematic all re-derive from
  `(values, solve(values))`.
- **The bulb is a constant reference load.** The same 3 V battery and 10 Ohm bulb (0.3 A nominal)
  recur through Basics, so later lessons compare against a brightness the learner already knows.
- **Text tracks the state.** The Maths tab is a function of the live values, not a static page,
  and the explanation changes when the switch opens.
- **Progress is touch-based.** The first change to a param, a power toggle or a part press marks
  the lesson done.

---

## 4. Live slice: `cap-charge` (time made visible)

`cap-charge` is `live:true` and `chart:true`. Its state is `{vc, on, pressed, hist[], empty, ...}`.

- **Clock.** The stage's frame callback calls `Xa()`. Here `dt` is the wall-clock time since the
  last frame, capped at 1 s, and the lesson's `step(state, values, dt, {on, pressed})` runs once.
- **Integration is exact, not Euler:**
  - Charging: `vc <- 3 + (vc - 3) * exp(-dt / (R * C))`.
  - Pressing the button discharges through 10 Ohm: `vc <- vc * exp(-dt / (10 * C))`.
  - Because the step is exact, any frame rate gives the same curve, and a 1 s stall cannot blow up.
- **Solve** reads the live state:
  - `I = on ? (3 - vc) / R : 0` and `tau = R * C`.
  - The chart carries a marker at 1.896 V, which is 63% of 3 V, labelled as one tau.
- **Maths** prints `tau = R x C` with the chosen R and C. It also prints the current at this moment
  and `Q = C x V` for the present voltage.
- **Panel refresh is throttled.** Panels redraw after 120 ms when the watched value has moved by
  more than 0.002 or the on/pressed state has changed. Otherwise they redraw once per second. The
  3D scene still runs at the frame rate.

The same pattern covers the other 22 live lessons:

- battery drain in `cells`;
- AC/DC with a chosen frequency, plus red and green LEDs with exponential smoothing;
- the capacitor tank, fade, flash, pair, smoothing and delay;
- the two-transistor flasher;
- the thermistor and heat alarm.

---

## 5. Render pipeline (stage chunk)

| Concern | Implementation |
|---|---|
| Renderer | WebGL with `antialias:false` (MSAA happens in the colour pass instead), alpha, premultiplied alpha, `powerPreference: high-performance` |
| Camera | Orthographic. Azimuth starts at 36 deg and snaps in 90 deg steps (Q/E); elevation is 43 deg. T switches to top view and R resets |
| Controls | Drag or WASD/arrows pan, with fling inertia. Middle-drag rotates with a ratchet. The wheel zooms exponentially, anchored at the cursor, clamped to 3.2-22. Touch supports pinch and twist. The target is clamped to the scene bounds |
| Light | RoomEnvironment PMREM at intensity 0.4, plus a key directional light `#fff6ea` at 1.2 |
| Shadows | A separate shadow-only light at intensity 0.001 casts 2048^2 VSM shadows (radius 11, 20 blur samples, bias -4e-4). `shadowMap.autoUpdate=false`; shadows re-render only when `shadowDirty` is set, i.e. when parts move |
| Ground | Semi-transparent plane `#7a5532` at opacity 0.32, plus a grid |
| Materials | `MeshStandardMaterial` through the helper `k(color, roughness, metalness, opts)`. There are no textures |
| Outline | 3 passes (normals+depth, colour, edge shader). Lines are `#141a2c`, thickness 1.3, scaled with zoom; an outer ring at x1.5 with weight 0.35 softens them. The pass also offers an sRGB output path for the thumbnails |
| Motion | Tween engine with easings linear, inOutCubic, outCubic, outQuart, inCubic, outBack and inBack. Parts drop in with outBack and leave with inBack plus a fall |
| Current | Dots move along each `flows[].points` polyline at `0.04 * I`, so a current of zero leaves the dots still |
| Loop | On demand: `dirty` is set by input, tweens, flows or live lessons, and an idle bench costs nothing |
| Thumbnails | The same stage renders each lesson offscreen into a PNG blob for the lesson strip |

---

## 6. Physics models (closed form per lesson)

There is no general circuit solver (MNA or netlist). Each lesson writes the equation of its own
circuit.

| Model | Formula in the bundle | Used by |
|---|---|---|
| Bulb | R = 10 Ohm, so I = 0.3 A at 3 V. Brightness b = min(1.1, (I/0.3)^1.7), set to 0 below 0.08. Glow = 0.018 b^2 | Basics, switches |
| LED | Smoothed ideal diode plus series R: `I = r*softplus((V-Vf)/r)/max(R,1)` with r = 0.05 (linear branch above 20). Forward voltages: red 2.0, yellow 2.1, green 2.2, blue 3.0, white 3.1. Brightness = sqrt(I / 18 mA) | LEDs track, indicators |
| Burn-out | Resistor power is graded warm / hot within rating / too hot; it burns out at 260% (`On=260`). The LED has a current limit. `burnt` appears 18 times | led-burn, power |
| BJT | Diode equation with Is = 7e-15 and Vt = 25.85 mV, solved by a 48-step bisection; beta ~ 200 | transistors, sensors |
| NTC | `R = 1e4 * exp(3950 * (1/(T+273.15) - 1/298.15))` | thermistor, heat-alarm |
| LDR | `R = 1e6 / 10^(3x/100)`, which runs from 1 MOhm down to 1 kOhm over the slider | ldr-meter, night-light, night-set |
| RC | Exact exponential step (section 4) | the capacitors track |
| Meter | Per-test tables: dial position, socket, probe placement, and a reading computed from the solved circuit | multimeter |

Chart spec: `zt(label, xAxis, yAxis, fn, point)` returns an x-y curve plus the "you are here" point.
It is drawn in a resizable, foldable SVG card. Live lessons push samples into `state.hist`.

---

## 7. UI shell, persistence, i18n

- **Artboard.** A fixed 1672 x 941 px artboard is scaled uniformly to the viewport, so the layout
  is identical on every screen and only the scale changes.
- **Sidebar:** Home, Lessons, Components (parts library), Settings, Language.
- **Info card:** four tabs, Overview, Components, Circuit and Maths.
- **Bench overlays:**
  - the lesson strip with previous/next (`[` and `]`);
  - the progress card, "TRACK n/N";
  - a clock;
  - a view bar with turn, top view and reset;
  - a chart card.
- **Dialogs:**
  - a part inspector, which opens on a click on a highlighted part;
  - the circuit dialog, with Diagram/Breadboard, x-ray and zoom.
- **localStorage keys:**
  - `learn-electronics:done` (the set of finished lesson ids);
  - `learn-electronics:lang`;
  - saved param values per lesson;
  - the board view mode;
  - `electronics:board-xray`;
  - settings: `reducedMotion`, `labels` and `autoRotate`.
- **i18n.** `G(english)` looks the string up in the dictionary loaded for the language. DOM nodes
  carry `data-i18n` and `data-i18n-attr`. `?lang=xx` forces a language, and `?i18n=collect` records
  every requested key into `window.__i18nKeys`, which is how the dictionaries are harvested. Keying
  by the full English sentence means a typo fix in English orphans its translations.
- **Design tokens:**
  - `--bg #f6eddd`, `--bg-deep #efe3d1` and `--card #f9f2e5` give the paper look;
  - `--navy #18213d` is the ink and `--ink #4f535b` the body text;
  - `--teal #1aa3a1` is the accent and `--trace #d3bea3` the copper trace.
  - There is one light scheme only.

---

## 8. Content tree (7 tracks, 48 lessons)

Controls are `key[type]`. `F` marks a choice list of resistor values. `L` means live (time-stepped)
and `C` means it has a chart.

```
Learn Electronics
|
+-- 1 Basics (9) ............................ icon bolt
|   +-- closed-loop      closed[toggle]                 L     power none
|   +-- conductors       material[choice]               L     power none
|   +-- ohms-law         cells[choice] ohms[F]          L C   power none   (water-pipe analogy)
|   +-- circuit-types    type[choice]                   L     power none
|   +-- cells            cells[choice]                  L C   (battery drain)
|   +-- ac-dc            mode[choice] freq[choice]      L C   power none
|   +-- breadboard       step[choice] view[choice]
|   +-- multimeter       test[choice]                   L     power none   (6 meter tests)
|   +-- kirchhoff        show[choice] r1 r2 r3[F]
|
+-- 2 Resistors (8) ......................... icon resistor
|   +-- resistor         ohms[F]                          C
|   +-- colour-code      bands goal brush band1-3 mult tol      power none   (goal challenge)
|   +-- series           r1 r2[F]                         C
|   +-- parallel         r1 r2[F]                         C
|   +-- divider          r1 r2[F]                         C
|   +-- pot              pot[range]                       C
|   +-- power            cells[choice] ohms[F] watts[choice]  L C   (heat / burn-out)
|   +-- bridge           pot[range] rx[choice]            C
|
+-- 3 LEDs & diodes (5) ..................... icon led
|   +-- led-polarity     flip[choice]
|   +-- led-colours      color cells[choice]              C
|   +-- led-burn         ohms[choice]                     C   (burn-out)
|   +-- diode            dir[choice]
|   +-- led-series       cells[choice] color2             C
|
+-- 4 Capacitors (9) ........................ icon cap
|   +-- cap-tank         tank[choice] pipe[choice]      L C   (water-tank analogy)
|   +-- cap-charge       ohms[F]                        L C
|   +-- cap-fade                                        L C
|   +-- cap-flash                                       L C
|   +-- cap-pair         pair[choice]                   L C
|   +-- cap-smooth       smooth[choice] speed[choice]   L C
|   +-- cap-delay                                       L C
|   +-- cap-code         d1 d2 zeros[range]                   power none
|   +-- flasher          c1 c2                          L C   (uses 2 transistors: forward dependency)
|
+-- 5 Switches & buttons (4) ................ icon switch
|   +-- push-button                                     L
|   +-- slide-switch     sw[choice]
|   +-- and-or           s1 s2[toggle] wiring[choice]
|   +-- two-way          s1 s2[choice]
|
+-- 6 Transistors (6) ....................... icon npn
|   +-- transistor-control  pot[range]                   C
|   +-- transistor-switch                              L
|   +-- transistor-gain     rb[F]                        C
|   +-- npn-pnp             kind[choice]               L
|   +-- darlington          pair[choice]               L
|   +-- transistor-not                                 L
|
+-- 7 Sensors (7) ........................... icon sensor
    +-- ldr-meter        light[range]                         power meter
    +-- night-light      light[range]                     C
    +-- thermistor                                      L C
    +-- reed             magnet[toggle]
    +-- heat-alarm                                      L C
    +-- night-set        trim[range] light[range]         C
    +-- hall             magnet[choice/toggle]
```

Prerequisite graph as the track order implies it:

```
Basics --> Resistors --> LEDs & diodes --> Capacitors --> Switches --> Transistors --> Sensors
   \            \______ divider _____________________________________________/      ^
    \______________ switch, loop ___________________________/                       |
                                       flasher (Capacitors #9) needs Transistors ---+  (out of order)
```

---

## 9. The learning model

### 9.1 The per-lesson loop the site runs

| Beat | What the learner does | What the system does | Where |
|---|---|---|---|
| 1 Claim | reads one sentence | states one idea as a heading plus a short lede | Overview tab |
| 2 Look | orbits the bench | shows a real-looking circuit; 3 callouts name the parts that matter | 3D + callouts |
| 3 Poke | clicks the highlighted part or a control | changes one variable, then re-solves in the same frame | `interact.param`, params |
| 4 See | watches the current dots, glow, meter and chart | turns the invisible (current, charge, heat) into motion and light | flows, leds, chart |
| 5 Explain | opens Maths | prints the governing equation with the learner's current numbers | `maths(values, solved)` |
| 6 Transfer | opens Circuit and flips to Breadboard | shows the same circuit as a schematic and as a breadboard | `schematic()`, boardview |
| 7 Mark | (automatic) | marks the lesson done on the first touch and moves the track counter | `Pa.complete` |

### 9.2 The principles under it

1. **Concrete -> symbolic -> numeric.** The learner meets the object first (3D), the symbol second
   (schematic) and the number third (Maths with live values). That is the concrete-representational-
   abstract order, built into the tab order.
2. **One variable per lesson.** Most lessons have one or two controls, so a change in the output
   has exactly one cause.
3. **Zero-latency cause and effect.** A closed-form `solve()` makes every input produce its output
   in the same frame. The learner experiments, not waits.
4. **Make the invisible visible.** Current becomes moving dots at a speed proportional to I. Charge
   becomes a filling tank or a voltage curve. Heat becomes a warm-hot-burnt state.
5. **Analogy as a bridge, then retire it.** The water pipe (Ohm's law) and water tank (capacitor)
   open their tracks. The real part follows in the very next lesson.
6. **A fixed reference load.** The 3 V battery and 10 Ohm bulb come back again and again, so
   brightness is a unit the learner already reads.
7. **Safe failure.** An LED without a resistor burns out, and an overloaded resistor runs hot and
   then burns. Breaking the circuit is taught, not prevented.
8. **Measurement literacy.** The multimeter lesson teaches where the probes and the dial go for each
   of 6 quantities, so the bench maps onto real tools.
9. **Three representations of one circuit.** The 3D bench, the schematic and the breadboard all
   come from one spec, which trains the learner to translate between them.
10. **Low-friction progress.** There is no quiz and no gate. Touching a lesson completes it, and
    progress is counted per track.

### 9.3 Where the model is thin

| Gap | Consequence | Cheap fix in the same architecture |
|---|---|---|
| Done on touch | Progress measures exposure, not understanding | Add a **Predict** beat before Poke: "what will I be?". `solve()` already knows the answer, so it can grade the prediction for free |
| No retrieval | Nothing brings back an old lesson | Reuse the reference bulb: ask for an earlier lesson's number inside a later bench |
| Only one challenge (`goal` in colour-code) | Little productive struggle | A `goal` on every bench ("make the LED draw 10 mA"), checked by `solve()` |
| Fixed benches | No free build | The parts library and scene specs already exist; a sandbox would need a general solver (MNA), which the site avoids |
| `flasher` before Transistors | A forward dependency inside Capacitors | Move it to the end of Transistors, or mark it as a preview |
| `replaceState` routing | Back leaves the site; lessons cannot be revisited through history | `pushState` per lesson |
| English sentence as the i18n key | A copy edit silently orphans its translations | Stable keys |

### 9.4 A stronger loop: PREDICT - POKE - EXPLAIN - TRANSFER - PROVE

```
Claim -> Look -> PREDICT (number or yes/no, graded by solve) -> Poke -> See
      -> Explain (maths with live values) -> Transfer (schematic / breadboard)
      -> PROVE (goal: reach a target value; done = goal met, not touch)
```

Mastery could be "prediction right twice in a row, plus the goal met", which still costs only one
`solve()` per check.

---

## 10. What t27 can take from this

Mapping this onto t27 is a proposal, not something built. Under the course-27 format (9 modules x
3 lessons), the 48 lessons fold into 27 like this:

| Module | Lesson 1 | Lesson 2 | Lesson 3 |
|---|---|---|---|
| M1 The loop | closed-loop | conductors | circuit-types |
| M2 Ohm's law | ohms-law (pipe analogy) | resistor | power (burn-out) |
| M3 Combining resistance | series | parallel | divider |
| M4 Measuring | multimeter | kirchhoff | colour-code |
| M5 Diodes and LEDs | diode | led-burn | led-colours |
| M6 Capacitors I | cap-tank (analogy) | cap-charge (tau) | cap-fade |
| M7 Capacitors II | cap-smooth | cap-delay | ac-dc |
| M8 Transistors | transistor-switch | transistor-gain | darlington |
| M9 Sensing | night-light | heat-alarm | hall |

**The reusable idea is the lesson record as a spec.** In t27 a lesson could be one `.t27` module:

- **Params** become typed constants with ranges.
- **`solve`** becomes a pure function.
- **The equations the Maths tab prints become `test` blocks.** Closed-loop asserts
  `I = 3 / 10 = 0.3`; cap-charge asserts that after one tau `vc` is within tolerance of
  `3 * (1 - e^-1) = 1.896`.

That gives three properties:

- The number the learner reads is the number a test checked, which the site cannot claim.
- The view stays a projection of the spec, which is T27F's job (`specs/t27f/frontend.t27`).
- Under only-t27, any port is t27 specs plus generated code; none of this site's JS would be copied.

---

## 11. What was not checked

- The full contents of the Components library page and of the Settings panel. Only the setting
  keys listed above were seen.
- The internals of the thumbnail render loop and the boardview SVG generator, which were sized but
  not traced.
- Whether any lesson substeps time inside `step()`. The AC lesson may; only `cap-charge` and
  `cap-tank` were read line by line.
- Accessibility beyond the `aria-valuenow` on the progress bar.
- Performance, which was not measured. No claim is made about frame rate or load time.
