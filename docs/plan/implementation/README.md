# Kế hoạch triển khai Orbit System Manager

## Stack và phạm vi

Go + gopsutil v4 + Wails v3; React + TypeScript + Vite, ECharts 2D và Three.js surface. Khóa dependency sau PoC.

Bản đầu: Windows và Ubuntu/Fedora x64; dashboard ba cột, Settings, metric thật, lịch sử 60 giây trong RAM. GPU/sensor theo khả năng kiểm chứng; đóng cửa sổ thoát hoàn toàn. Chưa thêm macOS, process explorer, cảnh báo, export, tray hoặc monitoring từ xa.

## Thứ tự và trạng thái

| Stage | Kế hoạch | Trạng thái |
|---|---|---|
| 1 | [Collector PoC](01-collector-poc.md) | Chưa bắt đầu |
| 2 | [UI và hiệu năng PoC](02-ui-performance-poc.md) | Chưa bắt đầu |
| 3 | [Dashboard dữ liệu thật](03-live-dashboard.md) | Chưa bắt đầu |
| 4 | [Hoàn thiện đa nền tảng](04-platform-hardening.md) | Chưa bắt đầu |
| 5 | [Đóng gói và cập nhật](05-packaging-updates.md) | Chưa bắt đầu |
| 6 | [Chuyển đổi và phát hành](06-migration-release.md) | Chưa bắt đầu |

## Quy tắc thực hiện

- [ ] Người dùng duyệt độ phủ metric sau stage 1.
- [ ] Người dùng duyệt baseline và ngưỡng hiệu năng sau stage 2, trước stage 3.
- [ ] Đánh dấu khi có bằng chứng; ghi kết quả/blocker trong stage.
- [ ] Chưa mua Authenticode; bắt buộc ký gói cập nhật.
- [ ] Giữ bản Rust để đối chiếu đến khi bản mới được nghiệm thu.

Chưa triển khai hoặc kiểm chứng.
