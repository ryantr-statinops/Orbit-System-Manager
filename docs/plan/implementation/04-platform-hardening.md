# Stage 4 — Hoàn thiện đa nền tảng

## Mục tiêu

Ổn định trên Windows, Ubuntu và Fedora khi quyền hoặc thiết bị thay đổi.

## Checklist

- [ ] Chạy collector trên worker riêng; cô lập lỗi và giới hạn thời gian chờ, không làm nghẽn UI.
- [ ] Inventory lúc khởi động/thay thiết bị; sensor tối đa mỗi 2 giây.
- [ ] Nếu stage 1 chứng minh cần helper, triển khai bằng Go: named pipe Windows, Unix socket Linux, chỉ cho phép đọc metric xác định.
- [ ] App chạy quyền thường; helper tùy chọn, xử lý hủy/từ chối nâng quyền và xác minh client; không mở endpoint mạng.
- [ ] Kiểm tra sleep/resume, reset counter, network đổi interface, thiết bị biến mất và driver không khả dụng.
- [ ] Kiểm tra nhiều GPU và ánh xạ đúng thiết bị.
- [ ] WebGL lỗi có thông báo/chart 2D thay thế; giải phóng tài nguyên khi unmount.
- [ ] Đóng cửa sổ dừng collector/helper; log giới hạn dung lượng.
- [ ] Test Go cho rate/gap/buffer/capability/shutdown; Vitest và Playwright cho trạng thái, Settings và tương tác.
- [ ] Review native/HiDPI/resize trên ba OS; benchmark lại khi cần.

## Điều kiện hoàn thành

Không còn blocker nghiệm thu; quyền/phần cứng thiếu có trạng thái rõ, app thoát sạch và đạt ngưỡng.

## Kết quả và blocker

Chưa thực hiện. Ghi kết quả/blocker và bằng chứng tại đây.
