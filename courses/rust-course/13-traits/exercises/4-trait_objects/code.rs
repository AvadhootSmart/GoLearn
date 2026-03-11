// Exercise 4: Trait Objects
//
// Learn to use trait objects for dynamic dispatch in Textio.
// Complete the TODO sections to implement the notification system.

// TODO: Define a Notifier trait that is object-safe with:
// - send(&self, message: &str) -> Result<(), String>
// - name(&self) -> &str
// - max_message_length(&self) -> usize



// TODO: Define a DeliveryReporter trait with associated type:
// - type Report
// - generate_report(&self) -> Self::Report
// This trait will NOT be object-safe due to the associated type



// SMS Notifier implementation
pub struct SmsNotifier {
    pub phone_number: String,
    pub country_code: String,
}

// TODO: Implement Notifier for SmsNotifier
// max_message_length returns 160 (standard SMS limit)



// Email Notifier implementation
pub struct EmailNotifier {
    pub email: String,
    pub smtp_server: String,
}

// TODO: Implement Notifier for EmailNotifier
// max_message_length returns 10000



// Push Notifier implementation
pub struct PushNotifier {
    pub device_token: String,
    pub platform: String,
}

// TODO: Implement Notifier for PushNotifier
// max_message_length returns 4096



// Webhook Notifier implementation
pub struct WebhookNotifier {
    pub url: String,
    pub timeout_ms: u32,
}

// TODO: Implement Notifier for WebhookNotifier
// max_message_length returns 100000



// DeliveryReport for the DeliveryReporter trait
pub struct DeliveryReport {
    pub notifier_name: String,
    pub messages_sent: u32,
    pub success_rate: f64,
}

// TODO: Implement DeliveryReporter for SmsNotifier
// type Report = DeliveryReport
// generate_report returns a DeliveryReport with the notifier name



// TODO: Create a function `send_notification` that:
// - Takes a reference to a dyn Notifier and a message string
// - Checks if the message fits within max_message_length
// - If too long, returns Err with "Message too long for {name}"
// - Otherwise, calls send and returns the result



// TODO: Create a function `broadcast_message` that:
// - Takes a slice of Box<dyn Notifier> and a message string
// - Sends the message through each notifier
// - Returns a count of successful sends



// TODO: Create a function `find_best_notifier` that:
// - Takes a slice of &dyn Notifier and a message length
// - Returns Option<&dyn Notifier> - the first notifier that can handle the length
// - Returns None if no notifier can handle the message



// TODO: Create a struct `NotificationManager` that:
// - Has a field `notifiers: Vec<Box<dyn Notifier>>`
// - Has a method `add` that adds a Box<dyn Notifier>
// - Has a method `broadcast` that sends to all notifiers
// - Has a method `list_notifiers` that prints all notifier names



fn main() {
    // Create notifiers
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

    // Test individual notifications
    println!("=== Individual Notifications ===");
    send_notification(&sms, "Hello via SMS!").unwrap();
    send_notification(&email, "Hello via Email!").unwrap();

    // Test message too long
    println!("\n=== Testing Length Limits ===");
    let long_message = "x".repeat(200);
    match send_notification(&sms, &long_message) {
        Ok(_) => println!("SMS sent"),
        Err(e) => println!("SMS error: {}", e),
    }

    // Test broadcast
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

    // Test find_best_notifier
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

    // Test NotificationManager
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

    // Test DeliveryReporter (static dispatch)
    println!("\n=== Delivery Report (Static Dispatch) ===");
    let sms_reporter = SmsNotifier {
        phone_number: String::from("+15553333333"),
        country_code: String::from("US"),
    };
    let report = sms_reporter.generate_report();
    println!("Report for: {}", report.notifier_name);
}
