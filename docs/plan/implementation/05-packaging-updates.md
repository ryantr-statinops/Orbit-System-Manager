# Stage 5 — Đóng gói và cập nhật

## Mục tiêu

Installer Windows x64; cập nhật qua GitHub Releases, hỏi trước khi tải/cài. Linux packages nằm ngoài phạm vi hiện tại.

## Checklist

- [ ] CI tạo Windows x64 NSIS installer; khai báo dependency/runtime cần thiết.
- [ ] Kiểm tra cài mới, chạy, gỡ cài đặt và nâng cấp trên môi trường sạch.
- [ ] Dùng một kênh stable; tạo manifest chỉ đúng OS, kiến trúc và loại package.
- [ ] Ký gói cập nhật, gắn public key vào app; giữ private key ngoài repo trong CI secret được bảo vệ.
- [ ] Từ chối gói thiếu/sai chữ ký hoặc checksum; không hạ xuống chế độ không xác minh.
- [ ] Kiểm tra sau startup, mỗi 24 giờ; có kiểm tra thủ công/tắt tự kiểm tra.
- [ ] Hiển thị phiên bản, notes, dung lượng/tiến độ; tải/cài sau đồng ý.
- [ ] Windows chạy installer đã xác minh; không tự thay thế executable khi cài thất bại.
- [ ] Kiểm tra mất mạng, gói hỏng, sai nền tảng, hủy quyền và cài thất bại; cung cấp hướng dẫn khôi phục bằng installer.
- [ ] Giữ Settings; hướng dẫn Windows ghi rõ chưa ký Authenticode.

## Điều kiện hoàn thành

Ba gói cài/nâng cấp được kiểm chứng; không báo thành công giả hoặc cam kết rollback chưa kiểm chứng.

## Kết quả và blocker

Chưa thực hiện. Ghi kết quả/blocker và bằng chứng tại đây.
