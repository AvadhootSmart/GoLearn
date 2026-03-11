use clap::{Parser, Subcommand};

/// Textio SMS CLI - Manage your SMS communications
#[derive(Parser)]
#[command(name = "textio")]
#[command(version = "1.0")]
struct Cli {
    /// TODO: Add global API key flag (optional, short: 'k', long: "api-key")
    // Hint: Use Option<String> and add global = true
    
    /// TODO: Add global verbose flag (optional, short: 'v', long: "verbose")
    // Hint: Use bool with global = true
    
    /// TODO: Add subcommand field
    // Hint: #[command(subcommand)] command: Commands,
}

/// TODO: Define the Commands enum with Subcommand derive
// Hint: Create an enum with variants for Send, History, and Contact
#[derive(Subcommand)]
enum Commands {
    // TODO: Add Send variant with 'to' and 'message' arguments
    // Hint: Use struct variant with named fields
    
    // TODO: Add History variant with optional 'limit' argument (default: 10)
    // Hint: Use usize with default_value
    
    // TODO: Add Contact variant that nests ContactCommands
    // Hint: Use #[command(subcommand)] Contact(ContactCommands)
}

/// TODO: Define ContactCommands enum for nested subcommands
#[derive(Subcommand)]
enum ContactCommands {
    // TODO: Add Add variant with name and phone (both positional)
    
    // TODO: Add List variant (no arguments)
    
    // TODO: Add Remove variant with name (positional)
}

fn main() {
    // TODO: Parse CLI arguments
    // let cli = Cli::parse();
    
    // TODO: Handle verbose flag
    // if cli.verbose { println!("Verbose mode enabled"); }
    
    // TODO: Handle API key (check if provided)
    // let api_key = cli.api_key.unwrap_or_else(|| { ... });
    
    // TODO: Match on commands and implement logic
    // match cli.command {
    //     Commands::Send { to, message } => { ... }
    //     Commands::History { limit } => { ... }
    //     Commands::Contact(cmd) => match cmd { ... }
    // }
    
    println!("CLI not implemented yet!");
}

// HINTS:
// 1. Global flags require global = true in the arg attribute:
//    #[arg(short, long, global = true)]
//
// 2. For nested subcommands, use:
//    #[command(subcommand)]
//    Contact(ContactCommands)
//
// 3. For positional arguments in struct variants, just declare them without #[arg]:
//    Add { name: String, phone: String }
//
// 4. The match statement for nested commands:
//    Commands::Contact(cmd) => match cmd {
//        ContactCommands::Add { name, phone } => { ... }
//        ContactCommands::List => { ... }
//        ContactCommands::Remove { name } => { ... }
//    }
