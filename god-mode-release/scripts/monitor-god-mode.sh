#!/bin/bash

# GOD Mode State Monitor Script
# Monitors GOD MODE state and triggers actions based on transitions

set -e

# Configuration
STATE_FILE="/tmp/god-mode-state"
LOG_FILE="$PWD/logs/god-mode.log"
PENDING_TIMEOUT_SECONDS=3
IDLE_TIMEOUT_SECONDS=300  # 5 minutes

# Create state file if it doesn't exist
mkdir -p "$(dirname "$STATE_FILE")"
[ ! -f "$STATE_FILE" ] && echo "OFF" > "$STATE_FILE"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] MONITOR: $1" >> "$LOG_FILE"
    echo "$1"
}

# State transition handling
handle_transition() {
    local old_state="$1"
    local new_state="$2"

    log "State transition: $old_state -> $new_state"

    case "$new_state" in
        "ON")
            log "GOD MODE activated - triggering swarm operations"
            ./scripts/swarm-trigger-activate.sh
            ;;
        "OFF")
            log "GOD MODE deactivated - cleaning up swarm operations"
            ./scripts/swarm-trigger-deactivate.sh
            ;;
        "PENDING")
            log "Starting transition animation"
            ./scripts/transition-animation.sh start
            # Set timeout for pending state
            (sleep $PENDING_TIMEOUT_SECONDS && check_pending_timeout) &
            PENDING_PID=$!
            echo $PENDING_PID > /tmp/god-mode-pending-pid
            ;;
    esac
}

# Check if state has been pending too long
check_pending_timeout() {
    if [ -f "/tmp/god-mode-pending-pid" ]; then
        local current_state=$(cat "$STATE_FILE")
        if [ "$current_state" = "PENDING" ]; then
            log "PENDING state timeout - forcing transition to OFF"
            echo "OFF" > "$STATE_FILE"
            handle_transition "PENDING" "OFF"
        fi
    fi
}

# Check for user inactivity
check_user_activity() {
    local last_user_event=$(cat /tmp/god-mode-last-event 2>/dev/null || echo "0")
    local current_time=$(date +%s)
    local idle_seconds=$((current_time - last_user_event))

    if [ $idle_seconds -ge $IDLE_TIMEOUT_SECONDS ]; then
        log "User idle for $idle_seconds seconds - considering GOD MODE auto-action"
        # Could trigger auto-action here if needed
    fi
}

# Main monitoring loop
monitor_loop() {
    log "Starting GOD MODE state monitor"

    while true; do
        local current_state=$(cat "$STATE_FILE")
        local previous_state="$current_state"

        # Wait for state change
        inotifywait -q -e modify "$STATE_FILE" 2>/dev/null || {
            # Fallback for systems without inotifywait
            sleep 1
            current_state=$(cat "$STATE_FILE")
        }

        if [ "$current_state" != "$previous_state" ]; then
            handle_transition "$previous_state" "$current_state"
        fi

        # Check user activity
        check_user_activity

        sleep 0.1
    done
}

# Signal handling
cleanup() {
    log "Monitor stopping"
    exit 0
}

trap cleanup SIGTERM SIGINT

# Start monitoring
log "Starting GOD MODE monitor"
monitor_loop