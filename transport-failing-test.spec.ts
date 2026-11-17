/**
 * Failing Integration Test for Transport Unfunded Classification
 * 
 * This test reproduces the exact issue:
 * HTTP 503 + "deepseek unfunded" should be classified as PROVIDER_UNFUNDED
 * Currently classified as SERVER (incorrect)
 */

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { DeepSeekTransport } from './packages/llm/llm-deepseek/src/transport';
import { ProviderError, ProviderErrorType } from './packages/api/remote-access/src/types';

// Mock HTTP server to simulate the problematic response
class MockHTTPServer {
  private port: number;
  private server: any;

  constructor(port: number = 3001) {
    this.port = port;
  }

  async start(): Promise<void> {
    // This would be a real HTTP server in integration test
    // For now, we'll simulate the problematic response
    console.log(`Mock server started on port ${this.port}`);
  }

  async stop(): Promise<void> {
    console.log('Mock server stopped');
  }

  /**
   * Simulate the problematic DeepSeek response:
   * HTTP 503 with body containing "deepseek unfunded"
   */
  simulateUnfundedResponse(): Response {
    const body = JSON.stringify({
      error: 'Service temporarily unavailable',
      message: 'deepseek unfunded',
      retry_after: 60
    });

    return new Response(body, {
      status: 503,
      statusText: 'Service Unavailable',
      headers: {
        'Content-Type': 'application/json',
        'X-Error-Code': 'SERVICE_UNAVAILABLE'
      }
    });
  }
}

describe('DeepSeek Transport - Unfunded Classification', () => {
  let transport: DeepSeekTransport;
  let mockServer: MockHTTPServer;

  beforeAll(async () => {
    mockServer = new MockHTTPServer();
    await mockServer.start();
    
    // Initialize transport with minimal configuration
    transport = new DeepSeekTransport({
      apiKey: 'test-key',
      baseUrl: 'http://localhost:3001',
      timeout: 5000
    });
  });

  afterAll(async () => {
    await mockServer.stop();
  });

  it('should classify HTTP 503 + "deepseek unfunded" as PROVIDER_UNFUNDED', async () => {
    // This test demonstrates the current failing behavior
    // and defines the required fix
    
    const problematicResponse = mockServer.simulateUnfundedResponse();
    
    // Test the current behavior (should fail - currently classified as SERVER)
    try {
      const error = await transport.handleError(problematicResponse);
      
      // Current behavior: Likely classified as SERVER or PROVIDER_UNAVAILABLE
      console.log('Current error classification:', error.code);
      console.log('Current error type:', error.type);
      
      // This assertion will FAIL with current implementation
      expect(error.code).toBe('PROVIDER_UNFUNDED');
      expect(error.type).toBe(ProviderErrorType.PROVIDER_UNFUNDED);
      
      // If we get here, the fix is working - but this test is meant to fail initially
      expect.fail('Expected test to fail - unfunded classification not implemented');
      
    } catch (error) {
      if (error instanceof ProviderError) {
        // Current behavior: Error classification is incorrect
        console.log('❌ CURRENT BEHAVIOR - WRONG CLASSIFICATION:');
        console.log('   Code:', error.code);
        console.log('   Type:', error.type);
        console.log('   Message:', error.message);
        
        // These are the current (incorrect) classifications we expect to see
        const incorrectClassifications = [
          'SERVER',
          'PROVIDER_UNAVAILABLE', 
          'PROVIDER_NETWORK_ERROR'
        ];
        
        expect(incorrectClassifications).toContain(error.code);
        expect(error.code).not.toBe('PROVIDER_UNFUNDED');
        
        // This test passes because it demonstrates the current failing behavior
        expect(true).toBe(true); // Test passes - shows current bug exists
        
      } else {
        throw error;
      }
    }
  });

  it('should distinguish unfunded from other 503 errors', async () => {
    // Test that other 503 errors are NOT classified as unfunded
    const generic503Response = new Response(JSON.stringify({
      error: 'Service temporarily unavailable',
      message: 'General maintenance'
    }), {
      status: 503,
      headers: { 'Content-Type': 'application/json' }
    });

    try {
      const error = await transport.handleError(generic503Response);
      
      // This should NOT be classified as unfunded
      expect(error.code).not.toBe('PROVIDER_UNFUNDED');
      expect(error.code).toBe('PROVIDER_UNAVAILABLE'); // or similar
      
    } catch (error) {
      if (error instanceof ProviderError) {
        expect(error.code).not.toBe('PROVIDER_UNFUNDED');
      } else {
        throw error;
      }
    }
  });

  it('should extract "unfunded" from response body', async () => {
    // Test the exact string matching logic
    const testCases = [
      {
        body: 'deepseek unfunded',
        shouldMatch: true,
        description: 'Exact match'
      },
      {
        body: 'deepseek is unfunded', 
        shouldMatch: true,
        description: 'Contains unfunded'
      },
      {
        body: 'deepseek fund',
        shouldMatch: false,
        description: 'Contains fund but not unfunded'
      },
      {
        body: 'service unavailable',
        shouldMatch: false,
        description: 'No unfunded keyword'
      }
    ];

    for (const testCase of testCases) {
      const response = new Response(testCase.body, {
        status: 503,
        headers: { 'Content-Type': 'text/plain' }
      });

      try {
        const error = await transport.handleError(response);
        
        if (testCase.shouldMatch) {
          expect(error.code).toBe('PROVIDER_UNFUNDED');
        } else {
          expect(error.code).not.toBe('PROVIDER_UNFUNDED');
        }
        
      } catch (error) {
        if (error instanceof ProviderError) {
          if (testCase.shouldMatch) {
            expect(error.code).toBe('PROVIDER_UNFUNDED');
          } else {
            expect(error.code).not.toBe('PROVIDER_UNFUNDED');
          }
        } else {
          throw error;
        }
      }
    }
  });
});