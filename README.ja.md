# rust-logger

UNIX 標準の `logger` コマンドを Rust で実装し、Windows 環境（コマンドプロンプト / PowerShell）で安全に利用できるようにした CLI ツールです。日本語環境（Shift-JIS/UTF-16）からの入力を自動的に UTF-8 に変換し、Syslog サーバーへ送信します。

本ツールは、同じく UTF-8 対応で日本語を正しく表示できる Windows 用 Syslog サーバー **[vlt-syslogd](https://github.com/veltrea/vlt-syslogd)** との併用を想定して開発されました。

---

### [English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md)

---

## 特徴

- **UNIX `logger` 完全互換（引数仕様）**: `-n`, `-P`, `-p` などの標準的なオプションをサポート。
- **Windows 完全対応**: `cmd.exe` (Shift-JIS) や `PowerShell` (UTF-16) からの入力を適切にハンドリング。
- **シングルバイナリ**: 依存関係不要で動作。

## ドキュメント

- **[ユーザーマニュアル (MANUAL.md)](MANUAL.md)**: 詳しい使い方、バッチや PowerShell での活用例、文字化け対策ガイド。

## インストール

Releases ページから `rust-logger.exe` をダウンロードしてパスの通った場所に配置してください。

## 使い方

基本的な使い方は UNIX の `logger` コマンドと同じです。

```powershell
# 基本的な使用方法 (デフォルトは user.info)
rust-logger -n 127.0.0.1 -P 514 "ログメッセージ本文"

# 優先度とタグを指定
rust-logger -n 192.0.2.10 -p local0.warn -t MyBatchApp "警告: 処理に失敗しました"
```

### オプション

| オプション | 短縮 | 説明 | デフォルト |
| :--- | :--- | :--- | :--- |
| `--server` | `-n` | **[必須]** Syslog サーバーのホスト名または IP | - |
| `--port` | `-P` | 送信先ポート番号 | 514 |
| `--priority` | `-p` | 優先度 (`facility.level` 形式) | `user.info` |
| `--tag` | `-t` | タグ（識別子） | `rust-logger` |
| `--encoding` | - | 出力エンコーディング (`utf-8`, `shift_jis` 等) | `utf-8` |
| `[MESSAGE]` | - | ログメッセージ（コマンドライン引数の末尾） | - |

## 更新履歴

### RFC 5424 日本語対応の修正（2026/02/07）
RFC 5424 仕様の解釈ミスによる実装バグを修正しました：
- **MSG フィールドのフォーマット修正**: 
  - 正しく `MSG` フィールドの先頭に BOM を付与し、`MSG-UTF8` フォーマットとして構成するように修正しました。
- **エンコーディングに応じた自動判別**:
  - `UTF-8` 送信時は `MSG-UTF8`（BOMあり）、それ以外（Shift-JIS 等）は `MSG-ANY`（BOMなし）としてパケットを構成するように対応しました。
- **Windows 環境の利便性向上**: Rust ランタイムによる引数の正規化を背景に、`cmd.exe` や `PowerShell` からの日本語入力を透過的に扱える内部構造を整理しました。

## ライセンス

[MIT License](LICENSE)
