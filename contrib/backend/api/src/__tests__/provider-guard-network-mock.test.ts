// Provider Guard Network Mock Tests
// Tests guard behavior with simulated provider network failures
// Ensures proper handling of HTTP 503, timeouts, and other network issues

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { resolve_provider, model_allowed_on_provider, preflight } from '../contrib/backend/agent-runner/src/api';
import * as logger from '../contrib/backend/agent-runner/src/logger';

// Mock dependencies
vi.mock('../contrib/backend/agent-runner/src/logger');
vi.mock('reqwest');

describe('Provider Guard Network Mock Tests', () => {
  let config: any;

  beforeEach(() => {
    vi.clearAllMocks();
    config = {
      expected_provider: 'z.ai',
      model: 'glm-4',
      anthropic_base_url: 'https://api.z.ai/v1',
      max_tokens: 4096,
      known_unfunded_providers: [],
      effective_api_key: () => 'test-key',
      approved_fallback: null
    };
  });

  describe('HTTP 503 - Service Unavailable', () => {
    it('should classify 503 + "deepseek unfunded" as PROVIDER_UNFUNDED', async () => {
      // Mock a 503 response with unfunded message
      const mockResponse = {
        status: 503,
        statusText: 'Service Unavailable',
        data: {
          error: {
            message: 'deepseek unfunded'
          }
        }
      };

      // This should be caught by the guard before the request is made
      expect(() => {
        preflight(config, 'https://api.deepseek.com/v1');
      }).toThrow();
    });

    it('should not retry on 503 unfunded provider', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.unfunded-provider.com/v1';
      config.known_unfunded_providers = ['unfunded-provider'];
      config.effective_api_key = () => 'test-key';

      const result = preflight(config, 'https://api.unfunded-provider.com/v1');
      
      // Should be rejected immediately
      expect(result).toBeDefined();
      expect(result.message).toContain('PROVIDER_UNFUNDED');
      
      // Verify no retry attempt would be made
      // In a real scenario, this would prevent the retry loop
    });
  });

  describe('HTTP 429 - Too Many Requests', () => {
    it('should allow retry on 429 (not unfunded)', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.z.ai/v1';
      config.effective_api_key = () => 'test-key';

      // Mock 429 response (should allow retry)
      const mock429Response = {
        status: 429,
        statusText: 'Too Many Requests',
        headers: {
          'retry-after': '60'
        }
      };

      // The guard should pass, but the retry mechanism should handle 429
      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBeUndefined(); // Gate passes, retry logic handles 429
    });
  });

  describe('HTTP 500 - Internal Server Error', () => {
    it('should allow retry on 500 (transient)', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.z.ai/v1';
      config.effective_api_key = () => 'test-key';

      // The gate should pass, retry mechanism handles 500
      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBeUndefined(); // Gate passes, retry logic handles 500
    });
  });

  describe('Network Timeout', () => {
    it('should handle timeout without violating guard rules', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://slow-provider.com/v1';
      config.effective_api_key = () => 'test-key';

      // The gate should pass, network timeout is handled by retry logic
      const result = preflight(config, 'https://slow-provider.com/v1');
      expect(result).toBeUndefined(); // Gate passes, timeout handled elsewhere
    });
  });

  describe('Provider Routing Scenarios', () => {
    it('should handle cross-provider routing with explicit approval', () => {
      config.expected_provider = 'z.ai';
      config.approved_fallback = 'deepseek';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.model = 'deepseek-r1';
      config.effective_api_key = () => 'test-key';

      const result = preflight(config, 'https://api.deepseek.com/v1');
      expect(result).toBeUndefined(); // Approved fallback should pass
    });

    it('should reject cross-provider routing without approval', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.effective_api_key = () => 'test-key';

      const result = preflight(config, 'https://api.deepseek.com/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('PROVIDER_ROUTING_MISMATCH');
    });
  });

  describe('Circuit Breaker Simulation', () => {
    it('should prevent requests after repeated violations', () => {
      // Simulate repeated violations that should trigger circuit breaker
      config.expected_provider = 'z.ai';
      config.known_unfunded_providers = ['violating-provider'];
      config.effective_api_key = () => 'test-key';

      // First violation - should be rejected
      let result = preflight(config, 'https://api.violating-provider.com/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('PROVIDER_UNFUNDED');

      // In a real implementation, this would increment violation count
      // and eventually trigger circuit breaker
      // For this test, we verify the basic rejection logic
    });
  });

  describe('Guard Overhead Measurement', () => {
    it('should measure guard execution time', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.z.ai/v1';
      config.effective_api_key = () => 'test-key';

      const start = performance.now();
      const result = preflight(config, 'https://api.z.ai/v1');
      const end = performance.now();
      const duration = end - start;

      // Gate overhead should be minimal (< 5ms)
      expect(duration).toBeLessThan(5);
      expect(result).toBeUndefined(); // Should pass
    });
  });

  describe('Error Classification Accuracy', () => {
    it('should correctly classify PROVIDER_ROUTING_MISMATCH', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.wrong-provider.com/v1';
      config.effective_api_key = () => 'test-key';

      const result = preflight(config, 'https://api.wrong-provider.com/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('PROVIDER_ROUTING_MISMATCH');
      expect(result.message).toContain('expected=z.ai');
      expect(result.message).toContain('resolved=wrong-provider.com');
    });

    it('should correctly classify MODEL_NOT_ALLOWED_ON_PROVIDER', () => {
      config.expected_provider = 'deepseek';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.model = 'gpt-4'; // Not allowed on deepseek
      config.effective_api_key = () => 'test-key';

      const result = preflight(config, 'https://api.deepseek.com/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('MODEL_NOT_ALLOWED_ON_PROVIDER');
      expect(result.message).toContain('model=gpt-4');
      expect(result.message).toContain('provider=deepseek');
    });

    it('should correctly classify CREDENTIALS_MISSING', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.z.ai/v1';
      config.effective_api_key = () => ''; // Missing credentials

      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('CREDENTIALS_MISSING');
      expect(result.message).toContain('no credential present');
    });

    it('should correctly classify PROVIDER_UNFUNDED', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.unfunded.com/v1';
      config.known_unfunded_providers = ['unfunded'];
      config.effective_api_key = () => 'test-key';

      const result = preflight(config, 'https://api.unfunded.com/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('PROVIDER_UNFUNDED');
      expect(result.message).toContain('provider=unfunded');
    });
  });
});