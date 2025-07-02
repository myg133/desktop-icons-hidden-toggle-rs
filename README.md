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

每次执行 `hideIcons.exe` 程序，它都会自动切换桌面图标的显示/隐藏状态。您可以将其放置在方便的位置，例如桌面快捷方式，以便快速访问。

## 代码结构

- `src/main.rs`: 包含程序的主要逻辑，包括查找窗口句柄和切换图标可见性的函数。

## 许可证

本项目采用 MIT 许可证。详情请参阅 `LICENSE` 文件（如果存在）。