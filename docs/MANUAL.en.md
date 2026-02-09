# rust-logger User Manual

`rust-logger` is a tool that reproduces the standard UNIX `logger` command functionality on Windows, with enhanced support for Japanese environments. It is used to send messages to a Syslog server from various scripts and applications on Windows.

---

## 1. What is Syslog?
Syslog is a standard protocol for forwarding log messages over a network.
Typically, it involves a combination of a sender tool (like `rust-logger`) and a receiver/storage server (like `vlt-syslogd`).

### Message Components
In addition to the body text, a Syslog message includes the following information:
- **Priority**: The importance of the message. Specified as `facility.level` (e.g., `user.info`).
- **Tag**: An identifier indicating which application generated the log (e.g., `web-server`).
- **Timestamp**: The date and time the log was generated.
- **Hostname**: The name of the machine that sent the log.

---

## 2. Basic Usage
You can send a message simply by appending it to the end of the command line.

### Minimal Execution
```cmd
rust-logger -n 127.0.0.1 "Test message"
```

### Examples of Using Priority (-p)
It is important to specify the appropriate priority according to the situation to enable effective filtering and notification settings on the Syslog server side.

| Situation | Recommended Command Example | Intent |
| :--- | :--- | :--- |
| **Normal Operation Record** | `-p user.info` | Normal records like "Started..." or "Completed...". |
| **Attention Required** | `-p user.warning` | "Retry occurred", "HDD space is low", etc. |
| **Minor Error** | `-p user.err` | Failures like "Could not process some files". |
| **Critical Error** | `-p user.crit` | Fatal situations like "Cannot connect to database". |
| **Important Notice** | `-p user.notice` | Important records that are not errors, like "Configuration changed". |
| **Debug Output** | `-p user.debug` | Detailed output like variable contents during script development. |

### Common Options
- `-n [Server Name]`: Destination server (Required).
- `-p [Priority]`: `user.notice`, `local0.err`, etc. (Default: `user.info`).
- `-t [Tag]`: Application name, etc. (Default: `rust-logger`).

---

## 3. Practical Examples

### Usage in Batch Files (.bat)
Useful for notifications at the start/end of scripts or when errors occur.
```batch
@echo off
set SERVER=192.0.2.10
rust-logger -n %SERVER% -t "BackupScript" "Starting backup process"

copy C:\data D:\backup
if %ERRORLEVEL% NEQ 0 (
    rust-logger -n %SERVER% -p local0.err -t "BackupScript" "Error: Backup failed"
) else (
    rust-logger -n %SERVER% -t "BackupScript" "Backup completed"
)
```

### Usage in PowerShell (.ps1)
In PowerShell, it is recommended to insert the "Mojibake Countermeasure" described below at the beginning of the script.
```powershell
$Logger = "rust-logger.exe"
$Server = "127.0.0.1"

# Mojibake Countermeasure (Required)
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

& $Logger -n $Server -t "PS-Script" "Test notification from PowerShell"
```

---

## 4. Troubleshooting: Fixing Mojibake (Garbled Text)

When handling Japanese on Windows, please note the following points.

### If Mojibake occurs in PowerShell
PowerShell 5.1 tends to cause mojibake when passing arguments to external commands.
1.  Save the script with **UTF-8 with BOM**.
2.  Explicitly set the encoding within the script (see the PowerShell example above).

### If Mojibake occurs in Command Prompt
`rust-logger` automatically determines and processes input even if it is Shift-JIS (CP932), but if the display is disordered, temporarily change the code page to UTF-8.
```cmd
chcp 65001
```

### If Mojibake occurs at the Destination
If the destination Syslog server expects a specific encoding (like Shift-JIS), use the `--encoding` option.
```cmd
rust-logger -n 127.0.0.1 --encoding shift_jis "Japanese Message"
```

---

## 5. Priority (Level and Facility) Reference

| Level (Severity) | Description |
| :--- | :--- |
| `emerg`, `panic` | System is unusable (Highest priority) |
| `alert` | Action must be taken immediately |
| `crit` | Critical conditions |
| `err`, `error` | Error conditions |
| `warning`, `warn` | Warning conditions |
| `notice` | Normal but significant condition |
| `info` | Informational messages |
| `debug` | Debug-level messages |
