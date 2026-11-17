#!/usr/bin/env python3
"""
Real Chat Export Integration
Handles DSH session exports, real-time logs, and live incident detection
"""

import json
import re
import os
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Any
import subprocess

class RealChatExportIntegrator:
    """Integrate with DSH chat exports and real-time logs"""
    
    def __init__(self):
        self.supported_formats = [
            'dsh-session-export',
            'chat-session-json', 
            'deepseek-chat-export',
            'session-log-json'
        ]
        
        # Enhanced patterns for real chat formats
        self.chat_patterns = {
            'DSH_SESSION': [
                r'dsh_session_\d{8}T\d{6}\.json',
                r'session_\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}\.json',
                r'chat_export_\d{8}\.json'
            ],
            'MESSAGE_ROLES': [
                r'"role":\s*"user"',
                r'"role":\s*"assistant"', 
                r'"role":\s*"system"',
                r'"role":\s*"tool"'
            ],
            'COMMAND_EXECUTION': [
                r'Command executed: .+',
                r'Exit code: (\d+)',
                r'Output: .+',
                r'Turn \d+: .+'
            ],
            'ERROR_PATTERNS': [
                r'\[ERROR\].+',
                r'failed:.+',
                r'status \d{3}:.+' ,
                r'timeout:.+',
                r'retry:.+'
            ]
        }
    
    def discover_chat_exports(self, search_path: str = ".") -> List[Dict]:
        """Discover real chat export files in the workspace"""
        exports = []
        
        for root, dirs, files in os.walk(search_path):
            # Skip hidden directories and common temp folders
            dirs[:] = [d for d in dirs if not d.startswith('.') and d not in ['temp', 'tmp', '__pycache__']]
            
            for file in files:
                file_path = Path(root) / file
                
                # Check if file matches chat export patterns
                if self._is_chat_export_file(file_path, file):
                    export_info = {
                        'path': str(file_path),
                        'filename': file,
                        'format': self._identify_format(file),
                        'size': file_path.stat().st_size,
                        'modified': datetime.fromtimestamp(file_path.stat().st_mtime),
                        'session_id': self._extract_session_id(file),
                        'estimated_incidents': self._estimate_incidents(file_path)
                    }
                    exports.append(export_info)
        
        return sorted(exports, key=lambda x: x['modified'], reverse=True)
    
    def _is_chat_export_file(self, file_path: Path, filename: str) -> bool:
        """Check if file is a chat export"""
        # Check file extension
        if file_path.suffix.lower() not in ['.json', '.txt', '.log']:
            return False
        
        # Check filename patterns
        filename_lower = filename.lower()
        
        # Look for chat export indicators
        patterns = [
            'chat', 'session', 'export', 'dsh', 'turn',
            'message', 'conversation', 'dialog'
        ]
        
        # Check if any pattern is in filename and file has reasonable size
        has_pattern = any(pattern in filename_lower for pattern in patterns)
        reasonable_size = 1024 < file_path.stat().st_size < 10 * 1024 * 1024  # 1KB to 10MB
        
        return has_pattern and reasonable_size
    
    def _identify_format(self, filename: str) -> str:
        """Identify the specific chat export format"""
        filename_lower = filename.lower()
        
        for format_name, patterns in self.chat_patterns.items():
            if format_name == 'DSH_SESSION':
                for pattern in patterns:
                    if re.search(pattern, filename):
                        return 'dsh_session'
            elif any(pattern in filename_lower for pattern in ['dsh', 'deepseek']):
                return 'dsh_chat'
            elif 'session' in filename_lower:
                return 'session_json'
            elif 'chat' in filename_lower:
                return 'chat_json'
        
        return 'unknown'
    
    def _extract_session_id(self, filename: str) -> Optional[str]:
        """Extract session ID from filename"""
        # Look for patterns like session_2024-01-15_10-30-00, dsh_session_20240115T103000
        patterns = [
            r'session_(\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2})',
            r'dsh_session_(\d{8}T\d{6})',
            r'(\d{8}T\d{6})',
            r'session_(\d+)'
        ]
        
        for pattern in patterns:
            match = re.search(pattern, filename)
            if match:
                return match.group(1)
        
        return None
    
    def _estimate_incidents(self, file_path: Path) -> int:
        """Estimate number of incidents in file based on error patterns"""
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                
            error_count = 0
            for pattern in self.chat_patterns['ERROR_PATTERNS']:
                matches = re.findall(pattern, content, re.IGNORECASE)
                error_count += len(matches)
            
            # Also check for command execution patterns
            cmd_patterns = self.chat_patterns['COMMAND_EXECUTION']
            for pattern in cmd_patterns:
                matches = re.findall(pattern, content)
                error_count += len(matches) * 0.5  # Weight commands less than errors
            
            return max(1, int(error_count))
            
        except Exception:
            return 1  # Default to 1 if can't read
    
    def load_chat_export(self, file_path: str) -> Dict:
        """Load and parse a chat export file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            # Validate and normalize the chat export structure
            normalized = self._normalize_chat_structure(data)
            
            return {
                'file_path': file_path,
                'session_id': normalized.get('session_id', 'unknown'),
                'timestamp': normalized.get('timestamp', datetime.now().isoformat()),
                'messages': normalized.get('messages', []),
                'metadata': normalized.get('metadata', {}),
                'incident_count': self._count_actual_incidents(normalized)
            }
            
        except json.JSONDecodeError:
            # Try as text format
            return self._parse_text_chat_export(file_path)
        except Exception as e:
            raise ValueError(f"Failed to load chat export {file_path}: {e}")
    
    def _normalize_chat_structure(self, data: Dict) -> Dict:
        """Normalize different chat export formats to standard structure"""
        normalized = {}
        
        # Handle different possible structures
        if 'messages' in data:
            normalized['messages'] = data['messages']
        elif 'conversation' in data:
            normalized['messages'] = data['conversation']
        elif 'chat' in data:
            normalized['messages'] = data['chat']
        elif isinstance(data, list):
            normalized['messages'] = data
        
        # Extract metadata
        normalized['metadata'] = {}
        for key in ['session_id', 'timestamp', 'workspace', 'repository', 'branch']:
            if key in data:
                normalized['metadata'][key] = data[key]
        
        return normalized
    
    def _parse_text_chat_export(self, file_path: str) -> Dict:
        """Parse text-based chat exports"""
        messages = []
        timestamp = datetime.now().isoformat()
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            
            current_message = {}
            message_id = 1
            
            for line in lines:
                line = line.strip()
                if not line:
                    continue
                
                # Look for message patterns
                if re.search(r'\[USER\]|\[USER:\d+\]', line):
                    if current_message:
                        messages.append(current_message)
                    current_message = {
                        'id': f"msg_{message_id}",
                        'role': 'user',
                        'content': line,
                        'timestamp': datetime.now().isoformat()
                    }
                    message_id += 1
                elif re.search(r'\[ASSISTANT\]|\[ASSISTANT:\d+\]', line):
                    if current_message:
                        messages.append(current_message)
                    current_message = {
                        'id': f"msg_{message_id}",
                        'role': 'assistant', 
                        'content': line,
                        'timestamp': datetime.now().isoformat()
                    }
                    message_id += 1
                elif current_message:
                    current_message['content'] += '\n' + line
            
            if current_message:
                messages.append(current_message)
                
        except Exception as e:
            # Fallback to simple text parsing
            with open(file_path, 'r') as f:
                content = f.read()
            
            return {
                'file_path': file_path,
                'session_id': 'unknown',
                'timestamp': timestamp,
                'messages': [{
                    'id': 'msg_1',
                    'role': 'system',
                    'content': content,
                    'timestamp': timestamp
                }],
                'metadata': {},
                'incident_count': 0
            }
        
        return {
            'file_path': file_path,
            'session_id': 'unknown',
            'timestamp': timestamp,
            'messages': messages,
            'metadata': {},
            'incident_count': len([m for m in messages if any(error in m.get('content', '').lower() 
                                   for error in ['error', 'fail', 'exception'])])
        }
    
    def _count_actual_incidents(self, normalized_data: Dict) -> int:
        """Count actual incidents from normalized chat data"""
        incident_count = 0
        messages = normalized_data.get('messages', [])
        
        for message in messages:
            content = message.get('content', '').lower()
            if any(error_indicator in content for error_indicator in 
                   ['error', 'fail', 'exception', 'timeout', 'reject', '503']):
                incident_count += 1
        
        return incident_count
    
    def detect_real_incidents(self, chat_export: Dict) -> List[Dict]:
        """Detect incidents from real chat export data"""
        incidents = []
        messages = chat_export.get('messages', [])
        
        for i, message in enumerate(messages):
            if self._contains_incident(message):
                incident = self._create_incident_from_message(message, chat_export, i)
                incidents.append(incident)
        
        return incidents
    
    def _contains_incident(self, message: Dict) -> bool:
        """Check if message contains an incident"""
        content = message.get('content', '').lower()
        
        incident_indicators = [
            'error', 'fail', 'exception', 'timeout', 'reject',
            '503', '403', '404', '500', 'connection', 'build',
            'test', 'deploy', 'push', 'merge', 'conflict'
        ]
        
        return any(indicator in content for indicator in incident_indicators)
    
    def _create_incident_from_message(self, message: Dict, chat_export: Dict, message_index: int) -> Dict:
        """Create incident from chat message"""
        content = message.get('content', '')
        
        # Extract incident type
        incident_type = self._classify_incident_from_content(content)
        
        # Extract severity
        severity = self._extract_severity(content)
        
        # Create incident ID
        session_id = chat_export.get('session_id', 'unknown')
        incident_id = f"INC-{session_id}-{message_index:04d}"
        
        incident = {
            'id': incident_id,
            'source': 'dsh-chat-export',
            'workspace': chat_export.get('metadata', {}).get('workspace', ''),
            'repository': chat_export.get('metadata', {}).get('repository', ''),
            'branch': chat_export.get('metadata', {}).get('branch', ''),
            'symptom': self._extract_symptom(content),
            'error_class': incident_type,
            'severity': severity,
            'blocked_by': [],
            'evidence': [self._extract_evidence(content)],
            'suspected_root_cause': 'pending_analysis',
            'repair_chat': None,
            'status': 'OPEN',
            'message_metadata': {
                'message_id': message.get('id'),
                'role': message.get('role'),
                'timestamp': message.get('timestamp'),
                'message_index': message_index
            }
        }
        
        return incident
    
    def _classify_incident_from_content(self, content: str) -> str:
        """Classify incident type from content"""
        content_lower = content.lower()
        
        classification_patterns = {
            'BUILD_FAILURE': ['build', 'compile', 'make', 'cargo', 'npm run', 'yarn'],
            'TEST_FAILURE': ['test', 'assert', 'vitest', 'jest', 'pytest', 'unittest'],
            'DISK_PRESSURE': ['disk', 'space', 'storage', 'memory', 'quota'],
            'PROVIDER_UNFUNDED': ['503', 'payment', 'quota', 'funding', 'service unavailable'],
            'WRONG_WORKSPACE': ['workspace', 'directory', 'folder', 'path'],
            'GIT_PUSH_BLOCKED': ['push', 'merge', 'git', 'commit', 'branch'],
            'PROTECTED_STATE_RISK': ['protected', 'config', 'database', 'secret'],
            'UNKNOWN': []
        }
        
        for incident_type, patterns in classification_patterns.items():
            if any(pattern in content_lower for pattern in patterns):
                return incident_type
        
        return 'UNKNOWN'
    
    def _extract_severity(self, content: str) -> str:
        """Extract severity from content"""
        content_lower = content.lower()
        
        if any(keyword in content_lower for keyword in ['critical', 'fatal', 'severe', '503']):
            return 'high'
        elif any(keyword in content_lower for keyword in ['warning', 'warn', 'timeout', 'retry']):
            return 'medium'
        else:
            return 'low'
    
    def _extract_symptom(self, content: str) -> str:
        """Extract symptom description from content"""
        # Extract first meaningful error/failure line
        lines = content.split('\n')
        for line in lines:
            if any(keyword in line.lower() for keyword in ['error', 'fail', 'exception']):
                return line.strip()[:100]  # Limit to 100 chars
        
        return content[:100]  # Fallback to first 100 chars
    
    def _extract_evidence(self, content: str) -> str:
        """Extract evidence from content"""
        # Look for command outputs, error messages, stack traces
        lines = content.split('\n')
        evidence_lines = []
        
        for line in lines:
            if (any(keyword in line.lower() for keyword in ['command', 'exit code', 'output', 'error']) or
                'Error:' in line or 'Exception:' in line):
                evidence_lines.append(line.strip())
        
        if evidence_lines:
            return '\n'.join(evidence_lines[:3])  # Top 3 evidence lines
        
        return content[:200]  # Fallback to first 200 chars

def main():
    """Test the real chat export integration"""
    integrator = RealChatExportIntegrator()
    
    print("🔍 Real Chat Export Integration Test")
    print("=" * 40)
    
    # Discover chat exports
    exports = integrator.discover_chat_exports(".")
    print(f"Found {len(exports)} chat exports")
    
    for export in exports[:3]:  # Show top 3
        print(f"\n📁 {export['filename']}")
        print(f"   Format: {export['format']}")
        print(f"   Size: {export['size']} bytes")
        print(f"   Estimated incidents: {export['estimated_incidents']}")
        print(f"   Modified: {export['modified']}")
    
    if exports:
        # Test loading the first export
        first_export = exports[0]
        print(f"\n📖 Loading {first_export['filename']}...")
        
        try:
            chat_data = integrator.load_chat_export(first_export['path'])
            incidents = integrator.detect_real_incidents(chat_data)
            
            print(f"✅ Loaded successfully")
            print(f"   Session ID: {chat_data['session_id']}")
            print(f"   Messages: {len(chat_data['messages'])}")
            print(f"   Detected incidents: {len(incidents)}")
            
            if incidents:
                print(f"\n🚨 Sample incident:")
                sample = incidents[0]
                print(f"   ID: {sample['id']}")
                print(f"   Type: {sample['error_class']}")
                print(f"   Severity: {sample['severity']}")
                print(f"   Symptom: {sample['symptom']}")
                
        except Exception as e:
            print(f"❌ Error loading: {e}")

if __name__ == "__main__":
    main()