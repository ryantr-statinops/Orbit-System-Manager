import { useEffect, useRef, useState } from "react";
import {
  AmbientLight,
  Color,
  DirectionalLight,
  DoubleSide,
  Mesh,
  MeshStandardMaterial,
  PerspectiveCamera,
  Scene,
  WebGLRenderer,
  GridHelper,
} from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import type { DashboardFixture } from "./fixtures";
import { getRenderPixelRatio } from "./renderPolicy";
import { CpuSurfaceBuffer } from "./surfaceBuffer";

interface CpuSurfaceChartProps {
  fixture: DashboardFixture;
}

interface SurfaceViewer {
  scene: Scene;
  camera: PerspectiveCamera;
  renderer: WebGLRenderer;
  controls: OrbitControls;
  material: MeshStandardMaterial;
  mesh: Mesh | null;
  requestRender: () => void;
}

function CpuSurfaceChart({ fixture }: CpuSurfaceChartProps) {
  const host = useRef<HTMLDivElement>(null);
  const viewer = useRef<SurfaceViewer | null>(null);
  const surfaceBuffer = useRef<CpuSurfaceBuffer | null>(null);
  const [webglFailed, setWebglFailed] = useState(false);
  const hasSurfaceDimensions = fixture.coreCount >= 2 && fixture.samples.length >= 2;
  const hasCpuValues = fixture.samples.every(
    (sample) =>
      sample.cpuCorePercentages.length === fixture.coreCount &&
      sample.cpuCorePercentages.every((value) => value !== null),
  );

  useEffect(() => {
    const element = host.current;
    if (!element) return;

    let frameHandle = 0;
    let activeViewer: SurfaceViewer;

    try {
      const renderer = new WebGLRenderer({
        alpha: true,
        antialias: false,
        powerPreference: "low-power",
      });
      renderer.setPixelRatio(getRenderPixelRatio(window.devicePixelRatio));
      renderer.setClearColor(0x111a26, 0);
      element.appendChild(renderer.domElement);

      const scene = new Scene();
      scene.background = new Color(0x111a26);
      scene.add(new AmbientLight(0xffffff, 1.5));
      const keyLight = new DirectionalLight(0xc7e5ff, 2.2);
      keyLight.position.set(4, 8, 6);
      scene.add(keyLight);
      scene.add(new GridHelper(7, 14, 0x35506a, 0x26384b));

      const camera = new PerspectiveCamera(42, 1, 0.1, 100);
      camera.position.set(8, 7, 10);
      const controls = new OrbitControls(camera, renderer.domElement);
      controls.target.set(0, 0.7, 0);
      controls.enableDamping = false;
      controls.update();
      controls.saveState();

      const material = new MeshStandardMaterial({
        vertexColors: true,
        side: DoubleSide,
        roughness: 0.75,
        metalness: 0.05,
      });

      const requestRender = () => {
        if (document.hidden || frameHandle !== 0) return;
        frameHandle = requestAnimationFrame(() => {
          frameHandle = 0;
          renderer.render(scene, camera);
        });
      };

      const onVisibilityChange = () => {
        if (document.hidden) {
          if (frameHandle !== 0) cancelAnimationFrame(frameHandle);
          frameHandle = 0;
          return;
        }
        requestRender();
      };

      controls.addEventListener("change", requestRender);
      document.addEventListener("visibilitychange", onVisibilityChange);
      const resizeObserver = new ResizeObserver(() => {
        const width = Math.max(1, element.clientWidth);
        const height = Math.max(1, element.clientHeight);
        camera.aspect = width / height;
        camera.updateProjectionMatrix();
        renderer.setSize(width, height, false);
        requestRender();
      });
      resizeObserver.observe(element);

      activeViewer = {
        scene,
        camera,
        renderer,
        controls,
        material,
        mesh: null,
        requestRender,
      };
      viewer.current = activeViewer;
      requestRender();

      return () => {
        resizeObserver.disconnect();
        controls.removeEventListener("change", requestRender);
        document.removeEventListener("visibilitychange", onVisibilityChange);
        controls.dispose();
        if (frameHandle !== 0) cancelAnimationFrame(frameHandle);
        if (activeViewer.mesh) {
          scene.remove(activeViewer.mesh);
        }
        surfaceBuffer.current?.dispose();
        surfaceBuffer.current = null;
        material.dispose();
        renderer.dispose();
        renderer.domElement.remove();
        viewer.current = null;
      };
    } catch {
      setWebglFailed(true);
    }
  }, []);

  useEffect(() => {
    const activeViewer = viewer.current;
    if (!activeViewer) return;

    if (!hasSurfaceDimensions || !hasCpuValues) {
      if (activeViewer.mesh) {
        activeViewer.scene.remove(activeViewer.mesh);
        activeViewer.mesh = null;
        activeViewer.requestRender();
      }
      return;
    }

    let buffer = surfaceBuffer.current;
    if (
      buffer === null ||
      buffer.coreCount !== fixture.coreCount ||
      buffer.sampleCount !== fixture.samples.length
    ) {
      buffer?.dispose();
      buffer = new CpuSurfaceBuffer(fixture.coreCount, fixture.samples.length);
      surfaceBuffer.current = buffer;
    }

    if (!buffer.update(fixture.samples)) return;

    if (activeViewer.mesh === null) {
      activeViewer.mesh = new Mesh(buffer.geometry, activeViewer.material);
      activeViewer.scene.add(activeViewer.mesh);
    } else if (activeViewer.mesh.geometry !== buffer.geometry) {
      activeViewer.mesh.geometry = buffer.geometry;
    }

    activeViewer.requestRender();
  }, [fixture, hasCpuValues, hasSurfaceDimensions]);

  const message = webglFailed
    ? "WebGL could not be initialized in this environment."
    : !hasSurfaceDimensions
      ? "The surface needs at least two cores and two samples."
      : !hasCpuValues
        ? "CPU values are unavailable for this fixture."
        : null;

  return (
    <section className="surface-panel" aria-label="CPU utilization surface">
      <div className="surface-panel-heading">
        <div>
          <p className="eyebrow">CPU SURFACE</p>
          <p className="chart-description">Time × logical core × utilization</p>
        </div>
        <button
          className="camera-reset"
          type="button"
          onClick={() => viewer.current?.controls.reset()}
          disabled={message !== null}
        >
          Reset camera
        </button>
      </div>
      <div className="surface-plot">
        <div ref={host} className="surface-canvas" />
        {message !== null && <p className="surface-overlay">{message}</p>}
      </div>
    </section>
  );
}

export default CpuSurfaceChart;
