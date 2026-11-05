# Stage 1 — Collector PoC

## Mục tiêu

Kiểm chứng Go + gopsutil cho Windows x64, gồm giới hạn GPU/sensor. Phạm vi hiện tại chỉ gồm Windows Stage 1A.

## Checklist

- [x] Inventory Windows được người dùng xác nhận; khóa Go 1.27.0 và gopsutil v4.26.10.
- [x] Windows CLI thu CPU/RAM/swap/disk/network/NVIDIA; rate và reset có unit test.
- [x] Thử sensor ACPI; không gán nhầm thành nhiệt độ CPU package.
- [x] Ghi ma trận Windows bên dưới.
- [ ] Linux inventory/adapters được hoãn ngoài phạm vi Windows-only hiện tại; không chặn nghiệm thu Windows.
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

Theo yêu cầu người dùng ngày 2026-11-01, Stage 1 được đánh dấu hoàn thành tạm với bằng chứng giới hạn ở Windows Stage 1A. Linux inventory/adapters nằm ngoài phạm vi hỗ trợ hiện tại và chưa được kiểm chứng; không suy rộng kết quả sang phần cứng Windows chưa thử. Gate duyệt độ phủ metric Windows vẫn cần được xác nhận riêng trước Stage 3.

## Kiểm chứng Windows bổ sung — 2026-11-04

- `go test ./...`, `go vet ./...`, và build CLI thành công sau khi sửa sampler UI benchmark.
- Inventory mới đọc Windows 11 Pro 26H2 build `26300.9818` và RAM `34,099,601,408` byte; CPU inventory trả `permission_denied`, WMI GPU discovery cũng bị từ chối. Không có CPU model mới được xác nhận trong lần này.
- Ba mẫu Go cho thấy CPU `warming_up` ở mẫu đầu rồi `available`; memory/swap/disk/network và NVIDIA NVML đều `available`. Sensor ACPI vẫn `permission_denied`.
- NVIDIA được đọc qua NVML; `nvidia-smi` chạy được, nhưng chưa thu cùng thời điểm/cửa sổ để xác nhận sai khác VRAM cũ. `Get-Counter` và CIM đều bị quyền hệ thống từ chối nên chưa đối chiếu độc lập được CPU/RAM/disk/network.

Gate duyệt độ phủ metric vẫn chờ. Không quảng bá CPU inventory, sensor ACPI, Intel GPU metrics hoặc metric chưa đối chiếu như đã được xác minh.

## Đối chiếu Windows bổ sung — 2026-11-05

- `go test ./...`, `go vet ./...`, build CLI và build Wails đều thoát thành công. Windows Application Control chặn chạy trực tiếp các file `.exe` vừa build; chạy collector bằng `go run` vẫn hoạt động. Cảnh báo ghi Go module stat-cache bị từ chối không làm lệnh build thất bại.
- Inventory tiếp tục trả CPU model `null` do SMBIOS access denied; RAM tổng `34,099,601,408` byte và Quadro T1000 / driver `580.92` được đọc. CIM/WMI bị từ chối.
- Sample 1 giây: CPU warming-up ở mẫu đầu, sau đó `available` (khoảng 5.5–13.4% trong các mẫu ngắn); RAM, swap, disk capacity/I/O, network và NVIDIA đều có dữ liệu. ACPI sensor vẫn `permission_denied`.
- NVML và `nvidia-smi` được lấy trong cùng khoảng thời gian: utilization `0%`, nhiệt độ khoảng `57–59°C`, power khoảng `4.7–4.9 W`, VRAM tổng `4096 MiB`. VRAM used không khớp: NVML khoảng `319.3 MiB`, `nvidia-smi` `161 MiB` (lệch khoảng `158.3 MiB`); VRAM used chưa được xác minh và không nên quảng bá.
- `Get-CimInstance` bị từ chối. `typeperf` không chạy vì tài khoản không thuộc Performance Log Users và không elevated; Task Manager không mở được qua phiên điều khiển này. Vì vậy CPU/RAM used/disk/network chưa có đối chiếu độc lập. Không thay đổi quyền hệ thống.

Raw inventory/sample/NVIDIA output nằm trong `artifacts/collector-poc/` (Git ignore). Các giá trị có trạng thái `available` chỉ xác nhận collector đọc được trên máy này, không thay thế đối chiếu độc lập.
