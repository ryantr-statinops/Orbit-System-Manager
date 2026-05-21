# 3D Time-Series Visualization Engine (Three.js)

## 1. Visual Concept: 3D Terrain Grid / Ribbon Plot

Biến chuỗi thời gian của dữ liệu CPU/RAM thành một dải lụa hoặc lưới địa hình 3D (Terrain).

- **Trục X:** Đại diện cho các mốc thời gian (ví dụ: 60 giây gần nhất).
- **Trục Y:** Độ cao của lưới, tỷ lệ thuận với mức độ % sử dụng (0 - 100%).
- **Trục Z:** Chiều sâu không gian để phân tách các loại tài nguyên hoặc tạo hiệu ứng sóng cuộn.

## 2. WebGL & Three.js Pipeline

Thiết lập một Canvas hiệu năng cao chiếm toàn màn hình (Fullscreen, không viền, nền trong suốt hoặc tối):

import * as THREE from 'three';

// 1. Setup Scene, Camera & Renderer
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// 2. Create Time-Series Mesh (Ví dụ: Lưới phẳng biến dạng theo thời gian)
const widthSegments = 60; // Tương ứng 60 giây
const heightSegments = 10;
const geometry = new THREE.PlaneGeometry(10, 5, widthSegments, heightSegments);
const material = new THREE.MeshBasicMaterial({ color: 0x00ffcc, wireframe: true });
const terrain = new THREE.Mesh(geometry, material);
scene.add(terrain);

camera.position.z = 5;
camera.position.y = 2;
camera.lookAt(0,0,0);

// 3. Data Data Queue for Time-Series
let dataQueue = new Array(60).fill(0); // Chứa 60 điểm dữ liệu CPU

export function updateGridWithData(newCpuValue) {
    dataQueue.push(newCpuValue);
    dataQueue.shift(); // Cuộn dữ liệu cũ ra khỏi hàng đợi

    // Cập nhật tọa độ trục Y của các đỉnh trên Mesh dựa vào mảng dataQueue
    const positionAttribute = geometry.attributes.position;
    for (let i = 0; i <= widthSegments; i++) {
        const val = dataQueue[i] / 100.0; // Chuẩn hóa về khoảng 0 - 1
        for(let j = 0; j <= heightSegments; j++) {
            const index = i + j * (widthSegments + 1);
            positionAttribute.setY(index, val * 2); // Đẩy độ cao lưới lên
        }
    }
    geometry.attributes.position.needsUpdate = true; // Ép GPU cập nhật dữ liệu hình khối
}

// 4. Render Loop (60 FPS)
function animate() {
    requestAnimationFrame(animate);
    // Áp dụng kỹ thuật xoay hoặc di ch uyển camera nhẹ nhàng để tạo cảm giác không gian 3D
    renderer.render(scene, camera);
}
animate();
3. Mathematical Optimization: Linear Interpolation (Lerp)Vì dữ liệu hệ thống chỉ đổ về mỗi 1 giây (delta_t = 1s), nếu gán trực tiếp giá trị vào Mesh sẽ gây ra hiện tượng giật hình ở tần số 60Hz. Ta cần dùng thuật toán Lerp trong hàm cập nhật khung hình đồ họa:$$\text{Value}_{\text{render}} = \text{Value}_{\text{render}} + \alpha \times (\text{Value}_{\text{target}} - \text{Value}_{\text{render}})$$Với $\alpha \approx 0.05$ đến $0.1$ để tạo độ mượt chuyển động trượt.4. Resource Cleanup RulesChạy trên hình nền yêu cầu không được rò rỉ bộ nhớ. Khi khởi tạo lại hoặc thay đổi cấu trúc đồ họa, bắt buộc gọi: geometry.dispose() và material.dispose().
