# Plan — Responsive Auto-Fit khi Resize Window

> **Mục tiêu:** App tự động co giãn (auto-fit) tất cả thành phần UI khi người dùng resize cửa sổ desktop hoặc kéo splitter thay đổi kích thước 3 cột.

---

## Phase 1: Phân tích hiện trạng

### 1.1 Three.js scene — không có resize handler
**File:** `src/threeScene.js:10-18`

Renderer và camera aspect ratio chỉ được khởi tạo **một lần** trong constructor:
```js
const containerW = container.clientWidth || window.innerWidth * 0.5;
const containerH = container.clientHeight || window.innerHeight;
this.renderer.setSize(containerW, containerH);
```
Không có method `resize()`, không có event listener → scene giữ nguyên kích thước ban đầu khi window thay đổi.

### 1.2 Canvas 2D trong side columns — hardcoded dimensions
**File:** `index.html:70,97,106,108,114`

Các `<canvas>` đều có `width` và `height` cứng (VD: `width="290" height="160"`).
Khi kéo splitter làm cột to/nhỏ hơn 290px, canvas không thay đổi.

**Canvas bị ảnh hưởng:**
- `#boxplot-canvas` — CPU boxplot (290x160)
- `#ram-timeseries-canvas` — RAM time-series (290x80)
- `#density-canvas` — GPU density plot (290x60)
- `#gpu-timeseries-canvas` — GPU time-series (290x80)
- `#network-canvas` — Network chart (290x90)
- `canvas.core-spark` — 40x14 trong mỗi core bar

### 1.3 Không có `window.resize` listener
**File:** `src/index.js`

Entry point chỉ khởi tạo scene + client + animation loop, không đăng ký resize event.

### 1.4 Splitter kéo không trigger resize
**File:** `src/ui.js:750-791` (hàm `initSplitters`)

Khi kéo splitter, kích thước cột thay đổi nhưng các canvas bên trong không được cập nhật.

---

## Phase 2: Kế hoạch triển khai

### 2.1 Thêm `resize()` method vào `ThreeScene`
**File:** `src/threeScene.js`

```js
resize() {
  const container = this.renderer.domElement.parentElement;
  const w = container.clientWidth;
  const h = container.clientHeight;
  this.renderer.setSize(w, h);
  this.camera.aspect = w / h;
  this.camera.updateProjectionMatrix();
}
```

**Giải thích:**
- Lấy kích thước mới từ container (`#center-canvas`)
- Cập nhật renderer và camera aspect ratio
- `updateProjectionMatrix()` là bắt buộc để Three.js tính lại ma trận chiếu

---

### 2.2 Tạo hàm `handleResize` trong `ui.js`
**File:** `src/ui.js`

```js
export function handleResize() {
  if (window.__threeScene) {
    window.__threeScene.resize();
  }

  resizeCanvas(dom.boxplotCanvas, 290, 160);
  resizeCanvas(dom.ramTimeseriesCanvas, 290, 80);
  resizeCanvas(dom.densityCanvas, 290, 60);
  resizeCanvas(dom.gpuTimeseriesCanvas, 290, 80);
  resizeCanvas(dom.networkCanvas, 290, 90);

  drawTimeSeriesBoxplot();
  drawDensityPlot();
  drawGpuTimeSeries();
  drawRamTimeSeries();
  drawAllSparklines();
}

function resizeCanvas(canvas, defaultWidth, defaultHeight) {
  if (!canvas) return;
  const parent = canvas.parentElement;
  const w = parent ? parent.clientWidth : defaultWidth;
  canvas.width = Math.max(100, w - 16);
  canvas.height = defaultHeight;
}
```

---

### 2.3 Gắn resize listener trong `index.js`
**File:** `src/index.js`

**Import thêm:**
```js
import { ..., handleResize } from './ui.js';
```

**Trong `init()`, sau khi `const scene = new ThreeScene(dom.container)`:**
```js
window.__threeScene = scene;

let resizeTimeout;
window.addEventListener('resize', () => {
  cancelAnimationFrame(resizeTimeout);
  resizeTimeout = requestAnimationFrame(handleResize);
});
```

**Tại sao dùng `requestAnimationFrame`:**
- Debounce tự nhiên (chờ đến frame kế tiếp)
- Tránh gọi resize quá nhiều lần khi kéo góc window
- Đồng bộ với vòng lặp render của trình duyệt

---

### 2.4 Patch splitter `onUp` để trigger resize
**File:** `src/ui.js` (trong `initSplitters`, callback `onUp`)

Sau dòng `localStorage.setItem(...)`:
```js
handleResize();
```

---

### 2.5 Xoá hardcoded dimensions khỏi HTML (optional)
**File:** `index.html`

Có thể xoá `width` và `height` attributes khỏi các `<canvas>`:
- `#boxplot-canvas`: `width="290" height="160"` → bỏ
- `#ram-timeseries-canvas`: `width="290" height="80"` → bỏ
- `#density-canvas`: `width="290" height="60"` → bỏ
- `#gpu-timeseries-canvas`: `width="290" height="80"` → bỏ
- `#network-canvas`: `width="290" height="90"` → bỏ

> **Lưu ý:** Có thể giữ lại để fallback nếu JS load chậm.

---

### 2.6 Sparkline canvases resize
**File:** `src/ui.js`

Trong `handleResize()`:
```js
document.querySelectorAll('.core-spark').forEach((canvas, i) => {
  const parent = canvas.parentElement;
  const availableWidth = parent ? parent.clientWidth - 24 - 30 - 5 - 2 : 40;
  const newWidth = Math.max(40, Math.min(80, availableWidth * 0.15));
  if (canvas.width !== newWidth) {
    canvas.width = newWidth;
    drawSparkline(canvas, coreHistories[i]);
  }
});
```

---

## Phase 3: Kiểm thử

### 3.1 Test Three.js resize
```js
window.__threeScene.resize();
// Kiểm tra: renderer.domElement.width / height = container mới
```

### 3.2 Test 2D canvas resize
- Kéo splitter → cột trái rộng hơn
- Kiểm tra: `canvas#boxplot-canvas.width > 290`

### 3.3 Test window resize
- Kéo góc window to/nhỏ → 3D scene + 2D charts fill đúng

### 3.4 Test toggle sidebar
- Ẩn/hiện panel trái/phải → center column mở rộng, scene update

### 3.5 Edge cases
- Resize về kích thước rất nhỏ (min-width)
- Toggle sidebar nhiều lần liên tiếp
- Kéo splitter đến giới hạn (290px min)
- Window minimize rồi restore

---

## Phase 4: Tác động & Rủi ro

| File | Thay đổi | Rủi ro |
|---|---|---|
| `src/threeScene.js` | +1 method `resize()` | Thấp |
| `src/ui.js` | +2 functions + patch `initSplitters` | Trung bình |
| `src/index.js` | +5 dòng | Thấp |
| `index.html` | Xoá width/height (optional) | Thấp |

**Không ảnh hưởng:** `constants.js`, `state.js`, `websocket.js`, `timeSeries.js`, `style.css`

---

## Phase 5: File tham chiếu

| File | Vai trò |
|---|---|
| `src/threeScene.js` | Thêm `resize()` method |
| `src/ui.js` | Thêm `handleResize()`, patch `initSplitters` |
| `src/index.js` | Thêm event listener + expose scene |
| `index.html` | Xoá hardcoded canvas dimensions (optional) |

---

## Tiến độ

```
Phase 1 — Phân tích hiện trạng   [■■■■■■■■■■] Hoàn thành
Phase 2 — Kế hoạch triển khai     [□□□□□□□□□□] 0/6 tasks
Phase 3 — Kiểm thử                 [□□□□□□□□□□] 0/5 tasks
Phase 4 — Tác động & Rủi ro       [■■■■■■■■■■] Hoàn thành
```
