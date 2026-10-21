#!/bin/bash

# GOD MODE Activation Script for T27 OS Swarm Computing
# Author: Claude Code
# Version: 1.0.0
# Date: 2026-10-21

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
GOD_MODE_STATE_FILE="/tmp/god-mode-state"
LOG_FILE="$PWD/logs/god-mode.log"
CONFIG_FILE="$PWD/god-mode-config.t27"

# Create logs directory
mkdir -p "$(dirname "$LOG_FILE")"

echo -e "${BLUE}🔥 GOD MODE Activation Script${NC}"
echo "===================================="

# Function to print status
print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Function to log
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" >> "$LOG_FILE"
    echo "$1"
}

# Check if required files exist
check_requirements() {
    print_status "Checking requirements..."

    # Check binary files
    if [ ! -f "./tri" ]; then
        print_error "tri binary not found"
        exit 1
    fi

    if [ ! -f "./t27b" ]; then
        print_error "t27b binary not found"
        exit 1
    fi

    if [ ! -f "./t27c" ]; then
        print_error "t27c binary not found"
        exit 1
    fi

    # Check configuration
    if [ ! -f "$CONFIG_FILE" ]; then
        print_error "Configuration file not found: $CONFIG_FILE"
        exit 1
    fi

    # Check GOD MODE specification
    if [ ! -f "specs/t27os/god-mode-t27b-compatible.t27" ]; then
        print_error "GOD MODE specification not found"
        exit 1
    fi

    print_status "All requirements satisfied"
}

# Validate configuration
validate_config() {
    print_status "Validating configuration..."

    if ./t27b test "$CONFIG_FILE" >/dev/null 2>&1; then
        print_status "Configuration valid"
        return 0
    else
        print_error "Configuration validation failed"
        return 1
    fi
}

# Check current GOD MODE state
check_current_state() {
    if [ -f "$GOD_MODE_STATE_FILE" ]; then
        CURRENT_STATE=$(cat "$GOD_MODE_STATE_FILE")
        log "Current GOD MODE state: $CURRENT_STATE"
        case $CURRENT_STATE in
            "OFF")
                echo -e "${YELLOW}GOD MODE is currently OFF${NC}"
                ;;
            "ON")
                echo -e "${GREEN}GOD MODE is currently ON${NC}"
                ;;
            "PENDING")
                echo -e "${YELLOW}GOD MODE is in transition...${NC}"
                ;;
            *)
                echo -e "${YELLOW}Unknown state: $CURRENT_STATE${NC}"
                ;;
        esac
    else
        echo -e "${YELLOW}GOD MODE state file not found (initializing)${NC}"
        echo "OFF" > "$GOD_MODE_STATE_FILE"
        CURRENT_STATE="OFF"
    fi

    return $CURRENT_STATE
}

# Activate GOD MODE
activate_god_mode() {
    print_status "Activating GOD MODE..."

    # Update state to PENDING
    echo "PENDING" > "$GOD_MODE_STATE_FILE"
    log "State changed to PENDING"

    # Start animation sequence
    echo -e "${BLUE}Starting GOD MODE animation sequence...${NC}"

    # Simulate transition animation
    for i in {1..5}; do
        echo -n "🔥 "
        sleep 0.2
    done
    echo ""

    # Update state to ON
    echo "ON" > "$GOD_MODE_STATE_FILE"
    log "State changed to ON"

    print_status "GOD MODE activated successfully!"

    # Start background services
    start_background_services
}

# Deactivate GOD MODE
deactivate_god_mode() {
    print_status "Deactivating GOD MODE..."

    # Update state to PENDING
    echo "PENDING" > "$GOD_MODE_STATE_FILE"
    log "State changed to PENDING"

    # Animate transition
    echo -e "${BLUE}Deactivating GOD MODE...${NC}"
    for i in {1..3}; do
        echo -n "⚡ "
        sleep 0.3
    done
    echo ""

    # Update state to OFF
    echo "OFF" > "$GOD_MODE_STATE_FILE"
    log "State changed to OFF"

    print_status "GOD MODE deactivated"
}

# Start background services
start_background_services() {
    print_status "Starting background services..."

    # Start state monitoring
    ./scripts/monitor-god-mode.sh &
    MONITOR_PID=$!

    # Start animation service
    ./scripts/god-mode-animation.sh &
    ANIMATION_PID=$!

    # Start Swarm sync
    if [ "$GOD_MODE_AUTO_ACTIVATE" = true ]; then
        ./scripts/swarm-sync.sh &
        SWARM_PID=$!
    fi

    log "Background services started: PID=$MONITOR_PID,$ANIMATION_PID,$SWARM_PID"
}

# Stop background services
stop_background_services() {
    print_status "Stopping background services..."

    # Get PIDs from state file or environment
    if [ -f "/tmp/god-mode-pids" ]; then
        PIDS=$(cat /tmp/god-mode-pids)
        kill $PIDS 2>/dev/null || true
        rm -f /tmp/god-mode-pids
    fi

    log "Background services stopped"
}

# Show status
show_status() {
    echo -e "\n${BLUE}📊 GOD MODE Status${NC}"
    echo "=================="

    # Current state
    if [ -f "$GOD_MODE_STATE_FILE" ]; then
        STATE=$(cat "$GOD_MODE_STATE_FILE")
        case $STATE in
            "OFF")  echo -e "State: ${RED}OFF${NC}" ;;
            "ON")   echo -e "State: ${GREEN}ON${NC}" ;;
            "PENDING") echo -e "State: ${YELLOW}PENDING${NC}" ;;
        esac
    fi

    # Configuration
    echo "Configuration: $CONFIG_FILE"

    # Background services
    if [ -f "/tmp/god-mode-pids" ]; then
        echo "Background Services: Running"
        echo "  - State Monitor: Active"
        echo "  - Animation Service: Active"
        echo "  - Swarm Sync: Active"
    else
        echo "Background Services: Stopped"
    fi

    # Version
    echo "Version: 1.0.0"
    echo "Build Date: 2026-10-21"
}

# Main script logic
case "${1:-}" in
    "activate")
        check_requirements
        validate_config
        check_current_state
        activate_god_mode
        show_status
        ;;
    "deactivate")
        check_requirements
        deactivate_god_mode
        stop_background_services
        show_status
        ;;
    "status")
        check_requirements
        show_status
        ;;
    "restart")
        check_requirements
        deactivate_god_mode
        sleep 2
        activate_god_mode
        show_status
        ;;
    *)
        echo "Usage: $0 {activate|deactivate|status|restart}"
        echo ""
        echo "Commands:"
        echo "  activate    - Activate GOD MODE"
        echo "  deactivate  - Deactivate GOD MODE"
        echo "  status      - Show current status"
        echo "  restart     - Restart GOD MODE"
        echo ""
        echo "Environment:"
        echo "  GOD_MODE_AUTO_ACTIVATE - Auto-connect to Swarm (true/false)"
        exit 1
        ;;
esac

echo -e "\n${GREEN}🎉 GOD MODE operation completed successfully!${NC}"