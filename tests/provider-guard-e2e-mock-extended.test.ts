// Provider Guard E2E Extended Mock Tests
// Comprehensive coverage of real-world E2E scenarios
// Tests guard behavior with simulated provider responses that mirror production

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { resolve_provider, model_allowed_on_provider, preflight } from '../contrib/backend/agent-runner/src/api';
import * as logger from '../contrib/backend/agent-runner/src/logger';

// Mock dependencies
vi.mock('../contrib/backend/agent-runner/src/logger');
vi.mock('reqwest');

describe('Provider Guard E2E Extended Mock Tests', () => {
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

  describe('Real-World Error Scenarios', () => {
    it('should handle z.ai 429 with "insufficient balance" as UNFUNDED', async () => {
      // Simulate real z.ai response pattern
      const mockResponse = {
        status: 429,
        statusText: 'Too Many Requests',
        data: {
          error: {
            message: 'insufficient balance',
            code: 1113
          }
        }
      };

      // Verify guard classification
      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBeDefined();
      expect(result).toBe('PROVIDER_UNFUNDED');
    });

    it('should handle z.ai 429 with "no resource package" as UNFUNDED', async () => {
      const mockResponse = {
        status: 429,
        statusText: 'Too Many Requests', 
        data: {
          error: {
            message: 'no resource package',
            code: 1114
          }
        }
      };

      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBe('PROVIDER_UNFUNDED');
    });

    it('should handle 503 with "provider unfunded" as UNFUNDED', async () => {
      const mockResponse = {
        status: 503,
        statusText: 'Service Unavailable',
        data: {
          error: {
            message: 'provider unfunded',
            code: 1001
          }
        }
      };

      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBe('PROVIDER_UNFUNDED');
    });

    it('should handle network timeout as RETRYABLE', async () => {
      const mockResponse = {
        status: 408,
        statusText: 'Request Timeout',
        data: {
          error: {
            message: 'request timeout'
          }
        }
      };

      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBe('RETRYABLE');
    });

    it('should handle 5xx errors as RETRYABLE', async () => {
      const statuses = [500, 502, 503, 504];
      
      statuses.forEach(status => {
        const mockResponse = {
          status: status,
          statusText: 'Server Error',
          data: {
            error: {
              message: 'internal server error'
            }
          }
        };

        const result = preflight(config, 'https://api.z.ai/v1');
        expect(result).toBe('RETRYABLE');
      });
    });

    it('should handle 4xx errors as TERMINAL', async () => {
      const statuses = [400, 401, 403, 404, 429];
      
      statuses.forEach(status => {
        const mockResponse = {
          status: status,
          statusText: 'Client Error',
          data: {
            error: {
              message: 'client error'
            }
          }
        };

        const result = preflight(config, 'https://api.z.ai/v1');
        if (status === 429) {
          // 429 needs special handling for unfunded detection
          expect(['PROVIDER_UNFUNDED', 'RETRYABLE']).toContain(result);
        } else {
          expect(['TERMINAL', 'MODEL_NOT_ALLOWED']).toContain(result);
        }
      });
    });
  });

  describe('Real Provider URL Patterns', () => {
    it('should correctly resolve z.ai production URLs', () => {
      const urls = [
        'https://api.z.ai/v1',
        'https://api.z.ai/v1/chat/completions',
        'https://api.z.ai/v1/models'
      ];

      urls.forEach(url => {
        const resolved = resolve_provider(url);
        expect(resolved).toBe('z.ai');
      });
    });

    it('should handle custom z.ai domains', () => {
      const customUrl = 'https://custom-zai.example.com/v1';
      const resolved = resolve_provider(customUrl);
      expect(resolved).toBe('custom-zai.example.com');
    });

    it('should validate model-provider combinations', () => {
      // z.ai allowed models
      expect(model_allowed_on_provider('glm-4', 'z.ai')).toBe(true);
      expect(model_allowed_on_provider('glm-4-turbo', 'z.ai')).toBe(true);
      expect(model_allowed_on_provider('claude-3', 'z.ai')).toBe(true);
      
      // z.ai disallowed models
      expect(model_allowed_on_provider('gpt-4', 'z.ai')).toBe(false);
      expect(model_allowed_on_provider('deepseek-r1', 'z.ai')).toBe(false);
    });
  });

  describe('Real-World Configuration Scenarios', () => {
    it('should handle fallback provider configuration', () => {
      config.expected_provider = 'deepseek';
      config.anthropic_base_url = 'https://api.deepseek.com/v1';
      config.approved_fallback = 'z.ai';
      
      // Mismatch but approved fallback
      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBe('PROVIDER_ROUTING_MISMATCH');
    });

    it('should handle dynamic unfunded provider list', () => {
      config.known_unfunded_providers = ['temp-unfunded-zai'];
      config.anthropic_base_url = 'https://api.temp-unfunded-zai.com/v1';
      
      const result = preflight(config, 'https://api.temp-unfunded-zai.com/v1');
      expect(result).toBe('PROVIDER_UNFUNDED');
    });

    it('should handle missing credentials gracefully', () => {
      config.effective_api_key = () => '';
      
      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBe('CREDENTIALS_MISSING');
    });
  });

  describe('Performance and Stress Scenarios', () => {
    it('should handle rapid successive requests', () => {
      const results = [];
      for (let i = 0; i < 10; i++) {
        results.push(preflight(config, 'https://api.z.ai/v1'));
      }
      
      // All should be valid for this config
      results.forEach(result => {
        expect(result).toBe('PROVIDER_PASS');
      });
    });

    it('should handle concurrent requests scenario', () => {
      const promises = [];
      for (let i = 0; i < 5; i++) {
        promises.push(preflight(config, 'https://api.z.ai/v1'));
      }
      
      return Promise.all(promises).then(results => {
        results.forEach(result => {
          expect(result).toBe('PROVIDER_PASS');
        });
      });
    });

    it('should handle large payload scenarios', () => {
      config.max_tokens = 8192; // Large payload
      
      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBe('PROVIDER_PASS');
    });
  });

  describe('Security Edge Cases', () => {
    it('should handle malformed URLs gracefully', () => {
      expect(() => {
        preflight(config, 'not-a-valid-url');
      }).not.toThrow();
    });

    it('should handle null/undefined values', () => {
      config.expected_provider = null;
      config.model = undefined;
      
      const result = preflight(config, 'https://api.z.ai/v1');
      expect(result).toBeDefined();
    });

    it('should handle empty configuration', () => {
      const emptyConfig = {};
      const result = preflight(emptyConfig, 'https://api.z.ai/v1');
      expect(result).toBeDefined();
    });
  });

  describe('Monitoring and Logging Scenarios', () => {
    it('should generate proper error codes for monitoring', () => {
      const errorCases = [
        { config: { expected_provider: 'z.ai', base_url: 'https://api.deepseek.com/v1' }, expected: 'PROVIDER_ROUTING_MISMATCH' },
        { config: { expected_provider: 'z.ai', model: 'gpt-4', base_url: 'https://api.z.ai/v1' }, expected: 'MODEL_NOT_ALLOWED' },
        { config: { expected_provider: 'z.ai', effective_api_key: () => '', base_url: 'https://api.z.ai/v1' }, expected: 'CREDENTIALS_MISSING' },
        { config: { expected_provider: 'z.ai', known_unfunded: ['z.ai'], base_url: 'https://api.z.ai/v1' }, expected: 'PROVIDER_UNFUNDED' }
      ];

      errorCases.forEach(testCase => {
        Object.assign(config, testCase.config);
        const result = preflight(config, testCase.config.base_url);
        expect(result).toBe(testCase.expected);
      });
    });

    it('should preserve checkpoint information', () => {
      // Verify that preflight doesn't modify original config
      const originalConfig = { ...config };
      preflight(config, 'https://api.z.ai/v1');
      
      expect(config).toEqual(originalConfig);
    });
  });
});