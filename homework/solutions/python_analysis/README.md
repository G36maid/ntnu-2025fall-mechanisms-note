# 四連桿機構數值分析 (Four-Bar Linkage Numerical Analysis)

這個目錄包含使用 Newton-Raphson 方法分析四連桿機構的 Python 程式碼。

## 專案結構

```
python_analysis/
├── README.md              # 本文件
├── generate_figures.py    # 主程式：生成報告所需的圖表
├── pyproject.toml         # uv 專案配置檔
├── .python-version        # Python 版本指定
└── main.py               # (uv 自動生成，未使用)
```

生成的圖表會儲存在上層目錄：
```
../figures/
├── figure_a_position_analysis.png  # 圖表 A：θ3 與 θ4 位置分析
├── figure_b_convergence.png        # 圖表 B：Newton-Raphson 收斂圖
└── figure_combined_positions.png   # 額外圖表：組合位置圖
```

## 快速開始

### 環境要求
- Python 3.13+
- uv (Python 套件管理器)

### 安裝與執行

1. **安裝 uv** (如果尚未安裝):
   ```bash
   curl -LsSf https://astral.sh/uv/install.sh | sh
   ```

2. **執行程式生成圖表**:
   ```bash
   cd python_analysis
   uv run generate_figures.py
   ```

   程式會自動：
   - 建立虛擬環境 (`.venv/`)
   - 安裝所需套件 (`matplotlib`, `numpy`)
   - 執行分析並生成三張圖表

## 程式功能說明

### `generate_figures.py`

主要包含以下功能：

1. **`newton_raphson_4bar()`**
   - 使用 Newton-Raphson 迭代法求解四連桿機構位置
   - 支援收斂歷程追蹤（用於繪製收斂圖）

2. **`analytical_solution()`**
   - 使用解析解計算初始猜測值
   - 提高數值法的收斂穩定性

3. **`generate_figure_a()`**
   - 生成圖表 A：θ3 和 θ4 vs θ2 的分離曲線圖
   - 展示整個運動週期 (0° ≤ θ2 ≤ 360°)

4. **`generate_figure_b()`**
   - 生成圖表 B：Newton-Raphson 收斂分析
   - 選擇 θ2 = 45° 作為測試案例
   - 繪製迭代次數 vs 誤差（對數座標）

5. **`generate_combined_figure()`**
   - 生成額外的組合圖：θ3 與 θ4 在同一座標系

## 機構參數

- **桿長**:
  - r₁ = 6.0 (地桿)
  - r₂ = 2.0 (輸入桿/曲柄)
  - r₃ = 5.0 (連結桿)
  - r₄ = 5.0 (輸出桿/搖桿)

- **輸入條件**:
  - ω₂ = 100 rpm (角速度，本次位置分析未使用)
  - θ₂ = 0° ~ 360° (輸入桿角度範圍)

- **機構類型**:
  - 曲柄搖桿機構 (Crank-Rocker Mechanism)
  - 符合 Grashof 條件：S + L ≤ P + Q

## 數值方法細節

### Newton-Raphson 迭代公式

向量迴路方程式：
```
f₁(θ₃, θ₄) = r₂cos(θ₂) + r₃cos(θ₃) - r₄cos(θ₄) - r₁ = 0
f₂(θ₃, θ₄) = r₂sin(θ₂) + r₃sin(θ₃) - r₄sin(θ₄) = 0
```

Jacobian 矩陣：
```
J = [ -r₃sin(θ₃)   r₄sin(θ₄) ]
    [  r₃cos(θ₃)  -r₄cos(θ₄) ]
```

更新規則：
```
[Δθ₃]   [f₁]
[Δθ₄] = J⁻¹ × [-f₂]
```

### 收斂條件
- 容許誤差 (tolerance): 1.0e-9
- 最大迭代次數: 100
- 誤差定義: ||F|| = √(f₁² + f₂²)

## 常見問題

### Q: 為什麼使用解析解作為初始猜測？
A: Newton-Raphson 方法對初值敏感。使用解析解可以：
   1. 確保選擇正確的配置（Open vs Crossed）
   2. 提高收斂速度
   3. 避免奇異點問題

### Q: 如何修改機構參數？
A: 編輯 `generate_figures.py` 中的參數設定：
   ```python
   r1, r2, r3, r4 = 6.0, 2.0, 5.0, 5.0  # 修改這一行
   ```

### Q: 中文字體顯示問題？
A: 程式會嘗試使用以下字體（依序）：
   - Arial Unicode MS (macOS)
   - DejaVu Sans (Linux)
   - SimHei (Windows)
   
   如果仍有問題，可在程式中修改字體設定。

## 技術堆疊

- **Python 3.13**: 程式語言
- **NumPy**: 數值計算
- **Matplotlib**: 圖表繪製
- **uv**: 套件管理與虛擬環境

## 作者

國立台灣師範大學 機電工程學系
機構學課程作業

## 參考資料

1. Norton, R. L. (2019). *Design of Machinery*. McGraw-Hill Education.
2. 課堂講義：Ch5 機構運動分析 ── 解析法
3. [Newton-Raphson Method - Wikipedia](https://en.wikipedia.org/wiki/Newton%27s_method)