#!/usr/bin/env node

/**
 * Simple Provider Guard Diagnostic Tool
 * 
 * This tool provides basic diagnostic capabilities for the provider guard system.
 * It checks configurations and provides status information without external dependencies.
 * 
 * Usage:
 *   node scripts/provider-guard-simple-doctor.ts <command>
 * 
 * Commands:
 *   status     - Show current provider guard status
 *   validate   - Validate provider guard configuration  
 *   help       - Show this help message
 */

import { readFileSync, existsSync } from 'fs';
import { join } from 'path';

interface GuardStatus {
  status: 'healthy' | 'needs_attention' | 'degraded';
  details: {
    spec_exists: boolean;
    api_impl_exists: boolean;
    config_impl_exists: boolean;
    has_preflight: boolean;
    has_error_constants: boolean;
    has_circuit_breaker: boolean;
    has_invariants: boolean;
  };
  last_check: string;
  recommendations: string[];
}

async function main() {
  const args = process.argv.slice(2);
  
  if (args.length === 0 || args[0] === 'help') {
    showHelp();
    return;
  }
  
  const command = args[0];
  
  switch (command) {
    case 'status':
      showStatus();
      break;
    case 'validate':
      validateConfiguration();
      break;
    case 'test':
      await runBasicTests();
      break;
    default:
      console.error(`Unknown command: ${command}`);
      showHelp();
      process.exit(1);
  }
}

function showHelp() {
  console.log(`
Provider Guard Diagnostic Tool

Usage: node scripts/provider-guard-simple-doctor.ts <command>

Commands:
  status     Show current provider guard status
  validate   Validate provider guard configuration
  test       Run basic tests
  help       Show this help message

Examples:
  node scripts/provider-guard-simple-doctor.ts status
  node scripts/provider-guard-simple-doctor.ts validate
  node scripts/provider-guard-simple-doctor.ts test
`);
}

function showStatus() {
  console.log('Provider Guard Status');
  console.log('====================');
  
  const status = getGuardStatus();
  
  console.log(`Status: ${status.status}`);
  console.log(`Last Check: ${status.last_check}`);
  console.log('');
  console.log('Details:');
  
  Object.entries(status.details).forEach(([key, value]) => {
    const icon = value ? '✓' : '✗';
    console.log(`  ${icon} ${key.replace(/_/g, ' ')}: ${value ? 'Yes' : 'No'}`);
  });
  
  if (status.recommendations.length > 0) {
    console.log('');
    console.log('Recommendations:');
    status.recommendations.forEach(rec => {
      console.log(`  💡 ${rec}`);
    });
  }
  
  console.log('');
  console.log('Summary:');
  console.log(`  Implementation: ${status.status === 'healthy' ? 'READY' : 'NEEDS WORK'}`);
}

function validateConfiguration() {
  console.log('Provider Guard Configuration Validation');
  console.log('=====================================');
  
  const results: string[] = [];
  const warnings: string[] = [];
  const errors: string[] = [];
  
  // Check if files exist
  if (existsSync('specs/system/llm_request_provider_gate.t27')) {
    results.push('Provider guard specification exists');
  } else {
    errors.push('Provider guard specification missing');
  }
  
  if (existsSync('contrib/backend/agent-runner/src/api.rs')) {
    results.push('API implementation exists');
  } else {
    errors.push('API implementation missing');
  }
  
  if (existsSync('contrib/backend/agent-runner/src/config.rs')) {
    results.push('Configuration implementation exists');
  } else {
    errors.push('Configuration implementation missing');
  }
  
  // Check API implementation content
  if (existsSync('contrib/backend/agent-runner/src/api.rs')) {
    const apiContent = readFileSync('contrib/backend/agent-runner/src/api.rs', 'utf8');
    
    if (apiContent.includes('pub fn preflight')) {
      results.push('Preflight function implemented');
    } else {
      errors.push('Preflight function missing');
    }
    
    if (apiContent.includes('ERR_PROVIDER_UNFUNDED')) {
      results.push('PROVIDER_UNFUNDED error constant defined');
    } else {
      errors.push('PROVIDER_UNFUNDED error constant missing');
    }
    
    if (apiContent.includes('resolve_provider')) {
      results.push('Provider resolution function implemented');
    } else {
      errors.push('Provider resolution function missing');
    }
  }
  
  // Check spec content
  if (existsSync('specs/system/llm_request_provider_gate.t27')) {
    const specContent = readFileSync('specs/system/llm_request_provider_gate.t27', 'utf8');
    
    const requiredVerdicts = [
      'VERDICT_PASS',
      'VERDICT_ROUTING_MISMATCH', 
      'VERDICT_MODEL_NOT_ALLOWED',
      'VERDICT_CREDENTIALS_MISSING',
      'VERDICT_PROVIDER_UNFUNDED'
    ];
    
    let missingVerdicts = 0;
    requiredVerdicts.forEach(verdict => {
      if (specContent.includes(verdict)) {
        results.push(`Verdict ${verdict} defined`);
      } else {
        missingVerdicts++;
        errors.push(`Verdict ${verdict} missing`);
      }
    });
    
    if (specContent.includes('invariant')) {
      results.push('Invariants defined');
    } else {
      warnings.push('No invariants defined (recommended for safety)');
    }
    
    if (specContent.includes('circuit_breaker')) {
      results.push('Circuit breaker functionality defined');
    } else {
      warnings.push('Circuit breaker functionality not implemented');
    }
  }
  
  // Display results
  console.log('Checks:');
  results.forEach(result => {
    console.log(`  ✅ ${result}`);
  });
  
  if (warnings.length > 0) {
    console.log('');
    console.log('Warnings:');
    warnings.forEach(warning => {
      console.log(`  ⚠️  ${warning}`);
    });
  }
  
  if (errors.length > 0) {
    console.log('');
    console.log('Errors:');
    errors.forEach(error => {
      console.log(`  ❌ ${error}`);
    });
  }
  
  console.log('');
  console.log(`Result: ${errors.length === 0 ? 'VALID' : 'INVALID'}`);
  console.log(`Checks: ${results.length + warnings.length + errors.length}`);
  console.log(`Passed: ${results.length}`);
  console.log(`Warnings: ${warnings.length}`);
  console.log(`Errors: ${errors.length}`);
}

async function runBasicTests() {
  console.log('Provider Guard Basic Tests');
  console.log('========================');
  
  let passed = 0;
  let total = 0;
  
  // Test 1: Check file existence
  total++;
  if (existsSync('specs/system/llm_request_provider_gate.t27')) {
    console.log('✅ Specification file exists');
    passed++;
  } else {
    console.log('❌ Specification file missing');
  }
  
  // Test 2: Check API implementation
  total++;
  if (existsSync('contrib/backend/agent-runner/src/api.rs')) {
    console.log('✅ API implementation exists');
    passed++;
  } else {
    console.log('❌ API implementation missing');
  }
  
  // Test 3: Check Rust compilation
  total++;
  console.log('🔧 Testing Rust compilation...');
  try {
    const { execSync } = await import('child_process');
    execSync('cargo check --manifest-path contrib/backend/agent-runner/Cargo.toml', { 
      stdio: 'pipe' 
    });
    console.log('✅ Rust compilation successful');
    passed++;
  } catch (error) {
    console.log('❌ Rust compilation failed');
  }
  
  // Test 4: Check critical functions in API
  if (existsSync('contrib/backend/agent-runner/src/api.rs')) {
    total++;
    const apiContent = readFileSync('contrib/backend/agent-runner/src/api.rs', 'utf8');
    if (apiContent.includes('fn preflight')) {
      console.log('✅ Preflight function exists');
      passed++;
    } else {
      console.log('❌ Preflight function missing');
    }
  }
  
  // Test 5: Check critical constants
  if (existsSync('contrib/backend/agent-runner/src/api.rs')) {
    total++;
    const apiContent = readFileSync('contrib/backend/agent-runner/src/api.rs', 'utf8');
    if (apiContent.includes('ERR_PROVIDER_UNFUNDED')) {
      console.log('✅ PROVIDER_UNFUNDED constant exists');
      passed++;
    } else {
      console.log('❌ PROVIDER_UNFUNDED constant missing');
    }
  }
  
  // Test 6: Check spec completeness
  if (existsSync('specs/system/llm_request_provider_gate.t27')) {
    total++;
    const specContent = readFileSync('specs/system/llm_request_provider_gate.t27', 'utf8');
    const hasVerdicts = [
      'VERDICT_PASS',
      'VERDICT_ROUTING_MISMATCH',
      'VERDICT_MODEL_NOT_ALLOWED', 
      'VERDICT_CREDENTIALS_MISSING',
      'VERDICT_PROVIDER_UNFUNDED'
    ].every(verdict => specContent.includes(verdict));
    
    if (hasVerdicts) {
      console.log('✅ All required verdicts defined');
      passed++;
    } else {
      console.log('❌ Some verdicts missing from specification');
    }
  }
  
  // Test 7: Check invariants
  if (existsSync('specs/system/llm_request_provider_gate.t27')) {
    total++;
    const specContent = readFileSync('specs/system/llm_request_provider_gate.t27', 'utf8');
    const hasInvariants = specContent.includes('invariant');
    
    if (hasInvariants) {
      console.log('✅ Invariants defined');
      passed++;
    } else {
      console.log('⚠️  No invariants defined (optional but recommended)');
      passed++; // Don't fail for missing invariants
    }
  }
  
  console.log('');
  console.log(`Test Results: ${passed}/${total} passed`);
  
  if (passed === total) {
    console.log('🎉 All tests passed!');
    return true;
  } else {
    console.log('❌ Some tests failed');
    return false;
  }
}

function getGuardStatus(): GuardStatus {
  const details = {
    spec_exists: existsSync('specs/system/llm_request_provider_gate.t27'),
    api_impl_exists: existsSync('contrib/backend/agent-runner/src/api.rs'),
    config_impl_exists: existsSync('contrib/backend/agent-runner/src/config.rs'),
    has_preflight: false,
    has_error_constants: false,
    has_circuit_breaker: false,
    has_invariants: false
  };
  
  const recommendations: string[] = [];
  
  if (details.spec_exists) {
    const specContent = readFileSync('specs/system/llm_request_provider_gate.t27', 'utf8');
    details.has_invariants = specContent.includes('invariant');
    details.has_circuit_breaker = specContent.includes('circuit_breaker');
  }
  
  if (details.api_impl_exists) {
    const apiContent = readFileSync('contrib/backend/agent-runner/src/api.rs', 'utf8');
    // Check for provider guard functions (may need to be added)
    details.has_preflight = apiContent.includes('preflight') || apiContent.includes('resolve_provider');
    details.has_error_constants = apiContent.includes('ERR_PROVIDER_UNFUNDED') || apiContent.includes('ROUTING_MISMATCH');
  }
  
  // Determine overall status
  let status: 'healthy' | 'needs_attention' | 'degraded' = 'healthy';
  
  if (!details.spec_exists || !details.api_impl_exists) {
    status = 'degraded';
    recommendations.push('Core implementation files missing');
  } else if (!details.has_preflight || !details.has_error_constants) {
    status = 'needs_attention';
    recommendations.push('Preflight function or error constants missing');
  } else if (!details.has_invariants) {
    status = 'needs_attention';
    recommendations.push('Invariants recommended for safety');
  }
  
  if (!details.has_circuit_breaker) {
    recommendations.push('Consider adding circuit breaker functionality');
  }
  
  if (recommendations.length === 0) {
    recommendations.push('Implementation appears complete and functional');
  }
  
  return {
    status,
    details,
    last_check: new Date().toISOString(),
    recommendations
  };
}

// Run the main function
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch(console.error);
}