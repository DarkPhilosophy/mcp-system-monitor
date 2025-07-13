#!/bin/bash

# MCP System Monitor Demo Script
# This script runs the demo visualization and generates charts/screenshots

set -e

echo "🎯 MCP System Monitor Demo Visualization"
echo "========================================"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo is not installed. Please install Rust first."
    exit 1
fi

# Check if we're in the project directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the project root directory."
    exit 1
fi

echo "🔧 Building the project..."
cargo build --release

echo "🚀 Running demo visualization..."
cargo run --example demo_visualization --release

echo ""
echo "📊 Demo completed! Generated files:"
echo "=================================="

if [ -d "demo_output" ]; then
    echo "📁 Output directory: demo_output/"
    echo ""
    echo "📈 Generated charts:"
    ls -la demo_output/*.png 2>/dev/null || echo "   No PNG files found"
    echo ""
    echo "📋 Generated reports:"
    ls -la demo_output/*.txt 2>/dev/null || echo "   No TXT files found"
    echo ""
    
    # Show summary if available
    if [ -f "demo_output/system_summary.txt" ]; then
        echo "📋 Demo Summary:"
        echo "==============="
        cat demo_output/system_summary.txt
    fi
    
    echo ""
    echo "🎉 Demo files are ready!"
    echo "💡 You can now use these files for:"
    echo "   - Screenshots in documentation"
    echo "   - Charts in presentations"
    echo "   - Visual proof of system monitoring capabilities"
    
else
    echo "❌ Error: demo_output directory not found. Demo may have failed."
    exit 1
fi 