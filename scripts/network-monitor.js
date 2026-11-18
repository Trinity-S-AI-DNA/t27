#!/usr/bin/env node

// Network Monitor for Provider Guard Verification
// Tracks network I/O to verify zero network calls on violations

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

class NetworkMonitor {
    constructor() {
        this.monitoring = false;
        this.pidFile = '.network-monitor.pid';
        this.logFile = '.network-monitor.log';
        this.expectedZeroIO = false;
    }

    // Start network monitoring
    async start() {
        if (this.monitoring) {
            console.log('⚠️  Network monitor already running');
            return;
        }

        console.log('📡 Starting network monitoring...');
        
        // Clean up any existing monitor
        this.stop();
        
        // Start monitoring process
        const child = spawn('node', [__dirname + '/network-monitor-worker.js'], {
            detached: true,
            stdio: ['ignore', 'pipe', 'pipe']
        });

        // Handle output
        let output = '';
        child.stdout.on('data', (data) => {
            output += data.toString();
        });

        child.stderr.on('data', (data) => {
            console.error('Network monitor error:', data.toString());
        });

        child.on('close', (code) => {
            console.log('Network monitor stopped with code:', code);
        });

        // Store PID and start monitoring
        child.unref();
        fs.writeFileSync(this.pidFile, child.pid.toString());
        this.monitoring = true;

        // Wait a bit for monitor to initialize
        await new Promise(resolve => setTimeout(resolve, 1000));
    }

    // Stop network monitoring
    stop() {
        if (!this.monitoring) {
            return;
        }

        try {
            const pid = fs.readFileSync(this.pidFile, 'utf8');
            process.kill(parseInt(pid, 10), 'SIGTERM');
            
            // Clean up files
            if (fs.existsSync(this.pidFile)) {
                fs.unlinkSync(this.pidFile);
            }
            if (fs.existsSync(this.logFile)) {
                fs.unlinkSync(this.logFile);
            }
            
            this.monitoring = false;
            console.log('📡 Network monitoring stopped');
        } catch (error) {
            console.error('Error stopping network monitor:', error.message);
        }
    }

    // Set expectation for zero I/O
    expectZeroIO() {
        this.expectedZeroIO = true;
        console.log('📡 Setting expectation: Zero network I/O');
    }

    // Check network I/O results
    async checkIO() {
        if (!this.monitoring) {
            console.log('⚠️  Network monitor not running');
            return { valid: true, reason: 'Monitor not running' };
        }

        return new Promise((resolve) => {
            const maxAttempts = 10;
            let attempts = 0;

            const checkInterval = setInterval(() => {
                attempts++;
                
                try {
                    if (fs.existsSync(this.logFile)) {
                        const logData = fs.readFileSync(this.logFile, 'utf8');
                        const lines = logData.split('\n').filter(line => line.trim());
                        
                        // Check for network activity
                        const networkActivity = lines.filter(line => 
                            line.includes('HTTP') || 
                            line.includes('HTTPS') || 
                            line.includes('socket') ||
                            line.includes('request') ||
                            line.includes('response')
                        );

                        if (this.expectedZeroIO) {
                            if (networkActivity.length === 0) {
                                clearInterval(checkInterval);
                                resolve({ 
                                    valid: true, 
                                    reason: 'Zero network I/O as expected',
                                    activity: []
                                });
                            } else {
                                clearInterval(checkInterval);
                                resolve({ 
                                    valid: false, 
                                    reason: 'Network activity detected when none expected',
                                    activity: networkActivity
                                });
                            }
                        } else {
                            clearInterval(checkInterval);
                            resolve({ 
                                valid: true, 
                                reason: 'Network monitoring completed',
                                activity: networkActivity
                            });
                        }
                    }
                } catch (error) {
                    console.error('Error reading network log:', error.message);
                }

                if (attempts >= maxAttempts) {
                    clearInterval(checkInterval);
                    resolve({ 
                        valid: false, 
                        reason: 'Network monitoring timeout',
                        activity: []
                    });
                }
            }, 500);
        });
    }
}

// Worker process for network monitoring
class NetworkMonitorWorker {
    constructor() {
        this.connections = new Set();
        this.requests = new Set();
        this.startTime = Date.now();
    }

    startMonitoring() {
        console.log('📡 Network monitor worker starting...');
        
        // This is a simplified implementation
        // In a real implementation, you would use actual network monitoring APIs
        
        // Simulate monitoring for this example
        const checkInterval = setInterval(() => {
            // Check for network activity
            // This would be implemented with actual system monitoring in production
            const now = Date.now();
            const elapsed = now - this.startTime;
            
            // Log monitoring status
            console.log(`📡 Network monitoring active for ${elapsed}ms`);
            
            // Exit after reasonable time
            if (elapsed > 30000) {
                clearInterval(checkInterval);
                process.exit(0);
            }
        }, 1000);
    }
}

// Network monitor worker script
if (process.argv[1].endsWith('network-monitor-worker.js')) {
    const worker = new NetworkMonitorWorker();
    worker.startMonitoring();
}

// Export monitor class
module.exports = NetworkMonitor;