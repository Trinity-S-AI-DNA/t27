#!/bin/bash

# Fix Cache Issue for t27 Tools
# This script resolves the permission issues with Zig cache

echo "🔧 Fixing cache issues for t27 tools..."

# Create temporary cache directory with proper permissions
echo "Creating temporary cache directory..."
mkdir -p ~/tmp_zig_cache
chmod -R 755 ~/tmp_zig_cache

# Set environment variable
export ZIG_GLOBAL_CACHE_DIR=~/tmp_zig_cache
echo "✅ Set ZIG_GLOBAL_CACHE_DIR to: $ZIG_GLOBAL_CACHE_DIR"

# Add to shell profile for future sessions
echo "Adding to shell profile..."
if [ -f ~/.zshrc ]; then
    echo 'export ZIG_GLOBAL_CACHE_DIR=~/tmp_zig_cache' >> ~/.zshrc
    echo "✅ Added to ~/.zshrc"
fi

if [ -f ~/.bashrc ]; then
    echo 'export ZIG_GLOBAL_CACHE_DIR=~/tmp_zig_cache' >> ~/.bashrc
    echo "✅ Added to ~/.bashrc"
fi

# Verify t27c is available
if [ -f "./target/release/t27c" ]; then
    echo "✅ t27c compiler found at: ./target/release/t27c"
else
    echo "❌ t27c compiler not found. Please run:"
    echo "   cp ./.claude/worktrees/tri-claw-mcp-integration-63f8f9/target/release/t27c ./target/release/t27c"
fi

# Test basic functionality
echo ""
echo "🧪 Testing basic functionality..."

# Test t27c directly
export ZIG_GLOBAL_CACHE_DIR=~/tmp_zig_cache
echo "Testing t27c path command..."
if ./target/release/t27c path ./specs/demos/hello_world.t27 >/dev/null 2>&1; then
    echo "✅ t27c path command works"
else
    echo "❌ t27c path command failed"
fi

# Test tri health command
echo "Testing tri health command..."
if ./target/release/tri health >/dev/null 2>&1; then
    echo "✅ tri health command works"
else
    echo "❌ tri health command failed"
fi

echo ""
echo "🎉 Cache fix complete!"
echo ""
echo "Next steps:"
echo "1. Run: source ~/.zshrc  (or restart terminal)"
echo "2. Test: ./target/release/tri test ./specs/demos/hello_world.t27"
echo "3. If still blocked, try using the cache workaround:"
echo "   export ZIG_GLOBAL_CACHE_DIR=~/tmp_zig_cache"
echo "   ./target/release/tri test ./specs/demos/hello_world.t27"