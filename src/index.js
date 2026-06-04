import { ThreeScene } from './threeScene.js';
import { TelemetryClient } from './websocket.js';
import { dom, drawTimeSeriesBoxplot, drawDensityPlot, drawGpuTimeSeries, drawRamTimeSeries, drawAllSparklines, initSplitters, initSidebarToggles } from './ui.js';

// Side chart redraw throttle
let sideChartTick = 0;

function redrawSideCharts() {
  sideChartTick = (sideChartTick + 1) % 6;
  if (sideChartTick !== 0) return;

  drawDensityPlot();
  drawGpuTimeSeries();
  drawRamTimeSeries();
  drawTimeSeriesBoxplot();
  drawAllSparklines();
}

// Bootstrap
function init() {
  initSidebarToggles();
  initSplitters();
  const scene = new ThreeScene(dom.container);
  const client = new TelemetryClient();
  client.connect();

  // Animation loop
  function animate() {
    requestAnimationFrame(animate);
    scene.updateVertices(true);
    scene.controls.update();
    scene.renderer.render(scene.scene, scene.camera);
    redrawSideCharts();
  }
  animate();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}