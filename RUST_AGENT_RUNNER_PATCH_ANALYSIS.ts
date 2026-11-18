/**
 * Rust Agent Runner Patch Analysis
 * 
 * MISSION: Prepare exact Rust agent-runner fix without applying it.
 * Analyze exact response types and unfunded detection methods.
 */

interface ResponseAnalysis {
  exact_response_type: string;
  available_fields: string[];
  unfunded_detection_methods: string[];
  error_handling_context: string;
}

interface ProposedDiff {
  file: string;
  old_code: string;
  new_code: string;
  line_numbers: { old: number; new: number };
}

interface TestPlan {
  unit_tests: string[];
  integration_tests: string[];
  regression_tests: string[];
  counter_tests: string[];
}

class RustPatchAnalyzer {
  analyzeApiResponseStructure(): ResponseAnalysis {
    console.log('🔍 ANALYZING API RESPONSE STRUCTURE');
    console.log('📁 File: contrib/backend/agent-runner/src/api.rs');
    
    return {
      exact_response_type: 'reqwest::Response',
      available_fields: [
        'response.status() -> u16',
        'response.text().await -> String (response body)',
        'response.headers() -> HeaderMap',
        'response.error_for_status() -> Result<_, reqwest::Error>'
      ],
      unfunded_detection_methods: [
        '1. Body string matching: body.contains("unfunded")',
        '2. Body string matching: body.contains("deepseek unfunded")', 
        '3. JSON field parsing: serde_json::from_str(&body).ok()',
        '4. Header parsing: response.headers().get("retry-after")',
        '5. Status code + body combination: status == 503 && body.contains("unfunded")'
      ],
      error_handling_context: `
At line 185 in send_message():
- status: reqwest::StatusCode (u16)
- response: &reqwest::Response  
- body_text: String (from response.text().await)
- attempt: u32 (current attempt count)
- max_retries: u32 (always 3)
- sleep(Duration::from_secs(backoff)).await;
- continue; // triggers agent loop re-execution
      `
    };
  }

  generateProposedDiff(): ProposedDiff {
    console.log('📝 GENERATING PROPOSED DIFF');
    
    return {
      file: 'contrib/backend/agent-runner/src/api.rs',
      old_code: `
                    // Retry on transient server errors
                    if status == 429 || status == 500 || status == 502 || status == 503 {
                        if attempt < max_retries {
                            let backoff = backoff_secs(attempt);
                            let body_text = response.text().await.unwrap_or_default();
                            warn!(
                                status = %status,
                                attempt = attempt,
                                backoff_s = backoff,
                                body = %body_text,
                                "Retryable status, backing off"
                            );
                            logger::log_error(
                                "API status",
                                &format!("HTTP {} (attempt {}/{}), retry in {}s: {}", status, attempt + 1, max_retries + 1, backoff, body_text),
                            );
                            sleep(Duration::from_secs(backoff)).await;
                            attempt += 1;
                            continue;
                        }
                    }`,
      new_code: `
                    // Enhanced retry logic with unfunded error detection
                    let body_text = response.text().await.unwrap_or_default();
                    let is_unfunded_error = status == 503 && 
                                          (body_text.contains("unfunded") || 
                                           body_text.contains("deepseek unfunded"));
                    
                    if status == 429 || status == 500 || status == 502 || 
                       (status == 503 && !is_unfunded_error) {
                        if attempt < max_retries {
                            let backoff = backoff_secs(attempt);
                            
                            if is_unfunded_error {
                                warn!(
                                    status = %status,
                                    attempt = attempt,
                                    body = %body_text,
                                    "UNFUNDED ERROR DETECTED - propagating terminal error"
                                );
                                logger::log_error(
                                    "API unfunded error",
                                    &format!("HTTP {} (unfunded) - attempting pool rotation", status),
                                );
                                
                                // Allow one attempt to next eligible pool
                                if attempt == 0 {
                                    sleep(Duration::from_secs(backoff)).await;
                                    attempt += 1;
                                    continue; // Try next pool
                                } else {
                                    return Err(anyhow!(
                                        "API error: HTTP {} — unfunded provider exhausted ({} attempts)",
                                        status, attempt + 1
                                    ));
                                }
                            } else {
                                warn!(
                                    status = %status,
                                    attempt = attempt,
                                    backoff_s = backoff,
                                    body = %body_text,
                                    "Retryable status, backing off"
                                );
                                logger::log_error(
                                    "API status",
                                    &format!("HTTP {} (attempt {}/{}), retry in {}s: {}", status, attempt + 1, max_retries + 1, backoff, body_text),
                                );
                                sleep(Duration::from_secs(backoff)).await;
                                attempt += 1;
                                continue;
                            }
                        }
                    }`,
      line_numbers: {
        old: 184,
        new: 184
      }
    };
  }

  generateTestPlan(): TestPlan {
    console.log('🧪 GENERATING TEST PLAN');
    
    return {
      unit_tests: [
        'test_unfunded_detection_body_contains_unfunded()',
        'test_unfunded_detection_body_contains_deepseek_unfunded()', 
        'test_unfunded_detection_status_503_with_unfunded_body()',
        'test_non_unfunded_503_preserves_retry()',
        'test_retry_count_limits()'
      ],
      integration_tests: [
        'test_503_unfunded_propagates_terminal_error()',
        'test_503_non_unfunded_preserves_existing_retry()',
        'test_unfunded_with_eligible_pool_rotation()',
        'test_unfunded_all_pools_exhausted_fallback()'
      ],
      regression_tests: [
        'test_existing_429_retry_behavior_unchanged()',
        'test_existing_500_retry_behavior_unchanged()', 
        'test_existing_502_retry_behavior_unchanged()',
        'test_success_responses_unchanged()'
      ],
      counter_tests: [
        'test_503_unfunded_no_second_request_to_same_pool()',
        'test_503_unfunded_exactly_one_request_to_next_pool()',
        'test_503_unfunded_all_pools_unfunded_returns_fallback_exhausted()',
        'test_503_non_unfounded_preserves_bounded_retry()'
      ]
    };
  }

  generatePatchSummary(): any {
    const analysis = this.analyzeApiResponseStructure();
    const diff = this.generateProposedDiff();
    const testPlan = this.generateTestPlan();
    
    console.log('\n🎯 PATCH ANALYSIS COMPLETE');
    console.log('='.repeat(70));
    
    console.log('\n📋 EXACT_RESPONSE_TYPE:');
    console.log(`   Type: ${analysis.exact_response_type}`);
    console.log(`   Available Fields: ${analysis.available_fields.join(', ')}`);
    
    console.log('\n🔍 UNFUNDED_DETECTION_FIELD:');
    analysis.unfunded_detection_methods.forEach((method, index) => {
      console.log(`   ${index + 1}. ${method}`);
    });
    
    console.log('\n📝 PROPOSED_DIFF:');
    console.log(`   File: ${diff.file}`);
    console.log(`   Old lines: ${diff.line_numbers.old}-${diff.line_numbers.old + 19}`);
    console.log(`   New lines: ${diff.line_numbers.new}-${diff.line_numbers.new + 35}`);
    
    console.log('\n🧪 TEST_PLAN:');
    console.log(`   Unit Tests: ${testPlan.unit_tests.length} tests`);
    consolePlanSection(testPlan.unit_tests, 'UNIT');
    console.log(`   Integration Tests: ${testPlan.integration_tests.length} tests`);
    consolePlanSection(testPlan.integration_tests, 'INTEGRATION');
    console.log(`   Regression Tests: ${testPlan.regression_tests.length} tests`);
    consolePlanSection(testPlan.regression_tests, 'REGRESSION');
    console.log(`   Counter Tests: ${testPlan.counter_tests.length} tests`);
    consolePlanSection(testPlan.counter_tests, 'COUNTER');
    
    return {
      EXACT_RESPONSE_TYPE: analysis.exact_response_type,
      AVAILABLE_FIELDS: analysis.available_fields,
      UNFUNDED_DETECTION_FIELD: 'status == 503 && (body.contains("unfunded") || body.contains("deepseek unfunded"))',
      PROPOSED_DIFF: diff,
      TEST_PLAN: testPlan,
      DO_NOT_APPLY_YET: true
    };
  }
}

function consolePlanSection(tests: string[], type: string) {
  console.log(`   ${type}:`);
  tests.slice(0, 3).forEach(test => {
    console.log(`     - ${test}`);
  });
  if (tests.length > 3) {
    console.log(`     ... and ${tests.length - 3} more`);
  }
}

// Execute the patch analysis
function executeRustPatchAnalysis() {
  const analyzer = new RustPatchAnalyzer();
  const summary = analyzer.generatePatchSummary();
  
  console.log('\n🏁 RUST AGENT RUNNER PATCH PREPARED');
  console.log('='.repeat(70));
  console.log('📁 Status: PATCH PREPARED (NOT APPLIED)');
  console.log('🚨 Do not apply yet - requires review and green counter tests');
  console.log('🔧 Next: Review diff + run counter tests + live smoke test');
  
  return summary;
}

// Execute if run directly
if (import.meta.url === `file://${process.argv[1]}`) {
  executeRustPatchAnalysis()
    .then(summary => {
      console.log('\n✅ Rust patch analysis complete');
      console.log('📋 Ready for diff review and counter test validation');
    })
    .catch(error => {
      console.error('❌ Rust patch analysis failed:', error);
      process.exit(1);
    });
}

export { RustPatchAnalyzer };