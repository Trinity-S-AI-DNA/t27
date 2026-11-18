// T27-First Enforcement Mode Controller
// Deployment Status: BLOCK_NEW mode (not production-ready)
// Controls audit/block-new deployment with baseline suppression

export enum EnforcementMode {
  AUDIT_ONLY = 'AUDIT_ONLY',
  BLOCK_NEW = 'BLOCK_NEW', 
  CRITICAL_MIGRATION = 'CRITICAL_MIGRATION',
  FULL_ENFORCEMENT = 'FULL_ENFORCEMENT'
}

export interface EnforcementConfig {
  mode: EnforcementMode;
  baselineFile: string;
  currentViolations: number;
  businessLogicLanguages: string[];
  criticalAreas: string[];
  migrationPaths: string[];
  ciGate: {
    enabled: boolean;
    violationThreshold: number;
    baselineSuppression: boolean;
  };
  deploymentStatus: string;
  productionReady: boolean;
}

export class T27EnforcementController {
  private config: EnforcementConfig;

  constructor(config: EnforcementConfig) {
    this.config = config;
  }

  // Current mode: BLOCK_NEW (audit/block-new deployment)
  get currentMode(): EnforcementMode {
    return this.config.mode;
  }

  // Check if enforcement should be applied
  shouldEnforce(file: string, language: string): boolean {
    if (this.config.mode === EnforcementMode.AUDIT_ONLY) {
      return false; // Never block in audit mode
    }

    // Block new business logic in specified languages
    const isBusinessLogic = this.config.businessLogicLanguages.includes(language);
    const isNewViolation = this.isNewViolation(file);
    
    return isBusinessLogic && isNewViolation;
  }

  // Check if file is a new violation (not in baseline)
  isNewViolation(file: string): boolean {
    // Load baseline inventory and check if file is excluded
    // This would read the baseline file and check exclusions
    return !this.isBaselineExcluded(file);
  }

  // Check if file is excluded by baseline
  isBaselineExcluded(file: string): boolean {
    const baselineExclusions = [
      'contrib/backend/agent-runner/agent-runner.py',
      'contrib/backend/agent-runner/src/agent.rs',
      'contrib/backend/notebooklm/enrich.py', 
      'contrib/backend/notebooklm/content_registry.yaml',
      'contrib/backend/notebooklm/wrapup.py'
    ];
    
    return baselineExclusions.includes(file);
  }

  // CI Gate validation
  validateCIPipeline(): { valid: boolean; violations: number; newViolations: number } {
    // Simulate CI validation logic
    // In real implementation, this would run the T27 audit tool
    const newViolations = 0; // No new violations in baseline mode
    const totalViolations = this.config.currentViolations;
    
    return {
      valid: newViolations <= this.config.ciGate.violationThreshold,
      violations: totalViolations,
      newViolations: newViolations
    };
  }

  // Mode progression
  async promoteMode(newMode: EnforcementMode): Promise<void> {
    console.log(`Promoting T27 enforcement to ${newMode} mode`);
    
    switch (newMode) {
      case EnforcementMode.AUDIT_ONLY:
        this.config.mode = newMode;
        this.config.deploymentStatus = "AUDIT_ONLY_MODE";
        break;
        
      case EnforcementMode.BLOCK_NEW:
        this.config.mode = newMode;
        this.config.deploymentStatus = "BLOCK_NEW_MODE";
        break;
        
      case EnforcementMode.CRITICAL_MIGRATION:
        this.config.mode = newMode;
        this.config.deploymentStatus = "CRITICAL_MIGRATION_MODE";
        break;
        
      case EnforcementMode.FULL_ENFORCEMENT:
        this.config.mode = newMode;
        this.config.deploymentStatus = "FULL_ENFORCEMENT_MODE";
        this.config.productionReady = true;
        break;
    }
    
    console.log(`T27 enforcement status: ${this.config.deploymentStatus}`);
  }

  // Get migration priority for critical areas
  getMigrationPriority(): { area: string; file: string; priority: 'high' | 'medium' }[] {
    const criticalAreas = [
      { area: 'agent-runner workflow logic', file: 'contrib/backend/agent-runner/agent-runner.py', priority: 'high' as const },
      { area: 'session lifecycle logic', file: 'contrib/backend/notebooklm/wrapup.py', priority: 'high' as const },
      { area: 'content registry policy', file: 'contrib/backend/notebooklm/content_registry.yaml', priority: 'medium' as const },
      { area: 'enrichment policy', file: 'contrib/backend/notebooklm/enrich.py', priority: 'medium' as const }
    ];
    
    return criticalAreas;
  }

  // Get current status
  getStatus(): string {
    return `
T27-First Enforcement Status: ${this.config.deploymentStatus}
Current Mode: ${this.config.currentMode}
Baseline Violations: ${this.config.currentViolations}
Production Ready: ${this.config.productionReady}
Next Phase: ${this.config.productionReady ? 'Complete' : 'CI gate + critical migration'}
    `;
  }
}

// Default configuration for BLOCK_NEW mode
export const defaultConfig: EnforcementConfig = {
  mode: EnforcementMode.BLOCK_NEW,
  baselineFile: 'specs/architecture/misplaced-logic-inventory.t27',
  currentViolations: 198,
  businessLogicLanguages: ['python', 'typescript', 'javascript', 'bash', 'yaml', 'json'],
  criticalAreas: [
    'agent-runner workflow logic in Python',
    'notebooklm enrichment policy in Python', 
    'session lifecycle logic in Python',
    'content registry policy in YAML'
  ],
  migrationPaths: [
    'specs/automation/',
    'specs/policy/', 
    'specs/workflow/',
    'specs/rules/',
    'specs/orchestration/'
  ],
  ciGate: {
    enabled: true,
    violationThreshold: 0,  // Block new violations
    baselineSuppression: true
  },
  deploymentStatus: 'ENFORCEMENT_IMPLEMENTED_AUDIT_MODE',
  productionReady: false
};

// Initialize enforcement controller
export const enforcementController = new T27EnforcementController(defaultConfig);