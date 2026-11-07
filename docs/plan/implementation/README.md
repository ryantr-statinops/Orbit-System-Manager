# Kế hoạch triển khai Orbit System Manager

## Stack và phạm vi

Mục tiêu hỗ trợ hiện tại là Windows x64. Stack rebuild: Go + gopsutil v4 + Wails v3; React + TypeScript + Vite, ECharts 2D và Three.js surface. Bản đầu có dashboard ba cột, Settings, metric thật và lịch sử 60 giây trong RAM. Chỉ quảng bá GPU/sensor đã kiểm chứng. Không triển khai Linux/macOS trong phạm vi hiện tại.

## Thứ tự và trạng thái

| Stage | Kế hoạch | Trạng thái |
|---|---|---|
| 1 | [Collector PoC](01-collector-poc.md) | Độ phủ Windows Stage 1A đã được duyệt; giữ nguyên các metric và phần cứng chưa xác minh |
| 2 | [UI và hiệu năng PoC](02-ui-performance-poc.md) | Baseline đã được duyệt; chờ đính kèm bằng chứng và hoàn tất/review layout C |
| 3 | [Dashboard dữ liệu thật](03-live-dashboard.md) | Chưa bắt đầu — chờ artifact baseline được ghi nhận và duyệt UI C cuối |
| 4 | [Hoàn thiện trên Windows](04-platform-hardening.md) | Chưa bắt đầu |
| 5 | [Đóng gói và cập nhật](05-packaging-updates.md) | Chưa bắt đầu |
| 6 | [Chuyển đổi và phát hành](06-migration-release.md) | Chưa bắt đầu |

## Quy tắc và gate

- [x] Người dùng duyệt độ phủ metric Windows Stage 1A với các giới hạn đã ghi.
- [x] Người dùng duyệt baseline/ngưỡng hiệu năng Windows ngày 2026-11-07; artifact bằng chứng chờ đính kèm.
- [ ] Chỉ đánh dấu checklist khi có bằng chứng; ghi blocker ngay trong stage.
- [ ] Chưa mua Authenticode; gói cập nhật phải có chữ ký được xác minh.
- [ ] Giữ bản Rust đến khi bản Go/Wails được nghiệm thu.

Linux/macOS và lịch sử kiểm chứng cũ không bị xóa, nhưng không thuộc ma trận hỗ trợ hiện tại. Stage 1 được duyệt cho phạm vi Windows Stage 1A; các metric chưa đối chiếu hoặc sai khác VRAM vẫn bị giới hạn như ghi trong stage file. Người dùng đã duyệt baseline Stage 2; artifact vẫn chờ đính kèm vào repository. Stage 3 chưa bắt đầu cho đến khi bằng chứng được ghi nhận và layout C được review cuối.

## Tài liệu tham chiếu

- [Documentation index](../../README.md)
- [Architecture](../../ARCHITECTURE.md)
- [Dashboard design](../../UI-DESIGN.md)
- [Development and validation](../../DEVELOPMENT.md)
- [Distribution and updates](../../DISTRIBUTION.md)
