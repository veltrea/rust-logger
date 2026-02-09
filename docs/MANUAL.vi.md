# Hướng dẫn sử dụng rust-logger

`rust-logger` là một công cụ tái tạo chức năng của lệnh UNIX tiêu chuẩn `logger` trên Windows, với sự hỗ trợ nâng cao cho môi trường tiếng Nhật. Nó được sử dụng để gửi tin nhắn đến máy chủ Syslog từ các tập lệnh và ứng dụng khác nhau trên Windows.

---

## 1. Syslog là gì?
Syslog là một giao thức tiêu chuẩn để chuyển tiếp tin nhắn nhật ký qua mạng.
Thông thường, nó liên quan đến sự kết hợp của một công cụ gửi (như `rust-logger`) và một máy chủ nhận/lưu trữ (như `vlt-syslogd`).

### Các thành phần của tin nhắn
Ngoài nội dung chính, một tin nhắn Syslog còn bao gồm các thông tin sau:
- **Mức ưu tiên (Priority)**: Mức độ quan trọng của tin nhắn. Được chỉ định dưới dạng `facility.level` (ví dụ: `user.info`).
- **Thẻ (Tag)**: Một định danh cho biết ứng dụng nào đã tạo ra nhật ký (ví dụ: `web-server`).
- **Dấu thời gian (Timestamp)**: Ngày và giờ nhật ký được tạo.
- **Tên máy chủ (Hostname)**: Tên của máy đã gửi nhật ký.

---

## 2. Cách sử dụng cơ bản
Bạn có thể gửi tin nhắn đơn giản bằng cách thêm nó vào cuối dòng lệnh.

### Thực thi tối thiểu
```cmd
rust-logger -n 127.0.0.1 "Tin nhắn thử nghiệm"
```

### Các ví dụ về sử dụng mức ưu tiên (-p)
Điều quan trọng là phải chỉ định mức ưu tiên phù hợp tùy theo tình huống để cho phép cài đặt lọc và thông báo hiệu quả ở phía máy chủ Syslog.

| Tình huống | Ví dụ lệnh được khuyến nghị | Mục đích |
| :--- | :--- | :--- |
| **Ghi lại hoạt động bình thường** | `-p user.info` | Các bản ghi bình thường như "Đã bắt đầu..." hoặc "Đã hoàn thành...". |
| **Cần chú ý** | `-p user.warning` | "Đã xảy ra thử lại", "Dung lượng ổ cứng thấp", v.v. |
| **Lỗi nhỏ** | `-p user.err` | Các lỗi thất bại như "Không thể xử lý một số tệp". |
| **Lỗi nghiêm trọng** | `-p user.crit` | Các tình huống chí tử như "Không thể kết nối với cơ sở dữ liệu". |
| **Thông báo quan trọng** | `-p user.notice` | Các bản ghi quan trọng không phải là lỗi, như "Cấu hình đã thay đổi". |
| **Đầu ra gỡ lỗi** | `-p user.debug` | Đầu ra chi tiết như nội dung biến trong quá trình phát triển tập lệnh. |

### Các tùy chọn thường dùng
- `-n [Tên máy chủ]`: Máy chủ đích (Bắt buộc).
- `-p [Mức ưu tiên]`: `user.notice`, `local0.err`, v.v. (Mặc định: `user.info`).
- `-t [Thẻ]`: Tên ứng dụng, v.v. (Mặc định: `rust-logger`).

---

## 3. Các ví dụ thực tế

### Sử dụng trong Tập lệnh Batch (.bat)
Hữu ích cho các thông báo khi bắt đầu/kết thúc tập lệnh hoặc khi xảy ra lỗi.
```batch
@echo off
set SERVER=192.0.2.10
rust-logger -n %SERVER% -t "BackupScript" "Bắt đầu quy trình sao lưu"

copy C:\data D:\backup
if %ERRORLEVEL% NEQ 0 (
    rust-logger -n %SERVER% -p local0.err -t "BackupScript" "Lỗi: Sao lưu thất bại"
) else (
    rust-logger -n %SERVER% -t "BackupScript" "Sao lưu hoàn tất"
)
```

### Sử dụng trong PowerShell (.ps1)
Trong PowerShell, nên chèn "Biện pháp đối phó Mojibake" được mô tả bên dưới vào đầu tập lệnh.
```powershell
$Logger = "rust-logger.exe"
$Server = "127.0.0.1"

# Biện pháp đối phó Mojibake (Bắt buộc)
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

& $Logger -n $Server -t "PS-Script" "Thông báo thử nghiệm từ PowerShell"
```

---

## 4. Khắc phục sự cố: Sửa lỗi Mojibake (Ký tự bị lỗi)

Khi xử lý tiếng Nhật (hoặc các ký tự đa byte) trên Windows, vui lòng lưu ý các điểm sau.

### Nếu Mojibake xảy ra trong PowerShell
PowerShell 5.1 có xu hướng gây ra lỗi ký tự khi truyền tham số cho các lệnh bên ngoài.
1.  Lưu tập lệnh với **UTF-8 with BOM**.
2.  Thiết lập mã hóa rõ ràng trong tập lệnh (xem ví dụ PowerShell ở trên).

### Nếu Mojibake xảy ra trong Command Prompt
`rust-logger` tự động xác định và xử lý đầu vào ngay cả khi nó là Shift-JIS (CP932), nhưng nếu màn hình hiển thị bị rối, hãy tạm thời thay đổi bảng mã sang UTF-8.
```cmd
chcp 65001
```

### Nếu Mojibake xảy ra tại Điểm đích
Nếu máy chủ Syslog đích mong đợi một mã hóa cụ thể (như Shift-JIS), hãy sử dụng tùy chọn `--encoding`.
```cmd
rust-logger -n 127.0.0.1 --encoding shift_jis "Tin nhắn tiếng Nhật"
```

---

## 5. Tham khảo Mức ưu tiên (Level và Facility)

| Mức độ (Severity) | Mô tả |
| :--- | :--- |
| `emerg`, `panic` | Hệ thống không thể sử dụng (Mức ưu tiên cao nhất) |
| `alert` | Phải thực hiện hành động ngay lập tức |
| `crit` | Các điều kiện chí tử |
| `err`, `error` | Các điều kiện lỗi |
| `warning`, `warn` | Các điều kiện cảnh báo |
| `notice` | Điều kiện bình thường nhưng quan trọng |
| `info` | Tin nhắn thông tin |
| `debug` | Tin nhắn cấp độ gỡ lỗi |
