/* ================================================================
   Positron System Console — Main Application (Light Theme)
   ================================================================
   - Three.js 3D Solid Surface Terrain (60×16 grid) on white bg
   - Jet/Rainbow Color Mapping (MATLAB-style)
   - Black grid overlay (EdgesGeometry) + Dashed Bounding Box
   - Y-axis Lerp (α=0.1) for smooth height transitions
   - Canvas 2D charts: Boxplot, Density (KDE), Network Line
   - 16 Core Bars with Mini Sparklines
   - WebSocket Client + Mock Telemetry Fallback
   ================================================================ */

/* =================================================================
   1. CONSTANTS
   ================================================================= */
const ALPHA = 0.1;
const SEGMENTS_X = 60;
const SEGMENTS_Z = 16;
const MAX_HEIGHT = 4.5;
const Z_STEP = 1.0;
const WS_URL = 'ws://127.0.0.1:8080';
const CORE_COUNT = 16;
const SPARK_POINTS = 40;

/* Jet/Rainbow palette (MATLAB-style: blue → cyan → green → yellow → red) */
const JET_COLORS = [
  { pos: 0.00, color: new THREE.Color(0x00008F) },
  { pos: 0.25, color: new THREE.Color(0x00FFFF) },
  { pos: 0.50, color: new THREE.Color(0x00FF00) },
  { pos: 0.75, color: new THREE.Color(0xFFFF00) },
  { pos: 1.00, color: new THREE.Color(0xFF0000) },
];

/* =================================================================
   2. STATE
   ================================================================= */
let targetGrid  = Array.from({ length: SEGMENTS_Z + 1 },
                              () => Array(SEGMENTS_X + 1).fill(0));
let renderGrid  = Array.from({ length: SEGMENTS_Z + 1 },
                              () => Array(SEGMENTS_X + 1).fill(0));

let isConnected = false;
let latestMetrics = null;
let retryDelay = 2000;
let reconnectTimer = null;
let ws = null;
let clock = null;
let mockInterval = null;

/* History buffers for side charts */
let historyDensity = [];   // array of GPU load values
let historyNetwork = {     // for line chart
  time: [],
  sent: [],
  recv: []
};
let ramHistory = [];       // RAM percent history for RAM boxplot

/* Per-core history for sparklines */
const coreHistories = Array.from({ length: CORE_COUNT }, () => []);

/* =================================================================
   3. DOM REFERENCES
   ================================================================= */
const $ = (id) => document.getElementById(id);
const dom = {
  coreList:      $('core-list'),
  cpuAvg:        $('cpu-avg'),
  cpuStd:        $('cpu-std'),
  cpuVar:        $('cpu-var'),
  cpuBarFill:    $('cpu-bar-fill'),
  clock:         $('clock'),
  statusDot:     $('status-dot'),
  statusTxt:     $('status-text'),
  ramText:       $('ram-text'),
  ramBarFill:    $('ram-bar-fill'),
  gpu0Text:      $('gpu0-text'),
  gpu1Text:      $('gpu1-text'),
  container:     $('center-canvas'),
  boxplotCanvas: $('boxplot-canvas'),
  ramBoxCanvas:  $('ram-boxplot-canvas'),
  densityCanvas: $('density-canvas'),
  networkCanvas: $('network-canvas'),
};

/* Pre-create 16 core bars with spark canvases */
const coreBars = [];
for (let i = 0; i < CORE_COUNT; i++) {
  const label = `C${String(i).padStart(2, '0')}`;
  const el = document.createElement('div');
  el.className = 'core-bar';
  el.innerHTML = `
    <span class="core-label">${label}</span>
    <div class="core-track"><div class="core-fill" id="cf-${i}"></div></div>
    <span class="core-value" id="cv-${i}">--%</span>
    <canvas class="core-spark" id="sp-${i}" width="40" height="14"></canvas>
  `;
  dom.coreList.appendChild(el);
  coreBars.push({
    fill:  $(`cf-${i}`),
    val:   $(`cv-${i}`),
    spark: $(`sp-${i}`),
  });
}

/* =================================================================
   4. COLOR UTILITIES (Jet/Rainbow — MATLAB-style)
   ================================================================= */
const _tmpCol = new THREE.Color();

function getJetColor(t, out) {
  t = Math.max(0, Math.min(1, t));
  for (let i = 0; i < JET_COLORS.length - 1; i++) {
    const a = JET_COLORS[i];
    const b = JET_COLORS[i + 1];
    if (t >= a.pos && t <= b.pos) {
      const segT = (b.pos - a.pos) > 0
        ? (t - a.pos) / (b.pos - a.pos)
        : 0;
      out.lerpColors(a.color, b.color, segT);
      return;
    }
  }
  out.copy(JET_COLORS[JET_COLORS.length - 1].color);
}

/* =================================================================
   5. THREE.JS SETUP
   ================================================================= */

const scene = new THREE.Scene();
scene.background = new THREE.Color(0xffffff);

const camera = new THREE.PerspectiveCamera(
  45,
  dom.container.clientWidth / dom.container.clientHeight || 2,
  0.1, 200
);
camera.position.set(40, 24, 40);
camera.lookAt(0, 1.5, 0);

const renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
renderer.setSize(dom.container.clientWidth, dom.container.clientHeight);
renderer.setClearColor(0xffffff);
dom.container.appendChild(renderer.domElement);

/* Grid Geometry */
const geometry = new THREE.PlaneGeometry(
  SEGMENTS_X, SEGMENTS_Z * Z_STEP,
  SEGMENTS_X, SEGMENTS_Z
);
geometry.rotateX(-Math.PI / 2);

const posCount = geometry.attributes.position.count;
const colorArray = new Float32Array(posCount * 3);
geometry.setAttribute('color', new THREE.BufferAttribute(colorArray, 3));

const posAttr = geometry.attributes.position;
for (let iz = 0; iz <= SEGMENTS_Z; iz++) {
  for (let ix = 0; ix <= SEGMENTS_X; ix++) {
    const idx = iz * (SEGMENTS_X + 1) + ix;
    const x = ix - SEGMENTS_X / 2;
    const z = iz * Z_STEP - (SEGMENTS_Z * Z_STEP) / 2;
    posAttr.setXYZ(idx, x, 0, z);
  }
}
posAttr.needsUpdate = true;

const initJet = JET_COLORS[0].color;
for (let i = 0; i < posCount; i++) {
  colorArray[i * 3]     = initJet.r;
  colorArray[i * 3 + 1] = initJet.g;
  colorArray[i * 3 + 2] = initJet.b;
}
geometry.attributes.color.needsUpdate = true;

/* Surface Mesh — Solid Opaque (Jet-colored) */
const surfaceMat = new THREE.MeshBasicMaterial({
  vertexColors: true,
  side: THREE.DoubleSide,
});
const surfaceMesh = new THREE.Mesh(geometry, surfaceMat);

/* Black Grid Overlay via EdgesGeometry (MATLAB-style) */
const edgesGeo = new THREE.EdgesGeometry(geometry);
const edgesMat = new THREE.LineBasicMaterial({
  color: 0x000000,
  transparent: true,
  opacity: 0.25,
});
const wireframeOverlay = new THREE.LineSegments(edgesGeo, edgesMat);

/* Group */
const gridGroup = new THREE.Group();
gridGroup.add(surfaceMesh);
gridGroup.add(wireframeOverlay);
scene.add(gridGroup);

/* Dashed Bounding Box with Tick Marks (MATLAB-style) */
function createBoundingBox() {
  const bw = SEGMENTS_X;
  const bh = MAX_HEIGHT + 0.3;
  const bd = SEGMENTS_Z * Z_STEP;
  const box = new THREE.Group();

  /* Dashed edges */
  const boxEdgeGeo = new THREE.EdgesGeometry(new THREE.BoxGeometry(bw, bh, bd));
  const boxEdgeMat = new THREE.LineDashedMaterial({
    color: 0x6c757d,
    dashSize: 0.6,
    gapSize: 0.5,
    transparent: true,
    opacity: 0.45,
  });
  const boxEdges = new THREE.LineSegments(boxEdgeGeo, boxEdgeMat);
  boxEdges.computeLineDistances();
  box.add(boxEdges);

  /* Tick marks along axes */
  const tickLen = 0.35;
  const tickMat = new THREE.LineBasicMaterial({
    color: 0x6c757d, transparent: true, opacity: 0.4,
  });

  /* X-axis ticks (bottom front edge) */
  for (let x = -bw / 2; x <= bw / 2 + 0.01; x += 10) {
    const pts = [
      new THREE.Vector3(x, 0, -bd / 2),
      new THREE.Vector3(x, -tickLen, -bd / 2),
    ];
    box.add(new THREE.Line(new THREE.BufferGeometry().setFromPoints(pts), tickMat));
  }
  /* Y-axis ticks (left front edge) — evenly spaced, includes max */
  for (let y = 0; y <= bh + 0.01; y += bh / 4) {
    const pts = [
      new THREE.Vector3(-bw / 2, y, -bd / 2),
      new THREE.Vector3(-bw / 2 - tickLen, y, -bd / 2),
    ];
    box.add(new THREE.Line(new THREE.BufferGeometry().setFromPoints(pts), tickMat));
  }
  /* Z-axis ticks (bottom left edge) */
  for (let z = -bd / 2; z <= bd / 2 + 0.01; z += 4) {
    const pts = [
      new THREE.Vector3(-bw / 2, 0, z),
      new THREE.Vector3(-bw / 2, -tickLen, z),
    ];
    box.add(new THREE.Line(new THREE.BufferGeometry().setFromPoints(pts), tickMat));
  }

  box.position.set(0, bh / 2, 0);
  return box;
}
const boundingBox = createBoundingBox();
scene.add(boundingBox);

/* OrbitControls for Mouse Drag Interaction */
const controls = new THREE.OrbitControls(camera, renderer.domElement);
controls.target.set(0, 1.0, 0);
controls.enableDamping = true;
controls.dampingFactor = 0.08;
controls.rotateSpeed = 0.8;
controls.minDistance = 8;
controls.maxDistance = 80;
controls.maxPolarAngle = Math.PI / 2.1;
controls.update();

/* =================================================================
   6. VERTEX UPDATE (Lerp + Colors)
   ================================================================= */

function updateVertexBuffers(forRender) {
  if (forRender) {
    for (let iz = 0; iz <= SEGMENTS_Z; iz++) {
      for (let ix = 0; ix <= SEGMENTS_X; ix++) {
        const curr = renderGrid[iz][ix];
        const tgt  = targetGrid[iz][ix];
        renderGrid[iz][ix] = curr + ALPHA * (tgt - curr);
      }
    }
  }

  const pos  = geometry.attributes.position;
  const cols = geometry.attributes.color.array;

  for (let iz = 0; iz <= SEGMENTS_Z; iz++) {
    for (let ix = 0; ix <= SEGMENTS_X; ix++) {
      const idx = iz * (SEGMENTS_X + 1) + ix;
      const val = renderGrid[iz][ix];
      const h   = (val / 100) * MAX_HEIGHT;
      const t   = val / 100;

      pos.setY(idx, h);

      getJetColor(t, _tmpCol);
      cols[idx * 3]     = _tmpCol.r;
      cols[idx * 3 + 1] = _tmpCol.g;
      cols[idx * 3 + 2] = _tmpCol.b;
    }
  }

  pos.needsUpdate = true;
  geometry.attributes.color.needsUpdate = true;
}

/* =================================================================
   7. DATA PUSH (1Hz telemetry)
   ================================================================= */

function pushTelemetryRow(coreValues) {
  for (let iz = 0; iz < SEGMENTS_Z; iz++) {
    const val = (iz < coreValues.length) ? Math.max(0, coreValues[iz]) : 0;
    for (let ix = 0; ix <= SEGMENTS_X; ix++) {
      targetGrid[iz][ix] = val;
      /* renderGrid will smoothly converge via lerp in animate() */
    }
  }
}

/* =================================================================
   8. SPARKLINE DRAWING
   ================================================================= */

function drawSparkline(canvas, history) {
  if (!canvas || history.length < 2) return;
  const ctx = canvas.getContext('2d');
  const w = canvas.width, h = canvas.height;
  ctx.clearRect(0, 0, w, h);

  const max = Math.max(1, ...history);
  const len = history.length;
  ctx.strokeStyle = '#21918c';
  ctx.lineWidth = 1;
  ctx.beginPath();
  for (let i = 0; i < len; i++) {
    const x = (i / (len - 1)) * w;
    const y = h - (history[i] / max) * (h - 2) - 1;
    i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
  }
  ctx.stroke();
}

function drawAllSparklines() {
  for (let i = 0; i < CORE_COUNT; i++) {
    drawSparkline(coreBars[i].spark, coreHistories[i]);
  }
}

/* =================================================================
   9. BOXPLOT DRAWING
   ================================================================= */

function computeBoxplot(values) {
  if (values.length === 0) return { min: 0, q1: 0, med: 0, q3: 0, max: 0 };
  const s = values.slice().sort((a, b) => a - b);
  const n = s.length;
  const q1 = s[Math.round(n * 0.25)];
  const med = s[Math.round(n * 0.5)];
  const q3 = s[Math.round(n * 0.75)];
  return { min: s[0], q1, med, q3, max: s[n - 1] };
}

function drawBoxplot(canvas, bp, color) {
  if (!canvas) return;
  const ctx = canvas.getContext('2d');
  const w = canvas.width, h = canvas.height;
  ctx.clearRect(0, 0, w, h);

  const pad = 20;
  const drawW = w - pad * 2;
  const midY = h / 2;
  const range = Math.max(1, bp.max - bp.min);

  const mapX = (v) => pad + ((v - bp.min) / range) * drawW;

  /* Whisker line */
  ctx.strokeStyle = '#6c757d';
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(mapX(bp.min), midY);
  ctx.lineTo(mapX(bp.max), midY);
  ctx.stroke();

  /* Whisker caps */
  const capH = 6;
  ctx.beginPath();
  ctx.moveTo(mapX(bp.min), midY - capH);
  ctx.lineTo(mapX(bp.min), midY + capH);
  ctx.moveTo(mapX(bp.max), midY - capH);
  ctx.lineTo(mapX(bp.max), midY + capH);
  ctx.stroke();

  /* IQR Box */
  const boxLeft = mapX(bp.q1);
  const boxRight = mapX(bp.q3);
  const boxH = 14;
  ctx.fillStyle = color || '#21918c';
  ctx.globalAlpha = 0.25;
  ctx.fillRect(boxLeft, midY - boxH / 2, boxRight - boxLeft, boxH);
  ctx.globalAlpha = 1;
  ctx.strokeStyle = color || '#21918c';
  ctx.lineWidth = 1.5;
  ctx.strokeRect(boxLeft, midY - boxH / 2, boxRight - boxLeft, boxH);

  /* Median line */
  ctx.strokeStyle = color || '#21918c';
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(mapX(bp.med), midY - boxH / 2);
  ctx.lineTo(mapX(bp.med), midY + boxH / 2);
  ctx.stroke();
}

/* =================================================================
   10. DENSITY PLOT (KDE) DRAWING
   ================================================================= */

function drawDensityPlot(canvas, data, color) {
  if (!canvas || data.length < 3) return;
  const ctx = canvas.getContext('2d');
  const w = canvas.width, h = canvas.height;
  ctx.clearRect(0, 0, w, h);

  const pad = 4;
  const drawW = w - pad * 2;
  const drawH = h - pad * 2;

  /* Compute KDE */
  const n = data.length;
  const minVal = Math.min(...data);
  const maxVal = Math.max(...data);
  const range = Math.max(1, maxVal - minVal);
  const steps = 60;
  const bandwidth = range / 12;

  const density = [];
  for (let i = 0; i < steps; i++) {
    const x = minVal + (i / (steps - 1)) * range;
    let sum = 0;
    for (let j = 0; j < n; j++) {
      const d = (x - data[j]) / bandwidth;
      sum += Math.exp(-0.5 * d * d) / (bandwidth * Math.sqrt(2 * Math.PI));
    }
    density.push(sum / n);
  }
  const maxD = Math.max(...density, 0.001);

  /* Fill area */
  const c = color || '#21918c';
  ctx.fillStyle = c;
  ctx.globalAlpha = 0.2;
  ctx.beginPath();
  ctx.moveTo(pad, h - pad);
  for (let i = 0; i < steps; i++) {
    const x = pad + (i / (steps - 1)) * drawW;
    const y = (h - pad) - (density[i] / maxD) * drawH;
    i === 0 ? ctx.lineTo(x, y) : ctx.lineTo(x, y);
  }
  ctx.lineTo(pad + drawW, h - pad);
  ctx.closePath();
  ctx.fill();
  ctx.globalAlpha = 1;

  /* Stroke line */
  ctx.strokeStyle = c;
  ctx.lineWidth = 1.5;
  ctx.beginPath();
  for (let i = 0; i < steps; i++) {
    const x = pad + (i / (steps - 1)) * drawW;
    const y = (h - pad) - (density[i] / maxD) * drawH;
    i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
  }
  ctx.stroke();
}

/* =================================================================
   11. NETWORK LINE CHART DRAWING
   ================================================================= */

function drawNetworkChart(canvas, history) {
  if (!canvas) return;
  const ctx = canvas.getContext('2d');
  const w = canvas.width, h = canvas.height;
  ctx.clearRect(0, 0, w, h);

  const pad = { top: 4, right: 4, bottom: 14, left: 30 };
  const drawW = w - pad.left - pad.right;
  const drawH = h - pad.top - pad.bottom;

  const n = history.time.length;
  if (n < 2) return;

  const allVals = [...history.sent, ...history.recv, 1];
  const maxVal = Math.max(...allVals);
  const yMax = Math.ceil(maxVal * 1.2) || 10;

  /* Y axis labels */
  ctx.fillStyle = '#adb5bd';
  ctx.font = '8px monospace';
  ctx.textAlign = 'right';
  ctx.fillText(yMax + 'K', pad.left - 3, pad.top + 8);
  ctx.fillText('0', pad.left - 3, h - pad.bottom - 2);

  /* Grid lines */
  ctx.strokeStyle = '#f0f0f0';
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(pad.left, pad.top + drawH / 2);
  ctx.lineTo(w - pad.right, pad.top + drawH / 2);
  ctx.stroke();

  const mapX = (i) => pad.left + (i / (n - 1)) * drawW;
  const mapY = (v) => (h - pad.bottom) - (v / yMax) * drawH;

  /* Sent trace */
  ctx.strokeStyle = '#3b528b';
  ctx.lineWidth = 1.2;
  ctx.beginPath();
  for (let i = 0; i < n; i++) {
    const x = mapX(i), y = mapY(history.sent[i]);
    i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
  }
  ctx.stroke();

  /* Receive trace */
  ctx.strokeStyle = '#21918c';
  ctx.lineWidth = 1.2;
  ctx.beginPath();
  for (let i = 0; i < n; i++) {
    const x = mapX(i), y = mapY(history.recv[i]);
    i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
  }
  ctx.stroke();

  /* Labels */
  ctx.fillStyle = '#3b528b';
  ctx.font = '7px monospace';
  ctx.textAlign = 'left';
  ctx.fillText('▲SEND', pad.left + 2, pad.top + 8);

  ctx.fillStyle = '#21918c';
  ctx.fillText('▼RECV', pad.left + 40, pad.top + 8);

  /* X axis label */
  ctx.fillStyle = '#adb5bd';
  ctx.font = '7px monospace';
  ctx.textAlign = 'center';
  ctx.fillText('time (s) →', w / 2, h - 1);
}

/* =================================================================
   12. HUD UPDATES
   ================================================================= */

function updateCoreBars(cores) {
  if (!cores || cores.length < CORE_COUNT) return;

  let sum = 0, sumSq = 0;
  for (let i = 0; i < CORE_COUNT; i++) {
    const v = Math.max(0, Math.min(100, cores[i]));
    sum += v;
    sumSq += v * v;

    coreBars[i].fill.style.width = v + '%';
    coreBars[i].val.textContent = Math.round(v) + '%';

    /* Update spark history */
    coreHistories[i].push(v);
    if (coreHistories[i].length > SPARK_POINTS) coreHistories[i].shift();
  }

  const avg = sum / CORE_COUNT;
  const variance = (sumSq / CORE_COUNT) - (avg * avg);
  const stdDev = Math.sqrt(Math.max(0, variance));

  dom.cpuAvg.textContent = avg.toFixed(1) + '%';
  dom.cpuStd.textContent = stdDev.toFixed(2);
  dom.cpuVar.textContent = variance.toFixed(2);
  dom.cpuBarFill.style.width = Math.min(avg, 100) + '%';

  /* Boxplot from current cores */
  const bp = computeBoxplot(cores);
  drawBoxplot(dom.boxplotCanvas, bp, '#21918c');

  /* Sparklines */
  drawAllSparklines();
}

function updateHUD(metrics) {
  if (!metrics) return;

  const mem = metrics.memory_detail;
  if (mem) {
    const used  = mem.used_gb  || 0;
    const total = mem.total_gb || 31.8;
    const pct   = mem.percent  || 0;
    dom.ramText.textContent = `${used.toFixed(1)} / ${total.toFixed(1)} GB (${pct.toFixed(0)}%)`;
    dom.ramBarFill.style.width = Math.min(pct, 100) + '%';
  }

  const gpuDetail = metrics.gpu_detail;
  if (gpuDetail && gpuDetail.length > 0) {
    /* GPU 0 */
    const g0 = gpuDetail[0];
    dom.gpu0Text.textContent =
      `GPU 0 (${g0.name || 'Quadro T1000'}) — ${g0.temperature_c != null ? g0.temperature_c : '--'}°C — Load: ${g0.load_percent != null ? g0.load_percent.toFixed(0) : '--'}%`;

    /* GPU 1 if exists */
    if (gpuDetail.length > 1) {
      const g1 = gpuDetail[1];
      dom.gpu1Text.textContent =
        `GPU 1 (${g1.name || 'Intel UHD'}) — ${g1.temperature_c != null ? g1.temperature_c : '--'}°C — Load: ${g1.load_percent != null ? g1.load_percent.toFixed(0) : '--'}%`;
    }

    /* Feed density history */
    const load = g0.load_percent || 0;
    historyDensity.push(load);
    if (historyDensity.length > 120) historyDensity.shift();
  }

  const net = metrics.network_speed;
  if (net) {
    const sent = net.sent_kbps || 0;
    const recv = net.received_kbps || 0;
    historyNetwork.time.push(historyNetwork.time.length);
    historyNetwork.sent.push(sent);
    historyNetwork.recv.push(recv);
    const maxPts = 60;
    if (historyNetwork.time.length > maxPts) {
      historyNetwork.time.shift();
      historyNetwork.sent.shift();
      historyNetwork.recv.shift();
      /* Re-index */
      historyNetwork.time = historyNetwork.time.map((_, i) => i);
    }
  }
}

/* =================================================================
   13. CHART REDRAW LOOP (called each frame for smooth updates)
   ================================================================= */

/* Throttle counters for side chart redraws (~10Hz) */
let sideChartTick = 0;

function redrawSideCharts() {
  /* Throttle to every 6th frame (~10fps on 60Hz display) */
  sideChartTick = (sideChartTick + 1) % 6;
  if (sideChartTick !== 0) return;

  /* Density plot */
  if (historyDensity.length > 2) {
    drawDensityPlot(dom.densityCanvas, historyDensity, '#21918c');
  }

  /* RAM boxplot */
  if (latestMetrics && latestMetrics.memory_detail) {
    const pct = latestMetrics.memory_detail.percent || 0;
    ramHistory.push(pct);
    if (ramHistory.length > 60) ramHistory.shift();
    if (ramHistory.length > 2) {
      const bp = computeBoxplot(ramHistory);
      drawBoxplot(dom.ramBoxCanvas, bp, '#3b528b');
    }
  }

  /* Network chart */
  if (historyNetwork.time.length > 1) {
    drawNetworkChart(dom.networkCanvas, historyNetwork);
  }
}

/* =================================================================
   14. ANIMATION LOOP (60 FPS)
   ================================================================= */

function animate() {
  requestAnimationFrame(animate);

  const delta = Math.min(clock.getDelta(), 0.05);

  /* Y-axis Lerp */
  updateVertexBuffers(true);

  /* OrbitControls update (supports mouse drag) */
  controls.update();

  renderer.render(scene, camera);

  /* Side chart redraws (throttled to ~10Hz to save CPU) */
  redrawSideCharts();
}

/* =================================================================
   15. WEBSOCKET CLIENT
   ================================================================= */

function handleDisconnection() {
  if (isConnected) {
    isConnected = false;
    setStatus('offline', 'RECONNECTING...');
    startMockTelemetry();
  }

  if (reconnectTimer) {
    clearTimeout(reconnectTimer);
    reconnectTimer = null;
  }

  retryDelay = Math.min(retryDelay * 1.5, 30000);
  console.log(`[WS] Reconnecting in ${retryDelay / 1000}s...`);
  reconnectTimer = setTimeout(initWebSocket, retryDelay);
}

function initWebSocket() {
  if (ws) {
    try { ws.close(); } catch (_) {}
    ws = null;
  }

  try {
    ws = new WebSocket(WS_URL);
  } catch (err) {
    console.warn('[WS] Constructor failed:', err);
    handleDisconnection();
    return;
  }

  ws.onopen = () => {
    console.log('[WS] Connected to backend at', WS_URL);
    isConnected = true;
    retryDelay = 2000;
    setStatus('online', '● ACTIVE // R-CONNECTED');
    stopMockTelemetry();
  };

  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      latestMetrics = data;

      if (data.cpu_detail && data.cpu_detail.cores) {
        pushTelemetryRow(data.cpu_detail.cores);
      }
      updateCoreBars(data.cpu_detail ? data.cpu_detail.cores : null);
      updateHUD(data);
    } catch (err) {
      console.warn('[WS] Parse error:', err);
    }
  };

  ws.onclose = (event) => {
    console.log('[WS] Connection closed (code:', event.code, ')');
    handleDisconnection();
  };

  ws.onerror = (err) => {
    console.warn('[WS] Error:', err);
  };
}

/* =================================================================
   16. MOCK TELEMETRY
   ================================================================= */

function generateMockTelemetry() {
  if (isConnected) return;

  const cores = [];
  const t = Date.now() * 0.001;

  for (let i = 0; i < CORE_COUNT; i++) {
    const wave   = Math.sin(i * 0.6 + t * 0.8) * 12;
    const wave2  = Math.cos(i * 0.3 + t * 0.4) * 6;
    const noise  = (Math.random() - 0.5) * 12;
    const base   = 12 + (i / CORE_COUNT) * 10;
    cores.push(Math.max(2, Math.min(98, base + wave + wave2 + noise)));
  }

  const avg = cores.reduce((a, b) => a + b, 0) / CORE_COUNT;

  pushTelemetryRow(cores);
  updateCoreBars(cores);

  const mockMetrics = {
    metric_id: Math.floor(Date.now() / 1000),
    cpu_usage: avg,
    ram_usage: 40 + Math.sin(t * 0.3) * 10 + (Math.random() - 0.5) * 4,
    gpu_usage: 5 + Math.sin(t * 0.2) * 8 + Math.random() * 10,
    vram_usage: 4 + Math.random() * 6,
    gpu_temperature: 42 + Math.sin(t * 0.15) * 5 + Math.random() * 4,
    network_speed: {
      sent_kbps: 3 + Math.sin(t * 0.5) * 2 + Math.random() * 5,
      received_kbps: 8 + Math.sin(t * 0.4) * 4 + Math.random() * 10,
    },
    cpu_detail: {
      total: avg,
      cores: cores,
      avg_utilization: avg,
      std_dev: 5 + Math.random() * 8,
      variance: 25 + Math.random() * 80,
    },
    memory_detail: {
      total_gb: 31.8,
      used_gb: Math.max(4, Math.min(28, 14 + Math.sin(t * 0.3) * 3 + (Math.random() - 0.5) * 1)),
      percent: Math.max(10, Math.min(90, 44 + Math.sin(t * 0.3) * 8 + (Math.random() - 0.5) * 3)),
    },
    gpu_detail: [
      {
        name: 'Quadro T1000',
        load_percent: 5 + Math.sin(t * 0.2) * 8 + Math.random() * 10,
        temperature_c: 42 + Math.sin(t * 0.15) * 5 + Math.random() * 4,
      },
      {
        name: 'Intel UHD',
        load_percent: 2 + Math.sin(t * 0.35) * 3 + Math.random() * 5,
        temperature_c: 38 + Math.sin(t * 0.1) * 3 + Math.random() * 2,
      },
    ],
  };

  latestMetrics = mockMetrics;
  updateHUD(mockMetrics);
}

function startMockTelemetry() {
  if (mockInterval) return;
  setStatus('mock', '● OFFLINE // RUNNING MOCK');
  generateMockTelemetry();
  mockInterval = setInterval(generateMockTelemetry, 1000);
}

function stopMockTelemetry() {
  if (mockInterval) {
    clearInterval(mockInterval);
    mockInterval = null;
  }
}

/* =================================================================
   17. CLOCK & STATUS
   ================================================================= */

function updateClock() {
  const now = new Date();
  dom.clock.textContent =
    String(now.getHours()).padStart(2, '0') + ':' +
    String(now.getMinutes()).padStart(2, '0') + ':' +
    String(now.getSeconds()).padStart(2, '0');
}

function setStatus(state, text) {
  dom.statusDot.className = 'status-dot';
  if (state === 'online') {
    dom.statusTxt.textContent = text || '● ACTIVE // R-CONNECTED';
  } else if (state === 'mock') {
    dom.statusDot.classList.add('mock');
    dom.statusTxt.textContent = text || '● OFFLINE // RUNNING MOCK';
  } else {
    dom.statusDot.classList.add('offline');
    dom.statusTxt.textContent = text || 'DISCONNECTED';
  }
}

/* =================================================================
   18. WINDOW RESIZE
   ================================================================= */

function onResize() {
  const w = dom.container.clientWidth;
  const h = dom.container.clientHeight;
  if (w > 0 && h > 0) {
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    renderer.setSize(w, h);
  }
}
window.addEventListener('resize', onResize);

/* =================================================================
   19. INITIALIZATION
   ================================================================= */

function init() {
  updateClock();
  setInterval(updateClock, 1000);

  clock = new THREE.Clock();
  updateVertexBuffers(false);

  /* Initial boxplot draw */
  const initCores = Array(CORE_COUNT).fill(5);
  const bp = computeBoxplot(initCores);
  drawBoxplot(dom.boxplotCanvas, bp, '#21918c');

  animate();

  initWebSocket();

  setTimeout(() => {
    if (!isConnected) {
      startMockTelemetry();
    }
  }, 3000);

  console.log('[Positron Light Console] Initialized.');
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
