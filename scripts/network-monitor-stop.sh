#!/bin/bash

# Network Monitor Stop Script
# Stops network monitoring and generates report

echo "📡 Stopping Network Monitor..."

# Check if network monitor is running
if [ -f ".network-monitor.pid" ]; then
    PID=$(cat .network-monitor.pid)
    if ps -p $PID > /dev/null; then
        echo "🛑 Sending SIGTERM to network monitor (PID: $PID)..."
        kill -TERM $PID
        
        # Wait for graceful shutdown
        for i in {1..10}; do
            if ! ps -p $PID > /dev/null; then
                echo "✅ Network monitor stopped gracefully"
                break
            fi
            sleep 1
        done
        
        # Force kill if still running
        if ps -p $PID > /dev/null; then
            echo "⚠️  Force killing network monitor..."
            kill -KILL $PID
        fi
    else
        echo "⚠️  Network monitor PID file exists but process is not running"
    fi
    
    # Clean up PID file
    rm -f .network-monitor.pid
else
    echo "ℹ️  Network monitor not running"
fi

# Generate report if log exists
if [ -f ".network-monitor.log" ]; then
    echo "📊 Generating network monitoring report..."
    
    REPORT_FILE="network-monitor-report-$(date +%Y%m%d-%H%M%S).txt"
    
    {
        echo "Network Monitoring Report"
        echo "======================"
        echo "Generated: $(date)"
        echo ""
        
        echo "Log Summary:"
        echo "  Total lines: $(wc -l < .network-monitor.log | tr -d ' ')"
        echo "  Log size: $(du -h .network-monitor.log | cut -f1)"
        echo ""
        
        echo "Network Activity Analysis:"
        echo "  HTTP/HTTPS requests: $(grep -i "HTTP" .network-monitor.log | wc -l | tr -d ' ')"
        echo "  Socket connections: $(grep -i "socket" .network-monitor.log | wc -l | tr -d ' ')"
        echo "  Network requests: $(grep -i "request" .network-monitor.log | wc -l | tr -d ' ')"
        echo "  Responses: $(grep -i "response" .network-monitor.log | wc -l | tr -d ' ')"
        echo "  curl/wget usage: $(grep -i "curl\|wget" .network-monitor.log | wc -l | tr -d ' ')"
        echo ""
        
        echo "Activity Timeline:"
        echo "$(head -20 .network-monitor.log)"
        echo ""
        echo "..."
        echo ""
        
        echo "Latest Activity:"
        echo "$(tail -10 .network-monitor.log)"
        
    } > "$REPORT_FILE"
    
    echo "✅ Report saved to: $REPORT_FILE"
    
    # Keep only the last 5 reports
    ls -t network-monitor-report-*.txt | tail -n +6 | xargs rm -f
else
    echo "⚠️  Network monitor log not found"
fi

echo "🧹 Network monitor cleanup complete"