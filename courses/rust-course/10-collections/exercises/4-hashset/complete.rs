// HashSet Exercise - Textio SMS API - Complete Solution

use std::collections::HashSet;

pub struct PhoneRegistry {
    registered: HashSet<String>,
    blocked: HashSet<String>,
    premium: HashSet<String>,
}

impl PhoneRegistry {
    pub fn new() -> Self {
        PhoneRegistry {
            registered: HashSet::new(),
            blocked: HashSet::new(),
            premium: HashSet::new(),
        }
    }

    pub fn register(&mut self, phone: String) -> bool {
        self.registered.insert(phone)
    }

    pub fn unregister(&mut self, phone: &str) -> bool {
        self.registered.remove(phone)
    }

    pub fn is_registered(&self, phone: &str) -> bool {
        self.registered.contains(phone)
    }

    pub fn block(&mut self, phone: String) {
        self.blocked.insert(phone);
    }

    pub fn unblock(&mut self, phone: &str) {
        self.blocked.remove(phone);
    }

    pub fn is_blocked(&self, phone: &str) -> bool {
        self.blocked.contains(phone)
    }

    pub fn mark_premium(&mut self, phone: String) {
        self.premium.insert(phone);
    }

    pub fn is_premium(&self, phone: &str) -> bool {
        self.premium.contains(phone)
    }

    pub fn count(&self) -> usize {
        self.registered.len()
    }

    pub fn get_reachable(&self) -> HashSet<&String> {
        self.registered.difference(&self.blocked).collect()
    }

    pub fn get_blocked_registered(&self) -> HashSet<&String> {
        self.registered.intersection(&self.blocked).collect()
    }

    pub fn get_blocked_unregistered(&self) -> HashSet<&String> {
        self.blocked.difference(&self.registered).collect()
    }

    pub fn all_registered(&self, phones: &HashSet<String>) -> bool {
        self.registered.is_superset(phones)
    }

    pub fn none_registered(&self, phones: &HashSet<String>) -> bool {
        self.registered.is_disjoint(phones)
    }

    pub fn merge(&mut self, other: HashSet<String>) {
        self.registered.extend(other);
    }

    pub fn get_active_premium(&self) -> HashSet<&String> {
        self.premium.difference(&self.blocked).collect()
    }
}

pub fn unique_phones<'a>(lists: &[&'a [&'a str]]) -> HashSet<&'a str> {
    lists.iter().flat_map(|l| l.iter().copied()).collect()
}

pub fn common_phones<'a>(lists: &[&'a [&'a str]]) -> HashSet<&'a str> {
    if lists.is_empty() {
        return HashSet::new();
    }
    
    let mut result: HashSet<&'a str> = lists[0].iter().copied().collect();
    for list in &lists[1..] {
        result = result.intersection(&list.iter().copied().collect()).copied().collect();
    }
    result
}

pub fn unique_to_single_list<'a>(lists: &[&'a [&'a str]]) -> HashSet<&'a str> {
    use std::collections::HashMap;
    let mut counts: HashMap<&'a str, usize> = HashMap::new();
    
    for list in lists {
        let seen: HashSet<&'a str> = list.iter().copied().collect();
        for phone in seen {
            *counts.entry(phone).or_insert(0) += 1;
        }
    }
    
    counts.into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(phone, _)| phone)
        .collect()
}

pub fn deduplicate_phones(phones: Vec<String>) -> Vec<String> {
    let set: HashSet<String> = phones.into_iter().collect();
    set.into_iter().collect()
}

fn main() {
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
    
    println!("\nBlocking numbers...");
    registry.block("+2222222222".to_string());
    registry.block("+4444444444".to_string());
    
    println!("Is +2222222222 blocked: {}", registry.is_blocked("+2222222222"));
    println!("Is +1111111111 blocked: {}", registry.is_blocked("+1111111111"));
    
    registry.mark_premium("+1111111111".to_string());
    registry.mark_premium("+3333333333".to_string());
    registry.block("+3333333333".to_string());
    
    println!("\nPremium status:");
    println!("  +1111111111: {}", registry.is_premium("+1111111111"));
    println!("  +2222222222: {}", registry.is_premium("+2222222222"));
    
    println!("\nReachable phones: {:?}", registry.get_reachable());
    println!("Blocked and registered: {:?}", registry.get_blocked_registered());
    println!("Blocked but unregistered: {:?}", registry.get_blocked_unregistered());
    println!("Active premium: {:?}", registry.get_active_premium());
    
    let check_set: HashSet<String> = ["+1111111111".to_string(), "+2222222222".to_string()]
        .into_iter().collect();
    println!("\nAre [+1111111111, +2222222222] all registered: {}", registry.all_registered(&check_set));
    
    let check_set2: HashSet<String> = ["+9999999999".to_string(), "+8888888888".to_string()]
        .into_iter().collect();
    println!("Are [+9999999999, +8888888888] all unregistered: {}", registry.none_registered(&check_set2));
    
    let new_phones: HashSet<String> = ["+5555555555".to_string(), "+6666666666".to_string()]
        .into_iter().collect();
    registry.merge(new_phones);
    println!("\nAfter merge, total: {}", registry.count());
    
    let list1 = ["+111", "+222", "+333"];
    let list2 = ["+222", "+333", "+444"];
    let list3 = ["+333", "+444", "+555"];
    let all_lists = [&list1[..], &list2[..], &list3[..]];
    
    let unique = unique_phones(&all_lists);
    println!("\nUnique phones from all lists: {:?}", unique);
    
    let common = common_phones(&all_lists);
    println!("Common phones in all lists: {:?}", common);
    
    let exclusive = unique_to_single_list(&all_lists);
    println!("Phones in exactly one list: {:?}", exclusive);
    
    let dupes = vec![
        "+111".to_string(), "+222".to_string(), "+111".to_string(),
        "+333".to_string(), "+222".to_string(), "+444".to_string()
    ];
    let deduped = deduplicate_phones(dupes);
    println!("Deduplicated: {:?}", deduped);
}
