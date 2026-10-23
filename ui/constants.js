import * as THREE from 'three';

// All project‑wide constants
export const ALPHA = 0.1;
export const CORE_COUNT = 16;
export const TIME_WINDOW = 30; // 30‑second rolling window
export const MAX_HEIGHT = 20.0;
export const SQUARE_SPAN = 60;
export const SEGMENTS_X = CORE_COUNT - 1;
export const SEGMENTS_Z = TIME_WINDOW - 1;
export const X_SPACING = SQUARE_SPAN / SEGMENTS_X;
export const Z_SPACING = SQUARE_SPAN / SEGMENTS_Z;
export const PLANE_WIDTH = SQUARE_SPAN;
export const PLANE_DEPTH = SQUARE_SPAN;
export const WS_URL = 'ws://127.0.0.1:8080';
export const SPARK_POINTS = 40;
export const ACCENT_COLOR = new THREE.Color('#21918c');
