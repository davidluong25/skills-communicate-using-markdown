#!/usr/bin/env bash

# Markdown Analyzer Demo Script
# Demonstrates all features of the markdown analyzer

set -euo pipefail

ANALYZER="./bin/md-analyzer"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

# Helper function
demo_step() {
    echo ""
    echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BOLD}${BLUE}  $1${NC}"
    echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "${YELLOW}Press Enter to continue...${NC}"
    read -r
}

clear

echo -e "${BOLD}${GREEN}"
cat << "EOF"
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║          Markdown Analyzer - Interactive Demo            ║
║                                                           ║
║     Phân tích dự án và chạy file Markdown                ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

echo ""
echo "This demo will show you all features of the Markdown Analyzer."
echo ""
echo -e "${YELLOW}Press Enter to start...${NC}"
read -r

# Demo 1: List files
demo_step "1. List all markdown files (Liệt kê tất cả file MD)"
$ANALYZER list

# Demo 2: Statistics
demo_step "2. Show statistics (Hiển thị thống kê)"
$ANALYZER stats

# Demo 3: Analysis
demo_step "3. Analyze all files (Phân tích chi tiết)"
$ANALYZER analyze

# Demo 4: View a file
demo_step "4. View README.md (Xem file README)"
echo "Showing first 30 lines of README.md..."
echo ""
$ANALYZER view README.md | head -35

# Demo 5: Tree structure
demo_step "5. Show project tree (Cấu trúc dự án)"
$ANALYZER tree

# Final message
echo ""
echo -e "${BOLD}${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${GREEN}  Demo Complete! (Hoàn thành!)${NC}"
echo -e "${BOLD}${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "You can now use the markdown analyzer with these commands:"
echo ""
echo "  ./bin/md-analyzer list      - List files"
echo "  ./bin/md-analyzer analyze   - Full analysis"
echo "  ./bin/md-analyzer stats     - Statistics"
echo "  ./bin/md-analyzer view FILE - View a file"
echo "  ./bin/md-analyzer tree      - Project structure"
echo "  ./bin/md-analyzer help      - Help"
echo ""
echo "See docs/VIETNAMESE-GUIDE.md for Vietnamese documentation."
echo "See docs/md-analyzer-guide.md for full English documentation."
echo ""
