# Kế hoạch triển khai Orbit System Manager

## Stack và phạm vi

Go + gopsutil v4 + Wails v3; React + TypeScript + Vite, ECharts 2D và Three.js surface. Khóa dependency sau PoC.

Bản đầu: Windows và Ubuntu/Fedora x64; dashboard ba cột, Settings, metric thật, lịch sử 60 giây trong RAM. GPU/sensor theo khả năng kiểm chứng; đóng cửa sổ thoát hoàn toàn. Chưa thêm macOS, process explorer, cảnh báo, export, tray hoặc monitoring từ xa.

## Thứ tự và trạng thái

| Stage | Kế hoạch | Trạng thái |
|---|---|---|
| 1 | [Collector PoC](01-collector-poc.md) | Hoàn thành tạm theo yêu cầu người dùng — Windows Stage 1A; Linux còn chờ |
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

Stage 1 được đánh dấu hoàn thành tạm; Linux, đối chiếu đầy đủ và duyệt độ phủ metric còn chờ. Stage 2 chưa bắt đầu.

## Reference guides

- [Documentation index](../../README.md): current application, rebuild requirements, and historical material.
- [Architecture](../../ARCHITECTURE.md): collector and platform hardening (stages 1, 4).
- [Web dashboard](../../UI-DESIGN.md): UI PoC and live dashboard (stages 2, 3).
- [Development and validation](../../DEVELOPMENT.md): environments, tests, baseline, and approval gates.
- [Distribution and updates](../../DISTRIBUTION.md): packaging, updater, and release acceptance (stages 5, 6).
