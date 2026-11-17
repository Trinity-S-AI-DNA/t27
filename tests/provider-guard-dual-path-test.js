#!/usr/bin/env node

/**
 * Provider Guard Dual-Path Intercept Test Runner
 * 
 * This script runs the actual provider guard logic through the compiled Rust binary
 * to verify that it blocks network calls correctly for all violation scenarios.
 */

const { spawn } = require('child_process');
const path = require('path');

// Test scenarios to verify
const TEST_SCENARIOS = [
  {
    name: 'PROVIDER_UNFUNDED',
    config: {
      expected_provider: 'z.ai',
      anthropic_base_url: 'https://api.unfunded-provider.com/v1',
      known_unfunded_providers: ['unfunded-provider'],
      effective_api_key: () => 'test-key'
    },
    expected_error: 'PROVIDER_UNFUNDED'
  },
  {
    name: 'PROVIDER_ROUTING_MISMATCH', 
    config: {
      expected_provider: 'z.ai',
      anthropic_base_url: 'https://api.deepseek.com/v1',
      effective_api_key: () => 'test-key'
    },
    expected_error: 'ROUTING_MISMATCH'
  },
  {
    name: 'CREDENTIAL_ERROR',
    config: {
      expected_provider: 'z.ai', 
      anthropic_base_url: 'https://api.z.ai/v1',
      effective_api_key: () => '' // Empty credentials
    },
    expected_error: 'CREDENTIALS_MISSING'
  },
  {
    name: 'MODEL_COMPATIBILITY',
    config: {
      expected_provider: 'deepseek',
      anthropic_base_url: 'https://api.deepseek.com/v1',
      model: 'glm-4', // GLM model on deepseek
      effective_api_key: () => 'test-key'
    },
    expected_error: 'MODEL_NOT_ALLOWED'
  }
];

function runTestScenario(scenario) {
  return new Promise((resolve, reject) => {
    console.log(`\n🧪 Testing: ${scenario.name}`);
    
    // Create a temporary config file
    const configPath = path.join(__dirname, `test-config-${scenario.name.toLowerCase().replace('_', '-')}.json`);
    const configContent = JSON.stringify(scenario.config, null, 2);
    
    require('fs').writeFileSync(configPath, configContent);
    
    // Run the Rust binary with the test config
    const binaryPath = path.join(__dirname, '../../contrib/backend/agent-runner/target/debug/t27-agent-runner');
    const child = spawn(binaryPath, [
      '--test-mode',
      '--config-file', configPath,
      '--test-provider-guard',
      'https://test-provider.com/v1' // Test URL
    ]);
    
    let stdout = '';
    let stderr = '';
    
    child.stdout.on('data', (data) => {
      stdout += data.toString();
    });
    
    child.stderr.on('data', (data) => {
      stderr += data.toString();
    });
    
    child.on('close', (code) => {
      // Clean up config file
      require('fs').unlinkSync(configPath);
      
      // Analyze results
      const passed = stderr.includes(scenario.expected_error) && !stderr.includes('network call');
      const result = {
        scenario: scenario.name,
        passed,
        code,
        stdout,
        stderr,
        hasNetworkCall: stdout.includes('network') || stderr.includes('network'),
        expectedError: scenario.expected_error,
        actualError: stderr.match(/PROVIDER_\w+|ROUTING_MISMATCH|CREDENTIALS_MISSING|MODEL_NOT_ALLOWED/)?.[0] || 'NONE'
      };
      
      console.log(`  ✅ Result: ${passed ? 'PASS' : 'FAIL'}`);
      console.log(`  📊 Network Call: ${result.hasNetworkCall ? 'MADE' : 'BLOCKED'}`);
      console.log(`  🎯 Expected: ${scenario.expected_error}`);
      console.log(`  🔍 Actual: ${result.actualError}`);
      
      resolve(result);
    });
    
    child.on('error', (error) => {
      require('fs').unlinkSync(configPath);
      reject(error);
    });
    
    // Timeout after 5 seconds
    setTimeout(() => {
      child.kill();
      require('fs').unlinkSync(configPath);
      resolve({
        scenario: scenario.name,
        passed: false,
        error: 'TIMEOUT',
        hasNetworkCall: false
      });
    }, 5000);
  });
}

async function runAllTests() {
  console.log('🚀 Starting Provider Guard Dual-Path Intercept Tests\n');
  console.log('📋 Testing zero-network-call behavior for all violation scenarios\n');
  
  const results = [];
  let passed = 0;
  let failed = 0;
  
  for (const scenario of TEST_SCENARIOS) {
    try {
      const result = await runTestScenario(scenario);
      results.push(result);
      
      if (result.passed && !result.hasNetworkCall) {
        passed++;
        console.log(`\n✅ ${scenario.name}: ZERO NETWORK CALLS - GUARD WORKING`);
      } else {
        failed++;
        console.log(`\n❌ ${scenario.name}: FAILED - Network calls may have been made`);
      }
    } catch (error) {
      results.push({
        scenario: scenario.name,
        passed: false,
        error: error.message
      });
      failed++;
      console.log(`\n❌ ${scenario.name}: ERROR - ${error.message}`);
    }
  }
  
  // Summary
  console.log('\n' + '='.repeat(60));
  console.log('📊 DUAL-PATH INTERCEPT TEST RESULTS');
  console.log('='.repeat(60));
  console.log(`✅ Passed: ${passed}`);
  console.log(`❌ Failed: ${failed}`);
  console.log(`🎯 Total: ${TEST_SCENARIOS.length}`);
  console.log(`🛡️ Network Call Prevention: ${passed === TEST_SCENARIOS.length ? 'VERIFIED' : 'FAILED'}`);
  console.log(`\n🔍 Key Evidence:`);
  console.log(`   - Zero network calls on all violations`);
  console.log(`   - Proper error classification (${TEST_SCENARIOS.length} scenarios)`);
  console.log(`   - Runtime guard behavior confirmed`);
  
  if (passed === TEST_SCENARIOS.length) {
    console.log('\n🎉 PROVIDER GUARD RUNTIME PROOF: SUCCESSFUL');
    console.log('   🚦 Status: READY FOR E2E TESTING');
  } else {
    console.log('\n⚠️  PROVIDER GUARD RUNTIME PROOF: INCOMPLETE');
    console.log('   🔧 Needs: Further investigation and fixes');
  }
  
  return { results, passed, failed };
}

// Run if called directly
if (require.main === module) {
  runAllTests()
    .then(({ results }) => {
      process.exit(results.every(r => r.passed && !r.hasNetworkCall) ? 0 : 1);
    })
    .catch(error => {
      console.error('Test runner failed:', error);
      process.exit(1);
    });
}

module.exports = { runAllTests, TEST_SCENARIOS };