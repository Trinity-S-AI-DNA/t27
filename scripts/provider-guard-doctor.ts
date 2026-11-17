#!/usr/bin/env node

/**
 * Provider Guard Doctor CLI
 * 
 * This tool provides diagnostic and monitoring capabilities for the provider guard system.
 * It can check guard status, validate configurations, and measure performance.
 * 
 * Usage:
 *   node scripts/provider-guard-doctor.ts --status
 *   node scripts/provider-guard-doctor.ts --validate-config
 *   node scripts/provider-guard-doctor.ts --test-gate
 *   node scripts/provider-guard-doctor.ts --measure-overhead
 *   node scripts/provider-guard-doctor.ts --circuit-breaker-status
 * 
 * Output formats:
 *   --json: Machine-readable JSON output
 *   --human: Human-readable text output (default)
 */

import { readFileSync, existsSync } from 'fs';
import { join } from 'path';
import { program, Option } from 'commander';
import { Config } from '../contrib/backend/agent-runner/src/config';
import { 
  resolve_provider, 
  model_allowed_on_provider, 
  preflight,
  ERR_ROUTING_MISMATCH,
  ERR_MODEL_NOT_ALLOWED,
  ERR_CREDENTIALS_MISSING,
  ERR_PROVIDER_UNFUNDED
} from '../contrib/backend/agent-runner/src/api';

interface GuardMetrics {
  total_checks: number;
  violations_count: number;
  routing_mismatches: number;
  model_not_allowed: number;
  credentials_missing: number;
  provider_unfunded: number;
  average_overhead_ms: number;
  last_check_timestamp: string;
}

interface CircuitBreakerStatus {
  provider: string;
  state: 'closed' | 'half-open' | 'open';
  violation_count: number;
  last_violation: string | null;
  next_check_allowed: string;
}

interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
  suggestions: string[];
}

program
  .name('provider-guard-doctor')
  .description('Provider Guard diagnostic and monitoring tool')
  .version('1.0.0');

program
  .addOption(new Option('--json', 'Output in JSON format'))
  .addOption(new Option('--verbose', 'Verbose output'));

program
  .command('status')
  .description('Show current provider guard status')
  .action(() => {
    const status = getGuardStatus();
    outputResult(status, program.opts().json);
  });

program
  .command('validate-config')
  .description('Validate provider guard configuration')
  .action(() => {
    const validation = validateConfiguration();
    outputResult(validation, program.opts().json);
  });

program
  .command('test-gate')
  .description('Test provider guard with various scenarios')
  .action(() => {
    const testResults = testGateScenarios();
    outputResult(testResults, program.opts().json);
  });

program
  .command('measure-overhead')
  .description('Measure guard execution overhead')
  .action(() => {
    const overhead = measureOverhead();
    outputResult(overhead, program.opts().json);
  });

program
  .command('circuit-breaker-status')
  .description('Show circuit breaker status for all providers')
  .action(() => {
    const circuitBreakerStatus = getCircuitBreakerStatus();
    outputResult(circuitBreakerStatus, program.opts().json);
  });

program
  .command('health-check')
  .description('Comprehensive health check of provider guard system')
  .action(() => {
    const healthCheck = comprehensiveHealthCheck();
    outputResult(healthCheck, program.opts().json);
  });

program.parse();

function outputResult(result: any, jsonFormat: boolean = false) {
  if (jsonFormat) {
    console.log(JSON.stringify(result, null, 2));
  } else {
    outputHumanReadable(result);
  }
}

function outputHumanReadable(result: any) {
  console.log('\n' + '='.repeat(60));
  
  if (result.status === 'success') {
    console.log(`✅ ${result.title || 'Operation Successful'}`);
  } else {
    console.log(`❌ ${result.title || 'Operation Failed'}`);
  }
  
  if (result.message) {
    console.log(`\n${result.message}`);
  }
  
  if (result.data) {
    console.log('\nDetails:');
    if (Array.isArray(result.data)) {
      result.data.forEach((item: any, index: number) => {
        console.log(`  ${index + 1}. ${item}`);
      });
    } else if (typeof result.data === 'object') {
      Object.entries(result.data).forEach(([key, value]) => {
        console.log(`  ${key}: ${value}`);
      });
    } else {
      console.log(`  ${result.data}`);
    }
  }
  
  if (result.errors && result.errors.length > 0) {
    console.log('\nErrors:');
    result.errors.forEach((error: string) => {
      console.log(`  ❌ ${error}`);
    });
  }
  
  if (result.warnings && result.warnings.length > 0) {
    console.log('\nWarnings:');
    result.warnings.forEach((warning: string) => {
      console.log(`  ⚠️  ${warning}`);
    });
  }
  
  if (result.suggestions && result.suggestions.length > 0) {
    console.log('\nSuggestions:');
    result.suggestions.forEach((suggestion: string) => {
      console.log(`  💡 ${suggestion}`);
    });
  }
  
  console.log('='.repeat(60) + '\n');
}

function getGuardStatus(): any {
  try {
    const config = loadConfig();
    const metrics = loadMetrics();
    
    return {
      status: 'success',
      title: 'Provider Guard Status',
      data: {
        expected_provider: config.expected_provider,
        model: config.model,
        known_unfunded_providers: config.known_unfunded_providers,
        approved_fallback: config.approved_fallback,
        total_checks: metrics.total_checks,
        violations_count: metrics.violations_count,
        average_overhead_ms: metrics.average_overhead_ms,
        last_check: metrics.last_check_timestamp
      }
    };
  } catch (error) {
    return {
      status: 'error',
      title: 'Failed to Get Guard Status',
      message: error instanceof Error ? error.message : 'Unknown error',
      errors: [error instanceof Error ? error.message : String(error)]
    };
  }
}

function validateConfiguration(): ValidationResult {
  const config = loadConfig();
  const errors: string[] = [];
  const warnings: string[] = [];
  const suggestions: string[] = [];
  
  // Validate expected provider
  if (!config.expected_provider || config.expected_provider.trim() === '') {
    errors.push('Expected provider is not configured');
  }
  
  // Validate model
  if (!config.model || config.model.trim() === '') {
    errors.push('Model is not configured');
  }
  
  // Validate API key
  if (!config.effective_api_key() || config.effective_api_key().trim() === '') {
    warnings.push('No API key configured - requests will fail with CREDENTIALS_MISSING');
  }
  
  // Validate known unfunded providers
  if (config.known_unfunded_providers && Array.isArray(config.known_unfunded_providers)) {
    if (config.known_unfunded_providers.length === 0) {
      suggestions.push('Consider configuring known unfunded providers for proactive blocking');
    }
  }
  
  // Validate base URL resolution
  if (config.anthropic_base_url) {
    const resolved = resolve_provider(config.anthropic_base_url);
    const expected = config.expected_provider.toLowerCase();
    
    if (resolved !== expected) {
      if (config.approved_fallback && config.approved_fallback.toLowerCase() === resolved) {
        // This is okay - it's an approved fallback
      } else {
        warnings.push(`Base URL resolves to '${resolved}' but expected '${expected}' - this may cause routing violations`);
      }
    }
  }
  
  // Check model allowlist compatibility
  if (config.anthropic_base_url && config.model) {
    const resolved = resolve_provider(config.anthropic_base_url);
    if (!model_allowed_on_provider(config.model, resolved)) {
      errors.push(`Model '${config.model}' is not allowed on provider '${resolved}'`);
    }
  }
  
  return {
    valid: errors.length === 0,
    errors,
    warnings,
    suggestions
  };
}

function testGateScenarios(): any {
  const config = loadConfig();
  const testResults = [];
  
  const scenarios = [
    {
      name: 'Normal Request',
      base_url: config.anthropic_base_url,
      expected: 'pass',
      description: 'Should pass with valid configuration'
    },
    {
      name: 'Routing Mismatch',
      base_url: 'https://api.deepseek.com/v1',
      expected: 'violation',
      description: 'Should reject routing to different provider'
    },
    {
      name: 'Credentials Missing',
      base_url: config.anthropic_base_url,
      expected: 'violation',
      description: 'Should reject when no API key is available'
    }
  ];
  
  scenarios.forEach(scenario => {
    try {
      // Temporarily modify config for test
      const originalBaseUrl = config.anthropic_base_url;
      const originalApiKey = config.effective_api_key;
      
      config.anthropic_base_url = scenario.base_url;
      if (scenario.name === 'Credentials Missing') {
        config.effective_api_key = () => '';
      }
      
      const result = preflight(config, scenario.base_url);
      
      testResults.push({
        name: scenario.name,
        status: result ? 'violation' : 'pass',
        expected: scenario.expected,
        passed: result ? scenario.expected === 'violation' : scenario.expected === 'pass',
        description: scenario.description,
        error: result ? result.message : null
      });
      
      // Restore original config
      config.anthropic_base_url = originalBaseUrl;
      config.effective_api_key = originalApiKey;
      
    } catch (error) {
      testResults.push({
        name: scenario.name,
        status: 'error',
        expected: scenario.expected,
        passed: false,
        description: scenario.description,
        error: error instanceof Error ? error.message : String(error)
      });
    }
  });
  
  const passed = testResults.filter(r => r.passed).length;
  const total = testResults.length;
  
  return {
    status: 'success',
    title: 'Provider Gate Test Results',
    data: {
      passed_tests: `${passed}/${total}`,
      test_results: testResults
    },
    message: `Completed ${total} tests with ${passed} passing`,
    ...(passed < total && {
      warnings: [`Some tests failed - review results above`]
    })
  };
}

function measureOverhead(): any {
  const config = loadConfig();
  const iterations = 100;
  const times: number[] = [];
  
  // Warm up
  for (let i = 0; i < 10; i++) {
    preflight(config, config.anthropic_base_url);
  }
  
  // Measure execution time
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    preflight(config, config.anthropic_base_url);
    const end = performance.now();
    times.push(end - start);
  }
  
  const average = times.reduce((a, b) => a + b, 0) / times.length;
  const max = Math.max(...times);
  const min = Math.min(...times);
  
  return {
    status: 'success',
    title: 'Guard Overhead Measurement',
    data: {
      iterations,
      average_overhead_ms: average.toFixed(3),
      max_overhead_ms: max.toFixed(3),
      min_overhead_ms: min.toFixed(3),
      overhead_percentage: ((average / 100) * 100).toFixed(2), // Assuming 100ms typical request
      threshold_ms: 5,
      meets_requirement: average < 5
    },
    message: `Average overhead: ${average.toFixed(3)}ms ${average < 5 ? '(✅ meets requirement)' : '(❌ exceeds threshold)'}`
  };
}

function getCircuitBreakerStatus(): CircuitBreakerStatus[] {
  // In a real implementation, this would read from persistent storage
  // For now, return mock data for demonstration
  return [
    {
      provider: 'z.ai',
      state: 'closed',
      violation_count: 0,
      last_violation: null,
      next_check_allowed: 'now'
    },
    {
      provider: 'deepseek',
      state: 'half-open',
      violation_count: 3,
      last_violation: '2026-10-30T10:30:00Z',
      next_check_allowed: '2026-10-30T11:00:00Z'
    }
  ];
}

function comprehensiveHealthCheck(): any {
  const configValidation = validateConfiguration();
  const overheadMeasurement = measureOverhead();
  const circuitBreakerStatus = getCircuitBreakerStatus();
  
  const allViolations = circuitBreakerStatus
    .filter(s => s.violation_count > 0)
    .reduce((sum, s) => sum + s.violation_count, 0);
  
  const overallStatus = configValidation.valid && 
                       overheadMeasurement.meets_requirement && 
                       allViolations === 0 ? 'healthy' : 'needs_attention';
  
  return {
    status: 'success',
    title: 'Provider Guard Health Check',
    data: {
      overall_status: overall_status,
      configuration_valid: configValidation.valid,
      overhead_ok: overheadMeasurement.meets_requirement,
      active_violations: allViolations,
      provider_states: circuitBreakerStatus.length,
      circuit_breaker_status: circuitBreakerStatus
    },
    message: overallStatus === 'healthy' ? 
      'Provider guard system is healthy and functioning correctly' :
      'Provider guard system needs attention - see details above',
    ...(configValidation.errors.length > 0 && {
      errors: configValidation.errors
    }),
    ...(configValidation.warnings.length > 0 && {
      warnings: configValidation.warnings
    }),
    ...(configValidation.suggestions.length > 0 && {
      suggestions: configValidation.suggestions
    })
  };
}

function loadConfig(): Config {
  try {
    // Try to load config from environment or default
    return Config.from_env();
  } catch (error) {
    // Create a minimal config for testing
    const config = new Config();
    config.expected_provider = 'z.ai';
    config.model = 'glm-4';
    config.anthropic_base_url = 'https://api.z.ai/v1';
    config.max_tokens = 4096;
    config.known_unfunded_providers = [];
    config.effective_api_key = () => 'test-key';
    return config;
  }
}

function loadMetrics(): GuardMetrics {
  // In a real implementation, this would read from metrics storage
  return {
    total_checks: 0,
    violations_count: 0,
    routing_mismatches: 0,
    model_not_allowed: 0,
    credentials_missing: 0,
    provider_unfunded: 0,
    average_overhead_ms: 0,
    last_check_timestamp: new Date().toISOString()
  };
}