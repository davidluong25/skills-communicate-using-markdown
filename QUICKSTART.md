# Quick Start - Claude Monitoring System

## 🚀 Quick Setup (2 minutes)

### 1. Install Dependencies

```bash
npm install
```

### 2. Configure Environment

```bash
# Copy example configuration
cp .env.example .env

# Edit with your settings (optional - defaults will work for local testing)
nano .env
```

### 3. Start the System

**Option A: Using the startup script (Recommended)**

```bash
./start-monitoring.sh start
```

**Option B: Manual start (two terminals)**

Terminal 1 - Server:
```bash
npm run server
```

Terminal 2 - Watcher:
```bash
npm start
```

## ✅ Verify It's Working

1. Check the server is running:
   ```bash
   curl http://localhost:8386/health
   ```

2. Create a test file:
   ```bash
   mkdir -p ~/.claude/tasks/test-team
   echo '{"task": "test"}' > ~/.claude/tasks/test-team/test.json
   ```

3. Watch the watcher logs - you should see:
   ```
   [add] Phát hiện thay đổi: /home/user/.claude/tasks/test-team/test.json
   ✅ Upload thành công: test.json - Status: 200
   ```

## 📊 Common Commands

```bash
# Start everything
./start-monitoring.sh start

# Check status
./start-monitoring.sh status

# View logs
./start-monitoring.sh logs server
./start-monitoring.sh logs watcher

# Stop everything
./start-monitoring.sh stop

# Restart
./start-monitoring.sh restart
```

## 🎯 What's Next?

- **Integrate with Claude Code**: When you use Claude Code CLI, files will automatically sync
- **Build Dashboard**: Create a UI to visualize the synced data
- **Add Webhooks**: Trigger actions when files change
- **Custom Handlers**: Process specific file types differently

## 📚 Full Documentation

See [docs/claude-monitoring.md](docs/claude-monitoring.md) for complete documentation.

## 🐛 Troubleshooting

**"node: command not found"**
- Install Node.js v14+ from https://nodejs.org

**"npm install fails"**
- Try: `rm -rf node_modules package-lock.json && npm install`

**"Port 8386 already in use"**
- Edit `.env` and change `PORT=8386` to another port

**"Cannot find module 'chokidar'"**
- Run `npm install` to install dependencies

## 🔐 Security Note

The `.env` file contains sensitive information (API_KEY) and is excluded from git. Never commit this file!

---

**Happy Monitoring! 🦍**
