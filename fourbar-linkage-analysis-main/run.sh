#!/bin/bash
# Four-Bar Linkage Analysis Project - Quick Run Script
# Usage: ./run.sh [python|rust|help]

set -e

PROJECT_NAME="Four-Bar Linkage Analysis"
RUST_BIN="four_bar_sim"

show_help() {
    cat << EOF
============================================
$PROJECT_NAME - Run Script
============================================

Usage: ./run.sh [command]

Commands:
  rust          Run the Rust GUI simulator (default)
  python        Run Python figure generation script
  build         Build Rust project in release mode
  dev           Run Rust project in development mode
  test          Run Rust tests
  clean         Clean build artifacts
  help          Show this help message

Examples:
  ./run.sh              # Run Rust GUI simulator
  ./run.sh python       # Generate report figures with Python
  ./run.sh build        # Build optimized Rust binary

Project Structure:
  src/              Rust source code (main simulator)
  pyscript/         Python analysis scripts (for report)
  docs/             Documentation and technical report
  figures/          Generated figures and plots

EOF
}

run_rust() {
    echo "=============================================="
    echo "Running Rust GUI Simulator..."
    echo "=============================================="
    echo ""

    if [ ! -f "Cargo.toml" ]; then
        echo "❌ Error: Cargo.toml not found"
        echo "Are you in the project root directory?"
        exit 1
    fi

    echo "🔨 Building and running (release mode)..."
    cargo run --release
}

run_python() {
    echo "=============================================="
    echo "Running Python Figure Generator..."
    echo "=============================================="
    echo ""

    if ! command -v uv &> /dev/null; then
        echo "❌ Error: uv is not installed"
        echo ""
        echo "Please install uv first:"
        echo "  curl -LsSf https://astral.sh/uv/install.sh | sh"
        echo ""
        exit 1
    fi

    echo "✓ uv is installed"
    echo ""
    echo "🚀 Generating figures for report..."
    echo ""

    uv run pyscript/generate_figures.py

    echo ""
    echo "=============================================="
    echo "✅ Figures generated successfully!"
    echo "=============================================="
    echo ""
    echo "Output location: ./figures/"
    echo "  📊 figure_a_position_analysis.png"
    echo "  📈 figure_b_convergence.png"
    echo "  📉 figure_combined_positions.png"
    echo ""
}

build_rust() {
    echo "=============================================="
    echo "Building Rust Project (Release Mode)..."
    echo "=============================================="
    echo ""

    cargo build --release

    echo ""
    echo "✅ Build complete!"
    echo "Binary location: ./target/release/$RUST_BIN"
}

dev_rust() {
    echo "=============================================="
    echo "Running Rust Project (Development Mode)..."
    echo "=============================================="
    echo ""

    cargo run
}

test_rust() {
    echo "=============================================="
    echo "Running Rust Tests..."
    echo "=============================================="
    echo ""

    cargo test
}

clean_all() {
    echo "=============================================="
    echo "Cleaning Build Artifacts..."
    echo "=============================================="
    echo ""

    if [ -d "target" ]; then
        echo "🗑️  Removing Rust target directory..."
        cargo clean
    fi

    if [ -d ".venv" ]; then
        echo "🗑️  Removing Python virtual environment..."
        rm -rf .venv
    fi

    if [ -d "pyscript/.venv" ]; then
        echo "🗑️  Removing pyscript virtual environment..."
        rm -rf pyscript/.venv
    fi

    echo ""
    echo "✅ Clean complete!"
}

# Main script logic
case "${1:-rust}" in
    rust)
        run_rust
        ;;
    python)
        run_python
        ;;
    build)
        build_rust
        ;;
    dev)
        dev_rust
        ;;
    test)
        test_rust
        ;;
    clean)
        clean_all
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo "❌ Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
