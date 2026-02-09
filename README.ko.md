# rust-logger

UNIX 표준 `logger` 명령어를 Rust로 구현하여 Windows 환경(명령 프롬프트 / PowerShell)에서 안전하고 신뢰할 수 있게 사용할 수 있도록 제작된 CLI 도구입니다. 일본어 및 기타 현지 인코딩(Shift-JIS/UTF-16) 입력을 자동으로 UTF-8로 변환하여 Syslog 서버로 전송합니다.

이 도구는 UTF-8을 지원하고 다국어 문자를 올바르게 표시하는 Windows용 Syslog 서버인 **[vlt-syslogd](https://github.com/veltrea/vlt-syslogd)**와 함께 사용하도록 최적화되어 있습니다.

---

### [English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md)

---

## 주요 기능

- **UNIX `logger` 호환성**: `-n`, `-P`, `-p` 등 표준 옵션 지원.
- **Windows 완벽 지원**: `cmd.exe` (Shift-JIS) 및 `PowerShell` (UTF-16) 입력을 적절하게 처리.
- **단일 바이너리**: 종속성 없이 독립 실행 파일로 동작.

## 문서

- **[사용자 매뉴얼 (MANUAL.md)](MANUAL.md)**: 자세한 사용법, 배치/PowerShell 활용 예시 및 문자 인코딩 문제 해결 가이드.

## 설치 방법

Releases 페이지에서 `rust-logger.exe`를 다운로드하여 시스템 PATH에 포함된 디렉토리에 저장하세요.

## 사용법

기본적인 사용법은 UNIX의 `logger` 명령어와 동일합니다.

```powershell
# 기본 사용법 (기본 우선순위는 user.info)
rust-logger -n 127.0.0.1 -P 514 "로그 메시지 본문"

# 우선순위 및 태그 지정
rust-logger -n 192.0.2.10 -p local0.warn -t MyBatchApp "경고: 처리에 실패했습니다"
```

### 옵션

| 옵션 | 축약형 | 설명 | 기본값 |
| :--- | :--- | :--- | :--- |
| `--server` | `-n` | **[필수]** Syslog 서버 호스트명 또는 IP | - |
| `--port` | `-P` | 대상 포트 번호 | 514 |
| `--priority` | `-p` | 우선순위 (`facility.level` 형식) | `user.info` |
| `--tag` | `-t` | 태그 (식별자) | `rust-logger` |
| `--encoding` | - | 출력 인코ディング (`utf-8`, `shift_jis` 등) | `utf-8` |
| `[MESSAGE]` | - | 로그 메시지 (인수 끝에 위치) | - |

## 릴리스 노트

### RFC 5424 다국어 지원 수정 (2026/02/07)
RFC 5424 규격과 관련된 구현 버그를 수정했습니다:
- **MSG 필드 형식**: `MSG-UTF8` 형식을 위해 `MSG` 필드 시작 부분에 BOM을 올바르게 추가했습니다.
- **자동 인코딩 감지**: UTF-8은 `MSG-UTF8`(BOM 포함), 그 외(Shift-JIS 등)는 `MSG-ANY`(BOM 없음)로 패킷 구조를 구성하도록 대응했습니다.
- **Windows 통합**: Rust의 인수 정규화를 활용하여 `cmd.exe` 및 `PowerShell`로부터의 현지 입력을 투명하게 처리하도록 내부 구조를 개선했습니다.

## 라이선스

[MIT License](LICENSE)
