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

### 2.1 Thêm `resize()` method vào `ThreeScene` (2.1)
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

### 2.2 Tạo hàm `handleResize` trong `ui.js` (2.2)
**File:** `src/ui.js`

**Thêm cuối file** (trước export hoặc cuối module):

```js
export function handleResize() {
  // Resize Three.js
  if (window.__threeScene) {
    window.__threeScene.resize();
  }

  // Resize 2D canvases in col-left
  resizeCanvas(dom.boxplotCanvas, 290, 160);
  // Resize 2D canvases in col-right
  resizeCanvas(dom.ramTimeseriesCanvas, 290, 80);
  resizeCanvas(dom.densityCanvas, 290, 60);
  resizeCanvas(dom.gpuTimeseriesCanvas, 290, 80);
  resizeCanvas(dom.networkCanvas, 290, 90);

  // Redraw all charts
  drawTimeSeriesBoxplot();
  drawDensityPlot();
  drawGpuTimeSeries();
  drawRamTimeSeries();
  drawAllSparklines();
}

function resizeCanvas(canvas, defaultHeight) {
  if (!canvas) return;
  const parent = canvas.parentElement;
  const w = parent ? parent.clientWidth : defaultHeight;
  const ratio = canvas.width / canvas.height || 1;
  canvas.width = Math.max(100, w - 16); // account for card padding
  canvas.height = Math.round(canvas.width / ratio);
}
```

> **Lưu ý:** Các canvas trong side columns không cần thay đổi `height`, chỉ cần `width` co giãn theo cột. Giữ nguyên aspect ratio để nội dung không bị méo.

---

### 2.3 Gắn resize listener trong `index.js` (2.3)
**File:** `src/index.js`

**Import thêm:**
```js
import { ..., handleResize } from './ui.js';
```

**Trong `init()`, sau khi `const scene = new ThreeScene(dom.container)`:**
```js
// Expose scene cho resize handler
window.__threeScene = scene;

// Debounced resize handler
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

### 2.4 Patch splitter `onUp` để trigger resize (2.4)
**File:** `src/ui.js:775-784` (trong `initSplitters`)

Trong callback `onUp`, thêm sau dòng `localStorage.setItem(...)`:
```js
handleResize();
```

Đảm bảo import hoặc expose `handleResize` trong scope của `initSplitters`.

---

### 2.5 Xoá hardcoded dimensions khỏi HTML (2.5 — optional)
**File:** `index.html`

Có thể xoá `width` và `height` attributes khỏi các `<canvas>` vì chúng sẽ được set động qua JS:
- `#boxplot-canvas`: `width="290" height="160"` → bỏ
- `#ram-timeseries-canvas`: `width="290" height="80"` → bỏ
- `#density-canvas`: `width="290" height="60"` → bỏ
- `#gpu-timeseries-canvas`: `width="290" height="80"` → bỏ
- `#network-canvas`: `width="290" height="90"` → bỏ

Không xoá `canvas.core-spark` (40x14) vì chúng được tạo động trong JS.

> **Lưu ý:** Việc này có thể làm mất kích thước mặc định nếu JS load chậm — cần cân nhắc hoặc giữ lại để fallback.

---

### 2.6 Sparkline canvases resize (2.6)
**File:** `src/ui.js`

Các sparkline canvas (40x14) trong core bars được tạo động ở dòng 29-41. Khi cột trái rộng hơn 290px, có thể tăng chiều rộng sparkline để tận dụng không gian.

Trong `handleResize()`:
```js
document.querySelectorAll('.core-spark').forEach((canvas, i) => {
  const parent = canvas.parentElement;
  const availableWidth = parent ? parent.clientWidth - 24 - 30 - 5 - 2 : 40; // label + track + value + gap
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
// Mở DevTools Console:
window.__threeScene.resize();
// Kiểm tra: renderer.domElement.width / height = container mới
```

### 3.2 Test 2D canvas resize
```js
// Kéo splitter giữa → cột trái rộng hơn
// Kiểm tra: canvas#boxplot-canvas.width > 290
// Kiểm tra: nội dung boxplot được redraw đúng
```

### 3.3 Test window resize
- Kéo góc window to ra / nhỏ lại
- Kiểm tra: 3D scene fill đúng center column
- Kiểm tra: 2D charts fill đúng side columns

### 3.4 Test toggle sidebar
- Nhấn nút mũi tên trên top bar để ẩn/hiện panel trái/phải
- Kiểm tra: center column mở rộng chiếm khoảng trống
- Kiểm tra: 3D scene renderer update kích thước

### 3.5 Edge cases
- Resize về kích thước rất nhỏ (min-width)
- Toggle sidebar nhiều lần liên tiếp
- Kéo splitter đến giới hạn (290px min)
- Window bị minimize rồi restore

---

## Phase 4: Tác động & Rủi ro

| File | Thay đổi | Rủi ro |
|---|---|---|
| `src/threeScene.js` | +1 method `resize()` | Thấp — chỉ thêm method mới |
| `src/ui.js` | +2 functions: `handleResize()`, `resizeCanvas()` + patch `initSplitters` | Trung bình — cần import đúng |
| `src/index.js` | +5 dòng | Thấp |
| `index.html` | Có thể xoá width/height (optional) | Thấp — nếu JS không load thì canvas mất kích thước mặc định |

**Không ảnh hưởng:** `constants.js`, `state.js`, `websocket.js`, `timeSeries.js`, `style.css`

---

## Phase 5: File tham chiếu

| File | Vai trò |
|---|---|
| `src/threeScene.js` | Cần thêm `resize()` method |
| `src/ui.js` | Cần thêm `handleResize()`, patch `initSplitters` |
| `src/index.js` | Cần thêm event listener + expose scene |
| `index.html` | Có thể xoá hardcoded canvas dimensions |
| `style.css` | Không cần thay đổi (CSS đã dùng `flex: 1` cho center canvas) |

---

## Tiến độ

```
Phase 1 — Phân tích hiện trạng   [■■■■■■■■■■] Hoàn thành
Phase 2 — Kế hoạch triển khai     [□□□□□□□□□□] 0/6 tasks
Phase 3 — Kiểm thử                 [□□□□□□□□□□] 0/5 tasks
Phase 4 — Tác động & Rủi ro       [■■■■■■■■■■] Hoàn thành
```
