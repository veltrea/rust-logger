# rust-logger 用户手册

`rust-logger` 是一个在 Windows 上重现 UNIX 标准 `logger` 命令功能的工具，并针对日语环境进行了增强。它用于从 Windows 上的各种脚本和应用程序向 Syslog 服务器发送消息。

---

## 1. 什么是 Syslog？
Syslog 是用于通过网络转发日志消息的标准协议。
通常，它结合了发送端工具（如 `rust-logger`）和接收/存储端服务器（如 `vlt-syslogd`）来使用。

### 消息的组成部分
除了正文之外，Syslog 消息还包含以下信息：
- **优先级 (Priority)**: 消息的重要性。指定为 `facility.level` (例如: `user.info`)。
- **标签 (Tag)**: 指示哪个应用程序生成了日志的标识符 (例如: `web-server`)。
- **时间戳**: 生成日志的日期和时间。
- **主机名**: 发送日志的机器名称。

---

## 2. 基本用法
您只需将消息添加到命令行的末尾即可发送。

### 最小执行
```cmd
rust-logger -n 127.0.0.1 "测试消息"
```

### 优先级 (-p) 使用示例
根据情况指定适当的优先级对于在 Syslog 服务器端启用有效的过滤和通知设置非常重要。

| 情况 | 推荐命令示例 | 意图 |
| :--- | :--- | :--- |
| **正常操作记录** | `-p user.info` | 正常记录，例如“已开始...”或“已完成...”。 |
| **需引起注意** | `-p user.warning` | “发生重试”，“硬盘空间不足”等。 |
| **轻微错误** | `-p user.err` | 失败，例如“无法处理某些文件”。 |
| **严重错误** | `-p user.crit` | 致命情况，例如“无法连接到据库”。 |
| **重要通知** | `-p user.notice` | 非错误的重记录，例如“配置已更改”。 |
| **调试输出** | `-p user.debug` | 脚本开发期间的详细输出，例如变量内容。 |

### 常用选项
- `-n [服务器名]`: 目标服务器 (必需)。
- `-p [优先级]`: `user.notice`, `local0.err` 等 (默认: `user.info`)。
- `-t [标签]`: 应用程序名称等 (默认: `rust-logger`)。

---

## 3. 实际应用示例

### 在批处理文件 (.bat) 中使用
用于脚本开始/结束时的通知或发生错误时的通知。
```batch
@echo off
set SERVER=192.0.2.10
rust-logger -n %SERVER% -t "BackupScript" "开始备份过程"

copy C:\data D:\backup
if %ERRORLEVEL% NEQ 0 (
    rust-logger -n %SERVER% -p local0.err -t "BackupScript" "错误: 备份失败"
) else (
    rust-logger -n %SERVER% -t "BackupScript" "备份完成"
)
```

### 在 PowerShell (.ps1) 中使用
在 PowerShell 中，建议在脚本开头插入下述“乱码对策”。
```powershell
$Logger = "rust-logger.exe"
$Server = "127.0.0.1"

# 乱码对策 (必需)
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

& $Logger -n $Server -t "PS-Script" "来自 PowerShell 的测试通知"
```

---

## 4. 故障排除：解决乱码 (Mojibake)

在 Windows 上处理日语（或多字节字符）时，请注意以下几点。

### 如果 PowerShell 中出现乱码
PowerShell 5.1 在将参数传递给外部命令时容易导致乱码。
1.  以 **UTF-8 with BOM** 保存脚本。
2.  在脚本中显式设置编码（请参见上面的 PowerShell 示例）。

### 如果命令提示符中出现乱码
即使输入是 Shift-JIS (CP932)，`rust-logger` 也会自动确定并处理输入，但如果显示混乱，请暂时将代码页更改为 UTF-8。
```cmd
chcp 65001
```

### 如果目标服务器出现乱码
如果目标 Syslog 服务器需要特定的编码（如 Shift-JIS），请使用 `--encoding` 选项。
```cmd
rust-logger -n 127.0.0.1 --encoding shift_jis "日语消息"
```

---

## 5. 优先级 (Level 和 Facility) 参考

| 级别 (Severity) | 描述 |
| :--- | :--- |
| `emerg`, `panic` | 系统不可用 (最高优先级) |
| `alert` | 必须立即采取行动 |
| `crit` | 关键状况 |
| `err`, `error` | 错误状况 |
| `warning`, `warn` | 警告状况 |
| `notice` | 正常但重要的状况 |
| `info` | 信息性消息 |
| `debug` | 调试级消息 |
