use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Read};
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

/// Textio SMS CLI with professional patterns
#[derive(Parser)]
#[command(name = "textio")]
#[command(version = "1.0")]
struct Cli {
    /// Recipient phone number
    #[arg(short, long)]
    to: Option<String>,

    /// Message content (or read from stdin with --stdin)
    #[arg(short, long)]
    message: Option<String>,

    /// Read message from stdin
    #[arg(long)]
    stdin: bool,

    /// Output format: text or json
    #[arg(long, default_value = "text")]
    format: String,

    /// Show progress bar
    #[arg(long)]
    progress: bool,

    /// Simulate an error for testing exit codes
    #[arg(long)]
    error: Option<String>,

    /// Number of messages to simulate sending
    #[arg(short, long, default_value = "1")]
    count: u32,
}

#[derive(Debug)]
enum TextioError {
    NetworkError(String),
    InvalidInput(String),
    AuthFailed(String),
    RateLimited(String),
    InternalError(String),
}

impl std::fmt::Display for TextioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextioError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            TextioError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            TextioError::AuthFailed(msg) => write!(f, "Authentication failed: {}", msg),
            TextioError::RateLimited(msg) => write!(f, "Rate limited: {}", msg),
            TextioError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl From<TextioError> for ExitCode {
    fn from(error: TextioError) -> Self {
        let code = match error {
            TextioError::NetworkError(_) => 10,
            TextioError::InvalidInput(_) => 20,
            TextioError::AuthFailed(_) => 30,
            TextioError::RateLimited(_) => 40,
            TextioError::InternalError(_) => 50,
        };
        ExitCode::from(code)
    }
}

fn print_success(msg: &str) {
    println!("{}", msg.green().bold());
}

fn print_error(msg: &str) {
    eprintln!("{} {}", "Error:".red().bold(), msg);
}

fn print_warning(msg: &str) {
    println!("{} {}", "Warning:".yellow().bold(), msg);
}

fn print_info(msg: &str) {
    println!("{}", msg.cyan());
}

fn read_from_stdin() -> Result<String, TextioError> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| TextioError::InvalidInput(format!("Failed to read stdin: {}", e)))?;
    
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        return Err(TextioError::InvalidInput(
            "Empty message from stdin".to_string(),
        ));
    }
    
    Ok(trimmed)
}

fn create_progress_bar(len: u64) -> ProgressBar {
    let bar = ProgressBar::new(len);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    bar
}

fn show_progress(steps: u64) {
    let bar = create_progress_bar(steps);
    
    for _ in 0..steps {
        bar.inc(1);
        thread::sleep(Duration::from_millis(100));
    }
    
    bar.finish_with_message("Done!");
}

#[derive(serde::Serialize)]
struct MessageResult {
    to: String,
    message: String,
    status: String,
    message_id: String,
}

fn format_as_json(results: &[MessageResult]) -> String {
    serde_json::to_string_pretty(results).unwrap_or_else(|_| "{}".to_string())
}

fn format_as_text(results: &[MessageResult]) -> String {
    results
        .iter()
        .map(|r| {
            format!(
                "To: {}\nMessage: {}\nStatus: {}\nID: {}",
                r.to, r.message, r.status, r.message_id
            )
        })
        .collect::<Vec<_>>()
        .join("\n---\n")
}

fn run(cli: Cli) -> Result<(), TextioError> {
    if let Some(error_type) = &cli.error {
        return match error_type.as_str() {
            "network" => Err(TextioError::NetworkError(
                "Unable to connect to API server".to_string(),
            )),
            "auth" => Err(TextioError::AuthFailed(
                "Invalid API key".to_string(),
            )),
            "rate" => Err(TextioError::RateLimited(
                "Too many requests. Try again in 60 seconds.".to_string(),
            )),
            "input" => Err(TextioError::InvalidInput(
                "Invalid phone number format".to_string(),
            )),
            _ => Err(TextioError::InternalError(
                "Unknown error type".to_string(),
            )),
        };
    }

    let message = if cli.stdin {
        print_info("Reading message from stdin...");
        read_from_stdin()?
    } else {
        cli.message
            .clone()
            .ok_or_else(|| TextioError::InvalidInput("Message required (--message or --stdin)".to_string()))?
    };

    let to = cli
        .to
        .clone()
        .ok_or_else(|| TextioError::InvalidInput("Recipient required (--to)".to_string()))?;

    if message.len() > 160 {
        print_warning(&format!(
            "Message is {} characters (max 160). It will be truncated.",
            message.len()
        ));
    }

    let count = cli.count;
    
    if cli.progress && count > 0 {
        print_info(&format!("Sending {} message(s)...", count));
        show_progress(count as u64);
    }

    let results: Vec<MessageResult> = (0..count)
        .map(|i| {
            let msg_suffix = if count > 1 {
                format!(" ({}/{})", i + 1, count)
            } else {
                String::new()
            };
            
            MessageResult {
                to: to.clone(),
                message: if message.len() > 160 {
                    format!("{}{}", &message[..160], msg_suffix)
                } else {
                    format!("{}{}", message, msg_suffix)
                },
                status: "sent".to_string(),
                message_id: format!("msg_{}", uuid::Uuid::new_v4()),
            }
        })
        .collect();

    let output = match cli.format.as_str() {
        "json" => format_as_json(&results),
        _ => format_as_text(&results),
    };

    println!();
    println!("{}", output);
    println!();

    Ok(())
}

fn main() -> ExitCode {
    let use_color = io::stdout().is_terminal();
    colored::control::set_override(use_color);

    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => {
            print_success("✓ Operation completed successfully!");
            ExitCode::SUCCESS
        }
        Err(e) => {
            print_error(&e.to_string());
            e.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_to_exit_code() {
        assert_eq!(
            Into::<u8>::into(ExitCode::from(TextioError::NetworkError("test".into()))),
            10
        );
        assert_eq!(
            Into::<u8>::into(ExitCode::from(TextioError::InvalidInput("test".into()))),
            20
        );
        assert_eq!(
            Into::<u8>::into(ExitCode::from(TextioError::AuthFailed("test".into()))),
            30
        );
    }

    #[test]
    fn test_json_format() {
        let results = vec![MessageResult {
            to: "+1234567890".to_string(),
            message: "Hello".to_string(),
            status: "sent".to_string(),
            message_id: "msg_123".to_string(),
        }];

        let json = format_as_json(&results);
        assert!(json.contains("+1234567890"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_text_format() {
        let results = vec![MessageResult {
            to: "+1234567890".to_string(),
            message: "Hello".to_string(),
            status: "sent".to_string(),
            message_id: "msg_123".to_string(),
        }];

        let text = format_as_text(&results);
        assert!(text.contains("To: +1234567890"));
        assert!(text.contains("Message: Hello"));
    }
}
