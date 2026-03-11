// Exercise 1: File Operations
// 
// Implement file operations for the Textio messaging system.
// Complete the functions marked with TODO.

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Represents a contact in the Textio system
#[derive(Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub phone: String,
}

/// Load contacts from a CSV file
/// 
/// File format:
/// # Comments start with #
/// Name, phone_number
/// 
/// Example:
/// John Doe, +1234567890
/// Jane Smith, +0987654321
pub fn load_contacts(path: &Path) -> io::Result<Vec<Contact>> {
    // TODO: Implement this function
    // 1. Open the file using File::open
    // 2. Create a BufReader
    // 3. Iterate through lines
    // 4. Skip empty lines and lines starting with #
    // 5. Parse each line as "name, phone"
    // 6. Return the vector of contacts
    
    todo!()
}

/// Save contacts to a CSV file
/// 
/// Write contacts in the format:
/// # Textio Contacts
/// Name, phone
pub fn save_contacts(path: &Path, contacts: &[Contact]) -> io::Result<()> {
    // TODO: Implement this function
    // 1. Create a new file using File::create
    // 2. Create a BufWriter
    // 3. Write the header comment
    // 4. Write each contact on a new line
    // 5. Flush the writer
    
    todo!()
}

/// Append a message log entry to a file
/// 
/// Format: [timestamp] To: phone | Status: status | Message: message
pub fn append_to_log(path: &Path, phone: &str, status: &str, message: &str) -> io::Result<()> {
    // TODO: Implement this function
    // 1. Use OpenOptions to open file in append mode (create if doesn't exist)
    // 2. Write a formatted log entry with timestamp
    // 3. Use the get_timestamp() helper function
    
    todo!()
}

/// Copy a template file to a new location
/// 
/// Read the template, replace {{name}} placeholder with actual name
pub fn personalize_template(template_path: &Path, output_path: &Path, name: &str) -> io::Result<()> {
    // TODO: Implement this function
    // 1. Read the template file content
    // 2. Replace all occurrences of "{{name}}" with the actual name
    // 3. Write the personalized content to output_path
    
    todo!()
}

/// Read file metadata and return file size
pub fn get_file_size(path: &Path) -> io::Result<u64> {
    // TODO: Implement this function
    // Use fs::metadata to get file information
    
    todo!()
}

/// Check if a file exists
pub fn file_exists(path: &Path) -> bool {
    // TODO: Implement this function
    // Use Path::exists method
    
    todo!()
}

/// Create a backup of a file by appending .bak extension
pub fn create_backup(path: &Path) -> io::Result<PathBuf> {
    // TODO: Implement this function
    // 1. Read the original file content
    // 2. Create backup path with .bak extension
    // 3. Write content to backup path
    // 4. Return the backup path
    
    todo!()
}

// Helper function for timestamps
fn get_timestamp() -> String {
    // In real code, you'd use chrono or time crate
    // For this exercise, we return a fixed format
    "2024-01-15 10:30:00".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_test_file(path: &Path, content: &str) {
        fs::write(path, content).expect("Failed to create test file");
    }

    #[test]
    fn test_load_contacts() {
        let path = Path::new("test_contacts.txt");
        let content = "# Test contacts\nJohn Doe, +1234567890\nJane Smith, +0987654321\n";
        setup_test_file(path, content);
        
        let contacts = load_contacts(path).expect("Failed to load contacts");
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].name, "John Doe");
        assert_eq!(contacts[0].phone, "+1234567890");
        
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_save_and_load_contacts() {
        let path = Path::new("test_save_contacts.txt");
        let contacts = vec![
            Contact { name: "Alice".to_string(), phone: "+1111111111".to_string() },
            Contact { name: "Bob".to_string(), phone: "+2222222222".to_string() },
        ];
        
        save_contacts(path, &contacts).expect("Failed to save");
        let loaded = load_contacts(path).expect("Failed to load");
        
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].name, "Alice");
        
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_append_to_log() {
        let path = Path::new("test_log.txt");
        let _ = fs::remove_file(path);
        
        append_to_log(path, "+1234567890", "sent", "Hello!")
            .expect("Failed to append");
        
        let content = fs::read_to_string(path).expect("Failed to read");
        assert!(content.contains("+1234567890"));
        assert!(content.contains("sent"));
        assert!(content.contains("Hello!"));
        
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_file_exists() {
        let path = Path::new("test_exists.txt");
        fs::write(path, "test").ok();
        
        assert!(file_exists(path));
        assert!(!file_exists(Path::new("nonexistent.txt")));
        
        fs::remove_file(path).ok();
    }
}

fn main() {
    println!("Run tests with: cargo test");
}
