#!/usr/bin/env python3
"""
生成四連桿機構分析報告所需的圖表
Four-Bar Linkage Analysis - Figure Generator for Report

This script generates:
- Figure A: θ3 and θ4 vs θ2 plots
- Figure B: Newton-Raphson convergence plot (iterations vs error)
"""

from pathlib import Path

import matplotlib
import matplotlib.pyplot as plt
import numpy as np

# Use default font (no Chinese font needed)
matplotlib.rcParams["font.sans-serif"] = ["DejaVu Sans", "Arial"]
matplotlib.rcParams["axes.unicode_minus"] = False

# 設定輸出目錄（儲存到專案根目錄的 figures 資料夾）
# pyscript/generate_figures.py -> project_root/figures/
OUTPUT_DIR = Path(__file__).parent.parent / "figures"
OUTPUT_DIR.mkdir(exist_ok=True)


def newton_raphson_4bar(
    r1, r2, r3, r4, theta2_deg, init_guess, track_convergence=False
):
    """
    使用 Newton-Raphson 法解四連桿機構位置

    Parameters:
    -----------
    r1, r2, r3, r4 : float
        四連桿的桿長
    theta2_deg : float
        輸入桿角度（度）
    init_guess : tuple
        初始猜測 (theta3_rad, theta4_rad)
    track_convergence : bool
        是否記錄收斂歷程

    Returns:
    --------
    theta3, theta4 : float
        解得的角度（弧度）
    converged : bool
        是否收斂
    history : list (optional)
        收斂歷程 [(iter, error_norm), ...]
    """
    theta2 = np.radians(theta2_deg)
    theta3, theta4 = init_guess  # 初始猜測 (radians)

    tol = 1e-9
    max_iter = 100

    history = [] if track_convergence else None

    for iteration in range(max_iter):
        # 計算誤差函數值 (f1, f2)
        f1 = r2 * np.cos(theta2) + r3 * np.cos(theta3) - r4 * np.cos(theta4) - r1
        f2 = r2 * np.sin(theta2) + r3 * np.sin(theta3) - r4 * np.sin(theta4)

        error_norm = np.sqrt(f1**2 + f2**2)

        if track_convergence:
            history.append((iteration, error_norm))

        if abs(f1) < tol and abs(f2) < tol:
            if track_convergence:
                return theta3, theta4, True, history
            return theta3, theta4, True

        # Jacobian Matrix Elements
        J11 = -r3 * np.sin(theta3)
        J12 = r4 * np.sin(theta4)
        J21 = r3 * np.cos(theta3)
        J22 = -r4 * np.cos(theta4)

        det = J11 * J22 - J12 * J21

        if abs(det) < 1e-9:
            print(f"Warning: Singularity at theta2={theta2_deg:.1f}°")
            if track_convergence:
                return theta3, theta4, False, history
            return theta3, theta4, False

        # 解線性方程組 J * d_theta = -F
        d_theta3 = ((-f1) * J22 - (-f2) * J12) / det
        d_theta4 = (J11 * (-f2) - J21 * (-f1)) / det

        theta3 += d_theta3
        theta4 += d_theta4

    print(f"Warning: Max iterations reached at theta2={theta2_deg:.1f}°")
    if track_convergence:
        return theta3, theta4, False, history
    return theta3, theta4, False


def analytical_solution(r1, r2, r3, r4, theta2_deg):
    """
    使用解析解計算四連桿機構的初始猜測值
    Returns the Open configuration solution
    """
    theta2 = np.radians(theta2_deg)

    # 使用解析法求解 (開口型配置)
    K1 = r1 / r2
    K2 = r1 / r4
    K3 = (r1**2 + r2**2 + r3**2 - r4**2) / (2 * r2 * r3)

    A = np.cos(theta2) - K1 - K2 * np.cos(theta2) + K3
    B = -2 * np.sin(theta2)
    C = K1 - (K2 + 1) * np.cos(theta2) + K3

    # 使用半角替換求解 θ4
    discriminant = B**2 - 4 * A * C
    if discriminant < 0:
        return None, None

    # Open configuration (取較小的角度)
    theta4_solution1 = 2 * np.arctan((-B + np.sqrt(discriminant)) / (2 * A))
    theta4_solution2 = 2 * np.arctan((-B - np.sqrt(discriminant)) / (2 * A))

    # 選擇 Open configuration
    theta4 = theta4_solution1

    # 由 θ4 反推 θ3
    K4 = r1 / r3
    K5 = (r4**2 - r1**2 - r2**2 - r3**2) / (2 * r2 * r3)

    D = np.cos(theta2) - K1 + K4 * np.cos(theta4) + K5
    E = -2 * np.sin(theta2)
    F = K1 + (K4 - 1) * np.cos(theta4) + K5

    discriminant2 = E**2 - 4 * D * F
    if discriminant2 < 0:
        return theta4, None

    theta3 = 2 * np.arctan((-E - np.sqrt(discriminant2)) / (2 * D))

    return theta3, theta4


def generate_figure_a():
    """
    Figure A: θ3 and θ4 vs θ2 Position Analysis
    """
    print("Generating Figure A: Position Analysis...")

    # 參數設定
    r1, r2, r3, r4 = 6.0, 2.0, 5.0, 5.0
    theta2_range = np.linspace(0, 360, 361)

    theta3_sol = []
    theta4_sol = []

    # 使用解析解獲得第一個點的初始猜測
    t3_init, t4_init = analytical_solution(r1, r2, r3, r4, theta2_range[0])
    if t3_init is None or t4_init is None:
        # 如果解析解失敗，使用經驗值
        current_guess = (np.radians(45), np.radians(45))
    else:
        current_guess = (t3_init, t4_init)

    for t2 in theta2_range:
        t3, t4, converged = newton_raphson_4bar(r1, r2, r3, r4, t2, current_guess)

        if converged:
            theta3_sol.append(np.degrees(t3))
            theta4_sol.append(np.degrees(t4))
            # 使用上一次的解作為下一次的初始猜測
            current_guess = (t3, t4)
        else:
            theta3_sol.append(np.nan)
            theta4_sol.append(np.nan)

    # Plot
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8))

    # θ3 vs θ2
    ax1.plot(theta2_range, theta3_sol, "b-", linewidth=2, label="θ₃ (Link 3)")
    ax1.set_xlabel("θ₂ (Input Crank Angle) [deg]", fontsize=12)
    ax1.set_ylabel("θ₃ [deg]", fontsize=12)
    ax1.set_title("Link 3 Angular Position", fontsize=14, fontweight="bold")
    ax1.grid(True, alpha=0.3)
    ax1.legend(fontsize=11)
    ax1.set_xlim(0, 360)

    # θ4 vs θ2
    ax2.plot(theta2_range, theta4_sol, "r-", linewidth=2, label="θ₄ (Link 4 / Rocker)")
    ax2.set_xlabel("θ₂ (Input Crank Angle) [deg]", fontsize=12)
    ax2.set_ylabel("θ₄ [deg]", fontsize=12)
    ax2.set_title("Link 4 Angular Position", fontsize=14, fontweight="bold")
    ax2.grid(True, alpha=0.3)
    ax2.legend(fontsize=11)
    ax2.set_xlim(0, 360)

    plt.tight_layout()

    # Save figure
    output_path = OUTPUT_DIR / "figure_a_position_analysis.png"
    plt.savefig(output_path, dpi=300, bbox_inches="tight")
    print(f"✓ Figure A saved: {output_path}")
    plt.close()

    return theta3_sol, theta4_sol


def generate_figure_b():
    """
    Figure B: Newton-Raphson Convergence Analysis
    Iterations vs Error (log scale)
    """
    print("\nGenerating Figure B: Convergence Analysis...")

    # 參數設定
    r1, r2, r3, r4 = 6.0, 2.0, 5.0, 5.0
    test_theta2 = 45.0  # 選擇 θ2 = 45° 作為測試案例

    # Use analytical solution with offset as initial guess to demonstrate convergence
    t3_true, t4_true = analytical_solution(r1, r2, r3, r4, test_theta2)
    if t3_true is not None and t4_true is not None:
        # Offset by 20 degrees to show convergence process
        init_guess = (t3_true + np.radians(20), t4_true + np.radians(20))
    else:
        init_guess = (np.radians(60), np.radians(60))

    t3, t4, converged, history = newton_raphson_4bar(
        r1, r2, r3, r4, test_theta2, init_guess, track_convergence=True
    )

    if history:
        iterations = [h[0] for h in history]
        errors = [h[1] for h in history]

        # Plot
        fig, ax = plt.subplots(figsize=(10, 6))

        ax.semilogy(
            iterations,
            errors,
            "o-",
            linewidth=2,
            markersize=8,
            color="darkgreen",
            label="Error Norm ||F||",
        )
        ax.axhline(
            y=1e-9,
            color="red",
            linestyle="--",
            linewidth=1.5,
            label="Tolerance = 1e-9",
        )

        ax.set_xlabel("Iteration Number", fontsize=12)
        ax.set_ylabel("Error ||F||", fontsize=12)
        ax.set_title(
            f"Newton-Raphson Convergence Analysis (θ₂ = {test_theta2}°)",
            fontsize=14,
            fontweight="bold",
        )
        ax.grid(True, alpha=0.3, which="both")
        ax.legend(fontsize=11)

        # Add convergence information
        textstr = f"Total Iterations: {len(iterations)}\nFinal Error: {errors[-1]:.2e}"
        props = dict(boxstyle="round", facecolor="wheat", alpha=0.8)
        ax.text(
            0.65,
            0.95,
            textstr,
            transform=ax.transAxes,
            fontsize=11,
            verticalalignment="top",
            bbox=props,
        )

        plt.tight_layout()

        # Save figure
        output_path = OUTPUT_DIR / "figure_b_convergence.png"
        plt.savefig(output_path, dpi=300, bbox_inches="tight")
        print(f"✓ Figure B saved: {output_path}")
        plt.close()

        # Output numerical results
        print(f"\nConvergence Results (θ₂ = {test_theta2}°):")
        print(f"  θ₃ = {np.degrees(t3):.4f}°")
        print(f"  θ₄ = {np.degrees(t4):.4f}°")
        print(f"  Iterations = {len(iterations)}")
        print(f"  Final Error = {errors[-1]:.2e}")


def generate_combined_figure():
    """
    Additional Figure: Combined θ3 and θ4 on same plot
    """
    print("\nGenerating Additional Figure: Combined Position Plot...")

    # Parameters
    r1, r2, r3, r4 = 6.0, 2.0, 5.0, 5.0
    theta2_range = np.linspace(0, 360, 361)

    theta3_sol = []
    theta4_sol = []

    # Use analytical solution for initial guess
    t3_init, t4_init = analytical_solution(r1, r2, r3, r4, theta2_range[0])
    if t3_init is None or t4_init is None:
        current_guess = (np.radians(45), np.radians(45))
    else:
        current_guess = (t3_init, t4_init)

    for t2 in theta2_range:
        t3, t4, converged = newton_raphson_4bar(r1, r2, r3, r4, t2, current_guess)

        if converged:
            theta3_sol.append(np.degrees(t3))
            theta4_sol.append(np.degrees(t4))
            current_guess = (t3, t4)
        else:
            theta3_sol.append(np.nan)
            theta4_sol.append(np.nan)

    # Plot
    fig, ax = plt.subplots(figsize=(10, 6))

    ax.plot(theta2_range, theta3_sol, "b-", linewidth=2, label="θ₃ (Link 3)")
    ax.plot(theta2_range, theta4_sol, "r-", linewidth=2, label="θ₄ (Link 4)")

    ax.set_xlabel("θ₂ (Input Crank Angle) [deg]", fontsize=12)
    ax.set_ylabel("Angle [deg]", fontsize=12)
    ax.set_title(
        "Four-Bar Linkage Position Analysis",
        fontsize=14,
        fontweight="bold",
    )
    ax.grid(True, alpha=0.3)
    ax.legend(fontsize=12, loc="best")
    ax.set_xlim(0, 360)

    # Add parameters info
    textstr = f"Parameters:\nr₁ = {r1}, r₂ = {r2}\nr₃ = {r3}, r₄ = {r4}"
    props = dict(boxstyle="round", facecolor="lightblue", alpha=0.8)
    ax.text(
        0.02,
        0.98,
        textstr,
        transform=ax.transAxes,
        fontsize=10,
        verticalalignment="top",
        bbox=props,
    )

    plt.tight_layout()

    output_path = OUTPUT_DIR / "figure_combined_positions.png"
    plt.savefig(output_path, dpi=300, bbox_inches="tight")
    print(f"✓ Combined figure saved: {output_path}")
    plt.close()


if __name__ == "__main__":
    print("=" * 60)
    print("Four-Bar Linkage Analysis - Figure Generator")
    print("=" * 60)

    # Generate Figure A
    theta3_data, theta4_data = generate_figure_a()

    # Generate Figure B
    generate_figure_b()

    # Generate combined figure
    generate_combined_figure()

    print("\n" + "=" * 60)
    print("All figures generated successfully!")
    print(f"Figures saved to: {OUTPUT_DIR.absolute()}")
    print("=" * 60)
