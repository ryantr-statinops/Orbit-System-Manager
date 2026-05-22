## 📁 Cấu trúc tổng thể

index.html   → Layout HTML (bố cục 3 cột)
style.css    → CSS Layout (kích thước, màu sắc, khoảng cách)
main.js      → JavaScript biểu đồ (cấu hình vẽ, kích thước canvas, màu sắc biểu đồ)
────────────────────────────────────────────────────────────────────────────────
🎨 1. LAYOUT & KÍCH THƯỚC KHUNG —  style.css 
Bố cục 3 cột (Layout chính)
┌────────────────┬─────┬────────────────────────────────────────────────────────────────────┐
│ Vị trí         │ Dòn │ Mô tả                                                              │
│                │ g   │                                                                    │
├────────────────┼─────┼────────────────────────────────────────────────────────────────────┤
│ #main-layout   │ ~66 │ Grid 3 cột: 290px 1fr 290px — cột trái 290px, cột phải 290px, giữa │
│                │     │ co giãn                                                            │
│ #col-left      │ ~72 │ Cột trái — scroll dọc, padding: 6px 8px                            │
│ #col-right     │ ~72 │ Cột phải — scroll dọc, padding: 6px 8px                            │
│ #center-canvas │ ~82 │ Trung tâm — 3D canvas, position: relative, overflow: hidden        │
└────────────────┴─────┴────────────────────────────────────────────────────────────────────┘
Kích thước & màu sắc card
┌─────────────────┬─────┬───────────────────────────────────────────────────────────────────┐
│ Selector        │ Dòn │ Mô tả                                                             │
│                 │ g   │                                                                   │
├─────────────────┼─────┼───────────────────────────────────────────────────────────────────┤
│ .card           │ ~90 │ Card: background: #ffffff, border 1px solid #e5e7eb, padding: 8px │
│                 │     │ 10px                                                              │
│ .card-header    │ ~94 │ Header card: chữ 9px, color: #6c757d, border-bottom               │
│ .stat-row       │ ~10 │ Hàng thống kê: 11px, border-bottom #f0f0f0                        │
│                 │ 2   │                                                                   │
│ .stat-bar-track │ ~11 │ Thanh bar nền: height: 6px, background: #e9ecef                   │
│                 │ 6   │                                                                   │
│ .stat-bar-fill  │ ~12 │ Thanh bar fill: #21918c (teal), transition: width 0.3s            │
│                 │ 1   │                                                                   │
│ .ram-fill       │ ~12 │ RAM bar fill: #3b528b (xanh dương đậm)                            │
│                 │ 7   │                                                                   │
└─────────────────┴─────┴───────────────────────────────────────────────────────────────────┘
Core Bars (16 cores)
┌─────────────┬──────┬─────────────────────────────────────────────┐
│ Selector    │ Dòng │ Mô tả                                       │
├─────────────┼──────┼─────────────────────────────────────────────┤
│ .core-bar   │ ~131 │ Flex row, gap 5px, font 9.5px               │
│ .core-track │ ~140 │ Track bar: height: 8px, background: #e9ecef │
│ .core-fill  │ ~147 │ Fill bar: #21918c, transition: width 0.25s  │
│ .core-spark │ ~157 │ Sparkline: width: 40px, height: 14px        │
└─────────────┴──────┴─────────────────────────────────────────────┘
Canvas biểu đồ
┌─────────────────────┬──────┬──────────────────────────────────────┐
│ Selector            │ Dòng │ Mô tả                                │
├─────────────────────┼──────┼──────────────────────────────────────┤
│ #boxplot-canvas     │ ~163 │ Boxplot: width: 100%, border #f0f0f0 │
│ #ram-boxplot-canvas │ ~163 │ RAM Boxplot: tương tự                │
│ #density-canvas     │ ~163 │ KDE density: tương tự                │
│ #network-canvas     │ ~163 │ Network chart: tương tự              │
└─────────────────────┴──────┴──────────────────────────────────────┘
────────────────────────────────────────────────────────────────────────────────
📐 2. KÍCH THƯỚC CANVAS & BIỂU ĐỒ —  index.html 
Trong file  index.html , kích thước  width/height  của các canvas được set cứng qua HTML attributes:
┌─────────────────────┬─────────────┬─────────────────────────────┐
│ Canvas              │ Dòng        │ Kích thước (width × height) │
├─────────────────────┼─────────────┼─────────────────────────────┤
│ #boxplot-canvas     │ ~36         │ 290 × 100                   │
│ #ram-boxplot-canvas │ ~52         │ 290 × 60                    │
│ #density-canvas     │ ~64         │ 290 × 80                    │
│ #network-canvas     │ ~67         │ 290 × 90                    │
│ Core sparklines     │ main.js ~69 │ 40 × 14 (tạo động trong JS) │
└─────────────────────┴─────────────┴─────────────────────────────┘
────────────────────────────────────────────────────────────────────────────────
🖌️ 3. CẤU HÌNH MÀU SẮC & VẼ BIỂU ĐỒ —  main.js 
Màu chủ đạo (Accent Color)
- Dòng  ~19 :  const ACCENT_COLOR = new THREE.Color('#21918c')  — màu teal Viridis, dùng cho toàn bộ 3D surface, wireframe, và sidebar charts
3D Three.js (bắt đầu từ dòng  ~73 )
┌───────────────────────┬──────────┬────────────────────────────────────┐
│ Cấu hình              │ Dòng     │ Mô tả                              │
├───────────────────────┼──────────┼────────────────────────────────────┤
│ ALPHA                 │ ~3       │ Hệ số lerp = 0.1 (làm mượt độ cao) │
│ CORE_COUNT            │ ~4       │ 16 cores                           │
│ TIME_WINDOW           │ ~5       │ 60s rolling window                 │
│ MAX_HEIGHT            │ ~6       │ Chiều cao tối đa terrain = 9.0     │
│ scene.background      │ ~76      │ #f8f9fa                            │
│ camera.position       │ ~81      │ Góc nhìn isometric                 │
│ fillMat.opacity       │ ~98      │ Độ opaque surface fill = 0.06      │
│ wireMat.opacity       │ ~106     │ Độ opaque wireframe = 0.55         │
│ Bounding box color    │ ~120     │ 0x6c757d (xám)                     │
│ Bounding box dash/gap │ ~122-123 │ dashSize: 0.5, gapSize: 0.6        │
└───────────────────────┴──────────┴────────────────────────────────────┘
Boxplot —  drawBoxplot()  (dòng ~223)
┌─────────────┬────────────────────────────────────────────┐
│ Cấu hình    │ Mô tả                                      │
├─────────────┼────────────────────────────────────────────┤
│ pad = 20    │ Khoảng cách lề 2 bên                       │
│ boxH = 14   │ Chiều cao hộp IQR                          │
│ Màu fill    │ color param (mặc định #21918c), alpha 0.25 │
│ Màu stroke  │ color, lineWidth 1.5                       │
│ Median line │ lineWidth = 2                              │
└─────────────┴────────────────────────────────────────────┘
KDE Density —  drawDensityPlot()  (dòng ~263)
┌────────────────────────┬──────────────────────┐
│ Cấu hình               │ Mô tả                │
├────────────────────────┼──────────────────────┤
│ pad = 4                │ Lề                   │
│ steps = 60             │ Số điểm sampling KDE │
│ bandwidth = range / 12 │ Bandwidth KDE        │
│ Màu fill               │ color, alpha 0.2     │
│ Line width             │ 1.5                  │
└────────────────────────┴──────────────────────┘
Network Line Chart —  drawNetworkChart()  (dòng ~320)
┌──────────────────────────────────────────────────┬──────────────────────────────┐
│ Cấu hình                                         │ Mô tả                        │
├──────────────────────────────────────────────────┼──────────────────────────────┤
│ pad = { top: 4, right: 4, bottom: 14, left: 30 } │ Lề chi tiết                  │
│ Màu Sent line                                    │ #3b528b (xanh dương Viridis) │
│ Màu Recv line                                    │ #21918c (teal)               │
│ Line width                                       │ 1.2                          │
└──────────────────────────────────────────────────┴──────────────────────────────┘
Sparkline —  drawSparkline()  (dòng ~191)
┌─────────────┬─────────┐
│ Cấu hình    │ Mô tả   │
├─────────────┼─────────┤
│ strokeStyle │ #21918c │
│ lineWidth   │ 1       │
└─────────────┴─────────┘
────────────────────────────────────────────────────────────────────────────────
🎯 Tóm tắt nhanh
┌─────────────────────────────────┬──────────┬──────────────────────────────────────────────┐
│ Bạn muốn thay đổi               │ Vào file │ Vị trí                                       │
├─────────────────────────────────┼──────────┼──────────────────────────────────────────────┤
│ Kích thước sidebar (290px)      │ style.cs │ #main-layout grid                            │
│                                 │ s        │                                              │
│ Màu nền, border, padding card   │ style.cs │ .card, .card-header                          │
│                                 │ s        │                                              │
│ Kích thước canvas biểu đồ       │ index.ht │ Attributes width/height trên thẻ <canvas>    │
│                                 │ ml       │                                              │
│ Màu teal chủ đạo (#21918c)      │ main.js  │ Dòng 19: ACCENT_COLOR                        │
│ Màu fill boxplot, density,      │ main.js  │ Hàm drawBoxplot, drawDensityPlot,            │
│ network                         │          │ drawNetworkChart                             │
│ Độ cao 3D terrain               │ main.js  │ Dòng 6: MAX_HEIGHT                           │
│ Độ opaque wireframe/surface     │ main.js  │ Dòng ~98-106                                 │
│ Hệ số làm mượt lerp             │ main.js  │ Dòng 3: ALPHA                                │
│ Kích thước sparkline            │ main.js  │ Dòng ~69: width="40" height="14"             │
│ Màu Top bar / Status            │ style.cs │ #top-bar, .status-dot                        │
│                                 │ s        │                                              │
└─────────────────────────────────┴──────────┴──────────────────────────────────────────────┘