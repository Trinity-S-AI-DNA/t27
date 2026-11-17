# T27 Chimera Examples

## Example 1: Incident Analysis and Repair Pipeline

This chimera composes three existing specs to create an autonomous incident repair workflow.

### Composed Specs:
1. **github::issues::issue_list** - List GitHub issues
2. **github::comments::comment_list** - Retrieve comments for analysis
3. **base::debounce::debouncer_should_exec** - Rate limiting for repairs

### Chimera Specification:

```t27
spec "incident_analysis" {
    input = GitHubIssueState
    output = IncidentReport
    effects = ["read_repository", "read_network"]
}

spec "comment_analysis" {
    input = [Comment]
    output = SentimentAnalysis
    effects = ["read_network"]
    invariants = ["valid_sentiment_score"]
}

spec "repair_execution" {
    input = RepairTask
    output = ImplementationEvidence
    effects = ["create_worktree", "edit_code", "run_tests"]
    requires_approval = ["force_push", "delete_branch"]
    invariants = ["tests_pass", "no_regression"]
}

chimera "autonomous_incident_repair" {
    compose = [
        incident_analysis.analyze,
        comment_analysis.analyze,
        repair_task.execute
    ]
    
    invariants = [
        "no_unapproved_destructive_action",
        "no_silent_provider_fallback",
        "completion_with_evidence"
    ]
    
    effects = ["read_repository", "read_network", "create_worktree", "edit_code", "run_tests"]
    requires_approval = ["force_push", "delete_branch"]
}
```

### Type Compatibility:
- `GitHubIssueState.issue_id` → `github::issues::issue_list(issue_id: u32)`
- `[Comment]` → `comment_list(issue_id: u32)` → `SentimentAnalysis`
- `RepairTask` → `repair_task.execute(task: RepairTask)` → `ImplementationEvidence`

## Example 2: FPGA Acceleration Pipeline

This chimera composes FPGA specs to create an accelerated computing pipeline.

### Composed Specs:
1. **fpga::gf16_accel::gf_mul** - GF(16) multiplication
2. **fpga::memory::dma_transfer** - Memory operations
3. **fpga::apb_bridge::apb_write** - Peripheral communication

### Chimera Specification:

```t27
spec "gf16_computation" {
    input = Gf16Input
    output = Gf16Result
    effects = ["compute_gf16"]
    invariants = ["gf16_field_properties"]
}

spec "memory_transfer" {
    input = TransferConfig
    output = TransferResult
    effects = ["read_memory", "write_memory"]
    invariants = ["memory_safety"]
}

spec "peripheral_control" {
    input = PeripheralCommand
    output = PeripheralResponse
    effects = ["write_peripheral", "read_peripheral"]
    requires_approval = ["system_reset"]
    invariants = ["peripheral_bounds"]
}

chimera "accelerated_vector_processing" {
    compose = [
        gf16_computation.multiply,
        memory_transfer.dma,
        peripheral_control.configure
    ]
    
    invariants = [
        "memory_coherence",
        "computation_accuracy",
        "peripheral_safety"
    ]
    
    effects = ["compute_gf16", "read_memory", "write_memory", "write_peripheral", "read_peripheral"]
    requires_approval = ["system_reset"]
}
```

### Type Compatibility:
- `Gf16Input` → `gf16_computation.multiply(input: Gf16Input)` → `Gf16Result`
- `TransferConfig` → `memory_transfer.dma(config: TransferConfig)` → `TransferResult`
- `PeripheralCommand` → `peripheral_control.configure(cmd: PeripheralCommand)` → `PeripheralResponse`

## Example 3: Compiler Pipeline Optimization

This chimera composes compiler specs to create an optimized compilation workflow.

### Composed Specs:
1. **compiler::parser::parse** - Parse source code
2. **compiler::optimizer::optimize** - Apply optimizations
3. **compiler::codegen::generate** - Generate code

### Chimera Specification:

```t27
spec "source_parsing" {
    input = SourceCode
    output = AST
    effects = ["read_source"]
    invariants = ["syntax_validity"]
}

spec "program_optimization" {
    input = AST
    output = OptimizedAST
    effects = ["transform_ast"]
    invariants = ["optimization_correctness", "no_semantic_change"]
}

spec "code_generation" {
    input = OptimizedAST
    output = CompiledCode
    effects = ["write_output"]
    requires_approval = ["overwrite_existing"]
    invariants = ["type_safety", "memory_safety"]
}

chimera "optimized_compilation_pipeline" {
    compose = [
        source_parsing.parse,
        program_optimization.optimize,
        code_generation.generate
    ]
    
    invariants = [
        "syntax_preservation",
        "semantic_preservation",
        "optimization_correctness",
        "type_safety",
        "memory_safety"
    ]
    
    effects = ["read_source", "transform_ast", "write_output"]
    requires_approval = ["overwrite_existing"]
}
```

### Type Compatibility:
- `SourceCode` → `source_parsing.parse(code: SourceCode)` → `AST`
- `AST` → `program_optimization.optimize(ast: AST)` → `OptimizedAST`
- `OptimizedAST` → `code_generation.generate(ast: OptimizedAST)` → `CompiledCode`

## Key Observations

### 1. Type Safety
- Strong typing across domains enables safe composition
- Output types consistently match input types
- Generic types enable flexible composition patterns

### 2. Effect Management
- Effects are clearly declared and composable
- Approval requirements are explicit and hierarchical
- Effect conflicts can be detected at composition time

### 3. Invariant Preservation
- Invariants from all composed specs are preserved
- Conflicting invariants can be detected and resolved
- Global invariants can be added for chimera-specific requirements

### 4. Approval Gates
- Destructive actions require explicit approval
- Approval requirements are aggregated from composed specs
- Hierarchical approval can be implemented

### 5. Evidence Preservation
- Each step produces traceable output
- Execution is replayable through preserved state
- Evidence requirements are merged and not weakened

## Validation Scenarios

### Successful Compositions:
1. Type-compatible specs with compatible effects
2. Non-conflicting invariants
3. Available required approvals
4. Complete evidence chain

### Failed Compositions:
1. Type mismatches
2. Conflicting invariants
3. Missing required approvals
4. Insufficient evidence requirements
5. Privilege escalation attempts

## Implementation Considerations

1. **Type Checker**: Verify input/output compatibility at composition time
2. **Effect Merger**: Combine and resolve effect declarations
3. **Invariant Resolver**: Detect and resolve conflicting invariants
4. **Approval Aggregator**: Collect and prioritize approval requirements
5. **Evidence Chain**: Maintain traceability across all composed steps