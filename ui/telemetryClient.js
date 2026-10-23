import { connectionState, targetGrid, gpuHistory0, gpuHistory1, historyDensity } from './state.js';
import { tsBuffer } from './timeSeries.js';
import { updateCoreBars, updateHUD, setStatus, startMockTelemetry, stopMockTelemetry } from './ui.js';

export class TelemetryClient {
  constructor() {
    this.unlisten = null;
  }

  async connect() {
    // Check if running inside Tauri desktop shell
    const isTauri = typeof window !== 'undefined' && Boolean(window.__TAURI_INTERNALS__ || window.__TAURI__);

    if (isTauri) {
      try {
        let listenFn = window.__TAURI__?.event?.listen;
        if (!listenFn) {
          const tauriEvent = await import('@tauri-apps/api/event').catch(() => null);
          listenFn = tauriEvent?.listen;
        }

        if (listenFn) {
          connectionState.isConnected = true;
          setStatus('online');
          stopMockTelemetry();

          this.unlisten = await listenFn('telemetry-data', (event) => {
            this._handleMessage(event.payload);
          });
          return;
        }
      } catch (err) {
        console.warn('Could not connect to Tauri event channel, falling back to mock:', err);
      }
    }

    // Standalone browser fallback (dev preview or testing outside Tauri)
    connectionState.isConnected = false;
    setStatus('mock');
    startMockTelemetry();
  }

  disconnect() {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
    stopMockTelemetry();
  }

  _handleMessage(data) {
    if (!data) return;
    connectionState.latestMetrics = data;
    if (data.cpu_detail?.cores) pushTelemetryRow(data.cpu_detail.cores);
    updateCoreBars(data.cpu_detail?.cores);
    updateHUD(data);

    // Populate GPU history for chart rendering from real data
    if (data.gpu_detail && data.gpu_detail.length > 0) {
      data.gpu_detail.forEach((gpu, idx) => {
        const load = Math.max(0, Math.min(100, Number(gpu.load_percent) || 0));
        if (idx === 0) {
          gpuHistory0.push(load);
          historyDensity.push(load);
        } else if (idx === 1) {
          gpuHistory1.push(load);
        }
      });
      while (gpuHistory0.length > 30) gpuHistory0.shift();
      while (gpuHistory1.length > 30) gpuHistory1.shift();
      while (historyDensity.length > 100) {
        historyDensity.splice(0, historyDensity.length - 100);
      }
    }
  }
}

function pushTelemetryRow(cores) {
  const values = normalizeCores(cores);
  tsBuffer.addSample(values);
  targetGrid.shift();
  targetGrid.push(values);
}

function normalizeCores(cores) {
  const values = cores.slice(0, targetGrid[0].length).map(value => clamp(Number(value) || 0));
  while (values.length < targetGrid[0].length) values.push(0);
  return values;
}

function clamp(value) {
  return Math.max(0, Math.min(100, value));
}
