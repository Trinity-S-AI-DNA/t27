#!/bin/bash
# M2 UART Evidence Capture Script
# Monitor AX7203 UART output to verify bench007_probe functionality

SERIAL_DEVICE="/dev/cu.usbserial-210512180081"
BAUD_RATE="115200"
OUTPUT_FILE="uart_evidence_$(date +%Y%m%d_%H%M%S).txt"
TIMEOUT_SECONDS=10

echo "=== M2 UART Evidence Capture ==="
echo "Device: $SERIAL_DEVICE"
echo "Baud: $BAUD_RATE" 
echo "Duration: $TIMEOUT_SECONDS seconds"
echo "Output file: $OUTPUT_FILE"
echo "=================================="

# Check if device exists
if [ ! -c "$SERIAL_DEVICE" ]; then
    echo "ERROR: Serial device not found: $SERIAL_DEVICE"
    exit 1
fi

echo "Starting UART capture..."
echo "Capturing $TIMEOUT_SECONDS seconds of output..."

# Use cat with timeout for simple capture
{
    cat "$SERIAL_DEVICE" &
    CAT_PID=$!
    sleep $TIMEOUT_SECONDS
    kill -9 $CAT_PID 2>/dev/null
} > "$OUTPUT_FILE" 2>&1

echo "UART capture complete."

# Check if we got any data
if [ -s "$OUTPUT_FILE" ]; then
    echo "✅ SUCCESS: UART data captured!"
    echo "File size: $(wc -c < "$OUTPUT_FILE") bytes"
    echo "First few lines:"
    head -10 "$OUTPUT_FILE"
    echo ""
    echo "Last few lines:"
    tail -10 "$OUTPUT_FILE"
else
    echo "⚠️  WARNING: No UART data captured in $TIMEOUT_SECONDS seconds"
    echo "This could mean:"
    echo "  1. bench007_probe is not sending data"
    echo "  2. Wrong baud rate or device"
    echo "  3. Communication issue"
fi

echo ""
echo "=== UART Evidence Analysis ==="

# Look for specific patterns
if grep -q "RESULT" "$OUTPUT_FILE" 2>/dev/null; then
    echo "✅ bench007_probe RESULT messages found"
    grep "RESULT" "$OUTPUT_FILE"
elif grep -q "[0-9]" "$OUTPUT_FILE" 2>/dev/null; then
    echo "✅ Numeric data found (possible telemetry)"
    grep -o "[0-9]+" "$OUTPUT_FILE" | head -10
else
    echo "⚠️  No recognizable patterns found in UART output"
fi

echo "=== Complete Evidence File ==="
echo "Full UART log saved to: $OUTPUT_FILE"
echo "Evidence file contents:"
cat "$OUTPUT_FILE"