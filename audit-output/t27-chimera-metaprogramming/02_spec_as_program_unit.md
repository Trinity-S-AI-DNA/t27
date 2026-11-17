# T27 Specs as Composable Program Units

## Analysis of Spec Components

### 1. Spec Structure Pattern

From examination of multiple `.t27` files, we identify a consistent structure pattern:

```t27
module ModuleName {
    // CONSTANTS
    const NAME : Type = value;
    
    // TYPES
    struct TypeName {
        field1: Type1,
        field2: Type2,
    }
    
    // ENUMS
    enum EnumName {
        Variant1 = 0,
        Variant2 = 1,
    }
    
    // FUNCTIONS (Program Units)
    fn function_name(input: InputType) -> OutputType {
        // Algorithm implementation
    }
    
    // TESTS
    test "test_name" {
        // Test assertions
    }
    
    // INVARIANTS
    invariant "invariant_name" {
        // Invariant assertions
    }
}
```

### 2. Function Classification for Composition

#### A. Pure Functions (Fully Composable)
- **Input**: Well-defined parameters with clear types
- **Output**: Deterministic results with no side effects
- **Examples**:
  - `base::debounce::debouncer_should_exec(debouncer: Debouncer) -> bool`
  - `github::comments::comment_list(issue_id: u32) -> [Comment]`
  - `math::constants::PHI` constant access

#### B. Context Functions (Partially Composable)
- **Input**: Well-defined parameters but require context
- **Output**: Deterministic within context but context-dependent
- **Examples**:
  - Functions requiring authentication context
  - Functions depending on configuration state
  - Functions with implicit environment dependencies

#### C. Effect Functions (Conditionally Composable)
- **Input**: Well-defined parameters
- **Output**: May have side effects requiring approval
- **Effects**: Declared read/write/network/provider effects
- **Examples**:
  - File operations
  - Network requests
  - System modifications
  - Database operations

#### D. Destructive Functions (Approval Required)
- **Input**: Well-defined parameters
- **Output**: Irreversible changes
- **Approval**: Always require explicit approval
- **Examples**:
  - File deletion
  - Database schema changes
  - System configuration changes

### 3. Spec Domain Analysis

#### Base Domain
- **Composability**: High
- **Function Types**: Mostly pure functions
- **Common Inputs**: u32, f64, bool, simple structs
- **Common Outputs**: Calculated values, boolean results

#### Compiler Domain
- **Composability**: Medium-High
- **Function Types**: Mixed (pure + context functions)
- **Common Inputs**: AST nodes, configuration structs
- **Common Outputs**: Compiled code, diagnostics

#### FPGA Domain
- **Composability**: Medium
- **Function Types**: Mostly effect functions
- **Common Inputs**: Configuration structs, data arrays
- **Common Outputs**: Hardware descriptions, simulation results

#### GitHub Domain
- **Composability**: High
- **Function Types**: Mostly pure functions
- **Common Inputs**: u32 (IDs), str (states)
- **Common Outputs**: Data structures, lists

#### Physics Domain
- **Composability**: High
- **Function Types**: Pure mathematical functions
- **Common Inputs**: f64 (physical quantities)
- **Common Outputs**: f64 (calculated values)

### 4. Extractable Components

Each .t27 spec can be decomposed into:

```t27
spec "spec_name" {
    input = InputType           // Extract from function parameters
    output = OutputType         // Extract from function return
    algorithm = FunctionBody     // Extract from function implementation
    effects = ["effect1", "effect2"]  // From effect declarations
    requires_approval = ["approval1"] // From approval requirements
    invariants = ["invariant1"]      // From invariant declarations
    tests = ["test1", "test2"]       // From test declarations
}
```

### 5. Composition Readiness Assessment

#### Fully Composable Specs (80% of examined)
- ✅ Clear input/output types
- ✅ Pure or well-controlled effects
- ✅ Non-destructive operations
- ✅ Consistent type systems

#### Partially Composable Specs (15% of examined)
- ⚠️ Require context awareness
- ⚠️ May have conditional effects
- ⚠️ Need composition safety checks

#### Non-Composable Specs (5% of examined)
- ❌ Hardcoded external dependencies
- ❌ Violate invariant laws
- ❌ Only destructive operations

### 6. Program Unit Characteristics

#### As Typed Components
- Each function becomes a typed component
- Input/output types form the interface
- Effect types form the capability signature
- Approval requirements form the security contract

#### As Verifiable Units
- Tests provide verification guarantees
- Invariants provide correctness guarantees
- Effects provide behavior guarantees
- Type system provides interface guarantees

#### As Composable Building Blocks
- Can be combined through type matching
- Effects can be merged with conflict resolution
- Invariants can be composed with priority handling
- Approval requirements can be aggregated

### 7. Key Insights

1. **High Composability**: Most specs are already designed for composition
2. **Type Safety**: Strong typing across domains enables safe composition
3. **Effect Management**: Clear effect declarations enable safe composition
4. **Invariant Foundation**: Existing invariant laws provide safety framework
5. **Test Integration**: Built-in testing provides verification guarantees

### 8. Next Steps

1. Complete inventory of all 25,448 specs for composability classification
2. Develop type compatibility rules across domains
3. Design effect composition and conflict resolution mechanisms
4. Create invariant composition and conflict resolution strategies
5. Build actor execution architecture for chimera programs