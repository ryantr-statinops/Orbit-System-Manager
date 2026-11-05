# Kế hoạch triển khai Orbit System Manager

## Stack và phạm vi

Mục tiêu hỗ trợ hiện tại là Windows x64. Stack rebuild: Go + gopsutil v4 + Wails v3; React + TypeScript + Vite, ECharts 2D và Three.js surface. Bản đầu có dashboard ba cột, Settings, metric thật và lịch sử 60 giây trong RAM. Chỉ quảng bá GPU/sensor đã kiểm chứng. Không triển khai Linux/macOS trong phạm vi hiện tại.

## Thứ tự và trạng thái

| Stage | Kế hoạch | Trạng thái |
|---|---|---|
| 1 | [Collector PoC](01-collector-poc.md) | Độ phủ Windows Stage 1A đã được duyệt; giữ nguyên các metric và phần cứng chưa xác minh |
| 2 | [UI và hiệu năng PoC](02-ui-performance-poc.md) | Đang triển khai — browser checks đạt; native WebView2 và baseline còn blocker |
| 3 | [Dashboard dữ liệu thật](03-live-dashboard.md) | Chưa bắt đầu — chờ duyệt độ phủ metric Stage 1 và baseline Stage 2 |
| 4 | [Hoàn thiện trên Windows](04-platform-hardening.md) | Chưa bắt đầu |
| 5 | [Đóng gói và cập nhật](05-packaging-updates.md) | Chưa bắt đầu |
| 6 | [Chuyển đổi và phát hành](06-migration-release.md) | Chưa bắt đầu |

## Quy tắc và gate

- [ ] Người dùng duyệt độ phủ metric Windows sau Stage 1.
- [ ] Người dùng duyệt baseline/ngưỡng hiệu năng Windows sau Stage 2, trước Stage 3.
- [ ] Chỉ đánh dấu checklist khi có bằng chứng; ghi blocker ngay trong stage.
- [ ] Chưa mua Authenticode; gói cập nhật phải có chữ ký được xác minh.
- [ ] Giữ bản Rust đến khi bản Go/Wails được nghiệm thu.

Linux/macOS và lịch sử kiểm chứng cũ không bị xóa, nhưng không thuộc ma trận hỗ trợ hiện tại. Stage 1 được duyệt với phạm vi và giới hạn ghi trong tài liệu stage; không suy rộng sang metric chưa đối chiếu. Stage 2 đã build và review được Wails native fixture, nhưng phép đo native hợp lệ và duyệt baseline còn chờ. Chưa bắt đầu Stage 3 cho đến khi baseline Stage 2 có bằng chứng hợp lệ và được duyệt.

## Tài liệu tham chiếu

- [Documentation index](../../README.md)
- [Architecture](../../ARCHITECTURE.md)
- [Dashboard design](../../UI-DESIGN.md)
- [Development and validation](../../DEVELOPMENT.md)
- [Distribution and updates](../../DISTRIBUTION.md)
