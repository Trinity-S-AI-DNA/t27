#!/bin/bash
# Temporary helper script for graceful DSH restart
# Note: This is a temporary helper - canonical verification must use .t27 skill

set -euo pipefail

echo "🔄 Graceful DSH Restart Helper"
echo "============================="

# Find DSH processes
echo "🔍 Finding DSH processes..."
DSH_PIDS=$(pgrep -f "deepseek.*harness" || true)

if [ -z "$DSH_PIDS" ]; then
    echo "✅ No DSH processes found - already stopped"
else
    echo "📋 Found DSH processes: $DSH_PIDS"
    
    # Try graceful shutdown first
    echo "🛑 Attempting graceful shutdown..."
    for pid in $DSH_PIDS; do
        if kill -TERM $pid 2>/dev/null; then
            echo "⏳ SIGTERM sent to PID: $pid"
        fi
    done
    
    # Wait for graceful shutdown
    echo "⏱️ Waiting for graceful shutdown (10s timeout)..."
    sleep 10
    
    # Check if processes are still running
    STILL_RUNNING=""
    for pid in $DSH_PIDS; do
        if kill -0 $pid 2>/dev/null; then
            STILL_RUNNING="$STILL_RUNNING $pid"
        fi
    done
    
    if [ -n "$STILL_RUNNING" ]; then
        echo "⚠️  Processes still running: $STILL_RUNNING"
        echo "🔨 Force termination required..."
        for pid in $STILL_RUNNING; do
            if kill -KILL $pid 2>/dev/null; then
                echo "🔥 KILL sent to PID: $pid"
            fi
        done
        sleep 2
    else
        echo "✅ Graceful shutdown successful"
    fi
fi

echo "🚀 Starting DSH..."
cd /Users/playom/deepseek-harness
pnpm run dev:web &

echo "⏱️ Waiting for DSH to start..."
sleep 15

# Check if DSH is running
if pgrep -f "deepseek.*harness" > /dev/null; then
    NEW_PID=$(pgrep -f "deepseek.*harness")
    echo "✅ DSH restarted successfully with PID: $NEW_PID"
    
    # Test accessibility
    if curl -s --max-time 5 "http://127.0.0.1:3080" > /dev/null; then
        echo "✅ DSH web interface accessible"
    else
        echo "⚠️  DSH web interface not responding"
    fi
else
    echo "❌ DSH failed to start"
    exit 1
fi

echo "🎯 Ready for .t27 verification skill execution"