import { WS_URL } from './constants.js';
import { connectionState, targetGrid } from './state.js';
import { tsBuffer } from './timeSeries.js';
import { updateCoreBars, updateHUD, setStatus, startMockTelemetry, stopMockTelemetry } from './ui.js';

export class TelemetryClient {
  constructor() {
    this.ws = null;
    this.retryDelay = 2000;
    this.reconnectTimer = null;
  }

  connect() {
    if (this.ws) this.ws.close();

    this.ws = new WebSocket(WS_URL);
    this.ws.onopen = () => {
      connectionState.isConnected = true;
      setStatus('online');
      stopMockTelemetry();
    };
    this.ws.onmessage = event => this._handleMessage(JSON.parse(event.data));
    this.ws.onclose = () => {
      connectionState.isConnected = false;
      setStatus('mock');
      startMockTelemetry();
      this._scheduleReconnect();
    };
    this.ws.onerror = () => {
      this.ws.close();
    };
  }

  _handleMessage(data) {
    connectionState.latestMetrics = data;
    if (data.cpu_detail?.cores) pushTelemetryRow(data.cpu_detail.cores);
    updateCoreBars(data.cpu_detail?.cores);
    updateHUD(data);
  }

  _scheduleReconnect() {
    if (this.reconnectTimer) return;
    this.reconnectTimer = setTimeout(() => {
      this.connect();
      this.reconnectTimer = null;
    }, this.retryDelay);
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
