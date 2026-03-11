// Exercise 3: CLI Arguments - Complete Solution

use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Send {
        to: String,
        message: String,
        from: Option<String>,
    },
    Status {
        message_id: String,
    },
    Contacts {
        action: ContactAction,
    },
    Config {
        key: Option<String>,
        value: Option<String>,
    },
    Help,
    Version,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContactAction {
    List,
    Add { name: String, phone: String },
    Remove { phone: String },
}

pub fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("No command provided. Use -h for help.".to_string());
    }
    
    let first_arg = &args[1];
    
    match first_arg.as_str() {
        "-h" | "--help" => Ok(Command::Help),
        "-v" | "--version" => Ok(Command::Version),
        "send" => parse_send_command(&args[2..]),
        "status" => parse_status_command(&args[2..]),
        "contacts" => parse_contacts_command(&args[2..]),
        "config" => parse_config_command(&args[2..]),
        cmd => Err(format!("Unknown command: {}. Use -h for help.", cmd)),
    }
}

fn parse_send_command(args: &[String]) -> Result<Command, String> {
    let (flags, positional) = extract_flags(args);
    
    if positional.len() < 2 {
        return Err("Usage: send <to> <message> [--from <sender>]".to_string());
    }
    
    let to = validate_phone(&positional[0])?;
    let message = positional[1].clone();
    
    let from = flags
        .iter()
        .find(|(flag, _)| flag == "--from" || flag == "-f")
        .and_then(|(_, value)| value.clone());
    
    Ok(Command::Send { to, message, from })
}

fn parse_status_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("Usage: status <message_id>".to_string());
    }
    
    let message_id = args[0].clone();
    
    if !message_id.starts_with("msg_") {
        return Err("Invalid message ID format. Expected 'msg_...'".to_string());
    }
    
    Ok(Command::Status { message_id })
}

fn parse_contacts_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("Usage: contacts <list|add|remove> [args...]".to_string());
    }
    
    let action = match args[0].as_str() {
        "list" => ContactAction::List,
        "add" => {
            if args.len() < 3 {
                return Err("Usage: contacts add <name> <phone>".to_string());
            }
            let phone = validate_phone(&args[2])?;
            ContactAction::Add {
                name: args[1].clone(),
                phone,
            }
        }
        "remove" => {
            if args.len() < 2 {
                return Err("Usage: contacts remove <phone>".to_string());
            }
            let phone = validate_phone(&args[1])?;
            ContactAction::Remove { phone }
        }
        subcmd => return Err(format!("Unknown contacts subcommand: {}", subcmd)),
    };
    
    Ok(Command::Contacts { action })
}

fn parse_config_command(args: &[String]) -> Result<Command, String> {
    let key = args.get(0).cloned();
    let value = args.get(1).cloned();
    
    Ok(Command::Config { key, value })
}

pub fn validate_phone(phone: &str) -> Result<String, String> {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    
    if digits.len() < 10 || digits.len() > 15 {
        return Err(format!(
            "Invalid phone number: must be 10-15 digits, got {}",
            digits.len()
        ));
    }
    
    if phone.starts_with('+') {
        Ok(phone.to_string())
    } else {
        Ok(format!("+{}", digits))
    }
}

pub fn extract_flags(args: &[String]) -> (Vec<(String, Option<String>)>, Vec<String>) {
    let mut flags = Vec::new();
    let mut positional = Vec::new();
    let mut i = 0;
    
    let value_flags = ["--from", "-f", "--file", "-o", "--output"];
    
    while i < args.len() {
        let arg = &args[i];
        
        if arg.starts_with('-') {
            let flag_name = arg.clone();
            
            if value_flags.contains(&arg.as_str()) && i + 1 < args.len() {
                let value = args[i + 1].clone();
                flags.push((flag_name, Some(value)));
                i += 2;
            } else {
                flags.push((flag_name, None));
                i += 1;
            }
        } else {
            positional.push(arg.clone());
            i += 1;
        }
    }
    
    (flags, positional)
}

pub fn build_help() -> String {
    let mut help = String::new();
    
    help.push_str("Textio - SMS Messaging CLI\n\n");
    help.push_str("USAGE:\n");
    help.push_str("    textio <command> [options]\n\n");
    help.push_str("COMMANDS:\n");
    help.push_str("    send <to> <message>    Send an SMS message\n");
    help.push_str("        --from <id>        Set sender ID\n");
    help.push_str("    status <id>            Check message status\n");
    help.push_str("    contacts <action>      Manage contacts\n");
    help.push_str("        list               List all contacts\n");
    help.push_str("        add <name> <phone> Add a contact\n");
    help.push_str("        remove <phone>     Remove a contact\n");
    help.push_str("    config [key] [value]   View or set configuration\n\n");
    help.push_str("OPTIONS:\n");
    help.push_str("    -h, --help             Print this help message\n");
    help.push_str("    -v, --version          Print version info\n");
    
    help
}

pub fn build_version() -> String {
    "Textio CLI v1.0.0".to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match parse_args(&args) {
        Ok(command) => {
            match command {
                Command::Help => println!("{}", build_help()),
                Command::Version => println!("{}", build_version()),
                Command::Send { to, message, from } => {
                    println!("Sending message...");
                    println!("  To: {}", to);
                    println!("  Message: {}", message);
                    if let Some(sender) = from {
                        println!("  From: {}", sender);
                    }
                }
                Command::Status { message_id } => {
                    println!("Checking status for: {}", message_id);
                }
                Command::Contacts { action } => {
                    match action {
                        ContactAction::List => println!("Listing contacts..."),
                        ContactAction::Add { name, phone } => {
                            println!("Adding contact: {} ({})", name, phone);
                        }
                        ContactAction::Remove { phone } => {
                            println!("Removing contact: {}", phone);
                        }
                    }
                }
                Command::Config { key, value } => {
                    match (key, value) {
                        (None, None) => println!("Showing all config..."),
                        (Some(k), None) => println!("Getting config: {}", k),
                        (Some(k), Some(v)) => println!("Setting {} = {}", k, v),
                        (None, Some(_)) => unreachable!(),
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Use -h for help.");
            std::process::exit(1);
        }
    }
}
