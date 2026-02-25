# Python Analysis Scripts

This directory contains Python scripts for generating static figures and plots used in the technical report.

## Purpose

These scripts are used for:
- **Report Generation**: Create publication-quality figures for the technical report
- **Numerical Validation**: Verify Newton-Raphson convergence characteristics
- **Position Analysis**: Generate θ₃ and θ₄ vs θ₂ plots

**Note**: The Python scripts are for report/paper figure generation only. The main interactive simulator is implemented in Rust (see `../src/`).

## Structure

```
pyscript/
├── README.md              # This file
└── generate_figures.py    # Main figure generation script
```

## Requirements

- Python 3.13+
- [uv](https://github.com/astral-sh/uv) - Fast Python package manager

## Quick Start

From the project root directory:

```bash
# Run the figure generator
./run.sh python

# Or manually with uv
uv run pyscript/generate_figures.py
```

## Generated Figures

All figures are saved to `../figures/`:

1. **figure_a_position_analysis.png**
   - Two-panel plot showing θ₃ and θ₄ vs θ₂
   - Demonstrates complete rotation cycle (0° - 360°)
   - Used in Section 4 of the report

2. **figure_b_convergence.png**
   - Newton-Raphson convergence analysis at θ₂ = 45°
   - Log-scale plot of error vs iterations
   - Demonstrates quadratic convergence
   - Used in Section 5 of the report

3. **figure_combined_positions.png**
   - Combined plot of θ₃ and θ₄ on same axes
   - Shows relative motion characteristics
   - Optional figure for comparison

## Script Details

### `generate_figures.py`

**Functions:**

- `newton_raphson_4bar()`: Newton-Raphson solver implementation
- `analytical_solution()`: Closed-form solution for initial guess
- `generate_figure_a()`: Position analysis plots
- `generate_figure_b()`: Convergence analysis plot
- `generate_combined_figure()`: Combined position plot

**Default Parameters:**
```python
r1 = 6.0  # Ground link
r2 = 2.0  # Input crank
r3 = 5.0  # Coupler link
r4 = 5.0  # Output rocker
```

**Modifying Parameters:**

Edit the values in `generate_figures.py`:
```python
# Line ~152
r1, r2, r3, r4 = 6.0, 2.0, 5.0, 5.0  # Change here
```

## Dependencies

Managed automatically by `uv` via `../pyproject.toml`:

- `numpy`: Numerical computations
- `matplotlib`: Plotting and visualization

## Output Specifications

- **Format**: PNG
- **DPI**: 300 (publication quality)
- **Labels**: English (to avoid font issues)
- **Colors**:
  - Blue: θ₃ (Link 3 / Coupler)
  - Red: θ₄ (Link 4 / Rocker)
  - Green: Convergence curve

## Troubleshooting

### Font Warnings

If you see font warnings, they're harmless. The script uses fallback fonts (DejaVu Sans, Arial).

### Virtual Environment

The `uv` tool automatically manages virtual environments. No manual setup needed.

### Regenerating Figures

To regenerate all figures:
```bash
cd ..
./run.sh python
```

Or delete specific figures and run again:
```bash
rm ../figures/figure_a_*.png
uv run pyscript/generate_figures.py
```

## Integration with Main Project

1. **Report**: Figures are referenced in `docs/REPORT.md`
2. **Rust Simulator**: The Rust implementation (../src/) uses the same Newton-Raphson algorithm
3. **Validation**: Python results validate the Rust implementation

## Performance

- Full cycle analysis (361 points): ~1 second
- Three figures generation: ~2-3 seconds total
- No optimization needed (batch processing)

## Future Enhancements

- [ ] Add velocity/acceleration plots
- [ ] Generate animation frames for videos
- [ ] Comparative analysis with different mechanisms
- [ ] Export data to CSV for external analysis

## References

See main project README and technical report for complete references.

---

**Note**: For the interactive GUI simulator, see the main Rust project in `../src/`.