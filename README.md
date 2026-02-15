# mini_wc

`mini_wc` 是一个用 Rust 编写的轻量命令行工具，用来统计文本文件的行数、单词数、字节数和字符数，功能类似简化版 `wc`。

## 功能简介

- 默认输出：行数、单词数、字节数、字符数
- 按选项输出单项统计：
- `-l` 行数
- `-w` 单词数
- `-c` 字节数
- `-m` 字符数（Unicode 标量）

## 运行方法（中文）

### 1) 用 Cargo 运行（开发调试）

在项目目录执行：

```powershell
cd d:\rust_test\mini_wc
cargo run -- a.txt
```

指定模式示例：

```powershell
cargo run -- -w a.txt
```

### 2) 安装成系统命令（之后可直接用，不用每次 cargo）

首次安装：

```powershell
cd d:\rust_test\mini_wc
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

安装完成后，打开新终端可直接执行：

```powershell
mini_wc a.txt
miniwc -l a.txt
miniwc -w a.txt
miniwc -c a.txt
miniwc -m a.txt
```

如果在当前目录运行本地可执行文件，请使用：

```powershell
.\mini_wc.exe a.txt
```

## 命令用法

```text
mini_wc <file>
mini_wc -l <file>   # lines
mini_wc -w <file>   # words
mini_wc -c <file>   # bytes
mini_wc -m <file>   # chars (unicode)
mini_wc --help
```

## 示例输出

```text
lines=2 words=8 bytes=42 chars=42  a.txt
words=8  a.txt
```

## 项目结构

```text
mini_wc/
  src/main.rs   # 程序入口
  src/cli.rs    # 参数解析与输出逻辑
  src/stats.rs  # 统计核心逻辑与单元测试
  install.ps1   # 一键安装到用户命令目录
```
