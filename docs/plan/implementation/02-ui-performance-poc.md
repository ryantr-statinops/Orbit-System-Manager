# Stage 2 — UI và hiệu năng PoC

## Mục tiêu

Xác minh surface và đo baseline trên Windows, Ubuntu, Fedora trước khi xây dashboard.

## Checklist

- [ ] Dựng Wails v3 + React + TypeScript + Vite; khóa dependency tương thích và runtime frontend tương ứng.
- [ ] Thiết lập browser preview bằng fixture deterministic, không phụ thuộc collector.
- [ ] Dùng ECharts theo module cho 2D; Three.js trực tiếp cho surface, camera và vòng render.
- [ ] Tạo fixture 0/1/4/16/32/64 core, lịch sử 60 giây; có trạng thái empty, single và unavailable.
- [ ] Tái sử dụng buffer mesh; tách cập nhật metric khỏi render, tránh React state cho từng frame.
- [ ] Kiểm tra rotate/pan/zoom/reset; giới hạn pixel ratio và giảm render khi cửa sổ bị ẩn.
- [ ] Xác minh GTK/WebKitGTK/WebView từng OS; ghi dependency và lỗi rendering.
- [ ] Đo bản release: thời gian đến UI/mẫu đầu, RAM cả cây tiến trình, CPU toàn máy và frame time p95 khi tương tác.
- [ ] Chạy idle và tương tác 30 phút; ghi cấu hình, scaling, độ phân giải và phương pháp đo.
- [ ] Người dùng duyệt baseline/ngưỡng trước stage 3.

## Điều kiện hoàn thành

PoC chạy native trên ba môi trường, benchmark tái lập; giải quyết vấn đề stack trước stage 3.

## Kết quả và blocker

Đã triển khai một phần trên `main`; toàn bộ commit đang ở local và chưa push.

- Có Wails v3.0.0 host, React/TypeScript/Vite browser preview và lockfile frontend.
- Fixture deterministic có 0/1/4/16/32/64 core; trạng thái history 60 mẫu, empty, single và unavailable.
- Có ECharts CPU history và Three.js surface với OrbitControls, reset camera, buffer tái sử dụng, pixel ratio tối đa 1.5 và bỏ render khi tab ẩn.
- Có Playwright harness ghi thời gian đến UI trong browser preview và p95 frame interval khi tương tác. Chỉ số này không đại diện cho native Wails.
- Có `cmd/ui-benchmark` xuất JSONL gồm CPU toàn máy và RSS/process count của cây tiến trình được chọn.

Windows inventory của môi trường phát triển: Windows 11 Pro 26H2 build 26300, Go 1.27.0, Node 22.23.1, npm 10.9.8, WebView2 156.0.4314.58. `wails3 doctor` báo môi trường Windows sẵn sàng, `CGO_ENABLED=0`.

**Kiểm tra đang chờ theo yêu cầu người dùng:** chạy lại `npm ci`, `npm run build`, `npm test`, `npm run benchmark:browser`, `go test ./...`, `go vet ./...`, build và mở ứng dụng Wails native. Cần cài Playwright Chromium trước khi chạy browser harness.

**Blocker nghiệm thu:** chưa xem được giao diện trực tiếp; chưa chạy phép đo idle/tương tác 30 phút; chưa kiểm chứng native trên Ubuntu/Fedora; chưa có baseline hoặc ngưỡng được duyệt. Chưa đánh dấu checklist Stage 2 hoàn thành. Stage 3 tiếp tục chờ đủ bằng chứng và duyệt baseline.

Lệnh kiểm tra frontend sau này:

```sh
cd frontend
npm ci
npx playwright install chromium
npm test
npm run benchmark:browser
npm run build
```

Lệnh native sampler sau khi mở ứng dụng và lấy PID:

```sh
go run ./cmd/ui-benchmark --pid <pid> --interval 1s --duration 30m > artifacts/ui-benchmark/session.jsonl
```

Tạo thư mục `artifacts/ui-benchmark/` trước khi chuyển hướng output. Thư mục này được Git ignore.
