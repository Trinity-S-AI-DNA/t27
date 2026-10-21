#!/bin/bash

# GOD Mode Animation Service
# Handles visual effects and animations for GOD MODE transitions

set -e

# Configuration
STATE_FILE="/tmp/god-mode-state"
LOG_FILE="$PWD/logs/god-mode.log"
ANIMATION_PID_FILE="/tmp/god-mode-animation-pid"
FRAME_RATE=30  # FPS
GLOW_STEPS=10

# Animation states
ANIMATION_STATE="idle"
CURRENT_GLOW_LEVEL=0.0
TARGET_GLOW_LEVEL=0.0

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] ANIMATION: $1" >> "$LOG_FILE"
}

# Calculate glow intensity based on state and progress
calculate_glow_intensity() {
    local state="$1"
    local progress="$2"

    case "$state" in
        "OFF")
            echo "0.0"
            ;;
        "ON")
            echo "1.0"
            ;;
        "PENDING")
            # Triangle wave animation
            local scaled=$(echo "$progress * 4" | bc -l)
            local pos=$(echo "$scaled / 1" | bc)
            local frac=$(echo "$scaled - $pos" | bc -l)

            if [ $(($pos % 2)) -eq 0 ]; then
                echo "$frac"
            else
                echo "1.0 - $frac" | bc -l
            fi
            ;;
        *)
            echo "0.0"
            ;;
    esac
}

# Update visual effects
update_visual_effects() {
    local glow_intensity="$1"

    # Update system-wide glow settings
    # This would interface with the actual T27 OS rendering system
    log "Setting glow intensity to: $glow_intensity"

    # Example: Update environment variable for UI components
    export GOD_MODE_GLOW_INTENSITY="$glow_intensity"

    # Trigger UI refresh
    if [ -f "$PWD/scripts/refresh-ui.sh" ]; then
        "$PWD/scripts/refresh-ui.sh" &
    fi
}

# Animation loop
animate() {
    local frame=0
    local total_frames=0

    while [ "$ANIMATION_STATE" != "idle" ]; do
        case "$ANIMATION_STATE" in
            "activating")
                total_frames=$GLOW_STEPS
                local progress=$(echo "scale=2; $frame / $total_frames" | bc -l)
                CURRENT_GLOW_LEVEL=$(calculate_glow_intensity "PENDING" "$progress")
                ;;
            "deactivating")
                total_frames=$GLOW_STEPS
                local progress=$(echo "scale=2; $frame / $total_frames" | bc -l)
                CURRENT_GLOW_LEVEL=$(calculate_glow_intensity "PENDING" "$progress")
                ;;
            "active")
                CURRENT_GLOW_LEVEL=$(calculate_glow_intensity "ON" "$(date +%s.%N)")
                ;;
            "inactive")
                CURRENT_GLOW_LEVEL=0.0
                ;;
        esac

        update_visual_effects "$CURRENT_GLOW_LEVEL"

        frame=$((frame + 1))
        if [ $frame -gt $total_frames ] && [ $total_frames -gt 0 ]; then
            frame=0
            case "$ANIMATION_STATE" in
                "activating")
                    ANIMATION_STATE="active"
                    log "Animation complete: GOD MODE active"
                    ;;
                "deactivating")
                    ANIMATION_STATE="inactive"
                    log "Animation complete: GOD Mode inactive"
                    ;;
            esac
        fi

        sleep $(echo "scale=3; 1 / $FRAME_RATE" | bc -l)
    done
}

# Start animation
start_animation() {
    local action="$1"

    log "Starting animation: $action"

    case "$action" in
        "activate")
            ANIMATION_STATE="activating"
            ;;
        "deactivate")
            ANIMATION_STATE="deactivating"
            ;;
        *)
            log "Unknown animation action: $action"
            return 1
            ;;
    esac

    # Start animation loop in background
    animate &
    ANIMATION_PID=$!
    echo $ANIMATION_PID > "$ANIMATION_PID_FILE"
}

# Stop animation
stop_animation() {
    log "Stopping animation"
    ANIMATION_STATE="idle"

    if [ -f "$ANIMATION_PID_FILE" ]; then
        local pid=$(cat "$ANIMATION_PID_FILE")
        kill $pid 2>/dev/null || true
        rm -f "$ANIMATION_PID_FILE"
    fi

    # Reset visual effects
    update_visual_effects "0.0"
}

# Monitor state changes and trigger animations
animation_monitor() {
    log "Starting animation monitor"

    while true; do
        local current_state=$(cat "$STATE_FILE" 2>/dev/null || echo "OFF")

        case "$current_state" in
            "ON")
                if [ "$ANIMATION_STATE" = "inactive" ]; then
                    start_animation "activate"
                fi
                ;;
            "OFF")
                if [ "$ANIMATION_STATE" = "active" ]; then
                    start_animation "deactivate"
                fi
                ;;
        esac

        sleep 0.1
    done
}

# Signal handling
cleanup() {
    log "Animation service stopping"
    stop_animation
    exit 0
}

trap cleanup SIGTERM SIGINT

# Main execution
case "${1:-}" in
    "start")
        animation_monitor
        ;;
    "stop")
        cleanup
        ;;
    *)
        echo "Usage: $0 {start|stop}"
        exit 1
        ;;
esac