## 📁 Cấu trúc Frontend (Module ES6)

```
src/
├── index.js         → Entry point (init ThreeScene, TelemetryClient, animation loop)
├── state.js         → Global state (grids, histories, connection state)
├── ui.js            → DOM, HUD, charts (boxplot, density, network, sparklines), splitters
├── threeScene.js    → 3D CPU topology (Three.js PlaneGeometry + OrbitControls)
├── timeSeries.js    → TimeSeriesBuffer (block-based boxplot data)
├── websocket.js     → TelemetryClient (WebSocket + auto-reconnect + mock fallback)
└── constants.js     → All constants (ALPHA, CORE_COUNT, COLORS, WS_URL...)
```

────────────────────────────────────────────────────────────────────────────────
🎨 **LAYOUT & CSS** — `style.css`

Bố cục 3 cột (Flexbox với splitters có thể kéo thả)
┌────────────────┬──────┬──────────────────────────────────────────────────────┐
│ Vị trí         │ Dòng │ Mô tả                                                │
├────────────────┼──────┼──────────────────────────────────────────────────────┤
│ #main-layout   │ ~72  │ Flex row: col-left (290px) splitter center splitter  │
│                │      │ col-right (290px). Center co giãn với flex: 1        │
│ #top-bar       │ ~44  │ Fixed height: 34px, flex: space-between             │
│ #col-left      │ ~79  │ Cột trái — width: 290px, scroll dọc                │
│ #col-right     │ ~79  │ Cột phải — width: 290px, scroll dọc                │
│ #center-canvas │ ~102 │ Trung tâm — flex: 1, position: relative             │
│ .splitter      │ ~107 │ 5px width, cursor: col-resize, collapse animation   │
└────────────────┴──────┴──────────────────────────────────────────────────────┘

Cards (`style.css` ~ dòng 125)
┌─────────────────┬──────────┬─────────────────────────────────────────────────┐
│ Selector        │ Mô tả                                                      │
├─────────────────┼──────────┼─────────────────────────────────────────────────┤
│ .card           │ background: #ffffff, border: 1px solid #e5e7eb, padding: 8px 10px │
│ .card-header    │ 0.72rem uppercase, color: #6c757d, border-bottom         │
│ .stat-row       │ Flex space-between, padding: 1px 2px, font-size: 0.88rem │
│ .stat-bar-track │ height: 6px (RAM: 16px), background: #e9ecef            │
│ .stat-bar-fill  │ #21918c (teal), transition: width 0.3s ease             │
│ .ram-bar-track  │ height: 16px                                             │
│ .core-bar       │ Flex row, gap: 5px, font: 0.76rem                       │
│ .core-track     │ height: 8px, background: #e9ecef                        │
│ .core-fill      │ #21918c, transition: width 0.25s                       │
│ .core-spark     │ width: 40px, height: 14px (canvas)                      │
└─────────────────┴──────────┴─────────────────────────────────────────────────┘

────────────────────────────────────────────────────────────────────────────────
📐 **CANVAS KÍCH THƯỚC** — `index.html`

┌─────────────────────┬──────────────┬────────────────────────────────┐
│ Canvas              │ Dòng index   │ Kích thước (width × height)    │
├─────────────────────┼──────────────┼────────────────────────────────┤
│ #boxplot-canvas     │ ~59          │ 290 × 160                     │
│ #ram-timeseries-canvas│ ~85        │ 290 × 80                      │
│ #density-canvas     │ ~92          │ 290 × 60                      │
│ #gpu-timeseries-canvas│ ~93         │ 290 × 80                      │
│ #network-canvas     │ ~98          │ 290 × 90                      │
│ Core sparklines     │ ui.js ~56    │ 40 × 14 (tạo động trong JS)  │
└─────────────────────┴──────────────┴────────────────────────────────┘

────────────────────────────────────────────────────────────────────────────────
🖌️ **CẤU HÌNH MÀU SẮC & BIỂU ĐỒ** — `src/ui.js`

Màu chủ đạo (Accent Color): `#21918c` (Viridis teal) — định nghĩa trong `src/constants.js`

**3D Three.js** — `src/threeScene.js`
┌──────────────────────────┬────────┬──────────────────────────────────────┐
│ Cấu hình                 │ Dòng   │ Mô tả                                │
├──────────────────────────┼────────┼──────────────────────────────────────┤
│ ALPHA                    │ const. │ Hệ số lerp = 0.1 (làm mượt độ cao)   │
│ CORE_COUNT               │ const. │ 16 cores                             │
│ TIME_WINDOW              │ const. │ 30s rolling window                   │
│ MAX_HEIGHT               │ const. │ Chiều cao tối đa terrain = 20        │
│ SQUARE_SPAN              │ const. │ 60 units                             │
│ scene.background         │ ~16    │ #f8f9fa (trắng xám)                  │
│ camera.position          │ ~19    │ (48, 70, 42) — góc isometric        │
│ MeshBasicMaterial.opacity│ ~46    │ fill: 0.06, wireframe: 0.55          │
│ Color                    │ const. │ #21918c (teal Viridis)               │
└──────────────────────────┴────────┴──────────────────────────────────────┘

**Boxplot** — `ui.js` function `drawTimeSeriesBoxplot()` (~ dòng 170)
┌──────────────────────┬─────────────────────────────────────────────────┐
│ Cấu hình             │ Mô tả                                           │
├──────────────────────┼─────────────────────────────────────────────────┤
│ margin left: 28      │ Khoảng cách lề trái                                │
│ Vertical boxplot     │ Vẽ boxplot dọc (thay vì ngang)                  │
│ Màu fill             │ #21918c, alpha 0.25                               │
│ Màu stroke           │ #21918c, lineWidth 1.2                            │
│ Median line          │ lineWidth = 2                                   │
│ Stripplot            │ Jittered scatter cho block hiện tại (NOW)       │
│ Time blocks          │ 5s mỗi block, tối đa 30 blocks (TimeSeriesBuffer)│
└──────────────────────┴─────────────────────────────────────────────────┘

**KDE Density** — `ui.js` function `drawDensityPlot()` (~ dòng 240)
┌─────────────────────────┬──────────────────────────────────────────┐
│ Cấu hình                │ Mô tả                                    │
├─────────────────────────┼──────────────────────────────────────────┤
│ pad = 4                 │ Lề                                        │
│ steps = 60              │ Số điểm sampling KDE                      │
│ bandwidth = range / 12  │ Bandwidth KDE (Silverman's rule of thumb) │
│ kernel                  │ Gaussian (exp(-0.5 * d²))                │
│ Màu fill                │ #21918c, alpha 0.2                       │
│ Line width              │ 1.5                                      │
│ Data source             │ GPU 0 load history (hoặc first GPU)      │
└─────────────────────────┴──────────────────────────────────────────┘

**GPU Time-Series** — `ui.js` function `drawGpuTimeSeries()` (~ dòng 310)
┌──────────────────────────┬──────────────────────────────────────────┐
│ Cấu hình                 │ Mô tả                                    │
├──────────────────────────┼──────────────────────────────────────────┤
│ pad                      │ top: 4, right: 4, bottom: 14, left: 30  │
│ GPU 0 line               │ #21918c (teal)                          │
│ GPU 1 line               │ #d4a017 (gold, conditional)             │
│ Line width               │ 1.5, fill alpha 0.1                     │
│ Window                   │ 30 data points                          │
└──────────────────────────┴──────────────────────────────────────────┘

**Network Chart** — `ui.js` function `drawNetwork()` + `drawLine()` (~ dòng 413)
┌──────────────────────────┬──────────────────────────────────────────┐
│ Cấu hình                 │ Mô tả                                    │
├──────────────────────────┼──────────────────────────────────────────┤
│ Recv line                │ #21918c (teal)                          │
│ Sent line                │ #fde725 (yellow Viridis)                │
│ Line width               │ 2.0                                     │
│ Normalization            │ scale bởi max value trong window         │
│ Window                   │ 30 data points                          │
└──────────────────────────┴──────────────────────────────────────────┘

**Sparkline** — `ui.js` f
