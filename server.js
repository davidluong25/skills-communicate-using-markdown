require('dotenv').config();
const express = require('express');
const multer = require('multer');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = process.env.PORT || 8386;
const API_KEY = process.env.API_KEY || 'your-secret-api-key';

// Cấu hình multer để lưu file tạm
const storage = multer.diskStorage({
    destination: (req, file, cb) => {
        const uploadDir = path.join(__dirname, 'uploads');
        if (!fs.existsSync(uploadDir)) {
            fs.mkdirSync(uploadDir, { recursive: true });
        }
        cb(null, uploadDir);
    },
    filename: (req, file, cb) => {
        const timestamp = Date.now();
        const originalName = file.originalname;
        cb(null, `${timestamp}-${originalName}`);
    }
});

const upload = multer({ 
    storage: storage,
    limits: { fileSize: 50 * 1024 * 1024 } // 50MB max
});

// Middleware check Auth
const requireAuth = (req, res, next) => {
    const authHeader = req.headers.authorization;
    
    if (!authHeader) {
        return res.status(401).json({ error: 'No authorization header' });
    }
    
    const token = authHeader.split(' ')[1];
    
    if (token === API_KEY) {
        return next();
    }
    
    return res.status(401).json({ error: 'Unauthorized - Invalid API key' });
};

// Middleware để parse JSON
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Health check endpoint
app.get('/health', (req, res) => {
    res.json({ 
        status: 'ok', 
        service: 'OrcMate Claude Watcher Server',
        timestamp: new Date().toISOString()
    });
});

// Endpoint nhận file từ watcher
app.post('/api/upload', requireAuth, upload.single('file'), (req, res) => {
    try {
        const { eventType, filePath } = req.body;
        const file = req.file;
        
        if (!file) {
            return res.status(400).json({ error: 'No file uploaded' });
        }
        
        console.log(`📥 Nhận được file từ Claude Watcher:`);
        console.log(`   Event: ${eventType}`);
        console.log(`   Path: ${filePath}`);
        console.log(`   Size: ${file.size} bytes`);
        console.log(`   Saved to: ${file.path}`);
        
        // TODO: Xử lý file tại đây
        // - Parse JSON nếu là config.json
        // - Lưu vào database
        // - Cập nhật UI/Dashboard
        // - Trigger webhooks
        
        res.json({ 
            success: true, 
            message: 'File received and processed',
            data: {
                eventType,
                originalPath: filePath,
                savedPath: file.path,
                size: file.size,
                timestamp: new Date().toISOString()
            }
        });
    } catch (error) {
        console.error('Error processing upload:', error);
        res.status(500).json({ 
            error: 'Failed to process file',
            message: error.message 
        });
    }
});

// Alternative endpoint name for compatibility
app.post('/api/claude-webhook', requireAuth, upload.single('file'), (req, res) => {
    try {
        const { eventType, filePath } = req.body;
        const file = req.file;
        
        if (!file) {
            return res.status(400).json({ error: 'No file uploaded' });
        }
        
        console.log(`📥 [Webhook] Nhận được file từ Claude:`);
        console.log(`   Event: ${eventType}`);
        console.log(`   Path: ${filePath}`);
        
        res.json({ 
            success: true, 
            message: 'File processed via webhook',
            eventType,
            filePath
        });
    } catch (error) {
        console.error('Webhook error:', error);
        res.status(500).json({ error: 'Webhook processing failed' });
    }
});

// Start server
app.listen(PORT, () => {
    console.log(`🚀 OrcMate Server đang chạy tại http://localhost:${PORT}`);
    console.log(`📡 API Endpoint: http://localhost:${PORT}/api/upload`);
    console.log(`🔐 API Key: ${API_KEY.substring(0, 10)}...`);
    console.log(`\n💡 Để kiểm tra: curl http://localhost:${PORT}/health`);
});

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('\n🛑 Đang dừng server...');
    process.exit(0);
});
