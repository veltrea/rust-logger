# rust-logger

这是一个用 Rust 编写的 UNIX 标准 `logger` 命令的实现，专为 Windows 环境（命令提示符 / PowerShell）下的安全和可靠使用而设计。它能自动将来自本地化编码（如 Shift-JIS / UTF-16）的输入转换为 UTF-8，并发送到 Syslog 服务器。

本工具旨在与 **[vlt-syslogd](https://github.com/veltrea/vlt-syslogd)** 完美配合使用，后者是一款支持 UTF-8 并能正确显示多语言字符的 Windows Syslog 服务器。

---

### [English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md)

---

## 特性

- **UNIX `logger` 兼容性**: 支持 `-n`、`-P` 和 `-p` 等标准选项。
- **完善的 Windows 支持**: 能够妥善处理来自 `cmd.exe` (Shift-JIS) 和 `PowerShell` (UTF-16) 的输入。
- **单个二进制文件**: 作为独立的可执行文件运行，无需任何依赖。

## 文档

- **[用户手册 (MANUAL.md)](MANUAL.md)**: 详细的使用说明、批处理/PowerShell 的应用实例以及字符编码故障排除指南。

## 安装

从 Releases 页面下载 `rust-logger.exe`，并将其放置在系统的 PATH 环境变量所包含的目录中。

## 用法

基本用法与 UNIX 的 `logger` 命令完全相同。

```powershell
# 基本用法 (默认优先级为 user.info)
rust-logger -n 127.0.0.1 -P 514 "日志消息内容"

# 指定优先级和标签
rust-logger -n 192.0.2.10 -p local0.warn -t MyBatchApp "警告：处理失败"
```

### 选项

| 选项 | 简写 | 说明 | 默认值 |
| :--- | :--- | :--- | :--- |
| `--server` | `-n` | **[必填]** Syslog 服务器的主机名或 IP | - |
| `--port` | `-P` | 目标端口号 | 514 |
| `--priority` | `-p` | 优先级 (格式为 `facility.level`) | `user.info` |
| `--tag` | `-t` | 标签 (标识符) | `rust-logger` |
| `--encoding` | - | 输出编码 (`utf-8`、`shift_jis` 等) | `utf-8` |
| `[MESSAGE]` | - | 日志消息 (位于参数末尾) | - |

## 发行说明

### RFC 5424 多语言支持修复 (2026/02/07)
修复了与 RFC 5424 规范相关的实现错误：
- **MSG 字段格式**: 正确地在 `MSG` 字段开头添加了 BOM 以符合 `MSG-UTF8` 格式。
- **自动编码检测**: 对于 UTF-8 配置为 `MSG-UTF8`（含 BOM），对于其他编码（如 Shift-JIS）配置为 `MSG-ANY`（无 BOM）。
- **Windows 集成**: 优化了内部结构，利用 Rust 的参数规范化透明地处理来自 `cmd.exe` 和 `PowerShell` 的本地化输入。

## 许可证

[MIT License](LICENSE)
