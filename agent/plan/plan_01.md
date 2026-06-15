# Plan 01 — Foundation & Cleanup

> **Mục tiêu:** Đưa dự án về trạng thái "chạy được" và "sạch" trước khi phát triển tính năng mới.

---

## Phase 1: 🚨 Fix lỗi chặn (Critical)

### 1.1 Fix lỗi Python backend — IndentationError (app.py)

**File:** `app.py`
**Vị trí:** Method `WebSocketServer.start()` (~ dòng 405-416)

**Vấn đề:** 4 dòng `logger.info(...)` bị sai indentation — đang ở cấp ngoài hàm thay vì trong method body.

```python
# SAI — indent 4 spaces (cấp độ global)
    logger.info("=" * 52)
    logger.info("  Orbit System Manager — Backend Service")
    ...

# Đúng — indent 8 spaces (trong method body)
        logger.info("=" * 52)
        logger.info("  Orbit System Manager — Backend Service")
        ...
```

**Kiểm tra:** `conda run -n orbit-system python app.py --port 8080` phải chạy được.

---

### 1.2 Cài Rust toolchain

**Cần:** Rust ≥ 1.77.2 + Cargo

```bash
# Windows: https://rustup.rs
rustup-init.exe
# Kiểm tra
rustc --version   # ≥ 1.77
cargo --version
```

**Tại sao:** Bắt buộc để build Tauri desktop app (`npm run build:app`).

---

### 1.3 Xoá plotly.js — 3.59 MB dead weight

**Vấn đề:**
- `libs/plotly.min.js` — **3.59 MB** file vendored, không được import ở bất kỳ file JS/HTML nào
- `package.json` có `"plotly.js": "^2.20.0"` — cũng không dùng
- Project dùng Canvas 2D API tự vẽ (trong `ui.js`) + Three.js cho 3D

**Cần làm:**
```bash
npm uninstall plotly.js
rm libs/plotly.min.js
# Nếu thư mục libs/ trống → xoá luôn: rm -rf libs/
```

**Kiểm tra:** `npx rollup -c` vẫn build thành công, frontend vẫn hiển thị charts.

---

### 1.4 Fix Python backend syntax (app.py)

**Xong sau 1.1** — Xác nhận bằng `python -c "import ast; ast.parse(open('app.py').read()); print('OK')"`.

---

## Phase 2: 🛠️ Củng cố nền tảng

### 2.1 Fix `.git/info/exclude` — xoá rule `/agent`

**File:** `.git/info/exclude`
**Dòng cần xoá:** `/agent` (nằm trong block `# kanban-managed-symlinked-ignored-paths`)

**Vấn đề:** Rule này ngăn Git theo dõi file mới trong thư mục `agent/`, buộc phải `git add -f` mỗi lần thêm file.

**Cách fix:** Xoá dòng `/agent` khỏi file.

---

### 2.2 Cập nhật remote URL (nếu cần)

```bash
git remote set-url origin https://github.com/ryantr-statinops/Orbit-System-Manager.git
```

GitHub đã tự động redirect từ `task_manager_3D_visualize` → `Orbit-System-Manager`.

---

### 2.3 Thêm linting & formatting scripts

**Python:**
- Thêm `ruff` (hoặc `flake8`) vào dev dependencies
- Config trong `pyproject.toml` hoặc `setup.cfg`
- Script: `ruff check app.py`

**JavaScript:**
- Thêm `eslint` vào devDependencies (npm)
- Script: `"lint": "eslint src/"` trong `package.json`

---

### 2.4 Thêm `.vscode/settings.json`

Để VS Code tự động dùng đúng Python interpreter từ conda:

```json
{
  "python.defaultInterpreterPath": "C:\\Users\\Admin\\miniconda3\\envs\\orbit-system\\python.exe",
  "python.terminal.activateEnvironment": true,
  "files.associations": {
    "*.md": "markdown"
  }
}
```

---

## Phase 3: 🧪 Kiểm thử tích hợp

### 3.1 Chạy thử Python backend

```bash
conda activate orbit-system
python app.py --port 8080
# Kỳ vọng: Server ready. Press Ctrl+C to stop.
```

### 3.2 Kiểm tra WebSocket kết nối

```bash
# Terminal 1: python app.py --port 8080
# Terminal 2: conda run -n orbit-system python -c "
import asyncio
import websockets
async def test():
    async with websockets.connect('ws://localhost:8080') as ws:
        msg = await asyncio.wait_for(ws.recv(), timeout=5)
        print('Received:', msg[:200])
asyncio.run(test())
"
```

### 3.3 Kiểm tra Frontend + Three.js

```bash
npm run build          # Build Rollup → dist/bundle.js
# Mở index.html trong browser
# Kỳ vọng: 3D scene hiển thị với mock data (vì backend chưa chạy)
```

### 3.4 Kiểm tra Tauri dev mode (sau khi cài Rust)

```bash
npm run dev
# Kỳ vọng: Cửa sổ desktop mở, frontend load, WebSocket kết nối backend
```

---

## Phase 4: 📋 Implement spec còn thiếu

Các tính năng đã được spec trong `agent/promts/` nhưng chưa implement:

### 4.1 Viridis/Jet colormap cho 3D Surface

**File:** `src/threeScene.js`
**Spec:** `agent/promts/05_ui_layout_rules.md` (phần 2 — MATLAB-style)

**Cần làm:**
- Thay thế `MeshBasicMaterial` màu đơn `#21918c` bằng vertex coloring
- Map độ cao Y → Jet colormap (xanh đậm → cyan → xanh lá → vàng → đỏ)
- Hoặc Viridis colormap (tím → xanh dương → teal → xanh lá → vàng)

### 4.2 Z-axis smooth scrolling

**File:** `src/threeScene.js`, `src/constants.js`
**Spec:** `agent/promts/06_interpolation_and_smoothing_rules.md`

**Cần làm:**
- Thêm `THREE.Clock` để lấy `deltaTime` giữa các frame
- Trượt mượt mesh theo trục Z thay vì shift đột ngột mỗi giây
- Config alpha riêng cho Z-scroll

### 4.3 Axis labels + Bounding box cho 3D scene

**File:** `src/threeScene.js`
**Spec:** `agent/promts/05_ui_layout_rules.md`

**Cần làm:**
- Vẽ khung hộp dotted line bao quanh mesh (`LineSegments` + `LineDashedMaterial`)
- Thêm axis labels (X: cores, Z: time, Y: load %)
- Grid lines nền xám mảnh

### 4.4 Vertex coloring theo độ cao (Terrain Heatmap)

**File:** `src/threeScene.js`
**Spec:** `agent/promts/03_threejs_3d_timeseries.md`

**Cần làm:**
- Sử dụng `BufferAttribute` cho màu sắc (`geometry.setAttribute('color', ...)`)
- Map từ 0-100% load → gradient Viridis
- Cập nhật màu mỗi frame trong `updateVertices()`

---

## Phase 5: 🚀 Roadmap features

Từ `agent/promts/04_lively_integration_plan.md`:

- [ ] **Process Manager Cluster:** Top processes → 3D Point Cloud với K-means clustering
- [ ] **Global Keyboard Search:** Command palette tìm kiếm file/process
- [ ] **Power Optimization:** Auto-hạ FPS (60→1) khi phát hiện fullscreen/gaming
- [ ] **Auto-updater:** Plugin Tauri updater
- [ ] **Tray icon + auto-start:** Windows system tray integration

---

## Tổng quan tiến độ

```
Phase 1 — Fix lỗi chặn         [■■■■■■■■■■] 4/4 tasks
Phase 2 — Củng cố nền tảng     [■■□□□□□□□□] 1/4 tasks
Phase 3 — Kiểm thử tích hợp    [□□□□□□□□□□] 0/4 tasks
Phase 4 — Spec còn thiếu       [□□□□□□□□□□] 0/4 tasks
Phase 5 — Roadmap features     [□□□□□□□□□□] 0/7 tasks
```

---

## File tham chiếu

| File | Vai trò |
|------|---------|
| `app.py` | Python backend (cần fix) |
| `src/threeScene.js` | 3D rendering engine |
| `src/constants.js` | Constants (ALPHA, CORE_COUNT, COLORS...) |
| `src/ui.js` | Canvas 2D charts |
| `src/websocket.js` | WebSocket client + mock |
| `src-tauri/` | Rust/Tauri desktop shell |
| `.git/info/exclude` | Local git exclude rules |
| `agent/promts/*.md` | Specification documents |
