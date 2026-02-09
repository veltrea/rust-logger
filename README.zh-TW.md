# rust-logger

這是一個用 Rust 編寫的 UNIX 標準 `logger` 指令的實現，專為 Windows 環境（命令提示字元 / PowerShell）下的安全和可靠使用而設計。它能自動將來自本地化編碼（如 Shift-JIS / UTF-16）的輸入轉換為 UTF-8，並發送到 Syslog 伺服器。

本工具旨在與 **[vlt-syslogd](https://github.com/veltrea/vlt-syslogd)** 完美配合使用，後者是一款支援 UTF-8 並能正確顯示多語言字元的 Windows Syslog 伺服器。

---

### [English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md)

---

## 特性

- **UNIX `logger` 相容性**: 支援 `-n`、`-P` 和 `-p` 等標準選項。
- **完善的 Windows 支援**: 能夠妥善處理來自 `cmd.exe` (Shift-JIS) 和 `PowerShell` (UTF-16) 的輸入。
- **單個執行檔**: 作為獨立的可執行文件運行，無需任何依賴。

## 文件

- **[使用手冊 (MANUAL.md)](MANUAL.md)**: 詳細的使用說明、批次檔/PowerShell 的應用實例以及字元編碼疑難排解指南。

## 安裝

從 Releases 頁面下載 `rust-logger.exe`，並將其放置在系統的 PATH 環境變數所包含的目錄中。

## 用法

基本用法與 UNIX 的 `logger` 指令完全相同。

```powershell
# 基本用法 (預設優先等級為 user.info)
rust-logger -n 127.0.0.1 -P 514 "日誌訊息內容"

# 指定優先等級和標籤
rust-logger -n 192.0.2.10 -p local0.warn -t MyBatchApp "警告：處理失敗"
```

### 選項

| 選項 | 簡寫 | 說明 | 預設值 |
| :--- | :--- | :--- | :--- |
| `--server` | `-n` | **[必填]** Syslog 伺服器的主機名稱或 IP | - |
| `--port` | `-P` | 目標連接埠號碼 | 514 |
| `--priority` | `-p` | 優先等級 (格式為 `facility.level`) | `user.info` |
| `--tag` | `-t` | 標籤 (識別碼) | `rust-logger` |
| `--encoding` | - | 輸出編碼 (`utf-8`、`shift_jis` 等) | `utf-8` |
| `[MESSAGE]` | - | 日誌訊息 (位於參數末尾) | - |

## 發行說明

### RFC 5424 多語言支援修復 (2026/02/07)
修復了與 RFC 5424 規範相關的實現錯誤：
- **MSG 欄位格式**: 正確地在 `MSG` 欄位開頭添加了 BOM 以符合 `MSG-UTF8` 格式。
- **自動編碼檢測**: 對於 UTF-8 配置為 `MSG-UTF8`（含 BOM），對於其他編碼（如 Shift-JIS）配置為 `MSG-ANY`（無 BOM）。
- **Windows 整合**: 優化了內部結構，利用 Rust 的參數規範化透明地處理來自 `cmd.exe` 和 `PowerShell` 的本地化輸入。

## 授權條款

[MIT License](LICENSE)
