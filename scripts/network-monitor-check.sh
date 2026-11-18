#!/bin/bash

# Network Monitor Check Script
# Checks network monitoring results for provider guard verification

EXPECT_ZERO_IO=false
VERBOSE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --expect-zero-io)
            EXPECT_ZERO_IO=true
            shift
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--expect-zero-io] [--verbose]"
            exit 1
            ;;
    esac
done

echo "🔍 Checking Network Monitor Results..."

# Check if network monitor is running
if [ -f ".network-monitor.pid" ]; then
    PID=$(cat .network-monitor.pid)
    if ps -p $PID > /dev/null; then
        echo "⚠️  Network monitor still running with PID: $PID"
        echo "   Please stop the monitor first: ./scripts/network-monitor-stop.sh"
        exit 1
    fi
fi

# Check if network log exists
if [ ! -f ".network-monitor.log" ]; then
    echo "❌ Network monitor log not found"
    echo "   Did you start the monitor with: ./scripts/network-monitor-start.sh"
    exit 1
fi

# Analyze network log
echo "📊 Analyzing network activity log..."

# Count total lines in log
TOTAL_LINES=$(wc -l < .network-monitor.log | tr -d ' ')
echo "   Total log lines: $TOTAL_LINES"

# Count network-related activity
NETWORK_KEYWORDS=("HTTP" "HTTPS" "socket" "request" "response" "curl" "wget" "connection")
NETWORK_ACTIVITY=0

for keyword in "${NETWORK_KEYWORDS[@]}"; do
    COUNT=$(grep -i "$keyword" .network-monitor.log | wc -l | tr -d ' ')
    if [ $COUNT -gt 0 ]; then
        NETWORK_ACTIVITY=$((NETWORK_ACTIVITY + COUNT))
        if [ "$VERBOSE" = true ]; then
            echo "   $keyword: $COUNT occurrences"
        fi
    fi
done

echo "   Network activity detected: $NETWORK_KEYWORDS"

# Check for zero I/O expectation
if [ "$EXPECT_ZERO_IO" = true ]; then
    if [ $NETWORK_ACTIVITY -eq 0 ]; then
        echo "✅ PASS: Zero network I/O as expected"
        echo "   No network activity detected"
        exit 0
    else
        echo "❌ FAIL: Network activity detected when none expected"
        echo "   Found $NETWORK_KEYWORDS network-related activities"
        
        if [ "$VERBOSE" = true ]; then
            echo "   Network activity details:"
            grep -i "HTTP\|HTTPS\|socket\|request\|response" .network-monitor.log | head -10
        fi
        
        exit 1
    fi
else
    if [ $NETWORK_ACTIVITY -gt 0 ]; then
        echo "✅ PASS: Network activity detected as expected"
        echo "   Found $NETWORK_KEYWORDS network-related activities"
        exit 0
    else
        echo "⚠️  WARNING: No network activity detected"
        echo "   This may be expected or indicate monitoring issues"
        exit 0
    fi
fi