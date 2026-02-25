# 機構學作業解答 (Mechanisms Homework Solutions)

本目錄包含機構學課程的作業解答，使用 Markdown 格式整理，便於閱讀與版本控制。

## 目錄結構

```
solutions/
├── README.md                           # 本文件
├── computer_aided_analysis.md          # 電腦輔助機構分析報告
├── hw1.md                              # 作業 1 解答
├── hw3.md                              # 作業 3 解答
├── figures/                            # 圖表資料夾
│   ├── figure_a_position_analysis.png
│   ├── figure_b_convergence.png
│   └── figure_combined_positions.png
└── python_analysis/                    # Python 分析程式
    ├── README.md
    ├── generate_figures.py
    ├── run.sh
    └── pyproject.toml
```

## 作業列表

### ✅ 作業 1 (Homework 1)
- **檔案**: [hw1.md](hw1.md)
- **內容**: 基本概念與機構分析

### ✅ 作業 3 (Homework 3)
- **檔案**: [hw3.md](hw3.md)
- **內容**: 連桿機構分析

### ✅ 電腦輔助機構分析 (Computer Aided Mechanism Analysis)
- **檔案**: [computer_aided_analysis.md](computer_aided_analysis.md)
- **主題**: 使用 Newton-Raphson 方法分析四連桿機構
- **內容**:
  - 理論推導（向量迴路方程、Jacobian 矩陣）
  - Python 程式實作
  - 位置分析圖表
  - Newton-Raphson 收斂分析
  - 數值解與解析解比較

## Python 分析工具

### 快速開始

如果您想重新生成電腦輔助分析的圖表：

```bash
cd python_analysis
./run.sh
```

或者使用 `uv` 直接執行：

```bash
cd python_analysis
uv run generate_figures.py
```

### 系統需求

- Python 3.13+
- uv (Python 套件管理器)
- 相依套件: numpy, matplotlib (自動安裝)

### 生成的圖表

所有圖表儲存於 `figures/` 目錄：

1. **figure_a_position_analysis.png**
   - 位置分析：θ₃ 和 θ₄ 對應 θ₂ 的變化
   - 分離式上下兩圖，清楚展示各桿角度變化

2. **figure_b_convergence.png**
   - Newton-Raphson 收斂分析
   - 展示迭代次數 vs 誤差（對數座標）
   - 證明二次收斂特性

3. **figure_combined_positions.png**
   - θ₃ 與 θ₄ 組合在同一座標系
   - 便於比較兩桿的相對運動

## 機構參數 (用於電腦輔助分析)

- **桿長**:
  - r₁ = 6.0 (地桿 / Ground Link)
  - r₂ = 2.0 (輸入桿 / Input Crank)
  - r₃ = 5.0 (連結桿 / Coupler Link)
  - r₄ = 5.0 (輸出桿 / Output Rocker)

- **輸入條件**:
  - ω₂ = 100 rpm
  - θ₂ = 0° ~ 360°

- **機構類型**: 曲柄搖桿機構 (Crank-Rocker Mechanism)

## 數值方法說明

### Newton-Raphson 迭代法

**向量迴路方程**:
```
f₁(θ₃, θ₄) = r₂cos(θ₂) + r₃cos(θ₃) - r₄cos(θ₄) - r₁ = 0
f₂(θ₃, θ₄) = r₂sin(θ₂) + r₃sin(θ₃) - r₄sin(θ₄) = 0
```

**Jacobian 矩陣**:
```
J = [ ∂f₁/∂θ₃   ∂f₁/∂θ₄ ]   [ -r₃sin(θ₃)   r₄sin(θ₄) ]
    [ ∂f₂/∂θ₃   ∂f₂/∂θ₄ ] = [  r₃cos(θ₃)  -r₄cos(θ₄) ]
```

**更新公式**:
```
[Δθ₃]       [f₁]
[Δθ₄] = -J⁻¹[f₂]

θ₃ⁿᵉʷ = θ₃ᵒˡᵈ + Δθ₃
θ₄ⁿᵉʷ = θ₄ᵒˡᵈ + Δθ₄
```

**收斂條件**: |f₁| < 10⁻⁹ 且 |f₂| < 10⁻⁹

## 檔案格式規範

- 所有作業解答使用 Markdown 格式
- 數學公式使用 LaTeX 語法（$...$）
- 程式碼使用 fenced code blocks
- 圖片使用相對路徑引用

## 相關連結

- [課程筆記目錄](../../notes/)
- [原始題目](../problems/)
- [Python 分析工具說明](python_analysis/README.md)

## 更新記錄

- 2024-12-03: 新增電腦輔助機構分析報告與圖表生成工具
- 使用 uv 管理 Python 環境
- 所有圖表標籤改為英文以避免字體問題

## 作者

國立台灣師範大學 機電工程學系
機構學課程

## 參考資料

1. Norton, R. L. (2019). *Design of Machinery*. McGraw-Hill Education.
2. 課堂講義：Ch1-Ch7
3. [Newton-Raphson Method](https://en.wikipedia.org/wiki/Newton%27s_method)

---

*Last updated: 2024-12-03*