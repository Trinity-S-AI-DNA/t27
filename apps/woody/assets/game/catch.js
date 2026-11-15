// Woody Catch — the four-lane catcher, rebuilt on the shared 3D engine.
//
// This replaces a DOM implementation that moved emoji in absolutely-positioned
// divs. The rules are carried over unchanged, deliberately: four lanes, four
// drop types worth 10/20/30/50, a combo multiplier of 1 + floor(combo/5)/2
// capped at 3, three lives, and a spawn interval of max(900 - level*50, 280)ms.
// Anyone who had a high score before should recognise the game they set it in.
//
// The one conversion that matters: the old loop moved drops in percent of
// screen height per 16ms tick. Here they move in world units per second, so
// every rate is multiplied by 10 — `0.8 %/tick` is 50 %/s, and the fall is
// 16.6 units, which is 8.1 units/s.

import {
  THREE, createStage, addLights, skyTexture, flat, mesh, buildWoody,
  contactShadow, Dust, horizontalInput, haptic, rand, NEON,
} from '/assets/game/engine.js';

// ── Look ────────────────────────────────────────────────────────────────────
// Daylight, matching the skate game: two games on the same stack should not
// look like they come from different shops.
const SKY_TOP = '#2f8fd8';
const SKY_MID = '#8ec9ea';
const SKY_LOW = '#cfeef7';
const GROUND = 0x86c95f;
const LANE_X = [-3, -1, 1, 3];
const LANE_W = 2;

// ── Rules, carried over from the DOM version ────────────────────────────────
const TYPES = [
  { pts: 10, color: 0x22c55e, build: () => new THREE.ConeGeometry(0.34, 0.7, 5) },        // leaf
  { pts: 20, color: 0xa855f7, build: () => new THREE.OctahedronGeometry(0.4, 0) },        // bud
  { pts: 30, color: 0xfbbf24, build: () => new THREE.OctahedronGeometry(0.46, 0) },       // star
  { pts: 50, color: 0x06b6d4, build: () => new THREE.IcosahedronGeometry(0.44, 0) },      // gem
];
const START_LIVES = 3;
const SPAWN_Y = 16;
const CATCH_LOW = 1.0;
const CATCH_HIGH = 2.6;
const MISS_Y = -0.6;
const FALL_SCALE = 10;   // %/tick → units/s

function randType(level) {
  const r = Math.random();
  // Higher levels tilt slightly toward the rare drops.
  const t = level > 5 ? [0.94, 0.84, 0.68] : [0.96, 0.88, 0.72];
  if (r > t[0]) return 3;
  if (r > t[1]) return 2;
  if (r > t[2]) return 1;
  return 0;
}

export function start(container, opts = {}) {
  const onTick = opts.onTick || function () { };
  const onEnd = opts.onEnd || function () { };
  const onCatch = opts.onCatch || function () { };

  const stage = createStage(container, { fov: 58, far: 200 });
  const { renderer, scene, camera } = stage;

  scene.background = skyTexture(SKY_TOP, SKY_MID, SKY_LOW, 0.46);
  scene.fog = new THREE.Fog(0xcfeef7, 34, 96);
  addLights(scene, { sky: 0xd8f0ff, ground: 0x6f8f5a });

  // ── Ground and lanes ──────────────────────────────────────────────────────
  const ground = mesh(new THREE.PlaneGeometry(60, 60), GROUND);
  ground.rotation.x = -Math.PI / 2;
  ground.position.z = -8;
  scene.add(ground);

  // A lit strip under each lane: without them the drop's lane is ambiguous
  // until the last moment, which reads as unfair rather than as difficult.
  const laneMat = new THREE.MeshBasicMaterial({
    color: 0xffffff, transparent: true, opacity: 0.18, depthWrite: false,
  });
  const laneMarks = [];
  for (const x of LANE_X) {
    const strip = new THREE.Mesh(new THREE.PlaneGeometry(LANE_W - 0.2, 34), laneMat.clone());
    strip.rotation.x = -Math.PI / 2;
    strip.position.set(x, 0.02, -13);
    scene.add(strip);
    laneMarks.push(strip);
  }

  // A back wall of low-poly trees, so the lanes have something behind them.
  {
    const trunkGeo = new THREE.CylinderGeometry(0.16, 0.28, 1, 5);
    const crownGeo = new THREE.IcosahedronGeometry(1, 0);
    const trunks = new THREE.InstancedMesh(trunkGeo, flat(0x8a5a3b), 26);
    const crowns = new THREE.InstancedMesh(crownGeo, new THREE.MeshLambertMaterial({ flatShading: true }), 26);
    trunks.frustumCulled = crowns.frustumCulled = false;
    const dummy = new THREE.Object3D();
    const tint = new THREE.Color();
    for (let i = 0; i < 26; i++) {
      const h = rand(3, 7);
      const x = rand(-26, 26);
      const z = rand(-34, -20);
      dummy.position.set(x, h * 0.35, z);
      dummy.rotation.set(0, rand(0, Math.PI), 0);
      dummy.scale.set(1, h * 0.7, 1);
      dummy.updateMatrix();
      trunks.setMatrixAt(i, dummy.matrix);
      const r = rand(1.3, 2.4);
      dummy.position.set(x, h * 0.7 + r * 0.5, z);
      dummy.rotation.set(rand(0, 1), rand(0, Math.PI), rand(0, 1));
      dummy.scale.set(r, r * rand(0.7, 1.1), r);
      dummy.updateMatrix();
      crowns.setMatrixAt(i, dummy.matrix);
      crowns.setColorAt(i, tint.setHex([0x3f9142, 0x4faa4b, 0x2f7d3a][i % 3]));
    }
    trunks.instanceMatrix.needsUpdate = true;
    crowns.instanceMatrix.needsUpdate = true;
    if (crowns.instanceColor) crowns.instanceColor.needsUpdate = true;
    scene.add(trunks, crowns);
  }

  // ── Woody with a basket ───────────────────────────────────────────────────
  const { bird, headGroup, tail, armL, armR } = buildWoody({ headTurn: 0 });
  const player = new THREE.Group();
  // Facing the camera, standing just behind the plane the drops fall in. Held
  // in front of a back-turned bird the basket is invisible — the drops would
  // vanish into his shoulders.
  bird.rotation.y = Math.PI;
  bird.position.z = -0.8;
  player.add(bird);

  const basket = new THREE.Group();
  {
    const rim = mesh(new THREE.TorusGeometry(0.52, 0.09, 4, 12), 0xb5793f);
    rim.rotation.x = Math.PI / 2;
    rim.position.y = 0.44;
    const bowl = mesh(new THREE.CylinderGeometry(0.5, 0.34, 0.5, 10, 1, true), 0xcf9455);
    bowl.material.side = THREE.DoubleSide;
    bowl.position.y = 0.2;
    basket.add(rim, bowl);
    basket.position.set(0, 0.82, 0.34);
    basket.scale.setScalar(0.82);
    player.add(basket);
  }
  // Both arms forward, holding it.
  // Reaching toward the viewer, around the basket.
  armL.rotation.z = -0.55; armL.rotation.x = 1.1;
  armR.rotation.z = 0.55; armR.rotation.x = 1.1;

  const shadow = contactShadow(0.7, 0.7);
  scene.add(shadow);
  scene.add(player);

  const dust = new Dust(scene, 24);

  // ── Drops ─────────────────────────────────────────────────────────────────
  const dropGeo = TYPES.map((t) => t.build());
  const dropMat = TYPES.map((t) => new THREE.MeshLambertMaterial({
    color: t.color, emissive: t.color, emissiveIntensity: 0.35, flatShading: true,
  }));
  const haloGeo = new THREE.RingGeometry(0.6, 0.85, 12);
  const drops = [];

  function spawnDrop(level, forceType) {
    const typeIdx = forceType === undefined ? randType(level) : forceType;
    const laneIdx = (Math.random() * LANE_X.length) | 0;
    const m = new THREE.Mesh(dropGeo[typeIdx], dropMat[typeIdx]);
    m.position.set(LANE_X[laneIdx], SPAWN_Y, 0);
    m.rotation.set(rand(0, 3), rand(0, 3), rand(0, 3));
    // The two rare drops get a halo, so they are worth reacting to from the
    // top of the screen rather than recognised as they land.
    if (typeIdx >= 2) {
      const halo = new THREE.Mesh(haloGeo, new THREE.MeshBasicMaterial({
        color: TYPES[typeIdx].color, transparent: true, opacity: 0.4,
        side: THREE.DoubleSide, depthWrite: false,
      }));
      m.add(halo);
      m.userData.halo = halo;
    }
    m.userData.type = typeIdx;
    m.userData.lane = laneIdx;
    m.userData.speed = (0.8 + level * 0.12 + Math.random() * 0.2) * FALL_SCALE;
    scene.add(m);
    drops.push(m);
  }

  function removeDrop(i) {
    const m = drops[i];
    scene.remove(m);
    drops.splice(i, 1);
  }

  // ── Input ─────────────────────────────────────────────────────────────────
  let laneIdx = 1;
  let steer = 0;
  const detachInput = horizontalInput(renderer.domElement, (v) => {
    // The old game moved a whole lane per key press; a drag now picks the lane
    // under the finger, which is what a four-lane board on a phone wants.
    steer = v;
    const wanted = Math.round((v + 1) / 2 * (LANE_X.length - 1));
    if (Math.abs(v) > 0.08) laneIdx = Math.max(0, Math.min(LANE_X.length - 1, wanted));
  }, { gain: 2.0 });

  const onKeyStep = (e) => {
    if (e.key === 'ArrowLeft' || e.key === 'a') laneIdx = Math.max(0, laneIdx - 1);
    if (e.key === 'ArrowRight' || e.key === 'd') laneIdx = Math.min(LANE_X.length - 1, laneIdx + 1);
  };
  window.addEventListener('keydown', onKeyStep);

  // ── Loop ──────────────────────────────────────────────────────────────────
  let score = 0, lives = START_LIVES, level = 1, combo = 0, best = 0;
  let alive = true, raf = 0, clock = 0, spawnAcc = 0, shake = 0;
  let playerX = LANE_X[laneIdx];
  let last = performance.now();

  function finish() {
    if (!alive) return;
    alive = false;
    onEnd({ score, level, combo: best });
  }

  function frame(now) {
    raf = requestAnimationFrame(frame);
    // Clamped so a backgrounded tab does not resume with one enormous step
    // that drops every falling item straight past the basket.
    const dt = Math.min((now - last) / 1000, 0.05);
    last = now;
    clock += dt;

    if (alive) {
      const spawnEvery = Math.max(280, 900 - level * 50) / 1000;
      spawnAcc += dt;
      if (spawnAcc >= spawnEvery) {
        spawnAcc = 0;
        spawnDrop(level);
        // Double-spawn past level 3, the cheap one, as before.
        if (level > 3 && Math.random() > 0.6) spawnDrop(level, 0);
      }
    }

    // ── Player ──
    const targetX = LANE_X[laneIdx];
    playerX += (targetX - playerX) * Math.min(1, dt * 14);
    player.position.x = playerX;
    player.rotation.z = (targetX - playerX) * 0.18;
    player.position.y = Math.sin(clock * 6) * 0.03;
    headGroup.rotation.x = Math.sin(clock * 9) * 0.06;
    tail.rotation.y = Math.sin(clock * 4) * 0.12;
    shadow.position.set(playerX, 0.03, 0);

    for (const strip of laneMarks) strip.material.opacity = 0.14;
    laneMarks[laneIdx].material.opacity = 0.42;

    // ── Drops ──
    for (let i = drops.length - 1; i >= 0; i--) {
      const m = drops[i];
      m.position.y -= m.userData.speed * dt;
      m.rotation.y += dt * 2.5;
      if (m.userData.halo) m.userData.halo.rotation.y = -m.rotation.y;

      if (!alive) continue;

      if (m.position.y <= CATCH_HIGH && m.position.y >= CATCH_LOW && m.userData.lane === laneIdx) {
        const t = TYPES[m.userData.type];
        const mult = Math.min(3, 1 + Math.floor(combo / 5) * 0.5);
        const earned = Math.round(t.pts * mult);
        score += earned;
        combo += 1;
        best = Math.max(best, combo);
        level = 1 + Math.floor(score / 300);
        dust.spawn(m.position.x, 1.2, 0);
        haptic('light');
        onCatch({ points: earned, combo, type: m.userData.type });
        removeDrop(i);
        continue;
      }

      if (m.position.y < MISS_Y) {
        removeDrop(i);
        combo = 0;
        lives -= 1;
        shake = 1;
        haptic('medium');
        if (lives <= 0) finish();
      }
    }

    dust.update(dt);

    // ── Camera ──
    const jitter = shake * 0.25;
    camera.position.set(
      playerX * 0.35 + rand(-jitter, jitter),
      4.4 + rand(-jitter, jitter),
      7.4);
    camera.lookAt(playerX * 0.2, 2.1, -2);
    shake = Math.max(0, shake - dt * 3);

    if (alive) onTick({ score, lives, level, combo });
    renderer.render(scene, camera);
  }
  raf = requestAnimationFrame(frame);

  function stop() {
    alive = false;
    cancelAnimationFrame(raf);
    detachInput();
    window.removeEventListener('keydown', onKeyStep);
    stage.dispose();
  }

  // A test seam. The catch window is 1.6 units tall and a drop crosses it in
  // about a tenth of a second, so a blind harness scores zero whether or not
  // the collision works — it cannot tell a bug from bad luck. This lets a test
  // aim at the lowest drop and prove the path.
  stop.debug = {
    lowestDropLane: () => {
      let lowest = null;
      for (const d of drops) if (!lowest || d.position.y < lowest.position.y) lowest = d;
      return lowest ? lowest.userData.lane : null;
    },
    setLane: (i) => { laneIdx = Math.max(0, Math.min(LANE_X.length - 1, i)); },
    dropCount: () => drops.length,
  };

  return stop;
}
