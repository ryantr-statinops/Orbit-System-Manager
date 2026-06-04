import { ThreeScene } from './threeScene.js';
import { TelemetryClient } from './websocket.js';
import { dom } from './ui.js';

// Bootstrap
function init() {
  const scene = new ThreeScene(dom.container);
  const client = new TelemetryClient();
  client.connect();
  // Animation loop
  function animate() {
    requestAnimationFrame(animate);
    scene.updateVertices(true);
    scene.renderer.render(scene.scene, scene.camera);
  }
  animate();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}