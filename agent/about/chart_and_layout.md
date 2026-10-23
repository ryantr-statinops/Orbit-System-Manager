# Slint UI Layout & Component Architecture

Tài liệu này mô tả cấu trúc giao diện native của **Orbit System Manager** được định nghĩa bằng ngôn ngữ khai báo **Slint** trong `ui/appwindow.slint`.

---

## 🎨 Bố cục giao diện 3 cột (3-Column Layout)

Ứng dụng sử dụng bố cục 3 cột hiện đại được điều phối qua `HorizontalLayout` với kích thước cửa sổ tối thiểu `1024 × 720` và kích thước ưu tiên `1280 × 800`.

```
┌────────────────────────────────────────────────────────────────────────┐
│  🛸 ORBIT SYSTEM MANAGER          ● NATIVE SLINT ENGINE   1 HZ SAMPLING │ (Top Bar)
├───────────────┬────────────────────────────────────────┬───────────────┤
│  LEFT COLUMN  │             CENTER COLUMN              │ RIGHT COLUMN  │
│    (310px)    │                (Flex: 1)               │    (290px)    │
│               │                                        │               │
│ - RAM Track   │ - 3D Topology Viewport Container       │ - GPU Card    │
│ - 16 CoreBars │ - Max/Idle Core Summary Bar            │ - GPU Load Bar│
│ - HUD Stats   │ - Load Distribution Indicators         │ - Net Speeds  │
└───────────────┴────────────────────────────────────────┴───────────────┘
```

---

## 🧩 Danh sách Component (`ui/appwindow.slint`)

### 1. `CoreBar` (Thanh tải từng nhân CPU)
* **Thuộc tính đầu vào:**
  * `in property <int> core_id`: Chỉ số nhân (0 $\rightarrow$ 15).
  * `in property <float> load`: Mức tải thực tế từ 0.0 đến 100.0%.
* **Đặc tính đồ hoạ:**
  * Chiều cao: `22px`, bo góc `4px`, viền `#dee2e6`.
  * Màu sắc động:
    * Load $> 85\%$: Màu đỏ (`#fa5252`).
    * Load $> 55\%$: Màu vàng cam (`#fab005`).
    * Load $\le 55\%$: Màu xanh Viridis teal (`#21918c`).
  * Animation: `animate width { duration: 300ms; easing: ease-out; }` tạo hiệu ứng co giãn mượt mà khi mức tải thay đổi.

### 2. `StatCard` (Thẻ hiển thị thông số HUD)
* **Thuộc tính đầu vào:**
  * `in property <string> title`: Tiêu đề nhỏ viết hoa (ví dụ: `AVG UTIL`, `STD DEV`).
  * `in property <string> value`: Giá trị số nổi bật font size `18px`, đậm `700`.
  * `in property <string> subtitle`: Thông tin phụ (tùy chọn).
* **Đặc tính đồ hoạ:** Nền trắng `#ffffff`, bo góc `6px`, đệm `10px`, viền `#e9ecef`.

---

## 🔗 Liên kết dữ liệu giữa Rust và Slint (Data Binding)

Các thuộc tính trên `AppWindow` được Rust cập nhật định kỳ mỗi 1 giây qua timer trong `src/main.rs`:

| Thuộc tính Slint | Kiểu dữ liệu | Nguồn từ Rust `TelemetryPayload` |
| :--- | :--- | :--- |
| `cpu_avg` | `float` | `payload.cpu_detail.avg_utilization` |
| `cpu_std` | `float` | `payload.cpu_detail.std_dev` |
| `cpu_var` | `float` | `payload.cpu_detail.variance` |
| `cores` | `[CoreItem]` | `payload.cpu_detail.cores` (16 phần tử `CoreItem { id, load }`) |
| `ram_percent` | `float` | `payload.memory_detail.percent` |
| `ram_used_gb` | `float` | `payload.memory_detail.used_gb` |
| `ram_total_gb` | `float` | `payload.memory_detail.total_gb` |
| `net_sent_kbps` | `float` | `payload.network_speed.sent_kbps` |
| `net_recv_kbps` | `float` | `payload.network_speed.received_kbps` |
| `gpu_load` | `float` | `payload.gpu_detail[0].load_percent` |
| `gpu_temp` | `float` | `payload.gpu_detail[0].temperature_c` |
| `gpu_name` | `string` | `payload.gpu_detail[0].name` |
