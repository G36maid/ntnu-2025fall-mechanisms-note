---
title: "四連桿機構運動分析：基於 Newton-Raphson 數值方法之研究"
author: "鍾詠傑"
student_id: "41173058h"
affiliation: "國立臺灣師範大學 機電工程學系"
date: "2025 年 12 月"
abstract: |
  本研究採用 Newton-Raphson 數值方法對四連桿機構進行位置分析。針對給定桿長參數的四連桿組，當輸入桿旋轉一周（$0^\circ \le \theta_2 \le 360^\circ$）時，計算連結桿與輸出桿的角度變化。研究結果顯示，Newton-Raphson 方法具有快速收斂特性（典型迭代次數 5-15 次，誤差 < $10^{-9}$），適用於連續運動分析。本研究亦開發互動式模擬器與自動化圖表生成系統，驗證數值解之正確性與收斂行為。
keywords: "四連桿機構、Newton-Raphson 方法、運動分析、數值方法、機構學"
---

## 1. 緒論

### 1.1 研究背景

機構學中，位置分析是進行速度與加速度分析的基礎。四連桿機構作為最基本的平面連桿組，廣泛應用於工程實務中。雖然四連桿機構存在封閉形式的解析解（Freudenstein Equation），但在處理複雜機構或進行連續運動分析時，數值方法（如 Newton-Raphson 法）具有更高的通用性與計算效率。

### 1.2 研究目的

本研究旨在：

1. 建立四連桿機構之向量迴路方程式（Vector Loop Equation）
2. 實作 Newton-Raphson 數值迭代演算法求解位置問題
3. 分析數值方法的收斂特性與精度
4. 驗證數值解與機構運動學理論的一致性

### 1.3 研究範圍

本研究針對特定參數的曲柄搖桿機構（Crank-Rocker Mechanism）進行完整轉動週期（$0^\circ$ 至 $360^\circ$）的位置分析，不涉及速度與加速度分析。

---

## 2. 理論基礎

### 2.1 向量迴路方程式

考慮一四連桿機構，各桿向量定義如下：

- $\vec{r_1}$：固定桿（地桿，Ground Link）
- $\vec{r_2}$：輸入桿（曲柄，Input Crank）
- $\vec{r_3}$：連結桿（浮桿，Coupler Link）
- $\vec{r_4}$：輸出桿（搖桿，Output Rocker）

根據向量封閉原理，向量迴路方程式為：

$$
\vec{r_2} + \vec{r_3} - \vec{r_4} - \vec{r_1} = 0
\tag{1}
$$

### 2.2 投影方程式

將式 (1) 分別投影至 X 軸與 Y 軸，可得兩個非線性方程式：

$$
\begin{aligned}
f_1(\theta_3, \theta_4) &= r_2 \cos\theta_2 + r_3 \cos\theta_3 - r_4 \cos\theta_4 - r_1 = 0 \\
f_2(\theta_3, \theta_4) &= r_2 \sin\theta_2 + r_3 \sin\theta_3 - r_4 \sin\theta_4 = 0
\end{aligned}
\tag{2}
$$

其中 $\theta_2$ 為已知輸入角，$\theta_3$ 與 $\theta_4$ 為待求未知數。

### 2.3 Newton-Raphson 方法

Newton-Raphson 方法是求解非線性方程組的經典數值方法。對於方程組 $\mathbf{F}(\boldsymbol{\theta}) = \mathbf{0}$，其迭代公式為：

$$
\boldsymbol{\theta}^{(k+1)} = \boldsymbol{\theta}^{(k)} - \mathbf{J}^{-1}(\boldsymbol{\theta}^{(k)}) \mathbf{F}(\boldsymbol{\theta}^{(k)})
\tag{3}
$$

其中 $\mathbf{J}$ 為 Jacobian 矩陣。

### 2.4 Jacobian 矩陣推導

對式 (2) 分別對 $\theta_3$ 與 $\theta_4$ 求偏微分：

$$
\mathbf{J} = \begin{bmatrix}
\frac{\partial f_1}{\partial \theta_3} & \frac{\partial f_1}{\partial \theta_4} \\
\frac{\partial f_2}{\partial \theta_3} & \frac{\partial f_2}{\partial \theta_4}
\end{bmatrix}
= \begin{bmatrix}
-r_3 \sin\theta_3 & r_4 \sin\theta_4 \\
r_3 \cos\theta_3 & -r_4 \cos\theta_4
\end{bmatrix}
\tag{4}
$$

Jacobian 矩陣的行列式值為：

$$
\det(\mathbf{J}) = r_3 r_4 \sin(\theta_4 - \theta_3)
\tag{5}
$$

當 $\theta_3 \approx \theta_4$ 或 $|\theta_4 - \theta_3| \approx 180^\circ$ 時，機構處於奇異位形（Singular Configuration），數值解可能失效。

### 2.5 修正量計算

利用 Cramer 法則或矩陣求逆，可得角度修正量：

$$
\begin{aligned}
\Delta\theta_3 &= \frac{f_1 (r_4 \cos\theta_4) - f_2 (r_4 \sin\theta_4)}{\det(\mathbf{J})} \\
\Delta\theta_4 &= \frac{f_2 (r_3 \sin\theta_3) - f_1 (r_3 \cos\theta_3)}{\det(\mathbf{J})}
\end{aligned}
\tag{6}
$$

### 2.6 Grashof 條件

根據 Grashof 定理，設 $S$、$L$、$P$、$Q$ 分別為最短桿、最長桿及另外兩桿長度，若滿足：

$$
S + L \le P + Q
\tag{7}
$$

則至少有一桿可作完整轉動。當最短桿為輸入桿且固定桿為鄰桿時，機構為曲柄搖桿機構。

---

## 3. 研究方法

### 3.1 機構參數設定

本研究採用以下機構參數：

- 固定桿長度：$r_1 = 6.0$ (單位長度)
- 輸入桿長度：$r_2 = 2.0$
- 連結桿長度：$r_3 = 5.0$
- 輸出桿長度：$r_4 = 5.0$
- 輸入角範圍：$0^\circ \le \theta_2 \le 360^\circ$（取樣間隔 $1^\circ$）

**機構類型判定：**

計算 Grashof 參數：$S = 2.0$，$L = 6.0$，$P = 5.0$，$Q = 5.0$

$$
S + L = 8.0 \le P + Q = 10.0
$$

滿足 Grashof 條件，且最短桿為輸入桿，故此機構為**曲柄搖桿機構**。

### 3.2 Newton-Raphson 演算法實作

本研究採用以下演算法流程：

#### 演算法 1：四連桿機構位置求解（Newton-Raphson 方法）

```
輸入：桿長 (r₁, r₂, r₃, r₄)、輸入角 θ₂、初始猜測值 (θ₃⁽⁰⁾, θ₄⁽⁰⁾)
輸出：連結桿角 θ₃、輸出桿角 θ₄、收斂狀態

1. 設定收斂參數：
   - 容許誤差 ε = 10⁻⁹
   - 最大迭代次數 N_max = 100

2. 初始化：k ← 0, θ₃ ← θ₃⁽⁰⁾, θ₄ ← θ₄⁽⁰⁾

3. 迭代求解：
   WHILE k < N_max DO
      a. 計算誤差函數值：
         f₁ ← r₂cos(θ₂) + r₃cos(θ₃) - r₄cos(θ₄) - r₁
         f₂ ← r₂sin(θ₂) + r₃sin(θ₃) - r₄sin(θ₄)
      
      b. 檢查收斂條件：
         IF |f₁| < ε AND |f₂| < ε THEN
            RETURN (θ₃, θ₄, TRUE)
         END IF
      
      c. 計算 Jacobian 矩陣元素：
         J₁₁ ← -r₃sin(θ₃)
         J₁₂ ← r₄sin(θ₄)
         J₂₁ ← r₃cos(θ₃)
         J₂₂ ← -r₄cos(θ₄)
      
      d. 計算行列式：
         D ← J₁₁·J₂₂ - J₁₂·J₂₁
      
      e. 奇異性檢查：
         IF |D| < 10⁻⁹ THEN
            RETURN (θ₃, θ₄, FALSE)  // 奇異點
         END IF
      
      f. 計算修正量（Cramer's Rule）：
         Δθ₃ ← ((-f₁)·J₂₂ - (-f₂)·J₁₂) / D
         Δθ₄ ← (J₁₁·(-f₂) - J₂₁·(-f₁)) / D
      
      g. 更新角度：
         θ₃ ← θ₃ + Δθ₃
         θ₄ ← θ₄ + Δθ₄
      
      h. k ← k + 1
   END WHILE

4. IF k ≥ N_max THEN
      RETURN (θ₃, θ₄, FALSE)  // 未收斂
   END IF
```

#### 演算法 2：完整週期分析

```
輸入：桿長參數、輸入角陣列 Θ₂ = [0°, 1°, ..., 360°]
輸出：角度解陣列 Θ₃[], Θ₄[]

1. 初始化：
   - 設定初始猜測值 (θ₃⁽⁰⁾, θ₄⁽⁰⁾)  // 基於幾何分析
   - 建立空陣列 Θ₃[], Θ₄[]

2. FOR EACH θ₂ IN Θ₂ DO
      a. 呼叫演算法 1 求解 (θ₃, θ₄, 收斂狀態)
      
      b. IF 收斂成功 THEN
            - 將 θ₃, θ₄ 加入結果陣列
            - 更新下次初始猜測值：(θ₃⁽⁰⁾, θ₄⁽⁰⁾) ← (θ₃, θ₄)
         ELSE
            - 記錄奇異點或未收斂情況
         END IF
   END FOR

3. RETURN (Θ₃, Θ₄)
```

### 3.3 初始猜測值策略

為確保收斂至正確的機構位形（開口型 Open 或交叉型 Crossed），本研究採用**連續追蹤策略**：

1. **第一步**（$\theta_2 = 0^\circ$）：基於幾何分析給定初始猜測值
2. **後續步**：使用前一步的收斂解作為下一步的初始猜測值

此策略可有效避免位形跳躍，並顯著減少迭代次數（通常 3-5 次即可收斂）。

### 3.4 收斂準則

本研究採用雙重收斂準則：

$$
|f_1(\theta_3, \theta_4)| < \epsilon \quad \text{且} \quad |f_2(\theta_3, \theta_4)| < \epsilon
$$

其中容許誤差 $\epsilon = 10^{-9}$，確保解的高精度。

---

## 4. 結果與分析

### 4.1 位置分析結果

圖 4.1 展示了輸入桿旋轉一周（$0^\circ$ 至 $360^\circ$）時，連結桿角 $\theta_3$ 與輸出桿角 $\theta_4$ 的變化曲線。

![圖 4.1：θ₃ 與 θ₄ 位置分析曲線](../figures/figure_a_position_analysis.png)

**圖 4.1：連結桿與輸出桿角度隨輸入角變化之關係**

**觀察結果：**

1. **連結桿（θ₃）**：角度在約 -360° 至 0° 範圍內變化，呈現週期性的非線性振盪特性
2. **輸出桿（θ₄）**：角度在約 400° 至 500° 範圍內往復擺動，擺動範圍約 100°
3. **機構特性確認**：輸出桿的有限擺動範圍證實了此機構為曲柄搖桿型態

圖 4.2 將兩條曲線繪製於同一座標系統中，便於比較兩者的相對運動關係。

![圖 4.2：組合位置分析圖](../figures/figure_combined_positions.png)

**圖 4.2：θ₃ 與 θ₄ 角度變化綜合比較**

### 4.2 收斂特性分析

為驗證 Newton-Raphson 方法的收斂行為，本研究選擇 $\theta_2 = 45^\circ$ 作為測試案例，追蹤完整的迭代過程。

![圖 4.3：Newton-Raphson 收斂曲線（θ₂ = 45°）](../figures/figure_b_convergence.png)

**圖 4.3：Newton-Raphson 方法收斂行為分析**

**收斂統計：**

- **迭代次數**：13 次
- **最終誤差**：$< 1.0 \times 10^{-9}$
- **收斂率**：二次收斂（Quadratic Convergence）

從圖 4.3 可觀察到典型的二次收斂特性：誤差在每次迭代後呈指數級下降。初始數次迭代誤差下降較慢，但當進入收斂域後，誤差快速趨近於零。

### 4.3 計算效率分析

本研究統計了完整週期（361 個點）的計算效率：

| 指標 | 數值 |
|------|------|
| 總迭代次數 | ~1500 次 |
| 平均迭代次數/點 | 4.2 次 |
| 收斂成功率 | 100% |
| 計算精度 | $< 10^{-9}$ |

結果顯示，採用連續追蹤策略後，平均每點僅需 4-5 次迭代即可達到極高精度，證明了該方法的計算效率。

---

## 5. 討論

### 5.1 數值方法之優勢

相較於解析解（Freudenstein Equation），Newton-Raphson 數值方法具有以下優勢：

1. **通用性高**：可直接擴展至多連桿或空間機構
2. **實作簡潔**：無需推導複雜的封閉解
3. **精度可控**：可依需求調整容許誤差
4. **連續分析適用**：配合追蹤策略可高效處理連續運動

### 5.2 初始猜測值之重要性

Newton-Raphson 方法對初始猜測值敏感。不當的初始值可能導致：

1. **收斂至錯誤解**：跳至另一組位形（Open ↔ Crossed）
2. **收斂速度慢**：迭代次數增加
3. **無法收斂**：誤差振盪或發散

本研究採用的連續追蹤策略有效解決了此問題，確保解的連續性與正確性。

### 5.3 奇異性問題

當 Jacobian 矩陣行列式趨近於零時，即：

$$
\det(\mathbf{J}) = r_3 r_4 \sin(\theta_4 - \theta_3) \approx 0
$$

機構處於奇異位形。此時數值解可能失效。理論上發生條件為：

- $\theta_3 \approx \theta_4$（桿 3 與桿 4 共線）
- $|\theta_4 - \theta_3| \approx 180^\circ$（桿 3 與桿 4 反向共線）

本研究所選機構在完整運動週期中未遭遇奇異性，所有測試點均成功收斂。

### 5.4 機構運動學特性

從位置分析結果可知：

1. **非線性運動**：雖然輸入桿等速旋轉，但輸出桿與連結桿均為變速運動
2. **週期性**：機構運動具有嚴格週期性（週期 = 360°）
3. **連續性**：角度變化連續光滑，無突跳現象

此特性符合曲柄搖桿機構的典型行為。

### 5.5 研究限制

本研究存在以下限制：

1. **位置分析限定**：未涉及速度與加速度分析
2. **平面機構限定**：未擴展至空間機構
3. **單一案例**：僅分析特定桿長參數

未來可擴展至速度/加速度分析、動力分析及機構綜合（Synthesis）。

---

## 6. 結論

本研究成功運用 Newton-Raphson 數值方法完成四連桿機構之位置分析，主要成果如下：

1. **理論驗證**：數值解與機構運動學理論高度一致，驗證了方法的正確性
2. **收斂特性**：證實 Newton-Raphson 方法具有二次收斂特性，計算效率高
3. **實用策略**：連續追蹤策略有效提升收斂速度（平均 4-5 次迭代）與解的穩定性
4. **工程應用**：開發之演算法與模擬系統可應用於機構設計與教學

相較於傳統圖解法，數值方法提供高精度的連續解，為後續的速度、加速度及動力分析奠定基礎。本研究證明了數值方法在計算機輔助機構分析（CAMA）中的有效性與實用性。

### 6.1 未來展望

建議未來研究方向：

1. 擴展至速度與加速度分析（數值微分法）
2. 整合動力分析（考慮慣性力與外力）
3. 發展機構最佳化設計工具
4. 擴展至空間機構分析（3D 連桿組）

---

## 誌謝

感謝國立臺灣師範大學機電工程學系機構學課程提供理論基礎與研究指導。本研究使用開源軟體工具（Rust、Python、Matplotlib）完成實作與視覺化。

---

## 參考文獻

1. Norton, R. L. (2019). *Design of Machinery: An Introduction to the Synthesis and Analysis of Mechanisms and Machines* (6th ed.). McGraw-Hill Education.

2. Erdman, A. G., Sandor, G. N., & Kota, S. (2001). *Mechanism Design: Analysis and Synthesis* (4th ed., Vol. 1). Prentice Hall.

3. Uicker, J. J., Pennock, G. R., & Shigley, J. E. (2017). *Theory of Machines and Mechanisms* (5th ed.). Oxford University Press.

4. Angeles, J. (2014). *Fundamentals of Robotic Mechanical Systems: Theory, Methods, and Algorithms* (4th ed.). Springer.

5. 國立臺灣師範大學機電工程學系（2024）。*機構學講義：第四章至第五章 —— 連桿機構運動分析*。

6. Freudenstein, F. (1955). Approximate synthesis of four-bar linkages. *Transactions of the ASME*, 77, 853-861.

7. Haug, E. J. (1989). *Computer Aided Kinematics and Dynamics of Mechanical Systems* (Vol. 1). Allyn and Bacon.

8. Press, W. H., Teukolsky, S. A., Vetterling, W. T., & Flannery, B. P. (2007). *Numerical Recipes: The Art of Scientific Computing* (3rd ed.). Cambridge University Press.

---

## 附錄

### 附錄 A：程式執行環境

**Rust 模擬器：**
- 版本：Rust 1.70+
- 框架：eframe + egui
- 執行指令：`cargo run --release`

**Python 圖表生成：**
- 版本：Python 3.13+
- 套件管理：uv
- 主要函式庫：NumPy, Matplotlib
- 執行指令：`uv run pyscript/generate_figures.py`

### 附錄 B：圖表清單

所有圖表儲存於 `figures/` 目錄：

- `figure_a_position_analysis.png` - 位置分析（分離圖）
- `figure_b_convergence.png` - 收斂特性分析
- `figure_combined_positions.png` - 位置分析（組合圖）

圖表規格：300 DPI，PNG 格式，適用於學術論文印刷。

### 附錄 C：線上模擬器

本研究開發之互動式模擬器已部署至 GitHub Pages：

**網址**：[https://g36maid.github.io/fourbar-linkage-analysis/](https://g36maid.github.io/fourbar-linkage-analysis/)

使用者可即時調整機構參數、觀察運動軌跡，無需安裝任何軟體。

![圖 C.1：互動式模擬器介面截圖](../figures/demo.png)

**圖 C.1：Web 版互動式四連桿機構模擬器**

模擬器主要功能：

- **即時參數調整**：透過滑桿即時修改桿長（r₁, r₂, r₃, r₄）與輸入角 θ₂
- **動畫播放**：自動旋轉功能，可調整動畫速度（0.5 - 10.0 °/frame）
- **軌跡追蹤**：顯示耦合點運動軌跡（橘色曲線）
- **視覺化輔助**：可選顯示網格、角度標註
- **機構類型判定**：自動計算並顯示 Grashof 條件與機構型態
- **高效能計算**：使用 WebAssembly 技術，達到 60 FPS 流暢渲染

---

**報告完成日期**：2025 年 12 月 3 日  
**版本**：1.0