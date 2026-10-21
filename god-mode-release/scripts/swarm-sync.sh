#!/bin/bash

# GOD Mode Swarm Synchronization Script
# Handles synchronization of GOD MODE state across Swarm nodes

set -e

# Configuration
STATE_FILE="/tmp/god-mode-state"
LOG_FILE="$PWD/logs/god-mode.log"
SYNC_INTERVAL_MS=1000
SWARM_CONFIG="$PWD/god-mode-config.t27"
MAX_RETRIES=3
RETRY_DELAY_MS=5000

# Swarm node management
SWARM_NODES_FILE="/tmp/god-mode-nodes"
LEADER_NODE_FILE="/tmp/god-mode-leader"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] SWARM: $1" >> "$LOG_FILE"
    echo "SWARM: $1"
}

# Initialize swarm connection
init_swarm_connection() {
    log "Initializing Swarm connection"

    # Check if we have the required tools
    if ! command -v curl &> /dev/null; then
        log "Warning: curl not found - using local mode only"
        return 1
    fi

    # Get swarm configuration
    if [ -f "$SWARM_CONFIG" ]; then
        local auto_connect=$(./t27b extract-config "$SWARM_CONFIG" "auto_connect")
        if [ "$auto_connect" = "true" ]; then
            connect_to_swarm
        else
            log "Swarm auto-connect disabled - running in local mode"
        fi
    else
        log "Swarm config not found - running in local mode"
    fi
}

# Connect to Swarm network
connect_to_swarm() {
    log "Connecting to Swarm network"

    # Try to discover swarm nodes
    local nodes=$(discover_swarm_nodes)

    if [ -n "$nodes" ]; then
        echo "$nodes" > "$SWARM_NODES_FILE"
        log "Connected to Swarm nodes: $nodes"

        # Determine leader (first node alphabetically)
        local leader=$(echo "$nodes" | head -n 1)
        echo "$leader" > "$LEADER_NODE_FILE"
        log "Elected leader: $leader"
    else
        log "No Swarm nodes discovered"
        return 1
    fi
}

# Discover available Swarm nodes
discover_swarm_nodes() {
    # This is a placeholder - real implementation would use
    # Swarm discovery protocol or configuration
    echo "node1.example.com:8080
node2.example.com:8080
node3.example.com:8080"
}

# Synchronize state across swarm
sync_state() {
    local current_state=$(cat "$STATE_FILE" 2>/dev/null || echo "OFF")

    if [ -f "$SWARM_NODES_FILE" ]; then
        local nodes=$(cat "$SWARM_NODES_FILE")
        local leader=$(cat "$LEADER_NODE_FILE" 2>/dev/null || echo "")

        log "Syncing state '$current_state' to Swarm"

        # Sync with leader first
        if [ -n "$leader" ]; then
            sync_with_node "$leader" "$current_state"
        fi

        # Broadcast to other nodes
        echo "$nodes" | while read -r node; do
            if [ "$node" != "$leader" ] && [ -n "$node" ]; then
                sync_with_node "$node" "$current_state"
            fi
        done
    fi
}

# Sync state with specific node
sync_with_node() {
    local node="$1"
    local state="$2"
    local retry_count=0

    while [ $retry_count -lt $MAX_RETRIES ]; do
        log "Attempting to sync with $node (attempt $((retry_count + 1))/$MAX_RETRIES)"

        # Send state update to node
        local response=$(curl -s -X POST \
            -H "Content-Type: application/json" \
            -d "{\"state\":\"$state\",\"timestamp\":\"$(date +%s)\"}" \
            "http://$node/api/god-mode/sync" \
            --connect-timeout 5 \
            --max-time 10 2>/dev/null)

        if [ $? -eq 0 ] && [ -n "$response" ]; then
            local success=$(echo "$response" | jq -r '.success' 2>/dev/null || echo "false")
            if [ "$success" = "true" ]; then
                log "Successfully synced with $node"
                return 0
            fi
        fi

        retry_count=$((retry_count + 1))
        if [ $retry_count -lt $MAX_RETRIES ]; then
            sleep $(echo "scale=3; $RETRY_DELAY_MS / 1000" | bc -l)
        fi
    done

    log "Failed to sync with $node after $MAX_RETRIES attempts"
    return 1
}

# Handle incoming state updates
handle_state_update() {
    local incoming_state="$1"
    local current_state=$(cat "$STATE_FILE" 2>/dev/null || echo "OFF")

    log "Received state update: $incoming_state (current: $current_state)"

    # Only update if incoming state is different and has higher priority
    if [ "$incoming_state" != "$current_state" ]; then
        case "$incoming_state" in
            "ON")
                if [ "$current_state" != "ON" ]; then
                    echo "$incoming_state" > "$STATE_FILE"
                    log "State updated to: $incoming_state"
                fi
                ;;
            "PENDING")
                if [ "$current_state" = "OFF" ]; then
                    echo "$incoming_state" > "$STATE_FILE"
                    log "State updated to: $incoming_state"
                fi
                ;;
        esac
    fi
}

# Consensus mechanism for distributed state
consensus_decision() {
    local current_state=$(cat "$STATE_FILE" 2>/dev/null || echo "OFF")

    if [ -f "$SWARM_NODES_FILE" ]; then
        local nodes=$(cat "$SWARM_NODES_FILE")
        local node_count=$(echo "$nodes" | wc -l)
        local required_majority=$((node_count / 2 + 1))

        # Count votes for ON vs OFF
        local on_votes=0
        local off_votes=0

        echo "$nodes" | while read -r node; do
            if [ -n "$node" ]; then
                local node_state=$(query_node_state "$node")
                if [ "$node_state" = "ON" ]; then
                    on_votes=$((on_votes + 1))
                else
                    off_votes=$((off_votes + 1))
                fi
            fi
        done

        log "Consensus: ON=$on_votes, OFF=$off_votes, Required=$required_majority"

        # Make decision based on majority
        if [ $on_votes -ge $required_majority ]; then
            echo "ON"
        elif [ $off_votes -ge $required_majority ]; then
            echo "OFF"
        else
            echo "$current_state"  # Maintain current state if no majority
        fi
    else
        echo "$current_state"  # No swarm, use local state
    fi
}

# Query state from specific node
query_node_state() {
    local node="$1"

    local response=$(curl -s -X GET \
        "http://$node/api/god-mode/state" \
        --connect-timeout 3 \
        --max-time 5 2>/dev/null)

    if [ $? -eq 0 ] && [ -n "$response" ]; then
        echo "$response" | jq -r '.state' 2>/dev/null || echo "OFF"
    else
        echo "OFF"
    fi
}

# Sync main loop
sync_loop() {
    log "Starting Swarm synchronization loop"

    while true; do
        # Check if we need to reconnect
        if [ ! -f "$SWARM_NODES_FILE" ] || [ $(find "$SWARM_NODES_FILE" -mmin +5 -print 2>/dev/null) ]; then
            init_swarm_connection
        fi

        # Perform state sync
        sync_state

        # Check for consensus
        local consensus_state=$(consensus_decision)
        local current_state=$(cat "$STATE_FILE" 2>/dev/null || echo "OFF")

        if [ "$consensus_state" != "$current_state" ]; then
            log "Applying consensus decision: $consensus_state"
            echo "$consensus_state" > "$STATE_FILE"
        fi

        sleep $(echo "scale=3; $SYNC_INTERVAL_MS / 1000" | bc -l)
    done
}

# HTTP API server for incoming sync requests
start_api_server() {
    log "Starting API server for incoming sync requests"

    # This would be a proper HTTP server in production
    # Using netcat here for demonstration
    while true; do
        echo "HTTP/1.1 200 OK
Content-Type: application/json

{\"success\": true, \"state\": \"$(cat "$STATE_FILE" 2>/dev/null || echo "OFF\")\"}" | nc -l -p 8080 -q 1 2>/dev/null || true
        sleep 0.1
    done
}

# Main execution
case "${1:-}" in
    "start")
        init_swarm_connection
        sync_loop &
        SWARM_PID=$!
        echo $SWARM_PID > /tmp/god-mode-swarm-pid
        ;;
    "stop")
        if [ -f "/tmp/god-mode-swarm-pid" ]; then
            kill $(cat /tmp/god-mode-swarm-pid) 2>/dev/null || true
            rm -f /tmp/god-mode-swarm-pid
        fi
        ;;
    "api")
        start_api_server
        ;;
    *)
        echo "Usage: $0 {start|stop|api}"
        exit 1
        ;;
esac