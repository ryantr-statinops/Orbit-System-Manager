# Task 01 — Foundation & Cleanup Checklist

> **Phase:** Foundation
> **Bắt đầu:** 15/06/2026
> **Plan tham chiếu:** `agent/plan/plan_01.md`

---

## Phase 1: 🚨 Fix lỗi chặn (Critical)

### [ ] 1.1 Fix lỗi Python backend — IndentationError

- [ ] Mở `app.py`, tìm method `WebSocketServer.start()` (~ dòng 405-416)
- [ ] Sửa indent của 4 dòng `logger.info(...)` vào đúng trong method body
- [ ] Kiểm tra: `conda activate orbit-system && python app.py --port 8080`
- [ ] Kỳ vọng: Server chạy, không báo lỗi

### [ ] 1.2 Cài Rust toolchain

- [ ] Download rustup-init.exe từ https://rustup.rs
- [ ] Chạy cài đặt (mặc định)
- [ ] Kiểm tra: `rustc --version` (≥ 1.77) và `cargo --version`

### [ ] 1.3 Xoá plotly.js — 3.59 MB dead weight

- [ ] Kiểm tra plotly.js có thực sự không được import: `rg "plotly" src/ index.html`
- [ ] `npm uninstall plotly.js`
- [ ] `rm -rf libs/plotly.min.js` (hoặc xoá thủ công)
- [ ] Nếu thư mục `libs/` trống: `rm -rf libs/`
- [ ] Build lại: `npx rollup -c`
- [ ] Mở `index.html` kiểm tra charts vẫn hiển thị

### [ ] 1.4 Xác nhận Python syntax OK

- [ ] `conda run -n orbit-system python -c "import ast; ast.parse(open('app.py').read()); print('OK')"`
- [ ] Kỳ vọng: in ra `OK`

---

## Phase 2: 🛠️ Củng cố nền tảng

### [ ] 2.1 Fix `.git/info/exclude` — xoá rule `/agent`

- [ ] Mở `.git/info/exclude`
- [ ] Xoá dòng `/agent` trong block `# kanban-managed-symlinked-ignored-paths`
- [ ] Kiểm tra: `git status` — file mới trong `agent/` hiện ra

### [ ] 2.2 Cập nhật remote URL

- [ ] `git remote set-url origin https://github.com/ryantr-statinops/Orbit-System-Manager.git`
- [ ] Kiểm tra: `git remote -v`

### [ ] 2.3 Thêm linting & formatting

- [ ] **Python:** `pip install ruff` (trong conda env)
- [ ] Tạo `pyproject.toml` với config ruff
- [ ] Chạy thử: `ruff check app.py`
- [ ] **JavaScript:** `npm install --save-dev eslint`
- [ ] Tạo `.eslintrc.json` cơ bản
- [ ] Thêm script `"lint"` vào `package.json`

### [ ] 2.4 Tạo `.vscode/settings.json`

- [ ] Tạo thư mục `.vscode/`
- [ ] Tạo file `settings.json` với python interpreter path đúng

---

## Phase 3: 🧪 Kiểm thử tích hợp

### [ ] 3.1 Chạy thử Python backend

- [ ] `conda activate orbit-system`
- [ ] `python app.py --port 8080`
- [ ] Kỳ vọng: `Server is ready. Press Ctrl+C to stop.`
- [ ] Dừng với Ctrl+C

### [ ] 3.2 Kiểm tra WebSocket

```python
import asyncio
import websockets
async def test():
    async with websockets.connect('ws://localhost:8080') as ws:
        msg = await asyncio.wait_for(ws.recv(), timeout=5)
        print('Received')
asyncio.run(test())
```

- [ ] Chạy backend trong Terminal 1
- [ ] Chạy script test trong Terminal 2
- [ ] Kỳ vọng: nhận được JSON metrics

### [ ] 3.3 Kiểm tra Frontend

- [ ] `npm run build` — build thành công
- [ ] Mở `index.html` trong browser
- [ ] Kỳ vọng: 3D scene hiển thị, mock data chạy

### [ ] 3.4 Kiểm tra Tauri dev mode

- [ ] `npm run dev`
- [ ] Kỳ vọng: Cửa sổ desktop mở, UI hiển thị

---

## Phase 4: 📋 Implement spec còn thiếu

### [ ] 4.1 Viridis/Jet colormap cho 3D Surface

- [ ] Đọc spec: `agent/promts/05_ui_layout_rules.md`
- [ ] Mở `src/threeScene.js`
- [ ] Thêm vertex colors với BufferAttribute
- [ ] Map độ cao Y → Jet hoặc Viridis gradient
- [ ] Kiểm tra: màu sắc thay đổi theo load %

### [ ] 4.2 Z-axis smooth scrolling

- [ ] Đọc spec: `agent/promts/06_interpolation_and_smoothing_rules.md`
- [ ] Mở `src/threeScene.js` + `src/constants.js`
- [ ] Thêm `THREE.Clock`
- [ ] Implement scroll mượt theo deltaTime

### [ ] 4.3 Axis labels + Bounding box

- [ ] Đọc spec: `agent/promts/05_ui_layout_rules.md` (phần MATLAB)
- [ ] Vẽ dotted bounding box với LineDashedMaterial
- [ ] Thêm axis labels

### [ ] 4.4 Vertex coloring (Terrain Heatmap)

- [ ] Đọc spec: `agent/promts/03_threejs_3d_timeseries.md`
- [ ] Update màu trong `updateVertices()`
- [ ] Gradient Viridis từ 0-100%

---

## Phase 5: 🚀 Roadmap features

### [ ] 5.1 Process Manager Cluster
- [ ] Thu thập top processes từ backend
- [ ] Hiển thị 3D Point Cloud
- [ ] K-means clustering

### [ ] 5.2 Global Keyboard Search
- [ ] Backend: file search index
- [ ] Frontend: command palette overlay

### [ ] 5.3 Power Optimization
- [ ] Detect fullscreen/gaming
- [ ] Auto-hạ FPS 60→1
- [ ] Resume khi thoát fullscreen

### [ ] 5.4 Auto-updater
- [ ] Plugin Tauri updater
- [ ] Release workflow GitHub

### [ ] 5.5 Tray icon + auto-start
- [ ] System tray icon
- [ ] Windows auto-start registry

---

## Tiến độ tổng thể

```
Phase 1 — Fix lỗi chặn         [□□□□] 0/4 tasks
Phase 2 — Củng cố nền tảng     [□□□□] 0/4 tasks
Phase 3 — Kiểm thử tích hợp    [□□□□] 0/4 tasks
Phase 4 — Spec còn thiếu       [□□□□] 0/4 tasks
Phase 5 — Roadmap features     [□□□□□] 0/5 tasks
```

---

## Ghi chú

- **Mỗi task nên tạo 1 branch riêng:** `git checkout -b fix/indentation-error`
- **Commit message format:** `<type>: <description>` (feat/fix/docs/chore)
- **Sau mỗi task nhỏ → commit** (không gom quá nhiều)
- **Sau mỗi phase → push** lên GitHub
