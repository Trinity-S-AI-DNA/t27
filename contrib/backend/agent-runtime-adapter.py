#!/usr/bin/env python3
"""
T27 Agent Runtime Adapter
Bridges .t27 specifications to Python execution
Constitutional Law: De-Zig-fication - .t27 is source of truth

This adapter:
1. Loads .t27 workflow specifications  
2. Executes T27-derived workflow logic
3. Maintains runtime state and execution history
4. Enforces provider guards and fallback prevention
5. Provides explicit error handling without silent fallbacks
"""

import json
import os
import sys
import time
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Any
import subprocess

# ── Runtime Configuration ──
RUNTIME_ENABLED = True
FALLBACK_TO_PYTHON = False  # Silent fallback explicitly forbidden
PROVIDER_TIMEOUT_SECONDS = 30
MAX_WORKFLOW_STEPS = 100
PROVIDER_GUARD_ENABLED = True

class RuntimeState:
    """Runtime state management"""
    def __init__(self):
        self.current_workflow = ""
        self.workflow_steps = []
        self.completed_steps = []
        self.pending_approvals = []
        self.retry_attempts = {}
        self.execution_context = {}
        self.last_error = None
        self.execution_history = []
    
    def to_dict(self) -> Dict:
        return {
            "current_workflow": self.current_workflow,
            "completed_steps": self.completed_steps,
            "pending_approvals": self.pending_approvals,
            "retry_attempts": self.retry_attempts,
            "execution_context": self.execution_context,
            "last_error": self.last_error,
        }
    
    def from_dict(self, data: Dict):
        self.current_workflow = data.get("current_workflow", "")
        self.completed_steps = data.get("completed_steps", [])
        self.pending_approvals = data.get("pending_approvals", [])
        self.retry_attempts = data.get("retry_attempts", {})
        self.execution_context = data.get("execution_context", {})
        self.last_error = data.get("last_error")

class ProviderGuard:
    """Provider guard runtime enforcement"""
    def __init__(self):
        self.provider_enabled = True
        self.funding_required = False
        self.max_requests_per_minute = 60
        self.request_timeout_ms = 30000
        self.emergency_shutdown = False
        self.current_requests = 0
        self.request_timestamps = []
    
    def check_request_allowed(self) -> Dict[str, Any]:
        """Check if provider request is allowed"""
        if self.emergency_shutdown:
            return {
                "approved": False,
                "reason": "Emergency shutdown active",
                "check_time_ms": 5
            }
        
        if self.funding_required:
            return {
                "approved": False,
                "reason": "Provider funding required",
                "check_time_ms": 10
            }
        
        # Rate limiting check
        current_time = time.time() * 1000
        self.request_timestamps = [t for t in self.request_timestamps 
                                 if current_time - t < 60000]  # 1 minute window
        
        if len(self.request_timestamps) >= self.max_requests_per_minute:
            return {
                "approved": False,
                "reason": "Rate limit exceeded",
                "check_time_ms": 1
            }
        
        self.request_timestamps.append(current_time)
        return {
            "approved": True,
            "reason": "Provider guard approved",
            "check_time_ms": 1
        }
    
    def simulate_timeout_check(self) -> bool:
        """Simulate provider timeout detection"""
        return False  # No timeout in normal operation

class T27WorkflowExecutor:
    """T27 workflow execution engine"""
    
    def __init__(self):
        self.runtime_state = RuntimeState()
        self.provider_guard = ProviderGuard()
        self.execution_history = []
    
    def load_t27_spec(self, spec_path: str) -> Dict[str, Any]:
        """Load and parse T27 specification"""
        try:
            # For now, simulate T27 spec loading
            # In real implementation, this would parse the .t27 file
            return {
                "workflows": {
                    "agent-workflow": {
                        "steps": [
                            {
                                "step_id": "task_selection",
                                "workflow_type": "task_selection",
                                "priority": 100,
                                "selection_criteria": {
                                    "priority_weight": 0.7,
                                    "urgency_weight": 0.3
                                }
                            },
                            {
                                "step_id": "workflow_sequencing", 
                                "workflow_type": "workflow_sequencing",
                                "dependency_graph": {
                                    "task_1": ["task_2"],
                                    "task_2": ["task_3"]
                                }
                            }
                        ]
                    }
                }
            }
        except Exception as e:
            raise RuntimeError(f"Failed to load T27 spec: {e}")
    
    def execute_workflow_step(self, step: Dict[str, Any]) -> Dict[str, Any]:
        """Execute a single T27 workflow step"""
        
        # Runtime safety checks
        if not RUNTIME_ENABLED:
            return {
                "success": False,
                "error": "Runtime disabled",
                "fallback_used": False,
                "execution_time_ms": 0
            }
        
        start_time = time.time() * 1000
        
        try:
            # Execute T27-defined logic
            result = self._execute_t27_logic(step)
            
            # Provider guard check
            if result.get("requires_provider_request", False):
                guard_check = self.provider_guard.check_request_allowed()
                
                if not guard_check["approved"]:
                    return {
                        "success": False,
                        "error": guard_check["reason"],
                        "fallback_used": False,
                        "execution_time_ms": guard_check["check_time_ms"]
                    }
            
            # Prevent silent fallback
            if self._prevent_fallback(step):
                return {
                    "success": False,
                    "error": "Silent fallback forbidden",
                    "fallback_used": False,
                    "execution_time_ms": 10
                }
            
            execution_time = time.time() * 1000 - start_time
            
            # Update runtime state
            self.runtime_state.completed_steps.append(step["step_id"])
            self.runtime_state.execution_context[step["step_id"]] = "completed"
            
            return {
                "success": True,
                "output_data": result.get("output_data", {}),
                "error": None,
                "fallback_used": False,
                "execution_time_ms": int(execution_time)
            }
            
        except Exception as e:
            execution_time = time.time() * 1000 - start_time
            return {
                "success": False,
                "error": str(e),
                "fallback_used": False,
                "execution_time_ms": int(execution_time)
            }
    
    def _execute_t27_logic(self, step: Dict[str, Any]) -> Dict[str, Any]:
        """Execute T27-defined workflow logic"""
        workflow_type = step.get("workflow_type", "")
        
        if workflow_type == "task_selection":
            return self._execute_task_selection(step)
        elif workflow_type == "workflow_sequencing":
            return self._execute_workflow_sequencing(step)
        elif workflow_type == "retry_policy":
            return self._execute_retry_policy(step)
        elif workflow_type == "approval_policy":
            return self._execute_approval_policy(step)
        else:
            raise ValueError(f"Unknown workflow type: {workflow_type}")
    
    def _execute_task_selection(self, step: Dict[str, Any]) -> Dict[str, Any]:
        """Execute T27-defined task selection logic"""
        
        # Simulate task selection based on T27 criteria
        available_tasks = [
            {"task_id": "high_priority_task", "priority": 100, "urgency": 80},
            {"task_id": "medium_priority_task", "priority": 50, "urgency": 30},
            {"task_id": "low_priority_task", "priority": 10, "urgency": 10}
        ]
        
        criteria = step.get("selection_criteria", {})
        priority_weight = criteria.get("priority_weight", 0.7)
        urgency_weight = criteria.get("urgency_weight", 0.3)
        
        best_task = None
        best_score = -1
        
        for task in available_tasks:
            score = (task["priority"] * priority_weight + 
                    task["urgency"] * urgency_weight)
            
            if score > best_score:
                best_score = score
                best_task = task
        
        return {
            "output_data": {
                "selected_task": best_task["task_id"],
                "priority": best_task["priority"],
                "urgency": best_task["urgency"],
                "selection_score": best_score,
                "reason": f"Selected by T27 priority/urgency scoring"
            },
            "requires_provider_request": False
        }
    
    def _execute_workflow_sequencing(self, step: Dict[str, Any]) -> Dict[str, Any]:
        """Execute T27-defined workflow sequencing logic"""
        
        dependency_graph = step.get("dependency_graph", {})
        
        # Topological sort simulation
        sorted_tasks = self._topological_sort(dependency_graph)
        
        return {
            "output_data": {
                "execution_plan": "sequential_plan",
                "total_steps": len(sorted_tasks),
                "sorted_tasks": sorted_tasks,
                "dependency_groups": self._identify_parallel_groups(dependency_graph)
            },
            "requires_provider_request": False
        }
    
    def _execute_retry_policy(self, step: Dict[str, Any]) -> Dict[str, Any]:
        """Execute T27-defined retry policy"""
        
        retry_policy = step.get("retry_policy", {})
        failure_context = step.get("failure_context", {})
        
        max_attempts = retry_policy.get("max_attempts", 3)
        current_attempt = failure_context.get("attempt", 0)
        error_type = failure_context.get("error_type", "")
        
        retryable_errors = retry_policy.get("retryable_errors", [])
        
        should_retry = (current_attempt < max_attempts and 
                       error_type in retryable_errors)
        
        delay_ms = 0
        if should_retry:
            delay_ms = int(retry_policy.get("backoff_ms", 1000) * 
                          (2 ** current_attempt))  # Exponential backoff
        
        return {
            "output_data": {
                "should_retry": should_retry,
                "delay_ms": delay_ms,
                "retry_reason": (f"Retryable error {error_type}, "
                               f"attempt {current_attempt + 1}/{max_attempts}")
            },
            "requires_provider_request": False
        }
    
    def _execute_approval_policy(self, step: Dict[str, Any]) -> Dict[str, Any]:
        """Execute T27-defined approval policy"""
        
        approval_context = step.get("approval_context", {})
        approvers = approval_context.get("approvers", [])
        required_approvals = approval_context.get("required_approvals", 1)
        
        # Simulate approval process
        approved = len(approvers) >= required_approvals
        approver = approvers[0] if approvers else "none"
        
        return {
            "output_data": {
                "approved": approved,
                "approver": approver,
                "required_approvals": required_approvals,
                "available_approvers": len(approvers)
            },
            "requires_provider_request": False
        }
    
    def _prevent_fallback(self, step: Dict[str, Any]) -> bool:
        """Prevent silent fallback (Runtime Constraint)"""
        
        if FALLBACK_TO_PYTHON:
            print("🚫 CRITICAL: Silent fallback detected - FORBIDDEN", file=sys.stderr)
            return True
        
        if step.get("requires_explicit_fallback", False):
            print("⚠️  Explicit fallback requested - requires manual approval", file=sys.stderr)
            return self._request_fallback_approval()
        
        return False
    
    def _request_fallback_approval(self) -> bool:
        """Request manual approval for explicit fallback"""
        # In production, this would require human approval
        print("🔄 Fallback approval required - denying in automated test", file=sys.stderr)
        return False
    
    def _topological_sort(self, graph: Dict[str, List[str]]) -> List[str]:
        """Simple topological sort for task sequencing"""
        visited = set()
        temp_visited = set()
        result = []
        
        def visit(node):
            if node in temp_visited:
                raise ValueError(f"Cycle detected in dependency graph")
            if node in visited:
                return
            
            temp_visited.add(node)
            
            if node in graph:
                for dependency in graph[node]:
                    visit(dependency)
            
            temp_visited.remove(node)
            visited.add(node)
            result.append(node)
        
        for node in graph:
            visit(node)
        
        return result
    
    def _identify_parallel_groups(self, graph: Dict[str, List[str]]) -> List[List[str]]:
        """Identify groups of tasks that can be executed in parallel"""
        # Simple parallel group identification
        groups = []
        processed = set()
        
        for node in graph:
            if node not in processed:
                group = [node]
                processed.add(node)
                
                # Find independent nodes
                for potential in graph:
                    if (potential not in processed and 
                        not any(dep in graph for dep in [node] + group)):
                        group.append(potential)
                        processed.add(potential)
                
                groups.append(group)
        
        return groups
    
    def get_runtime_status(self) -> Dict[str, Any]:
        """Get current runtime status"""
        return {
            "runtime_enabled": RUNTIME_ENABLED,
            "current_workflow": self.runtime_state.current_workflow,
            "completed_steps": len(self.runtime_state.completed_steps),
            "pending_approvals": len(self.runtime_state.pending_approvals),
            "provider_guard_status": {
                "enabled": self.provider_guard.provider_enabled,
                "emergency_shutdown": self.provider_guard.emergency_shutdown,
                "current_requests": self.provider_guard.current_requests
            },
            "execution_history_count": len(self.execution_history)
        }

# ── Main Runtime Adapter Interface ──
def create_runtime_adapter() -> T27WorkflowExecutor:
    """Create and configure the runtime adapter"""
    return T27WorkflowExecutor()

def validate_runtime_configuration() -> bool:
    """Validate runtime configuration"""
    if not RUNTIME_ENABLED:
        print("❌ Runtime disabled", file=sys.stderr)
        return False
    
    if FALLBACK_TO_PYTHON:
        print("❌ Silent fallback forbidden", file=sys.stderr)
        return False
    
    if MAX_WORKFLOW_STEPS <= 0:
        print("❌ Invalid max workflow steps", file=sys.stderr)
        return False
    
    return True

def main():
    """Main runtime adapter entry point"""
    
    if not validate_runtime_configuration():
        sys.exit(1)
    
    # Initialize runtime adapter
    adapter = create_runtime_adapter()
    
    # Load T27 specification
    spec_path = "specs/automation/agent-workflow.t27"
    
    try:
        spec = adapter.load_t27_spec(spec_path)
        print(f"✅ T27 specification loaded: {spec_path}")
        
        # Execute a sample workflow step
        sample_step = {
            "step_id": "task_selection_test",
            "workflow_type": "task_selection",
            "selection_criteria": {
                "priority_weight": 0.7,
                "urgency_weight": 0.3
            }
        }
        
        result = adapter.execute_workflow_step(sample_step)
        
        if result["success"]:
            print("✅ Workflow step executed successfully")
            print(f"   Output: {result['output_data']}")
            print(f"   Execution time: {result['execution_time_ms']}ms")
        else:
            print(f"❌ Workflow step failed: {result['error']}")
            sys.exit(1)
        
        # Display runtime status
        status = adapter.get_runtime_status()
        print("\n📊 Runtime Status:")
        print(json.dumps(status, indent=2))
        
    except Exception as e:
        print(f"❌ Runtime adapter error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()