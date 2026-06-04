# Task Manager 3D Visualize

Ứng dụng giám sát hệ thống theo thời gian thực cho CPU, RAM, GPU và network. Phần frontend chạy trong trình duyệt bằng ES module, phần backend là server WebSocket viết bằng Python trong `app.py`.

## Cấu trúc dự án

```text
.
|-- app.py
|-- index.html
|-- style.css
|-- src/
|   |-- constants.js
|   |-- index.js
|   |-- state.js
|   |-- threeScene.js
|   |-- timeSeries.js
|   |-- ui.js
|   `-- websocket.js
|-- assets/
|-- libs/
|-- package.json
|-- rollup.config.js
|-- requirements.txt
`-- README.md
```

## Yêu cầu

- Python 3.9 trở lên
- Node.js 18 trở lên
- `pip`
- `npm`

## Cài đặt

### Backend Python

```bash
pip install -r requirements.txt
```

### Frontend

```bash
npm install
```

## Cách chạy dự án

Dự án hiện cần chạy theo 2 tiến trình riêng:

### 1. Chạy backend WebSocket

```bash
python app.py
```

Backend lắng nghe tại:

- `ws://127.0.0.1:8080`

### 2. Phục vụ frontend qua HTTP

Không mở trực tiếp `index.html` bằng `file://`, vì frontend đang dùng ES module và cần được phục vụ qua HTTP.

Bạn có thể dùng server tĩnh bất kỳ. Ví dụ:

```bash
python -m http.server 8000
```

Sau đó mở:

- `http://127.0.0.1:8000/index.html`

## Build

Dự án có sẵn cấu hình Rollup để đóng gói frontend:

```bash
npm run build
```

Lệnh này tạo bundle tại `dist/bundle.js` theo cấu hình trong `rollup.config.js`.

## Chế độ phát triển

Nếu muốn Rollup theo dõi thay đổi và build lại tự động:

```bash
npm run start
```

Lưu ý: đây không phải web server. Bạn vẫn cần một server HTTP riêng để mở trang trong trình duyệt.

## Ghi chú quan trọng

- Nếu `GPUtil` không cài đặt được hoặc hệ thống không có GPU tương thích, backend vẫn chạy bình thường, chỉ là dữ liệu GPU có thể về `0` hoặc fallback.
- Frontend đang tải `Three.js` từ CDN và `OrbitControls` từ `libs/OrbitControls.js`.
- Điểm vào của frontend là `src/index.js`, không còn là một `main.js` độc lập như phiên bản cũ.

## Giấy phép

MIT
