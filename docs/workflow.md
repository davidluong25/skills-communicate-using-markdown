# OrcMate Workflow Guide

## 🎯 The OrcMate Development Lifecycle

OrcMate is designed around a simple, repeatable workflow that keeps your AI-assisted development organized and isolated. This guide walks you through the complete lifecycle of working on a task.

---

## 📋 Workflow Overview

```
┌─────────────┐
│   START     │  Create isolated environment
│  orc start  │  (Git Worktree + Tmux Session)
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    CODE     │  AI in Pane 1, Testing in Pane 2
│   + TEST    │  Iterate until feature is ready
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   REVIEW    │  Review changes, commit
│  + COMMIT   │  Push to remote (optional)
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    MERGE    │  Merge branch to main
│  (Manual)   │  Switch to main, merge agent/<task>
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    CLEAN    │  Clean up environment
│  orc clean  │  Remove worktree, session, branch
└─────────────┘
```

---

## 🚀 Step-by-Step Guide

### Step 1: Start a New Task

Navigate to your git repository and start a new OrcMate session:

```bash
cd /path/to/your/project
orc start fix-login-bug
```

**What happens:**
- ✅ Creates a new git worktree at `.worktrees/fix-login-bug`
- ✅ Creates a new branch `agent/fix-login-bug`
- ✅ Copies your `.env` file (if it exists)
- ✅ Creates a tmux session named `fix-login-bug`
- ✅ Splits the window into 2 panes:
  - **Pane 1 (Top, 70%):** AI Assistant pane
  - **Pane 2 (Bottom, 30%):** Testing/Monitoring pane

You're automatically attached to the session and ready to work!

---

### Step 2: Work with AI (Pane 1)

The top pane is your **Action Pane** - where you interact with your AI assistant.

**If using Claude Code or similar AI:**
```bash
# The AI assistant should start automatically
# If not, you can manually start it:
claude
# or
cursor
# or any other AI tool
```

**AI Workflow:**
1. Describe your task to the AI
2. Review the AI's suggestions
3. Let the AI make changes to your code
4. Ask follow-up questions
5. Iterate until satisfied

**Tmux Navigation:**
- `Ctrl+Space j` - Move to bottom pane (Testing)
- `Ctrl+Space k` - Move to top pane (AI)
- `Ctrl+Space d` - Detach from session (keeps running)

---

### Step 3: Test and Validate (Pane 2)

The bottom pane is your **Monitor Pane** - where you run tests, check logs, and validate changes.

**Switch to Pane 2:**
```bash
Ctrl+Space j
```

**Common commands:**
```bash
# Run tests
npm test
# or
pytest
# or
cargo test

# Check linting
npm run lint
# or
flake8 .

# Run the application
npm run dev
# or
python app.py

# View logs
tail -f logs/app.log

# Check git status
git status
git diff
```

**Pro Tip:** Keep this pane open in a watch mode for instant feedback:
```bash
npm run test -- --watch
# or
pytest-watch
```

---

### Step 4: Review and Commit

Once you're happy with the changes:

**Review what changed:**
```bash
git status
git diff
```

**Commit your work:**
```bash
git add .
git commit -m "Fix login bug: Handle null email edge case"
```

**Push to remote (optional):**
```bash
git push origin agent/fix-login-bug
```

**Or use the helper function (if using zsh addon):**
```bash
orc_commit "Fix login bug: Handle null email edge case"
```

---

### Step 5: Merge to Main Branch

When your feature is complete and tested, merge it back to your main branch.

**Option A: From within the worktree**
```bash
# Switch to main in the worktree
git checkout main
git merge agent/fix-login-bug

# Push if needed
git push origin main
```

**Option B: From your main repository**
```bash
# Detach from tmux
Ctrl+Space d

# Go to main repository
cd /path/to/your/project  # (not the worktree)

# Merge the branch
git checkout main
git merge agent/fix-login-bug
git push origin main
```

---

### Step 6: Clean Up

After merging, clean up the OrcMate environment:

```bash
orc clean fix-login-bug
```

**What happens:**
- ✅ Kills the tmux session
- ✅ Removes the git worktree directory
- ✅ Deletes the `agent/fix-login-bug` branch

**Your main repository is clean and ready for the next task!**

---

## 🎨 Advanced Workflows

### Working on Multiple Tasks Simultaneously

OrcMate supports multiple parallel tasks:

```bash
# Start multiple sessions
orc start feature-authentication
orc start fix-database-bug
orc start refactor-api

# List all active sessions
orc list

# Switch between sessions (if using zsh addon)
orc_switch feature-authentication
```

**In tmux, switch sessions:**
```bash
Ctrl+Space )  # Next session
Ctrl+Space (  # Previous session
```

---

### Detaching and Reattaching

**Detach from a session (keeps it running):**
```bash
Ctrl+Space d
```

**Reattach later:**
```bash
tmux attach -t fix-login-bug
# or if using aliases:
ta fix-login-bug
```

**This means:**
- ✅ Your AI assistant keeps running
- ✅ Your tests keep running
- ✅ You can disconnect and come back anytime
- ✅ Perfect for remote development over SSH

---

### Collaborating with AI on Long-Running Tasks

For complex features that span multiple sessions:

1. **Don't clean up immediately** - Keep the worktree active
2. **Commit frequently** - Push to remote branch regularly
3. **Detach when taking breaks** - The session persists
4. **Resume anytime** - Just reattach to the session

```bash
# Day 1: Start the task
orc start complex-feature
# ... work with AI ...
Ctrl+Space d  # Detach

# Day 2: Resume
tmux attach -t complex-feature
# ... continue working ...

# Day 3: Finish and clean up
orc clean complex-feature
```

---

## 💡 Pro Tips

### 1. **Stay in Keyboard Mode**
- Don't use the mouse! It breaks your flow
- Learn the tmux keybindings (see `orc_keys`)
- Muscle memory = Flow state

### 2. **Use Both Panes Effectively**
- Top pane: AI conversation and code review
- Bottom pane: Continuous testing and validation
- Think of it as "Async Pair Programming"

### 3. **Commit Often, Clean Late**
- Make small, frequent commits in your worktree
- This helps the AI understand context better
- Clean up only after merging to main

### 4. **Use Descriptive Task Names**
- Good: `fix-login-null-email-bug`
- Bad: `fix` or `test`
- Clear names help when switching between multiple tasks

### 5. **Copy Mode for Code Review**
- Enter copy mode: `Ctrl+Space [`
- Navigate with Vim keys: `h, j, k, l`
- Select text: `v` (visual mode)
- Copy: `y` (yank)
- Paste: `Ctrl+Space p`

---

## 🔍 Troubleshooting

### "Worktree already exists"
```bash
# List all worktrees
git worktree list

# Remove manually if needed
git worktree remove .worktrees/task-name --force
```

### "Tmux session already exists"
```bash
# List sessions
tmux list-sessions

# Kill a session
tmux kill-session -t task-name
```

### "Branch already exists"
```bash
# List branches
git branch -a

# Delete if needed
git branch -D agent/task-name
```

### Clean up everything at once
```bash
orc clean task-name
# This handles all of the above automatically
```

---

## 📚 Next Steps

- Read [architecture.md](./architecture.md) to understand how OrcMate works internally
- Explore the keyboard shortcuts: `orc_keys` (if using zsh addon)
- Customize your `.tmux.conf` for your preferences
- Share your OrcMate workflow with your team!

---

**Remember:** The goal of OrcMate is to help you stay focused, organized, and in "flow state" while working with AI assistants. Embrace the keyboard-only workflow, and you'll never go back! 🦍
