# Network Protocol & Data Bridge Specification

> ⚠️ **ARCHIVE NOTE (v2.0):** Cơ chế WebSocket server cục bộ đã được thay thế bằng **Slint Native Event Loop Timer** trực tiếp trong bộ nhớ. Không còn sử dụng giao thức WebSocket TCP hay mở cổng mạng `8080`.

## 1. Protocol Definition

## 2. Data Contract (JSON Schema)

Mỗi giây một lần, WebSocket Server phải đẩy ra một gói tin có cấu trúc chuẩn hóa sau đây. Hãy đảm bảo kiểu dữ liệu nhất quán:

```json
{
  "metric_id": 168492000,
  "cpu_usage": 16.0,
  "ram_usage": 45.3,
  "gpu_usage": 0.0,
  "vram_usage": 7.0,
  "network_speed": {
    "sent_kbps": 8.0,
    "received_kbps": 16.0
  }
}
```

## 3. Frontend Client Lifecycle (JavaScript)

Trang web chạy trên Lively Wallpaper phải triển khai một máy trạng thái (State Machine) bền bỉ để kết nối với Bridge:

```javascript
class DataBridgeClient {
    constructor(url) {
        this.url = url;
        this.ws = null;
        this.currentData = null;
        this.connect();
    }

    connect() {
        this.ws = new WebSocket(this.url);
        
        this.ws.onmessage = (event) => {
            this.currentData = JSON.parse(event.data);
            // Kích hoạt callback xử lý dữ liệu của Three.js tại đây
        };

        this.ws.onclose = () => {
            console.warn("WebSocket disconnected. Attempting reconnect in 3s...");
            setTimeout(() => this.connect(), 3000); // Tự động kết nối lại
        };

        this.ws.onerror = (err) => {
            console.error("Socket error: ", err);
            this.ws.close();
        };
    }
}
```

## 4. Performance & Security Tuning

- **Local Isolation**: Chỉ bind server tại địa chỉ `127.0.0.1` hoặc `localhost`. Không mở port ra ngoài mạng công cộng (Mạng LAN/Internet) để bảo mật hệ thống.
- **Serialization Overhead**: Hạn chế gửi các chuỗi text dài. Mọi thông tin nên được mã hóa ở dạng số thực (Float) hoặc số nguyên (Integer) nhằm tối ưu tốc độ phân tích cú pháp (parsing) của JavaScript Engine.
