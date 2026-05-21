# UI Layout Rules: Positron IDE & Tidyverse Light Aesthetic

## 1. Objective

Tái cấu trúc toàn bộ giao diện Frontend thành **Giao diện nền sáng (Light Theme)**, mô phỏng chính xác không gian làm việc học thuật của **Positron IDE** và ngôn ngữ trực quan hóa dữ liệu của các gói **Tidyverse (ggplot2 / RStudio)**. Loại bỏ hoàn toàn phong cách Cyberpunk/Dark-mode cũ. Đảm bảo bố cục phẳng, sạch sẽ, sử dụng viền mảnh và font chữ Monospace đều tăm tắp, không chồng lấp.

## 2. Color Palette & Typography (Academic Light Theme)

Mọi thành phần CSS trong `style.css` phải tuân thủ nghiêm ngặt bảng màu sáng hệ thống sau:

- **Global Background:** `#f8f9fa` hoặc `#ffffff` (Nền trắng tinh khiết, tạo không gian mở tối đa).
- **Component & Card Background:** `#ffffff` (Trắng) kết hợp đường viền mảnh `1px solid #e5e7eb` (Xám nhạt). Không bo góc (`border-radius: 0px` hoặc tối đa `4px`).
- **Text & Labels:** `#212529` (Đen xám học thuật cho tiêu đề) và `#495057` (Xám cho thông số phụ).
- **Data & Accent Colors (Viridis Continuous Scale):**
  - Mức độ thấp (Low): `#3b528b` (Deep Blue)
  - Mức độ trung bình (Mid): `#21918c` (Teal / Muted Green)
  - Mức độ cao (High): `#fde725` (Muted Yellow / Chartreuse)
- **Typography:** Bắt buộc sử dụng font Monospace hệ thống (`JetBrains Mono`, `Fira Code` hoặc `Consolas`). Toàn bộ nhãn, text, và số liệu thống kê phải căn lề thẳng hàng.

## 3. Interface Grid Layout (3-Column Academic Structure)

Màn hình được chia làm 3 cột dọc chính bằng CSS Grid (`grid-template-columns: 25% 1fr 25%`) với thiết kế phẳng độc lập:

### 3.1 Cột 1: Data Viewer & Logical Processors (~25%)

- **Phía trên (Environment Monitor):** Hiển thị thông số tổng quan của Intel Core i9-9880H: `Avg Utilization (%)`, `Std Dev (sigma)`, `Variance (sigma^2)`. Các thanh Bar-chart ngang hiển thị tải tổng thể sử dụng màu `#21918c` trên nền xám nhạt `#e9ecef`.
- **Phía dưới (Logical Processors):** Danh sách 16 nhân (C00 - C15). Hiển thị dưới dạng các dòng text Monospace kèm thanh tiến trình (Progress Bar) phẳng, thanh mảnh, có tích hợp mini sparkline (đường đồ thị phụ) bên cạnh mỗi nhân để theo dõi xu hướng.
- **Dưới cùng:** Biểu đồ Boxplot phẳng (`Copy Number vs OncobreastLineage`) hiển thị phân phối phân vị thống kê tải của hệ thống.

### 3.2 Cột 2: Core 3D Topology Canvas (Trung tâm - ~50%)

- **Nội dung:** Canvas Three.js hiển thị lưới bề mặt 3D chuỗi thời gian (Time-series Surface/Terrain).
- **Quy chuẩn 3D Render:**
  - Nền Canvas: Phải để màu trắng (`renderer.setClearColor(0xffffff, 1)`).
  - Khung lưới tọa độ (Grid Axes): Hiển thị các đường lưới bao quanh hộp 3D bằng màu xám mảnh (`#dcedc8`).
  - Lưới bề mặt (Mesh Wireframe): Sử dụng các đường Segments cực mảnh. Màu sắc của các đỉnh (Vertices) phải được ánh xạ theo độ cao (Trục Y) dựa trên bảng màu `Viridis` (Chuyển dần từ xanh thẫm sang xanh lá và điểm đỉnh màu vàng).

### 3.3 Cột 3: Resource Telemetry & Distribution (~25%)

- **Thẻ 1: System Memory (RAM):** Hiển thị dung lượng dạng phân tích `14.4 / 31.8 GB (45%)`. Tích hợp một biểu đồ Boxplot nằm ngang ngay bên dưới để biểu diễn phân vị sử dụng bộ nhớ theo thời gian.
- **Thẻ 2: GPU Telemetry:** Hiển thị thông số song song của `GPU 0 (NVIDIA Quadro T1000)` và `GPU 1 (Intel UHD)`. Phía dưới là một biểu đồ mật độ dạng làm mịn (Density Plot / Kernel Density Estimation) màu Teal nhạt đổ bóng diện tích để đo lường tần suất biến động của GPU.
- **Thẻ 3: Network Speeds:** Hiển thị lưu lượng `SEND` và `RECEIVE` (Kbps) thời gian thực dưới dạng biểu đồ đường (Line Chart) toán học, nét vẽ mảnh `1px`, không đổ màu phân vùng dưới đường vẽ.

## 4. Execution & Updating Rules

- **Tần suất cập nhật:** Luồng WebSocket từ Python Backend truyền dữ liệu 1Hz.
- **Cơ chế nội suy:** Luồng Render 60FPS của Frontend sử dụng thuật toán nội suy Lerp để làm mượt các đỉnh của lưới 3D trung tâm và tiến trình co giãn của các biểu đồ mật độ, Boxplot ở hai bên cột, đảm bảo toàn bộ giao diện nền sáng chuyển động mượt mà như một ứng dụng phân tích thời gian thực.

# UI Layout Rules: MATLAB-Style 3D Surface Mathematical Aesthetic

## 1. Objective
Chỉnh sửa toàn diện Canvas 3D trung tâm để mô phỏng chính xác 100% biểu đồ toán học bề mặt phẳng (Solid Surface with Mesh Overlay) theo phong cách đồ thị hàm số MATLAB. Loại bỏ hoàn toàn hiệu ứng low-poly wireframe trong suốt hiện tại.

## 2. 3D Mesh & Shading Specifications
- **Surface Material:** Thay thế `Wireframe: true` bằng vật liệu đặc (Opaque Surface). Sử dụng `THREE.MeshLambertMaterial` hoặc `THREE.MeshBasicMaterial` với thuộc tính `side: THREE.DoubleSide` để đổ màu khối đặc.
- **Wireframe Overlay:** Tạo một bản sao lưới thứ hai chồng lên trên (hoặc dùng `THREE.LineSegments` kết hợp với `THREE.EdgesGeometry`) để vẽ các đường lưới vuông góc bao bọc bề mặt. Đường lưới này bắt buộc phải có màu đen mảnh (`color: #000000`, `linewidth: 1`) và phải bị che khuất tự nhiên bởi các đỉnh lồi phía trước (Depth Test kích hoạt).

## 3. Jet/Rainbow Color Shader Matrix
Thuật toán tô màu đỉnh (Vertex Coloring) dựa trên cao độ $Y$ trong vòng lặp phải ánh xạ chính xác theo phổ màu Jet-Scale:
- $Y_{\text{min}}$ (Thung lũng lõm sâu): Màu xanh lam đậm (`#00008F`).
- $Y_{\text{mid-low}}$: Màu xanh dương sáng / Cyan (`#00FFFF`).
- $Y_{\text{mid}}$ (Mặt phẳng nền): Màu xanh lá cây (`#00FF00`).
- $Y_{\text{mid-high}}$: Màu vàng chanh (`#FFFF00`).
- $Y_{\text{max}}$ (Đỉnh lồi cao): Màu đỏ rực (`#FF0000`).

## 4. Camera & Bounding Box Blueprint
- **Camera Aspect:** Điều chỉnh góc Camera bám sát tỷ lệ Isometric trực giao. Đặt `camera.position` ở góc cao nghiêng góc tầm $45^{\circ}$ nhìn xuống tâm diện tích của Mesh.
- **Dotted Bounding Box:** Dựng một khung hộp lập phương bao quanh mô hình bằng nét đứt hoặc nét chấm mảnh xám (`#6c757d`). 
- **Axes Labels:** Hiển thị 3 trục tọa độ vuông góc có các vạch chia số Monospace màu đen rõ ràng tại các cạnh biên của hộp lập phương phía sau.