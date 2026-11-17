import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { resolve_provider, model_allowed_on_provider, preflight } from '../contrib/backend/agent-runner/src/api';

// Mock logger to avoid actual logging during tests
vi.mock('../contrib/backend/agent-runner/src/logger');

// Mock global fetch to track network calls
const originalFetch = global.fetch;

describe('Provider Guard Dual-Path Intercept Tests', () => {
  let config: any;

  beforeEach(() => {
    vi.clearAllMocks();
    // Mock the original functions to return expected values
    global.fetch = vi.fn();
    
    config = {
      expected_provider: 'z.ai',
      model: 'glm-4',
      anthropic_base_url: 'https://api.z.ai/v1',
      known_unfunded_providers: [],
      effective_api_key: () => 'test-key',
      approved_fallback: null,
      log_file: '/tmp/test-log.jsonl'
    };
  });

  afterEach(() => {
    global.fetch = originalFetch;
  });

  describe('PROVIDER_UNFUNDED - Zero Network Call Test', () => {
    it('should block unfunded providers before any network I/O', async () => {
      // Configure for unfunded provider
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.unfunded-provider.com/v1';
      config.known_unfunded_providers = ['unfunded-provider'];

      // Track network call attempts
      const fetchSpy = vi.spyOn(global, 'fetch');
      
      // Execute guard - should throw immediately
      expect(() => {
        preflight(config, 'https://api.unfunded-provider.com/v1');
      }).toThrow();

      // CRITICAL TEST: Verify NO network call was attempted
      expect(fetchSpy).not.toHaveBeenCalled();
      expect(fetchSpy).toHaveBeenCalledTimes(0);
      
      // Verify the error is correct
      try {
        preflight(config, 'https://api.unfunded-provider.com/v1');
      } catch (error: any) {
        expect(error.message).toContain('PROVIDER_UNFUNDED');
        expect(error.message).toContain('unfunded-provider');
      }
    });

    it('should not make HTTP requests even when network is available', async () => {
      // Mock a successful network response
      (global.fetch as any).mockResolvedValue({
        status: 200,
        json: () => ({ data: 'success' })
      });

      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.unfunded-provider.com/v1';
      config.known_unfunded_providers = ['unfunded-provider'];

      // Execute guard - should reject before network I/O
      expect(() => {
        preflight(config, 'https://api.unfunded-provider.com/v1');
      }).toThrow();

      // Verify network was never reached despite being mocked
      expect(fetchSpy).not.toHaveBeenCalled();
      expect((global.fetch as any).mock.calls.length).toBe(0);
    });
  });

  describe('PROVIDER_ROUTING_MISMATCH - Zero Network Call Test', () => {
    it('should block routing mismatch before any network I/O', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.known_unfunded_providers = [];

      const fetchSpy = vi.spyOn(global, 'fetch');

      // Should throw immediately
      expect(() => {
        preflight(config, 'https://api.deepseek.com/v1');
      }).toThrow();

      // Verify no network call
      expect(fetchSpy).not.toHaveBeenCalled();
    });
  });

  describe('CREDENTIAL_ERROR - Zero Network Call Test', () => {
    it('should block missing credentials before any network I/O', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.z.ai/v1';
      config.effective_api_key = () => ''; // Empty credentials

      const fetchSpy = vi.spyOn(global, 'fetch');

      expect(() => {
        preflight(config, 'https://api.z.ai/v1');
      }).toThrow();

      expect(fetchSpy).not.toHaveBeenCalled();
    });
  });

  describe('MODEL_COMPATIBILITY - Zero Network Call Test', () => {
    it('should block incompatible models before any network I/O', () => {
      config.expected_provider = 'deepseek';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.model = 'glm-4'; // GLM model on deepseek provider

      const fetchSpy = vi.spyOn(global, 'fetch');

      expect(() => {
        preflight(config, 'https://api.deepseek.com/v1');
      }).toThrow();

      expect(fetchSpy).not.toHaveBeenCalled();
    });
  });

  describe('POSITIVE - Allow Normal Requests', () => {
    it('should allow valid requests to proceed', () => {
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.z.ai/v1';
      config.known_unfunded_providers = [];

      // Valid case should not throw
      expect(() => {
        preflight(config, 'https://api.z.ai/v1');
      }).not.toThrow();

      // Note: In a real integration, this would be where network calls happen
      // but this test focuses on guard behavior only
    });
  });

  describe('Dual-Path Evidence Collection', () => {
    it('should provide observable evidence of guard interception', () => {
      const consoleSpy = vi.spyOn(console, 'log');
      
      config.expected_provider = 'z.ai';
      config.anthropic_base_url = 'https://api.unfunded-provider.com/v1';
      config.known_unfunded_providers = ['unfunded-provider'];

      // Execute guard
      try {
        preflight(config, 'https://api.unfunded-provider.com/v1');
      } catch (error) {
        // Log the interception evidence
        console.log('GUARD_INTERCEPTED:', error.message);
      }

      // Verify evidence was logged
      expect(consoleSpy).toHaveBeenCalledWith(
        expect.stringContaining('GUARD_INTERCEPTED')
      );
    });
  });
});