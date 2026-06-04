import { coreHistories, historyDensity, gpuHistory0, gpuHistory1, historyNetwork } from './state.js';
import { tsBuffer } from './timeSeries.js';
import { ACCENT_COLOR } from './constants.js';

// Simple DOM helper
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
  ramHybridTrack: $('ram-hybrid-track'),
  allocationBlocks: $('allocation-blocks'),
  processDots: $('process-dots'),
  ramTooltip: $('ram-tooltip'),
};

// Core bar creation (simplified for brevity)
export const coreBars = [];
for (let i = 0; i < 16; i++) {
  const el = document.createElement('div');
  el.className = 'core-bar';
  el.innerHTML = `<span class="core-label">C${String(i).padStart(2, '0')}</span>`;
  dom.coreList.appendChild(el);
  coreBars.push(el);
}

// HUD overlay
const hudOverlay = document.createElement('div');
hudOverlay.className = 'hud-overlay';
hudOverlay.innerHTML = `<div class="hud-title">SYSTEM MONITOR // 3D TOPOLOGY</div>`;
dom.container.appendChild(hudOverlay);

// Placeholder functions for chart drawing – to be expanded later
export function drawAllSparklines() {}
export function drawTimeSeriesBoxplot() {}
export function updateCoreBars() {}
export function updateHUD() {}
export function setStatus() {}
export function updateClock() {}