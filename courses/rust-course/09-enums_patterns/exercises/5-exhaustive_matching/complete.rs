// Exercise 5: Exhaustive Matching - Complete Solution

use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalStatus {
    Queued,
    Processing,
    Completed,
    Failed,
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

impl fmt::Display for MessageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageStatus::Pending => write!(f, "Pending"),
            MessageStatus::Sent => write!(f, "Sent"),
            MessageStatus::Delivered { timestamp } => write!(f, "Delivered at {}", timestamp),
            MessageStatus::Failed { code, reason } => write!(f, "Failed ({}): {}", code, reason),
            MessageStatus::Cancelled { by_user } => {
                if *by_user {
                    write!(f, "Cancelled by user")
                } else {
                    write!(f, "Cancelled by system")
                }
            }
        }
    }
}

pub fn is_terminal(status: &MessageStatus) -> bool {
    matches!(status, MessageStatus::Delivered { .. } | MessageStatus::Failed { .. } | MessageStatus::Cancelled { .. })
}

pub fn can_retry(status: &MessageStatus) -> bool {
    match status {
        MessageStatus::Failed { code, .. } => *code < 500,
        MessageStatus::Pending => false,
        MessageStatus::Sent => false,
        MessageStatus::Delivered { .. } => false,
        MessageStatus::Cancelled { .. } => false,
    }
}

pub fn describe_external_status(status: &ExternalStatus) -> &'static str {
    match status {
        ExternalStatus::Queued => "Message is queued",
        ExternalStatus::Processing => "Message is being processed",
        ExternalStatus::Completed => "Message completed successfully",
        ExternalStatus::Failed => "Message processing failed",
        _ => "Unknown status",
    }
}

pub fn handle_delivery_result(result: &DeliveryResult) -> String {
    match result {
        DeliveryResult::Success { id, segments } => {
            format!("Message {} delivered successfully in {} segment(s)", id, segments)
        }
        DeliveryResult::PartialSuccess { id, delivered, total } => {
            format!("Message {} partially delivered: {}/{} segments", id, delivered, total)
        }
        DeliveryResult::Failed { code, reason } => {
            format!("Delivery failed (code {}): {}", code, reason)
        }
        DeliveryResult::Pending { retry_count } => {
            format!("Delivery pending (retry {})", retry_count)
        }
    }
}

pub fn count_segments(result: &DeliveryResult) -> Option<u8> {
    match result {
        DeliveryResult::Success { segments, .. } => Some(*segments),
        DeliveryResult::PartialSuccess { total, .. } => Some(*total),
        DeliveryResult::Failed { .. } => None,
        DeliveryResult::Pending { .. } => None,
    }
}

pub fn is_feature_enabled(feature: Feature, plan: Plan) -> bool {
    match (feature, plan) {
        (Feature::Sms, _) => true,
        (Feature::Mms, Plan::Free) => false,
        (Feature::Mms, Plan::Basic | Plan::Pro | Plan::Enterprise) => true,
        (Feature::ScheduledMessages, Plan::Pro | Plan::Enterprise) => true,
        (Feature::ScheduledMessages, Plan::Free | Plan::Basic) => false,
        (Feature::Webhooks, Plan::Pro | Plan::Enterprise) => true,
        (Feature::Webhooks, Plan::Free | Plan::Basic) => false,
        (Feature::Analytics, Plan::Enterprise) => true,
        (Feature::Analytics, Plan::Free | Plan::Basic | Plan::Pro) => false,
        (Feature::PrioritySupport, Plan::Enterprise) => true,
        (Feature::PrioritySupport, Plan::Free | Plan::Basic | Plan::Pro) => false,
    }
}

pub fn categorize_http_status(code: u16) -> HttpCategory {
    match code {
        100..=199 => HttpCategory::Informational,
        200..=299 => HttpCategory::Success,
        300..=399 => HttpCategory::Redirection,
        400..=499 => HttpCategory::ClientError,
        500..=599 => HttpCategory::ServerError,
        _ => HttpCategory::Unknown,
    }
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

pub fn analyze_results(results: &[DeliveryResult]) -> ResultSummary {
    let mut summary = ResultSummary {
        total: results.len(),
        successful: 0,
        failed: 0,
        pending: 0,
    };
    
    for result in results {
        match result {
            DeliveryResult::Success { .. } | DeliveryResult::PartialSuccess { .. } => {
                summary.successful += 1;
            }
            DeliveryResult::Failed { .. } => {
                summary.failed += 1;
            }
            DeliveryResult::Pending { .. } => {
                summary.pending += 1;
            }
        }
    }
    
    summary
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultSummary {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub pending: usize,
}

pub fn describe_slice(slice: &[i32]) -> &'static str {
    match slice {
        [] => "Empty",
        [_] => "One element",
        [_, _] => "Two elements",
        [_, _, _] => "Three elements",
        _ => "Many elements",
    }
}

pub fn extract_all_timestamps(statuses: &[MessageStatus]) -> Vec<u64> {
    statuses.iter().filter_map(|s| {
        match s {
            MessageStatus::Delivered { timestamp } => Some(*timestamp),
            _ => None,
        }
    }).collect()
}

pub fn status_to_priority(status: &MessageStatus) -> Priority {
    match status {
        MessageStatus::Pending => Priority::Low,
        MessageStatus::Sent => Priority::Normal,
        MessageStatus::Delivered { .. } => Priority::High,
        MessageStatus::Failed { .. } | MessageStatus::Cancelled { .. } => Priority::Critical,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

pub fn validate_transition(from: &MessageStatus, to: &MessageStatus) -> Result<(), String> {
    match (from, to) {
        // Pending can go to Sent or Cancelled
        (MessageStatus::Pending, MessageStatus::Sent) => Ok(()),
        (MessageStatus::Pending, MessageStatus::Cancelled { .. }) => Ok(()),
        
        // Sent can go to Delivered, Failed, or Cancelled
        (MessageStatus::Sent, MessageStatus::Delivered { .. }) => Ok(()),
        (MessageStatus::Sent, MessageStatus::Failed { .. }) => Ok(()),
        (MessageStatus::Sent, MessageStatus::Cancelled { .. }) => Ok(()),
        
        // Terminal states cannot transition
        (MessageStatus::Delivered { .. }, _) => Err(String::from("Cannot transition from Delivered")),
        (MessageStatus::Failed { .. }, _) => Err(String::from("Cannot transition from Failed")),
        (MessageStatus::Cancelled { .. }, _) => Err(String::from("Cannot transition from Cancelled")),
        
        // Invalid transitions
        (MessageStatus::Pending, MessageStatus::Delivered { .. }) => {
            Err(String::from("Cannot go from Pending to Delivered"))
        }
        (MessageStatus::Pending, MessageStatus::Failed { .. }) => {
            Err(String::from("Cannot go from Pending to Failed"))
        }
        (MessageStatus::Sent, MessageStatus::Pending) => {
            Err(String::from("Cannot go from Sent to Pending"))
        }
        (MessageStatus::Sent, MessageStatus::Sent) => {
            Err(String::from("Already in Sent state"))
        }
        (MessageStatus::Pending, MessageStatus::Pending) => {
            Err(String::from("Already in Pending state"))
        }
    }
}

pub fn summarize_plan_features(plan: Plan) -> Vec<&'static str> {
    let features = [Feature::Sms, Feature::Mms, Feature::ScheduledMessages, 
                    Feature::Webhooks, Feature::Analytics, Feature::PrioritySupport];
    
    let feature_names = ["SMS", "MMS", "Scheduled Messages", "Webhooks", "Analytics", "Priority Support"];
    
    features.iter()
        .zip(feature_names.iter())
        .filter(|(f, _)| is_feature_enabled(**f, plan))
        .map(|(_, name)| *name)
        .collect()
}

fn main() {
    println!("=== MessageStatus Display ===");
    println!("{}", MessageStatus::Pending);
    println!("{}", MessageStatus::Sent);
    println!("{}", MessageStatus::Delivered { timestamp: 1700000000 });
    println!("{}", MessageStatus::Failed { code: 500, reason: String::from("Timeout") });
    println!("{}", MessageStatus::Cancelled { by_user: true });

    println!("\n=== is_terminal ===");
    println!("Pending: {}", is_terminal(&MessageStatus::Pending));
    println!("Delivered: {}", is_terminal(&MessageStatus::Delivered { timestamp: 0 }));
    println!("Failed: {}", is_terminal(&MessageStatus::Failed { code: 0, reason: String::new() }));
    println!("Cancelled: {}", is_terminal(&MessageStatus::Cancelled { by_user: false }));

    println!("\n=== can_retry ===");
    let failed_400 = MessageStatus::Failed { code: 400, reason: String::from("Bad request") };
    let failed_500 = MessageStatus::Failed { code: 500, reason: String::from("Server error") };
    println!("Failed 400: {}", can_retry(&failed_400));
    println!("Failed 500: {}", can_retry(&failed_500));
    println!("Pending: {}", can_retry(&MessageStatus::Pending));

    println!("\n=== describe_external_status ===");
    println!("{:?}", describe_external_status(&ExternalStatus::Queued));
    println!("{:?}", describe_external_status(&ExternalStatus::Completed));

    println!("\n=== handle_delivery_result ===");
    let success = DeliveryResult::Success { id: String::from("msg_1"), segments: 1 };
    let partial = DeliveryResult::PartialSuccess { id: String::from("msg_2"), delivered: 2, total: 3 };
    let failed = DeliveryResult::Failed { code: 500, reason: String::from("Error") };
    let pending = DeliveryResult::Pending { retry_count: 1 };
    
    println!("Success: {}", handle_delivery_result(&success));
    println!("Partial: {}", handle_delivery_result(&partial));
    println!("Failed: {}", handle_delivery_result(&failed));
    println!("Pending: {}", handle_delivery_result(&pending));

    println!("\n=== count_segments ===");
    println!("Success: {:?}", count_segments(&success));
    println!("Partial: {:?}", count_segments(&partial));
    println!("Failed: {:?}", count_segments(&failed));

    println!("\n=== is_feature_enabled ===");
    println!("SMS on Free: {}", is_feature_enabled(Feature::Sms, Plan::Free));
    println!("MMS on Free: {}", is_feature_enabled(Feature::Mms, Plan::Free));
    println!("MMS on Pro: {}", is_feature_enabled(Feature::Mms, Plan::Pro));
    println!("Webhooks on Basic: {}", is_feature_enabled(Feature::Webhooks, Plan::Basic));
    println!("Webhooks on Pro: {}", is_feature_enabled(Feature::Webhooks, Plan::Pro));
    println!("Analytics on Enterprise: {}", is_feature_enabled(Feature::Analytics, Plan::Enterprise));

    println!("\n=== categorize_http_status ===");
    println!("100: {:?}", categorize_http_status(100));
    println!("200: {:?}", categorize_http_status(200));
    println!("301: {:?}", categorize_http_status(301));
    println!("404: {:?}", categorize_http_status(404));
    println!("500: {:?}", categorize_http_status(500));
    println!("999: {:?}", categorize_http_status(999));

    println!("\n=== analyze_results ===");
    let results = vec![
        DeliveryResult::Success { id: String::from("1"), segments: 1 },
        DeliveryResult::Failed { code: 500, reason: String::from("Error") },
        DeliveryResult::Success { id: String::from("2"), segments: 2 },
        DeliveryResult::Pending { retry_count: 1 },
    ];
    let summary = analyze_results(&results);
    println!("Summary: {:?}", summary);

    println!("\n=== describe_slice ===");
    println!("[]: {}", describe_slice(&[]));
    println!("[1]: {}", describe_slice(&[1]));
    println!("[1, 2]: {}", describe_slice(&[1, 2]));
    println!("[1, 2, 3]: {}", describe_slice(&[1, 2, 3]));
    println!("[1, 2, 3, 4, 5]: {}", describe_slice(&[1, 2, 3, 4, 5]));

    println!("\n=== extract_all_timestamps ===");
    let statuses = vec![
        MessageStatus::Pending,
        MessageStatus::Delivered { timestamp: 1000 },
        MessageStatus::Sent,
        MessageStatus::Delivered { timestamp: 2000 },
    ];
    println!("Timestamps: {:?}", extract_all_timestamps(&statuses));

    println!("\n=== status_to_priority ===");
    println!("Pending: {:?}", status_to_priority(&MessageStatus::Pending));
    println!("Sent: {:?}", status_to_priority(&MessageStatus::Sent));
    println!("Delivered: {:?}", status_to_priority(&MessageStatus::Delivered { timestamp: 0 }));
    println!("Failed: {:?}", status_to_priority(&MessageStatus::Failed { code: 0, reason: String::new() }));

    println!("\n=== validate_transition ===");
    let pending = MessageStatus::Pending;
    let sent = MessageStatus::Sent;
    let delivered = MessageStatus::Delivered { timestamp: 0 };
    
    println!("Pending -> Sent: {:?}", validate_transition(&pending, &sent));
    println!("Pending -> Delivered: {:?}", validate_transition(&pending, &delivered));
    println!("Sent -> Delivered: {:?}", validate_transition(&sent, &delivered));
    println!("Delivered -> Sent: {:?}", validate_transition(&delivered, &sent));

    println!("\n=== summarize_plan_features ===");
    println!("Free: {:?}", summarize_plan_features(Plan::Free));
    println!("Basic: {:?}", summarize_plan_features(Plan::Basic));
    println!("Pro: {:?}", summarize_plan_features(Plan::Pro));
    println!("Enterprise: {:?}", summarize_plan_features(Plan::Enterprise));
}
