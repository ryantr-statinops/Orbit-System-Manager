# Stage 1 — Collector PoC

## Mục tiêu

Chứng minh collector Go + gopsutil trên Windows, Ubuntu và Fedora; xác định giới hạn GPU/sensor.

## Checklist

- [ ] Lấy inventory OS/version, CPU, RAM, GPU và driver trên máy thử; để người dùng xác nhận.
- [ ] Chốt phiên bản OS x64 và dependency PoC.
- [ ] Tạo CLI Go đọc CPU tổng/từng core, RAM/swap, disk capacity/I/O và network từng interface.
- [ ] Tính rate bằng thời gian thực giữa mẫu; xử lý warming-up, reset counter và mất mẫu.
- [ ] Thử NVIDIA NVML: binding Linux, adapter DLL riêng trên Windows.
- [ ] Khảo sát AMD/Intel qua nguồn OS/driver; Linux dùng DRM/sysfs/hwmon khi có dữ liệu.
- [ ] Kiểm tra sensor Windows; không gán ACPI thermal zone thành nhiệt độ CPU package.
- [ ] Lập bảng OS × thiết bị × metric × nguồn × quyền × kết quả ngay trong file này.
- [ ] Đối chiếu công cụ hệ thống cùng khoảng lấy mẫu; giải thích sai khác.
- [ ] Đề xuất helper Go chỉ khi tăng quyền thực sự mở được metric cần thiết; không thêm C# hoặc driver riêng.

## Điều kiện hoàn thành

Người dùng duyệt độ phủ metric trước khi tiếp tục; nêu rõ metric quan trọng còn thiếu.

## Kết quả và blocker

Chưa thực hiện. Ghi kết quả/blocker và bằng chứng tại đây.
