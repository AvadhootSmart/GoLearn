# Phase 4: Capstone Projects - Implementation Guide

This document outlines the structure and content requirements for the final 4 modules of the Rust course. These are **capstone projects** that integrate all concepts learned in Phases 1-3.

---

## Module 23: Web Backend with Axum

### Overview
Build a real REST API backend using Axum framework. Students will create a working SMS API server.

### Directory Structure
```
23-web_backend/
├── exercises/
│   ├── 1-axum_basics/
│   │   ├── readme.md
│   │   ├── code.rs
│   │   ├── complete.rs
│   │   └── expected.txt
│   ├── 2-routing/
│   │   ├── readme.md
│   │   ├── code.rs
│   │   ├── complete.rs
│   │   └── expected.txt
│   ├── 3-handlers/
│   │   ├── readme.md
│   │   ├── code.rs
│   │   ├── complete.rs
│   │   └── expected.txt
│   └── 4-middleware/
│       ├── readme.md
│       ├── code.rs
│       ├── complete.rs
│       └── expected.txt
```

### Exercise 1: Axum Basics
**Topics**: Router, basic handlers, hello world, running server

**Key Concepts**:
- `Router::new()`
- `.route("/", get(handler))`
- `axum::response::IntoResponse`
- `#[tokio::main]`
- `axum::Server::bind()`

**Example Code**:
```rust
use axum::{Router, routing::get, response::IntoResponse};

async fn hello() -> impl IntoResponse {
    "Hello from Textio API"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Exercise 2: Routing
**Topics**: Path parameters, query parameters, HTTP methods

**Key Concepts**:
- `Path<T>` extractor
- `Query<T>` extractor
- Multiple routes
- POST/PUT/DELETE methods

**Endpoints**:
- `GET /api/messages/:id`
- `GET /api/messages?limit=10`
- `POST /api/messages`

### Exercise 3: Handlers
**Topics**: JSON extraction, state sharing, error handling

**Key Concepts**:
- `Json<T>` extractor
- `State<S>` extractor
- `Arc<AppState>`
- Custom error types implementing `IntoResponse`
- Response status codes

**Structs**:
```rust
#[derive(Deserialize)]
struct SendMessage {
    to: String,
    body: String,
}

#[derive(Serialize)]
struct MessageResponse {
    id: String,
    status: String,
}
```

### Exercise 4: Middleware
**Topics**: Tower middleware, layers, CORS, logging

**Key Concepts**:
- `axum::middleware`
- `tower::ServiceBuilder`
- `tower_http::CorsLayer`
- `tower_http::TraceLayer`
- Custom middleware
- Request ID generation

**Dependencies**:
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## Module 24: FFI & Unsafe

### Overview
Learn to interface Rust with C code, essential for Tauri apps and system programming.

### Directory Structure
```
24-ffi_unsafe/
├── exercises/
│   ├── 1-unsafe_basics/
│   ├── 2-calling_c/
│   ├── 3-ffi_patterns/
│   └── 4-safe_wrappers/
```

### Exercise 1: Unsafe Basics
**Topics**: unsafe blocks, raw pointers, unsafe functions

**Key Concepts**:
- When to use unsafe
- Five unsafe superpowers:
  1. Dereference raw pointers
  2. Call unsafe functions
  3. Access or modify mutable static variables
  4. Access fields of unions
  5. Implement unsafe traits
- `*const T` and `*mut T`
- `unsafe fn` vs `unsafe {}` block

**Example**:
```rust
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1 = {}", *r1);
        *r2 = 10;
    }
}
```

### Exercise 2: Calling C
**Topics**: FFI, extern "C", linking C libraries

**Key Concepts**:
- `extern "C"` block
- `#[link(name = "...")]`
- `bindgen` for generating bindings
- `cbindgen` for generating C headers
- C types: `c_int`, `c_char`, `c_void`

**Example**:
```rust
#[link(name = "c")]
extern "C" {
    fn abs(x: i32) -> i32;
    fn strlen(s: *const i8) -> usize;
}

fn main() {
    unsafe {
        println!("abs(-5) = {}", abs(-5));
    }
}
```

### Exercise 3: FFI Patterns
**Topics**: Passing strings, callbacks, opaque types, cdylib

**Key Concepts**:
- `CString` and `CStr`
- `Box::into_raw()` and `Box::from_raw()`
- Callbacks with `extern "C" fn`
- Opaque types (`*mut c_void`)
- `#[no_mangle]`
- `extern "C"` for exported functions
- `cdylib` vs `staticlib`

**Example**:
```rust
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_greeting(name: *const c_char) -> *mut c_char {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str().unwrap();
    let greeting = format!("Hello, {}!", name_str);
    CString::new(greeting).unwrap().into_raw()
}
```

### Exercise 4: Safe Wrappers
**Topics**: Creating safe APIs over unsafe code

**Key Concepts**:
- Safety invariants
- Documentation with `# Safety`
- RAII patterns for FFI resources
- Interior mutability for callbacks
- Drop implementations

**Example Pattern**:
```rust
/// Safe wrapper around C library
pub struct EncryptionKey {
    ptr: *mut c_void,
}

impl EncryptionKey {
    pub fn new(key: &[u8]) -> Result<Self, Error> {
        let ptr = unsafe { c_encrypt_new(key.as_ptr(), key.len()) };
        if ptr.is_null() {
            Err(Error::KeyGeneration)
        } else {
            Ok(Self { ptr })
        }
    }
    
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Safe because we maintain the invariant that ptr is valid
        unsafe { ... }
    }
}

impl Drop for EncryptionKey {
    fn drop(&mut self) {
        unsafe { c_encrypt_free(self.ptr) };
    }
}
```

---

## Module 25: CLI Tool Project

### Overview
Build a complete CLI tool for Textio SMS management. This integrates: structs, enums, error handling, file I/O, serde, clap, async.

### Directory Structure
```
25-cli_tool_project/
├── exercises/
│   ├── 1-project_setup/
│   ├── 2-core_logic/
│   ├── 3-cli_interface/
│   └── 4-integration/
```

### Final Project Structure
```
textio-cli/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point, clap parsing
│   ├── lib.rs            # Library root
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── send.rs       # Send SMS command
│   │   ├── contacts.rs   # Contact management
│   │   ├── config.rs     # Config management
│   │   └── history.rs    # Message history
│   ├── models/
│   │   ├── mod.rs
│   │   ├── message.rs    # Message struct
│   │   └── contact.rs    # Contact struct
│   ├── config.rs         # Config loading
│   ├── api.rs            # HTTP client for API
│   └── error.rs          # Error types
├── tests/
│   └── integration.rs
└── README.md
```

### Exercise 1: Project Setup
**Topics**: Project structure, dependencies, module organization

**Dependencies**:
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
anyhow = "1"
thiserror = "1"
colored = "2"
indicatif = "0.17"
toml = "0.8"
dirs = "5"
tracing = "0.1"
tracing-subscriber = "0.3"
```

### Exercise 2: Core Logic
**Topics**: Business logic without CLI concerns

**Key Components**:
- `Message` struct with validation
- `Contact` struct with phone validation
- `MessageStore` for history
- `ContactStore` for contacts
- `ApiClient` for HTTP requests

**Example**:
```rust
pub struct Message {
    pub id: String,
    pub to: String,
    pub body: String,
    pub status: MessageStatus,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(to: String, body: String) -> Result<Self, ValidationError> {
        Self::validate_phone(&to)?;
        Self::validate_body(&body)?;
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            to,
            body,
            status: MessageStatus::Pending,
            created_at: Utc::now(),
        })
    }
}
```

### Exercise 3: CLI Interface
**Topics**: Clap subcommands, colored output, progress bars

**Commands**:
```bash
textio send +1234567890 "Hello world"
textio send --from contacts.json --template "Hi {name}"
textio contacts add +1234567890 "John Doe"
textio contacts list
textio contacts remove +1234567890
textio config set api-key sk_xxx
textio history --limit 20
textio status <message-id>
```

**Example**:
```rust
#[derive(Parser)]
#[command(name = "textio")]
#[command(about = "SMS API CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Send {
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        message: String,
    },
    Contacts {
        #[command(subcommand)]
        action: ContactCommands,
    },
    Config {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
    },
    History {
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}
```

### Exercise 4: Integration
**Topics**: Config files, logging, tests, full integration

**Config File** (`~/.textio/config.toml`):
```toml
[api]
base_url = "https://api.textio.io"
api_key = "sk_xxx"
timeout_seconds = 30

[defaults]
sender_id = "Textio"
retry_count = 3

[logging]
level = "info"
file = "~/.textio/textio.log"
```

**Features**:
- Load config from file
- Override with CLI flags
- Colored error/success output
- Progress bars for batch operations
- JSON output mode (`--json`)
- Verbose mode (`-v`)
- Tests for all commands

---

## Module 26: API Project

### Overview
Build a complete async REST API backend. This is the final capstone integrating everything.

### Directory Structure
```
26-api_project/
├── exercises/
│   ├── 1-project_setup/
│   ├── 2-database_layer/
│   ├── 3-api_routes/
│   └── 4-error_handling_async/
```

### Final Project Structure
```
textio-api/
├── Cargo.toml
├── src/
│   ├── main.rs           # Server startup
│   ├── lib.rs
│   ├── config.rs         # Configuration
│   ├── error.rs          # Error types
│   ├── db/
│   │   ├── mod.rs
│   │   ├── pool.rs       # Connection pool
│   │   └── models.rs     # DB models
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── messages.rs   # Message endpoints
│   │   ├── contacts.rs   # Contact endpoints
│   │   └── health.rs     # Health check
│   ├── models/
│   │   ├── mod.rs
│   │   ├── message.rs
│   │   └── contact.rs
│   └── middleware/
│       ├── mod.rs
│       ├── auth.rs       # API key auth
│       └── logging.rs    # Request logging
├── migrations/           # SQL migrations (if using sqlx)
│   └── 001_init.sql
├── tests/
│   └── api_tests.rs
└── README.md
```

### Dependencies
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "limit"] }
thiserror = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

### Exercise 1: Project Setup
**Topics**: Project structure, config, basic server

**Config**:
```rust
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_address: String,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:textio.db".to_string()),
            bind_address: env::var("BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            api_key: env::var("API_KEY")
                .expect("API_KEY must be set"),
        })
    }
}
```

### Exercise 2: Database Layer
**Topics**: Database models, connection pool, CRUD operations

**Using sqlx**:
```rust
use sqlx::SqlitePool;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Message {
    pub id: String,
    pub to_phone: String,
    pub body: String,
    pub status: String,
    pub created_at: i64,
}

pub async fn create_message(
    pool: &SqlitePool,
    to: &str,
    body: &str,
) -> Result<Message, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp();
    
    sqlx::query_as!(
        Message,
        r#"
        INSERT INTO messages (id, to_phone, body, status, created_at)
        VALUES (?, ?, ?, 'pending', ?)
        RETURNING *
        "#,
        id, to, body, now
    )
    .fetch_one(pool)
    .await
}
```

**Alternative: In-memory mock**:
```rust
pub struct Database {
    messages: Arc<RwLock<HashMap<String, Message>>>,
    contacts: Arc<RwLock<HashMap<String, Contact>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(RwLock::new(HashMap::new())),
            contacts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
```

### Exercise 3: API Routes
**Topics**: Axum routes, handlers, JSON, pagination

**Endpoints**:
```
POST   /api/messages              - Send new SMS
GET    /api/messages              - List messages (paginated)
GET    /api/messages/:id          - Get message by ID
DELETE /api/messages/:id          - Cancel message

POST   /api/contacts              - Add contact
GET    /api/contacts              - List contacts
GET    /api/contacts/:id          - Get contact
PUT    /api/contacts/:id          - Update contact
DELETE /api/contacts/:id          - Delete contact

GET    /health                    - Health check
```

**Handler Example**:
```rust
pub async fn create_message(
    State(db): State<Arc<Database>>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<MessageResponse>, ApiError> {
    let message = db.create_message(&payload.to, &payload.body).await?;
    Ok(Json(MessageResponse::from(message)))
}

pub async fn list_messages(
    State(db): State<Arc<Database>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<MessageResponse>>, ApiError> {
    let messages = db.list_messages(params.limit, params.offset).await?;
    Ok(Json(messages.into_iter().map(MessageResponse::from).collect()))
}
```

### Exercise 4: Error Handling & Async
**Topics**: Error types, middleware, logging, graceful shutdown

**Error Types**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            Self::Internal(e) => {
                tracing::error!("Internal error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        
        let body = Json(serde_json::json!({
            "error": message,
        }));
        
        (status, body).into_response()
    }
}
```

**Graceful Shutdown**:
```rust
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    
    tracing::info!("Shutdown signal received");
}
```

---

## Expected Output Format

Each `expected.txt` should include:
1. CLI commands to test (curl examples)
2. Expected JSON responses
3. HTTP status codes
4. Example error responses

**Example for Module 26**:
```
# Health check
$ curl http://localhost:3000/health
{"status":"ok"}

# Create message
$ curl -X POST http://localhost:3000/api/messages \
  -H "Content-Type: application/json" \
  -H "X-API-Key: test-key" \
  -d '{"to":"+1234567890","body":"Hello world"}'
{"id":"550e8400-e29b-41d4-a716-446655440000","to":"+1234567890","body":"Hello world","status":"pending"}

# List messages
$ curl http://localhost:3000/api/messages?limit=10 \
  -H "X-API-Key: test-key"
[{"id":"...","to":"+1234567890","body":"Hello world","status":"pending"}]

# Validation error
$ curl -X POST http://localhost:3000/api/messages \
  -H "Content-Type: application/json" \
  -H "X-API-Key: test-key" \
  -d '{"to":"invalid","body":""}'
HTTP/1.1 400 Bad Request
{"error":"Invalid phone number format"}
```

---

## Summary

| Module | Type | Key Technologies |
|--------|------|------------------|
| 23 | Web Backend | Axum, Tower, async |
| 24 | FFI/Unsafe | unsafe, extern "C", cdylib |
| 25 | CLI Tool | clap, serde, anyhow, indicatif |
| 26 | API Project | Axum, sqlx, tokio, async patterns |

Each capstone should be **portfolio-ready** code demonstrating professional Rust development practices.
