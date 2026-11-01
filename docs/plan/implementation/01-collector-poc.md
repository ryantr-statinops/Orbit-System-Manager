# Stage 1 — Collector PoC

## Mục tiêu

Kiểm chứng Go + gopsutil trên Windows, Ubuntu và Fedora, gồm giới hạn GPU/sensor. Đợt này chỉ gồm Windows Stage 1A.

## Checklist

- [x] Inventory Windows được người dùng xác nhận; khóa Go 1.27.0 và gopsutil v4.26.10.
- [x] Windows CLI thu CPU/RAM/swap/disk/network/NVIDIA; rate và reset có unit test.
- [x] Thử sensor ACPI; không gán nhầm thành nhiệt độ CPU package.
- [x] Ghi ma trận Windows bên dưới.
- [ ] Ubuntu/Fedora inventory, Linux NVIDIA/DRM/sysfs/hwmon và metric AMD/Intel.
- [ ] Đối chiếu đầy đủ; chỉ xét helper nếu quyền cao hơn mở metric cần thiết.

## Kết quả Windows — 2026-11-01

Người dùng xác nhận Windows 11 Pro 26H2 (26300.9711), Intel Core i9-9880H (8/16 core), RAM 34,099,601,408 byte. Go `1.27.0 windows/amd64`, `CGO_ENABLED=0`; gopsutil `v4.26.10`.

| Metric | Kết quả Windows |
|---|---|
| CPU, RAM, swap | gopsutil sample `available` sau baseline; swap dùng page-file fallback. |
| Disk, network | Capacity, I/O và rate/interface `available`; chưa đối chiếu độc lập. |
| GPU | Quadro T1000 / 580.92 qua NVML. `nvidia-smi` gần cùng lúc khớp utilization, nhiệt độ, power; VRAM lệch cao hơn 159 MiB, chưa rõ nguyên nhân. Intel UHD 630 / driver 31.0.101.2137 từng được WMI phát hiện; chưa có adapter metric. |
| Sensor | ACPI trả `permission_denied`; không có số đọc. |

Windows Performance Counter/CIM chặn đối chiếu CPU/RAM/disk/network. Lần inventory mới cũng bị WMI từ chối đọc CPU/GPU; không xem đây là phần cứng không hỗ trợ. Raw JSON/JSONL ở `artifacts/collector-poc/` (Git ignore).

## Trạng thái và gate

Theo yêu cầu người dùng ngày 2026-11-01, Stage 1 được đánh dấu hoàn thành tạm với bằng chứng hiện có giới hạn ở Windows Stage 1A. Ubuntu/Fedora inventory/adapters và đối chiếu metric vẫn còn chờ; checklist tương ứng giữ chưa đánh dấu. Gate duyệt độ phủ metric vẫn cần được xác nhận riêng trước khi bắt đầu Stage 2. Không suy rộng kết quả Windows sang phần cứng chưa thử.
