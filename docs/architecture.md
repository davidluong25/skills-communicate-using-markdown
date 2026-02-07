# OrcMate Architecture

## 🏗️ System Design & Implementation Details

This document explains how OrcMate works under the hood, its design decisions, and technical implementation.

---

## 📐 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        OrcMate CLI                          │
│                       (bin/orc)                             │
│  Command Router: start | clean | list                      │
└────────────┬────────────────────────────┬───────────────────┘
             │                            │
             ▼                            ▼
    ┌────────────────┐          ┌────────────────────┐
    │   Git Layer    │          │    Tmux Layer      │
    │                │          │                    │
    │ • Worktrees    │          │ • Sessions         │
    │ • Branches     │          │ • Panes            │
    │ • .env Copy    │          │ • Layouts          │
    └────────────────┘          └────────────────────┘
             │                            │
             └──────────┬─────────────────┘
                        ▼
              ┌──────────────────┐
              │  Isolated Env    │
              │  .worktrees/     │
              │    └── task/     │
              │       ├── code   │
              │       └── .env   │
              └──────────────────┘
```

---

## 🧩 Core Components

### 1. CLI Entry Point (`bin/orc`)

**Purpose:** Single binary that orchestrates all OrcMate operations.

**Key Functions:**
- `orc_start()` - Creates environment
- `orc_clean()` - Destroys environment
- `orc_list()` - Shows active environments
- Error handling and validation

**Design Decisions:**
- Pure Bash script (no external dependencies beyond git/tmux)
- Idempotent operations (safe to run multiple times)
- Clear error messages with color coding
- Fail-fast with `set -euo pipefail`

---

### 2. Git Worktree Layer

**What are Git Worktrees?**

Git worktrees allow you to have multiple working directories from the same repository, each checked out to a different branch.

**Traditional Problem:**
```bash
# Traditional workflow problems:
git checkout feature-branch    # Switches entire repo
# Uncommitted changes? Stash them first!
# Running servers? Kill them first!
# Context switch = Lost flow state
```

**OrcMate Solution with Worktrees:**
```bash
# Each task gets its own directory:
project/
  ├── .git/              # Main git database
  ├── main-code/         # Your main branch (untouched)
  └── .worktrees/
      ├── feature-a/     # Branch: agent/feature-a
      ├── bugfix-b/      # Branch: agent/bugfix-b
      └── refactor-c/    # Branch: agent/refactor-c

# No more checkout conflicts!
# No more stashing!
# No more "wait, which branch am I on?"
```

**Implementation:**
```bash
# Create worktree (from bin/orc)
git worktree add -b "${BRANCH_PREFIX}/${task_name}" \
                 "${WORKTREE_DIR}/${task_name}"

# This creates:
# 1. New directory at .worktrees/<task-name>
# 2. New branch: agent/<task-name>
# 3. Checkout of that branch in the worktree directory
```

**Why `.worktrees/` Directory?**
- Convention: Keep all worktrees in one place
- Gitignored: Never committed to main repo
- Organized: Easy to find and manage
- Parallel: Multiple tasks, zero conflicts

**Branch Naming: `agent/*`**
- Clear intent: This branch is for AI-assisted work
- Easy filtering: `git branch | grep agent/`
- Namespace separation: Won't conflict with your `feature/*` branches

---

### 3. Tmux Session Layer

**What is Tmux?**

Tmux (Terminal Multiplexer) is like a window manager for your terminal. Think of it as "browser tabs for CLI".

**Why Tmux for OrcMate?**

1. **Persistence:** Sessions survive disconnections
   ```bash
   # SSH from laptop
   orc start my-feature
   # ... laptop battery dies ...
   # SSH from desktop
   tmux attach -t my-feature  # Everything still there!
   ```

2. **Layout Management:** Split panes for parallel workflows
   ```
   ┌───────────────────────────────────┐
   │  Pane 1: AI Assistant (70%)      │
   │  [Claude Code running]            │
   │                                   │
   ├───────────────────────────────────┤
   │  Pane 2: Testing (30%)           │
   │  $ npm test -- --watch           │
   └───────────────────────────────────┘
   ```

3. **Keyboard Control:** Zero mouse dependency
   - Navigate panes: `Ctrl+Space h/j/k/l`
   - Resize panes: `Ctrl+Space H/J/K/L`
   - Copy/paste: `Ctrl+Space [`, then `v`, `y`, `p`

**Implementation:**
```bash
# Create session (from bin/orc)
tmux new-session -d -s "$task_name" -c "$worktree_path"

# Split horizontally (70/30)
tmux split-window -v -p 30 -t "${task_name}:0"

# Send commands to panes
tmux send-keys -t "${task_name}:0.0" "claude" C-m    # Top pane
tmux send-keys -t "${task_name}:0.1" "zsh" C-m       # Bottom pane

# Attach
tmux attach-session -t "$task_name"
```

**Session Naming:**
- Session name = Task name
- Easy to remember: `tmux attach -t fix-login-bug`
- One-to-one mapping: Each worktree has its own session

---

### 4. Configuration Layer

#### Tmux Config (`config/.tmux.conf`)

**Design Philosophy:** "No Mouse, Maximum Keyboard"

**Key Features:**
```tmux
# Mouse disabled (forces keyboard usage)
set -g mouse off

# Ergonomic prefix (Ctrl+Space instead of Ctrl+b)
set -g prefix C-Space

# Vim-style navigation
bind h select-pane -L  # Left
bind j select-pane -D  # Down
bind k select-pane -U  # Up
bind l select-pane -R  # Right

# Status bar at top (reduces eye strain)
set -g status-position top

# Nord-inspired colors (minimalist, low distraction)
set -g status-style bg=#2E3440,fg=#D8DEE9
```

**Why These Choices?**

1. **No Mouse:** Forces muscle memory, faster in long run
2. **Ctrl+Space:** Easier to press than Ctrl+b (less hand movement)
3. **Vim Keys:** If you know Vim, you know Tmux
4. **Top Status Bar:** More ergonomic (eyes naturally look up)
5. **Nord Colors:** Low contrast = Less distraction = Better focus

#### Shell Integration (`config/.zshrc_addon`)

**Purpose:** Quality-of-life improvements for OrcMate users.

**Features:**
```zsh
# Aliases
alias os='orc start'     # Quick start
alias oc='orc clean'     # Quick clean
alias ol='orc list'      # Quick list

# Auto-detect worktree
is_orcmate_worktree() { ... }

# Show task in prompt
ORCMATE_TASK=$(get_orcmate_task)
PS1="🦍 [${ORCMATE_TASK}] ${PS1}"

# Helper functions
orc_switch <task>   # Switch between sessions
orc_commit "msg"    # Quick commit
orc_status          # Show current status
orc_keys            # Keyboard shortcuts reference
```

---

## 🔄 Complete System Flow

### `orc start <task-name>`

```
1. Validate inputs and check dependencies
   ├─ Is this a git repository?
   ├─ Is tmux installed?
   └─ Is git installed?

2. Check for conflicts
   ├─ Does worktree already exist?
   ├─ Does tmux session exist?
   └─ Does branch exist?

3. Create git worktree
   ├─ mkdir -p .worktrees/
   ├─ git worktree add -b agent/<task> .worktrees/<task>
   └─ Copy .env if exists

4. Create tmux session
   ├─ tmux new-session -d -s <task>
   ├─ tmux split-window -v -p 30
   ├─ Start AI in pane 1
   └─ Start shell in pane 2

5. Attach to session
   └─ tmux attach-session -t <task>
```

### `orc clean <task-name>`

```
1. Validate task name

2. Kill tmux session
   └─ tmux kill-session -t <task>

3. Remove git worktree
   └─ git worktree remove .worktrees/<task> --force

4. Delete git branch
   └─ git branch -D agent/<task>

5. Report success
```

### `orc list`

```
1. List git worktrees
   └─ git worktree list | grep .worktrees

2. List tmux sessions
   └─ tmux list-sessions

3. Format and display
```

---

## 🎯 Design Principles

### 1. **Isolation**
- Each task = Separate directory + Separate branch + Separate session
- No cross-contamination
- Safe for AI experimentation

### 2. **Simplicity**
- One binary (`orc`)
- Three commands (`start`, `clean`, `list`)
- Zero complex dependencies

### 3. **Keyboard-First**
- No mouse support in tmux
- All operations via keyboard shortcuts
- Muscle memory = Flow state

### 4. **Persistence**
- Tmux sessions survive disconnections
- Perfect for remote work
- Perfect for long AI conversations

### 5. **Minimal Setup**
- `./install.sh` and you're done
- No configuration files to edit (optional)
- Works out of the box

---

## 🔒 Security Considerations

### .env File Handling
```bash
# Copy .env to worktree (if exists)
if [ -f "${repo_root}/.env" ]; then
    cp "${repo_root}/.env" "${worktree_path}/.env"
fi
```

**Why?**
- Each worktree gets its own .env
- Can modify without affecting main repo
- AI can't accidentally commit secrets (worktree is gitignored)

**Security Note:**
- .env files should never be committed
- Add `.worktrees/` to `.gitignore` (installer does this)
- Use secret management tools for production

### Branch Isolation
```bash
# Agent branches are isolated
agent/feature-a  # AI work happens here
agent/feature-b  # Completely separate

main            # Your stable branch (never touched)
```

**Safety:**
- AI can't accidentally push to main
- Each task is a separate branch
- Easy to abandon failed experiments

---

## 📊 File Structure

```
project-root/
├── .git/                 # Git database (shared)
│   ├── worktrees/        # Worktree metadata
│   └── refs/heads/
│       └── agent/        # Agent branches
│
├── .worktrees/           # Gitignored!
│   ├── task-1/           # Isolated environment
│   │   ├── src/
│   │   ├── .env          # Copied from root
│   │   └── .git          # Link to main .git
│   │
│   └── task-2/           # Another task
│       └── ...
│
├── src/                  # Your main codebase
├── .env                  # Original env file
└── .gitignore            # Contains: .worktrees/
```

---

## 🔧 Technical Details

### Error Handling

**Strategy:** Fail-fast with clear messages

```bash
set -euo pipefail  # Exit on error, undefined vars, pipe failures

error() {
    echo -e "${RED}ERROR:${NC} $1" >&2
    exit 1
}
```

### Color Output

**Why colors?**
- Visual hierarchy
- Faster scanning
- Better UX in CLI

**Implementation:**
```bash
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'  # No Color
```

### Idempotency

**Goal:** Commands can be run multiple times safely

```bash
# Check before creating
if [ -d "$worktree_path" ]; then
    error "Worktree already exists"
fi

# Check before killing
if tmux has-session -t "$task_name" 2>/dev/null; then
    tmux kill-session -t "$task_name"
else
    warn "Session not found (skipping)"
fi
```

---

## 🚀 Performance

### Lightweight
- No daemon processes
- No background services
- Just shell scripts + git + tmux

### Fast Operations
- `orc start`: ~1-2 seconds (git worktree creation)
- `orc clean`: ~1 second (cleanup operations)
- `orc list`: Instant (just queries git and tmux)

---

## 🔮 Future Enhancements

### Potential Features (Not in MVP)

1. **Templates:** Pre-configured pane layouts
2. **Hooks:** Run custom scripts on start/clean
3. **Remote Sync:** Sync worktrees across machines
4. **Integration:** GitHub CLI integration for PRs
5. **Metrics:** Track time spent in each session
6. **AI Profiles:** Different pane layouts for different AI tools

---

## 📚 Technical References

### Git Worktrees
- [Git Worktree Documentation](https://git-scm.com/docs/git-worktree)
- Use case: Parallel branch development

### Tmux
- [Tmux Manual](https://man7.org/linux/man-pages/man1/tmux.1.html)
- Use case: Terminal multiplexing and session persistence

### Bash Scripting
- [Bash Best Practices](https://bertvv.github.io/cheat-sheets/Bash.html)
- Use case: Robust CLI tool development

---

## 🎓 Learning More

### Recommended Reading Order:
1. This document (architecture.md) - Understand the system
2. [workflow.md](./workflow.md) - Learn the user workflow
3. `config/.tmux.conf` - Study the keyboard bindings
4. `bin/orc` - Read the source code

### Hands-On Learning:
1. Install OrcMate: `./install.sh`
2. Start a session: `orc start test-task`
3. Explore the worktree: `ls .worktrees/test-task`
4. Try keyboard shortcuts: `Ctrl+Space ?`
5. Clean up: `orc clean test-task`

---

**Built with 💚 for developers who love keyboards, hate mice, and trust AI.**

🦍 **OrcMate** - Because AI coding should be organized, isolated, and keyboard-first.
