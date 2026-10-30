# Stage 3 — Dashboard dữ liệu thật

## Mục tiêu

Xây dashboard metric thật, giữ ba cột và màu tối; bắt đầu sau duyệt PoC.

## Checklist

- [ ] Go giữ buffer 60 giây; gửi mẫu mới qua Wails và cung cấp snapshot khi frontend đồng bộ lại.
- [ ] Metric có timestamp, đơn vị, nguồn, nullable và trạng thái availability/lỗi/quyền/stale.
- [ ] Chuẩn hóa byte và byte/giây trong backend; chuyển đơn vị ở UI, không thay dữ liệu thiếu bằng số 0.
- [ ] Giữ CPU/core/history bên trái, surface giữa, memory/GPU/network bên phải; thêm disk card thu gọn.
- [ ] Surface thể hiện từng core theo thời gian; nội suy không sinh mẫu telemetry mới.
- [ ] Camera, tooltip, chọn core/reset chạy độc lập collector.
- [ ] Pause đóng băng phần đang xem, collector tiếp tục; Resume chuyển về dữ liệu hiện tại.
- [ ] Thêm Settings: interval 1/2/5 giây, GPU/disk/interface và helper nếu đã được chứng minh cần thiết.
- [ ] Lưu Settings JSON có phiên bản; lịch sử chỉ trong RAM.
- [ ] Review 720×520, 1060×800, 1280×800, 1465×955; kiểm tra nhãn/dữ liệu thiếu.
- [ ] Demo có nhãn; desktop mặc định live.

## Điều kiện hoàn thành

Live data, Settings và trạng thái thiếu dữ liệu hoạt động; đạt ngưỡng stage 2, không tạo lịch sử giả.

## Kết quả và blocker

Chưa thực hiện. Ghi kết quả/blocker và bằng chứng tại đây.
