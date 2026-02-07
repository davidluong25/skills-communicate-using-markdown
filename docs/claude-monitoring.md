# Claude Code File Monitoring System

> **Hệ thống theo dõi và đồng bộ thay đổi từ Claude Code lên Dashboard**

## 📖 Tổng quan

Hệ thống này cho phép OrcMate theo dõi tự động các thay đổi từ Claude Code (lưu file local) và đồng bộ lên server để quản lý qua UI. Đây là bước chuyển đổi lớn từ mô hình CLI/tmux sang Dashboard monitoring.

### Kiến trúc hệ thống

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│  Claude Code    │         │  File Watcher    │         │  OrcMate Server │
│  (CLI/Agent)    │ ──────> │  (chokidar)      │ ──────> │  (Dashboard)    │
│  ~/.claude/     │  write  │  watcher.js      │  POST   │  server.js      │
└─────────────────┘         └──────────────────┘         └─────────────────┘
                                    │                              │
                                    │                              │
                                    └──────────────────────────────┘
                                            WebSocket/HTTP
```

## 🚀 Cài đặt

### Bước 1: Cài đặt dependencies

```bash
cd /path/to/orcmate
npm install
```

Các package được cài:
- **chokidar**: Theo dõi file thay đổi hiệu quả
- **axios**: Gửi HTTP requests
- **form-data**: Upload file dạng multipart/form-data
- **dotenv**: Quản lý biến môi trường
- **express**: Web server framework
- **multer**: Xử lý file uploads

### Bước 2: Cấu hình môi trường

Tạo file `.env` từ template:

```bash
cp .env.example .env
```

Chỉnh sửa `.env` với thông tin của bạn:

```env
SERVER_URL=http://localhost:8386/api/upload
API_KEY=your-secure-api-key-here
PORT=8386
```

**⚠️ Lưu ý:** File `.env` chứa thông tin nhạy cảm và đã được thêm vào `.gitignore`

## 📂 Cấu trúc thư mục theo dõi

Watcher theo dõi 2 thư mục chính:

```
~/.claude/
├── teams/
│   └── {team-name}/
│       └── config.json       # Cấu hình team
└── tasks/
    └── {team-name}/
        ├── task-1.json       # Thông tin task
        ├── task-2.json
        └── logs/             # Logs của các task
```

## 🎯 Cách sử dụng

### Chạy Server (Terminal 1)

Server nhận và xử lý file uploads:

```bash
npm run server
```

Output:
```
🚀 OrcMate Server đang chạy tại http://localhost:8386
📡 API Endpoint: http://localhost:8386/api/upload
🔐 API Key: your-secur...
```

### Chạy File Watcher (Terminal 2)

Watcher theo dõi thay đổi và tự động upload:

```bash
npm start
# hoặc
npm run dev
```

Output:
```
👀 Đang theo dõi thay đổi từ Claude...
📂 Theo dõi các thư mục:
   - /home/user/.claude/teams
   - /home/user/.claude/tasks
```

### Workflow hoàn chỉnh

1. **Khởi động Server:**
   ```bash
   npm run server
   ```

2. **Khởi động Watcher (terminal mới):**
   ```bash
   npm start
   ```

3. **Làm việc với Claude Code:**
   ```bash
   claude code "Fix the login bug"
   ```

4. **Tự động đồng bộ:**
   - Claude ghi file vào `~/.claude/tasks/`
   - Watcher phát hiện thay đổi
   - Upload lên server tự động
   - Dashboard cập nhật real-time

## 🔧 API Endpoints

### POST `/api/upload`

Upload file từ watcher lên server.

**Headers:**
```
Authorization: Bearer {API_KEY}
X-Claude-Hook: true
Content-Type: multipart/form-data
```

**Body (form-data):**
```
file: [File Binary]
eventType: "add" | "change"
filePath: "/path/to/original/file"
```

**Response:**
```json
{
  "success": true,
  "message": "File received and processed",
  "data": {
    "eventType": "add",
    "originalPath": "/home/user/.claude/tasks/my-task.json",
    "savedPath": "uploads/1234567890-my-task.json",
    "size": 1024,
    "timestamp": "2024-01-01T12:00:00.000Z"
  }
}
```

### GET `/health`

Kiểm tra trạng thái server.

**Response:**
```json
{
  "status": "ok",
  "service": "OrcMate Claude Watcher Server",
  "timestamp": "2024-01-01T12:00:00.000Z"
}
```

## ⚙️ Cấu hình nâng cao

### Thay đổi thư mục theo dõi

Chỉnh sửa `watcher.js`:

```javascript
const WATCH_PATHS = [
    path.join(os.homedir(), '.claude', 'teams'),
    path.join(os.homedir(), '.claude', 'tasks'),
    // Thêm thư mục khác nếu cần
    path.join(os.homedir(), '.claude', 'logs')
];
```

### Tùy chỉnh debounce time

Điều chỉnh thời gian chờ trước khi upload (tránh upload file chưa ghi xong):

```javascript
awaitWriteFinish: {
    stabilityThreshold: 2000, // Tăng lên 2 giây
    pollInterval: 100
}
```

### Bỏ qua file patterns

Thêm pattern vào `ignored`:

```javascript
ignored: [
    /(^|[\/\\])\../,     // Dotfiles
    /\.tmp$/,            // Temp files
    /~$/                 // Backup files
]
```

## 🔐 Bảo mật

### API Key

- **Không commit** file `.env` vào git
- Sử dụng key mạnh và unique cho mỗi môi trường
- Rotation key định kỳ (recommend: 90 ngày)

### HTTPS

Trong production, luôn dùng HTTPS:

```env
SERVER_URL=https://api.orcmate.com/api/upload
```

### Network Security

- Chỉ cho phép IP/domain tin cậy
- Sử dụng firewall rules
- Rate limiting trên server

## 🐛 Troubleshooting

### Watcher không phát hiện thay đổi

**Nguyên nhân:** Thư mục `~/.claude` chưa tồn tại

**Giải pháp:**
```bash
mkdir -p ~/.claude/teams
mkdir -p ~/.claude/tasks
```

### Upload thất bại với lỗi 401

**Nguyên nhân:** API Key không đúng

**Giải pháp:**
1. Kiểm tra file `.env` có tồn tại
2. Đảm bảo `API_KEY` giống nhau ở watcher và server
3. Restart cả watcher và server

### File bị upload nhiều lần

**Nguyên nhân:** `awaitWriteFinish` chưa được cấu hình

**Giải pháp:** Đã có sẵn trong code, tăng `stabilityThreshold` nếu vẫn xảy ra

### Server không nhận được file

**Nguyên nhân:** Server URL sai hoặc server chưa chạy

**Giải pháp:**
```bash
# Kiểm tra server
curl http://localhost:8386/health

# Kiểm tra log của watcher
# Xem có error message không
```

## 📊 Monitoring và Logs

### Xem logs của watcher

```bash
npm start 2>&1 | tee watcher.log
```

### Xem logs của server

```bash
npm run server 2>&1 | tee server.log
```

### Log format

**Watcher:**
```
[add] Phát hiện thay đổi: /home/user/.claude/tasks/task-1.json
✅ Upload thành công: task-1.json - Status: 200
```

**Server:**
```
📥 Nhận được file từ Claude Watcher:
   Event: add
   Path: /home/user/.claude/tasks/task-1.json
   Size: 1024 bytes
```

## 🚀 Production Deployment

### Chạy như một service (systemd)

Tạo file `/etc/systemd/system/orcmate-watcher.service`:

```ini
[Unit]
Description=OrcMate Claude Watcher
After=network.target

[Service]
Type=simple
User=your-user
WorkingDirectory=/path/to/orcmate
ExecStart=/usr/bin/node watcher.js
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Tạo file `/etc/systemd/system/orcmate-server.service`:

```ini
[Unit]
Description=OrcMate Server
After=network.target

[Service]
Type=simple
User=your-user
WorkingDirectory=/path/to/orcmate
ExecStart=/usr/bin/node server.js
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable và start:

```bash
sudo systemctl enable orcmate-watcher
sudo systemctl enable orcmate-server
sudo systemctl start orcmate-watcher
sudo systemctl start orcmate-server
```

### Chạy với PM2

```bash
npm install -g pm2

# Start services
pm2 start watcher.js --name orcmate-watcher
pm2 start server.js --name orcmate-server

# Auto-start on boot
pm2 startup
pm2 save
```

## 🎨 Tích hợp Dashboard

Dashboard có thể được xây dựng bằng:
- **React/Vue/Svelte** cho frontend
- **WebSocket** để cập nhật real-time
- **Database** (PostgreSQL/MongoDB) để lưu lịch sử

### Ví dụ cập nhật UI

Server có thể emit events qua WebSocket:

```javascript
// Trong server.js
const io = require('socket.io')(server);

app.post('/api/upload', requireAuth, upload.single('file'), (req, res) => {
    // ... xử lý file ...
    
    // Emit to connected clients
    io.emit('file-updated', {
        eventType,
        filePath,
        timestamp: new Date()
    });
    
    res.json({ success: true });
});
```

Frontend lắng nghe:

```javascript
// Dashboard client
socket.on('file-updated', (data) => {
    console.log('New file:', data);
    // Update UI
});
```

## 📚 Tham khảo

- [Chokidar Documentation](https://github.com/paulmillr/chokidar)
- [Express.js Guide](https://expressjs.com/)
- [Multer File Upload](https://github.com/expressjs/multer)

## 🤝 Contributing

Nếu bạn muốn đóng góp:

1. Fork repository
2. Tạo branch mới (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Mở Pull Request

## 📝 License

MIT License - See [LICENSE](../LICENSE) file for details.

---

**Built with 💚 for OrcMate - Your Keyboard-Only AI Pair Programmer Manager**
