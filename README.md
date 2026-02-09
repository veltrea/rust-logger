# rust-logger

A Rust implementation of the UNIX standard `logger` command, designed for safe and reliable use in Windows environments (Command Prompt / PowerShell). It automatically converts input from localized encodings (Shift-JIS/UTF-16) to UTF-8 and sends it to a Syslog server.

This tool was developed to work seamlessly with **[vlt-syslogd](https://github.com/veltrea/vlt-syslogd)**, a Windows Syslog server that supports UTF-8 and correctly displays localized characters.

---

### [English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md)

---

## Features

- **UNIX `logger` Compatibility**: Supports standard options such as `-n`, `-P`, and `-p`.
- **Full Windows Support**: Properly handles inputs from `cmd.exe` (Shift-JIS) and `PowerShell` (UTF-16).
- **Single Binary**: Runs as a standalone executable with no dependencies.

## Documentation

- **[User Manual (MANUAL.md)](MANUAL.md)**: Detailed usage, examples for Batch/PowerShell, and troubleshooting guide for character encoding.

## Installation

Download `rust-logger.exe` from the Releases page and place it in a directory included in your system's PATH.

## Usage

Basic usage is identical to the UNIX `logger` command.

```powershell
# Basic usage (default priority is user.info)
rust-logger -n 127.0.0.1 -P 514 "Log message body"

# Specifying priority and tag
rust-logger -n 192.0.2.10 -p local0.warn -t MyBatchApp "Warning: Process failed"
```

### Options

| Option | Shorthand | Description | Default |
| :--- | :--- | :--- | :--- |
| `--server` | `-n` | **[Required]** Syslog server hostname or IP | - |
| `--port` | `-P` | Destination port number | 514 |
| `--priority` | `-p` | Priority in `facility.level` format | `user.info` |
| `--tag` | `-t` | Tag (identifier) | `rust-logger` |
| `--encoding` | - | Output encoding (`utf-8`, `shift_jis`, etc.) | `utf-8` |
| `[MESSAGE]` | - | Log message (at the end of arguments) | - |

## Release Notes

### RFC 5424 Japanese Support Fix (2026/02/07)
Fixed implementation bugs related to RFC 5424 specification:
- **MSG Field Format**: Correctly added BOM to the start of the `MSG` field for `MSG-UTF8` format (previously erroneously added to the start of the packet).
- **Automatic Encoding Detection**: Configured packet structure as `MSG-UTF8` (with BOM) for UTF-8 and `MSG-ANY` (no BOM) for others (e.g., Shift-JIS).
- **Windows Integration**: Refined internal structure to transparently handle localized input from `cmd.exe` and `PowerShell` using Rust's argument normalization.

## License

[MIT License](LICENSE)

```text
Copyright (c) 2026 veltrea

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
