# Task Manager 3D Visualize (Positron & Tidyverse Aesthetic)

Hệ thống giám sát hiệu năng CPU (16 nhân Intel Core i9-9880H), RAM (32GB) và GPU (NVIDIA Quadro T1000) theo thời gian thực. Giao diện được thiết kế 100% nền sáng (Light Theme) theo phong cách học thuật của Positron IDE, các gói Tidyverse (ggplot2) và đồ thị toán học MATLAB.

## 🚀 Tính năng cốt lõi

- **Backend (Python CLI):** Thu thập dữ liệu hệ thống bất đồng bộ qua `asyncio`, `psutil` và `GPUtil`. Phân phối luồng qua WebSocket Server (Port 8080).
- **Frontend (Three.js Web Canvas):** Render lưới bề mặt 3D Topology chuỗi thời gian (Time-series Surface) sử dụng bộ màu Jet/Rainbow tương phản cao.
- **Dữ liệu Thống kê:** Tính toán trực tiếp Trung bình ($\mu$), Độ lệch chuẩn ($\sigma$) và Phương sai ($\sigma^2$) của các nhân CPU.
- **Làm mịn đồ họa:** Áp dụng thuật toán nội suy Lerp kép để triệt tiêu hiện tượng giật cục (jitter), cho chuyển động 60FPS mượt mà từ nguồn dữ liệu 1Hz.

## 🛠️ Khởi chạy nhanh

1. **Khởi động Backend:**

   ```bash
   pip install -r requirements.txt
   python app.py
   ```

2. **Khởi chạy Frontend:** Mở file index.html bằng trình duyệt hoặc nạp cả thư mục Frontend vào Lively Wallpaper để làm hình nền động.
