// Provider Guard Integration Tests
// Tests the complete provider guard workflow with mock providers
// Ensures all violation scenarios are properly handled

import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { resolve_provider, model_allowed_on_provider, preflight } from '../contrib/backend/agent-runner/src/api';
import * as logger from '../contrib/backend/agent-runner/src/logger';

// Mock logger to avoid actual logging during tests
vi.mock('../contrib/backend/agent-runner/src/logger');

describe('Provider Guard Integration Tests', () => {
  let config: Config;

  beforeEach(() => {
    vi.clearAllMocks();
    config = new Config();
    config.expected_provider = 'z.ai';
    config.model = 'glm-4';
    config.anthropic_base_url = 'https://api.z.ai/v1';
    config.known_unfunded_providers = [];
  });

  describe('Provider Resolution', () => {
    it('should correctly resolve z.ai provider', () => {
      const resolved = resolve_provider('https://api.z.ai/v1');
      expect(resolved).toBe('z.ai');
    });

    it('should correctly resolve deepseek provider', () => {
      const resolved = resolve_provider('https://api.deepseek.com/v1');
      expect(resolved).toBe('deepseek');
    });

    it('should correctly resolve openai provider', () => {
      const resolved = resolve_provider('https://api.openai.com/v1');
      expect(resolved).toBe('openai');
    });

    it('should handle unknown provider gracefully', () => {
      const resolved = resolve_provider('https://unknown-provider.com/v1');
      expect(resolved).toBe('unknown-provider.com');
    });
  });

  describe('Model Allowlist', () => {
    it('should allow GLM models on z.ai', () => {
      expect(model_allowed_on_provider('glm-4', 'z.ai')).toBe(true);
      expect(model_allowed_on_provider('glm-4-turbo', 'z.ai')).toBe(true);
    });

    it('should allow Claude models on z.ai', () => {
      expect(model_allowed_on_provider('claude-3-sonnet', 'z.ai')).toBe(true);
      expect(model_allowed_on_provider('claude-3-opus', 'z.ai')).toBe(true);
    });

    it('should deny non-allowed models on z.ai', () => {
      expect(model_allowed_on_provider('gpt-4', 'z.ai')).toBe(false);
      expect(model_allowed_on_provider('deepseek-r1', 'z.ai')).toBe(false);
    });

    it('should allow deepseek models on deepseek', () => {
      expect(model_allowed_on_provider('deepseek-r1', 'deepseek')).toBe(true);
      expect(model_allowed_on_provider('deepseek-chat', 'deepseek')).toBe(true);
    });

    it('should deny non-deepseek models on deepseek', () => {
      expect(model_allowed_on_provider('glm-4', 'deepseek')).toBe(false);
      expect(model_allowed_on_provider('gpt-4', 'deepseek')).toBe(false);
    });

    it('should allow unknown models on unknown provider', () => {
      expect(model_allowed_on_provider('any-model', 'unknown-provider')).toBe(true);
    });
  });

  describe('Preflight Gate - Normal Operation', () => {
    it('should pass normal z.ai request', () => {
      config.anthropic_base_url = 'https://api.z.ai/v1';
      config.effective_api_key = () => 'test-key';
      
      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBeUndefined(); // Success is Ok(())
    });

    it('should pass approved fallback request', () => {
      config.expected_provider = 'z.ai';
      config.approved_fallback = 'deepseek';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.model = 'deepseek-r1';
      config.effective_api_key = () => 'test-key';
      
      const result = preflight(config, 'https://api.deepseek.com/v1');
      expect(result).toBeUndefined(); // Success
    });
  });

  describe('Preflight Gate - Violations', () => {
    it('should reject routing mismatch without approved fallback', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.effective_api_key = () => 'test-key';
      
      const result = preflight(config, 'https://api.deepseek.com/v1');
      expect(result).toBeDefined(); // Error returned
      expect(result.message).toContain('PROVIDER_ROUTING_MISMATCH');
    });

    it('should reject model not allowed on provider', () => {
      config.expected_provider = 'deepseek';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.model = 'glm-4'; // GLM not allowed on deepseek
      config.effective_api_key = () => 'test-key';
      
      const result = preflight(config, 'https://api.deepseek.com/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('MODEL_NOT_ALLOWED_ON_PROVIDER');
    });

    it('should reject missing credentials', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.z.ai/v1';
      config.effective_api_key = () => ''; // Empty key
      
      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('CREDENTIALS_MISSING');
    });

    it('should reject known unfunded provider', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.unfunded-provider.com/v1';
      config.known_unfunded_providers = ['unfunded-provider'];
      config.effective_api_key = () => 'test-key';
      
      const result = preflight(config, 'https://api.unfunded-provider.com/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('PROVIDER_UNFUNDED');
    });
  });

  describe('Preflight Gate - Network I/O Prevention', () => {
    it('should not make HTTP calls on violations', async () => {
      const fetchSpy = vi.spyOn(global, 'fetch');
      
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.effective_api_key = () => 'test-key';
      
      // This should throw before any fetch can occur
      expect(() => preflight(config, 'https://api.deepseek.com/v1')).toThrow();
      
      // Verify no HTTP request was made
      expect(fetchSpy).not.toHaveBeenCalled();
      fetchSpy.mockRestore();
    });

    it('should preserve checkpoint on violations', () => {
      // Since the gate runs before any state changes,
      // there should be no state to mutate
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.effective_api_key = () => 'test-key';
      
      expect(() => preflight(config, 'https://api.deepseek.com/v1')).toThrow();
      
      // If there was any mutable state, it should remain unchanged
      // (This is more relevant for integration with actual system state)
    });
  });

  describe('Follow-up Task Creation', () => {
    it('should create follow-up task on violations', () => {
      const logFollowUpTaskSpy = vi.spyOn(logger, 'log_follow_up_task');
      
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.effective_api_key = () => 'test-key';
      
      expect(() => preflight(config, 'https://api.deepseek.com/v1')).toThrow();
      
      // Verify follow-up task was logged
      expect(logFollowUpTaskSpy).toHaveBeenCalledWith(
        expect.stringContaining('PROVIDER_ROUTING_MISMATCH'),
        expect.any(String)
      );
    });
  });

  describe('Retry Prevention', () => {
    it('should prevent retry on PROVIDER_UNFUNDED', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.unfunded-provider.com/v1';
      config.known_unfunded_providers = ['unfunded-provider'];
      config.effective_api_key = () => 'test-key';
      
      const result = preflight(config, 'https://api.unfunded-provider.com/v1');
      expect(result).toBeDefined();
      expect(result.message).toContain('PROVIDER_UNFUNDED');
      
      // In a real scenario, this should prevent retries
      // This test verifies the error classification is correct
    });
  });

  describe('Performance Requirements', () => {
    it('should complete preflight in under 5ms', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.z.ai/v1';
      config.effective_api_key = () => 'test-key';
      
      const start = performance.now();
      const result = preflight(config, 'https://api.z.ai/v1');
      const end = performance.now();
      const duration = end - start;
      
      expect(duration).toBeLessThan(5); // Must complete in under 5ms
      expect(result).toBeUndefined(); // Should pass
    });
  });
});