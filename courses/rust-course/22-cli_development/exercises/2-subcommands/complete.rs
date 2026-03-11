use clap::{Parser, Subcommand};

/// Textio SMS CLI - Manage your SMS communications
#[derive(Parser)]
#[command(name = "textio")]
#[command(version = "1.0")]
struct Cli {
    /// API key for authentication
    #[arg(short, long, global = true)]
    api_key: Option<String>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send an SMS message
    Send {
        /// Recipient phone number
        #[arg(short, long)]
        to: String,

        /// Message content
        #[arg(short, long)]
        message: String,
    },

    /// View message history
    History {
        /// Maximum number of messages to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Manage contacts
    #[command(subcommand)]
    Contact(ContactCommands),
}

#[derive(Subcommand)]
enum ContactCommands {
    /// Add a new contact
    Add {
        /// Contact name
        name: String,

        /// Phone number
        phone: String,
    },

    /// List all contacts
    List,

    /// Remove a contact
    Remove {
        /// Contact name to remove
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode enabled");
    }

    let api_key = cli.api_key.clone().unwrap_or_else(|| {
        if matches!(cli.command, Commands::Send { .. }) {
            eprintln!("Error: API key required for sending messages");
            eprintln!("Use --api-key or -k to provide your API key");
            std::process::exit(1);
        }
        "not_required".to_string()
    });

    match cli.command {
        Commands::Send { to, message } => {
            handle_send(&api_key, &to, &message, cli.verbose);
        }
        Commands::History { limit } => {
            handle_history(limit, cli.verbose);
        }
        Commands::Contact(cmd) => handle_contact(cmd, cli.verbose),
    }
}

fn handle_send(api_key: &str, to: &str, message: &str, verbose: bool) {
    println!("Sending SMS...");
    if verbose {
        println!("API Key: {}...", &api_key[..8.min(api_key.len())]);
    }
    println!("To: {}", to);
    println!("Message: {}", message);
    println!("---");
    println!("Message sent successfully!");
}

fn handle_history(limit: usize, verbose: bool) {
    let messages = [
        ("2024-01-15 10:30", "+1234567890", "Hello from Textio!"),
        ("2024-01-15 09:15", "+1987654321", "Your code is 123456"),
        ("2024-01-14 16:45", "+1555555555", "Meeting at 3pm"),
        ("2024-01-14 12:00", "+1444444444", "Order shipped!"),
        ("2024-01-13 08:30", "+1333333333", "Welcome to our service"),
    ];

    println!("Message History (showing {})", limit);
    println!("-------------------");

    for (i, (time, to, msg)) in messages.iter().enumerate() {
        if i >= limit {
            break;
        }
        if verbose {
            println!("[{}] To: {}", time, to);
            println!("  Message: {}", msg);
            println!();
        } else {
            println!("[{}] {} -> {}", time, to, msg);
        }
    }
}

fn handle_contact(cmd: ContactCommands, verbose: bool) {
    match cmd {
        ContactCommands::Add { name, phone } => {
            println!("Adding contact...");
            if verbose {
                println!("Name: {}", name);
                println!("Phone: {}", phone);
            }
            println!("Contact '{}' added successfully!", name);
        }
        ContactCommands::List => {
            let contacts = [
                ("Alice", "+1234567890"),
                ("Bob", "+1987654321"),
                ("Charlie", "+1555555555"),
            ];

            println!("Contacts:");
            println!("---------");
            for (name, phone) in contacts {
                println!("{}: {}", name, phone);
            }
        }
        ContactCommands::Remove { name } => {
            println!("Removing contact '{}'...", name);
            println!("Contact removed successfully!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_command() {
        let cli = Cli::try_parse_from([
            "textio",
            "send",
            "--to", "+1234567890",
            "--message", "Hello",
            "--api-key", "secret",
        ]);

        assert!(cli.is_ok());
        let cli = cli.unwrap();
        match cli.command {
            Commands::Send { to, message } => {
                assert_eq!(to, "+1234567890");
                assert_eq!(message, "Hello");
            }
            _ => panic!("Expected Send command"),
        }
    }

    #[test]
    fn test_history_with_limit() {
        let cli = Cli::try_parse_from([
            "textio",
            "history",
            "--limit", "5",
        ]);

        assert!(cli.is_ok());
        let cli = cli.unwrap();
        match cli.command {
            Commands::History { limit } => {
                assert_eq!(limit, 5);
            }
            _ => panic!("Expected History command"),
        }
    }

    #[test]
    fn test_contact_add() {
        let cli = Cli::try_parse_from([
            "textio",
            "contact",
            "add",
            "Alice",
            "+1234567890",
        ]);

        assert!(cli.is_ok());
        let cli = cli.unwrap();
        match cli.command {
            Commands::Contact(ContactCommands::Add { name, phone }) => {
                assert_eq!(name, "Alice");
                assert_eq!(phone, "+1234567890");
            }
            _ => panic!("Expected Contact Add command"),
        }
    }
}
