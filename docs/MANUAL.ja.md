# rust-logger ユーザーマニュアル

`rust-logger` は、UNIX 標準の `logger` コマンドの機能を Windows で再現し、さらに日本語環境への対応を強化したツールです。Windows 上の各種スクリプトやアプリケーションから Syslog サーバーへメッセージを送信するために使用します。

---

## 1. Syslog とは？
Syslog は、ネットワーク経由でログメッセージを転送するための標準的なプロトコルです。
通常、送信側のツール（`rust-logger` など）と、受信・保存側のサーバー（`vlt-syslogd` など）を組み合わせて利用します。

### メッセージの構成要素
Syslog メッセージには、本文のほかに以下の情報が含まれます。
- **優先度 (Priority)**: メッセージの重要度。`ファシリティ.レベル`（例: `user.info`）で指定します。
- **タグ (Tag)**: どのアプリケーションがログを出したかを示す識別子（例: `web-server`）。
- **タイムスタンプ**: ログが発生した日時。
- **ホスト名**: ログを送信したマシンの名前。

---

## 2. 基本的な記述方法
コマンドラインの末尾にメッセージを記述するだけで送信できます。

### 最小構成での実行
```cmd
rust-logger -n 127.0.0.1 "テストメッセージ"
```

### 優先度 (-p) の使い分け例
Syslog サーバー側でのフィルタリングや通知設定を活かすために、状況に応じて適切な優先度を指定することが重要です。

| 状況 | 推奨コマンド例 | 意図 |
| :--- | :--- | :--- |
| **通常の動作記録** | `-p user.info` | 「～を開始しました」「～を完了しました」などの通常記録。 |
| **注意喚起** | `-p user.warning` | 「リトライが発生しました」「HDD残量が少なめです」など。 |
| **軽微なエラー** | `-p user.err` | 「一部のファイルが処理できませんでした」などの失敗。 |
| **深刻なエラー** | `-p user.crit` | 「データベースに接続できません」などの致命的状況。 |
| **重要な通知** | `-p user.notice` | 「設定が変更されました」など、エラーではないが重要な記録。 |
| **デバッグ出力** | `-p user.debug` | スクリプト開発中の変数の中身などの詳細出力。 |

### よく使うオプション
- `-n [サーバー名]`: 送信先サーバー（必須）。
- `-p [優先度]`: `user.notice`, `local0.err` など（デフォルト: `user.info`）。
- `-t [タグ]`: アプリケーション名など（デフォルト: `rust-logger`）。

---

## 3. 実用的な活用例

### バッチファイル (.bat) での使用
スクリプトの開始や終了、エラー発生時の通知に便利です。
```batch
@echo off
set SERVER=192.0.2.10
rust-logger -n %SERVER% -t "BackupScript" "バックアップ処理を開始します"

copy C:\data D:\backup
if %ERRORLEVEL% NEQ 0 (
    rust-logger -n %SERVER% -p local0.err -t "BackupScript" "エラー: バックアップに失敗しました"
) else (
    rust-logger -n %SERVER% -t "BackupScript" "バックアップが完了しました"
)
```

### PowerShell (.ps1) での使用
PowerShell では、後述の「文字化け対策」をスクリプト冒頭に入れることを推奨します。
```powershell
$Logger = "rust-logger.exe"
$Server = "127.0.0.1"

# 文字化け対策（必須）
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

& $Logger -n $Server -t "PS-Script" "PowerShell からの通知テスト"
```

---

## 4. トラブルシューティング：文字化けの解決

Windows 環境で日本語を扱う場合、以下の点に注意してください。

### PowerShell で文字化けする場合
PowerShell 5.1 は、外部コマンドへ引数を渡す際に文字化けを起こしやすい仕様です。
1.  **UTF-8 with BOM** でスクリプトを保存してください。
2.  スクリプト内でエンコーディング設定を明示してください（上記の PowerShell 例を参照）。

### コマンドプロンプトで文字化けする場合
`rust-logger` は、入力が Shift-JIS (CP932) であっても自動的に判別して処理しますが、表示が乱れる場合は一時的にコードページを UTF-8 に変更してください。
```cmd
chcp 65001
```

### 送信先で文字化けする場合
送信先 Syslog サーバーが特別なエンコーディング（Shift-JIS など）を期待している場合は、`--encoding` オプションを使用してください。
```cmd
rust-logger -n 127.0.0.1 --encoding shift_jis "日本語メッセージ"
```

---

## 5. 優先度（レベルとファシリティ）リファレンス

| レベル (Severity) | 内容 |
| :--- | :--- |
| `emerg`, `panic` | システム使用不可（最優先） |
| `alert` | 即時の対応が必要 |
| `crit` | 致命的なエラー |
| `err`, `error` | 一般的なエラー |
| `warning`, `warn` | 警告 |
| `notice` | 通常だが重要な通知 |
| `info` | 情報 |
| `debug` | デバッグ情報 |
