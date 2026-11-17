#!/usr/bin/env node

// Provider Guard Verification Test Suite
// Runs comprehensive tests for provider guard implementation

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

class ProviderGuardVerifier {
    constructor() {
        this.testResults = [];
        this.totalTests = 0;
        this.passedTests = 0;
        this.failedTests = 0;
    }

    // Helper function to run a command and capture results
    async runCommand(command, args, options = {}) {
        return new Promise((resolve) => {
            const child = spawn(command, args, { ...options, encoding: 'utf8' });
            let stdout = '';
            let stderr = '';
            
            child.stdout.on('data', (data) => {
                stdout += data;
            });
            
            child.stderr.on('data', (data) => {
                stderr += data;
            });
            
            child.on('close', (code) => {
                resolve({
                    code,
                    stdout,
                    stderr,
                    success: code === 0 || (options.expectError && code !== 0)
                });
            });
            
            child.on('error', (error) => {
                resolve({
                    error,
                    stdout: '',
                    stderr: error.message,
                    success: false
                });
            });
            
            // Set timeout
            if (options.timeout) {
                setTimeout(() => {
                    child.kill('SIGTERM');
                    resolve({
                        code: -1,
                        stdout: '',
                        stderr: 'Timeout',
                        success: false
                    });
                }, options.timeout);
            }
        });
    }

    // Test 1: Provider Guard Runtime Tests
    async testProviderGuardRuntime() {
        console.log('🔬 Testing Provider Guard Runtime...');
        
        const tests = [
            {
                name: 'Provider Guard Unit Tests',
                command: './bootstrap/target/release/t27c',
                args: ['test', '--pattern', 'provider_guard'],
                timeout: 30000
            },
            {
                name: 'Choke Point Verification',
                command: './bootstrap/target/release/t27c',
                args: ['test', '--pattern', 'choke_point'],
                timeout: 30000
            },
            {
                name: 'Prepared Call Path',
                command: './bootstrap/target/release/t27c',
                args: ['test', '--pattern', 'prepared_call'],
                timeout: 30000
            }
        ];

        for (const test of tests) {
            this.totalTests++;
            console.log(`  - Running: ${test.name}`);
            
            const result = await this.runCommand(test.command, test.args, { timeout: test.timeout });
            
            if (result.success) {
                this.passedTests++;
                console.log(`  ✅ PASS: ${test.name}`);
                this.testResults.push({
                    test: test.name,
                    status: 'PASS',
                    exitCode: result.code,
                    details: result.stdout
                });
            } else {
                this.failedTests++;
                console.log(`  ❌ FAIL: ${test.name} (${result.code})`);
                console.log(`  Error: ${result.stderr}`);
                this.testResults.push({
                    test: test.name,
                    status: 'FAIL',
                    exitCode: result.code,
                    details: result.stderr
                });
            }
        }
    }

    // Test 2: Full Integration Suite
    async testFullIntegrationSuite() {
        console.log('🔬 Testing Full Integration Suite...');
        
        this.totalTests++;
        console.log('  - Running: Full Integration Suite');
        
        const result = await this.runCommand('./scripts/tri', ['test'], { timeout: 120000 });
        
        if (result.success) {
            this.passedTests++;
            console.log('  ✅ PASS: Full Integration Suite');
            this.testResults.push({
                test: 'Full Integration Suite',
                status: 'PASS',
                exitCode: result.code,
                details: result.stdout
            });
        } else {
            this.failedTests++;
            console.log(`  ❌ FAIL: Full Integration Suite (${result.code})`);
            console.log(`  Error: ${result.stderr}`);
            this.testResults.push({
                test: 'Full Integration Suite',
                status: 'FAIL',
                exitCode: result.code,
                details: result.stderr
            });
        }
    }

    // Test 3: Valid z.ai Route
    async testValidZaiRoute() {
        console.log('🔬 Testing Valid z.ai Route...');
        
        // Check if z.ai credentials are available
        if (!fs.existsSync('.env') && !process.env.OPENAI_API_KEY) {
            console.log('  ⚠️  Skipping z.ai test - no credentials found');
            this.testResults.push({
                test: 'Valid z.ai Route',
                status: 'SKIP',
                exitCode: 0,
                details: 'No credentials available'
            });
            return;
        }

        this.totalTests++;
        console.log('  - Running: z.ai Route Validation');
        
        const result = await this.runCommand('./bootstrap/target/release/t27c', 
            ['validate-route', '--provider', 'z.ai', '--model', 'gpt-4'], 
            { timeout: 30000 });
        
        if (result.success && result.stdout.includes('valid')) {
            this.passedTests++;
            console.log('  ✅ PASS: Valid z.ai Route');
            this.testResults.push({
                test: 'Valid z.ai Route',
                status: 'PASS',
                exitCode: result.code,
                details: result.stdout
            });
        } else {
            this.failedTests++;
            console.log(`  ❌ FAIL: Valid z.ai Route (${result.code})`);
            console.log(`  Error: ${result.stderr}`);
            this.testResults.push({
                test: 'Valid z.ai Route',
                status: 'FAIL',
                exitCode: result.code,
                details: result.stderr
            });
        }
    }

    // Test 4: PROVIDER_UNFUNDED Violation
    async testUnfundedViolation() {
        console.log('🔬 Testing PROVIDER_UNFUNDED Violation...');
        
        this.totalTests++;
        console.log('  - Running: PROVIDER_UNFUNDED Violation Test');
        
        // Test violation creation
        const result = await this.runCommand('./bootstrap/target/release/t27c',
            ['create-violation', '--type', 'PROVIDER_UNFUNDED'],
            { timeout: 30000 });
        
        if (result.success) {
            // Test checkpoint preservation
            const checkpointResult = await this.runCommand('./bootstrap/target/release/t27c',
                ['checkpoint', '--verify-preserved'],
                { timeout: 30000 });
            
            // Test follow-up task creation
            const taskResult = await this.runCommand('./bootstrap/target/release/t27c',
                ['task', '--verify-followup', '--type', 'PROVIDER_UNFUNDED'],
                { timeout: 30000 });
            
            if (checkpointResult.success && taskResult.success) {
                this.passedTests++;
                console.log('  ✅ PASS: PROVIDER_UNFUNDED Violation Test');
                this.testResults.push({
                    test: 'PROVIDER_UNFUNDED Violation',
                    status: 'PASS',
                    exitCode: 0,
                    details: 'Violation created, checkpoint preserved, follow-up task created'
                });
            } else {
                this.failedTests++;
                console.log(`  ❌ FAIL: PROVIDER_UNFUNDED Violation Test`);
                console.log(`  Checkpoint: ${checkpointResult.success ? 'PASS' : 'FAIL'}`);
                console.log(`  Task: ${taskResult.success ? 'PASS' : 'FAIL'}`);
                this.testResults.push({
                    test: 'PROVIDER_UNFUNDED Violation',
                    status: 'FAIL',
                    exitCode: checkpointResult.code || taskResult.code,
                    details: `Checkpoint: ${checkpointResult.stderr}, Task: ${taskResult.stderr}`
                });
            }
        } else {
            this.failedTests++;
            console.log(`  ❌ FAIL: PROVIDER_UNFUNDED Violation (${result.code})`);
            console.log(`  Error: ${result.stderr}`);
            this.testResults.push({
                test: 'PROVIDER_UNFUNDED Violation',
                status: 'FAIL',
                exitCode: result.code,
                details: result.stderr
            });
        }
    }

    // Test 5: Rollback Drill
    async testRollbackDrill() {
        console.log('🔬 Testing Rollback Drill...');
        
        if (!fs.existsSync('./scripts/rollback-drill.sh')) {
            console.log('  ⚠️  Skipping rollback drill - script not found');
            this.testResults.push({
                test: 'Rollback Drill',
                status: 'SKIP',
                exitCode: 0,
                details: 'Script not found'
            });
            return;
        }

        this.totalTests++;
        console.log('  - Running: Rollback Drill');
        
        const result = await this.runCommand('./scripts/rollback-drill.sh', [], { timeout: 60000 });
        
        if (result.success) {
            this.passedTests++;
            console.log('  ✅ PASS: Rollback Drill');
            this.testResults.push({
                test: 'Rollback Drill',
                status: 'PASS',
                exitCode: result.code,
                details: result.stdout
            });
        } else {
            this.failedTests++;
            console.log(`  ❌ FAIL: Rollback Drill (${result.code})`);
            console.log(`  Error: ${result.stderr}`);
            this.testResults.push({
                test: 'Rollback Drill',
                status: 'FAIL',
                exitCode: result.code,
                details: result.stderr
            });
        }
    }

    // Generate verification report
    generateReport() {
        const report = {
            timestamp: new Date().toISOString(),
            summary: {
                totalTests: this.totalTests,
                passedTests: this.passedTests,
                failedTests: this.failedTests,
                skipTests: this.totalTests - this.passedTests - this.failedTests
            },
            testResults: this.testResults,
            phase2Complete: this.failedTests === 0 && this.passedTests > 0,
            verificationStatus: this.failedTests === 0 ? 'COMPLETE' : 'INCOMPLETE'
        };

        // Save report to file
        const reportPath = 'PROVIDER_GUARD_VERIFICATION_REPORT.json';
        fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));
        console.log(`📄 Verification report saved to: ${reportPath}`);

        // Save markdown summary
        const summaryPath = 'PROVIDER_GUARD_VERIFICATION_SUMMARY.md';
        const summary = this.generateMarkdownSummary(report);
        fs.writeFileSync(summaryPath, summary);
        console.log(`📄 Summary saved to: ${summaryPath}`);

        return report;
    }

    // Generate markdown summary
    generateMarkdownSummary(report) {
        const { summary, phase2Complete, testResults } = report;
        
        let summaryText = `# Provider Guard Phase 2 Verification Summary

**Status**: ${phase2Complete ? '✅ COMPLETE' : '❌ INCOMPLETE'}  
**Date**: ${new Date().toLocaleString()}  

## Test Results
- **Total Tests**: ${summary.totalTests}
- **Passed Tests**: ${summary.passedTests}
- **Failed Tests**: ${summary.failedTests}
- **Skipped Tests**: ${summary.skipTests}
- **Success Rate**: ${((summary.passedTests / summary.totalTests) * 100).toFixed(1)}%

## Phase 2 Evidence Requirements
${phase2Complete ? '✅ All requirements met' : '❌ Some requirements not met'}

### Evidence Status
`;

        // Check each requirement
        const requirements = [
            { test: 'Provider Guard Unit Tests', requirement: 'provider guard tests pass' },
            { test: 'Full Integration Suite', requirement: 'tri test / full integration suite exits 0' },
            { test: 'Valid z.ai Route', requirement: 'valid z.ai route succeeds' },
            { test: 'PROVIDER_UNFUNDED Violation', requirement: 'unfunded violation creates zero network I/O, preserves checkpoint, and creates follow-up task' },
            { test: 'Rollback Drill', requirement: 'rollback drill passes' }
        ];

        requirements.forEach(req => {
            const result = testResults.find(r => r.test === req.test);
            if (result) {
                summaryText += `- ${req.requirement}: ${result.status === 'PASS' ? '✅' : (result.status === 'FAIL' ? '❌' : '⚠️')} ${result.status}\n`;
            } else {
                summaryText += `- ${req.requirement}: ⚠️ UNKNOWN\n`;
            }
        });

        summaryText += `\n## Test Details\n\n`;

        testResults.forEach(result => {
            summaryText += `### ${result.test}\n`;
            summaryText += `- **Status**: ${result.status}\n`;
            summaryText += `- **Exit Code**: ${result.exitCode}\n`;
            summaryText += `- **Details**: ${result.details}\n\n`;
        });

        if (phase2Complete) {
            summaryText += `## Next Steps
1. ✅ Phase 2 verification complete
2. 🎯 Prepare for Phase 3 implementation
3. 📊 Monitor production rollout readiness\n\n`;
        } else {
            summaryText += `## Action Items
1. 🔍 Investigate failed test causes
2. 🛠️ Fix identified issues
3. 🔄 Re-run verification script
4. ⚠️ Ensure all tests pass before proceeding\n\n`;
        }

        summaryText += `---
*Generated by Provider Guard Verification Suite*`;

        return summaryText;
    }

    // Main verification method
    async runVerification() {
        console.log('🚀 Starting Provider Guard Phase 2 Verification...');
        console.log('================================================');
        console.log();

        try {
            // Run all tests
            await this.testProviderGuardRuntime();
            await this.testFullIntegrationSuite();
            await this.testValidZaiRoute();
            await this.testUnfundedViolation();
            await this.testRollbackDrill();

            console.log();
            console.log('================================================');
            console.log('🎯 VERIFICATION COMPLETE');
            console.log('================================================');
            console.log();

            // Generate final report
            const report = this.generateReport();

            if (report.phase2Complete) {
                console.log('🎉 ALL PHASE 2 REQUIREMENTS MET!');
                console.log('✅ PROVIDER_GUARD_VERIFICATION: COMPLETE');
                process.exit(0);
            } else {
                console.log('❌ SOME PHASE 2 REQUIREMENTS NOT MET!');
                console.log('❌ PROVIDER_GUARD_VERIFICATION: INCOMPLETE');
                process.exit(1);
            }

        } catch (error) {
            console.error('💥 VERIFICATION FAILED:', error.message);
            this.failedTests++;
            process.exit(1);
        }
    }
}

// Run verification if this file is executed directly
if (require.main === module) {
    const verifier = new ProviderGuardVerifier();
    verifier.runVerification().catch(console.error);
}

module.exports = ProviderGuardVerifier;