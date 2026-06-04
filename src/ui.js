import { CORE_COUNT, SPARK_POINTS } from './constants.js';
import { connectionState, coreHistories, historyNetwork, targetGrid } from './state.js';
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
  `;
  dom.coreList.appendChild(el);
  coreBars.push(el);
}

const hudOverlay = document.createElement('div');
hudOverlay.className = 'hud-overlay';
hudOverlay.innerHTML = '<div class="hud-title">SYSTEM MONITOR // 3D TOPOLOGY</div>';
dom.container.appendChild(hudOverlay);

for (let i = 0; i < 24; i++) {
  const block = document.createElement('div');
  block.className = 'block';
  dom.allocationBlocks.appendChild(block);
}

updateClock();
setInterval(updateClock, 1000);

export function drawAllSparklines() {}
export function drawTimeSeriesBoxplot() {}

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
