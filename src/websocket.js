import { WS_URL, SPARK_POINTS } from './constants.js';
import { targetGrid, isConnected, latestMetrics, retryDelay, reconnectTimer, ws, mockInterval } from './state.js';
import { tsBuffer } from './timeSeries.js';
import { updateCoreBars, updateHUD, setStatus, startMockTelemetry, stopMockTelemetry } from './ui.js';

export class TelemetryClient {
  constructor() { this._init(); }
  _init() {
    this.ws = null;
    this.retryDelay = 2000;
    this.reconnectTimer = null;
  }
  connect() {
    if (this.ws) this.ws.close();
    this.ws = new WebSocket(WS_URL);
    this.ws.onopen = () => { isConnected = true; setStatus('online'); stopMockTelemetry(); };
    this.ws.onmessage = e => this._handleMessage(JSON.parse(e.data));
    this.ws.onclose = () => { isConnected = false; setStatus('offline'); this._scheduleReconnect(); };
    this.ws.onerror = () => { this.ws.close(); };
  }
  _handleMessage(data) {
    latestMetrics = data;
    if (data.cpu_detail?.cores) pushTelemetryRow(data.cpu_detail.cores);
    updateCoreBars(data.cpu_detail?.cores);
    updateHUD(data);
  }
  _scheduleReconnect() {
    if (this.reconnectTimer) return;
    this.reconnectTimer = setTimeout(() => { this.connect(); this.reconnectTimer = null; }, this.retryDelay);
  }
}