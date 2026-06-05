# Backend System Monitor Service: Specifications

## 1. Objective

Xây dựng một dịch vụ chạy ngầm (Background Service) gọn nhẹ trên Windows để thu thập dữ liệu hiệu năng phần cứng theo thời gian thực (Real-time) và phân phối qua giao thức WebSockets.

## 2. Hardware Targets (Dell Precision Workstation Target)

- **CPU:** Intel Core i9-9880H (8 Cores / 16 Logicals). Yêu cầu lấy tổng số % và chi tiết từng nhân nếu cần.
- **Memory:** Tổng dung lượng 32GB. Yêu cầu lấy dung lượng đã dùng (Used) và phần trăm (%).
- **GPU 0/1:** NVIDIA Quadro T1000 & Intel UHD Graphics. Lấy % sử dụng và nhiệt độ (nếu có).

## 3. Tech Stack Options & Implementation

Agent có thể chọn một trong hai phương án sau tùy thuộc vào môi trường cấu hình:

### Option A: Python Core (Khuyên dùng cho giai đoạn PoC)

- Thư viện: `psutil`, `websockets`, `asyncio`, `GPUtil`
- Tần suất quét (Sampling Rate): 1.0 Giây (1Hz). Không quét quá dày để tránh chiếm dụng chu kỳ CPU.

```python
import asyncio
import json
import psutil
import websockets

async def get_system_metrics():
    # Thu thập dữ liệu từ OS Kernel thông qua psutil
    metrics = {
        "timestamp": asyncio.get_event_loop().time(),
        "cpu": {
            "total": psutil.cpu_percent(interval=None),
            "cores": psutil.cpu_percent(interval=None, percpu=True)
        },
        "memory": {
            "total": psutil.virtual_memory().total,
            "used": psutil.virtual_memory().used,
            "percent": psutil.virtual_memory().percent
        }
    }
    return metrics

async def handler(websocket, path):
    print(f"Client connected: {websocket.remote_address}")
    while True:
        try:
            data = await get_system_metrics()
            await websocket.send(json.dumps(data))
            await asyncio.sleep(1) # Khống chế tần suất 1Hz
        except websockets.exceptions.ConnectionClosed:
            print("Client disconnected.")
            break

start_server = websockets.serve(handler, "localhost", 8080)
asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()
```

## 4. Key Considerations for Agent

- **Interval Control**: Không đặt interval của `cpu_percent` là blocking. Hãy để `interval=None` hoặc sử dụng cơ chế bất đồng bộ của `asyncio`.
- **JSON Payload Schema**: Giữ cấu trúc JSON phẳng hoặc phân cấp rõ ràng để Frontend xử lý với chi phí tính toán thấp nhất.
- **Error Handling**: Bắt các ngoại lệ khi mất kết nối socket để giải phóng tài nguyên hệ thống ngay lập tức.
