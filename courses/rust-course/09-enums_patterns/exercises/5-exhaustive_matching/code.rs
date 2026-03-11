// Exercise 5: Exhaustive Matching
// Ensuring all cases are handled in Textio

use std::fmt;

// This enum simulates an external library type that might be extended
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalStatus {
    Queued,
    Processing,
    Completed,
    Failed,
    // Library might add more variants later
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered { timestamp: u64 },
    Failed { code: u16, reason: String },
    Cancelled { by_user: bool },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Plan {
    Free,
    Basic,
    Pro,
    Enterprise,
}

#[derive(Debug, Clone, Copy)]
pub enum Feature {
    Sms,
    Mms,
    ScheduledMessages,
    Webhooks,
    Analytics,
    PrioritySupport,
}

#[derive(Debug, Clone)]
pub enum DeliveryResult {
    Success { id: String, segments: u8 },
    PartialSuccess { id: String, delivered: u8, total: u8 },
    Failed { code: u16, reason: String },
    Pending { retry_count: u8 },
}

// TODO: Implement fmt::Display for MessageStatus
// Must be exhaustive - handle all variants
impl fmt::Display for MessageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// TODO: Implement is_terminal for MessageStatus
// Returns true for Delivered, Failed, or Cancelled
// Use matches! macro
pub fn is_terminal(status: &MessageStatus) -> bool {
    todo!()
}

// TODO: Implement can_retry for MessageStatus
// Only Failed with certain codes can be retried (codes < 500)
// Must handle all variants explicitly
pub fn can_retry(status: &MessageStatus) -> bool {
    todo!()
}

// TODO: Implement describe_external_status
// Handle known variants explicitly, use _ for unknown
// Because ExternalStatus is non_exhaustive, you MUST use _
pub fn describe_external_status(status: &ExternalStatus) -> &'static str {
    todo!()
}

// TODO: Implement handle_delivery_result
// Must be exhaustive - handle all DeliveryResult variants
// Return appropriate message for each
pub fn handle_delivery_result(result: &DeliveryResult) -> String {
    todo!()
}

// TODO: Implement count_segments
// Returns Some(segment count) only for Success and PartialSuccess
// Use pattern matching with .. to ignore fields you don't need
pub fn count_segments(result: &DeliveryResult) -> Option<u8> {
    todo!()
}

// TODO: Implement is_feature_enabled
// Match on (feature, plan) tuple - must handle all combinations
// Use patterns with | for multiple matches
pub fn is_feature_enabled(feature: Feature, plan: Plan) -> bool {
    // Rules:
    // SMS: All plans
    // MMS: Not Free
    // ScheduledMessages: Pro and Enterprise only
    // Webhooks: Pro and Enterprise only
    // Analytics: Enterprise only
    // PrioritySupport: Enterprise only
    todo!()
}

// TODO: Implement categorize_http_status
// Use range patterns for HTTP status codes
// Must be exhaustive with _ catch-all
pub fn categorize_http_status(code: u16) -> HttpCategory {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub enum HttpCategory {
    Informational,
    Success,
    Redirection,
    ClientError,
    ServerError,
    Unknown,
}

// TODO: Implement analyze_results
// Takes a slice of DeliveryResult and returns summary
// Use slice patterns to handle different slice sizes
pub fn analyze_results(results: &[DeliveryResult]) -> ResultSummary {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultSummary {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub pending: usize,
}

// TODO: Implement describe_slice
// Use exhaustive slice patterns
pub fn describe_slice(slice: &[i32]) -> &'static str {
    todo!()
}

// TODO: Implement extract_all_timestamps
// Return timestamps from all Delivered statuses
pub fn extract_all_timestamps(statuses: &[MessageStatus]) -> Vec<u64> {
    todo!()
}

// TODO: Implement status_to_priority
// Use exhaustive matching
// Pending: Low, Sent: Normal, Delivered: High, Failed/Canceled: Critical
pub fn status_to_priority(status: &MessageStatus) -> Priority {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

// TODO: Implement validate_transition
// Check if status transition is valid
// Only certain transitions are allowed:
// - Pending -> Sent, Cancelled
// - Sent -> Delivered, Failed, Cancelled
// - Delivered, Failed, Cancelled -> (no transitions allowed)
pub fn validate_transition(from: &MessageStatus, to: &MessageStatus) -> Result<(), String> {
    todo!()
}

// TODO: Implement summarize_plan_features
// Return a list of all enabled features for a plan
pub fn summarize_plan_features(plan: Plan) -> Vec<&'static str> {
    todo!()
}

fn main() {
    // Test Display implementation
    println!("=== MessageStatus Display ===");
    println!("{}", MessageStatus::Pending);
    println!("{}", MessageStatus::Sent);
    println!("{}", MessageStatus::Delivered { timestamp: 1700000000 });
    println!("{}", MessageStatus::Failed { code: 500, reason: String::from("Timeout") });
    println!("{}", MessageStatus::Cancelled { by_user: true });

    // Test is_terminal
    println!("\n=== is_terminal ===");
    println!("Pending: {}", is_terminal(&MessageStatus::Pending));
    println!("Delivered: {}", is_terminal(&MessageStatus::Delivered { timestamp: 0 }));
    println!("Failed: {}", is_terminal(&MessageStatus::Failed { code: 0, reason: String::new() }));
    println!("Cancelled: {}", is_terminal(&MessageStatus::Cancelled { by_user: false }));

    // Test can_retry
    println!("\n=== can_retry ===");
    let failed_400 = MessageStatus::Failed { code: 400, reason: String::from("Bad request") };
    let failed_500 = MessageStatus::Failed { code: 500, reason: String::from("Server error") };
    println!("Failed 400: {}", can_retry(&failed_400));
    println!("Failed 500: {}", can_retry(&failed_500));
    println!("Pending: {}", can_retry(&MessageStatus::Pending));

    // Test describe_external_status
    println!("\n=== describe_external_status ===");
    println!("{:?}", describe_external_status(&ExternalStatus::Queued));
    println!("{:?}", describe_external_status(&ExternalStatus::Completed));

    // Test handle_delivery_result
    println!("\n=== handle_delivery_result ===");
    let success = DeliveryResult::Success { id: String::from("msg_1"), segments: 1 };
    let partial = DeliveryResult::PartialSuccess { id: String::from("msg_2"), delivered: 2, total: 3 };
    let failed = DeliveryResult::Failed { code: 500, reason: String::from("Error") };
    let pending = DeliveryResult::Pending { retry_count: 1 };
    
    println!("Success: {}", handle_delivery_result(&success));
    println!("Partial: {}", handle_delivery_result(&partial));
    println!("Failed: {}", handle_delivery_result(&failed));
    println!("Pending: {}", handle_delivery_result(&pending));

    // Test count_segments
    println!("\n=== count_segments ===");
    println!("Success: {:?}", count_segments(&success));
    println!("Partial: {:?}", count_segments(&partial));
    println!("Failed: {:?}", count_segments(&failed));

    // Test is_feature_enabled
    println!("\n=== is_feature_enabled ===");
    println!("SMS on Free: {}", is_feature_enabled(Feature::Sms, Plan::Free));
    println!("MMS on Free: {}", is_feature_enabled(Feature::Mms, Plan::Free));
    println!("MMS on Pro: {}", is_feature_enabled(Feature::Mms, Plan::Pro));
    println!("Webhooks on Basic: {}", is_feature_enabled(Feature::Webhooks, Plan::Basic));
    println!("Webhooks on Pro: {}", is_feature_enabled(Feature::Webhooks, Plan::Pro));
    println!("Analytics on Enterprise: {}", is_feature_enabled(Feature::Analytics, Plan::Enterprise));

    // Test categorize_http_status
    println!("\n=== categorize_http_status ===");
    println!("100: {:?}", categorize_http_status(100));
    println!("200: {:?}", categorize_http_status(200));
    println!("301: {:?}", categorize_http_status(301));
    println!("404: {:?}", categorize_http_status(404));
    println!("500: {:?}", categorize_http_status(500));
    println!("999: {:?}", categorize_http_status(999));

    // Test analyze_results
    println!("\n=== analyze_results ===");
    let results = vec![
        DeliveryResult::Success { id: String::from("1"), segments: 1 },
        DeliveryResult::Failed { code: 500, reason: String::from("Error") },
        DeliveryResult::Success { id: String::from("2"), segments: 2 },
        DeliveryResult::Pending { retry_count: 1 },
    ];
    let summary = analyze_results(&results);
    println!("Summary: {:?}", summary);

    // Test describe_slice
    println!("\n=== describe_slice ===");
    println!("[]: {}", describe_slice(&[]));
    println!("[1]: {}", describe_slice(&[1]));
    println!("[1, 2]: {}", describe_slice(&[1, 2]));
    println!("[1, 2, 3]: {}", describe_slice(&[1, 2, 3]));
    println!("[1, 2, 3, 4, 5]: {}", describe_slice(&[1, 2, 3, 4, 5]));

    // Test extract_all_timestamps
    println!("\n=== extract_all_timestamps ===");
    let statuses = vec![
        MessageStatus::Pending,
        MessageStatus::Delivered { timestamp: 1000 },
        MessageStatus::Sent,
        MessageStatus::Delivered { timestamp: 2000 },
    ];
    println!("Timestamps: {:?}", extract_all_timestamps(&statuses));

    // Test status_to_priority
    println!("\n=== status_to_priority ===");
    println!("Pending: {:?}", status_to_priority(&MessageStatus::Pending));
    println!("Sent: {:?}", status_to_priority(&MessageStatus::Sent));
    println!("Delivered: {:?}", status_to_priority(&MessageStatus::Delivered { timestamp: 0 }));
    println!("Failed: {:?}", status_to_priority(&MessageStatus::Failed { code: 0, reason: String::new() }));

    // Test validate_transition
    println!("\n=== validate_transition ===");
    let pending = MessageStatus::Pending;
    let sent = MessageStatus::Sent;
    let delivered = MessageStatus::Delivered { timestamp: 0 };
    
    println!("Pending -> Sent: {:?}", validate_transition(&pending, &sent));
    println!("Pending -> Delivered: {:?}", validate_transition(&pending, &delivered));
    println!("Sent -> Delivered: {:?}", validate_transition(&sent, &delivered));
    println!("Delivered -> Sent: {:?}", validate_transition(&delivered, &sent));

    // Test summarize_plan_features
    println!("\n=== summarize_plan_features ===");
    println!("Free: {:?}", summarize_plan_features(Plan::Free));
    println!("Basic: {:?}", summarize_plan_features(Plan::Basic));
    println!("Pro: {:?}", summarize_plan_features(Plan::Pro));
    println!("Enterprise: {:?}", summarize_plan_features(Plan::Enterprise));
}
