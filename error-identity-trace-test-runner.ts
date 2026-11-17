/**
 * Error Identity Trace Test Runner
 * 
 * This test runner simulates the exact conditions that trigger a PROVIDER_UNFUNDED error
 * and traces its identity through all boundaries to identify mutations.
 * 
 * This is TEMPORARY instrumentation for debugging only - not for production.
 */

import ErrorIdentityTracer, { 
  ErrorIdentityTracer as Tracer,
  createTransportWrapper,
  createLlmRetryWrapper,
  createAgentLoopWrapper,
  createResolverWrapper,
  createPoolUpdateWrapper,
  createPoolSelectionWrapper,
  createRetrySchedulerWrapper
} from './temporary-error-identity-tracer';

// Mock classes to simulate the environment
class ProviderError extends Error {
  constructor(public code: string, public status: number, message: string) {
    super(message);
    this.name = 'ProviderError';
  }
}

class LlmError extends Error {
  constructor(public code: string, public status: number, message: string) {
    super(message);
    this.name = 'LlmError';
  }
}

// Simulated transport function that correctly classifies PROVIDER_UNFUNDED
async function mockTransportWithError(): Promise<any> {
  console.log(`[transport] Simulating HTTP 503 + "deepseek unfunded" response`);
  
  // Simulate the problematic response
  throw new ProviderError('PROVIDER_UNFUNDED', 503, 'HTTP 503: deepseek unfunded');
}

// Mock LLM retry function
async function mockLlmRetry(error: Error): Promise<any> {
  console.log(`[llm-retry] Received error: ${error.message}`);
  console.log(`[llm-retry] Error code: ${(error as any).code}`);
  
  // Simulate retry delay based on Retry-After header
  if ((error as any).code === 'PROVIDER_UNFUNDED') {
    console.log(`[llm-retry] Detected PROVIDER_UNFUNDED - should not retry`);
    throw error; // Don't retry unfunded errors
  }
  
  // Simulate retry delay
  await new Promise(resolve => setTimeout(resolve, 2000));
  throw error;
}

// Mock agent loop
async function mockAgentLoop(): Promise<any> {
  try {
    console.log(`[agent-loop] Starting request...`);
    await mockTransportWithError();
  } catch (error) {
    console.log(`[agent-loop] Caught error: ${error.message}`);
    throw error; // Re-throw for wrapping simulation
  }
}

// Mock resolver
async function mockResolver(): Promise<any> {
  try {
    console.log(`[resolver] Making request through agent loop...`);
    await mockAgentLoop();
  } catch (error) {
    console.log(`[resolver] Received error: ${error.message} with code ${(error as any).code}`);
    
    // Check if this should update pool state
    if ((error as any).code === 'PROVIDER_UNFUNDED') {
      console.log(`[resolver] Setting skipUnfundedPool flag`);
      // This is where the fix would be applied
    }
    
    throw error;
  }
}

// Mock pool state management
function mockPoolUpdate(poolState: any, error?: Error): any {
  console.log(`[pool-state] Updating pool state`);
  if (error && (error as any).code === 'PROVIDER_UNFUNDED') {
    console.log(`[pool-state] Marking provider as unfunded`);
  }
  return { ...poolState, lastUpdate: Date.now() };
}

// Mock pool selection
function mockPoolSelection(poolState: any): any {
  console.log(`[pool-selection] Selecting next provider from pool`);
  return poolState.providers[0]; // Return first provider
}

// Mock retry scheduler
async function mockRetryScheduler(error: Error, nextAttempt: number): Promise<any> {
  console.log(`[retry-scheduler] Decision for error ${(error as any).code}, attempt ${nextAttempt}`);
  
  if ((error as any).code === 'PROVIDER_UNFUNDED') {
    console.log(`[retry-scheduler] PROVIDER_UNFUNDED - no retry`);
    return { shouldRetry: false, delay: 0 };
  }
  
  return { shouldRetry: true, delay: 5000 };
}

// Main trace function
export async function runErrorIdentityTrace(): Promise<{
  rawTrace: any[];
  analysis: any;
  traceId: string;
}> {
  const tracer = ErrorIdentityTracer.getInstance();
  const traceId = tracer.startNewTrace();
  
  console.log(`\n🚀 Starting Error Identity Trace: ${traceId}`);
  console.log(`🎯 Goal: Trace PROVIDER_UNFUNDED through all boundaries`);
  console.log(`⏱️  Will stop after 3 attempts or 15 seconds\n`);

  let attempt = 0;
  const startTime = Date.now();
  const maxDuration = 15000; // 15 seconds
  
  try {
    while (attempt < 3 && Date.now() - startTime < maxDuration) {
      attempt++;
      console.log(`\n=== ATTEMPT ${attempt} ===`);
      
      // Trace boundaries in order
      try {
        // 1. transport return/throw
        const wrappedTransport = createTransportWrapper(mockTransportWithError);
        await wrappedTransport();
        
      } catch (transportError) {
        console.log(`\n📡 [BOUNDARY 1/8] transport return/throw`);
        
        // 2. llm-retry entry
        const wrappedLlmRetry = createLlmRetryWrapper(mockLlmRetry);
        try {
          await wrappedLlmRetry(transportError);
        } catch (llmRetryError) {
          console.log(`\n🔄 [BOUNDARY 2/8] llm-retry entry`);
          console.log(`🔄 [BOUNDARY 3/8] llm-retry terminal-check decision`);
          
          // 4. agent-loop catch/wrap
          const wrappedAgentLoop = createAgentLoopWrapper(mockAgentLoop);
          try {
            await wrappedAgentLoop();
          } catch (agentError) {
            console.log(`\n🤖 [BOUNDARY 4/8] agent-loop catch/wrap`);
            
            // 5. API resolver catch
            const wrappedResolver = createResolverWrapper(mockResolver);
            try {
              await wrappedResolver();
            } catch (resolverError) {
              console.log(`\n🔍 [BOUNDARY 5/8] API resolver catch`);
              
              // 6. pool state update
              const poolState = { providers: ['deepseek', 'openai'], lastUpdate: Date.now() };
              const wrappedPoolUpdate = createPoolUpdateWrapper(mockPoolUpdate);
              const updatedPool = wrappedPoolUpdate(poolState, resolverError);
              console.log(`\n💾 [BOUNDARY 6/8] pool state update`);
              
              // 7. next pool selection
              const wrappedPoolSelection = createPoolSelectionWrapper(mockPoolSelection);
              const selectedPool = wrappedPoolSelection(updatedPool);
              console.log(`\n🎯 [BOUNDARY 7/8] next pool selection`);
              
              // 8. retry scheduler decision
              const wrappedScheduler = createRetrySchedulerWrapper(mockRetryScheduler);
              const decision = await wrappedScheduler(resolverError, attempt);
              console.log(`\n⏰ [BOUNDARY 8/8] retry scheduler decision`);
              
              // Check if we should continue based on retry decision
              if (decision.shouldRetry === false && attempt >= 3) {
                break;
              }
              
              if (decision.shouldRetry) {
                const delay = Math.min(decision.delay, 3000);
                console.log(`⏸️  Waiting ${delay}ms before next attempt...`);
                await new Promise(resolve => setTimeout(resolve, delay));
              }
            }
          }
        }
      }
    }
    
  } catch (error) {
    console.log(`\n❌ Trace stopped due to error: ${error.message}`);
  }
  
  const rawTrace = tracer.stopTrace();
  const analysis = tracer.analyzeTrace();
  
  return {
    rawTrace,
    analysis,
    traceId
  };
}

// Standalone execution
if (import.meta.url === `file://${process.argv[1]}`) {
  runErrorIdentityTrace()
    .then(result => {
      console.log('\n' + '='.repeat(60));
      console.log('🔍 ERROR IDENTITY TRACE RESULTS');
      console.log('='.repeat(60));
      
      console.log('\n📊 ANALYSIS:');
      console.log(`   Identity Preserved: ${result.analysis.identityPreserved ? 'YES' : 'NO'}`);
      console.log(`   Mutations Found: ${result.analysis.mutations.length}`);
      console.log(`   First Mutation: ${result.analysis.firstMutation || 'None'}`);
      console.log(`   Last Known Code: ${result.analysis.lastKnownCode || 'None'}`);
      
      if (result.analysis.matchesCriteria.length > 0) {
        console.log('\n🎯 CRITERIA MATCHED:');
        result.analysis.matchesCriteria.forEach(criterion => {
          console.log(`   ✅ ${criterion}`);
        });
      }
      
      console.log('\n📝 RAW TRACE:');
      result.rawTrace.forEach((entry, index) => {
        console.log(`\n--- ${index + 1}. ${entry.boundary} ---`);
        console.log(`   Constructor: ${entry.errorIdentity.constructorName}`);
        console.log(`   Code: ${entry.errorIdentity.code || 'N/A'}`);
        console.log(`   Status: ${entry.errorIdentity.status || 'N/A'}`);
        console.log(`   Message: ${entry.errorIdentity.message}`);
        console.log(`   Is LlmError: ${entry.errorIdentity.isLlmError}`);
        console.log(`   Time: ${entry.entryTime - result.rawTrace[0].entryTime}ms`);
        
        if (entry.mutation) {
          console.log(`   🔥 MUTATION: ${entry.mutation.property} changed from ${entry.mutation.oldValue} to ${entry.mutation.newValue}`);
        }
      });
      
      console.log('\n' + '='.repeat(60));
      console.log(`🏁 Trace completed in ${result.rawTrace.length} boundary crossings`);
      console.log('='.repeat(60));
      
    })
    .catch(error => {
      console.error('❌ Trace execution failed:', error);
      process.exit(1);
    });
}