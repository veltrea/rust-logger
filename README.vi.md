# rust-logger

Một bản triển khai Rust của lệnh `logger` tiêu chuẩn UNIX, được thiết kế để sử dụng an toàn và đáng tin cậy trong môi trường Windows (Command Prompt / PowerShell). Nó tự động chuyển đổi đầu vào từ các bảng mã nội địa (Shift-JIS/UTF-16) sang UTF-8 và gửi đến máy chủ Syslog.

Công cụ này được phát triển để hoạt động liền mạch với **[vlt-syslogd](https://github.com/veltrea/vlt-syslogd)**, một máy chủ Syslog dành cho Windows hỗ trợ UTF-8 và hiển thị chính xác các ký tự đa ngôn ngữ.

---

### [English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md)

---

## Tính năng

- **Tương thích UNIX `logger`**: Hỗ trợ các tùy chọn tiêu chuẩn như `-n`, `-P`, và `-p`.
- **Hỗ trợ Windows đầy đủ**: Xử lý chính xác dữ liệu nhập từ `cmd.exe` (Shift-JIS) và `PowerShell` (UTF-16).
- **Binary duy nhất**: Chạy dưới dạng tệp thực thi độc lập không có phụ thuộc.

## Tài liệu

- **[Hướng dẫn sử dụng (MANUAL.md)](MANUAL.md)**: Chi tiết cách dùng, ví dụ cho Batch/PowerShell và hướng dẫn khắc phục lỗi bảng mã ký tự.

## Cài đặt

Tải xuống `rust-logger.exe` từ trang Releases và đặt nó vào một thư mục có trong PATH của hệ thống.

## Cách sử dụng

Cách sử dụng cơ bản giống hệt với lệnh `logger` của UNIX.

```powershell
# Cách dùng cơ bản (ưu tiên mặc định là user.info)
rust-logger -n 127.0.0.1 -P 514 "Nội dung thông báo nhật ký"

# Chỉ định mức ưu tiên và thẻ
rust-logger -n 192.0.2.10 -p local0.warn -t MyBatchApp "Cảnh báo: Tiến trình thất bại"
```

### Tùy chọn

| Tùy chọn | Viết tắt | Mô tả | Mặc định |
| :--- | :--- | :--- | :--- |
| `--server` | `-n` | **[Bắt buộc]** Tên máy chủ hoặc IP của máy chủ Syslog | - |
| `--port` | `-P` | Số cổng đích | 514 |
| `--priority` | `-p` | Mức ưu tiên theo định dạng `facility.level` | `user.info` |
| `--tag` | `-t` | Thẻ (định danh) | `rust-logger` |
| `--encoding` | - | Bảng mã đầu ra (`utf-8`, `shift_jis`, v.v.) | `utf-8` |
| `[MESSAGE]` | - | Thông báo nhật ký (ở cuối các đối số) | - |

## Ghi chú phát hành

### Bản sửa lỗi hỗ trợ đa ngôn ngữ RFC 5424 (07/02/2026)
Đã sửa các lỗi triển khai liên quan đến đặc tả RFC 5424:
- **Định dạng trường MSG**: Đã thêm BOM chính xác vào đầu trường `MSG` cho định dạng `MSG-UTF8`.
- **Tự động phát hiện bảng mã**: Cấu trúc gói được định nghĩa là `MSG-UTF8` (có BOM) cho UTF-8 và `MSG-ANY` (không có BOM) cho các bảng mã khác.
- **Tích hợp Windows**: Tinh chỉnh cấu trúc nội bộ để xử lý minh bạch đầu vào từ `cmd.exe` và `PowerShell` bằng cách sử dụng chuẩn hóa đối số của Rust.

## Giấy phép

[MIT License](LICENSE)
