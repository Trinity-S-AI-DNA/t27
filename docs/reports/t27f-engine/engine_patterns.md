# Engine Patterns Report

This report compares the M0 rendering engine's patterns against Three.js equivalents, highlighting current implementation and missing features.

| Pattern | What it is | Three.js source | T27F today | Missing | Owner spec |
|---------|------------|----------------|-------------|---------|-------------|
| scene graph | A tree of nodes, each with a parent, visited top down | src/core/Object3D.js | VfGraph in visual_factory_contract.t27, a flat node table where each node names its parent; vf_layout walks it | per-node transform (only rectangles from layout, no translate, scale or rotate) | visual_factory_contract.t27 |
| dirty flags and update pass | A node marks itself changed and only changed subtrees are recomputed before drawing | src/core/Object3D.js (matrixWorldNeedsUpdate, updateMatrixWorld) | MISSING; vf_layout(g) is a pure function that lays out the whole graph on every call | a per-node changed mark and a layout pass that reuses unchanged rectangles | vf_layout.t27 |
| render list and sort | Visible objects are collected into a list and sorted (opaque front to back, transparent back to front) before drawing | src/renderers/webgl/WebGLRenderLists.js | vf_card paints nodes in node index order, the highest index on top (vf_pixel states the same rule) | an explicit z order and a visibility cull | vf_card.t27 |
| material and geometry split | Shape and look are separate objects, so one look is reused across shapes | src/materials/Material.js and src/core/BufferGeometry.js | vf_theme gives fill, outline and label colour per theme and node kind (the look); vf_layout gives the rectangles (the shape); vf_tokens holds the 32 values both read | a per-node look override; today the look is fixed by kind | vf_theme.t27 |
| render target | Drawing goes into a named buffer that can be the screen or an offscreen image | src/core/RenderTarget.js and src/renderers/WebGLRenderTarget.js | vf_card writes into one RGB byte buffer the caller passes in, which the host saves as a PPM frame | more than one target and composing one target into another | vf_card.t27 |
| loader and cache | Assets are loaded once by key and reused | src/loaders/Loader.js and src/loaders/Cache.js | MISSING; there are no external assets (the font is the built-in 5x7 glyph set, colours and sizes are tokens). vf_tokens resolves aliases with a hop limit of 8 and a cycle check, without a cache | nothing needed until images or fonts are loaded | vf_tokens.t27 |
| frame loop | A callback redraws on every display refresh | src/renderers/webgl/WebGLAnimation.js, started from src/renderers/WebGLRenderer.js (setAnimationLoop) | MISSING; one program run makes one frame. hot.t27 decides when a saved spec is exported again, which is redraw on change, not a loop | a clock and a redraw-on-change rule for screens | hot.t27 |

## Follow-up tasks
1. vf_layout.t27: add a test that states the layout of an unchanged graph is equal on a second call, as the base for a later changed-node pass.
2. vf_card.t27: state the paint order rule (node index, highest on top) as a test.
3. visual_factory_contract.t27: decide whether a node may carry an offset, and record the decision as a constant with a test.
4. vf_theme.t27: add a test that every node kind has a fill, an outline and a label colour in every theme.
5. vf_diff.t27: host program so two frames can be compared from the command line (already filed as issue 13662; do not file it twice).

## Limits of this report
- The Three.js paths were written from memory of the public repository and were not opened in this run; treat each as "path not verified".
- The T27F column was checked against declaration lists and header comments of the named specs, not against a full read of each spec.
- No measurement of speed was made; nothing here claims one engine is faster.