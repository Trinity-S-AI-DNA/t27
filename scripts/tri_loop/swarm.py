#!/usr/bin/env python3
r"""tri swarm -- one screen of the live swarm: lanes, throughput, and which model the bees are on.

WHY THIS EXISTS
---------------
On 2026-09-21 the swarm read `active 20` for eighteen minutes while it finished
six turns. Every bee was asleep in a 30-second backoff: NVIDIA answered 1,369
requests with 429 or 503. `active` is the number of bees that have started and
not finished, and it cannot tell working from waiting. The only reading that
can is finished-per-interval, and the model ranking the server now keeps
(BrowserOS #496) says why: each candidate's success rate over the last fifteen
minutes, its measured speed, and whether it can call a tool.

This prints both. With --since N it samples twice, N seconds apart, and reports
the finished delta as a rate, which is the number to judge a lane change by.

WHAT THIS ESTABLISHES, AND WHAT IT DOES NOT
-------------------------------------------
Established: what /queen/status answered, and, with --since, how many
dispatches finished between two readings.
Not established: that a finished turn produced useful work (read the verdicts),
or why a model's success rate is low beyond the refusal counts the server keeps
(BrowserOS #497): 503 is the model overloaded, 429 is one key's rate limit.

HEALTH MONITORING AND RESTART
------------------------------
This script now includes health monitoring capabilities:
--health-check: Perform health check on swarm service, exit 0 if healthy (HTTP 200)
--monitor: Continuous monitoring with automatic restart capability
--restart: Manually trigger swarm restart via Railway API
--simulate-failure: Test restart logic without actually failing
"""
import argparse
import json
import os
import sys
import time
import urllib.request
import urllib.error
import urllib.parse
from datetime import datetime
from typing import Dict, Optional, Tuple

DEFAULT_URL = "https://trios-agent-server-production.up.railway.app/queen/status"
RAILWAY_API_BASE = "https://api.railway.app/v0"


def read_swarm_status(url: str) -> Dict:
    """Read swarm status from the given URL."""
    try:
        with urllib.request.urlopen(url, timeout=30) as response:
            if response.status == 200:
                return json.load(response)
            else:
                raise urllib.error.HTTPError(url, response.status, "HTTP Error", {}, None)
    except Exception as error:
        raise error


def perform_health_check(url: str) -> Tuple[bool, int, Optional[str], float]:
    """
    Perform health check on swarm service.
    Returns: (healthy, status_code, error, response_time_seconds)
    """
    start_time = time.time()
    try:
        with urllib.request.urlopen(url, timeout=30) as response:
            response_time = time.time() - start_time
            if response.status == 200:
                return True, response.status, None, response_time
            else:
                return False, response.status, f"HTTP {response.status}", response_time
    except Exception as error:
        response_time = time.time() - start_time
        return False, 0, str(error), response_time


def restart_swarm_service() -> Tuple[bool, str, Optional[str]]:
    """
    Restart swarm service via Railway API.
    Returns: (success, message, request_id)
    """
    railway_token = os.environ.get("RAILWAY_TOKEN")
    railway_service_id = os.environ.get("RAILWAY_SERVICE_ID")
    railway_environment_id = os.environ.get("RAILWAY_ENVIRONMENT_ID")
    
    if not all([railway_token, railway_service_id]):
        return False, "Missing Railway credentials (RAILWAY_TOKEN, RAILWAY_SERVICE_ID required)", None
    
    try:
        url = f"{RAILWAY_API_BASE}/services/{railway_service_id}/restart"
        data = json.dumps({}).encode('utf-8')
        
        req = urllib.request.Request(url, data=data, method='POST')
        req.add_header('Authorization', f'Bearer {railway_token}')
        req.add_header('Content-Type', 'application/json')
        
        with urllib.request.urlopen(req, timeout=30) as response:
            if response.status == 200:
                result = json.loads(response.read().decode('utf-8'))
                request_id = result.get("requestId") or result.get("id")
                return True, f"Restart initiated successfully. Request ID: {request_id}", request_id
            else:
                error_text = response.read().decode('utf-8')
                return False, f"Railway API error: {response.status} - {error_text}", None
            
    except Exception as error:
        return False, f"Restart failed: {str(error)}", None


def simulate_failure() -> Tuple[bool, str]:
    """
    Simulate failure to test restart logic.
    Returns: (success, message)
    """
    print("Simulating swarm failure...")
    
    # Check if Railway credentials are available
    railway_token = os.environ.get("RAILWAY_TOKEN")
    railway_service_id = os.environ.get("RAILWAY_SERVICE_ID")
    
    if not railway_token or not railway_service_id:
        return False, "Cannot simulate restart: Railway credentials not available"
    
    # Perform restart
    success, message, request_id = restart_swarm_service()
    
    if success:
        return True, f"Restart simulation successful: {message}"
    else:
        return False, f"Restart simulation failed: {message}"


def monitor_swarm(interval_seconds: int = 60, max_retries: int = 3, enable_restart: bool = True) -> int:
    """
    Monitor swarm service with automatic restart capability.
    Returns: exit code (0=success, 1=error, 2=restart failed)
    """
    consecutive_failures = 0
    
    print(f"Starting swarm monitoring (interval: {interval}s, max_retries: {max_retries})")
    print(f"Health check URL: {DEFAULT_URL}")
    print(f"Automatic restart: {'enabled' if enable_restart else 'disabled'}")
    
    if enable_restart:
        railway_token = os.environ.get("RAILWAY_TOKEN")
        railway_service_id = os.environ.get("RAILWAY_SERVICE_ID")
        railway_environment_id = os.environ.get("RAILWAY_ENVIRONMENT_ID")
        
        if not all([railway_token, railway_service_id]):
            print("WARNING: Railway credentials incomplete - restart capability disabled")
            enable_restart = False
        else:
            print(f"Railway restart enabled for service: {railway_service_id}")
    
    try:
        while True:
            healthy, status_code, error, response_time = perform_health_check(DEFAULT_URL)
            
            if healthy:
                consecutive_failures = 0
                print(f"[{datetime.now().strftime('%Y-%m-%d %H:%M:%S')}] Healthy (HTTP {status_code}, {response_time:.2f}s)")
            else:
                consecutive_failures += 1
                print(f"[{datetime.now().strftime('%Y-%m-%d %H:%M:%S')}] UNHEALTHY (HTTP {status_code}, {response_time:.2f}s) - Failure #{consecutive_failures}")
                
                if error:
                    print(f"  Error: {error}")
                
                # Check if we should restart
                if enable_restart and consecutive_failures >= max_retries:
                    print(f"  Triggering restart after {consecutive_failures} consecutive failures...")
                    success, message, request_id = restart_swarm_service()
                    
                    if success:
                        print(f"  Restart initiated: {message}")
                        consecutive_failures = 0  # Reset counter after restart
                        print("  Waiting 60 seconds for restart to take effect...")
                        time.sleep(60)
                    else:
                        print(f"  Restart failed: {message}")
                        return 2  # Restart failed
            
            # Wait for next check
            time.sleep(interval_seconds)
            
    except KeyboardInterrupt:
        print("\nMonitoring stopped by user")
        return 0
    except Exception as error:
        print(f"Monitoring error: {error}")
        return 1


def show_coordinator_status() -> int:
    """Show coordinator status (placeholder for future implementation)."""
    print("Coordinator status: Not implemented yet")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(prog="tri swarm", description=__doc__.split("\n")[0])
    parser.add_argument("--url", default=os.environ.get("TRI_SWARM_STATUS_URL", DEFAULT_URL))
    parser.add_argument("--since", type=int, default=0, metavar="SECONDS",
                        help="sample twice, SECONDS apart, and report finished per hour")
    parser.add_argument("--json", action="store_true")
    
    # Health monitoring arguments
    parser.add_argument("--health-check", action="store_true",
                        help="perform health check and exit 0 if healthy (HTTP 200)")
    parser.add_argument("--monitor", action="store_true",
                        help="continuous monitoring with automatic restart")
    parser.add_argument("--monitor-interval", type=int, default=60, metavar="SECONDS",
                        help="monitoring interval in seconds (default: 60)")
    parser.add_argument("--max-retries", type=int, default=3, metavar="COUNT",
                        help="max consecutive failures before restart (default: 3)")
    parser.add_argument("--disable-restart", action="store_true",
                        help="disable automatic restart capability")
    parser.add_argument("--restart", action="store_true",
                        help="manually trigger swarm restart")
    parser.add_argument("--simulate-failure", action="store_true",
                        help="simulate failure to test restart logic")
    parser.add_argument("--coordinator", action="store_true",
                        help="show coordinator status")
    
    args = parser.parse_args()

    # Handle different modes
    if args.health_check:
        healthy, status_code, error, response_time = perform_health_check(args.url)
        if healthy:
            print(f"Health check passed: HTTP {status_code} ({response_time:.2f}s)")
            return 0
        else:
            print(f"Health check failed: HTTP {status_code} ({response_time:.2f}s)")
            if error:
                print(f"Error: {error}")
            return 2
    
    elif args.monitor:
        return monitor_swarm(
            interval_seconds=args.monitor_interval,
            max_retries=args.max_retries,
            enable_restart=not args.disable_restart
        )
    
    elif args.restart:
        success, message, request_id = restart_swarm_service()
        print(message)
        return 0 if success else 3
    
    elif args.simulate_failure:
        success, message = simulate_failure()
        print(message)
        return 0 if success else 3
    
    elif args.coordinator:
        return show_coordinator_status()
    
    # Original status mode
    try:
        first = read_swarm_status(args.url)
    except Exception as error:  # the swarm not answering IS the finding
        print(f"tri swarm: status did not answer: {error}", file=sys.stderr)
        return 2

    rate = None
    last = first
    if args.since > 0:
        time.sleep(args.since)
        try:
            last = read_swarm_status(args.url)
        except Exception as error:
            print(f"tri swarm: second reading failed: {error}", file=sys.stderr)
            return 2
        done = (last.get("dispatches", {}).get("finished") or 0) - (
            first.get("dispatches", {}).get("finished") or 0)
        rate = done * 3600 / args.since

    if args.json:
        print(json.dumps({"status": last, "finishedPerHour": rate}, indent=1))
        return 0

    workers = last.get("workers", {})
    dispatches = last.get("dispatches", {})
    print(f"lanes      {workers.get('active')}/{workers.get('capacity')} active "
          f"(active counts sleeping bees too; judge by finished/hour)")
    print(f"dispatches total {dispatches.get('total')}  finished {dispatches.get('finished')}  "
          f"unreviewed {dispatches.get('unreviewed')}  (total counts issues, not work)")
    if dispatches.get("finishedLastHour") is not None:
        print(f"last hour  finished {dispatches.get('finishedLastHour')}  "
              f"dispatched {dispatches.get('dispatchedLastHour')}")
    if rate is not None:
        print(f"throughput {rate:.1f} finished/hour over {args.since}s")
    models = last.get("models")
    if not models:
        print("models     ranking off (TRIOS_QUEEN_WORKER_MODEL_CANDIDATES unset)")
        return 0
    print(f"model      {models.get('chosen')}")
    for m in models.get("ranking", []):
        score = m.get("score")
        cost = f"{score:6.1f}s" if isinstance(score, (int, float)) else "     -"
        flags = " GONE" if m.get("gone") else ""
        print(f"  {cost}  ok {m.get('successRate'):.2f} n={m.get('samples'):<4} "
              f"tps {str(m.get('tokensPerSecond') or '-'):>4} tools {str(m.get('toolCalls')):<5} "
              f"{m.get('model')}{flags}")
        refusals = m.get("refusals") or {}
        if refusals:
            parts = "  ".join(f"{cause}={count}" for cause, count in sorted(refusals.items()))
            print(f"           refused: {parts}")
    print("cost = expected seconds per 500-token step; '-' = not yet measured or no tool call")
    print("refused: 429 = one key over its rate (fewer lanes per key); 503/stream = model overloaded (switch)")
    return 0


if __name__ == "__main__":
    sys.exit(main())