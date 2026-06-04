import { CORE_COUNT, TIME_WINDOW } from './constants.js';

export const targetGrid = Array.from({ length: TIME_WINDOW }, () => Array(CORE_COUNT).fill(0));
export const renderGrid = Array.from({ length: TIME_WINDOW }, () => Array(CORE_COUNT).fill(0));

export const connectionState = {
  isConnected: false,
  latestMetrics: null,
  retryDelay: 2000,
  reconnectTimer: null,
  mockInterval: null,
};

export const historyDensity = [];
export const gpuHistory0 = [];
export const gpuHistory1 = [];
export const historyNetwork = { time: [], sent: [], recv: [] };
export const coreHistories = Array.from({ length: CORE_COUNT }, () => []);
export const ramProcessData = [];
