#!/bin/bash
# T27 AUTONOMOUS SOLVER HEARTBEAT
# This script manages the lifecycle of issue resolution without human intervention.

LOG_FILE=".trinity/logs/autonomous-solver.log"
ISSUE_LIST=".trinity/pending_issues.txt"

echo "[$(date)] Starting Autonomous Solver Loop..." >> $LOG_FILE

while true; do
  echo "[$(date)] Scanning for priority tasks..." >> $LOG_FILE
  
  # 1. SYNC STATE
  git fetch origin master
  
  # 2. IDENTIFY TASKS
  # Use gh CLI to find issues with 'P0' or 'P1' labels
  gh issue list --label "P0" --label "P1" --json number,title > $ISSUE_LIST
  
  if [ ! -s $ISSUE_LIST ]; then
    echo "[$(date)] No high-priority issues found. Scanning epics..." >> $LOG_FILE
    # Scan for open epic-related issues
    gh issue list --search "epic" --json number,title >> $ISSUE_LIST
  fi

  # 3. TRIGGER AGENT
  # In the DSH environment, we signal the agent by updating a 'current-task' file
  # which the agent is instructed to monitor.
  if [ -s $ISSUE_LIST ]; then
    TOP_ISSUE=$(head -n 1 $ISSUE_LIST | jq -r '.number')
    echo "Current Target: Issue #$TOP_ISSUE" > .trinity/current-issue.md
    echo "[$(date)] Dispatching Agent to solve #$TOP_ISSUE" >> $LOG_FILE
    
    # We trigger the agent's autonomous loop via a signal file
    touch .trinity/signal/solve_now
  else
    echo "[$(date)] Queue empty. Sleeping for 1 hour..." >> $LOG_FILE
    sleep 3600
    continue
  fi

  # 4. VERIFICATION GATE
  # Wait for the agent to mark the issue as 'landed' or 'verified'
  # We check if the current-issue.md is cleared or updated to the next one.
  while grep -q "Issue #$TOP_ISSUE" .trinity/current-issue.md; do
    echo "[$(date)] Agent is working on #$TOP_ISSUE... waiting..." >> $LOG_FILE
    sleep 300
  done

  echo "[$(date)] Issue #$TOP_ISSUE resolved. Proceeding to next." >> $LOG_FILE
done
