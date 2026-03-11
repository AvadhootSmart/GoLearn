// Exercise 1: File Operations - Complete Solution

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub phone: String,
}

pub fn load_contacts(path: &Path) -> io::Result<Vec<Contact>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let contacts: Vec<Contact> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty() && !line.trim().starts_with('#'))
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                Some(Contact {
                    name: parts[0].trim().to_string(),
                    phone: parts[1].trim().to_string(),
                })
            } else {
                None
            }
        })
        .collect();
    
    Ok(contacts)
}

pub fn save_contacts(path: &Path, contacts: &[Contact]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    writeln!(writer, "# Textio Contacts")?;
    
    for contact in contacts {
        writeln!(writer, "{}, {}", contact.name, contact.phone)?;
    }
    
    writer.flush()?;
    Ok(())
}

pub fn append_to_log(path: &Path, phone: &str, status: &str, message: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    
    let timestamp = get_timestamp();
    writeln!(
        file,
        "[{}] To: {} | Status: {} | Message: {}",
        timestamp, phone, status, message
    )?;
    
    Ok(())
}

pub fn personalize_template(template_path: &Path, output_path: &Path, name: &str) -> io::Result<()> {
    let content = fs::read_to_string(template_path)?;
    let personalized = content.replace("{{name}}", name);
    fs::write(output_path, personalized)?;
    Ok(())
}

pub fn get_file_size(path: &Path) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

pub fn create_backup(path: &Path) -> io::Result<PathBuf> {
    let content = fs::read_to_string(path)?;
    let backup_path = path.with_extension("bak");
    fs::write(&backup_path, content)?;
    Ok(backup_path)
}

fn get_timestamp() -> String {
    "2024-01-15 10:30:00".to_string()
}

fn main() -> io::Result<()> {
    let contacts_path = Path::new("contacts.txt");
    
    let contacts = vec![
        Contact {
            name: "John Doe".to_string(),
            phone: "+1234567890".to_string(),
        },
        Contact {
            name: "Jane Smith".to_string(),
            phone: "+0987654321".to_string(),
        },
        Contact {
            name: "Bob Wilson".to_string(),
            phone: "+1122334455".to_string(),
        },
    ];
    
    save_contacts(contacts_path, &contacts)?;
    println!("Saved {} contacts", contacts.len());
    
    let loaded = load_contacts(contacts_path)?;
    println!("Loaded {} contacts:", loaded.len());
    for contact in &loaded {
        println!("  - {} ({})", contact.name, contact.phone);
    }
    
    let log_path = Path::new("message_log.txt");
    append_to_log(log_path, "+1234567890", "sent", "Hello, World!")?;
    append_to_log(log_path, "+0987654321", "delivered", "Test message")?;
    println!("Messages logged");
    
    if file_exists(contacts_path) {
        let size = get_file_size(contacts_path)?;
        println!("Contacts file size: {} bytes", size);
    }
    
    let backup_path = create_backup(contacts_path)?;
    println!("Backup created at: {:?}", backup_path);
    
    fs::remove_file(contacts_path).ok();
    fs::remove_file(log_path).ok();
    fs::remove_file(&backup_path).ok();
    
    Ok(())
}
