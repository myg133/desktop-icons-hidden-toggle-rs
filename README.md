# hideIcons

`hideIcons` 是一个用 Rust 编写的实用程序，用于在 Windows 系统上切换桌面图标的可见性。每次运行此程序，它都会检查桌面图标的当前状态（显示或隐藏），然后将其切换到相反的状态。

## 功能

- **切换桌面图标可见性**：运行程序即可隐藏或显示桌面图标。

## 构建和运行

### 先决条件

- [Rust 编程语言](https://www.rust-lang.org/)
- Windows 操作系统

### 构建

1. 克隆此仓库：
   ```bash
   git clone https://github.com/your-username/hideIcons.git
   cd hideIcons
   ```

2. 构建项目（调试模式）：
   ```bash
   cargo build
   ```

3. 构建项目（发布模式，优化性能）：
   ```bash
   cargo build --release
   ```

### 运行

- **调试模式运行**：
  ```bash
  cargo run
  ```

- **发布模式运行**：
  ```bash
  .\target\release\hideIcons.exe
  ```

## 使用方法

每次执行 `hideIcons.exe` 程序，它都会自动切换桌面图标的显示/隐藏状态。

### 创建快捷方式并添加快捷键

为了方便使用，您可以为 `hideIcons.exe` 创建一个桌面快捷方式，并为其配置一个快捷键：

1.  **找到 `hideIcons.exe`**：在文件资源管理器中，导航到您构建的 `hideIcons.exe` 文件。如果您是使用 `cargo build --release` 构建的，它通常位于 `target\release\` 目录下。

2.  **创建快捷方式**：右键点击 `hideIcons.exe` 文件，选择 "发送到" -> "桌面 (创建快捷方式)"。

3.  **配置快捷键**：
    -   在桌面上找到新创建的 `hideIcons.exe` 快捷方式。
    -   右键点击快捷方式，选择 "属性"。
    -   在 "快捷方式" 选项卡中，找到 "快捷键(K):" 字段。
    -   点击该字段，然后按下您想要设置的快捷键组合（例如 `Ctrl + Alt + D`）。系统会自动填充快捷键。
    -   点击 "应用"，然后点击 "确定"。

现在，您可以通过按下配置的快捷键来快速切换桌面图标的显示/隐藏状态。

## 代码结构

- `src/main.rs`: 包含程序的主要逻辑，包括查找窗口句柄和切换图标可见性的函数。

## 许可证

本项目采用 MIT 许可证。详情请参阅 `LICENSE` 文件（如果存在）。