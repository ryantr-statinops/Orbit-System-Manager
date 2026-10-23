# Task Plan 02 — Implementation Checklist: Pure Rust Native Migration

> **Plan tham chiếu:** `agent/plan/plan_02_rust_native_migration.md`

---

## Phase 1: Rust Telemetry Collector Module

### [x] 1.1 Thêm dependency vào `Cargo.toml`
- [x] Thêm `sysinfo = "0.33"`
- [x] Thêm `serde = { version = "1.0", features = ["derive"] }`
- [x] Thêm `slint = "1.9"` và `slint-build = "1.9"`

### [x] 1.2 Viết Data Structures (`src/telemetry/models.rs`)
- [x] Struct `TelemetryPayload`, `CpuDetail`, `MemoryDetail`, `GpuMetric`, `NetworkSpeed`

### [x] 1.3 Tạo Collector Service (`src/telemetry/collector.rs`)
- [x] Khởi tạo `sysinfo::System` và `Networks`.
- [x] Vòng lặp thu thập 1Hz cập nhật CPU per-core, RAM, I/O mạng.
---

## Phase 2: Cross-Platform GPU Abstraction

### [ ] 2.1 Định nghĩa Trait `GpuProvider`
- [ ] Interface:
  ```rust
  pub trait GpuProvider: Send + Sync {
      fn get_metrics(&mut self) -> Vec<GpuMetric>;
  }
  ```

### [ ] 2.2 Triển khai cho từng OS
- [ ] `#[cfg(target_os = "windows")]`: Thử khởi tạo NVML, nếu không có card rời thì trả về thông số GPU tích hợp hoặc fallback rỗng an toàn.
- [ ] `#[cfg(target_os = "linux")]`: Kiểm tra NVML trước, nếu fail kiểm tra `/sys/class/drm`.
- [ ] `#[cfg(target_os = "macos")]`: Đọc Apple Silicon unified load qua IOKit.

---

## Phase 3: Frontend Cutover (Tauri Event Listener)

### [ ] 3.1 Cài đặt `@tauri-apps/api`
- [ ] `npm install @tauri-apps/api@^2.0.0`

### [ ] 3.2 Cập nhật `src/websocket.js` $\rightarrow$ `src/telemetryClient.js`
- [ ] Kiểm tra xem có đang chạy trong môi trường Tauri không (`window.__TAURI_INTERNALS__`).
- [ ] Nếu trong Tauri:
  ```javascript
  import { listen } from '@tauri-apps/api/event';
  listen('telemetry-data', (event) => {
    this._handleMessage(event.payload);
  });
  ```
- [ ] Nếu chạy trình duyệt thông thường: Tự động chạy `startMockTelemetry()`.

---

## Phase 4: Thu dọn Dependencies & Hoàn tất chuyển dịch Slint

### [x] 4.1 Dọn dẹp Code cũ
- [x] Xoá bỏ `app.py`.
- [x] Xoá `environment.yml` và các ghi chú về Conda / Python.
- [x] Xoá toàn bộ hệ sinh thái Node.js/Web (`package.json`, `style.css`, `index.html`, `rollup.config.js`).

### [x] 4.2 Tích hợp Slint Native GUI (`ui/appwindow.slint`)
- [x] Thiết kế layout 3 cột bằng ngôn ngữ Slint.
- [x] Kết nối dữ liệu `sysinfo` vào Slint Timer 1Hz trong `src/main.rs`.
