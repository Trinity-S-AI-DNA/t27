// Host of course_hud.wasm. It chooses nothing: events in (hud_event), one raster out (putImageData).
// fill_rect, draw_text and clear happen inside the wasm, in the t27 painters. Owner-approved 2026-11-08 (tools/policy/foreign-exceptions.txt).
const W = 375, H = 812, KEY = { ArrowUp: 1, ArrowDown: 2, Enter: 3, Escape: 4, Backspace: 4, m: 6 };
const c = document.getElementById('hud'), ctx = c.getContext('2d'), img = ctx.createImageData(W, H);
const { instance } = await WebAssembly.instantiateStreaming(fetch('course_hud.wasm'));
const w = instance.exports;
function paint() {
  w.hud_paint();
  const rgb = new Uint8Array(w.memory.buffer, w.fb_ptr(), W * H * 3), px = img.data;
  for (let i = 0, j = 0; j < px.length; i += 3, j += 4) { px[j] = rgb[i]; px[j + 1] = rgb[i + 1]; px[j + 2] = rgb[i + 2]; px[j + 3] = 255; }
  ctx.putImageData(img, 0, 0);
  document.title = 'course_hud wasm: screen ' + w.hud_screen();
}
w.hud_reset(); paint();
addEventListener('keydown', e => {
  if (KEY[e.key]) { w.hud_event(KEY[e.key], -1, -1); paint(); e.preventDefault(); }
  else if (e.key >= '1' && e.key <= '3') { w.hud_answer(+e.key); paint(); }
});
c.addEventListener('click', e => { const r = c.getBoundingClientRect(); w.hud_event(5, e.clientX - r.left | 0, e.clientY - r.top | 0); paint(); });
setInterval(() => { w.hud_event(7, -1, -1); paint(); }, 1000);
