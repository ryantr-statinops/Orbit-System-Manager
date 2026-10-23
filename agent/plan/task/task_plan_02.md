# Task Plan 02 — Implementation Checklist: Pure Rust Native Migration

> **Plan tham chiếu:** `agent/plan/plan_02_rust_native_migration.md`

---

## Phase 1: Rust Telemetry Collector Module

### [ ] 1.1 Thêm dependency vào `src-tauri/Cargo.toml`
- [ ] Thêm `sysinfo = "0.33"`
- [ ] Thêm `tokio = { version = "1", features = ["sync", "time"] }`
- [ ] Thêm `serde = { version = "1.0", features = ["derive"] }`

### [ ] 1.2 Viết Data Structures (`src-tauri/src/telemetry/models.rs`)
- [ ] Struct `TelemetryPayload`:
  ```rust
  #[derive(serde::Serialize, Clone, Debug)]
  pub struct TelemetryPayload {
      pub cpu_percent: f32,
      pub cpu_detail: CpuDetail,
      pub memory_percent: f32,
      pub memory_used_gb: f32,
      pub memory_total_gb: f32,
      pub network: NetworkDetail,
      pub gpu_detail: Vec<GpuMetric>,
  }
  ```

### [ ] 1.3 Tạo Collector Service (`src-tauri/src/telemetry/collector.rs`)
- [ ] Khởi tạo `sysinfo::System`.
- [ ] Vòng lặp thu thập 1Hz (dùng `tokio::time::interval(Duration::from_secs(1))`).
- [ ] Gọi `app_handle.emit("telemetry-data", &payload)`.

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

## Phase 4: Thu dọn Dependencies & Build

### [ ] 4.1 Dọn dẹp Code cũ
- [ ] Xoá bỏ `app.py`.
- [ ] Xoá `environment.yml` và các ghi chú về Conda / Python.
- [ ] Xoá `src-tauri/binaries/` khỏi `tauri.conf.json` (`externalBin`).

### [ ] 4.2 Kiểm thử Build Đa nền tảng
- [ ] Chạy local: `npm run build && cargo check --manifest-path src-tauri/Cargo.toml`.
- [ ] Kích hoạt GitHub Actions test build trên Ubuntu, Windows và macOS.
