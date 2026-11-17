#!/bin/bash

# Network Monitor Start Script
# Starts network monitoring for provider guard verification

echo "📡 Starting Network Monitor..."

# Check if network monitor is already running
if [ -f ".network-monitor.pid" ]; then
    PID=$(cat .network-monitor.pid)
    if ps -p $PID > /dev/null; then
        echo "⚠️  Network monitor already running with PID: $PID"
        exit 0
    else
        echo "🗑️  Cleaning up stale PID file"
        rm -f .network-monitor.pid
    fi
fi

# Start network monitor
if [ -f "./network-monitor.js" ]; then
    node ./network-monitor.js &
    MONITOR_PID=$!
    echo $MONITOR_PID > .network-monitor.pid
    echo "✅ Network monitor started with PID: $MONITOR_PID"
    echo "   Monitoring for network activity..."
    
    # Give monitor time to initialize
    sleep 2
    
    # Check if monitor is running
    if ps -p $MONITOR_PID > /dev/null; then
        echo "✅ Network monitor is active"
        exit 0
    else
        echo "❌ Failed to start network monitor"
        rm -f .network-monitor.pid
        exit 1
    fi
else
    echo "❌ Network monitor script not found: ./network-monitor.js"
    exit 1
fi