import { CORE_COUNT, SPARK_POINTS } from './constants.js';
import { connectionState, coreHistories, historyNetwork, targetGrid, historyDensity, gpuHistory0, gpuHistory1 } from './state.js';
import { tsBuffer } from './timeSeries.js';

const $ = id => document.getElementById(id);

export const dom = {
  coreList: $('core-list'),
  cpuAvg: $('cpu-avg'),
  cpuStd: $('cpu-std'),
  cpuVar: $('cpu-var'),
  cpuBarFill: $('cpu-bar-fill'),
  clock: $('clock'),
  statusDot: $('status-dot'),
  statusTxt: $('status-text'),
  ramText: $('ram-text'),
  ramBarFill: $('ram-bar-fill'),
  gpu0Text: $('gpu0-text'),
  gpu1Text: $('gpu1-text'),
  container: $('center-canvas'),
  boxplotCanvas: $('boxplot-canvas'),
  densityCanvas: $('density-canvas'),
  gpuTimeseriesCanvas: $('gpu-timeseries-canvas'),
  networkCanvas: $('network-canvas'),
  allocationBlocks: $('allocation-blocks'),
  processDots: $('process-dots'),
};

export const coreBars = [];
for (let i = 0; i < CORE_COUNT; i++) {
  const el = document.createElement('div');
  el.className = 'core-bar';
  el.innerHTML = `
    <span class="core-label">C${String(i).padStart(2, '0')}</span>
    <span class="core-track"><span class="core-fill"></span></span>
    <span class="core-value">--%</span>
    <canvas class="core-spark" width="40" height="14"></canvas>
  `;
  el.querySelector('.core-spark').getContext('2d');
  dom.coreList.appendChild(el);
  coreBars.push(el);
}

const hudOverlay = document.createElement('div');
hudOverlay.className = 'hud-overlay';
hudOverlay.innerHTML = '<div class="hud-title">ORBIT SYSTEM MANAGER // 3D TOPOLOGY</div>';
dom.container.appendChild(hudOverlay);

for (let i = 0; i < 24; i++) {
  const block = document.createElement('div');
  block.className = 'block';
  dom.allocationBlocks.appendChild(block);
}

updateClock();
setInterval(updateClock, 1000);

export function drawSparkline(canvas, history) {
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

export function drawAllSparklines() {
  for (let i = 0; i < CORE_COUNT; i++) {
    const sparkCanvas = coreBars[i].querySelector('.core-spark');
    if (sparkCanvas) drawSparkline(sparkCanvas, coreHistories[i]);
  }
}

// ---- Boxplot helpers ----

function computeBoxplot(values) {
  if (values.length === 0) return { min: 0, q1: 0, med: 0, q3: 0, max: 0 };
  const s = values.slice().sort((a, b) => a - b);
  const n = s.length;
  return {
    min: s[0],
    q1:  s[Math.round(n * 0.25)],
    med: s[Math.round(n * 0.5)],
    q3:  s[Math.round(n * 0.75)],
    max: s[n - 1],
  };
}

function drawVerticalBoxplot(ctx, cx, topY, bottomY, boxWidth, bp, color) {
  const mapY = (v) => bottomY - (Math.max(0, Math.min(100, v)) / 100) * (bottomY - topY);

  const yMin = mapY(bp.min);
  const yMax = mapY(bp.max);
  const yQ1  = mapY(bp.q1);
  const yQ3  = mapY(bp.q3);
  const yMed = mapY(bp.med);
  const halfW = boxWidth / 2;

  ctx.strokeStyle = color || '#21918c';
  ctx.lineWidth = 1;

  /* Whisker line (min to max) */
  ctx.beginPath();
  ctx.moveTo(cx, yMin);
  ctx.lineTo(cx, yMax);
  ctx.stroke();

  /* Caps at min and max */
  const capW = 4;
  ctx.beginPath();
  ctx.moveTo(cx - capW, yMin);
  ctx.lineTo(cx + capW, yMin);
  ctx.moveTo(cx - capW, yMax);
  ctx.lineTo(cx + capW, yMax);
  ctx.stroke();

  /* IQR Box */
  const boxH = Math.max(2, Math.abs(yQ3 - yQ1));
  const boxTop = Math.min(yQ1, yQ3);
  ctx.fillStyle = color || '#21918c';
  ctx.globalAlpha = 0.25;
  ctx.fillRect(cx - halfW, boxTop, halfW * 2, boxH);
  ctx.globalAlpha = 1;
  ctx.strokeStyle = color || '#21918c';
  ctx.lineWidth = 1.2;
  ctx.strokeRect(cx - halfW, boxTop, halfW * 2, boxH);

  /* Median line */
  ctx.strokeStyle = color || '#21918c';
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(cx - halfW, yMed);
  ctx.lineTo(cx + halfW, yMed);
  ctx.stroke();
}

function drawStripplot(ctx, cx, topY, bottomY, boxWidth, rawValues, color) {
  if (!rawValues || rawValues.length === 0) return;

  const halfW = boxWidth * 0.35;
  const r = 2.5;

  for (let i = 0; i < rawValues.length; i++) {
    const v = Math.max(0, Math.min(100, rawValues[i]));
    const y = bottomY - (v / 100) * (bottomY - topY);

    const jitter = ((i / Math.max(1, rawValues.length - 1)) - 0.5) * halfW * 1.6;
    const x = cx + jitter;

    ctx.shadowColor = color || '#21918c';
    ctx.shadowBlur = 6;
    ctx.fillStyle = color || '#21918c';
    ctx.globalAlpha = 0.85;
    ctx.beginPath();
    ctx.arc(x, y, r, 0, Math.PI * 2);
    ctx.fill();

    ctx.shadowBlur = 0;
    ctx.fillStyle = '#ffffff';
    ctx.globalAlpha = 0.6;
    ctx.beginPath();
    ctx.arc(x, y, r * 0.45, 0, Math.PI * 2);
    ctx.fill();

    ctx.globalAlpha = 1;
  }
}

export function drawTimeSeriesBoxplot() {
  const canvas = dom.boxplotCanvas;
  const buffer = tsBuffer;
  if (!canvas || !buffer || !buffer.hasData()) return;

  const ctx = canvas.getContext('2d');
  const W = canvas.width;
  const H = canvas.height;
  ctx.clearRect(0, 0, W, H);

  const margin = { left: 28, right: 4, top: 10, bottom: 18 };
  const plotLeft = margin.left;
  const plotTop = margin.top;
  const plotW = W - margin.left - margin.right;
  const plotH = H - margin.top - margin.bottom;

  const color = '#21918c';
  const blocks = buffer.getBlocks();
  const currentRaw = buffer.getCurrentRawValues();
  const hasCurrent = currentRaw.length > 0;

  const totalSlots = blocks.length + (hasCurrent ? 1 : 0);
  if (totalSlots === 0) return;

  const slotWidth = plotW / Math.max(totalSlots, 1);

  // Y-axis labels + grid lines
  const yLabels = [0, 25, 50, 75, 100];
  ctx.fillStyle = '#adb5bd';
  ctx.font = '9px monospace';
  ctx.textAlign = 'right';
  ctx.textBaseline = 'middle';

  for (const pct of yLabels) {
    const y = plotTop + plotH * (1 - pct / 100);
    ctx.strokeStyle = '#f0f0f0';
    ctx.lineWidth = 0.5;
    ctx.beginPath();
    ctx.moveTo(plotLeft, Math.round(y) + 0.5);
    ctx.lineTo(W - margin.right, Math.round(y) + 0.5);
    ctx.stroke();

    ctx.fillStyle = '#adb5bd';
    ctx.fillText(pct + '%', plotLeft - 3, y);
  }

  // Render each finalized block as a vertical boxplot
  for (let i = 0; i < blocks.length; i++) {
    const block = blocks[i];
    const cx = plotLeft + i * slotWidth + slotWidth / 2;
    const bp = computeBoxplot(block.coreValues);

    drawVerticalBoxplot(ctx, cx, plotTop, plotTop + plotH, slotWidth * 0.6, bp, color);

    ctx.fillStyle = '#adb5bd';
    ctx.font = '8px monospace';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    if (i % 2 === 0 || i === blocks.length - 1) {
      ctx.fillText(block.timestamp.slice(3), cx, plotTop + plotH + 3);
    }
  }

  // Render the current (in-progress) block as a stripplot
  if (hasCurrent) {
    const cx = plotLeft + blocks.length * slotWidth + slotWidth / 2;

    ctx.strokeStyle = '#21918c';
    ctx.lineWidth = 0.5;
    ctx.setLineDash([2, 3]);
    ctx.beginPath();
    ctx.moveTo(cx, plotTop);
    ctx.lineTo(cx, plotTop + plotH);
    ctx.stroke();
    ctx.setLineDash([]);

    drawStripplot(ctx, cx, plotTop, plotTop + plotH, slotWidth, currentRaw, color);

    ctx.fillStyle = '#21918c';
    ctx.font = '8px monospace';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    ctx.fillText('NOW', cx, plotTop + plotH + 3);
  }

  // Horizontal axis baseline
  ctx.strokeStyle = '#e5e7eb';
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(plotLeft, plotTop + plotH + 0.5);
  ctx.lineTo(W - margin.right, plotTop + plotH + 0.5);
  ctx.stroke();

  // Y-axis left border
  ctx.beginPath();
  ctx.moveTo(plotLeft + 0.5, plotTop);
  ctx.lineTo(plotLeft + 0.5, plotTop + plotH);
  ctx.stroke();
}

export function drawDensityPlot() {
  const canvas = dom.densityCanvas;
  const data = historyDensity;
  if (!canvas || data.length < 3) return;
  const ctx = canvas.getContext('2d');
  const w = canvas.width, h = canvas.height;
  ctx.clearRect(0, 0, w, h);

  const pad = 4;
  const drawW = w - pad * 2;
  const drawH = h - pad * 2;

  const n         = data.length;
  const minVal    = Math.min(...data);
  const maxVal    = Math.max(...data);
  const range     = Math.max(1, maxVal - minVal);
  const steps     = 60;
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

  const c = '#21918c';
  ctx.fillStyle   = c;
  ctx.globalAlpha = 0.2;
  ctx.beginPath();
  ctx.moveTo(pad, h - pad);
  for (let i = 0; i < steps; i++) {
    const x = pad + (i / (steps - 1)) * drawW;
    const y = (h - pad) - (density[i] / maxD) * drawH;
    ctx.lineTo(x, y);
  }
  ctx.lineTo(pad + drawW, h - pad);
  ctx.closePath();
  ctx.fill();
  ctx.globalAlpha = 1;

  ctx.strokeStyle = c;
  ctx.lineWidth   = 1.5;
  ctx.beginPath();
  for (let i = 0; i < steps; i++) {
    const x = pad + (i / (steps - 1)) * drawW;
    const y = (h - pad) - (density[i] / maxD) * drawH;
    i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
  }
  ctx.stroke();
}

export function drawGpuTimeSeries() {
  const canvas = dom.gpuTimeseriesCanvas;
  const data0 = gpuHistory0;
  const data1 = gpuHistory1;
  if (!canvas || data0.length < 2) return;
  const ctx = canvas.getContext('2d');
  const w = canvas.width, h = canvas.height;
  ctx.clearRect(0, 0, w, h);

  const pad  = { top: 4, right: 4, bottom: 14, left: 30 };
  const drawW = w - pad.left - pad.right;
  const drawH = h - pad.top - pad.bottom;

  const yMax = 100;

  // Grid lines at 25%, 50%, 75%
  ctx.strokeStyle = '#f0f0f0';
  ctx.lineWidth = 1;
  for (let pct of [25, 50, 75]) {
    const y = (h - pad.bottom) - (pct / yMax) * drawH;
    ctx.beginPath();
    ctx.moveTo(pad.left, y);
    ctx.lineTo(w - pad.right, y);
    ctx.stroke();
  }

  // Y-axis labels
  ctx.fillStyle = '#adb5bd';
  ctx.font      = '8px monospace';
  ctx.textAlign = 'right';
  ctx.fillText('100%', pad.left - 3, pad.top + 8);
  ctx.fillText('50%',  pad.left - 3, pad.top + drawH / 2 + 3);
  ctx.fillText('0',    pad.left - 3, h - pad.bottom - 2);

  const drawLine = (data, strokeColor, fillColor) => {
    const n = data.length;
    if (n < 2) return;

    const mapX = (i) => pad.left + (i / (n - 1)) * drawW;
    const mapY = (v) => (h - pad.bottom) - (v / yMax) * drawH;

    ctx.strokeStyle = strokeColor;
    ctx.lineWidth   = 1.5;
    ctx.beginPath();
    for (let i = 0; i < n; i++) {
      const x = mapX(i), y = mapY(data[i]);
      i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
    }
    ctx.stroke();

    if (fillColor) {
      ctx.fillStyle   = fillColor;
      ctx.globalAlpha = 0.1;
      ctx.beginPath();
      ctx.moveTo(mapX(0), h - pad.bottom);
      for (let i = 0; i < n; i++) {
        ctx.lineTo(mapX(i), mapY(data[i]));
      }
      ctx.lineTo(mapX(n - 1), h - pad.bottom);
      ctx.closePath();
      ctx.fill();
      ctx.globalAlpha = 1;
    }
  };

  drawLine(data0, '#21918c', 'rgba(33, 145, 140, 0.15)');

  // Skip GPU1 if not enough data (single-GPU system)
  if (data1.length >= 2) {
    drawLine(data1, '#d4a017', 'rgba(212, 160, 23, 0.12)');
  }

  ctx.fillStyle = '#21918c';
  ctx.font      = '7px monospace';
  ctx.textAlign = 'left';
  ctx.fillText('\u25B2GPU0', pad.left + 2, pad.top + 8);

  if (data1.length >= 2) {
    ctx.fillStyle = '#d4a017';
    ctx.fillText('\u25BCGPU1', pad.left + 42, pad.top + 8);
  }

  ctx.fillStyle = '#adb5bd';
  ctx.font      = '7px monospace';
  ctx.textAlign = 'center';
  ctx.fillText('time (s) \u2192', w / 2, h - 1);
}

export function updateCoreBars(cores = []) {
  const values = normalizeCores(cores);
  values.forEach((value, i) => {
    const bar = coreBars[i];
    bar.querySelector('.core-fill').style.width = `${value}%`;
    bar.querySelector('.core-value').textContent = `${value.toFixed(0)}%`;
    coreHistories[i].push(value);
    if (coreHistories[i].length > SPARK_POINTS) coreHistories[i].shift();
  });
}

export function updateHUD(data) {
  const cores = normalizeCores(data.cpu_detail?.cores || []);
  const avg = Number(data.cpu_detail?.avg_utilization ?? average(cores));
  const std = Number(data.cpu_detail?.std_dev ?? stdDev(cores));
  const variance = Number(data.cpu_detail?.variance ?? std * std);
  const memory = data.memory_detail || {};

  dom.cpuAvg.textContent = `${avg.toFixed(1)}%`;
  dom.cpuStd.textContent = std.toFixed(2);
  dom.cpuVar.textContent = variance.toFixed(2);
  dom.cpuBarFill.style.width = `${clamp(avg)}%`;

  const ramPercent = Number(memory.percent ?? data.ram_usage ?? 0);
  const usedGb = Number(memory.used_gb ?? 0);
  const totalGb = Number(memory.total_gb ?? 0);
  dom.ramText.textContent = totalGb > 0
    ? `${usedGb.toFixed(1)} / ${totalGb.toFixed(1)} GB (${ramPercent.toFixed(1)}%)`
    : `${ramPercent.toFixed(1)}%`;
  dom.ramBarFill.style.width = `${clamp(ramPercent)}%`;
  updateRamBlocks(ramPercent);
  updateProcessDots(memory.processes || []);

  dom.gpu0Text.textContent = formatGpuLine('GPU 0', data.gpu_detail?.[0], data);
  dom.gpu1Text.textContent = formatGpuLine('GPU 1', data.gpu_detail?.[1]);
  drawNetwork(data.network_speed || {});
}

export function setStatus(status) {
  dom.statusDot.className = 'status-dot';
  if (status !== 'online') dom.statusDot.classList.add(status);
  dom.statusTxt.textContent = status === 'online'
    ? 'ONLINE'
    : status === 'mock'
      ? 'MOCK TELEMETRY'
      : 'OFFLINE';
}

export function updateClock() {
  dom.clock.textContent = new Date().toLocaleTimeString('en-GB');
}

export function startMockTelemetry() {
  if (connectionState.mockInterval) return;
  emitMockTelemetry();
  connectionState.mockInterval = setInterval(emitMockTelemetry, 1000);
}

export function stopMockTelemetry() {
  if (!connectionState.mockInterval) return;
  clearInterval(connectionState.mockInterval);
  connectionState.mockInterval = null;
}

function emitMockTelemetry() {
  const now = Date.now() / 1000;
  const cores = Array.from({ length: CORE_COUNT }, (_, i) => (
    clamp(38 + Math.sin(now * 0.8 + i * 0.7) * 22 + Math.random() * 18)
  ));
  const avg = average(cores);
  const std = stdDev(cores);
  const memoryPercent = clamp(54 + Math.sin(now * 0.25) * 8 + Math.random() * 3);
  const mock = {
    cpu_detail: {
      cores,
      avg_utilization: avg,
      std_dev: std,
      variance: std * std,
    },
    memory_detail: {
      total_gb: 32,
      used_gb: memoryPercent * 0.32,
      percent: memoryPercent,
      processes: mockProcesses(memoryPercent),
    },
    gpu_detail: [
      { name: 'Mock GPU 0', load_percent: clamp(34 + Math.sin(now * 0.6) * 28), temperature_c: 58 + Math.sin(now * 0.2) * 5 },
      { name: 'Mock GPU 1', load_percent: clamp(18 + Math.cos(now * 0.5) * 16), temperature_c: 44 + Math.cos(now * 0.2) * 4 },
    ],
    network_speed: {
      sent_kbps: clamp(120 + Math.sin(now * 1.2) * 90),
      received_kbps: clamp(320 + Math.cos(now * 0.9) * 220),
    },
  };

  targetGrid.shift();
  targetGrid.push(cores);
  tsBuffer.addSample(cores);
  updateCoreBars(cores);
  updateHUD(mock);

  // Populate GPU history for chart rendering
  const gpu0load = clamp(mock.gpu_detail[0].load_percent);
  const gpu1load = clamp(mock.gpu_detail[1].load_percent);
  gpuHistory0.push(gpu0load);
  gpuHistory1.push(gpu1load);
  if (gpuHistory0.length > 30) {
    gpuHistory0.shift();
    gpuHistory1.shift();
  }
  historyDensity.push(gpu0load);
  if (historyDensity.length > 100) {
    historyDensity.splice(0, historyDensity.length - 100);
  }
}

function normalizeCores(cores) {
  const values = cores.slice(0, CORE_COUNT).map(value => clamp(Number(value) || 0));
  while (values.length < CORE_COUNT) values.push(0);
  return values;
}

function average(values) {
  return values.length ? values.reduce((sum, value) => sum + value, 0) / values.length : 0;
}

function stdDev(values) {
  const avg = average(values);
  return Math.sqrt(average(values.map(value => (value - avg) ** 2)));
}

function clamp(value) {
  return Math.max(0, Math.min(100, value));
}

function formatGpuLine(label, gpu, fallback = {}) {
  if (!gpu && fallback.gpu_usage == null) return `${label} - --C - Load: --%`;
  const name = gpu?.name || label;
  const temp = Number(gpu?.temperature_c ?? fallback.gpu_temperature ?? 0);
  const load = Number(gpu?.load_percent ?? fallback.gpu_usage ?? 0);
  return `${name} - ${temp.toFixed(0)}C - Load: ${load.toFixed(1)}%`;
}

function updateRamBlocks(percent) {
  const activeCount = Math.round((clamp(percent) / 100) * dom.allocationBlocks.children.length);
  [...dom.allocationBlocks.children].forEach((block, i) => {
    block.classList.toggle('active', i < activeCount);
  });
}

function updateProcessDots(processes) {
  dom.processDots.innerHTML = '';
  processes.slice(0, 12).forEach((process, i) => {
    const dot = document.createElement('span');
    dot.className = 'proc-dot';
    dot.style.left = `${Math.min(96, 4 + i * 8)}%`;
    dot.title = `${process.name || 'process'}: ${process.memory_mb || 0} MB`;
    dom.processDots.appendChild(dot);
  });
}

function mockProcesses(memoryPercent) {
  return ['browser', 'python', 'editor', 'system', 'gpu-worker'].map((name, i) => ({
    name,
    memory_mb: Math.round(memoryPercent * (65 - i * 7)),
  }));
}

function drawNetwork(network) {
  const canvas = dom.networkCanvas;
  const ctx = canvas.getContext('2d');
  const sent = Number(network.sent_kbps || 0);
  const recv = Number(network.received_kbps || 0);

  historyNetwork.sent.push(sent);
  historyNetwork.recv.push(recv);
  if (historyNetwork.sent.length > 30) {
    historyNetwork.sent.shift();
    historyNetwork.recv.shift();
  }

  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawLine(ctx, historyNetwork.recv, '#21918c', canvas);
  drawLine(ctx, historyNetwork.sent, '#fde725', canvas);
}

function drawLine(ctx, values, color, canvas) {
  const max = Math.max(1, ...values);
  ctx.beginPath();
  values.forEach((value, i) => {
    const x = (i / Math.max(1, values.length - 1)) * canvas.width;
    const y = canvas.height - (value / max) * (canvas.height - 12) - 6;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  });
  ctx.strokeStyle = color;
  ctx.lineWidth = 2;
  ctx.stroke();
}

// ----------------------------------------------------------------
// Resizable Column Splitters
// ----------------------------------------------------------------

const STORAGE_KEY = 'orbit-column-layout';
const MIN_COL_WIDTH = 290;

export function initSplitters() {
  const colLeft = document.getElementById('col-left');
  const colRight = document.getElementById('col-right');
  const splitters = document.querySelectorAll('.splitter');

  if (!colLeft || !colRight || splitters.length === 0) return;

  // Restore saved widths
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved) {
      const { left, right } = JSON.parse(saved);
      if (typeof left === 'number' && left >= MIN_COL_WIDTH) colLeft.style.width = left + 'px';
      if (typeof right === 'number' && right >= MIN_COL_WIDTH) colRight.style.width = right + 'px';
    }
  } catch (e) {
    // ignore corrupt localStorage
  }

  splitters.forEach(splitter => {
    splitter.addEventListener('mousedown', (e) => {
      e.preventDefault();

      const target = splitter.dataset.target; // 'left' or 'right'
      const col = target === 'left' ? colLeft : colRight;
      const startX = e.clientX;
      const startWidth = col.offsetWidth;

      const onMove = (e) => {
        const delta = e.clientX - startX;
        let newWidth;
        // Left splitter: dragging right → left column wider (+delta)
        // Right splitter: dragging right → right column narrower (-delta)
        if (target === 'left') {
          newWidth = startWidth + delta;
        } else {
          newWidth = startWidth - delta;
        }
        newWidth = Math.max(MIN_COL_WIDTH, newWidth);
        col.style.width = newWidth + 'px';
        splitter.classList.add('dragging');
      };

      const onUp = () => {
        document.removeEventListener('mousemove', onMove);
        document.removeEventListener('mouseup', onUp);
        splitter.classList.remove('dragging');
        // Persist to localStorage
        localStorage.setItem(STORAGE_KEY, JSON.stringify({
          left: colLeft.offsetWidth,
          right: colRight.offsetWidth,
        }));
        // Prevent text selection from lingering
        document.body.style.userSelect = '';
      };

      // Prevent text selection while dragging
      document.body.style.userSelect = 'none';
      document.addEventListener('mousemove', onMove);
      document.addEventListener('mouseup', onUp);
    });
  });
}
