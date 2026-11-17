# T27 Transport Unfunded Classification - Minimal Patch Plan
**Generated**: 2026-10-29T15:00:16+07:00  
**Priority**: P0 - Immediate Fix  
**Disk Impact**: Minimal (no deletion required)  

---

## 🎯 PROBLEM STATEMENT

```
HTTP 503 + "deepseek unfunded" -> PROVIDER_UNFUNDED
```

### Current Behavior
- HTTP 503 errors from DeepSeek provider
- Message contains "unfunded" substring  
- Not properly classified as PROVIDER_UNFUNDED
- May cause improper retry/fallback logic

### Target Behavior  
- HTTP 503 + "unfunded" → PROVIDER_UNFUNDED classification
- Proper error handling for unfunded provider state
- Circuit breaker isolation for unfunded requests

---

## 📁 REQUIRED PATCH FILES

| File | Current Status | Risk Level | Backup Required |
|------|----------------|------------|-----------------|
| `packages/llm/llm-deepseek/src/transport.ts` | Modified | HIGH ❌ | YES |
| `packages/api/remote-access/src/types.ts` | Modified | MEDIUM ⚠️ | YES |
| `packages/client/connection/src/rpc.ts` | Modified | MEDIUM ⚠️ | YES |
| `packages/llm/llm-deepseek/src/retry-policy.ts` | Clean | LOW ✅ | NO |

---

## 🔧 SPECIFIC CHANGES NEEDED

### 1. Transport Classification (`transport.ts`)

**Current Logic (likely):**
```typescript
// Hypothetical current implementation
if (response.status === 503) {
  // Generic 503 handling, not checking "unfunded"
  throw new Error('Provider unavailable');
}
```

**Required Changes:**
```typescript
// Enhanced unfunded classification
if (response.status === 503) {
  const responseText = await response.text();
  if (responseText.includes('unfunded')) {
    throw new ProviderUnfundedError('DeepSeek provider unfunded', {
      code: 'PROVIDER_UNFUNDED',
      originalResponse: response,
      retryable: true,
      metadata: { provider: 'deepseek', status: 503 }
    });
  }
  // Handle other 503 cases
  throw new ProviderUnavailableError('Provider temporarily unavailable', {
    code: 'PROVIDER_UNAVAILABLE',
    originalResponse: response
  });
}
```

### 2. Error Type Definitions (`types.ts`)

**Add new error type:**
```typescript
export class ProviderUnfundedError extends Error {
  public readonly code = 'PROVIDER_UNFUNDED';
  public readonly retryable = true;
  public readonly metadata?: Record<string, any>;
  
  constructor(message: string, options?: {
    code?: string;
    originalResponse?: Response;
    metadata?: Record<string, any>;
  }) {
    super(message);
    this.name = 'ProviderUnfundedError';
    Object.assign(this, options);
  }
}

// Update error classification
export type ProviderErrorType = 
  | 'PROVIDER_UNAVAILABLE'
  | 'PROVIDER_RATE_LIMITED' 
  | 'PROVIDER_UNFUNDED'  // NEW
  | 'PROVIDER_AUTH_ERROR'
  | 'PROVIDER_NETWORK_ERROR';
```

### 3. Client RPC Handling (`rpc.ts`)

**Enhanced error handling:**
```typescript
try {
  const result = await deepSeekCall();
  return result;
} catch (error) {
  if (error instanceof ProviderUnfundedError) {
    // Specific handling for unfunded state
    return {
      success: false,
      error: 'PROVIDER_UNFUNDED',
      message: error.message,
      metadata: error.metadata,
      shouldRetry: error.retryable,
      fallbackRequired: true
    };
  }
  // Handle other provider errors
  throw error;
}
```

### 4. Retry Policy Updates (`retry-policy.ts`)

**Unfunded-specific retry logic:**
```typescript
export const unfundedRetryPolicy: RetryPolicy = {
  maxAttempts: 3,
  baseDelayMs: 1000,
  maxDelayMs: 30000,
  backoffFactor: 2,
  shouldRetry: (error) => {
    return error.code === 'PROVIDER_UNFUNDED' && error.retryable;
  },
  shouldFallback: (error) => {
    return error.code === 'PROVIDER_UNFUNDED';
  }
};
```

---

## 🧪 TEST FILES TO CREATE

| Test File | Purpose | Risk |
|----------|---------|------|
| `packages/llm/llm-deepseek/tests/transport-unfunded.spec.ts` | Unit tests for unfunded classification | LOW ✅ |
| `packages/client/connection/tests/rpc-unfunded.host.spec.ts` | Integration tests for unfunded handling | LOW ✅ |
| `test-transport-unfunded-patch.js` | Manual validation script | LOW ✅ |

---

## 🔍 PRE-PATCH VERIFICATION

### Current State Check
```bash
# Check current transport implementation
git diff HEAD~0 -- packages/llm/llm-deepseek/src/transport.ts

# Check existing test files
find packages/llm/llm-deepseek/tests -name "*unfunded*" 

# Verify no breaking changes
npm test -- packages/llm/llm-deepseek
```

### Backup Before Patch
```bash
# Create backup of modified files
mkdir -p patch-backup/$(date +%Y-%m-%d)
cp packages/llm/llm-deepseek/src/transport.ts patch-backup/$(date +%Y-%m-%d)/
cp packages/api/remote-access/src/types.ts patch-backup/$(date +%Y-%m-%d)/
cp packages/client/connection/src/rpc.ts patch-backup/$(date +%Y-%m-%d)/
```

---

## 📋 PATCH SEQUENCE

### Phase 1: Error Classification (Low Risk)
1. Update `types.ts` with new error type
2. Add unit tests for error classification
3. Test: `npm test -- packages/api/remote-access`

### Phase 2: Transport Logic (Medium Risk)  
1. Update `transport.ts` with unfunded detection
2. Add transport-specific tests
3. Test: `npm test -- packages/llm/llm-deepseek`

### Phase 3: Client Integration (Medium Risk)
1. Update `rpc.ts` with unfunded handling
2. Add integration tests
3. Test: `npm test -- packages/client/connection`

### Phase 4: Validation (Low Risk)
1. Run full test suite
2. Manual testing with actual 503 responses
3. Performance verification

---

## 🚨 RISK MITIGATION

### High Risk Areas
- **DSH modified files**: 31 tracked + 39 untracked changes
- **Active branch**: `fix/cordis-preserve-fiber-state`
- **Worktrees**: 2 active worktrees with potential conflicts

### Mitigation Strategy
1. **Backup all modified files** before patching
2. **Test in isolated environment** first
3. **Rollback plan** ready if issues occur
4. **Minimal changes** - only touch necessary files

### Rollback Commands
```bash
# Emergency rollback
cd /Users/playom/deepseek-harness
git reset --hard HEAD
git clean -fd  # Only if necessary
```

---

## 📊 SPACE REQUIREMENTS

- **Current free space**: 324MB
- **Patch impact**: Minimal (no file growth expected)
- **Testing impact**: Minimal (new tests only)
- **Safe threshold**: 324MB > 100MB required ✅

**Status**: SPACE SUFFICIENT - No cleanup needed

---

## 🎯 IMPLEMENTATION CHECKLIST

- [x] Read-only analysis complete
- [ ] Backup modified files
- [ ] Update error type definitions
- [ ] Implement unfunded detection in transport
- [ ] Update client error handling
- [ ] Add comprehensive tests
- [ ] Run full test suite
- [ ] Manual validation
- [ ] Performance verification

---

## 🔧 EXECUTE COMMANDS

```bash
# Backup current state
mkdir -p patch-backup/$(date +%Y-%m-%d)
cp packages/llm/llm-deepseek/src/transport.ts patch-backup/$(date +%Y-%m-%d)/
cp packages/api/remote-access/src/types.ts patch-backup/$(date +%Y-%m-%d)/
cp packages/client/connection/src/rpc.ts patch-backup/$(date +%Y-%m-%d)/

# Verify current tests pass
npm test -- packages/llm/llm-deepseek
npm test -- packages/api/remote-access  
npm test -- packages/client/connection

# Begin patch implementation (to follow)
```

---

**Status**: READY_FOR_PATCH_IMPLEMENTATION  
**Next**: Apply minimal unfunded classification patch  
**Risk**: MANAGEABLE with proper backup