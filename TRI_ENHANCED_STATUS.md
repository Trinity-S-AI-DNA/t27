# Enhanced Status Command for tri CLI

## 🎯 Current Status Command
```bash
./target/release/tri status
```

## 🚀 Enhanced Status Command (Proposed)

```bash
# Enhanced status with detailed information
./target/release/tri status --verbose

# JSON output for machine processing
./target/release/tri status --json

# Specific domain focus
./target/release/tri status --domains compiler,testing

# History tracking
./target/release/tri status --history 5
```

## 📊 Enhanced Status Information

### Current Output:
```json
{
  "ring": 47,
  "verdict": "GREEN",
  "score": 1.0,
  "domains": 17,
  "phase2_completion": "5/5",
  "active_task": "numeric-debt-sprint",
  "timestamp": "2026-04-06T16:51:00Z"
}
```

### Enhanced Output:
```json
{
  "ring": 47,
  "verdict": "GREEN",
  "score": 1.0,
  "domains": {
    "total": 17,
    "healthy": 17,
    "warning": 0,
    "critical": 0
  },
  "phase2_completion": {
    "current": "5/5",
    "progress": 100,
    "remaining": [],
    "completed": [
      "spec-parsing",
      "type-checking",
      "backend-generation",
      "test-execution",
      "verification"
    ]
  },
  "active_task": "numeric-debt-sprint",
  "timestamp": "2026-04-06T16:51:00Z",
  "tools": {
    "tri": {
      "version": "1.0.0",
      "status": "healthy",
      "commands": [
        "status", "skill", "cell", "gen", "test", 
        "verdict", "experience", "doctor", "health",
        "serve", "fpga", "mutate", "reseal"
      ]
    },
    "t27c": {
      "version": "0.5.0",
      "status": "healthy",
      "backends": ["rust", "verilog", "javascript"],
      "compilation_stats": {
        "total_specs": 691,
        "successful": 680,
        "failed": 11,
        "success_rate": 98.4
      }
    },
    "t27b": {
      "version": "0.2.0",
      "status": "available",
      "last_build": "2026-10-20T20:40:00Z"
    }
  },
  "specs": {
    "total": 691,
    "by_domain": {
      "demos": 4,
      "base": 32,
      "compiler": 16,
      "chips": 147,
      "policies": 8,
      "vault": 2
    },
    "test_coverage": {
      "total": 691,
      "with_tests": 620,
      "coverage_percentage": 89.7
    }
  },
  "performance": {
    "avg_compile_time": "1.2s",
    "avg_test_time": "0.8s",
    "cache_hit_rate": "85%",
    "memory_usage": "256MB"
  },
  "issues": {
    "open": 3,
    "critical": 0,
    "warning": 2,
    "info": 1
  }
}
```

## 🔧 Implementation Plan

### 1. Add new status command options
In `cli/tri/src/main.rs`:

```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...
    Status {
        #[arg(long)]
        verbose: bool,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        domains: Vec<String>,
        #[arg(long)]
        history: Option<usize>,
    },
    // ... rest of commands ...
}
```

### 2. Create enhanced status function
```rust
fn enhanced_status(cmd: StatusArgs) -> Result<()> {
    let root = find_trinity_root()?;
    
    let mut status = basic_status()?;
    
    if cmd.verbose || cmd.json {
        // Add detailed information
        status.tools = get_tool_status()?;
        status.specs = get_spec_stats(&root)?;
        status.performance = get_performance_metrics()?;
        status.issues = get_open_issues()?;
    }
    
    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&status)?);
    } else {
        print_status_pretty(&status, cmd.verbose);
    }
    
    Ok(())
}
```

### 3. Helper functions
```rust
fn get_tool_status() -> Result<ToolStatus> {
    Ok(ToolStatus {
        tri: get_tri_version()?,
        t27c: get_t27c_version()?,
        t27b: get_t27b_version()?,
    })
}

fn get_spec_stats(root: &Path) -> Result<SpecStats> {
    // Count specs by directory
    // Calculate test coverage
    // Return statistics
}
```

## 🎨 Output Formatting

### Verbose Text Output:
```
📊 Trinity S³AI Status - Ring 47
=====================================
Verdict: GREEN ✅
Score: 1.0 / 1.0

🔧 Tools Status:
  • tri v1.0.0: Healthy (13 commands)
  • t27c v0.5.0: Healthy (98.4% success)
  • t27b v0.2.0: Available

📁 Specifications: 691 total
  • demos: 4
  • base: 32
  • compiler: 16
  • chips: 147
  • policies: 8
  • vault: 2

🧪 Test Coverage: 89.7% (620/691 specs)

⚡ Performance:
  • Avg compile: 1.2s
  • Cache hit rate: 85%
  • Memory: 256MB

🐛 Issues: 3 open (0 critical, 2 warning, 1 info)
```

## 📈 Usage Examples

### Basic Status:
```bash
./target/release/tri status
# Shows current ring and verdict
```

### Detailed Status:
```bash
./target/release/tri status --verbose
# Shows full status report
```

### Machine Readable:
```bash
./target/release/tri status --json > status.json
# Export to JSON for automation
```

### Domain Specific:
```bash
./target/release/tri status --domains compiler,testing
# Focus on specific domains
```

## 🔍 Integration Benefits

1. **Health Monitoring**: Quick overview of tool health
2. **Progress Tracking**: See compilation and test progress
3. **Performance Insights**: Monitor performance metrics
4. **Issue Management**: Track open issues and their severity
5. **Automation Ready**: JSON output for CI/CD pipelines

## 🚀 Next Steps

1. Implement enhanced status command
2. Add helper functions for detailed stats
3. Create pretty printing functions
4. Add JSON serialization
5. Test with real data
6. Update documentation

This enhancement provides immediate value while maintaining backward compatibility with the existing status command.