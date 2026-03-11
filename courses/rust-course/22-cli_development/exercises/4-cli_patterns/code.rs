use clap::Parser;
use std::io::{self, Read};
use std::process::ExitCode;

/// Textio SMS CLI with professional patterns
#[derive(Parser)]
#[command(name = "textio")]
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
}

/// TODO: Define error types with corresponding exit codes
// Hint: Create an enum with NetworkError, InvalidInput, AuthFailed, etc.
// impl From<ErrorType> for ExitCode
#[derive(Debug)]
enum TextioError {
    // TODO: Add error variants
}

/// TODO: Implement exit codes for each error
impl From<TextioError> for ExitCode {
    fn from(_error: TextioError) -> Self {
        // TODO: Match on error and return appropriate ExitCode
        ExitCode::FAILURE
    }
}

/// TODO: Create a function to read from stdin
// Hint: Use io::stdin().read_to_string() or lines()
fn read_from_stdin() -> Result<String, TextioError> {
    // TODO: Implement stdin reading
    Err(TextioError::InvalidInput)
}

/// TODO: Create colored output functions
// Hint: Use colored crate: "text".green(), "text".red(), etc.
fn print_success(msg: &str) {
    // TODO: Print in green
    println!("{}", msg);
}

fn print_error(msg: &str) {
    // TODO: Print in red with "Error:" prefix
    eprintln!("{}", msg);
}

fn print_warning(msg: &str) {
    // TODO: Print in yellow
    println!("{}", msg);
}

/// TODO: Create progress bar display
// Hint: Use indicatif crate
fn show_progress(steps: u64) {
    // TODO: Create and run a progress bar
    // 1. Create ProgressBar::new(steps)
    // 2. Set a style
    // 3. Increment and finish
}

/// TODO: Create JSON output formatter
fn format_as_json(to: &str, message: &str, status: &str) -> String {
    // TODO: Use serde_json to format output
    format!("{{\"to\": \"{}\", \"message\": \"{}\", \"status\": \"{}\"}}", to, message, status)
}

/// TODO: Create text output formatter with colors
fn format_as_text(to: &str, message: &str, status: &str) -> String {
    // TODO: Format with colored output
    format!("To: {}\nMessage: {}\nStatus: {}", to, message, status)
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // TODO: Handle --error flag for testing exit codes
    // match cli.error.as_deref() {
    //     Some("network") => { print_error("Network error"); return ExitCode::from(10); }
    //     Some("auth") => { print_error("Auth failed"); return ExitCode::from(30); }
    //     ...
    // }

    // TODO: Get message from argument or stdin
    // let message = if cli.stdin {
    //     read_from_stdin()?
    // } else {
    //     cli.message.ok_or(TextioError::InvalidInput)?
    // };

    // TODO: Validate required arguments
    // let to = cli.to.ok_or(TextioError::InvalidInput)?;

    // TODO: Show progress if --progress flag is set
    // if cli.progress {
    //     show_progress(10);
    // }

    // TODO: Format and output based on --format flag
    // let output = match cli.format.as_str() {
    //     "json" => format_as_json(&to, &message, "sent"),
    //     _ => format_as_text(&to, &message, "sent"),
    // };
    // println!("{}", output);

    // TODO: Print success message
    // print_success("Message sent successfully!");

    println!("CLI patterns not implemented yet!");
    ExitCode::SUCCESS
}

// HINTS:
//
// 1. For colored output, add to Cargo.toml:
//    [dependencies]
//    colored = "2.1"
//
//    Then use:
//    use colored::Colorize;
//    fn print_success(msg: &str) {
//        println!("{}", msg.green());
//    }
//    fn print_error(msg: &str) {
//        eprintln!("{} {}", "Error:".red().bold(), msg);
//    }
//
// 2. For progress bars, add to Cargo.toml:
//    indicatif = "0.17"
//
//    Then use:
//    use indicatif::{ProgressBar, ProgressStyle};
//    fn show_progress(steps: u64) {
//        let bar = ProgressBar::new(steps);
//        bar.set_style(
//            ProgressStyle::with_template("[{elapsed_precise}] {bar:40} {pos}/{len}")
//                .unwrap()
//        );
//        for _ in 0..steps {
//            bar.inc(1);
//            std::thread::sleep(std::time::Duration::from_millis(100));
//        }
//        bar.finish();
//    }
//
// 3. For stdin reading:
//    fn read_from_stdin() -> Result<String, TextioError> {
//        let mut input = String::new();
//        io::stdin().read_to_string(&mut input)
//            .map_err(|_| TextioError::InvalidInput)?;
//        Ok(input.trim().to_string())
//    }
//
// 4. For exit codes:
//    impl From<TextioError> for ExitCode {
//        fn from(error: TextioError) -> Self {
//            match error {
//                TextioError::NetworkError => ExitCode::from(10),
//                TextioError::InvalidInput => ExitCode::from(20),
//                TextioError::AuthFailed => ExitCode::from(30),
//            }
//        }
//    }
