# Hướng Dẫn Sử Dụng Markdown Analyzer (Tiếng Việt)

## Giới Thiệu

Markdown Analyzer là công cụ phân tích và quản lý file markdown trong dự án của bạn. Công cụ này giúp bạn:
- Phân tích tất cả file markdown
- Xem thống kê chi tiết
- Đọc file markdown với highlight
- Kiểm tra cấu trúc dự án

## Cài Đặt

Công cụ đã được cài đặt sẵn trong thư mục `bin/`. Để sử dụng:

```bash
# Đảm bảo file có quyền thực thi
chmod +x bin/md-analyzer

# Chạy trực tiếp
./bin/md-analyzer help
```

## Các Lệnh Cơ Bản

### 1. Liệt Kê Tất Cả File Markdown

```bash
./bin/md-analyzer list
```

Hiển thị danh sách tất cả file `.md` trong dự án với kích thước.

**Kết quả:**
```
Tìm thấy 10 file:
  1. README.md (8.0K)
  2. docs/architecture.md (16K)
  3. docs/workflow.md (8.0K)
  ...
```

### 2. Phân Tích Chi Tiết

```bash
./bin/md-analyzer analyze
```

Phân tích toàn bộ file markdown và hiển thị:
- Số dòng (LINES)
- Số từ (WORDS)
- Số ký tự (CHARS)
- Số tiêu đề (HEADERS)
- Số code block (CODE)
- Số link (LINKS)
- Số hình ảnh (IMAGES)

**Kết quả:**
```
FILE                                        LINES    WORDS      CHARS  HEADERS     CODE    LINKS   IMAGES
----                                        -----    -----      -----  -------     ----    -----   ------
README.md                                     313     1223       8147       62        8        4        0
docs/architecture.md                          499     1705      13557       70       18        4        0
----                                        -----    -----      -----  -------     ----    -----   ------
TOTAL                                        1399     5483      38588      221       56       24        1
```

### 3. Xem File Cụ Thể

```bash
./bin/md-analyzer view README.md
./bin/md-analyzer view docs/workflow.md
```

Hiển thị nội dung file với màu sắc (nếu có công cụ hỗ trợ).

### 4. Thống Kê Tổng Quan

```bash
./bin/md-analyzer stats
```

Hiển thị:
- Tổng số file markdown
- Tổng kích thước
- Kích thước trung bình
- File lớn nhất
- Các file tài liệu quan trọng (README, LICENSE, docs/)

**Kết quả:**
```
Thống kê dự án:
  Tổng số file markdown: 10
  Tổng kích thước: 37KB
  Kích thước trung bình: 3KB
  File lớn nhất: docs/architecture.md (13KB)

Tài liệu:
  ✓ README.md
  ✓ LICENSE
  ✓ thư mục docs/
```

### 5. Xem Cấu Trúc Dự Án

```bash
./bin/md-analyzer tree
```

Hiển thị cây thư mục của dự án.

### 6. Trợ Giúp

```bash
./bin/md-analyzer help
```

Hiển thị hướng dẫn sử dụng đầy đủ.

## Ví Dụ Sử Dụng

### Kiểm Tra Tài Liệu Dự Án

```bash
# Bước 1: Xem thống kê tổng quan
./bin/md-analyzer stats

# Bước 2: Liệt kê tất cả file
./bin/md-analyzer list

# Bước 3: Phân tích chi tiết
./bin/md-analyzer analyze

# Bước 4: Đọc file quan trọng
./bin/md-analyzer view README.md
```

### Tìm File Cần Cập Nhật

```bash
# Phân tích để tìm file có ít nội dung
./bin/md-analyzer analyze

# Xem file cần cập nhật
./bin/md-analyzer view docs/some-file.md
```

### Kiểm Tra Trước Khi Commit

```bash
# Xem thống kê
./bin/md-analyzer stats

# Phân tích thay đổi
./bin/md-analyzer analyze

# Commit nếu OK
git add .
git commit -m "Cập nhật tài liệu"
```

## Tính Năng

### Tự Động Tìm File
- Tự động tìm tất cả file `.md` trong dự án
- Bỏ qua thư mục `.git/`, `node_modules/`, `.worktrees/`

### Phân Tích Thông Minh
- **Tiêu đề**: Đếm tất cả cấp độ (#, ##, ###, ...)
- **Code block**: Đếm cặp dấu ``` (ba dấu backtick)
- **Link**: Phát hiện link markdown `[text](url)`
- **Hình ảnh**: Phát hiện hình ảnh `![alt](url)`

### Hiển Thị Có Màu
- 🔵 Xanh: Thông tin
- 🟢 Xanh lá: Thành công
- 🟡 Vàng: Cảnh báo
- 🔴 Đỏ: Lỗi

## Quy Trình Làm Việc Nhanh

```bash
# 1. Xem tổng quan
./bin/md-analyzer stats

# 2. Xem danh sách file
./bin/md-analyzer list

# 3. Phân tích chi tiết
./bin/md-analyzer analyze

# 4. Đọc file cụ thể
./bin/md-analyzer view README.md
```

## Xuất Kết Quả Ra File

```bash
# Lưu kết quả phân tích
./bin/md-analyzer analyze > phan-tich.txt

# Lưu thống kê
./bin/md-analyzer stats > thong-ke.txt

# Xem sau
cat phan-tich.txt
```

## Khắc Phục Sự Cố

### Lỗi "Permission denied"

```bash
chmod +x bin/md-analyzer
```

### Không tìm thấy file markdown

Đảm bảo bạn đang ở thư mục gốc của dự án:
```bash
cd /path/to/your/project
./bin/md-analyzer list
```

### Cài đặt syntax highlighting

Để có highlight đẹp hơn khi xem file:

```bash
# macOS
brew install bat

# Ubuntu/Debian
apt install bat

# Hoặc dùng Python
pip install pygments
```

## Tài Liệu Chi Tiết

Xem thêm:
- [Hướng dẫn đầy đủ (English)](md-analyzer-guide.md)
- [Quick Start (English)](QUICKSTART.md)

## Câu Hỏi Thường Gặp

**Q: File nào sẽ được phân tích?**  
A: Tất cả file có đuôi `.md` trong dự án, ngoại trừ thư mục `.git/`, `node_modules/`, `.worktrees/`.

**Q: Làm sao để xem một file cụ thể?**  
A: Dùng lệnh `./bin/md-analyzer view <tên-file.md>`

**Q: Kết quả phân tích có chính xác không?**  
A: Có, công cụ đếm chính xác số dòng, từ, ký tự, tiêu đề, code block, link và hình ảnh.

**Q: Có thể tích hợp vào CI/CD không?**  
A: Có, bạn có thể chạy các lệnh trong pipeline CI/CD để kiểm tra tài liệu tự động.

## Liên Hệ

Nếu có vấn đề hoặc câu hỏi:
- Xem source code trong `bin/md-analyzer`
- Mở issue trong repository
- Đọc tài liệu hướng dẫn

---

**Phiên bản:** 1.0.0  
**Ngôn ngữ:** Bash  
**Yêu cầu:** Git, Bash (đã có sẵn trên hầu hết hệ thống)
