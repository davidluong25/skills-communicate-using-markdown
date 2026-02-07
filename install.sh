#!/usr/bin/env bash

# OrcMate Installation Script
# Installs OrcMate and sets up configuration files

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo ""
echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${BLUE}  🦍 OrcMate Installer${NC}"
echo -e "${BOLD}${BLUE}  Keyboard-First AI Terminal Environment${NC}"
echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Helper functions
error() {
    echo -e "${RED}✗ ERROR:${NC} $1" >&2
    exit 1
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

info() {
    echo -e "${BLUE}→${NC} $1"
}

warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    warn "Running as root. This will install files in /root directory."
    read -p "Continue? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Step 1: Check dependencies
info "Checking dependencies..."

MISSING_DEPS=()

if ! command -v tmux &> /dev/null; then
    MISSING_DEPS+=("tmux")
fi

if ! command -v git &> /dev/null; then
    MISSING_DEPS+=("git")
fi

if [ ${#MISSING_DEPS[@]} -ne 0 ]; then
    error "Missing required dependencies: ${MISSING_DEPS[*]}\n\nPlease install them first:\n  Ubuntu/Debian: sudo apt-get install ${MISSING_DEPS[*]}\n  MacOS: brew install ${MISSING_DEPS[*]}\n  Fedora: sudo dnf install ${MISSING_DEPS[*]}"
fi

success "All dependencies found (tmux, git)"

# Step 2: Install the CLI binary
info "Installing OrcMate CLI..."

BIN_SOURCE="${SCRIPT_DIR}/bin/orc"
BIN_TARGET="/usr/local/bin/orc"

if [ ! -f "$BIN_SOURCE" ]; then
    error "Source file not found: $BIN_SOURCE"
fi

# Check if /usr/local/bin is writable
if [ -w "/usr/local/bin" ]; then
    # Create symlink
    if [ -L "$BIN_TARGET" ] || [ -f "$BIN_TARGET" ]; then
        warn "Existing orc command found at $BIN_TARGET"
        read -p "Overwrite? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -f "$BIN_TARGET"
        else
            info "Skipping CLI installation"
            BIN_TARGET=""
        fi
    fi
    
    if [ -n "$BIN_TARGET" ]; then
        ln -sf "$BIN_SOURCE" "$BIN_TARGET"
        chmod +x "$BIN_SOURCE"
        success "CLI installed at $BIN_TARGET"
    fi
else
    warn "/usr/local/bin is not writable. Trying with sudo..."
    sudo ln -sf "$BIN_SOURCE" "$BIN_TARGET"
    sudo chmod +x "$BIN_SOURCE"
    success "CLI installed at $BIN_TARGET (using sudo)"
fi

# Step 3: Install Tmux configuration
info "Installing Tmux configuration..."

TMUX_SOURCE="${SCRIPT_DIR}/config/.tmux.conf"
TMUX_TARGET="${HOME}/.tmux.conf"

if [ ! -f "$TMUX_SOURCE" ]; then
    error "Tmux config not found: $TMUX_SOURCE"
fi

# Backup existing .tmux.conf if it exists
if [ -f "$TMUX_TARGET" ]; then
    BACKUP="${TMUX_TARGET}.backup.$(date +%Y%m%d_%H%M%S)"
    warn "Existing .tmux.conf found"
    info "Creating backup at: $BACKUP"
    cp "$TMUX_TARGET" "$BACKUP"
    success "Backup created"
fi

# Create symlink or copy
read -p "Symlink (recommended) or copy .tmux.conf? (s/c) " -n 1 -r
echo
if [[ $REPLY =~ ^[Ss]$ ]]; then
    ln -sf "$TMUX_SOURCE" "$TMUX_TARGET"
    success "Tmux config symlinked to $TMUX_TARGET"
else
    cp "$TMUX_SOURCE" "$TMUX_TARGET"
    success "Tmux config copied to $TMUX_TARGET"
fi

# Step 4: Install Zsh addon (optional)
info "Installing Zsh shell integration (optional)..."

ZSHRC_SOURCE="${SCRIPT_DIR}/config/.zshrc_addon"
ZSHRC_TARGET="${HOME}/.zshrc"

if [ -f "$ZSHRC_TARGET" ]; then
    # Check if already sourced
    if grep -q "orcmate.*zshrc_addon" "$ZSHRC_TARGET" 2>/dev/null; then
        info "OrcMate already integrated in .zshrc"
    else
        read -p "Add OrcMate integration to .zshrc? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "" >> "$ZSHRC_TARGET"
            echo "# OrcMate Shell Integration" >> "$ZSHRC_TARGET"
            echo "source ${ZSHRC_SOURCE}" >> "$ZSHRC_TARGET"
            success "OrcMate integration added to .zshrc"
        else
            info "Skipping .zshrc integration"
            info "You can manually add: source ${ZSHRC_SOURCE}"
        fi
    fi
else
    info ".zshrc not found. Skipping shell integration."
    info "If you use zsh, add this to your .zshrc: source ${ZSHRC_SOURCE}"
fi

# Step 5: Create worktrees directory in current git repo (if applicable)
if git rev-parse --git-dir > /dev/null 2>&1; then
    REPO_ROOT=$(git rev-parse --show-toplevel)
    WORKTREE_DIR="${REPO_ROOT}/.worktrees"
    
    if [ ! -d "$WORKTREE_DIR" ]; then
        info "Creating .worktrees directory in git repository..."
        mkdir -p "$WORKTREE_DIR"
        
        # Add to .gitignore if not already there
        GITIGNORE="${REPO_ROOT}/.gitignore"
        if [ -f "$GITIGNORE" ]; then
            if ! grep -q "^[[:space:]]*\.worktrees/" "$GITIGNORE" 2>/dev/null; then
                echo "" >> "$GITIGNORE"
                echo "# OrcMate worktrees" >> "$GITIGNORE"
                echo ".worktrees/" >> "$GITIGNORE"
                success "Added .worktrees/ to .gitignore"
            fi
        else
            echo ".worktrees/" > "$GITIGNORE"
            success "Created .gitignore with .worktrees/"
        fi
        
        success "Created .worktrees directory"
    fi
fi

# Installation complete
echo ""
echo -e "${BOLD}${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${GREEN}  ✓ Installation Complete!${NC}"
echo -e "${BOLD}${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BOLD}Quick Start:${NC}"
echo ""
echo -e "  1. Navigate to a git repository:"
echo -e "     ${BLUE}cd /path/to/your/project${NC}"
echo ""
echo -e "  2. Start a new OrcMate session:"
echo -e "     ${GREEN}orc start fix-login-bug${NC}"
echo ""
echo -e "  3. Work with AI in Pane 1, test in Pane 2"
echo ""
echo -e "  4. When done, clean up:"
echo -e "     ${GREEN}orc clean fix-login-bug${NC}"
echo ""
echo -e "${BOLD}Commands:${NC}"
echo -e "  ${BLUE}orc help${NC}     Show all commands"
echo -e "  ${BLUE}orc list${NC}     List active environments"
echo ""
echo -e "${BOLD}Documentation:${NC}"
echo -e "  ${BLUE}${SCRIPT_DIR}/docs/${NC}"
echo ""
echo -e "${BOLD}Tmux Key Reminder:${NC}"
echo -e "  Prefix changed to ${GREEN}Ctrl+Space${NC}"
echo -e "  Navigate panes: ${GREEN}Ctrl+Space h/j/k/l${NC}"
echo ""
echo -e "${YELLOW}Note:${NC} If you use zsh, restart your shell or run: ${BLUE}source ~/.zshrc${NC}"
echo ""
echo -e "${BOLD}Happy coding! 🦍${NC}"
echo ""
