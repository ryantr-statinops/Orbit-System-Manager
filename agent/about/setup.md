# Python Environment Setup — Conda & Pip Guide

## 📖 Giới thiệu

Dự án **Orbit System Manager** sử dụng **Miniconda** để quản lý môi trường Python độc lập, kết hợp **pip** để cài đặt một số thư viện không có sẵn trên Conda channels.

### Tại sao dùng Conda?

| Vấn đề | Giải pháp Conda |
|--------|-----------------|
| Mỗi dự án cần Python version khác nhau | Conda tạo environment riêng biệt, mỗi env có Python version riêng |
| Xung đột thư viện giữa các dự án | Mỗi env là một sandbox biệt lập |
| Cần quản lý cả Python lẫn system libraries | Conda quản lý được cả (ví dụ: `numpy` + `mkl`) |
| Tái hiện chính xác môi trường trên máy khác | File `environment.yml` chứa toàn bộ cấu hình |

---

## 🔧 Cài đặt Miniconda

### Nếu chưa có Miniconda

```bash
# Windows — tải installer từ:
# https://docs.conda.io/en/latest/miniconda.html
# Chạy file .exe, chọn "Install for All Users" hoặc "Just Me"
# Sau đó mở Anaconda Prompt hoặc cmd
conda --version
# Kỳ vọng: conda 24.x.x
```

### Kiểm tra Conda đã cài chưa

```bash
conda --version
which conda       # Linux/Mac
where conda       # Windows (cmd)
```

---

## 🚀 Sử dụng môi trường cho dự án này

### Cách 1: Tạo từ file `environment.yml` (khuyên dùng)

```bash
# Tạo mới environment name=orbit-system từ file cấu hình
conda env create -f environment.yml

# Kích hoạt
conda activate orbit-system

# Kiểm tra
python --version
# Python 3.11.x
pip list
# psutil, websockets, GPUtil
```

### Cách 2: Tạo thủ công nếu bạn muốn custom

```bash
# Tạo environment với Python 3.11
conda create -n orbit-system python=3.11 -y

# Kích hoạt
conda activate orbit-system

# Cài dependencies
pip install -r requirements.txt
```

### Sau khi kích hoạt, chạy backend

```bash
python app.py --port 8080
# Kỳ vọng: WebSocket server starting at ws://127.0.0.1:8080
```

---

## 📦 Khi nào dùng Conda, khi nào dùng Pip?

```
┌──────────────────────────────────────────────────────────────────┐
│                     NGUYÊN TẮC CƠ BẢN                           │
│                                                                  │
│  Ưu tiên Conda → nếu package có sẵn trên conda-forge             │
│  Dùng Pip       → nếu không có trên Conda hoặc cần version mới   │
└──────────────────────────────────────────────────────────────────┘
```

### Bảng quyết định cho dự án này

| Package | Kênh | Lý do |
|---------|------|-------|
| `python` | conda | Conda quản lý Python version tốt hơn |
| `psutil` | conda-forge | Có sẵn, ổn định |
| `websockets` | conda-forge | Có sẵn, ổn định |
| `GPUtil` | pip (PyPI) | Không có trên Conda |

### Cách cài qua Conda

```bash
# Tìm kiếm package trên Conda
conda search psutil

# Cài từ conda-forge (khuyên dùng)
conda install -c conda-forge psutil

# Cài nhiều package cùng lúc
conda install -c conda-forge psutil websockets
```

### Cách cài qua Pip

```bash
# Kích hoạt conda environment TRƯỚC khi dùng pip
conda activate orbit-system

# Cài từ PyPI
pip install GPUtil

# Cài từ requirements.txt
pip install -r requirements.txt
```

> ⚠️ **Lưu ý quan trọng:** Luôn kích hoạt conda environment trước khi dùng `pip install`. 
> Nếu không, pip sẽ cài vào system Python, không phải environment của bạn.

---

## 📋 Các lệnh Conda thường dùng

### Quản lý environment

```bash
# Danh sách tất cả environment
conda env list

# Tạo mới
conda create -n <tên-env> python=3.11

# Kích hoạt
conda activate <tên-env>

# Thoát
conda deactivate

# Xoá
conda env remove -n <tên-env>

# Clone
conda create -n <tên-mới> --clone <tên-cũ>
```

### Quản lý packages

```bash
# Liệt kê packages trong env hiện tại
conda list

# Tìm kiếm
conda search <package-name>

# Cài từ conda-forge
conda install -c conda-forge <package-name>

# Gỡ
conda remove <package-name>

# Cập nhật
conda update <package-name>
```

### Xuất / Nhập environment

```bash
# Xuất ra file (dùng --no-builds để tương thích cross-platform)
conda env export -n orbit-system --no-builds > environment.yml

# Nhập từ file
conda env create -f environment.yml

# Cập nhật từ file (sau khi pull code mới)
conda env update -f environment.yml --prune
```

---

## 💡 Mẹo & Best Practices

### 1. Luôn kích hoạt environment trước khi làm việc

```bash
# Mở terminal mới → luôn chạy lệnh này đầu tiên
conda activate orbit-system
```

### 2. Khi code thay đổi (pull từ GitHub)

```bash
conda env update -f environment.yml --prune
```

### 3. Khi thêm dependency mới

```bash
# Bước 1: Cài thử
conda activate orbit-system
conda install -c conda-forge <thư-viện-mới>
# hoặc
pip install <thư-viện-mới>

# Bước 2: Kiểm tra chạy được không
python app.py

# Bước 3: Cập nhật cả requirements.txt và environment.yml
pip freeze | findstr <thư-viện-mới>  # Windows
pip list --format=freeze | grep <thư-viện-mới>  # Linux/Mac
# Sau đó update cả 2 file

# Bước 4: Re-export environment.yml
conda env export -n orbit-system --no-builds > environment.yml
```

### 4. Không bao giờ dùng `pip install` bên ngoài conda env

```bash
# SAI — sẽ cài vào system Python
pip install psutil

# ĐÚNG — cài vào đúng environment
conda activate orbit-system
pip install psutil
```

### 5. Kiểm tra Python đang dùng từ conda env nào

```bash
which python
# Kỳ vọng: /c/Users/Admin/miniconda3/envs/orbit-system/bin/python
# hoặc: C:\Users\Admin\miniconda3\envs\orbit-system\python.exe
```

---

## 🔍 Debug & Xử lý sự cố

### `conda: command not found`

→ Miniconda chưa được thêm vào PATH. Chạy:
```bash
# Windows cmd:
C:\Users\Admin\miniconda3\Scripts\conda.exe

# Hoặc sửa PATH:
# System Properties → Environment Variables → thêm:
# C:\Users\Admin\miniconda3\Scripts
# C:\Users\Admin\miniconda3
```

### `Python not found` dù đã activate env

```bash
# Kiểm tra Python ở đâu
which python

# Nếu trỏ sai, thử:
conda activate orbit-system
conda install python=3.11 -y
```

### `ImportError: No module named psutil`

```bash
# Kiểm tra đã activate đúng env chưa
conda activate orbit-system

# Kiểm tra package
conda list | findstr psutil

# Nếu chưa có, cài lại
conda install -c conda-forge psutil
```

### `GPUtil not found` / GPU metrics không hoạt động

```bash
# GPUtil chỉ có trên PyPI
conda activate orbit-system
pip install GPUtil

# Nếu không có GPU NVIDIA, app vẫn chạy — frontend tự mock data
```

---

## 📁 File liên quan

| File | Vai trò |
|------|---------|
| `environment.yml` | **Cấu hình Conda chính** — dùng để tái hiện môi trường |
| `requirements.txt` | Pip dependencies — dự phòng nếu không dùng Conda |
| `app.py` | Python backend — chạy với `conda activate orbit-system && python app.py` |

---

## 🔄 Flow làm việc hàng ngày

```bash
# 1. Mở terminal
# 2. Kích hoạt môi trường
conda activate orbit-system

# 3. Nếu mới pull code
conda env update -f environment.yml --prune

# 4. Chạy backend
python app.py --port 8080

# 5. Mở terminal khác → chạy frontend
npm run dev
```

> 📌 **Ghi nhớ:** Mỗi lần mở terminal mới → `conda activate orbit-system` là bước đầu tiên.
