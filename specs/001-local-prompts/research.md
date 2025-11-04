# Research: Local Prompts Implementation

**Feature**: 001-local-prompts  
**Date**: 2025-11-04  
**Status**: Complete

This document consolidates research findings for implementing local prompts support in the Twig MCP server. All technical unknowns from the implementation plan have been resolved.

---

## 1. YAML Frontmatter Parsing

### Decision
Use **`gray_matter` v0.3.2** for parsing YAML frontmatter from Markdown files.

### Rationale
- **Purpose-built**: Specifically designed for extracting frontmatter from documents, not general YAML parsing
- **Mature & well-maintained**: 50+ GitHub stars, recent release (July 2025), based on popular Node.js library
- **Excellent serde integration**: Native support for deserializing into custom Rust structs
- **Multi-format support**: Supports YAML, JSON, TOML with feature flags
- **Good error handling**: Uses `thiserror` for proper error types, returns `Result<T, Error>`
- **Rust 2024 compatible**: No MSRV issues, uses modern Rust practices

### Alternatives Considered
- **`markdown-frontmatter` v0.4.0**: Simpler but less mature, smaller community
- **`pulldown-cmark-frontmatter` v0.4.0**: Tightly coupled to pulldown-cmark, unconventional format
- **Direct YAML parsing with `serde_yaml_ng`**: Would require manual frontmatter extraction logic

### Implementation Pattern
```rust
use gray_matter::{Matter, engine::YAML};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PromptMetadata {
    title: Option<String>,
    description: Option<String>,
    arguments: Option<Vec<PromptArgument>>,
}

fn parse_prompt_file(content: &str) -> gray_matter::Result<(PromptMetadata, String)> {
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse_with_struct::<PromptMetadata>(content)?;
    
    Ok((
        parsed.data.unwrap_or_default(),
        parsed.content,
    ))
}
```

### Cargo Dependencies
```toml
[dependencies]
gray_matter = "0.3"
serde = { version = "1.0", features = ["derive"] }
```

---

## 2. Jinja Template Rendering

### Decision
Use **`minijinja` v2.12.0** for Jinja template rendering.

### Rationale
- **High Jinja2 compatibility**: Supports all core features including `{{ variable }}` substitution, filters, conditionals, loops
- **Minimal dependencies**: Only requires `serde` as core dependency, extremely lightweight
- **Performance optimized**: Designed for small templates with minimal overhead, used in production by HuggingFace, Cube.dev, PRQL
- **Excellent error handling**: Detailed, actionable error messages with context preservation
- **String template support**: Native support via `add_template()` and `add_raw_template()`, no file system dependency
- **Mature & well-maintained**: 2.3k GitHub stars, maintained by Armin Ronacher (creator of Jinja2/Flask), version 2.12.0 released Aug 2025
- **Strong ecosystem adoption**: 3.4k+ dependents on GitHub
- **Rust 2024 compatible**: MSRV 1.70

### Alternatives Considered
- **Tera v1.20.1**: Heavier dependency footprint, less active maintenance, requires very recent Rust (1.85)
- **Askama v0.14.0**: Compile-time templates only, does NOT support rendering from strings (dealbreaker)
- **Upon v0.10.0**: Smaller ecosystem, less Jinja2 compatibility, fewer features

### Implementation Pattern
```rust
use minijinja::{Environment, context};
use std::collections::HashMap;

fn render_prompt_template(
    template_content: &str,
    arguments: HashMap<String, serde_json::Value>,
) -> Result<String, minijinja::Error> {
    let mut env = Environment::new();
    
    // Add template from string
    env.add_template("prompt", template_content)?;
    
    // Get and render template
    let tmpl = env.get_template("prompt")?;
    let rendered = tmpl.render(context! {
        ..arguments  // Spread arguments into context
    })?;
    
    Ok(rendered)
}
```

### Error Handling
```rust
// Template syntax errors are caught at add_template time
match env.add_template("prompt", "Hello {{ unclosed") {
    Ok(_) => {},
    Err(e) => {
        // Error: syntax error: expected }}, got end of input
        eprintln!("Template error: {}", e);
    }
}
```

### Cargo Dependencies
```toml
[dependencies]
minijinja = "2.12.0"
```

---

## 3. File System Watching

### Decision
Use **`notify` v8.2.0 + `notify-debouncer-full` v0.6.0** for cross-platform file watching.

### Rationale

#### `notify` - Industry Standard
- **Maturity**: 62+ million downloads, 9.7M recent downloads
- **Active maintenance**: Latest v8.2.0, Rust 2024 compatible (MSRV 1.85)
- **Wide adoption**: Used by alacritty, cargo-watch, deno, rust-analyzer, mdBook
- **Excellent cross-platform support** with native backends:
  - Linux/Android: inotify
  - macOS: FSEvents (default) or kqueue
  - Windows: ReadDirectoryChangesW
  - iOS/BSD: kqueue
  - Fallback: polling watcher

#### `notify-debouncer-full` - Enhanced Event Processing
- **6+ million downloads**, 1.2M recent downloads
- **Intelligent event handling**:
  - Merges duplicate events
  - Stitches rename events together properly
  - Prevents spurious events (no Modify after Create)
  - Tracks file system IDs for better rename detection
  - Emits single Remove event for directory deletion

#### Tokio Compatibility
- Works perfectly with tokio through channel adapters
- Event handling doesn't block the tokio runtime
- Watcher runs in background threads, events forwarded to tokio tasks

### Alternatives Considered
- **watchexec (3.8M downloads)**: Overkill, built on notify anyway
- **hotwatch (328K downloads)**: Less mature, missing advanced debouncing
- **Custom polling**: Would require significant effort for cross-platform support

### Implementation Pattern
```rust
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use std::path::Path;
use std::time::Duration;
use tokio::sync::mpsc;

pub async fn watch_prompts_directory<P: AsRef<Path>>(
    path: P,
) -> Result<mpsc::UnboundedReceiver<DebounceEventResult>, Box<dyn std::error::Error>> {
    // Create tokio channel for async communication
    let (tx, rx) = mpsc::unbounded_channel();

    // Create debouncer with callback that forwards to tokio channel
    let mut debouncer = new_debouncer(
        Duration::from_secs(2), // debounce timeout (meets SC-003: <2s)
        None, // use default tick rate
        move |result: DebounceEventResult| {
            let _ = tx.send(result);
        },
    )?;

    // Start watching the directory
    debouncer.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    // Keep debouncer alive in background thread
    tokio::task::spawn_blocking(move || {
        // Keep the debouncer alive
        // In production, wire up graceful shutdown signal
        std::thread::park();
        drop(debouncer);
    });

    Ok(rx)
}

// Process events in MCP server
async fn handle_file_events(mut rx: mpsc::UnboundedReceiver<DebounceEventResult>) {
    while let Some(result) = rx.recv().await {
        match result {
            Ok(events) => {
                for event in events {
                    // event.kind: Create, Modify, Remove, Rename
                    // event.paths: affected file paths
                    
                    // Reload prompts and notify MCP clients
                    // server.peer().notify_prompts_list_changed(()).await?;
                }
            }
            Err(errors) => {
                for error in errors {
                    eprintln!("Watch error: {:?}", error);
                }
            }
        }
    }
}
```

### Key Implementation Considerations
1. **Lifecycle management**: Keep debouncer alive for server lifetime, dropping stops watching
2. **Debounce timing**: 2 seconds meets SC-003 requirement (<2 seconds for notifications)
3. **Event filtering**: Use `event.kind` to distinguish Create/Modify/Remove events
4. **Resource limits**: Linux inotify limits exist, but 100 files is well within defaults
5. **Recursive vs Non-recursive**: Use `RecursiveMode::NonRecursive` for flat directory structure

### Cargo Dependencies
```toml
[dependencies]
notify = "8.2"
notify-debouncer-full = "0.6"
```

---

## 4. Integration Testing with rmcp

### Decision
Use **in-memory duplex channels** (`tokio::io::duplex`) for integration tests, following rmcp's standard testing patterns.

### Rationale
- **Fast**: No process spawning overhead
- **Deterministic**: Full control over timing and lifecycle
- **Easy to debug**: Both client and server run in same process
- **Well-tested**: Used extensively in rmcp's own test suite (v0.8.3)
- **No special test utilities needed**: Standard tokio + rmcp APIs

### Testing Pattern for Prompts

```rust
use rmcp::{ServerHandler, ServiceExt, model::*};
use tokio::io::duplex;

#[tokio::test]
async fn test_list_prompts() -> anyhow::Result<()> {
    // 1. Create in-memory duplex channel
    let (server_transport, client_transport) = duplex(4096);

    // 2. Spawn server in separate task
    let server_handle = tokio::spawn(async move {
        let server = MyServer::new().serve(server_transport).await?;
        server.waiting().await?;
        anyhow::Ok(())
    });

    // 3. Create client
    let client = ().serve(client_transport).await?;

    // 4. Make requests
    let result = client.list_all_prompts().await?;
    
    // 5. Assert on results
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "code-review");

    // 6. Clean up
    client.cancel().await?;
    server_handle.await??;
    
    Ok(())
}
```

### Testing Prompt Retrieval with Arguments

```rust
#[tokio::test]
async fn test_get_prompt_with_arguments() -> anyhow::Result<()> {
    let (server_transport, client_transport) = duplex(4096);

    let server_handle = tokio::spawn(async move {
        MyServer::new().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });

    let client = ().serve(client_transport).await?;

    // Test prompt with parameters
    let result = client.get_prompt(GetPromptRequestParam {
        name: "code-review".into(),
        arguments: Some(
            serde_json::json!({
                "language": "Python",
                "style": "detailed"
            }).as_object().unwrap().clone()
        ),
    }).await?;

    // Verify template rendering
    assert_eq!(result.messages.len(), 1);
    match &result.messages[0].content {
        PromptMessageContent::Text { text } => {
            assert!(text.contains("Python"));
        }
        _ => panic!("Expected text content"),
    }

    client.cancel().await?;
    server_handle.await??;
    Ok(())
}
```

### Testing Notifications

```rust
use std::sync::Arc;
use tokio::sync::Notify;
use rmcp::{ClientHandler, service::NotificationContext, RoleClient};

#[derive(Clone)]
struct TestClient {
    notification_received: Arc<Notify>,
}

impl ClientHandler for TestClient {
    async fn on_prompts_list_changed(
        &self,
        _params: (),
        _context: NotificationContext<RoleClient>,
    ) {
        self.notification_received.notify_one();
    }
}

#[tokio::test]
async fn test_prompts_list_changed_notification() -> anyhow::Result<()> {
    let (server_transport, client_transport) = duplex(4096);
    let notification_received = Arc::new(Notify::new());
    
    let server_handle = tokio::spawn(async move {
        let server = MyServer::new().serve(server_transport).await?;
        // Simulate file change triggering notification
        server.peer().notify_prompts_list_changed(()).await?;
        server.waiting().await?;
        anyhow::Ok(())
    });

    let client = TestClient {
        notification_received: notification_received.clone(),
    }.serve(client_transport).await?;

    // Wait for notification (with timeout for safety)
    tokio::select! {
        _ = notification_received.notified() => {
            // Success!
        }
        _ = tokio::time::sleep(Duration::from_secs(5)) => {
            panic!("Notification not received within timeout");
        }
    }

    client.cancel().await?;
    server_handle.await??;
    Ok(())
}
```

### Best Practices
1. **Always use `#[tokio::test]`** for async tests
2. **Always call `client.cancel().await?`** before waiting for server to avoid deadlocks
3. **Use `anyhow::Result<()>`** for test return types
4. **Use `?` operator** for error propagation
5. **Increase buffer size** if testing large messages: `duplex(65536)`
6. **Use `Arc<Notify>`** for notification synchronization

### Alternative: Testing Built Binary

For end-to-end tests with the actual binary:

```rust
use rmcp::transport::{TokioChildProcess, ConfigureCommandExt};
use tokio::process::Command;

#[tokio::test]
async fn test_with_built_binary() -> anyhow::Result<()> {
    let transport = TokioChildProcess::new(
        Command::new("cargo").configure(|cmd| {
            cmd.arg("run").arg("--").arg("start");
        })
    )?;

    let client = ().serve(transport).await?;
    
    let prompts = client.list_all_prompts().await?;
    assert!(!prompts.is_empty());

    client.cancel().await?;
    Ok(())
}
```

### No Special Test Dependencies Required
rmcp provides all necessary testing APIs out of the box. The integration is seamless with tokio's test framework.

---

## Summary of Technical Decisions

| Component | Library | Version | Rationale |
|-----------|---------|---------|-----------|
| YAML Frontmatter | gray_matter | 0.3.2 | Purpose-built, excellent serde integration |
| Jinja Templates | minijinja | 2.12.0 | High compatibility, minimal deps, excellent errors |
| File Watching | notify + notify-debouncer-full | 8.2.0 + 0.6.0 | Industry standard, cross-platform, tokio-compatible |
| Integration Testing | tokio::io::duplex | (built-in) | Fast, deterministic, rmcp standard pattern |

All dependencies are:
- ✅ Actively maintained
- ✅ Rust 2024 compatible
- ✅ Production-tested
- ✅ Well-documented
- ✅ Meet performance requirements

**All NEEDS CLARIFICATION items from Technical Context are now RESOLVED.**
