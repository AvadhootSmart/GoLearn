use clap::Parser;

/// Textio SMS CLI - Send messages from the command line
#[derive(Parser)]
#[command(name = "textio")]
#[command(version = "1.0")]
#[command(about = "Textio SMS CLI - Send messages from the command line")]
struct Cli {
    /// The phone number recipient (e.g., +1234567890)
    #[arg(short, long)]
    to: String,

    /// The message content to send
    #[arg(short, long)]
    message: String,

    /// API key for authentication (or set TEXTIO_API_KEY env var)
    #[arg(short, long, env = "TEXTIO_API_KEY")]
    api_key: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Request timeout in seconds
    #[arg(long, default_value = "30")]
    timeout: u64,
}

fn main() {
    let cli = Cli::parse();

    let api_key = cli.api_key.unwrap_or_else(|| {
        eprintln!("Error: API key required. Set TEXTIO_API_KEY environment variable or use --api-key");
        std::process::exit(1);
    });

    println!("Sending SMS...");
    println!("To: {}", cli.to);
    println!("Message: {}", cli.message);

    if cli.verbose {
        println!("---");
        println!("Verbose mode enabled");
        println!("API Key: {}...", &api_key[..8.min(api_key.len())]);
        println!("Timeout: {}s", cli.timeout);
    }

    println!("---");
    println!("Message sent successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::try_parse_from([
            "textio",
            "--to", "+1234567890",
            "--message", "Hello",
            "--api-key", "secret123",
        ]);
        
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        assert_eq!(cli.to, "+1234567890");
        assert_eq!(cli.message, "Hello");
        assert_eq!(cli.api_key, Some("secret123".to_string()));
        assert_eq!(cli.timeout, 30);
    }

    #[test]
    fn test_cli_with_verbose() {
        let cli = Cli::try_parse_from([
            "textio",
            "-t", "+1234567890",
            "-m", "Test",
            "-k", "key",
            "-v",
            "--timeout", "60",
        ]);
        
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        assert!(cli.verbose);
        assert_eq!(cli.timeout, 60);
    }
}
