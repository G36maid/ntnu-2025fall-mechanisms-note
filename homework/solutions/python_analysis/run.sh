#!/bin/bash
# 四連桿機構分析 - 快速執行腳本
# Quick run script for Four-Bar Linkage Analysis

set -e

echo "=============================================="
echo "四連桿機構分析 - 圖表生成器"
echo "Four-Bar Linkage Analysis - Figure Generator"
echo "=============================================="
echo ""

# 檢查是否安裝 uv
if ! command -v uv &> /dev/null; then
    echo "❌ Error: uv is not installed"
    echo "Please install uv first:"
    echo "  curl -LsSf https://astral.sh/uv/install.sh | sh"
    exit 1
fi

echo "✓ uv is installed"
echo ""

# 執行程式生成圖表
echo "🚀 Running figure generation..."
echo ""

uv run generate_figures.py

echo ""
echo "=============================================="
echo "✅ 完成！圖表已生成於 ../figures/ 目錄"
echo "✅ Done! Figures saved to ../figures/ directory"
echo "=============================================="
echo ""
echo "生成的圖表 / Generated figures:"
echo "  - figure_a_position_analysis.png  (θ3 & θ4 vs θ2)"
echo "  - figure_b_convergence.png        (Newton-Raphson convergence)"
echo "  - figure_combined_positions.png   (Combined position plot)"
echo ""
