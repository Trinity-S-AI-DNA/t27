// Shared low-poly game engine for the shop's games.
//
// Every game here is an ES module that three.js is imported into on demand —
// never part of the wasm bundle, which every customer who opens the menu has
// to download whether or not they ever play. The Rust side stays a thin shell:
// HUD, score submission, and a `stop()` it can call on unmount.
//
// This module is the part the games have in common: the renderer and its
// sizing, the sky, the lighting rig, the mascot, and the pointer input. Woody
// himself lives here because he is the brand's face and must be the same bird
// in every game — built from primitives rather than a loaded model, for the
// same reason three.js is lazy: no extra megabytes on a phone in a beach bar.

import * as THREE from '/assets/vendor/three.module.min.js';

export { THREE };

// ── Palette ─────────────────────────────────────────────────────────────────
// Taken off assets/logo.jpg.
export const CREST = 0xf24b3a;
export const FEATHER = 0xe0342a;
export const BEAK = 0xf2a327;
export const HOODIE = 0x2f63d8;
export const HOODIE_DARK = 0x1f47a8;
export const TRIM = 0xf6f8ff;
export const FOOT = 0xe8902a;
export const NEON = 0x39ff14;

export function rand(a, b) { return a + Math.random() * (b - a); }
export function pick(arr) { return arr[(Math.random() * arr.length) | 0]; }

export function flat(color) {
  return new THREE.MeshLambertMaterial({ color, flatShading: true });
}

export function mesh(geo, color) {
  return new THREE.Mesh(geo, flat(color));
}

/// A vertical gradient for `scene.background`. Four pixels wide because only
/// the vertical axis carries information. The pale stop belongs on the horizon
/// — put it lower and the sky is still saturated where the fog has already
/// gone white, and the seam shows as a band.
export function skyTexture(top, mid, low, horizonAt = 0.52) {
  const c = document.createElement('canvas');
  c.width = 4; c.height = 256;
  const g = c.getContext('2d');
  const grad = g.createLinearGradient(0, 0, 0, 256);
  grad.addColorStop(0, top);
  grad.addColorStop(Math.max(0, horizonAt - 0.14), mid);
  grad.addColorStop(horizonAt, low);
  grad.addColorStop(1, low);
  g.fillStyle = grad;
  g.fillRect(0, 0, 4, 256);
  const t = new THREE.CanvasTexture(c);
  t.colorSpace = THREE.SRGBColorSpace;
  return t;
}

/// Three lights, each doing one job: hemisphere for the ambient sky/ground
/// bounce, a warm key for shape, a dim cool fill so shadowed sides do not go
/// flat black. Flat-shaded low-poly needs the fill more than a smooth model
/// does — its facets go abruptly dark.
export function addLights(scene, { sky = 0xd8f0ff, ground = 0x6f8f5a } = {}) {
  scene.add(new THREE.HemisphereLight(sky, ground, 1.0));
  const key = new THREE.DirectionalLight(0xfff2d0, 0.85);
  key.position.set(-8, 16, 6);
  scene.add(key);
  const fill = new THREE.DirectionalLight(0x9fd0ff, 0.25);
  fill.position.set(7, 6, -8);
  scene.add(fill);
  return { key, fill };
}

/// Renderer + scene + camera, wired to the container's real size.
///
/// The container is usually still unlaid-out on the frame a game mounts, so
/// `clientWidth` reads 0 and the canvas is created 0x0 — the game then runs
/// perfectly and draws nothing. Measure with fallbacks and keep watching: a
/// window `resize` never fires when the element itself is laid out late, which
/// is exactly that case.
export function createStage(container, { fov = 62, near = 0.1, far = 500 } = {}) {
  function measure() {
    const w = container.clientWidth || container.parentElement?.clientWidth || window.innerWidth;
    const h = container.clientHeight || container.parentElement?.clientHeight || window.innerHeight;
    return { w: Math.max(1, w), h: Math.max(1, h) };
  }

  const renderer = new THREE.WebGLRenderer({ antialias: true, powerPreference: 'low-power' });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  const first = measure();
  renderer.setSize(first.w, first.h);
  renderer.domElement.style.display = 'block';
  container.appendChild(renderer.domElement);

  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(fov, first.w / first.h, near, far);

  const onResize = () => {
    const { w, h } = measure();
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    renderer.setSize(w, h);
  };
  window.addEventListener('resize', onResize);
  let ro = null;
  if (typeof ResizeObserver !== 'undefined') {
    ro = new ResizeObserver(onResize);
    ro.observe(container);
  }
  requestAnimationFrame(onResize);

  function dispose() {
    window.removeEventListener('resize', onResize);
    if (ro) ro.disconnect();
    // Without this the GL context leaks; a few screen visits and the browser
    // starts refusing to create new ones.
    renderer.dispose();
    const el = renderer.domElement;
    if (el.parentNode) el.parentNode.removeChild(el);
  }

  return { renderer, scene, camera, measure, onResize, dispose };
}

/// Woody, the brand's woodpecker.
///
/// The silhouette is what has to survive being six metres away at speed, so
/// the swept-back red crest and the long beak are oversized on purpose — they
/// are what people read as "Woody" from across the screen.
///
/// Returns the group plus the sub-groups a game may want to animate.
export function buildWoody({ headTurn = -0.75 } = {}) {
  const bird = new THREE.Group();
  const headGroup = new THREE.Group();
  const tail = new THREE.Group();
  const armL = new THREE.Group();
  const armR = new THREE.Group();

  for (const sx of [-0.2, 0.2]) {
    const leg = mesh(new THREE.CylinderGeometry(0.07, 0.07, 0.42, 6), FOOT);
    leg.position.set(sx, 0.26, 0);
    bird.add(leg);
    const foot = mesh(new THREE.BoxGeometry(0.3, 0.09, 0.52), FOOT);
    foot.position.set(sx, 0.06, 0);
    bird.add(foot);
  }

  const body = mesh(new THREE.CapsuleGeometry(0.42, 0.42, 3, 8), HOODIE);
  body.position.y = 0.95;
  bird.add(body);

  // Hood bunched at the back, and the white fur trim from the logo.
  const hood = mesh(new THREE.IcosahedronGeometry(0.3, 0), HOODIE_DARK);
  hood.position.set(0, 1.24, 0.3);
  bird.add(hood);
  const collar = mesh(new THREE.TorusGeometry(0.34, 0.11, 4, 10), TRIM);
  collar.rotation.x = Math.PI / 2;
  collar.position.y = 1.3;
  bird.add(collar);
  const zip = mesh(new THREE.BoxGeometry(0.07, 0.62, 0.04), TRIM);
  zip.position.set(0, 0.95, -0.41);
  bird.add(zip);

  // Each arm in its own group so it swings about the shoulder rather than
  // about its own centre.
  for (const [group, sx] of [[armL, -1], [armR, 1]]) {
    const upper = mesh(new THREE.BoxGeometry(0.17, 0.52, 0.17), HOODIE);
    upper.position.y = -0.26;
    const cuff = mesh(new THREE.BoxGeometry(0.19, 0.1, 0.19), TRIM);
    cuff.position.y = -0.5;
    const glove = mesh(new THREE.IcosahedronGeometry(0.16, 0), TRIM);
    glove.position.y = -0.62;
    group.add(upper, cuff, glove);
    group.position.set(sx * 0.45, 1.15, 0);
    group.rotation.z = sx * 0.95;
    bird.add(group);
  }

  for (let i = -1; i <= 1; i++) {
    const feather = mesh(new THREE.BoxGeometry(0.15, 0.06, 0.8), FEATHER);
    feather.position.set(i * 0.14, 0, 0.4);
    feather.rotation.set(-0.25, i * 0.22, 0);
    tail.add(feather);
  }
  tail.position.set(0, 0.78, 0.32);
  bird.add(tail);

  const neck = mesh(new THREE.CylinderGeometry(0.15, 0.19, 0.3, 7), FEATHER);
  neck.position.y = 1.44;
  bird.add(neck);

  headGroup.add(mesh(new THREE.IcosahedronGeometry(0.33, 0), FEATHER));

  const upperBeak = mesh(new THREE.ConeGeometry(0.13, 0.66, 5), BEAK);
  upperBeak.rotation.x = -Math.PI / 2;
  upperBeak.position.set(0, 0.02, -0.5);
  const lowerBeak = mesh(new THREE.ConeGeometry(0.1, 0.5, 5), 0xd98a15);
  lowerBeak.rotation.x = -Math.PI / 2;
  lowerBeak.position.set(0, -0.12, -0.42);
  headGroup.add(upperBeak, lowerBeak);

  for (const sx of [-1, 1]) {
    const white = mesh(new THREE.IcosahedronGeometry(0.13, 0), 0xffffff);
    white.position.set(sx * 0.19, 0.1, -0.22);
    const pupil = mesh(new THREE.IcosahedronGeometry(0.06, 0), 0x11131a);
    pupil.position.set(sx * 0.22, 0.1, -0.31);
    headGroup.add(white, pupil);
  }

  // The comb: three cones sweeping back over the skull.
  for (const [r, h, z, tilt] of [[0.17, 0.62, 0.16, -0.9], [0.15, 0.52, 0.36, -1.15], [0.12, 0.4, 0.54, -1.35]]) {
    const spike = mesh(new THREE.ConeGeometry(r, h, 5), CREST);
    spike.position.set(0, 0.3, z);
    spike.rotation.x = tilt;
    headGroup.add(spike);
  }

  headGroup.position.y = 1.78;
  headGroup.rotation.y = headTurn;
  bird.add(headGroup);

  return { bird, headGroup, tail, armL, armR };
}

/// A fake contact shadow. Real shadow maps cost far more than this on a phone
/// and buy nothing at this scale, but without *something* a character looks
/// like it is hovering.
export function contactShadow(radiusX = 0.75, radiusZ = 1.5) {
  const shadow = new THREE.Mesh(
    new THREE.CircleGeometry(1, 16),
    new THREE.MeshBasicMaterial({ color: 0x0a1a08, transparent: true, opacity: 0.28, depthWrite: false }));
  shadow.rotation.x = -Math.PI / 2;
  shadow.scale.set(radiusX, radiusZ, 1);
  return shadow;
}

/// Recycled motes in a single InstancedMesh — one draw call, and it does more
/// for the sense of speed than any amount of extra scenery.
export class Dust {
  constructor(scene, count = 36) {
    this.count = count;
    this.dummy = new THREE.Object3D();
    this.motes = [];
    for (let i = 0; i < count; i++) this.motes.push({ x: 0, y: 0, z: 0, life: 0, vx: 0, vy: 0, scale: 1 });
    this.mesh = new THREE.InstancedMesh(
      new THREE.IcosahedronGeometry(0.09, 0),
      // Unlit, or the motes turn into little grey pebbles bouncing off the road.
      new THREE.MeshBasicMaterial({ color: 0xf3f7ee, transparent: true, opacity: 0.4, depthWrite: false }),
      count);
    this.mesh.frustumCulled = false;
    scene.add(this.mesh);
  }

  spawn(x, y, z) {
    const m = this.motes[(Math.random() * this.count) | 0];
    m.x = x + rand(-0.4, 0.4);
    m.y = y;
    m.z = z;
    m.vx = rand(-1.2, 1.2);
    m.vy = rand(1.2, 3.0);
    m.life = 1;
    m.scale = rand(0.6, 1.4);
  }

  update(dt, drift = 0) {
    for (let i = 0; i < this.count; i++) {
      const m = this.motes[i];
      if (m.life <= 0) {
        this.dummy.scale.setScalar(0);
        this.dummy.position.set(0, -50, 0);
      } else {
        m.life -= dt * 1.6;
        m.x += m.vx * dt;
        m.y += m.vy * dt;
        m.vy -= 3.2 * dt;
        m.z += drift * dt;
        this.dummy.position.set(m.x, Math.max(0.06, m.y), m.z);
        this.dummy.scale.setScalar(Math.max(0, m.life) * m.scale);
      }
      this.dummy.rotation.set(m.life * 4, m.life * 3, 0);
      this.dummy.updateMatrix();
      this.mesh.setMatrixAt(i, this.dummy.matrix);
    }
    this.mesh.instanceMatrix.needsUpdate = true;
  }
}

/// Horizontal pointer/keyboard input, normalised to -1..1.
///
/// Steering is a target the game eases toward, so a touch that jumps across
/// the screen does not teleport the player. Returns a `detach()`.
export function horizontalInput(el, onValue, { gain = 2.4 } = {}) {
  let active = false;
  const fromX = (clientX) => {
    const rect = el.getBoundingClientRect();
    const t = (clientX - rect.left) / rect.width;
    onValue(Math.max(-1, Math.min(1, (t - 0.5) * gain)));
  };
  const down = (e) => { active = true; fromX(e.touches ? e.touches[0].clientX : e.clientX); };
  const move = (e) => { if (active) fromX(e.touches ? e.touches[0].clientX : e.clientX); };
  const up = () => { active = false; onValue(0); };
  const key = (e, isDown) => {
    if (e.key === 'ArrowLeft' || e.key === 'a') onValue(isDown ? -1 : 0);
    if (e.key === 'ArrowRight' || e.key === 'd') onValue(isDown ? 1 : 0);
  };
  const keyDown = (e) => key(e, true);
  const keyUp = (e) => key(e, false);

  el.addEventListener('touchstart', down, { passive: true });
  el.addEventListener('touchmove', move, { passive: true });
  el.addEventListener('touchend', up, { passive: true });
  el.addEventListener('mousedown', down);
  window.addEventListener('mousemove', move);
  window.addEventListener('mouseup', up);
  window.addEventListener('keydown', keyDown);
  window.addEventListener('keyup', keyUp);

  return function detach() {
    el.removeEventListener('touchstart', down);
    el.removeEventListener('touchmove', move);
    el.removeEventListener('touchend', up);
    el.removeEventListener('mousedown', down);
    window.removeEventListener('mousemove', move);
    window.removeEventListener('mouseup', up);
    window.removeEventListener('keydown', keyDown);
    window.removeEventListener('keyup', keyUp);
  };
}

/// Telegram's haptics, if we are inside the Mini App.
export function haptic(kind = 'light') {
  try { window.Telegram.WebApp.HapticFeedback.impactOccurred(kind); } catch (e) { /* not in Telegram */ }
}
