# Stage 6 — Chuyển đổi và phát hành

## Mục tiêu

Chuyển Go thành bản chính sau duyệt; lưu mốc Rust trong Git, thử nghiệm trước stable.

## Checklist

- [ ] Tổng hợp bằng chứng stage 1–5: ma trận metric, baseline đã duyệt, native checks và nâng cấp package.
- [ ] Người dùng duyệt chuyển đổi trước khi bỏ build Rust/Slint.
- [ ] Lưu commit/tag mốc Rust cuối; chuyển entrypoint và build chính sang Go/Wails.
- [ ] Bỏ dependency/build cũ; giữ asset/tài liệu bản mới còn dùng.
- [ ] Cập nhật CI: Go tests/checks, frontend lint/typecheck/tests/build và package trên ba OS.
- [ ] Chuẩn hóa lệnh browser preview, desktop dev, test và release; cập nhật README, AGENTS.md và tài liệu UI/preview.
- [ ] Công bố metric/thiết bị đã kiểm chứng, giới hạn sensor, dependency và cách bật helper nếu có.
- [ ] Kiểm tra license dependency/asset; hướng dẫn cài, cập nhật, gỡ cài và khôi phục bản trước.
- [ ] Release thử nghiệm, kiểm tra installer/update; đưa vào stable sau nghiệm thu.
- [ ] Ghi release notes, known issues và kết quả kiểm chứng; cập nhật trạng thái stage trong README kế hoạch.

## Điều kiện hoàn thành

Bản mới được duyệt, CI/package đạt yêu cầu; stable có bằng chứng nâng cấp và giới hạn hỗ trợ.

## Kết quả và blocker

Chưa thực hiện. Ghi mốc Git/release, blocker và bằng chứng tại đây.
