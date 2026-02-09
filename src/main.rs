mod core_syslog;

use clap::Parser;
use core_syslog::{Facility, Severity, send_syslog};

/// Simple Syslog CLI tool for Windows (logger compatible)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Syslog server host (IP or hostname)
    #[arg(short = 'n', long = "server", required = true)]
    server: String,

    /// Syslog server port
    #[arg(short = 'P', long, default_value_t = 514)]
    port: u16,

    /// Priority (facility.level). Defaults to "user.info".
    #[arg(short = 'p', long, default_value = "user.info")]
    priority: String,

    /// Tag (App name)
    #[arg(short = 't', long, default_value = "rust-logger")]
    tag: String,

    /// Output encoding (utf-8, shift_jis, etc.). Defaults to "utf-8".
    #[arg(long, default_value = "utf-8")]
    encoding: String,

    /// Log message (positional arguments)
    #[arg(trailing_var_arg = true)]
    message_args: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Parse Priority (facility.level)
    let (facility, severity) = if args.priority.contains('.') {
        let parts: Vec<&str> = args.priority.splitn(2, '.').collect();
        (Facility::from_str(parts[0]), Severity::from_str(parts[1]))
    } else {
        (Facility::User, Severity::from_str(&args.priority))
    };

    // Construct Message Content
    let msg_content = if !args.message_args.is_empty() {
        args.message_args.join(" ")
    } else {
        return Err("No message provided. Please pass the message as arguments.".into());
    };

    let result = send_syslog(
        &args.server,
        args.port,
        facility,
        severity,
        &args.tag,
        &args.encoding,
        &msg_content,
    )?;

    println!("{}", result);

    Ok(())
}
