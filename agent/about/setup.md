# Rust Environment Setup & Developer Guide

## 📖 Giới thiệu

Dự án **Orbit System Manager (v2.0)** được xây dựng hoàn toàn bằng **Rust** và **Slint GUI Framework**. 
Toàn bộ mã nguồn chạy native, **không cần Python, không cần Node.js, không cần Conda**.

---

## 🔧 Yêu cầu hệ thống

| Công cụ | Phiên bản yêu cầu | Mục đích |
| :--- | :--- | :--- |
| **Rust & Cargo** | $\ge$ 1.77.2 | Trình biên dịch mã nguồn và quản lý crates |
| **C++ Build Tools** | MSVC (Windows) / GCC (Linux) / Xcode CLI (macOS) | Cần thiết để biên dịch Slint GPU backend |
| **Git** | Bất kỳ | Quản lý mã nguồn |

---

## 🚀 Cài đặt môi trường phát triển

### 1. Cài đặt Rust (rustup)
Truy cập trang chủ chính thức: [https://rustup.rs](https://rustup.rs)

* **Windows:** Tải `rustup-init.exe` và chạy cài đặt mặc định (Default `x86_64-pc-windows-msvc`).
* **Linux / macOS:**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  ```

Kiểm tra sau khi cài đặt:
```bash
rustc --version
cargo --version
```

### 2. Cài đặt thư viện phụ thuộc cho Linux (Ubuntu / Debian)
Nếu phát triển trên Linux, cài đặt các thư viện đồ họa để Slint biên dịch:
```bash
sudo apt update && sudo apt install -y \
    build-essential pkg-config libfontconfig1-dev \
    libx11-dev libxkbcommon-dev libwayland-dev \
    libxrandr-dev libxi-dev libgl1-mesa-dev libegl1-mesa-dev
```

### 3. VS Code Extensions khuyên dùng
Để có trải nghiệm phát triển Slint tốt nhất:
* **rust-analyzer**: Hỗ trợ code completion, refactoring, type checking cho Rust.
* **Slint (Slint - GUI for Embedded and Desktop)**: Hỗ trợ syntax highlighting và **Live Preview** trực tiếp cho các file `.slint`.

---

## 💻 Các lệnh Cargo thường dùng

### Chạy ứng dụng ở chế độ Development
```bash
cargo run
```
Slint sẽ tự động biên dịch `ui/appwindow.slint` qua `build.rs` và mở cửa sổ desktop native ngay lập tức.

### Kiểm tra cú pháp nhanh (không sinh binary)
```bash
cargo check
```

### Biên dịch bản phát hành tối ưu (Release Build)
```bash
cargo build --release
```
Tệp thực thi độc lập (standalone executable) sẽ nằm tại:
* Windows: `target/release/orbit-system-manager.exe`
* Linux / macOS: `target/release/orbit-system-manager`

---

## 📁 Cấu trúc file mã nguồn

| File / Thư mục | Vai trò |
| :--- | :--- |
| `Cargo.toml` | Khai báo dependencies (`slint`, `sysinfo`, `serde`) |
| `build.rs` | Script biên dịch Slint UI sang Rust code |
| `src/main.rs` | Entry point, kết nối dữ liệu `sysinfo` vào Slint Model |
| `src/telemetry/` | Module quét CPU, RAM, Network bằng `sysinfo` |
| `ui/appwindow.slint` | Mã nguồn giao diện Slint (3-column layout) |
