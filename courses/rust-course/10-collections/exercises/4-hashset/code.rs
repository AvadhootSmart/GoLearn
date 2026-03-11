// HashSet Exercise - Textio SMS API
//
// Complete the functions below to work with HashSets in the context
// of an SMS messaging system.

use std::collections::HashSet;

/// Manages unique phone numbers for SMS campaigns
pub struct PhoneRegistry {
    registered: HashSet<String>,
    blocked: HashSet<String>,
    premium: HashSet<String>,
}

impl PhoneRegistry {
    /// Create a new phone registry
    pub fn new() -> Self {
        // Your code here
        todo!()
    }

    /// Register a new phone number
    /// TODO: Use insert - returns true if newly added
    pub fn register(&mut self, phone: String) -> bool {
        // Your code here
        todo!()
    }

    /// Remove a phone number from registry
    /// TODO: Use remove - returns true if it was present
    pub fn unregister(&mut self, phone: &str) -> bool {
        // Your code here
        todo!()
    }

    /// Check if a phone is registered
    /// TODO: Use contains
    pub fn is_registered(&self, phone: &str) -> bool {
        // Your code here
        todo!()
    }

    /// Block a phone number
    /// TODO: Use insert to add to blocked set
    pub fn block(&mut self, phone: String) {
        // Your code here
        todo!()
    }

    /// Unblock a phone number
    /// TODO: Use remove
    pub fn unblock(&mut self, phone: &str) {
        // Your code here
        todo!()
    }

    /// Check if a phone is blocked
    /// TODO: Use contains
    pub fn is_blocked(&self, phone: &str) -> bool {
        // Your code here
        todo!()
    }

    /// Mark a phone as premium subscriber
    /// TODO: Use insert
    pub fn mark_premium(&mut self, phone: String) {
        // Your code here
        todo!()
    }

    /// Check if a phone is premium
    /// TODO: Use contains
    pub fn is_premium(&self, phone: &str) -> bool {
        // Your code here
        todo!()
    }

    /// Get count of registered phones
    pub fn count(&self) -> usize {
        // Your code here
        todo!()
    }

    /// Get phones that can receive messages (registered but not blocked)
    /// TODO: Use difference to find registered phones not in blocked
    pub fn get_reachable(&self) -> HashSet<&String> {
        // Your code here
        todo!()
    }

    /// Get phones that are both blocked and registered
    /// TODO: Use intersection
    pub fn get_blocked_registered(&self) -> HashSet<&String> {
        // Your code here
        todo!()
    }

    /// Get phones that are blocked but NOT registered
    /// TODO: Use difference
    pub fn get_blocked_unregistered(&self) -> HashSet<&String> {
        // Your code here
        todo!()
    }

    /// Check if all phones in a list are registered
    /// TODO: Use is_superset
    pub fn all_registered(&self, phones: &HashSet<String>) -> bool {
        // Your code here
        todo!()
    }

    /// Check if none of the phones in a list are registered
    /// TODO: Use is_disjoint
    pub fn none_registered(&self, phones: &HashSet<String>) -> bool {
        // Your code here
        todo!()
    }

    /// Merge another registry's registered phones into this one
    /// TODO: Use extend to add all phones from the other set
    pub fn merge(&mut self, other: HashSet<String>) {
        // Your code here
        todo!()
    }

    /// Get premium phones that are not blocked
    /// TODO: Use difference
    pub fn get_active_premium(&self) -> HashSet<&String> {
        // Your code here
        todo!()
    }
}

/// Find unique phone numbers from multiple lists
/// TODO: Create a HashSet from the iterator
pub fn unique_phones<'a>(lists: &[&'a [&'a str]]) -> HashSet<&'a str> {
    // Your code here
    todo!()
}

/// Find phone numbers that appear in ALL lists
/// TODO: Use intersection of all sets
pub fn common_phones<'a>(lists: &[&'a [&'a str]]) -> HashSet<&'a str> {
    // Your code here
    todo!()
}

/// Find phone numbers that appear in exactly one list
/// TODO: Use symmetric_difference or count occurrences
pub fn unique_to_single_list<'a>(lists: &[&'a [&'a str]]) -> HashSet<&'a str> {
    // Your code here
    todo!()
}

/// Remove duplicate phone numbers from a vector
/// TODO: Use HashSet to deduplicate, return as Vec
pub fn deduplicate_phones(phones: Vec<String>) -> Vec<String> {
    // Your code here
    todo!()
}

fn main() {
    // Test PhoneRegistry
    let mut registry = PhoneRegistry::new();
    
    println!("Registering phones...");
    println!("  +1111111111: {}", registry.register("+1111111111".to_string()));
    println!("  +2222222222: {}", registry.register("+2222222222".to_string()));
    println!("  +3333333333: {}", registry.register("+3333333333".to_string()));
    println!("  +1111111111 (again): {}", registry.register("+1111111111".to_string()));
    
    println!("\nTotal registered: {}", registry.count());
    
    println!("\nChecking registration:");
    println!("  +1111111111: {}", registry.is_registered("+1111111111"));
    println!("  +9999999999: {}", registry.is_registered("+9999999999"));
    
    // Block some numbers
    println!("\nBlocking numbers...");
    registry.block("+2222222222".to_string());
    registry.block("+4444444444".to_string());  // Not registered
    
    println!("Is +2222222222 blocked: {}", registry.is_blocked("+2222222222"));
    println!("Is +1111111111 blocked: {}", registry.is_blocked("+1111111111"));
    
    // Mark premium
    registry.mark_premium("+1111111111".to_string());
    registry.mark_premium("+3333333333".to_string());
    registry.block("+3333333333".to_string());  // Premium but blocked
    
    println!("\nPremium status:");
    println!("  +1111111111: {}", registry.is_premium("+1111111111"));
    println!("  +2222222222: {}", registry.is_premium("+2222222222"));
    
    // Test set operations
    println!("\nReachable phones: {:?}", registry.get_reachable());
    println!("Blocked and registered: {:?}", registry.get_blocked_registered());
    println!("Blocked but unregistered: {:?}", registry.get_blocked_unregistered());
    println!("Active premium: {:?}", registry.get_active_premium());
    
    // Test subset operations
    let check_set: HashSet<String> = ["+1111111111".to_string(), "+2222222222".to_string()]
        .into_iter().collect();
    println!("\nAre [+1111111111, +2222222222] all registered: {}", registry.all_registered(&check_set));
    
    let check_set2: HashSet<String> = ["+9999999999".to_string(), "+8888888888".to_string()]
        .into_iter().collect();
    println!("Are [+9999999999, +8888888888] all unregistered: {}", registry.none_registered(&check_set2));
    
    // Test merge
    let new_phones: HashSet<String> = ["+5555555555".to_string(), "+6666666666".to_string()]
        .into_iter().collect();
    registry.merge(new_phones);
    println!("\nAfter merge, total: {}", registry.count());
    
    // Test unique_phones
    let list1 = ["+111", "+222", "+333"];
    let list2 = ["+222", "+333", "+444"];
    let list3 = ["+333", "+444", "+555"];
    let all_lists = [&list1[..], &list2[..], &list3[..]];
    
    let unique = unique_phones(&all_lists);
    println!("\nUnique phones from all lists: {:?}", unique);
    
    // Test common_phones
    let common = common_phones(&all_lists);
    println!("Common phones in all lists: {:?}", common);
    
    // Test unique_to_single_list
    let exclusive = unique_to_single_list(&all_lists);
    println!("Phones in exactly one list: {:?}", exclusive);
    
    // Test deduplicate
    let dupes = vec![
        "+111".to_string(), "+222".to_string(), "+111".to_string(),
        "+333".to_string(), "+222".to_string(), "+444".to_string()
    ];
    let deduped = deduplicate_phones(dupes);
    println!("Deduplicated: {:?}", deduped);
}
