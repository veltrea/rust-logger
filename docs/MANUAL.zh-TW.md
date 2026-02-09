# rust-logger 使用手冊

`rust-logger` 是一個在 Windows 上重現 UNIX 標準 `logger` 命令功能的工具，並針對日語環境進行了增強。它用於從 Windows 上的各種腳本和應用程式向 Syslog 伺服器發送訊息。

---

## 1. 什麼是 Syslog？
Syslog 是用於通過網路轉發日誌訊息的標準協定。
通常，它結合了發送端工具（如 `rust-logger`）和接收/存儲端伺服器（如 `vlt-syslogd`）來使用。

### 訊息的組成部分
除了正文之外，Syslog 訊息還包含以下資訊：
- **優先級 (Priority)**: 訊息的重要性。指定為 `facility.level` (例如: `user.info`)。
- **標籤 (Tag)**: 指示哪個應用程式生成了日誌的識別碼 (例如: `web-server`)。
- **時間戳**: 生成日誌的日期和時間。
- **主機名**: 發送日誌的機器名稱。

---

## 2. 基本用法
您只需將訊息添加到命令行的末尾即可發送。

### 最小執行
```cmd
rust-logger -n 127.0.0.1 "測試訊息"
```

### 優先級 (-p) 使用範例
根據情況指定適當的優先級對於在 Syslog 伺服器端啟用有效的過濾和通知設置非常重要。

| 情況 | 推薦命令範例 | 意圖 |
| :--- | :--- | :--- |
| **正常操作記錄** | `-p user.info` | 正常記錄，例如「已開始...」或「已完成...」。 |
| **需引起注意** | `-p user.warning` | 「發生重試」，「硬碟空間不足」等。 |
| **輕微錯誤** | `-p user.err` | 失敗，例如「無法處理某些檔案」。 |
| **嚴重錯誤** | `-p user.crit` | 致命情況，例如「無法連接到資料庫」。 |
| **重要通知** | `-p user.notice` | 非錯誤的重要記錄，例如「配置已更改」。 |
| **偵錯輸出** | `-p user.debug` | 腳本開發期間的詳細輸出，例如變數內容。 |

### 常用選項
- `-n [伺服器名]`: 目標伺服器 (必需)。
- `-p [優先級]`: `user.notice`, `local0.err` 等 (默認: `user.info`)。
- `-t [標籤]`: 應用程式名稱等 (默認: `rust-logger`)。

---

## 3. 實際應用範例

### 在批次處理檔案 (.bat) 中使用
用於腳本開始/結束時的通知或發生錯誤時的通知。
```batch
@echo off
set SERVER=192.0.2.10
rust-logger -n %SERVER% -t "BackupScript" "開始備份過程"

copy C:\data D:\backup
if %ERRORLEVEL% NEQ 0 (
    rust-logger -n %SERVER% -p local0.err -t "BackupScript" "錯誤: 備份失敗"
) else (
    rust-logger -n %SERVER% -t "BackupScript" "備份完成"
)
```

### 在 PowerShell (.ps1) 中使用
在 PowerShell 中，建議在腳本開頭插入下述「亂碼對策」。
```powershell
$Logger = "rust-logger.exe"
$Server = "127.0.0.1"

# 亂碼對策 (必需)
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

& $Logger -n $Server -t "PS-Script" "來自 PowerShell 的測試通知"
```

---

## 4. 故障排除：解決亂碼 (Mojibake)

在 Windows 上處理日語（或多字節字符）時，請注意以下幾點。

### 如果 PowerShell 中出現亂碼
PowerShell 5.1 在將參數傳遞給外部命令時容易導致亂碼。
1.  以 **UTF-8 with BOM** 保存腳本。
2.  在腳本中顯式設置編碼（請參見上面的 PowerShell 範例）。

### 如果命令提示字元中出現亂碼
即使輸入是 Shift-JIS (CP932)，`rust-logger` 也會自動確定並處理輸入，但如果顯示混亂，請暫時將代碼頁更改為 UTF-8。
```cmd
chcp 65001
```

### 如果目標伺服器出現亂碼
如果目標 Syslog 伺服器需要特定的編碼（如 Shift-JIS），請使用 `--encoding` 選項。
```cmd
rust-logger -n 127.0.0.1 --encoding shift_jis "日語訊息"
```

---

## 5. 優先級 (Level 和 Facility) 參考

| 級別 (Severity) | 描述 |
| :--- | :--- |
| `emerg`, `panic` | 系統不可用 (最高優先級) |
| `alert` | 必須立即採取行動 |
| `crit` | 關鍵狀況 |
| `err`, `error` | 錯誤狀況 |
| `warning`, `warn` | 警告狀況 |
| `notice` | 正常但重要的狀況 |
| `info` | 訊息性內容 |
| `debug` | 偵錯級內容 |
