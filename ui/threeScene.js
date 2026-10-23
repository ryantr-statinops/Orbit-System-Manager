import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { ALPHA, CORE_COUNT, TIME_WINDOW, MAX_HEIGHT, SQUARE_SPAN, SEGMENTS_X, SEGMENTS_Z, X_SPACING, Z_SPACING, PLANE_WIDTH, PLANE_DEPTH, ACCENT_COLOR } from './constants.js';
import { targetGrid, renderGrid } from './state.js';

export class ThreeScene {
  constructor(container) {
    this.scene = new THREE.Scene();
    this.scene.background = new THREE.Color('#f8f9fa');
    const containerW = container.clientWidth || window.innerWidth * 0.5;
    const containerH = container.clientHeight || window.innerHeight;
    const aspect = containerW / containerH;
    this.camera = new THREE.PerspectiveCamera(40, aspect, 0.1, 200);
    this.camera.position.set(PLANE_WIDTH * 0.8, MAX_HEIGHT * 3.5, PLANE_DEPTH * 0.7);
    this.camera.lookAt(0, MAX_HEIGHT * 0.3, 0);
    this.renderer = new THREE.WebGLRenderer({ antialias: true });
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    this.renderer.setSize(containerW, containerH);
    this.renderer.setClearColor('#f8f9fa');
    container.appendChild(this.renderer.domElement);
    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableDamping = true;
    this.controls.dampingFactor = 0.08;
    this.controls.autoRotate = false;
    this._initGeometry();
  }

  resize() {
    const container = this.renderer.domElement.parentElement;
    const w = container.clientWidth;
    const h = container.clientHeight;
    this.renderer.setSize(w, h);
    this.camera.aspect = w / h;
    this.camera.updateProjectionMatrix();
  }

  _initGeometry() {
    const geometry = new THREE.PlaneGeometry(PLANE_WIDTH, PLANE_DEPTH, SEGMENTS_X, SEGMENTS_Z);
    geometry.rotateX(-Math.PI / 2);
    const posAttr = geometry.attributes.position;
    for (let iz = 0; iz <= SEGMENTS_Z; iz++) {
      for (let ix = 0; ix <= SEGMENTS_X; ix++) {
        const idx = iz * CORE_COUNT + ix;
        const x = ix * X_SPACING - PLANE_WIDTH / 2;
        const z = iz * Z_SPACING - PLANE_DEPTH / 2;
        posAttr.setXYZ(idx, x, 0, z);
      }
    }
    posAttr.needsUpdate = true;
    const fillMat = new THREE.MeshBasicMaterial({ color: ACCENT_COLOR, transparent: true, opacity: 0.06, side: THREE.DoubleSide });
    const wireMat = new THREE.MeshBasicMaterial({ color: ACCENT_COLOR, wireframe: true, transparent: true, opacity: 0.55 });
    const fillMesh = new THREE.Mesh(geometry, fillMat);
    const wireMesh = new THREE.Mesh(geometry, wireMat);
    this.gridGroup = new THREE.Group();
    this.gridGroup.add(fillMesh, wireMesh);
    this.scene.add(this.gridGroup);
  }

  updateVertices(doLerp) {
    if (doLerp) {
      for (let iz = 0; iz <= SEGMENTS_Z; iz++) {
        const rRow = renderGrid[iz];
        const tRow = targetGrid[iz];
        for (let ix = 0; ix <= SEGMENTS_X; ix++) {
          rRow[ix] += ALPHA * (tRow[ix] - rRow[ix]);
        }
      }
    }
    const pos = this.gridGroup.children[0].geometry.attributes.position;
    for (let iz = 0; iz <= SEGMENTS_Z; iz++) {
      const row = renderGrid[iz];
      for (let ix = 0; ix <= SEGMENTS_X; ix++) {
        const idx = iz * CORE_COUNT + ix;
        const val = row[ix];
        const h = (val / 100) * MAX_HEIGHT;
        pos.setY(idx, h);
      }
    }
    pos.needsUpdate = true;
  }
}