/**
 * Temporary Error Identity Tracer
 * 
 * This instrumentation traces PROVIDER_UNFUNDED error identity across all boundaries
 * to identify where error codes/types are being mutated.
 * 
 * This is TEMPORARY instrumentation for debugging only - not for production.
 */

export interface ErrorIdentity {
  constructorName: string;
  code?: string;
  status?: number;
  message: string;
  isLlmError: boolean;
  stackTopFrame?: string;
  timestamp: number;
  location: string;
}

export interface TraceEntry {
  boundary: string;
  errorIdentity: ErrorIdentity;
  entryTime: number;
  mutation?: {
    property: string;
    oldValue: any;
    newValue: any;
  };
}

export class ErrorIdentityTracer {
  private static instance: ErrorIdentityTracer;
  private traces: Map<string, TraceEntry[]> = new Map();
  private traceId: string = '';
  private startTime: number = 0;
  private isRunning: boolean = false;

  private constructor() {}

  static getInstance(): ErrorIdentityTracer {
    if (!ErrorIdentityTracer.instance) {
      ErrorIdentityTracer.instance = new ErrorIdentityTracer();
    }
    return ErrorIdentityTracer.instance;
  }

  startNewTrace(): string {
    this.traceId = `trace_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    this.startTime = Date.now();
    this.traces.set(this.traceId, []);
    this.isRunning = true;
    console.log(`[ErrorIdentityTracer] Starting new trace: ${this.traceId}`);
    return this.traceId;
  }

  stopTrace(): TraceEntry[] {
    this.isRunning = false;
    const trace = this.traces.get(this.traceId) || [];
    const duration = Date.now() - this.startTime;
    console.log(`[ErrorIdentityTracer] Trace ${this.traceId} completed in ${duration}ms with ${trace.length} entries`);
    return trace;
  }

  captureErrorIdentity(error: Error, location: string): ErrorIdentity {
    if (!this.isRunning) {
      return {
        constructorName: 'UnknownError',
        message: 'Tracer not running',
        isLlmError: false,
        timestamp: Date.now(),
        location: 'tracer-not-running'
      };
    }

    const stack = error.stack?.split('\n');
    const stackTopFrame = stack?.[1]?.trim();
    
    // Check if this is an LLM error - we'll need to check specific error types
    let isLlmError = false;
    if (error.constructor.name === 'LlmError' || 
        error.constructor.name === 'ProviderError' ||
        (error as any).type === 'PROVIDER_UNFUNDED') {
      isLlmError = true;
    }

    const errorIdentity: ErrorIdentity = {
      constructorName: error.constructor.name,
      code: (error as any).code || (error as any).status,
      status: (error as any).status,
      message: error.message,
      isLlmError,
      stackTopFrame,
      timestamp: Date.now(),
      location
    };

    console.log(`[ErrorIdentityTracer] Captured error at ${location}:`, errorIdentity);
    return errorIdentity;
  }

  traceBoundary(boundary: string, error: Error, mutation?: { property: string; oldValue: any; newValue: any }): void {
    if (!this.isRunning) return;

    const errorIdentity = this.captureErrorIdentity(error, boundary);
    const traceEntry: TraceEntry = {
      boundary,
      errorIdentity,
      entryTime: Date.now(),
      mutation
    };

    const trace = this.traces.get(this.traceId) || [];
    trace.push(traceEntry);
    this.traces.set(this.traceId, trace);

    console.log(`[ErrorIdentityTracer] Boundary ${boundary}:`, errorIdentity);
    if (mutation) {
      console.log(`[ErrorIdentityTracer] MUTATION DETECTED at ${boundary}:`, mutation);
    }
  }

  getTrace(): TraceEntry[] {
    return this.traces.get(this.traceId) || [];
  }

  analyzeTrace(): {
    identityPreserved: boolean;
    mutations: Array<{ boundary: string; property: string; oldValue: any; newValue: any }>;
    firstMutation?: string;
    lastKnownCode?: string;
    matchesCriteria: Array<string>;
  } {
    const trace = this.getTrace();
    const mutations = [];
    let identityPreserved = true;
    let firstMutation: string | undefined;
    let lastKnownCode: string | undefined;

    for (let i = 0; i < trace.length - 1; i++) {
      const current = trace[i];
      const next = trace[i + 1];
      
      // Check for mutations
      if (current.errorIdentity.constructorName !== next.errorIdentity.constructorName) {
        identityPreserved = false;
        firstMutation = firstMutation || next.boundary;
        mutations.push({
          boundary: next.boundary,
          property: 'constructorName',
          oldValue: current.errorIdentity.constructorName,
          newValue: next.errorIdentity.constructorName
        });
      }

      if (current.errorIdentity.code !== next.errorIdentity.code) {
        identityPreserved = false;
        firstMutation = firstMutation || next.boundary;
        mutations.push({
          boundary: next.boundary,
          property: 'code',
          oldValue: current.errorIdentity.code,
          newValue: next.errorIdentity.code
        });
        lastKnownCode = next.errorIdentity.code;
      }

      if (current.errorIdentity.isLlmError !== next.errorIdentity.isLlmError) {
        identityPreserved = false;
        firstMutation = firstMutation || next.boundary;
        mutations.push({
          boundary: next.boundary,
          property: 'isLlmError',
          oldValue: current.errorIdentity.isLlmError,
          newValue: next.errorIdentity.isLlmError
        });
      }
    }

    // Check specific criteria patterns
    const matchesCriteria: string[] = [];
    
    // A) llm-retry changes PROVIDER_UNFUNDED -> SERVER
    trace.forEach(entry => {
      if (entry.boundary.includes('llm-retry') && entry.errorIdentity.code === 'SERVER' && 
          trace.some(t => t.boundary.includes('transport') && t.errorIdentity.code === 'PROVIDER_UNFUNDED')) {
        matchesCriteria.push('A) llm-retry changes PROVIDER_UNFUNDED -> SERVER');
      }
    });

    // B) agent loop wraps it in generic Error
    trace.forEach(entry => {
      if (entry.boundary.includes('agent-loop') && 
          entry.errorIdentity.constructorName === 'Error' &&
          trace.some(t => t.errorIdentity.constructorName !== 'Error')) {
        matchesCriteria.push('B) agent loop wraps it in generic Error');
      }
    });

    // C) resolver receives PROVIDER_UNFUNDED but does not set skipUnfundedPool
    trace.forEach(entry => {
      if (entry.boundary.includes('resolver') && entry.errorIdentity.code === 'PROVIDER_UNFUNDED') {
        matchesCriteria.push('C) resolver receives PROVIDER_UNFUNDED but does not set skipUnfundedPool');
      }
    });

    // D) resolver sets state, but a new resolver instance ignores it
    trace.forEach(entry => {
      if (entry.boundary.includes('resolver') && entry.errorIdentity.code === 'PROVIDER_UNFUNDED') {
        matchesCriteria.push('D) resolver sets state, but a new resolver instance ignores it');
      }
    });

    // E) retry scheduler runs before resolver pool rotation
    trace.forEach(entry => {
      if (entry.boundary.includes('retry-scheduler') && 
          trace.some(t => t.boundary.includes('resolver') && t.entryTime > entry.entryTime)) {
        matchesCriteria.push('E) retry scheduler runs before resolver pool rotation');
      }
    });

    return {
      identityPreserved,
      mutations,
      firstMutation,
      lastKnownCode,
      matchesCriteria
    };
  }
}

// Utility functions for specific boundary wrapping
export function createTransportWrapper(transportFn: Function): Function {
  const tracer = ErrorIdentityTracer.getInstance();
  
  return async function(...args: any[]) {
    tracer.startNewTrace();
    
    try {
      const result = await transportFn(...args);
      tracer.stopTrace();
      return result;
    } catch (error) {
      tracer.traceBoundary('transport-return/throw', error);
      throw error;
    }
  };
}

export function createLlmRetryWrapper(llmRetryFn: Function): Function {
  const tracer = ErrorIdentityTracer.getInstance();
  
  return async function(...args: any[]) {
    const transportError = args.find(arg => arg instanceof Error);
    if (transportError) {
      tracer.traceBoundary('llm-retry-entry', transportError);
    }

    try {
      const result = await llmRetryFn(...args);
      
      // Check terminal decision
      const shouldRetry = args.find(arg => arg === 'shouldRetry' || typeof arg === 'boolean');
      if (shouldRetry !== undefined) {
        console.log(`[llm-retry] Terminal check: shouldRetry = ${shouldRetry}`);
      }
      
      return result;
    } catch (error) {
      tracer.traceBoundary('llm-retry-terminal-check', error);
      throw error;
    }
  };
}

export function createAgentLoopWrapper(agentFn: Function): Function {
  const tracer = ErrorIdentityTracer.getInstance();
  
  return async function(...args: any[]) {
    try {
      const result = await agentFn(...args);
      return result;
    } catch (error) {
      console.log(`[agent-loop] Original error:`, tracer.captureErrorIdentity(error, 'agent-loop-before-wrap'));
      
      // Simulate agent loop wrapping
      const wrappedError = new Error(`Agent loop error: ${error.message}`);
      Object.assign(wrappedError, {
        code: error.code,
        status: error.status,
        originalError: error
      });
      
      tracer.traceBoundary('agent-loop-catch/wrap', wrappedError, {
        property: 'constructorName',
        oldValue: error.constructor.name,
        newValue: wrappedError.constructor.name
      });
      
      throw wrappedError;
    }
  };
}

export function createResolverWrapper(resolverFn: Function): Function {
  const tracer = ErrorIdentityTracer.getInstance();
  
  return async function(...args: any[]) {
    try {
      const result = await resolverFn(...args);
      return result;
    } catch (error) {
      tracer.traceBoundary('api-resolver-catch', error);
      
      // Check if this should update pool state
      if (error.code === 'PROVIDER_UNFUNDED') {
        console.log(`[resolver] Received PROVIDER_UNFUNDED - updating pool state...`);
        // This is where we'd normally set skipUnfundedPool
      }
      
      throw error;
    }
  };
}

export function createPoolUpdateWrapper(poolUpdateFn: Function): Function {
  const tracer = ErrorIdentityTracer.getInstance();
  
  return function(...args: any[]) {
    const poolState = args[0];
    const error = args[1] || args.find(arg => arg instanceof Error);
    
    if (error && error.code === 'PROVIDER_UNFUNDED') {
      console.log(`[pool-state] Updating pool state for PROVIDER_UNFUNDED`);
      tracer.traceBoundary('pool-state-update', error);
    }
    
    return poolUpdateFn(...args);
  };
}

export function createPoolSelectionWrapper(selectFn: Function): Function {
  const tracer = ErrorIdentityTracer.getInstance();
  
  return function(...args: any[]) {
    const result = selectFn(...args);
    console.log(`[pool-selection] Selected pool:`, result);
    return result;
  };
}

export function createRetrySchedulerWrapper(schedulerFn: Function): Function {
  const tracer = ErrorIdentityTracer.getInstance();
  
  return async function(...args: any[]) {
    const error = args.find(arg => arg instanceof Error);
    
    if (error) {
      tracer.traceBoundary('retry-scheduler-decision', error);
      
      // Check retry decision logic
      const shouldRetry = error.code === 'PROVIDER_UNFUNDED' ? false : true;
      console.log(`[retry-scheduler] Decision for ${error.code}: shouldRetry = ${shouldRetry}`);
    }
    
    return schedulerFn(...args);
  };
}

// TOP-LEVEL declaration - not after an expression's closing brace
export interface ErrorIdentityTracerDefaultExport {
  ErrorIdentityTracer: typeof ErrorIdentityTracer;
  createTransportWrapper: typeof createTransportWrapper;
  createLlmRetryWrapper: typeof createLlmRetryWrapper;
  createAgentLoopWrapper: typeof createAgentLoopWrapper;
  createResolverWrapper: typeof createResolverWrapper;
  createPoolUpdateWrapper: typeof createPoolUpdateWrapper;
  createPoolSelectionWrapper: typeof createPoolSelectionWrapper;
  createRetrySchedulerWrapper: typeof createRetrySchedulerWrapper;
}

const errorIdentityTracerModule: ErrorIdentityTracerDefaultExport = {
  ErrorIdentityTracer,
  createTransportWrapper,
  createLlmRetryWrapper,
  createAgentLoopWrapper,
  createResolverWrapper,
  createPoolUpdateWrapper,
  createPoolSelectionWrapper,
  createRetrySchedulerWrapper,
};

export default errorIdentityTracerModule;