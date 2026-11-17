# Pi-AI Stream Timeout Fix Guide

## Problem
- **Error:** "pi-ai stream idle timeout after 300000ms"
- **Root cause:** Default 5-minute timeout in DSH/pi-ai adapter
- **Impact:** Long thinking sessions (>5 minutes) get interrupted

## Solution

### Quick Fix: Applied to Active Configuration

**✅ Already applied to:** `/Users/playom/.dsh/profiles/web/cordis.patch.yml`

```yaml
- id: llm-pi-ai
  config:
    providers:
      # ACTIVE PROVIDER - T27 Ladder (timeout fix applied)
      t27-ladder:
        displayName: T27 Ladder
        apiKeyEnv: T27_LADDER_API_KEY
        api: openai-completions
        baseURL: http://127.0.0.1:3131/v1
        streamIdleTimeoutMs: 3600000  # 1 hour in milliseconds ✅
        timeoutMs: 3600000           # HTTP timeout ✅
        
      # Alternative providers (if needed):
      # ollama:
      #   displayName: Ollama Cloud
      #   api: openai-completions
      #   baseURL: http://127.0.0.1:11434/v1
      #   streamIdleTimeoutMs: 3600000
      #   timeoutMs: 3600000
```

### After Configuration Changes

1. **Restart the harness** to apply the new configuration:
   ```bash
   # The running process keeps old code in memory
   # Restart required for changes to take effect
   ```

2. **Verify the fix** by running a long-thinking session

## Technical Details

### Current Issue
- Default timeout: 300,000ms (5 minutes)
- Error location: `packages/llm/llm-pi-ai/src/adapter.ts:448`
- Error message: `"pi-ai stream idle timeout after ${streamIdleTimeout}ms"`

### Configuration Schema
- File: `packages/llm/llm-pi-ai/src/config.ts`
- Default: `export const DEFAULT_STREAM_IDLE_TIMEOUT_MS = 300_000`
- Validation: Must be positive finite number ≤ MAX_TIMER_DELAY_MS

### Best Practices
1. **For long thinking sessions:** Set to 3600000ms (1 hour)
2. **For production testing:** Start with 1800000ms (30 minutes)
3. **For streaming with heartbeat:** Consider implementing heartbeat mechanism instead of just increasing timeout

### Next Steps (from user's original plan)

1. ✅ **Immediate fix:** Apply `streamIdleTimeoutMs: 3600000` to provider route
2. ✅ **Restart harness:** Done after configuration changes
3. **Test:** Verify long thinking sessions work
4. **t27 transition:** Test t27 native stream path on funded route
5. **Cutover:** Switch default route to t27 native

## File Locations
- **Configuration:** `/Users/playom/t27/cordis.patch.yml`
- **Schema:** `/Users/playom/deepseek-harness/packages/llm/llm-pi-ai/src/config.ts`
- **Adapter:** `/Users/playom/deepseek-harness/packages/llm/llm-pi-ai/src/adapter.ts`