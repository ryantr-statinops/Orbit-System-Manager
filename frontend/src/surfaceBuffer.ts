import { BufferAttribute, BufferGeometry, DynamicDrawUsage } from "three";
import type { FixtureSample } from "./fixtures";

export class CpuSurfaceBuffer {
  readonly geometry: BufferGeometry;
  readonly positions: Float32Array;
  readonly colors: Float32Array;
  readonly coreCount: number;
  readonly sampleCount: number;

  constructor(coreCount: number, sampleCount: number) {
    if (coreCount < 2 || sampleCount < 2) {
      throw new RangeError("A CPU surface needs at least two cores and two samples.");
    }

    this.coreCount = coreCount;
    this.sampleCount = sampleCount;
    this.positions = new Float32Array(coreCount * sampleCount * 3);
    this.colors = new Float32Array(coreCount * sampleCount * 3);
    this.geometry = new BufferGeometry();

    for (let core = 0; core < coreCount; core += 1) {
      for (let sample = 0; sample < sampleCount; sample += 1) {
        const vertex = core * sampleCount + sample;
        const positionOffset = vertex * 3;
        this.positions[positionOffset] = (sample / (sampleCount - 1) - 0.5) * 7;
        this.positions[positionOffset + 2] = (core / (coreCount - 1) - 0.5) * 4;
        this.setColor(vertex, 0);
      }
    }

    const indices: number[] = [];
    for (let core = 0; core < coreCount - 1; core += 1) {
      for (let sample = 0; sample < sampleCount - 1; sample += 1) {
        const topLeft = core * sampleCount + sample;
        const topRight = topLeft + 1;
        const bottomLeft = topLeft + sampleCount;
        const bottomRight = bottomLeft + 1;
        indices.push(topLeft, bottomLeft, topRight, topRight, bottomLeft, bottomRight);
      }
    }

    const positionAttribute = new BufferAttribute(this.positions, 3);
    positionAttribute.setUsage(DynamicDrawUsage);
    this.geometry.setAttribute("position", positionAttribute);
    this.geometry.setAttribute("color", new BufferAttribute(this.colors, 3));
    this.geometry.setIndex(indices);
  }

  update(samples: FixtureSample[]): boolean {
    if (samples.length !== this.sampleCount) return false;

    for (let core = 0; core < this.coreCount; core += 1) {
      for (let sample = 0; sample < this.sampleCount; sample += 1) {
        const value = samples[sample].cpuCorePercentages[core];
        if (value == null || !Number.isFinite(value)) return false;

        const vertex = core * this.sampleCount + sample;
        this.positions[vertex * 3 + 1] = (value / 100) * 2.5;
        this.setColor(vertex, value / 100);
      }
    }

    const positionAttribute = this.geometry.getAttribute("position") as BufferAttribute;
    const colorAttribute = this.geometry.getAttribute("color") as BufferAttribute;
    positionAttribute.needsUpdate = true;
    colorAttribute.needsUpdate = true;
    this.geometry.computeVertexNormals();
    this.geometry.computeBoundingSphere();
    return true;
  }

  dispose(): void {
    this.geometry.dispose();
  }

  private setColor(vertex: number, intensity: number): void {
    const value = Math.max(0, Math.min(1, intensity));
    const offset = vertex * 3;
    this.colors[offset] = 0.2 + value * 0.65;
    this.colors[offset + 1] = 0.4 + (1 - Math.abs(value - 0.5) * 2) * 0.35;
    this.colors[offset + 2] = 0.95 - value * 0.65;
  }
}
