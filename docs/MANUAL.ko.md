# rust-logger 사용자 매뉴얼

`rust-logger`는 UNIX 표준 `logger` 명령의 기능을 Windows에서 재현하고, 일본어 환경에 대한 지원을 강화한 도구입니다. Windows의 각종 스크립트나 애플리케이션에서 Syslog 서버로 메시지를 전송하는 데 사용됩니다.

---

## 1. Syslog란 무엇인가?
Syslog는 네트워크를 통해 로그 메시지를 전송하기 위한 표준 프로토콜입니다.
일반적으로 송신 측 도구(`rust-logger` 등)와 수신/저장 측 서버(`vlt-syslogd` 등)를 조합하여 사용합니다.

### 메시지의 구성 요소
본문 외에도 돌로그 메시지에는 다음 정보가 포함됩니다.
- **우선순위 (Priority)**: 메시지의 중요도. `facility.level` (예: `user.info`) 형식으로 지정합니다.
- **태그 (Tag)**: 로그를 생성한 애플리케이션을 나타내는 식별자 (예: `web-server`).
- **타임스탬프**: 로그가 생성된 날짜와 시간.
- **호스트 이름**: 로그를 전송한 머신의 이름.

---

## 2. 기본 사용법
명령행 끝에 메시지를 추가하기만 하면 전송할 수 있습니다.

### 최소 실행
```cmd
rust-logger -n 127.0.0.1 "테스트 메시지"
```

### 우선순위 (-p) 사용 예
Syslog 서버 측에서 효과적인 필터링 및 알림 설정을 활성화하려면 상황에 따라 적절한 우선순위를 지정하는 것이 중요합니다.

| 상황 | 권장 명령 예 | 의도 |
| :--- | :--- | :--- |
| **정상 작동 기록** | `-p user.info` | "...를 시작했습니다" 또는 "...를 완료했습니다"와 같은 정상 기록. |
| **주의 필요** | `-p user.warning` | "재시도 발생", "HDD 용량 부족" 등. |
| **경미한 오류** | `-p user.err` | "일부 파일을 처리할 수 없습니다"와 같은 실패. |
| **심각한 오류** | `-p user.crit` | "데이터베이스에 연결할 수 없습니다"와 같은 치명적인 상황. |
| **중요한 알림** | `-p user.notice` | "구성이 변경되었습니다"와 같이 오류는 아니지만 중요한 기록. |
| **디버그 출력** | `-p user.debug` | 스크립트 개발 중의 변수 내용 등 상세 출력. |

### 자주 사용하는 옵션
- `-n [서버 이름]`: 대상 서버 (필수).
- `-p [우선순위]`: `user.notice`, `local0.err` 등 (기본값: `user.info`).
- `-t [태그]`: 애플리케이션 이름 등 (기본값: `rust-logger`).

---

## 3. 실제 활용 예

### 배치 파일 (.bat)에서 사용
스크립트 시작/종료 시 또는 오류 발생 시 알림에 유용합니다.
```batch
@echo off
set SERVER=192.0.2.10
rust-logger -n %SERVER% -t "BackupScript" "백업 프로세스 시작"

copy C:\data D:\backup
if %ERRORLEVEL% NEQ 0 (
    rust-logger -n %SERVER% -p local0.err -t "BackupScript" "오류: 백업 실패"
) else (
    rust-logger -n %SERVER% -t "BackupScript" "백업 완료"
)
```

### PowerShell (.ps1)에서 사용
PowerShell에서는 스크립트 시작 부분에 아래의 "문자 깨짐 대책"을 삽입하는 것이 좋습니다.
```powershell
$Logger = "rust-logger.exe"
$Server = "127.0.0.1"

# 문자 깨짐 대책 (필수)
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

& $Logger -n $Server -t "PS-Script" "PowerShell에서의 테스트 알림"
```

---

## 4. 문제 해결: 문자 깨짐 (Mojibake) 해결

Windows에서 일본어(또는 멀티바이트 문자)를 다룰 때 다음 사항에 주의하십시오.

### PowerShell에서 문자가 깨지는 경우
PowerShell 5.1은 외부 명령에 인수를 전달할 때 문자가 깨지기 쉽습니다.
1.  스크립트를 **UTF-8 with BOM**으로 저장하십시오.
2.  스크립트 내에서 인코딩을 명시적으로 설정하십시오 (위의 PowerShell 예 참조).

### 명령 프롬프트에서 문자가 깨지는 경우
`rust-logger`는 입력이 Shift-JIS (CP932)인 경우에도 자동으로 판별하여 처리하지만, 표시가 흐트러지는 경우 일시적으로 코드 페이지를 UTF-8로 변경하십시오.
```cmd
chcp 65001
```

### 대상 서버에서 문자가 깨지는 경우
대상 Syslog 서버가 특정 인코딩 (예: Shift-JIS)을 기대하는 경우 `--encoding` 옵션을 사용하십시오.
```cmd
rust-logger -n 127.0.0.1 --encoding shift_jis "일본어 메시지"
```

---

## 5. 우선순위 (Level 및 Facility) 참조

| 레벨 (Severity) | 설명 |
| :--- | :--- |
| `emerg`, `panic` | 시스템 사용 불가 (최우선 순위) |
| `alert` | 즉시 조치를 취해야 함 |
| `crit` | 치명적인 상태 |
| `err`, `error` | 오류 상태 |
| `warning`, `warn` | 경고 상태 |
| `notice` | 정상적이지만 중요한 상태 |
| `info` | 정보 메시지 |
| `debug` | 디버그 수준 메시지 |
