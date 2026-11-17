#!/usr/bin/env python3
"""
Regression Tests for Doctor MVP Output Path Resolution
Prevents nested directory creation and path traversal issues
"""

import pytest
import json
import os
import tempfile
from pathlib import Path
import sys
import shutil

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from doctor_cli import DoctorCLI

class TestDoctorOutputPathResolution:
    """Test output path resolution to prevent nested directory creation"""
    
    def setup_method(self):
        """Setup test environment"""
        self.temp_dir = tempfile.mkdtemp()
        self.original_cwd = os.getcwd()
        
        # Change to temp directory to test CWD-based scenarios
        os.chdir(self.temp_dir)
        
        # Create CLI instance
        self.cli = DoctorCLI()
        
    def teardown_method(self):
        """Cleanup test environment"""
        os.chdir(self.original_cwd)
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_output_path_from_repo_root(self):
        """Test output path resolution when working from repository root"""
        # Test relative path resolution
        relative_path = "incidents/scan-001.json"
        output_path = self.cli._resolve_output_path(relative_path)
        
        # Should resolve to audit-output/doctor-mvp/incidents/scan-001.json
        assert output_path.is_relative_to(self.cli.output_root)
        assert "contrib/doctor-mvp/contrib/doctor-mvp" not in output_path.as_posix()
        assert str(output_path).endswith("incidents/scan-001.json")
    
    def test_output_path_from_different_cwd(self):
        """Test output path resolution from different working directory"""
        # Create nested directory to simulate the problematic scenario
        nested_dir = Path(self.temp_dir) / "contrib/doctor-mvp"
        nested_dir.mkdir(parents=True, exist_ok=True)
        
        # Change to nested directory
        original_cwd = os.getcwd()
        os.chdir(nested_dir)
        
        try:
            # Test that output path still resolves correctly
            relative_path = "incidents/scan-001.json"
            output_path = self.cli._resolve_output_path(relative_path)
            
            # Should NOT create nested contrib/doctor-mvp/contrib/doctor-mvp
            assert "contrib/doctor-mvp/contrib/doctor-mvp" not in output_path.as_posix()
            assert output_path.is_relative_to(self.cli.output_root)
            
        finally:
            os.chdir(original_cwd)
    
    def test_forbidden_nested_path_raises_error(self):
        """Test that paths creating forbidden nested directories raise errors"""
        # Test forbidden pattern
        with pytest.raises(ValueError, match="Forbidden nested output path"):
            self.cli._resolve_output_path("contrib/doctor-mvp/test.json")
        
        with pytest.raises(ValueError, match="Forbidden nested output path"):
            self.cli._resolve_output_path("subdir/contrib/doctor-mvp/test.json")
    
    def test_path_escape_raises_error(self):
        """Test that paths trying to escape the output root raise errors"""
        # Test path traversal attempts
        with pytest.raises(ValueError, match="Output path escapes allowed root"):
            self.cli._resolve_output_path("../../etc/passwd")
        
        with pytest.raises(ValueError, match="Output path escapes allowed root"):
            self.cli._resolve_output_path("../outside-output.json")
    
    def test_output_directory_creation(self):
        """Test that output directories are created correctly"""
        # Test creating nested output directories
        relative_path = "incidents/scan-001/subdir/data.json"
        output_path = self.cli._resolve_output_path(relative_path)
        
        # Should create directories without error
        self.cli._create_output_dirs(output_path)
        
        # Verify directories exist
        assert output_path.parent.exists()
        assert output_path.parent.is_dir()
        
        # Verify it's not creating nested contrib directories
        assert "contrib/doctor-mvp/contrib/doctor-mvp" not in output_path.as_posix()
    
    def test_cli_integration_scan_from_nested_dir(self):
        """Test CLI scan command from nested directory doesn't create nested output"""
        # Create test log file
        log_file = Path(self.temp_dir) / "test-log.json"
        test_log = {
            "timestamp": "2026-10-30T01:11:20Z",
            "level": "ERROR",
            "message": "Test error message"
        }
        with open(log_file, 'w') as f:
            json.dump(test_log, f)
        
        # Create nested working directory
        nested_dir = Path(self.temp_dir) / "contrib/doctor-mvp"
        nested_dir.mkdir(parents=True, exist_ok=True)
        
        original_cwd = os.getcwd()
        os.chdir(nested_dir)
        
        try:
            # Test CLI scan from nested directory
            result = self.cli.scan([str(log_file)], "output")
            
            # Verify output file was created in correct location
            output_file = self.cli._resolve_output_path("output/incidents.json")
            assert output_file.exists()
            
            # Verify no nested directories were created
            assert not (Path(self.temp_dir) / "contrib/doctor-mvp" / "contrib/doctor-mvp").exists()
            
        finally:
            os.chdir(original_cwd)

class TestDoctorOutputPathFromVariousCWDs:
    """Test output path behavior from various working directories"""
    
    @pytest.fixture
    def cli_with_temp_dir(self):
        """Create CLI instance with temporary directory"""
        temp_dir = tempfile.mkdtemp()
        original_cwd = os.getcwd()
        os.chdir(temp_dir)
        
        cli = DoctorCLI()
        
        yield cli
        
        os.chdir(original_cwd)
        shutil.rmtree(temp_dir, ignore_errors=True)
    
    def test_from_repo_root_equivalent(self, cli_with_temp_dir):
        """Test from directory that simulates repository root"""
        cli = cli_with_temp_dir
        
        # Create directory structure to simulate repo
        repo_sim_dir = Path(cli.temp_dir) / "contrib" / "doctor-mvp"
        repo_sim_dir.mkdir(parents=True, exist_ok=True)
        
        # Change to repo simulation directory
        original_cwd = os.getcwd()
        os.chdir(repo_sim_dir)
        
        try:
            relative_path = "reports/scan-001.json"
            output_path = cli._resolve_output_path(relative_path)
            
            # Should resolve correctly without nesting
            assert output_path.is_relative_to(cli.output_root)
            assert "contrib/doctor-mvp/contrib/doctor-mvp" not in output_path.as_posix()
            
        finally:
            os.chdir(original_cwd)
    
    def test_from_arbitrary_cwd(self, cli_with_temp_dir):
        """Test from arbitrary working directory"""
        cli = cli_with_temp_dir
        
        # Test from completely arbitrary directory
        arbitrary_dir = Path(cli.temp_dir) / "some" / "other" / "directory"
        arbitrary_dir.mkdir(parents=True, exist_ok=True)
        
        original_cwd = os.getcwd()
        os.chdir(arbitrary_dir)
        
        try:
            relative_path = "data/results.json"
            output_path = cli._resolve_output_path(relative_path)
            
            # Should still resolve correctly
            assert output_path.is_relative_to(cli.output_root)
            assert output_path.name == "results.json"
            
        finally:
            os.chdir(original_cwd)

if __name__ == "__main__":
    # Run tests directly
    pytest.main([__file__, "-v"])