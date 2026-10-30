# Stage 2 — UI và hiệu năng PoC

## Mục tiêu

Xác minh surface và đo baseline trên Windows, Ubuntu, Fedora trước khi xây dashboard.

## Checklist

- [ ] Dựng Wails v3 + React + TypeScript + Vite; khóa dependency tương thích và runtime frontend tương ứng.
- [ ] Thiết lập browser preview bằng fixture deterministic, không phụ thuộc collector.
- [ ] Dùng ECharts theo module cho 2D; Three.js trực tiếp cho surface, camera và vòng render.
- [ ] Tạo fixture 0/1/4/16/32/64 core, lịch sử 60 giây; có trạng thái empty, single và unavailable.
- [ ] Tái sử dụng buffer mesh; tách cập nhật metric khỏi render, tránh React state cho từng frame.
- [ ] Kiểm tra rotate/pan/zoom/reset; giới hạn pixel ratio và giảm render khi cửa sổ bị ẩn.
- [ ] Xác minh GTK/WebKitGTK/WebView từng OS; ghi dependency và lỗi rendering.
- [ ] Đo bản release: thời gian đến UI/mẫu đầu, RAM cả cây tiến trình, CPU toàn máy và frame time p95 khi tương tác.
- [ ] Chạy idle và tương tác 30 phút; ghi cấu hình, scaling, độ phân giải và phương pháp đo.
- [ ] Người dùng duyệt baseline/ngưỡng trước stage 3.

## Điều kiện hoàn thành

PoC chạy native trên ba môi trường, benchmark tái lập; giải quyết vấn đề stack trước stage 3.

## Kết quả và blocker

Chưa thực hiện. Ghi ngưỡng đã duyệt và bằng chứng tại đây.
