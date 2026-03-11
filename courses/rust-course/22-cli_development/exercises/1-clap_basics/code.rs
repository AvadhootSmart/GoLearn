use clap::Parser;

/// Textio SMS CLI - Send messages from the command line
#[derive(Parser)]
#[command(name = "textio")]
#[command(version = "1.0")]
#[command(about = "Textio SMS CLI - Send messages from the command line")]
struct Cli {
    /// TODO: Add the phone number recipient (required, short: 't', long: "to")
    // Hint: This should be a required String with short and long flags
    
    /// TODO: Add the message content (required, short: 'm', long: "message")
    // Hint: This should be a required String with short and long flags
    
    /// TODO: Add API key (optional, reads from TEXTIO_API_KEY env var)
    // Hint: Use Option<String> and add env = "TEXTIO_API_KEY" to the arg attribute
    // Note: You'll need clap's env feature for this
    
    /// TODO: Add verbose mode flag (optional, short: 'v', long: "verbose")
    // Hint: This should be a bool flag
    
    /// TODO: Add timeout in seconds (optional, default: 30, long: "timeout")
    // Hint: Use u64 with default_value = "30"
}

fn main() {
    // TODO: Parse the CLI arguments
    // let cli = Cli::parse();
    
    // TODO: Implement the following logic:
    // 1. Check if API key is provided, print error if not
    // 2. Print "Sending SMS..." with the recipient and message
    // 3. If verbose mode is enabled, print additional details
    // 4. Print the timeout value
    
    println!("CLI not implemented yet!");
}

// HINTS:
// 1. For env variable support, add to Cargo.toml:
//    clap = { version = "4.4", features = ["derive", "env"] }
//
// 2. The full implementation should look like:
//    #[arg(short, long, env = "TEXTIO_API_KEY")]
//    api_key: Option<String>,
//
// 3. To check if api_key exists:
//    let api_key = cli.api_key.unwrap_or_else(|| {
//        eprintln!("Error: API key required. Set TEXTIO_API_KEY or use --api-key");
//        std::process::exit(1);
//    });
//
// 4. For verbose output:
//    if cli.verbose {
//        println!("Verbose mode enabled");
//        println!("Timeout: {}s", cli.timeout);
//    }
