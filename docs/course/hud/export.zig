// Export glue for course_hud.wasm, the one hand-written input (see specs/t27f/build.t27).
// Host chooses nothing: events in, one RGB raster out. Owner-approved 2026-11-08 (tools/policy/foreign-exceptions.txt).
const h = @import("hm.zig");
var st = h.hud_init();
var lrn = h.learner_init();
var fb: [h.RGB_N]u8 = undefined;
export fn hud_reset() void { st = h.hud_init(); lrn = h.learner_init(); }
export fn hud_event(ev: u8, x: i32, y: i32) void { st = h.hud_step(st, ev, x, y); }
export fn hud_answer(a: u32) void { lrn = h.learner_answer(lrn, st.scene, a); }
export fn hud_paint() void { h.hud_render(st, lrn, &fb); }
export fn fb_ptr() usize { return @intFromPtr(&fb); }
export fn hud_screen() u8 { return st.screen; }
