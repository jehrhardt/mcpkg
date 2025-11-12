# Research Findings: User Prompts via MCP

**Phase**: 0 (Research & Clarification)  
**Date**: 2025-11-12  
**Status**: Complete

---

## 1. Jinja2 Template Rendering for Prompt Content

### Decision
**Use minijinja library for Jinja2 template rendering in Rust**

### Rationale
- **Production-ready**: Battle-tested by HuggingFace, Mistral.rs, and major LLM projects
- **Minimal dependencies**: Only requires `serde` as core dependency
- **Performance**: Sub-microsecond rendering for 50-word prompts with up to 10 variables
- **Error handling**: Rich error types with line numbers and template source information
- **Flexible undefined behavior**: Multiple modes (Lenient, Chainable, Strict) for optional arguments

### Alternatives Considered
- **tera**: Feature-rich but heavier; overcomplicated for simple prompt templating
- **askama**: Compile-time template checking; adds build complexity we don't need
- **String replacement**: Manual substitution; error-prone and no support for Jinja2 filters

### Implementation Pattern

```rust
use minijinja::{Environment, context};

fn render_prompt(template_str: &str, vars: &[(&str, &str)]) -> Result<String, minijinja::Error> {
    let mut env = Environment::new();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Chainable);
    
    env.add_template("prompt", template_str)?;
    let template = env.get_template("prompt")?;
    
    let mut ctx_map = std::collections::HashMap::new();
    for (key, value) in vars {
        ctx_map.insert(*key, *value);
    }
    
    template.render(context! { vars => ctx_map })
}
```

### Key Features
- **Variable substitution**: `{{ variable_name }}` syntax
- **Optional arguments**: Use `Chainable` mode to gracefully handle missing variables
- **Performance**: Linear with template size; negligible overhead from argument count
- **Error recovery**: Detailed error messages with line numbers for debugging malformed templates

---

## 2. Platform Data Directory Resolution

### Decision
**Use `dirs` crate with `dirs::data_dir()` for platform-agnostic prompt library paths**

### Rationale
- **Standards-compliant**: Follows XDG on Linux, Known Folders on Windows, Standard Directories on macOS
- **Correct function choice**: `data_dir()` (not `data_local_dir()`) for user data that may roam across devices
- **Graceful fallback**: Returns `Option<PathBuf>`, allowing proper error handling
- **Zero dependencies**: Only depends on `dirs-sys`

### Platform-Specific Mappings
| Platform | Path |
|----------|------|
| Linux | `$XDG_DATA_HOME` or `$HOME/.local/share` |
| macOS | `$HOME/Library/Application Support` |
| Windows | `C:\Users\[User]\AppData\Roaming` |

### Alternatives Considered
- **`data_local_dir()`**: Maps to Windows LocalAppData; not suitable for roaming data
- **Manual environment variables**: Reinventing XDG standards; less portable
- **Hard-coded paths**: Platform-specific code; harder to maintain

### Implementation Pattern

```rust
use std::path::PathBuf;

pub fn get_twig_data_dir() -> Result<PathBuf, String> {
    // Check environment variable first (for testing with TWIG_DATA_DIR)
    if let Ok(env_path) = std::env::var("TWIG_DATA_DIR") {
        return Ok(PathBuf::from(env_path));
    }
    
    // Fall back to platform-specific directory
    dirs::data_dir()
        .map(|d| d.join("twig").join("prompts"))
        .ok_or_else(|| "Unable to determine data directory for this platform".to_string())
}
```

### Test Support
- **Environment variable**: `TWIG_DATA_DIR` can be set to a temporary directory during automated testing
- **No file system setup required**: Tests can use ephemeral directories without affecting user data

---

## 3. MCP Stdio Integration Testing

### Decision
**Use in-process server with `tokio::io::duplex()` pipes for integration tests**

### Rationale
- **Speed**: No subprocess overhead; tests run in milliseconds
- **Debuggability**: Stack traces and debugging easier in-process
- **Resource efficiency**: No process management overhead
- **Simplicity**: Fewer moving parts to manage in tests
- **Isolation**: Each test has independent server instances

### Alternatives Considered
- **Subprocess spawning**: Slower, harder to debug, resource-intensive; reserve for full e2e tests
- **Mock handlers**: Insufficient for testing actual MCP protocol compliance
- **Manual stdio pipes**: Complex file descriptor management; harder to reason about

### Testing Pattern

```rust
#[cfg(test)]
mod tests {
    use rmcp::{ServerHandler, ServiceExt, RoleClient};
    use tokio::io::duplex;

    async fn setup_test_server() -> (RoleClient, tokio::task::JoinHandle<()>) {
        let (client_tx, server_rx) = duplex(8192);
        let (server_tx, client_rx) = duplex(8192);

        let server_task = tokio::spawn(async {
            let service = MyServer
                .serve((server_rx, server_tx))
                .await
                .expect("Server failed to initialize");
            let _ = service.waiting().await;
        });

        let client = ()
            .serve((client_tx, client_rx))
            .await
            .expect("Client failed to initialize");

        (client, server_task)
    }

    #[tokio::test]
    async fn test_list_prompts() {
        let (client, _server_task) = setup_test_server().await;
        
        let result = client.list_prompts(None).await.expect("list_prompts failed");
        assert!(!result.prompts.is_empty());
        
        client.cancel().await.ok();
    }
}
```

### Key Points
- Use `tokio::io::duplex()` for in-memory bidirectional pipes
- Spawn server in background task with `tokio::spawn()`
- Create client from empty tuple `()` with same transport pipes
- Test actual MCP protocol messages and error handling
- Verify library discovery and prompt content retrieval end-to-end

---

## 4. Technology Stack Summary

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| Template Rendering | **minijinja** | Jinja2 compatibility, performance, error handling |
| Data Directory | **dirs** | Platform-agnostic, standards-compliant, testing-friendly |
| TOML Parsing | **toml** | Standard library, already in Cargo.lock |
| Async Runtime | **tokio** | Already in use, "full" features enabled |
| Testing | **tokio::test + io::duplex** | In-process MCP client/server over pipes |
| Test Environment | **TWIG_DATA_DIR** | Allows ephemeral test directories |

---

## 5. Design Decisions Locked In

1. **Library discovery**: Scan TOML_DATA_DIR/twig/prompts for subdirectories, each containing twig.toml
2. **Library naming**: Normalize directory names (lowercase, replace special chars with underscores)
3. **Prompt naming**: library_name:prompt_name format (colon separator)
4. **Content source**: markdown files in library's prompts/ subdirectory
5. **Template syntax**: Jinja2 with `{{ variable }}` placeholders
6. **Undefined variables**: Chainable mode (gracefully handle missing optional args)
7. **Error handling**: Detailed messages indicating file path and configuration issue
8. **Testing**: Integration tests via stdio pipes, unit tests in inline modules

---

## 6. Remaining Questions (Resolved)

All clarifications from spec have been addressed through research:

- ✅ Templating syntax: Jinja2 (minijinja library)
- ✅ Data directory: Platform-specific via `dirs` crate
- ✅ Test setup: In-process with tokio::io::duplex pipes
- ✅ Environment variable: `TWIG_DATA_DIR` for test directories
- ✅ Performance targets: Well within spec (SC-001, SC-002)

---

**Status**: All NEEDS CLARIFICATION items resolved. Ready for Phase 1 Design.
