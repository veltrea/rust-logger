use chrono::Local;
use encoding_rs::Encoding;
use std::net::UdpSocket;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

impl Severity {
    pub fn from_str(s: &str) -> Severity {
        match s.to_lowercase().as_str() {
            "emerg" | "panic" => Severity::Emergency,
            "alert" => Severity::Alert,
            "crit" | "critical" => Severity::Critical,
            "err" | "error" => Severity::Error,
            "warn" | "warning" => Severity::Warning,
            "notice" => Severity::Notice,
            "info" | "information" => Severity::Info,
            "debug" => Severity::Debug,
            _ => Severity::Info,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Facility {
    Kern = 0,
    User = 1,
    Mail = 2,
    Auth = 4,
    Syslog = 5,
    Lpr = 6,
    News = 7,
    Uucp = 8,
    Authpriv = 10,
    Ftp = 11,
    Local0 = 16,
    Local1 = 17,
    Local2 = 18,
    Local3 = 19,
    Local4 = 20,
    Local5 = 21,
    Local6 = 22,
    Local7 = 23,
}

impl Facility {
    pub fn from_str(s: &str) -> Facility {
        match s.to_lowercase().as_str() {
            "kern" => Facility::Kern,
            "user" => Facility::User,
            "mail" => Facility::Mail,
            "auth" => Facility::Auth,
            "syslog" => Facility::Syslog,
            "lpr" => Facility::Lpr,
            "news" => Facility::News,
            "uucp" => Facility::Uucp,
            "authpriv" => Facility::Authpriv,
            "ftp" => Facility::Ftp,
            "local0" => Facility::Local0,
            "local1" => Facility::Local1,
            "local2" => Facility::Local2,
            "local3" => Facility::Local3,
            "local4" => Facility::Local4,
            "local5" => Facility::Local5,
            "local6" => Facility::Local6,
            "local7" => Facility::Local7,
            _ => Facility::Local0,
        }
    }
}

pub fn send_syslog(
    server: &str,
    port: u16,
    facility: Facility,
    severity: Severity,
    tag: &str,
    encoding_label: &str,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let pri = (facility as u8) * 8 + (severity as u8);

    // Get Hostname
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string());

    // Timestamp (RFC 3164)
    let timestamp = Local::now().format("%b %e %H:%M:%S").to_string();

    // Construct Packet
    let msg_field = if encoding_label.to_lowercase() == "utf-8" {
        format!("\u{FEFF}{}", message) // MSG-UTF8 format (BOM + UTF-8)
    } else {
        message.to_string() // MSG-ANY format
    };

    let packet = format!("<{}>{} {} {}: {}", pri, timestamp, hostname, tag, msg_field);

    // Encode Packet
    let encoding = Encoding::for_label(encoding_label.as_bytes())
        .ok_or_else(|| format!("Invalid encoding: {}", encoding_label))?;

    let (encoded_cow, _, had_errors) = encoding.encode(&packet);
    if had_errors {
        return Err(format!(
            "Encoding errors occurred when converting to {}",
            encoding_label
        )
        .into());
    }

    // Send UDP
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let target = format!("{}:{}", server, port);

    socket.send_to(&encoded_cow, &target)?;

    Ok(format!(
        "Success: Sent to {} (Encoding: {})",
        target,
        encoding.name()
    ))
}
