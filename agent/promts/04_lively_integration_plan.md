# Desktop App Integration & Future Roadmap (Tauri v2)

## 1. Architecture Overview

Dự án đã chuyển từ Lively Wallpaper (WebView-based wallpaper) sang **Tauri v2** — một desktop native shell sử dụng Rust làm backbone. Quyết định này mang lại:

- **Hiệu năng cao hơn**: Rust runtime với footprint cực thấp, không phụ thuộc Chromium ẩn.
- **Bundle độc lập**: Tạo file `.msi` / `.exe` installer, user không cần cài Python, Node.js, hay Lively.
- **Sidecar support**: Python backend (`app.py`) được đóng gói thành binary standalone và chạy như một tiến trình con (sidecar) khi app khởi động.

## 2. Tauri Project Structure

```
src-tauri/
├── Cargo.toml          # Rust dependencies (tauri v2, tauri-plugin-shell, tauri-plugin-log)
├── tauri.conf.json      # Window config, bundle targets (msi/nsis), external binaries
├── src/
│   ├── main.rs          # Entry point (hides console in release mode)
│   └── lib.rs           # Core logic: spawn Python sidecar, manage lifecycle
├── binaries/            # Python backend compiled binary (auto-bundled)
└── icons/               # App icons (ico, icns, png)
```

## 3. Python Backend Sidecar Lifecycle

```rust
// Trong src-tauri/src/lib.rs
app.shell()
    .sidecar("app")                                                    // Tìm binary trong src-tauri/binaries/
    .ok()
    .and_then(|cmd| {
        cmd.args(["--port", "8080", "--quiet"])
           .spawn()
           .ok()
    });
```

- **Khi app mở**: Rust code tự động spawn Python sidecar với args `--port 8080 --quiet`.
- **Khi app đóng**: Rust code bắt sự kiện `CloseRequested`, gọi `child.kill()` để dọn dẹp tiến trình.
- **Nếu sidecar không có**: Frontend tự động fallback sang mock telemetry (dữ liệu giả lập).

## 4. Build & Deployment

```bash
# 1. Cài dependencies
npm install                   # Node.js (Three.js, Rollup, Tauri CLI)
conda activate orbit-system   # Python environment
pip install -r requirements.txt
pip install pyinstaller        # Để đóng gói app.py thành binary

# 2. Build Python backend thành binary standalone
pyinstaller --onefile --name app --distpath src-tauri/binaries app.py

# 3. Build frontend
npm run build

# 4. Build desktop app (ra .msi / .exe)
npm run build:app
```

File cài đặt sau build sẽ nằm tại:
| Định dạng | Đường dẫn |
|-----------|-----------|
| 📦 MSI | `src-tauri/target/release/bundle/msi/` |
| 📦 NSIS | `src-tauri/target/release/bundle/nsis/` |

## 5. Development Workflow

```bash
# Terminal 1: Python backend
conda activate orbit-system
python app.py --port 8080

# Terminal 2: Frontend dev server
npm run start          # Rollup watch → dist/bundle.js

# Terminal 3 (optional): Tauri desktop
npm run dev            # Mở cửa sổ desktop, load frontend từ localhost:3000
```

> Khi chạy `npm run dev`, Tauri tự động:
> 1. Chạy `node scripts/dev-server.mjs` (static server tại port 3000)
> 2. Mở WebView window trỏ tới `http://localhost:3000`
> 3. Frontend kết nối WebSocket tới Python backend

## 6. Future Upgrade Roadmap

- [x] **✅ Tauri v2 Desktop Shell**: Chuyển từ Lively Wallpaper → desktop native app
- [ ] **Mô-đun Process Manager Cluster**: Biến top processes đang chiếm dụng tài nguyên thành Point Cloud 3D, dùng thuật toán phân cụm (K-means/DBSCAN).
- [ ] **Mô-đun Global Keyboard Search**: Tích hợp thanh Command Palette tìm kiếm file/tiến trình hệ thống.
- [ ] **Mô-đun Power Optimization**: Tự động hạ FPS WebGL (60 → 1 FPS) khi phát hiện fullscreen app / gaming.
- [ ] **Integrations**: Auto-start with Windows, tray icon, dark/light theme toggle.
- [ ] **Auto-updater**: Sử dụng `@tauri-apps/plugin-updater` để tự động cập nhật phiên bản mới.
