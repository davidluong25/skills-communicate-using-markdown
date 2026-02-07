require('dotenv').config();
const chokidar = require('chokidar');
const axios = require('axios');
const fs = require('fs');
const path = require('path');
const FormData = require('form-data');
const os = require('os');

// --- CẤU HÌNH ---
const SERVER_URL = process.env.SERVER_URL || 'http://localhost:8386/api/upload';
const API_KEY = process.env.API_KEY || 'your-secret-api-key';

// Đường dẫn cần theo dõi (Sử dụng os.homedir() để lấy đường dẫn ~)
// Bạn có thể dùng glob pattern để chỉ định rõ file
const WATCH_PATHS = [
    path.join(os.homedir(), '.claude', 'teams'), // Theo dõi config teams
    path.join(os.homedir(), '.claude', 'tasks')  // Theo dõi tasks/logs
];

// --- HÀM GỬI FILE ---
const uploadFile = async (filePath, eventType) => {
    try {
        // Bỏ qua các file tạm hoặc file hệ thống nếu cần
        if (path.basename(filePath).startsWith('.')) return;

        console.log(`[${eventType}] Phát hiện thay đổi: ${filePath}`);

        const form = new FormData();
        form.append('file', fs.createReadStream(filePath));
        form.append('eventType', eventType); // 'add' hoặc 'change'
        form.append('filePath', filePath);   // Gửi kèm đường dẫn gốc để server biết file nào

        const response = await axios.post(SERVER_URL, form, {
            headers: {
                ...form.getHeaders(),
                'Authorization': `Bearer ${API_KEY}`,
                'X-Claude-Hook': 'true'
            },
            maxContentLength: Infinity,
            maxBodyLength: Infinity
        });

        console.log(`✅ Upload thành công: ${path.basename(filePath)} - Status: ${response.status}`);
    } catch (error) {
        console.error(`❌ Upload thất bại [${path.basename(filePath)}]:`, error.message);
        if (error.response) {
            console.error('Server response:', error.response.data);
        }
    }
};

// --- KHỞI TẠO WATCHER ---
const watcher = chokidar.watch(WATCH_PATHS, {
    persistent: true,
    ignoreInitial: true, // Không upload toàn bộ file cũ khi mới chạy script
    ignored: /(^|[\/\\])\../, // Bỏ qua dotfiles (ví dụ .DS_Store)
    awaitWriteFinish: {
        stabilityThreshold: 1000, // Đợi 1s sau khi file ngừng ghi mới upload
        pollInterval: 100
    }
});

// Gắn sự kiện (Hooks)
watcher
    .on('add', filePath => uploadFile(filePath, 'add'))
    .on('change', filePath => uploadFile(filePath, 'change'))
    .on('ready', () => {
        console.log('👀 Đang theo dõi thay đổi từ Claude...');
        console.log(`📂 Theo dõi các thư mục:`);
        WATCH_PATHS.forEach(p => console.log(`   - ${p}`));
    })
    .on('error', error => console.error(`Watcher error: ${error}`));

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('\n🛑 Đang dừng watcher...');
    watcher.close().then(() => {
        console.log('✓ Watcher đã dừng');
        process.exit(0);
    });
});
