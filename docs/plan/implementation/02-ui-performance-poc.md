# Stage 2 — UI và hiệu năng PoC

## Mục tiêu

Xác minh surface và đo baseline native trên Windows x64 trước khi xây dashboard hoàn chỉnh.

## Checklist

- [x] Dựng Wails v3 + React + TypeScript + Vite; khóa dependency tương thích và runtime frontend tương ứng.
- [x] Thiết lập browser preview bằng fixture deterministic, không phụ thuộc collector.
- [x] Dùng ECharts theo module cho 2D; Three.js trực tiếp cho surface, camera và vòng render.
- [x] Tạo fixture 0/1/4/16/32/64 core, lịch sử 60 giây; có trạng thái empty, single và unavailable.
- [x] Tái sử dụng buffer mesh; tách cập nhật fixture khỏi render và không dùng React state cho từng frame.
- [x] Giới hạn pixel ratio và giảm render khi cửa sổ bị ẩn; code và unit test pixel-ratio đã kiểm tra.
- [ ] Native review camera: orbit/rotate và reset đã hoạt động; pan/zoom chưa được xác minh.
- [x] Xác minh WebView2 trên Windows; ghi phiên bản runtime và xác nhận fixture render được.
- [ ] Đo bản release: thời gian đến UI/mẫu đầu, RAM cả cây tiến trình, CPU toàn máy và frame time p95 khi tương tác.
- [ ] Chạy idle và tương tác 30 phút; ghi cấu hình, scaling, độ phân giải và phương pháp đo.
- [ ] Người dùng duyệt baseline/ngưỡng trước stage 3.

## Điều kiện hoàn thành

PoC chạy native trên Windows x64; browser và native benchmark có phương pháp tái lập, kết quả và blocker được ghi rõ; người dùng duyệt baseline trước Stage 3.

## Kết quả và blocker

Đã triển khai một phần trên `main`; các commit hiện có đã được push. Kiểm chứng Windows và duyệt baseline vẫn còn chờ.

- Có Wails v3.0.0 host, React/TypeScript/Vite browser preview và lockfile frontend.
- Fixture deterministic có 0/1/4/16/32/64 core; trạng thái history 60 mẫu, empty, single và unavailable.
- Có ECharts CPU history và Three.js surface với OrbitControls, reset camera, buffer tái sử dụng, pixel ratio tối đa 1.5 và bỏ render khi tab ẩn.
- Có Playwright harness ghi thời gian đến UI trong browser preview và p95 frame interval khi tương tác. Chỉ số này không đại diện cho native Wails.
- Có `cmd/ui-benchmark` xuất JSONL gồm CPU toàn máy và RSS/process count của cây tiến trình được chọn. Đã sửa lỗi kiểu tham số ở `cpu.Percent`.

Windows inventory của môi trường phát triển: Windows 11 Pro 26H2 build 26300, Go 1.27.0, Node 22.23.1, npm 10.9.8, WebView2 156.0.4314.58. `wails3 doctor` báo môi trường Windows sẵn sàng, `CGO_ENABLED=0`.

**Kiểm tra hoàn thành ngày 2026-11-04:** `npm ci` thành công; `npm test` — 4 file/14 test đạt; `npm run build` thành công; `go test ./...` và `go vet ./...` đạt; Go CLI, UI benchmark sampler và Wails app đều build được. Sửa riêng lỗi TypeScript tooltip, Vitest quét nhầm thư mục e2e và selector heading không duy nhất. Bundle production khoảng 1.2 MB, vượt cảnh báo 500 kB.

Browser benchmark Playwright đạt với 64 core/60 fixture samples: UI ready `614.8 ms`, 11 frame intervals, p95 `16.7 ms`. Đây chỉ là browser preview; số mẫu tương tác còn ít và không thay thế native Wails baseline.

**Native startup update — 2026-11-04:** WebView2 ban đầu không ghi được vào profile mặc định dưới `%APPDATA%`. Sau khi cấu hình profile riêng trong `%LOCALAPPDATA%\OrbitSystemManager\WebView2`, Wails khởi động, tạo WebView2 environment và phục vụ frontend assets. Tuy nhiên, công cụ điều khiển desktop hết thời gian chờ khi bind cửa sổ để review; native UI chưa được quan sát trực tiếp. Sampler chỉ thu được dữ liệu partial với RSS `null` sau khi tiến trình đích kết thúc, nên không dùng làm baseline.

**Blocker native:** chưa review được tương tác trực tiếp, chưa thu GPU/native frame-time và chưa chạy idle/tương tác 30 phút. Baseline chưa đủ bằng chứng để duyệt. Không đánh dấu Stage 2 hoàn thành.

## Gate còn mở

- Khắc phục hoặc xác định giới hạn WebView2 data-folder trên Windows để review native.
- Thu GPU/frame-time native; chạy idle và tương tác 30 phút mỗi chế độ.
- Đánh giá cảnh báo bundle lớn trong baseline; xin người dùng duyệt baseline/ngưỡng.
- Stage 3 vẫn chờ baseline Windows và duyệt metric Stage 1. Linux/Fedora ngoài phạm vi hiện tại.

Lệnh browser benchmark đã dùng:

```powershell
Set-Location frontend
npm test
npm run build
npm run benchmark:browser
Set-Location ..
```

Lệnh native sampler sau khi mở ứng dụng và lấy PID:

```sh
go run ./cmd/ui-benchmark --pid <pid> --interval 1s --duration 30m > artifacts/ui-benchmark/session.jsonl
```

Tạo thư mục `artifacts/ui-benchmark/` trước khi chuyển hướng output. Thư mục này được Git ignore.

## Native Windows recheck — 2026-11-05

- `go test ./...`, `go vet ./...`, build CLI, build UI sampler và build Wails thoát thành công. Optimized release build (`go build -trimpath -ldflags "-s -w"`) tạo file 13,720,064 byte; Windows Application Control chặn chạy trực tiếp file build. `go run .` khởi động được Wails v3 trên WebView2 `156.0.4314.58` và phục vụ fixture.
- Review native qua cửa sổ Wails: tiêu đề, metric cards, history chart và surface 3D hiển thị; đổi fixture sang 64 core và kéo xoay surface hoạt động. Zoom chưa xác nhận rõ. Đây là review fixture, không phải live dashboard.
- Preflight sampler 10 giây: CPU toàn máy có dữ liệu; process tree `partial` trong cả 10 mẫu, chỉ thấy một process và RSS là `null`. Chẩn đoán `gopsutil.Process.MemoryInfo` trả `Access is denied`; không dùng dữ liệu này làm baseline.
- `PresentMon` chưa cài. Hướng dẫn chính thức yêu cầu Performance Log Users hoặc chạy elevated để thu ETW; `typeperf` cũng xác nhận tài khoản hiện tại không có quyền đó. Không đổi nhóm/quyền và không chạy elevated. Vì vậy GPU/frame-time native và RSS cây tiến trình chưa thu được; không chạy hai phép đo 30 phút khi preflight không hợp lệ. ([PresentMon access requirements](https://github.com/GameTechDev/PresentMon#troubleshooting))
- `npm test`: 4 file, 14 test đạt. `npm run build` đạt; bundle JavaScript `1,201.99 kB` raw / `353.14 kB` gzip và còn cảnh báo chunk trên 500 kB. Chưa code-split khi chưa có số đo native startup để đánh giá tác động.
- `npm run benchmark:browser` được thử lại nhưng Vite/Playwright không thể truy cập socket `127.0.0.1:9245` do quyền mạng của môi trường; lần chạy bị dừng. Kết quả browser gần nhất vẫn là 2026-11-04: UI ready `614.8 ms`, 11 frame intervals, p95 `16.7 ms`; không xem đây là native baseline.

Raw sampler preflight nằm trong `artifacts/ui-benchmark/` (Git ignore). Native baseline vẫn bị chặn bởi quyền đọc tiến trình/ETW; cần một môi trường có quyền đo hợp lệ trước khi duyệt Stage 2.
