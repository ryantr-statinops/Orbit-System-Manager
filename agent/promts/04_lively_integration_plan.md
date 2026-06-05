# Lively Wallpaper Integration & Future Upgrades Plan

## 1. Lively Wallpaper Environment Setup

Lively Wallpaper chạy các trang web bằng một cửa sổ trình duyệt Chromium ẩn (CefSharp hoặc WebView2).

- Tạo một thư mục dự án chứa: `index.html`, `style.css`, `main.js`, và thư viện Three.js bản phân phối cục bộ (không dùng CDN để đảm bảo chạy offline).
- Thêm file cấu hình `LivelyInfo.json` vào thư mục gốc để Lively nhận diện:

```json
{
  "App": "Chromium Web",
  "Title": "3D Statistics System Monitor",
  "Description": "Real-time 3D Topology Dashboard for OS Developers and Applied Statistics Students.",
  "Author": "Trần Quốc Khang",
  "Version": "1.0.0",
  "Contact": ""
}
```

## 2. Deployment Steps

- **Bước 1**: Khởi động Backend Service (Python/Node.js Server) để mở cổng WebSocket.
- **Bước 2**: Mở giao diện Lively Wallpaper -> Nhấn Add Wallpaper -> Chọn thư mục dự án hoặc kéo file `index.html` thả vào ứng dụng.
- **Bước 3**: Kiểm tra log hiển thị để đảm bảo WebView kết nối thành công tới `ws://localhost:8080`.

## 3. Future Upgrade Roadmap (Kế hoạch nâng cấp dài hạn)

Sau khi hoàn thiện biểu đồ chuỗi thời gian 3D cơ bản, hệ thống sẽ tiến hành tích hợp các tính năng nâng cao sau:

- [ ] **Mô-đun Fastfetch CLI Integration**: Cho phép hình nền kích hoạt các tiến trình chạy ngầm thu thập siêu dữ liệu OS và kết xuất ra các chuỗi ký tự hiển thị dạng khối văn bản 3D (3D Text Geometry).
- [ ] **Mô-đun Process Manager Cluster**: Biến các tiến trình đang chiếm dụng tài nguyên cao của Windows Task Manager thành các cụm điểm dữ liệu (Scatter Plot / Point Cloud 3D). Sử dụng thuật toán phân cụm (Clustering) toán học để gom nhóm các tiến trình hệ thống, trình duyệt, hoặc phần mềm đồ họa riêng biệt dựa trên tỷ lệ CPU/RAM tương đối.
- [ ] **Mô-đun Global Custom Keyboard Search**: Tích hợp bộ lắng nghe phím tắt toàn cục (Global Hooks) ở tầng Backend. Khi nhấn tổ hợp phím chỉ định, một thanh công cụ tìm kiếm tối giản (Minimalist Command Palette) sẽ xuất hiện đè lên hình nền Web 3D, cho phép tìm kiếm file hệ thống thông qua thuật toán Indexing cục bộ siêu tốc.
- [ ] **Mô-đun Power Optimization**: Tự động hạ FPS của WebGL từ 60 FPS xuống 1 FPS hoặc đóng băng vòng lặp kết xuất khi phát hiện hệ thống chạy ứng dụng Toàn màn hình (Fullscreen Focus) hoặc khi chơi game đồ họa nặng nhằm giải phóng hoàn toàn bộ nhớ cho card đồ họa NVIDIA Quadro.
