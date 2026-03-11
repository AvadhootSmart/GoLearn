// Textio Test Organization Exercise - Complete Solution

use std::cell::RefCell;

pub struct Message {
    pub id: String,
    pub to: String,
    pub body: String,
}

impl Message {
    pub fn new(id: &str, to: &str, body: &str) -> Self {
        Message {
            id: id.to_string(),
            to: to.to_string(),
            body: body.to_string(),
        }
    }
}

pub struct Client {
    api_key: String,
    messages: RefCell<Vec<Message>>,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Client {
            api_key: api_key.to_string(),
            messages: RefCell::new(Vec::new()),
        }
    }

    pub fn send(&self, to: &str, body: &str) -> Result<Message, String> {
        if !to.starts_with('+') {
            return Err("Phone number must start with '+'".to_string());
        }
        if body.is_empty() {
            return Err("Message body cannot be empty".to_string());
        }
        if body.len() > 160 {
            return Err("Message too long (max 160 characters)".to_string());
        }

        let message = Message {
            id: format!("msg_{}", self.messages.borrow().len() + 1),
            to: to.to_string(),
            body: body.to_string(),
        };
        
        self.messages.borrow_mut().push(message.clone());
        Ok(message)
    }

    pub fn send_batch(&self, messages: Vec<(&str, &str)>) -> Vec<Result<Message, String>> {
        messages.iter().map(|(to, body)| self.send(to, body)).collect()
    }

    pub fn message_count(&self) -> usize {
        self.messages.borrow().len()
    }

    pub fn clear(&self) {
        self.messages.borrow_mut().clear();
    }
}

pub fn calculate_cost(message_count: u32) -> u32 {
    if message_count <= 100 {
        return 0;
    }
    let billable = message_count - 100;
    let units = (billable + 99) / 100;
    units * 5
}

pub fn validate_phone(phone: &str) -> Result<String, String> {
    if !phone.starts_with('+') {
        return Err("Phone number must start with '+'".to_string());
    }
    
    let digits: String = phone[1..].chars().filter(|c| c.is_ascii_digit()).collect();
    
    match digits.len() {
        0..=9 => Err(format!("Phone too short: {} digits", digits.len())),
        10..=15 => Ok(phone.to_string()),
        _ => Err(format!("Phone too long: {} digits", digits.len())),
    }
}

pub fn validate_and_panic(phone: &str) {
    if let Err(e) = validate_phone(phone) {
        panic!("{}", e);
    }
}

pub fn parse_message_batch(input: &str) -> Vec<Message> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some(Message::new(
                    &format!("batch_{}", parts[0]),
                    parts[0],
                    parts[1],
                ))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixtures {
        use super::*;
        
        pub fn valid_phone() -> &'static str {
            "+1234567890"
        }
        
        pub fn invalid_phone() -> &'static str {
            "invalid"
        }
        
        pub fn test_client() -> Client {
            Client::new("test_api_key_12345")
        }
        
        pub fn sample_messages() -> Vec<(&'static str, &'static str)> {
            vec![
                ("+1111111111", "First message"),
                ("+2222222222", "Second message"),
                ("+3333333333", "Third message"),
            ]
        }
        
        pub fn batch_input() -> &'static str {
            "+1111111111:Hello\n+2222222222:World\n+3333333333:Test"
        }
    }

    mod validation {
        use super::*;
        
        #[test]
        fn test_valid_phone() {
            let result = validate_phone("+1234567890");
            assert!(result.is_ok());
        }
        
        #[test]
        fn test_valid_phone_using_fixture() {
            let result = validate_phone(super::fixtures::valid_phone());
            assert!(result.is_ok());
        }
        
        #[test]
        fn test_invalid_phone_no_plus() {
            let result = validate_phone("1234567890");
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("'+'"));
        }
        
        #[test]
        fn test_invalid_phone_too_short() {
            let result = validate_phone("+123");
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("short"));
        }
        
        #[test]
        fn test_invalid_phone_too_long() {
            let result = validate_phone("+12345678901234567890");
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("long"));
        }
        
        #[test]
        #[should_panic(expected = "must start with '+'")]
        fn test_validate_and_panic_no_plus() {
            validate_and_panic("1234567890");
        }
        
        #[test]
        #[should_panic(expected = "too short")]
        fn test_validate_and_panic_short() {
            validate_and_panic("+123");
        }
        
        #[test]
        fn test_multiple_valid_phones() {
            let valid_phones = vec![
                "+1234567890",
                "+123456789012345",
                "+12345678901",
            ];
            
            for phone in valid_phones {
                assert!(
                    validate_phone(phone).is_ok(),
                    "Expected {} to be valid",
                    phone
                );
            }
        }
    }

    mod cost_calculation {
        use super::*;
        
        #[test]
        fn test_free_tier() {
            let test_cases = vec![
                (0, 0),
                (1, 0),
                (50, 0),
                (99, 0),
                (100, 0),
            ];
            
            for (count, expected) in test_cases {
                assert_eq!(
                    calculate_cost(count),
                    expected,
                    "Failed for count={}",
                    count
                );
            }
        }
        
        #[test]
        fn test_boundary() {
            assert_eq!(calculate_cost(100), 0);
            assert_eq!(calculate_cost(101), 5);
        }
        
        #[test]
        fn test_paid_tier() {
            let test_cases = vec![
                (101, 5),
                (150, 5),
                (199, 5),
                (200, 5),
                (201, 10),
                (250, 10),
                (300, 10),
                (301, 15),
                (1000, 45),
            ];
            
            for (count, expected) in test_cases {
                assert_eq!(
                    calculate_cost(count),
                    expected,
                    "Failed for count={}",
                    count
                );
            }
        }
        
        #[test]
        fn test_large_volumes() {
            assert_eq!(calculate_cost(10000), 495);
            assert_eq!(calculate_cost(100000), 4995);
        }
    }

    mod client_tests {
        use super::*;
        
        #[test]
        fn test_send_single_message() {
            let client = fixtures::test_client();
            let result = client.send("+1234567890", "Hello World");
            
            assert!(result.is_ok());
            let msg = result.unwrap();
            assert_eq!(msg.to, "+1234567890");
            assert_eq!(msg.body, "Hello World");
            assert!(msg.id.starts_with("msg_"));
        }
        
        #[test]
        fn test_send_with_fixture() {
            let client = fixtures::test_client();
            let result = client.send(fixtures::valid_phone(), "Test message");
            
            assert!(result.is_ok());
        }
        
        #[test]
        fn test_send_empty_body() {
            let client = fixtures::test_client();
            let result = client.send("+1234567890", "");
            
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("empty"));
        }
        
        #[test]
        fn test_send_too_long() {
            let client = fixtures::test_client();
            let long_body: String = "x".repeat(161);
            let result = client.send("+1234567890", &long_body);
            
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("too long"));
        }
        
        #[test]
        fn test_send_batch() {
            let client = fixtures::test_client();
            let messages = fixtures::sample_messages();
            
            let results = client.send_batch(messages);
            
            assert_eq!(results.len(), 3);
            assert!(results.iter().all(|r| r.is_ok()));
            assert_eq!(client.message_count(), 3);
        }
        
        #[test]
        fn test_send_batch_with_errors() {
            let client = fixtures::test_client();
            let messages = vec![
                ("+1111111111", "Valid"),
                ("invalid", "Invalid phone"),
                ("+2222222222", "Also valid"),
            ];
            
            let results = client.send_batch(messages);
            
            assert_eq!(results.len(), 3);
            assert!(results[0].is_ok());
            assert!(results[1].is_err());
            assert!(results[2].is_ok());
            assert_eq!(client.message_count(), 2);
        }
        
        #[test]
        fn test_message_count() {
            let client = fixtures::test_client();
            
            assert_eq!(client.message_count(), 0);
            
            client.send("+1111111111", "First").unwrap();
            assert_eq!(client.message_count(), 1);
            
            client.send("+2222222222", "Second").unwrap();
            assert_eq!(client.message_count(), 2);
        }
        
        #[test]
        fn test_clear() {
            let client = fixtures::test_client();
            client.send("+1234567890", "Test").unwrap();
            
            assert_eq!(client.message_count(), 1);
            
            client.clear();
            assert_eq!(client.message_count(), 0);
        }
        
        #[test]
        #[ignore = "Simulates slow network test"]
        fn slow_network_test() {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let client = fixtures::test_client();
            let result = client.send("+1234567890", "Slow test");
            assert!(result.is_ok());
        }
    }

    mod message_parsing {
        use super::*;
        
        #[test]
        fn test_parse_batch() {
            let input = "+1111111111:Hello\n+2222222222:World";
            let messages = parse_message_batch(input);
            
            assert_eq!(messages.len(), 2);
            assert_eq!(messages[0].to, "+1111111111");
            assert_eq!(messages[0].body, "Hello");
            assert_eq!(messages[1].to, "+2222222222");
            assert_eq!(messages[1].body, "World");
        }
        
        #[test]
        fn test_parse_batch_with_fixture() {
            let messages = parse_message_batch(fixtures::batch_input());
            assert_eq!(messages.len(), 3);
        }
        
        #[test]
        fn test_parse_batch_empty() {
            let messages = parse_message_batch("");
            assert_eq!(messages.len(), 0);
        }
        
        #[test]
        fn test_parse_batch_skips_empty_lines() {
            let input = "+111:First\n\n+222:Second\n\n";
            let messages = parse_message_batch(input);
            assert_eq!(messages.len(), 2);
        }
        
        #[test]
        fn test_parse_batch_invalid_lines() {
            let input = "+111:Valid\ninvalid\n+222:Also valid";
            let messages = parse_message_batch(input);
            assert_eq!(messages.len(), 2);
        }
    }

    #[test]
    #[ignore = "Long-running integration test"]
    fn complete_workflow() {
        let client = fixtures::test_client();
        
        // Step 1: Validate phone numbers
        let phones = vec!["+1111111111", "+2222222222", "+3333333333"];
        for phone in &phones {
            validate_phone(phone).expect("Phone should be valid");
        }
        
        // Step 2: Send messages
        let mut total_cost = 0;
        for phone in &phones {
            let msg = client.send(phone, "Test message").unwrap();
            assert!(msg.id.starts_with("msg_"));
            total_cost += 1;
        }
        
        // Step 3: Calculate costs
        let cost = calculate_cost(total_cost);
        assert_eq!(cost, 0);
        
        // Step 4: Send more to exceed free tier
        for i in 0..150 {
            let phone = format!("+1111111{:03}", i % 1000);
            client.send(&phone, "Bulk message").unwrap();
        }
        
        assert_eq!(client.message_count(), 153);
        let final_cost = calculate_cost(153);
        assert_eq!(final_cost, 5);
        
        // Step 5: Parse batch messages
        let batch = parse_message_batch(fixtures::batch_input());
        assert_eq!(batch.len(), 3);
        
        // Step 6: Clear and verify
        client.clear();
        assert_eq!(client.message_count(), 0);
    }
    
    #[test]
    fn test_fixture_reusability() {
        // Demonstrate using same fixtures multiple times
        let client1 = fixtures::test_client();
        let client2 = fixtures::test_client();
        
        client1.send(fixtures::valid_phone(), "From client 1").unwrap();
        client2.send(fixtures::valid_phone(), "From client 2").unwrap();
        
        assert_eq!(client1.message_count(), 1);
        assert_eq!(client2.message_count(), 1);
    }
}
