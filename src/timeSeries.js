import { CORE_COUNT, TIME_WINDOW, SPARK_POINTS } from './constants.js';
import { coreHistories } from './state.js';

export class TimeSeriesBuffer {
  constructor(blockSize = 5, maxBlocks = 12) {
    this.blockSize = blockSize;
    this.maxBlocks = maxBlocks;
    this.blocks = [];
    this.currentValues = [];
    this.tickCount = 0;
  }

  addSample(coreValues) {
    if (!coreValues || coreValues.length === 0) return;
    for (const v of coreValues) {
      this.currentValues.push(Math.max(0, Math.min(100, v)));
    }
    this.tickCount++;
    if (this.tickCount >= this.blockSize) {
      this._finalizeBlock();
    }
  }

  _finalizeBlock() {
    const now = new Date();
    const ts = String(now.getHours()).padStart(2, '0') + ':' +
      String(now.getMinutes()).padStart(2, '0') + ':' +
      String(now.getSeconds()).padStart(2, '0');
    this.blocks.push({ timestamp: ts, coreValues: [...this.currentValues] });
    if (this.blocks.length > this.maxBlocks) this.blocks.shift();
    this.currentValues = [];
    this.tickCount = 0;
  }

  getBlocks() { return this.blocks; }
  getCurrentRawValues() { return this.currentValues; }
  hasData() { return this.blocks.length > 0 || this.currentValues.length > 0; }
}

export const tsBuffer = new TimeSeriesBuffer(5, 12);