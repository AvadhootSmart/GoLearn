// Textio Multiple Type Parameters, Constraints, and PhantomData Exercise - Complete Solution
// Advanced generic patterns for Textio's SMS API

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

impl<K, V> KeyValue<K, V> {
    fn new(key: K, value: V) -> Self {
        KeyValue { key, value }
    }
    
    fn key(&self) -> &K {
        &self.key
    }
    
    fn value(&self) -> &V {
        &self.value
    }
}

struct Mapping<K, V> {
    entries: Vec<KeyValue<K, V>>,
}

impl<K: PartialEq, V: Clone> Mapping<K, V> {
    fn new() -> Self {
        Mapping { entries: Vec::new() }
    }
    
    fn insert(&mut self, key: K, value: V) {
        self.entries.push(KeyValue::new(key, value));
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        self.entries.iter()
            .find(|kv| &kv.key == key)
            .map(|kv| &kv.value)
    }
    
    fn contains_key(&self, key: &K) -> bool {
        self.entries.iter().any(|kv| &kv.key == key)
    }
    
    fn len(&self) -> usize {
        self.entries.len()
    }
}

struct Draft;
struct Validated;
struct Queued;
struct Sent;

struct StatefulMessage<T, State> {
    id: u64,
    content: T,
    _state: PhantomData<State>,
}

impl<T: Clone + Debug> StatefulMessage<T, Draft> {
    fn new(id: u64, content: T) -> Self {
        StatefulMessage {
            id,
            content,
            _state: PhantomData,
        }
    }
}

impl StatefulMessage<String, Draft> {
    fn validate(self) -> Result<StatefulMessage<String, Validated>, String> {
        if self.content.len() > 160 {
            Err("Message too long".to_string())
        } else if self.content.is_empty() {
            Err("Message cannot be empty".to_string())
        } else {
            Ok(StatefulMessage {
                id: self.id,
                content: self.content,
                _state: PhantomData,
            })
        }
    }
}

impl<T: Clone> StatefulMessage<T, Validated> {
    fn queue(self) -> StatefulMessage<T, Queued> {
        StatefulMessage {
            id: self.id,
            content: self.content,
            _state: PhantomData,
        }
    }
    
    fn content(&self) -> &T {
        &self.content
    }
}

impl<T: Debug> StatefulMessage<T, Queued> {
    fn send(self) -> StatefulMessage<T, Sent> {
        println!("Sending message {:?}...", self.content);
        StatefulMessage {
            id: self.id,
            content: self.content,
            _state: PhantomData,
        }
    }
}

impl<T> StatefulMessage<T, Sent> {
    fn id(&self) -> u64 {
        self.id
    }
    
    fn into_content(self) -> T {
        self.content
    }
}

struct Channel<Id, Message> {
    id: Id,
    name: String,
    _marker: PhantomData<Message>,
}

impl<Id: Clone + Debug, Message> Channel<Id, Message> {
    fn new(id: Id, name: String) -> Self {
        Channel {
            id,
            name,
            _marker: PhantomData,
        }
    }
    
    fn id(&self) -> &Id {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

struct Processor<Input, Output, Error> {
    name: String,
    processed_count: u64,
    _input: PhantomData<Input>,
    _output: PhantomData<Output>,
    _error: PhantomData<Error>,
}

impl<Input, Output, Error> Processor<Input, Output, Error>
where
    Input: Debug + Clone,
    Output: Display,
    Error: Debug,
{
    fn new(name: String) -> Self {
        Processor {
            name,
            processed_count: 0,
            _input: PhantomData,
            _output: PhantomData,
            _error: PhantomData,
        }
    }
    
    fn process<F>(&mut self, input: Input, transformer: F) -> Result<Output, Error>
    where
        F: FnOnce(Input) -> Result<Output, Error>,
    {
        println!("Processor '{}' processing: {:?}", self.name, input);
        self.processed_count += 1;
        transformer(input)
    }
    
    fn processed_count(&self) -> u64 {
        self.processed_count
    }
}

struct ServiceBuilder<Config = (), Transport = ()> {
    config: Option<Config>,
    transport: Option<Transport>,
}

impl<Config, Transport> ServiceBuilder<Config, Transport> {
    fn new() -> Self {
        ServiceBuilder {
            config: None,
            transport: None,
        }
    }
    
    fn with_config<C>(self, config: C) -> ServiceBuilder<C, Transport> {
        ServiceBuilder {
            config: Some(config),
            transport: self.transport,
        }
    }
    
    fn with_transport<T>(self, transport: T) -> ServiceBuilder<Config, T> {
        ServiceBuilder {
            config: self.config,
            transport: Some(transport),
        }
    }
}

impl<Config: Debug, Transport: Debug> ServiceBuilder<Config, Transport> {
    fn build(self) -> String {
        format!(
            "Service(config: {:?}, transport: {:?})",
            self.config, self.transport
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SmsMessage {
    content: String,
    recipient: String,
}

#[derive(Debug, Clone)]
struct EmailMessage {
    subject: String,
    body: String,
    recipient: String,
}

#[derive(Debug, Clone)]
struct ProcessError(String);

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProcessError: {}", self.0)
    }
}

fn main() {
    println!("=== KeyValue Tests ===");
    
    let kv1: KeyValue<u32, String> = KeyValue::new(1, "first".to_string());
    println!("Key: {}, Value: {}", kv1.key(), kv1.value());
    
    let kv2: KeyValue<String, SmsMessage> = KeyValue::new(
        "msg-001".to_string(),
        SmsMessage {
            content: "Hello".to_string(),
            recipient: "+1234567890".to_string(),
        },
    );
    println!("Key: {}, Recipient: {}", kv2.key(), kv2.value().recipient);

    println!("\n=== Mapping Tests ===");
    
    let mut mapping: Mapping<String, i32> = Mapping::new();
    mapping.insert("one".to_string(), 1);
    mapping.insert("two".to_string(), 2);
    mapping.insert("three".to_string(), 3);
    
    println!("Length: {}", mapping.len());
    println!("Get 'two': {:?}", mapping.get(&"two".to_string()));
    println!("Contains 'four': {}", mapping.contains_key(&"four".to_string()));

    println!("\n=== StatefulMessage Tests ===");
    
    let draft = StatefulMessage::<String, Draft>::new(1, "Hello World".to_string());
    println!("Created draft message with id 1");
    
    let validated = draft.validate().expect("Validation should succeed");
    println!("Message validated");
    
    let queued = validated.queue();
    println!("Message queued");
    
    let sent = queued.send();
    println!("Message sent! ID: {}", sent.id());

    println!("\n=== Channel Tests ===");
    
    let sms_channel: Channel<u32, SmsMessage> = Channel::new(1, "sms-primary".to_string());
    println!("Channel ID: {:?}, Name: {}", sms_channel.id(), sms_channel.name());
    
    let email_channel: Channel<String, EmailMessage> = Channel::new("email-001".to_string(), "email-main".to_string());
    println!("Channel ID: {:?}, Name: {}", email_channel.id(), email_channel.name());

    println!("\n=== Processor Tests ===");
    
    let mut processor: Processor<String, i32, ProcessError> = Processor::new("text-processor".to_string());
    
    let result1 = processor.process("42".to_string(), |s| {
        s.parse::<i32>().map_err(|e| ProcessError(e.to_string()))
    });
    println!("Process '42': {:?}", result1);
    println!("Processed count: {}", processor.processed_count());
    
    let result2 = processor.process("not a number".to_string(), |s| {
        s.parse::<i32>().map_err(|e| ProcessError(e.to_string()))
    });
    println!("Process 'not a number': {:?}", result2);
    println!("Processed count: {}", processor.processed_count());

    println!("\n=== ServiceBuilder Tests ===");
    
    let builder = ServiceBuilder::new()
        .with_config("production")
        .with_transport("https");
    
    let service_info = builder.build();
    println!("Built service: {}", service_info);
    
    let minimal_builder = ServiceBuilder::new();
    let minimal_service = minimal_builder.build();
    println!("Minimal service: {}", minimal_service);
}
