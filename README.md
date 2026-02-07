# OrcMate 🦍

> **Your Keyboard-Only AI Pair Programmer Manager**

OrcMate is a lightweight orchestrator that creates isolated, persistent, and keyboard-first development environments for AI-assisted coding. Built on top of **Git Worktrees** and **Tmux**, OrcMate eliminates the fear of AI agents messing up your main branch while keeping you in flow state.

---

## 🎯 The Problem

Modern AI coding assistants (like Claude Code, GitHub Copilot, Cursor) are powerful, but developers face three major challenges:

1. **Fear of Branch Contamination:** "What if the AI breaks my main branch?"
2. **Lost Session State:** "I closed my terminal and lost my AI conversation!"
3. **Mouse Distractions:** "Constantly switching between mouse and keyboard kills my flow"
4. **Monitoring AI Work:** "How do I track what my AI agents are doing across multiple sessions?"

## ✨ The Solution

OrcMate solves these with four core principles:

### 1. **Isolated Environments**
- Each task gets its own **Git Worktree** (separate directory + branch)
- Your main branch stays pristine and untouched
- Experiment freely without fear

### 2. **Persistent Sessions**
- **Tmux Sessions** that survive disconnects
- Resume your AI conversation after closing the terminal
- Perfect for remote development over SSH

### 3. **Keyboard-Only Workflow**
- Zero mouse support (by design!)
- Vim-style navigation everywhere
- Pure keyboard = Pure flow state

### 4. **Real-Time Monitoring** 🆕
- **File Watcher** automatically syncs Claude Code changes
- **Dashboard** for centralized monitoring across all agents
- Track tasks, logs, and configurations in real-time

---

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/orcmate.git
cd orcmate

# Run the installer
./install.sh
```

### Your First Session

```bash
# Navigate to your project
cd /path/to/your/project

# Start an OrcMate session
orc start fix-login-bug

# You'll get:
# - A new git worktree at .worktrees/fix-login-bug
# - A new branch: agent/fix-login-bug
# - A tmux session with 2 panes:
#   Pane 1 (Top, 70%): AI Assistant
#   Pane 2 (Bottom, 30%): Testing/Monitoring

# Work with AI, test your changes, commit

# When done, clean up
orc clean fix-login-bug
```

---

## 📚 Core Commands

| Command | Description |
|---------|-------------|
| `orc start <task-name>` | Create a new isolated environment |
| `orc clean <task-name>` | Remove environment (tmux session + worktree + branch) |
| `orc list` | List all active environments |
| `orc help` | Show all commands and usage |

---

## ⌨️ Essential Keyboard Shortcuts

OrcMate uses **Ctrl+Space** as the tmux prefix (more ergonomic than Ctrl+b).

| Shortcut | Action |
|----------|--------|
| `Ctrl+Space h/j/k/l` | Navigate between panes (Vim style) |
| `Ctrl+Space H/J/K/L` | Resize panes |
| `Ctrl+Space \|` | Split window vertically |
| `Ctrl+Space -` | Split window horizontally |
| `Ctrl+Space [` | Enter copy mode (then use `v` to select, `y` to copy) |
| `Ctrl+Space d` | Detach from session (keeps running!) |
| `Ctrl+Space ?` | Show all keybindings |

**Pro Tip:** Run `orc_keys` (if using zsh addon) for a full reference.

---

## 🧠 Philosophy: Why No Mouse?

### The Science of Flow State

Flow state (being "in the zone") requires:
1. **Uninterrupted focus** - Mouse switching breaks this
2. **Muscle memory** - Keyboard shortcuts become automatic
3. **Visual focus** - No searching for buttons/icons

### The OrcMate Way

```
Traditional Workflow:          OrcMate Workflow:
┌─────────────────┐           ┌─────────────────┐
│ Type code       │           │ Type code       │
│ Reach for mouse │  ❌       │ Ctrl+Space j    │  ✅
│ Click pane      │           │ Run test        │
│ Find terminal   │           │ Ctrl+Space k    │
│ Type command    │           │ Continue coding │
│ Reach for mouse │           │ [Flow state!]   │
│ ...             │           └─────────────────┘
└─────────────────┘
```

**Result:** 10x faster context switching, zero mental overhead.

---

## 🌳 Why Git Worktrees?

### The Traditional Problem

```bash
# You're on main branch, working
git checkout feature-branch  # ERROR: Uncommitted changes!

# So you stash...
git stash
git checkout feature-branch
# ... work on feature ...
git checkout main
git stash pop  # Merge conflicts! 😱
```

### The OrcMate Solution

```bash
# Your project structure with OrcMate:
project/
  ├── .git/                  # Shared git database
  ├── src/                   # Your main branch (untouched!)
  └── .worktrees/            # Isolated environments
      ├── fix-login/         # Branch: agent/fix-login
      ├── new-feature/       # Branch: agent/new-feature
      └── refactoring/       # Branch: agent/refactoring

# Work on multiple tasks without conflicts!
# No more stashing!
# No more "wait, which branch am I on?"
```

---

## 🎨 Example Workflow

### Scenario: Fixing a Login Bug

```bash
# 1. Start the session
$ orc start fix-login-bug

# You're now in .worktrees/fix-login-bug on branch agent/fix-login-bug

# 2. In Pane 1 (Top): Chat with AI
You: "Help me fix the login bug where null emails crash the app"
AI: "I'll add null checking and write tests..."
[AI makes changes]

# 3. In Pane 2 (Bottom): Test the changes
$ npm test
✓ login with valid email
✓ login with null email (new!)
All tests passed!

# 4. Review and commit
$ git add .
$ git commit -m "Fix: Handle null email in login"

# 5. Merge to main (outside OrcMate)
$ cd /path/to/main/repo
$ git merge agent/fix-login-bug
$ git push

# 6. Clean up
$ orc clean fix-login-bug
✓ Environment cleaned up!
```

---

## 📖 Documentation

- **[Workflow Guide](docs/workflow.md)** - Complete step-by-step usage guide
- **[Architecture](docs/architecture.md)** - How OrcMate works under the hood
- **[Claude Monitoring System](docs/claude-monitoring.md)** - File watcher and dashboard integration
- **[Tmux Config](config/.tmux.conf)** - Customizable keyboard shortcuts

---

## 🔍 Claude Code Monitoring System

OrcMate now includes a powerful file monitoring system that automatically tracks and syncs Claude Code changes to a centralized dashboard.

### Quick Setup

```bash
# 1. Install dependencies
npm install

# 2. Configure environment
cp .env.example .env
# Edit .env with your API key

# 3. Start the server (Terminal 1)
npm run server

# 4. Start the watcher (Terminal 2)
npm start
```

### What It Does

- **Watches** `~/.claude/teams/` and `~/.claude/tasks/` directories
- **Detects** file additions and changes in real-time
- **Uploads** changes to server with authentication
- **Displays** all agent activities in a centralized dashboard

### Architecture

```
Claude Code → File Watcher → API Server → Dashboard UI
   (local)      (chokidar)    (Express)    (React/Vue)
```

For complete documentation, see **[Claude Monitoring Guide](docs/claude-monitoring.md)**.

---

## 🛠️ Requirements

- **Git** (with worktree support, v2.15+)
- **Tmux** (v2.6+)
- **Bash** (for the CLI script)
- **Node.js** (v14+ for monitoring system)
- **npm** (for Node.js dependencies)
- **Zsh** (optional, for shell integration)

---

## 🎯 Use Cases

### Perfect For:
- ✅ AI-assisted development (Claude, Copilot, Cursor)
- ✅ Experimenting with risky refactors
- ✅ Working on multiple features in parallel
- ✅ Remote development over SSH
- ✅ Pair programming with AI
- ✅ Learning new codebases safely

### Not Ideal For:
- ❌ GUI-heavy development (OrcMate is CLI-first)
- ❌ Single-file quick edits (use your regular editor)
- ❌ Non-git projects (requires git repository)

---

## 🔧 Advanced Features

### Multiple Parallel Sessions

```bash
orc start feature-auth
orc start bugfix-validation
orc start refactor-api

# Switch between them:
Ctrl+Space )  # Next session
Ctrl+Space (  # Previous session

# Or use the helper (with zsh addon):
orc_switch feature-auth
```

### Detach and Resume

```bash
# Day 1: Start work
orc start complex-feature
[... work with AI ...]
Ctrl+Space d  # Detach (keeps running!)

# Day 2: Resume
tmux attach -t complex-feature
[... continue working ...]
```

### Shell Integration (Zsh)

If you use zsh, the `.zshrc_addon` provides:
- Aliases: `os`, `oc`, `ol`
- Prompt indicator: Shows current task
- Helper functions: `orc_status`, `orc_commit`, `orc_keys`

---

## 🤝 Contributing

OrcMate is designed to be minimal and focused. If you have ideas:

1. Keep it keyboard-first
2. Keep it lightweight (no heavy dependencies)
3. Keep it simple (bash + git + tmux)

---

## 📜 License

MIT License - See [LICENSE](LICENSE) file for details.

---

## 🙏 Credits

Built with inspiration from:
- **Git Worktrees** - For true isolation
- **Tmux** - For persistence and layout management
- **Vim** - For keyboard-first philosophy
- **AI Assistants** - For making us rethink our workflows

---

## 💬 Philosophy

> "The best tools disappear. OrcMate gets out of your way so you can focus on what matters: building great software with AI as your pair programmer."

**OrcMate doesn't make you a better coder. It removes the friction that prevents you from becoming one.**

---

**Built with 💚 for developers who love keyboards, hate mice, and trust AI.**

🦍 **OrcMate** - Your Keyboard-Only AI Pair Programmer Manager
