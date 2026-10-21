#!/bin/bash

# GOD Mode Status Reporter
# Provides detailed status information about GOD Mode operations

set -e

# Configuration
STATE_FILE="/tmp/god-mode-state"
LOG_FILE="$PWD/logs/god-mode.log"
CONFIG_FILE="$PWD/god-mode-config.t27"
PID_DIR="/tmp/god-mode-pids"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

print_header() {
    echo -e "${CYAN}"
    echo "╔═════════════════════════════════════════════════════════════════════════════╗"
    echo "║                    GOD MODE STATUS REPORT                                   ║"
    echo "║                         T27 OS Swarm Computing                              ║"
    echo "╚═════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""
}

print_section() {
    echo -e "${BLUE}▶ ${1}${NC}"
    echo ""
}

print_status_item() {
    local label="$1"
    local value="$2"
    local status="$3"

    case "$status" in
        "good") color="$GREEN" ;;
        "warning") color="$YELLOW" ;;
        "error") color="$RED" ;;
        "info") color="$BLUE" ;;
        *) color="$NC" ;;
    esac

    printf "  %-30s %s%s%s\n" "$label:" "$color" "$value" "$NC"
}

print_progress_bar() {
    local current="$1"
    local total="$2"
    local label="$3"

    local percentage=$((current * 100 / total))
    local filled=$((current * 20 / total))
    local empty=$((20 - filled))

    printf "  %s [%s%s] %d%%\n" "$label" \
           "$(printf '█' $filled)" \
           "$(printf ' ' $empty)" \
           "$percentage"
}

# Get current state
get_current_state() {
    if [ -f "$STATE_FILE" ]; then
        cat "$STATE_FILE"
    else
        echo "OFF"
    fi
}

# Get state description
get_state_description() {
    local state="$1"

    case "$state" in
        "OFF")
            echo "GOD MODE is disabled - normal operations"
            ;;
        "ON")
            echo "GOD MODE is active - swarm computing enabled"
            ;;
        "PENDING")
            echo "GOD MODE is transitioning - animations in progress"
            ;;
        *)
            echo "Unknown state: $state"
            ;;
    esac
}

# Check if services are running
check_services() {
    local services=()

    # Monitor service
    if pgrep -f "monitor-god-mode.sh" > /dev/null; then
        services+=("State Monitor: ${GREEN}Running${NC}")
    else
        services+=("State Monitor: ${RED}Stopped${NC}")
    fi

    # Animation service
    if pgrep -f "god-mode-animation.sh" > /dev/null; then
        services+=("Animation Service: ${GREEN}Running${NC}")
    else
        services+=("Animation Service: ${RED}Stopped${NC}")
    fi

    # Swarm sync service
    if pgrep -f "swarm-sync.sh" > /dev/null; then
        services+=("Swarm Sync: ${GREEN}Running${NC}")
    else
        services+=("Swarm Sync: ${RED}Stopped${NC}")
    fi

    # Process PIDs
    if [ -f "$PID_DIR" ]; then
        local pids=$(cat "$PID_DIR")
        services+=("Background PIDs: ${pids}")
    fi

    echo "${services[@]}"
}

# Check configuration validity
check_configuration() {
    local config_issues=()

    if [ ! -f "$CONFIG_FILE" ]; then
        config_issues+=("Configuration file missing")
    else
        # Check basic config
        local auto_connect=$(./t27b extract-config "$CONFIG_FILE" "auto_connect" 2>/dev/null || echo "not_found")
        if [ "$auto_connect" = "not_found" ]; then
            config_issues+=("Cannot read auto_connect setting")
        else
            print_status_item "Auto Connect" "$auto_connect" "good"
        fi
    fi

    echo "${config_issues[@]}"
}

# Check Swarm connectivity
check_swarm_connectivity() {
    local swarm_status=()

    if [ -f "/tmp/god-mode-nodes" ]; then
        local nodes=$(cat "/tmp/god-mode-nodes")
        local node_count=$(echo "$nodes" | wc -l)
        swarm_status+=("Nodes: $node_count")

        # Check leader
        if [ -f "/tmp/god-mode-leader" ]; then
            local leader=$(cat "/tmp/god-mode-leader")
            swarm_status+=("Leader: $leader")
        else
            swarm_status+=("Leader: ${RED}Not elected${NC}")
        fi

        # Test connectivity
        local connected_nodes=0
        echo "$nodes" | while read -r node; do
            if [ -n "$node" ]; then
                if curl -s --connect-timeout 2 "http://$node/api/god-mode/state" > /dev/null 2>&1; then
                    connected_nodes=$((connected_nodes + 1))
                fi
            fi
        done

        swarm_status+=("Connected: $connected_nodes/$node_count")

        if [ $connected_nodes -eq 0 ]; then
            swarm_status+=("Status: ${RED}Disconnected${NC}")
        elif [ $connected_nodes -lt $node_count ]; then
            swarm_status+=("Status: ${YELLOW}Partial${NC}")
        else
            swarm_status+=("Status: ${GREEN}Connected${NC}")
        fi
    else
        swarm_status+=("Status: ${YELLOW}Not connected${NC}")
    fi

    echo "${swarm_status[@]}"
}

# Show performance metrics
show_performance_metrics() {
    echo ""
    print_section "Performance Metrics"

    # System load
    if command -v uptime &> /dev/null; then
        local load=$(uptime | awk -F'load average:' '{ print $2 }')
        print_status_item "System Load" "$load" "info"
    fi

    # Memory usage
    if command -v free &> /dev/null; then
        local mem_info=$(free -m | awk 'NR==2{printf "%.1f%% (%d MB free)", $3*100/$2, $4}')
        print_status_item "Memory Usage" "$mem_info" "info"
    fi

    # CPU usage
    if command -v top &> /dev/null; then
        local cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}')
        print_status_item "CPU Usage" "${cpu_usage}%" "info"
    fi

    # Log file size
    if [ -f "$LOG_FILE" ]; then
        local log_size=$(du -h "$LOG_FILE" | cut -f1)
        print_status_item "Log Size" "$log_size" "info"
    fi
}

# Show recent activity
show_recent_activity() {
    echo ""
    print_section "Recent Activity"

    if [ -f "$LOG_FILE" ]; then
        echo "  Last 10 log entries:"
        echo ""
        tail -10 "$LOG_FILE" | while read -r line; do
            # Extract timestamp and message
            local timestamp=$(echo "$line" | cut -d']' -f2 | cut -d'[' -f2)
            local message=$(echo "$line" | cut -d']' -f3-)

            # Format and colorize based on content
            if echo "$message" | grep -q "error\|failed\|ERROR"; then
                color="$RED"
            elif echo "$message" | grep -q "success\|activated\|started"; then
                color="$GREEN"
            else
                color="$NC"
            fi

            printf "  [%s] %s%s%s\n" "$timestamp" "$color" "$message" "$NC"
        done
    else
        print_status_item "Log File" "Not found" "warning"
    fi
}

# Main status report
generate_status_report() {
    print_header

    # Current state
    echo ""
    print_section "Current State"
    local current_state=$(get_current_state)
    local state_desc=$(get_state_description "$current_state")

    print_status_item "State" "$current_state" "good"
    print_status_item "Description" "$state_desc" "info"

    # Services status
    echo ""
    print_section "Services Status"
    local services=$(check_services)
    for service in $services; do
        echo "  $service"
    done

    # Configuration
    echo ""
    print_section "Configuration"
    local config_issues=$(check_configuration)
    if [ ${#config_issues[@]} -eq 0 ]; then
        print_status_item "Status" "Valid" "good"
    else
        print_status_item "Status" "Issues found" "warning"
        for issue in "${config_issues[@]}"; do
            echo "  - $issue"
        done
    fi

    # Swarm connectivity
    echo ""
    print_section "Swarm Connectivity"
    local swarm_status=$(check_swarm_connectivity)
    for status in $swarm_status; do
        echo "  $status"
    done

    # Performance metrics
    show_performance_metrics

    # Recent activity
    show_recent_activity

    # Version info
    echo ""
    print_section "Version Information"
    print_status_item "Version" "1.0.0" "info"
    print_status_item "Build Date" "2026-10-21" "info"
    print_status_item "Author" "Claude Code" "info"

    echo ""
    print_section "Quick Actions"
    echo "  To activate:   ./scripts/activate-god-mode.sh activate"
    echo "  To deactivate: ./scripts/activate-god-mode.sh deactivate"
    echo "  To view logs:  tail -f $LOG_FILE"
}

# Main execution
case "${1:-}" in
    "json")
        # Output in JSON format for API integration
        current_state=$(get_current_state)
        echo "{\"state\": \"$current_state\", \"timestamp\": \"$(date +%s)\"}"
        ;;
    *)
        generate_status_report
        ;;
esac