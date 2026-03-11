// Exercise 4: Trait Objects - Complete Solution

pub trait Notifier {
    fn send(&self, message: &str) -> Result<(), String>;
    fn name(&self) -> &str;
    fn max_message_length(&self) -> usize;
}

pub trait DeliveryReporter {
    type Report;
    fn generate_report(&self) -> Self::Report;
}

pub struct SmsNotifier {
    pub phone_number: String,
    pub country_code: String,
}

impl Notifier for SmsNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        println!("[SMS] Sending to {}: {}", self.phone_number, message);
        Ok(())
    }

    fn name(&self) -> &str {
        "SMS Notifier"
    }

    fn max_message_length(&self) -> usize {
        160
    }
}

pub struct EmailNotifier {
    pub email: String,
    pub smtp_server: String,
}

impl Notifier for EmailNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        println!("[Email] Sending to {}: {}", self.email, message);
        Ok(())
    }

    fn name(&self) -> &str {
        "Email Notifier"
    }

    fn max_message_length(&self) -> usize {
        10000
    }
}

pub struct PushNotifier {
    pub device_token: String,
    pub platform: String,
}

impl Notifier for PushNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        println!("[Push:{}] Sending to {}: {}", self.platform, self.device_token, message);
        Ok(())
    }

    fn name(&self) -> &str {
        "Push Notifier"
    }

    fn max_message_length(&self) -> usize {
        4096
    }
}

pub struct WebhookNotifier {
    pub url: String,
    pub timeout_ms: u32,
}

impl Notifier for WebhookNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        println!("[Webhook] POST {} (timeout: {}ms): {}", self.url, self.timeout_ms, message);
        Ok(())
    }

    fn name(&self) -> &str {
        "Webhook Notifier"
    }

    fn max_message_length(&self) -> usize {
        100000
    }
}

pub struct DeliveryReport {
    pub notifier_name: String,
    pub messages_sent: u32,
    pub success_rate: f64,
}

impl DeliveryReporter for SmsNotifier {
    type Report = DeliveryReport;

    fn generate_report(&self) -> Self::Report {
        DeliveryReport {
            notifier_name: self.name().to_string(),
            messages_sent: 0,
            success_rate: 100.0,
        }
    }
}

fn send_notification(notifier: &dyn Notifier, message: &str) -> Result<(), String> {
    if message.len() > notifier.max_message_length() {
        return Err(format!("Message too long for {}", notifier.name()));
    }
    notifier.send(message)
}

fn broadcast_message(notifiers: &[Box<dyn Notifier>], message: &str) -> usize {
    let mut success_count = 0;
    for notifier in notifiers {
        if send_notification(notifier.as_ref(), message).is_ok() {
            success_count += 1;
        }
    }
    success_count
}

fn find_best_notifier<'a>(notifiers: &[&'a dyn Notifier], message_length: usize) -> Option<&'a dyn Notifier> {
    notifiers.iter().find(|n| n.max_message_length() >= message_length).copied()
}

struct NotificationManager {
    notifiers: Vec<Box<dyn Notifier>>,
}

impl NotificationManager {
    fn new() -> Self {
        NotificationManager {
            notifiers: Vec::new(),
        }
    }

    fn add(&mut self, notifier: Box<dyn Notifier>) {
        self.notifiers.push(notifier);
    }

    fn broadcast(&self, message: &str) -> usize {
        broadcast_message(&self.notifiers, message)
    }

    fn list_notifiers(&self) {
        println!("Registered notifiers:");
        for notifier in &self.notifiers {
            println!("  - {} (max: {} chars)", notifier.name(), notifier.max_message_length());
        }
    }
}

fn main() {
    let sms = SmsNotifier {
        phone_number: String::from("+15551234567"),
        country_code: String::from("US"),
    };

    let email = EmailNotifier {
        email: String::from("admin@textio.com"),
        smtp_server: String::from("smtp.textio.com"),
    };

    let push = PushNotifier {
        device_token: String::from("abc123"),
        platform: String::from("iOS"),
    };

    let webhook = WebhookNotifier {
        url: String::from("https://api.example.com/webhook"),
        timeout_ms: 5000,
    };

    println!("=== Individual Notifications ===");
    send_notification(&sms, "Hello via SMS!").unwrap();
    send_notification(&email, "Hello via Email!").unwrap();

    println!("\n=== Testing Length Limits ===");
    let long_message = "x".repeat(200);
    match send_notification(&sms, &long_message) {
        Ok(_) => println!("SMS sent"),
        Err(e) => println!("SMS error: {}", e),
    }

    println!("\n=== Broadcast Test ===");
    let notifiers: Vec<Box<dyn Notifier>> = vec![
        Box::new(SmsNotifier {
            phone_number: String::from("+15551111111"),
            country_code: String::from("US"),
        }),
        Box::new(EmailNotifier {
            email: String::from("team@textio.com"),
            smtp_server: String::from("smtp.textio.com"),
        }),
        Box::new(PushNotifier {
            device_token: String::from("token123"),
            platform: String::from("Android"),
        }),
    ];

    let sent_count = broadcast_message(&notifiers, "Broadcast message!");
    println!("Successfully sent to {} notifiers", sent_count);

    println!("\n=== Finding Best Notifier ===");
    let notifier_refs: Vec<&dyn Notifier> = vec![&sms, &email, &push, &webhook];
    
    match find_best_notifier(&notifier_refs, 150) {
        Some(n) => println!("For 150 chars: {}", n.name()),
        None => println!("No notifier found"),
    }
    
    match find_best_notifier(&notifier_refs, 5000) {
        Some(n) => println!("For 5000 chars: {}", n.name()),
        None => println!("No notifier found"),
    }

    println!("\n=== Notification Manager ===");
    let mut manager = NotificationManager::new();
    manager.add(Box::new(SmsNotifier {
        phone_number: String::from("+15552222222"),
        country_code: String::from("US"),
    }));
    manager.add(Box::new(EmailNotifier {
        email: String::from("ops@textio.com"),
        smtp_server: String::from("smtp.textio.com"),
    }));
    
    manager.list_notifiers();
    
    let count = manager.broadcast("Manager broadcast!");
    println!("Manager sent to {} notifiers", count);

    println!("\n=== Delivery Report (Static Dispatch) ===");
    let sms_reporter = SmsNotifier {
        phone_number: String::from("+15553333333"),
        country_code: String::from("US"),
    };
    let report = sms_reporter.generate_report();
    println!("Report for: {}", report.notifier_name);
}
