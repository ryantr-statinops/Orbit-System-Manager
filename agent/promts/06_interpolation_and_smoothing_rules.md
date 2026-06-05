# Interpolation & Smoothing Rules for Time-Series Jitter Reduction

## 1. Objective

Giải quyết hiện tượng giật cục (jitter) và chuyển động không mượt mà trong biểu đồ chuỗi thời gian 3D. Vấn đề cốt lõi là sự không khớp giữa tần số cập nhật dữ liệu hệ thống thấp (1Hz) và tần số render màn hình cao (60FPS).

Mục tiêu là tạo ra cảm giác chuyển động mượt mà cho lưới 3D, ngay cả khi dữ liệu đầu vào chỉ thay đổi mỗi giây.

## 2. Problem Analysis: 1Hz Data vs. 60FPS Render

- **1Hz (Input):** Backend gửi dữ liệu mới mỗi giây ($t_0, t_1, t_2...$).
- **60FPS (Output):** Three.js vẽ lại màn hình 60 lần mỗi giây ($f_0, f_1, f_2, ..., f_{59}, f_{60}...$).

**Hiện tượng giật:** Tại khung hình $f_0$ (thời điểm $t_0$), Mesh cập nhật lên độ cao $y_0$. Sau đó, từ $f_1$ đến $f_{59}$, độ cao giữ nguyên. Tại $f_{60}$ (thời điểm $t_1$), độ cao bất ngờ nhảy lên $y_1$, tạo ra một bước nhảy đột ngột mà mắt người cảm nhận là giật.

## 3. Solution: Double Smoothing Approach

Chúng ta sẽ áp dụng cơ chế làm mịn kép, kết hợp hai kỹ thuật:

### 3.1 Y-axis Lerp (Làm mịn độ cao tại chỗ)

Kỹ thuật này sử dụng Nội suy tuyến tính (Linear Interpolation - Lerp) để tính toán độ cao "render" ($y_{\text{render}}$) di chuyển dần từ độ cao hiện tại ($y_{\text{current}}$) tới độ cao mục tiêu ($y_{\text{target}}$) trong mỗi khung hình:

$$y_{\text{render}} = y_{\text{current}} + \alpha \times (y_{\text{target}} - y_{\text{current}})$$

- **$\alpha$ (alpha):** Hệ số làm mịn, nằm trong khoảng 0 đến 1.
  - Giá trị nhỏ (ví dụ: 0.05) $\rightarrow$ chuyển động rất mượt nhưng bám đuổi dữ liệu chậm.
  - Giá trị lớn (ví dụ: 0.2) $\rightarrow$ chuyển động bám đuổi nhanh nhưng ít mượt hơn.
- **Vị trí áp dụng:** Trong vòng lặp `animate()` của Three.js, trước khi render.

### 3.2 Z-axis Smoothed Scrolling (Làm mịn trượt thời gian)

Thay vì cuộn (shift) dữ liệu đột ngột mỗi giây, chúng ta sẽ "trượt" Mesh 3D theo trục Z một cách mượt mà theo thời gian:

1. **Backend:** Gửi thêm một timestamp chính xác (`metric_id` hoặc thời gian Unix) cùng mỗi gói dữ liệu.
2. **Frontend:** Tính toán thời gian trôi qua (`deltaTime`) giữa các khung hình render.
3. **Mượt trượt:** Trong vòng lặp `animate()`, di chuyển nhẹ vị trí trục Z của Mesh dựa vào `deltaTime` và một hệ số tốc độ cuộn cố định.

## 4. Key Rules for Agent Implementation

- **$\alpha$ Tuning:** Agent phải thiết lập một tham số $\alpha$ tùy chỉnh (mặc định là `0.1`) và áp dụng nhất quán trong hàm cập nhật Mesh.
- **Clock Component:** Sử dụng `THREE.Clock` để lấy `deltaTime` chính xác giữa các khung hình.
- **Interpolation Matrix:** Trong hàm `updateGridWithData()`, Agent không được gán trực tiếp dữ liệu mới vào `attributes.position.y`. Thay vào đó, nó phải lưu dữ liệu mới vào một mảng `targetHeights` phụ trợ.
- **Lerp in Animation:** Trong vòng lặp `animate()`, Agent phải duyệt qua tất cả các đỉnh và thực hiện phép Lerp từ `currentHeight` tới `targetHeight`.
- **needsUpdate Flag:** Đảm bảo gọi `geometry.attributes.position.needsUpdate = true` SAU khi đã thực hiện xong các phép Lerp để ép GPU vẽ lại Mesh.

## 5. Visual Proof Matrix

- **Hiện tại:** `TargetHeight(t=1s) = 50%` $\rightarrow$ Nhảy `renderY` từ 0 lên 50.
- **Sau khi áp dụng:** `TargetHeight(t=1s) = 50%`. Tại mỗi khung hình render tiếp theo ($f_1, f_2...$), `renderY` sẽ được tính toán: $0 \rightarrow 5 \rightarrow 9.5 \rightarrow 13.55...$ tiến dần tới 50 một cách mượt mà.
