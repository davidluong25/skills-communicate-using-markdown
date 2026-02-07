#!/usr/bin/env bash

# OrcMate Monitoring System Startup Script
# Starts both the server and watcher in the background

set -euo pipefail

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Log files
SERVER_LOG="${SCRIPT_DIR}/logs/server.log"
WATCHER_LOG="${SCRIPT_DIR}/logs/watcher.log"

# PID files
SERVER_PID="${SCRIPT_DIR}/.server.pid"
WATCHER_PID="${SCRIPT_DIR}/.watcher.pid"

# Create logs directory
mkdir -p "${SCRIPT_DIR}/logs"

# Helper functions
info() {
    echo -e "${BLUE}→${NC} $1"
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

warn() {
    echo -e "${YELLOW}!${NC} $1"
}

error() {
    echo -e "${RED}ERROR:${NC} $1" >&2
}

# Check if Node.js is installed
check_node() {
    if ! command -v node &> /dev/null; then
        error "Node.js is not installed. Please install Node.js v14+ first."
        exit 1
    fi
    
    local node_version=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
    if [ "$node_version" -lt 14 ]; then
        error "Node.js version 14+ required. Current version: $(node -v)"
        exit 1
    fi
}

# Check if dependencies are installed
check_deps() {
    if [ ! -d "${SCRIPT_DIR}/node_modules" ]; then
        warn "Dependencies not installed. Running npm install..."
        cd "${SCRIPT_DIR}"
        npm install
        success "Dependencies installed"
    fi
}

# Check if .env exists
check_env() {
    if [ ! -f "${SCRIPT_DIR}/.env" ]; then
        warn ".env file not found. Creating from .env.example..."
        cp "${SCRIPT_DIR}/.env.example" "${SCRIPT_DIR}/.env"
        warn "Please edit .env file with your configuration"
        echo ""
        echo "  nano ${SCRIPT_DIR}/.env"
        echo ""
    fi
}

# Stop running services
stop_services() {
    info "Stopping services..."
    
    # Stop server
    if [ -f "$SERVER_PID" ]; then
        local pid=$(cat "$SERVER_PID")
        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid"
            success "Server stopped (PID: $pid)"
        fi
        rm -f "$SERVER_PID"
    fi
    
    # Stop watcher
    if [ -f "$WATCHER_PID" ]; then
        local pid=$(cat "$WATCHER_PID")
        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid"
            success "Watcher stopped (PID: $pid)"
        fi
        rm -f "$WATCHER_PID"
    fi
}

# Start server
start_server() {
    info "Starting OrcMate Server..."
    cd "${SCRIPT_DIR}"
    node server.js >> "$SERVER_LOG" 2>&1 &
    echo $! > "$SERVER_PID"
    success "Server started (PID: $(cat $SERVER_PID))"
    info "Server logs: $SERVER_LOG"
}

# Start watcher
start_watcher() {
    info "Starting Claude Watcher..."
    cd "${SCRIPT_DIR}"
    node watcher.js >> "$WATCHER_LOG" 2>&1 &
    echo $! > "$WATCHER_PID"
    success "Watcher started (PID: $(cat $WATCHER_PID))"
    info "Watcher logs: $WATCHER_LOG"
}

# Show status
show_status() {
    echo ""
    echo "=========================================="
    echo "  OrcMate Monitoring System Status"
    echo "=========================================="
    echo ""
    
    # Server status
    if [ -f "$SERVER_PID" ] && kill -0 "$(cat $SERVER_PID)" 2>/dev/null; then
        echo -e "${GREEN}●${NC} Server: Running (PID: $(cat $SERVER_PID))"
        echo "  Logs: $SERVER_LOG"
    else
        echo -e "${RED}○${NC} Server: Stopped"
    fi
    
    echo ""
    
    # Watcher status
    if [ -f "$WATCHER_PID" ] && kill -0 "$(cat $WATCHER_PID)" 2>/dev/null; then
        echo -e "${GREEN}●${NC} Watcher: Running (PID: $(cat $WATCHER_PID))"
        echo "  Logs: $WATCHER_LOG"
    else
        echo -e "${RED}○${NC} Watcher: Stopped"
    fi
    
    echo ""
    echo "=========================================="
    echo ""
}

# Show logs
show_logs() {
    local service="$1"
    
    if [ "$service" = "server" ]; then
        if [ -f "$SERVER_LOG" ]; then
            tail -f "$SERVER_LOG"
        else
            error "Server log file not found"
        fi
    elif [ "$service" = "watcher" ]; then
        if [ -f "$WATCHER_LOG" ]; then
            tail -f "$WATCHER_LOG"
        else
            error "Watcher log file not found"
        fi
    else
        error "Unknown service: $service"
        echo "Usage: $0 logs [server|watcher]"
    fi
}

# Main function
main() {
    local command="${1:-start}"
    
    case "$command" in
        start)
            check_node
            check_deps
            check_env
            stop_services
            start_server
            sleep 2
            start_watcher
            sleep 1
            show_status
            echo ""
            info "To view logs:"
            echo "  $0 logs server   # Server logs"
            echo "  $0 logs watcher  # Watcher logs"
            echo ""
            info "To stop services:"
            echo "  $0 stop"
            echo ""
            ;;
        stop)
            stop_services
            show_status
            ;;
        restart)
            stop_services
            sleep 2
            start_server
            sleep 2
            start_watcher
            sleep 1
            show_status
            ;;
        status)
            show_status
            ;;
        logs)
            local service="${2:-}"
            if [ -z "$service" ]; then
                error "Please specify which service logs to show"
                echo "Usage: $0 logs [server|watcher]"
                exit 1
            fi
            show_logs "$service"
            ;;
        *)
            echo "OrcMate Monitoring System"
            echo ""
            echo "Usage: $0 [command]"
            echo ""
            echo "Commands:"
            echo "  start    Start both server and watcher (default)"
            echo "  stop     Stop all services"
            echo "  restart  Restart all services"
            echo "  status   Show service status"
            echo "  logs     Show logs (server|watcher)"
            echo ""
            echo "Examples:"
            echo "  $0 start"
            echo "  $0 logs server"
            echo "  $0 stop"
            exit 1
            ;;
    esac
}

# Run main
main "$@"
