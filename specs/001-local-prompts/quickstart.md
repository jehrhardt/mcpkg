# Quickstart: Local Prompts

**Feature**: 001-local-prompts  
**Audience**: Developers implementing the feature  
**Date**: 2025-11-04

This guide provides a step-by-step walkthrough for implementing the local prompts feature in Twig MCP server.

---

## Prerequisites

- Rust 2024 edition installed
- Familiarity with Tokio async runtime
- Understanding of MCP protocol basics
- rmcp crate 0.8.2 (already in project)

---

## Step 1: Add Dependencies

Update `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.5.50", features = ["derive"] }
rmcp = { version = "0.8.2", features = ["transport-io"] }
tokio = { version = "1.48.0", features = ["full"] }

# New dependencies for local prompts
gray_matter = "0.3"
minijinja = "2.12.0"
notify = "8.2"
notify-debouncer-full = "0.6"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
```

---

## Step 2: Create Module Structure

```bash
# Create prompts module directory
mkdir -p src/prompts

# Create module files
touch src/prompts/mod.rs
touch src/prompts/types.rs
touch src/prompts/parser.rs
touch src/prompts/renderer.rs
touch src/prompts/registry.rs
touch src/prompts/watcher.rs
```

Update `src/main.rs` to declare the module:

```rust
mod cli;
mod mcp;
mod prompts;  // Add this line

#[tokio::main]
async fn main() {
    // existing code
}
```

---

## Step 3: Define Data Types

Create `src/prompts/types.rs`:

```rust
use serde::Deserialize;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub(crate) struct PromptFile {
    pub name: String,
    pub path: PathBuf,
    pub metadata: PromptMetadata,
    pub content: String,
    pub modified: SystemTime,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub(crate) struct PromptMetadata {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PromptArgument {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum PromptError {
    #[error("Prompt not found: {0}")]
    NotFound(String),
    
    #[error("Invalid YAML frontmatter in {file}: {source}")]
    InvalidFrontmatter {
        file: String,
        source: gray_matter::Error,
    },
    
    #[error("Invalid Jinja template in {file}: {source}")]
    InvalidTemplate {
        file: String,
        source: minijinja::Error,
    },
    
    #[error("Missing required argument: {0}")]
    MissingArgument(String),
    
    #[error("Template rendering failed: {0}")]
    RenderError(#[from] minijinja::Error),
    
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<PromptError> for rmcp::ErrorData {
    fn from(error: PromptError) -> Self {
        match error {
            PromptError::NotFound(name) => {
                rmcp::ErrorData::invalid_params(format!("Prompt '{}' not found", name))
            }
            PromptError::MissingArgument(arg) => {
                rmcp::ErrorData::invalid_params(format!("Missing required argument: {}", arg))
            }
            _ => rmcp::ErrorData::internal_error(error.to_string()),
        }
    }
}

// Convert internal types to MCP types
impl From<PromptArgument> for rmcp::model::PromptArgument {
    fn from(arg: PromptArgument) -> Self {
        Self {
            name: arg.name,
            description: arg.description,
            required: Some(arg.required),
        }
    }
}
```

---

## Step 4: Implement Parser

Create `src/prompts/parser.rs`:

```rust
use super::types::{PromptFile, PromptMetadata, PromptError};
use gray_matter::{Matter, engine::YAML};
use std::fs;
use std::path::Path;

pub(crate) fn parse_prompt_file(path: &Path) -> Result<PromptFile, PromptError> {
    // Read file
    let content = fs::read_to_string(path)?;
    let modified = fs::metadata(path)?.modified()?;
    
    // Extract name from filename
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| PromptError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid filename",
        )))?
        .to_string();
    
    // Parse YAML frontmatter
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse_with_struct::<PromptMetadata>(&content)
        .map_err(|source| PromptError::InvalidFrontmatter {
            file: name.clone(),
            source,
        })?;
    
    let metadata = parsed.data.unwrap_or_default();
    let body = parsed.content;
    
    Ok(PromptFile {
        name,
        path: path.to_path_buf(),
        metadata,
        content: body,
        modified,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_prompt_with_frontmatter() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(file, "---").unwrap();
        writeln!(file, "title: Test Prompt").unwrap();
        writeln!(file, "description: A test").unwrap();
        writeln!(file, "---").unwrap();
        writeln!(file, "Hello {{ name }}!").unwrap();
        
        let result = parse_prompt_file(file.path());
        assert!(result.is_ok());
        
        let prompt = result.unwrap();
        assert_eq!(prompt.metadata.title, Some("Test Prompt".to_string()));
        assert_eq!(prompt.metadata.description, Some("A test".to_string()));
        assert!(prompt.content.contains("Hello {{ name }}!"));
    }
}
```

---

## Step 5: Implement Template Renderer

Create `src/prompts/renderer.rs`:

```rust
use super::types::PromptError;
use minijinja::Environment;
use serde_json::Value;
use std::collections::HashMap;

pub(crate) struct TemplateRenderer {
    env: Environment<'static>,
}

impl TemplateRenderer {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }
    
    pub fn render(
        &self,
        template_name: &str,
        template_content: &str,
        arguments: &HashMap<String, Value>,
    ) -> Result<String, PromptError> {
        // Create a new environment for this render
        let mut env = Environment::new();
        
        // Add template
        env.add_template(template_name, template_content)
            .map_err(|source| PromptError::InvalidTemplate {
                file: template_name.to_string(),
                source,
            })?;
        
        // Get template
        let tmpl = env.get_template(template_name)?;
        
        // Render with arguments
        let rendered = tmpl.render(arguments)?;
        
        Ok(rendered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple_template() {
        let renderer = TemplateRenderer::new();
        let mut args = HashMap::new();
        args.insert("name".to_string(), Value::String("World".to_string()));
        
        let result = renderer.render(
            "test",
            "Hello {{ name }}!",
            &args,
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }
}
```

---

## Step 6: Implement Prompt Registry

Create `src/prompts/registry.rs`:

```rust
use super::parser::parse_prompt_file;
use super::renderer::TemplateRenderer;
use super::types::{PromptFile, PromptError, PromptArgument};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;

pub(crate) struct PromptRegistry {
    prompts: Arc<RwLock<HashMap<String, PromptFile>>>,
    directory: PathBuf,
    renderer: TemplateRenderer,
}

impl PromptRegistry {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            prompts: Arc::new(RwLock::new(HashMap::new())),
            directory,
            renderer: TemplateRenderer::new(),
        }
    }
    
    pub async fn load_all(&self) -> Result<Vec<String>, PromptError> {
        let mut loaded = Vec::new();
        
        // Check if directory exists
        if !self.directory.exists() {
            return Ok(loaded);
        }
        
        // Read directory
        let entries = std::fs::read_dir(&self.directory)?;
        let mut prompts = self.prompts.write().await;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            // Skip non-markdown files
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            
            // Parse prompt file
            match parse_prompt_file(&path) {
                Ok(prompt) => {
                    loaded.push(prompt.name.clone());
                    prompts.insert(prompt.name.clone(), prompt);
                }
                Err(e) => {
                    eprintln!("Failed to parse {}: {}", path.display(), e);
                }
            }
        }
        
        Ok(loaded)
    }
    
    pub async fn get(&self, name: &str) -> Option<PromptFile> {
        let prompts = self.prompts.read().await;
        prompts.get(name).cloned()
    }
    
    pub async fn list(&self) -> Vec<PromptFile> {
        let prompts = self.prompts.read().await;
        prompts.values().cloned().collect()
    }
    
    pub async fn render(
        &self,
        name: &str,
        arguments: &HashMap<String, Value>,
    ) -> Result<String, PromptError> {
        let prompt = self.get(name).await
            .ok_or_else(|| PromptError::NotFound(name.to_string()))?;
        
        // Validate required arguments
        for arg in &prompt.metadata.arguments {
            if arg.required && !arguments.contains_key(&arg.name) {
                return Err(PromptError::MissingArgument(arg.name.clone()));
            }
        }
        
        // Render template
        self.renderer.render(&prompt.name, &prompt.content, arguments)
    }
}
```

---

## Step 7: Integrate with MCP Server

Update `src/mcp.rs`:

```rust
use crate::prompts::registry::PromptRegistry;
use rmcp::{
    ErrorData, RoleServer, ServerHandler, ServiceExt,
    model::*,
    service::RequestContext,
    transport::stdio,
};
use std::path::PathBuf;
use std::env;

pub(crate) async fn run() {
    let server = Server::new().await;
    let service = server
        .serve(stdio())
        .await
        .expect("Unable to serve MCP via stdio transport");
    service.waiting().await.expect("MCP server failed");
}

struct Server {
    registry: PromptRegistry,
}

impl Server {
    async fn new() -> Self {
        // Get prompts directory relative to current directory
        let prompts_dir = env::current_dir()
            .expect("Failed to get current directory")
            .join(".twig")
            .join("prompts");
        
        let registry = PromptRegistry::new(prompts_dir);
        
        // Load prompts
        if let Err(e) = registry.load_all().await {
            eprintln!("Failed to load prompts: {}", e);
        }
        
        Self { registry }
    }
}

impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "twig".to_string(),
                version: "0.1.0".to_string(),
                icons: None,
                website_url: None,
                title: None,
            },
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .build(),
            ..Default::default()
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        let prompts = self.registry.list().await;
        
        let mcp_prompts: Vec<Prompt> = prompts
            .into_iter()
            .map(|p| {
                let args: Vec<rmcp::model::PromptArgument> = p.metadata.arguments
                    .into_iter()
                    .map(Into::into)
                    .collect();
                
                Prompt {
                    name: p.name,
                    description: p.metadata.description,
                    arguments: if args.is_empty() { None } else { Some(args) },
                }
            })
            .collect();
        
        Ok(ListPromptsResult::with_all_items(mcp_prompts))
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        let args = request.arguments.unwrap_or_default();
        
        let rendered = self.registry
            .render(&request.name, &args)
            .await
            .map_err(ErrorData::from)?;
        
        Ok(GetPromptResult {
            description: None,
            messages: vec![PromptMessage::new_text(
                PromptMessageRole::User,
                rendered,
            )],
        })
    }
}
```

---

## Step 8: Write Integration Tests

Create `tests/integration/prompts_test.rs`:

```rust
use rmcp::{ServerHandler, ServiceExt, model::*};
use std::io::Write;
use tempfile::TempDir;
use tokio::io::duplex;

#[tokio::test]
async fn test_list_prompts() -> anyhow::Result<()> {
    // Setup test environment
    let temp_dir = TempDir::new()?;
    let prompts_dir = temp_dir.path().join(".twig").join("prompts");
    std::fs::create_dir_all(&prompts_dir)?;
    
    // Create test prompt file
    let mut file = std::fs::File::create(prompts_dir.join("test-prompt.md"))?;
    writeln!(file, "---")?;
    writeln!(file, "title: Test Prompt")?;
    writeln!(file, "---")?;
    writeln!(file, "Hello world!")?;
    
    // Start server
    let (server_transport, client_transport) = duplex(4096);
    
    let server_handle = tokio::spawn(async move {
        // Create server with test directory
        let server = create_test_server(prompts_dir).await;
        server.serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    
    // Create client
    let client = ().serve(client_transport).await?;
    
    // Test list prompts
    let result = client.list_all_prompts().await?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "test-prompt");
    
    // Cleanup
    client.cancel().await?;
    server_handle.await??;
    
    Ok(())
}
```

---

## Step 9: Add File Watching (Phase 2)

File watching will be implemented in Phase 2 (implementation phase) using the patterns from research.md.

---

## Step 10: Build and Test

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test

# Build release
cargo build --release

# Run server
cargo run -- start
```

---

## Testing Locally

Create test prompt:

```bash
mkdir -p .twig/prompts
cat > .twig/prompts/hello.md << 'EOF'
---
title: Hello Prompt
description: Says hello
arguments:
  - name: name
    description: Your name
    required: true
---
Hello, {{ name }}! How can I help you today?
EOF
```

Test with MCP client or inspector:

```bash
mise run dev:mcp
```

---

## Next Steps

1. Implement file watching for Phase 2
2. Add comprehensive unit tests for each module
3. Update documentation in `website/docs/`
4. Performance testing for 100+ prompts
5. Add logging throughout

---

## Common Issues

**Issue**: Prompts not loading  
**Solution**: Ensure `.twig/prompts/` directory exists and contains `.md` files

**Issue**: Template rendering errors  
**Solution**: Check Jinja syntax in prompt files, validate variable names

**Issue**: YAML parsing errors  
**Solution**: Validate YAML frontmatter syntax, ensure `---` delimiters

---

## Performance Tips

- Registry uses `RwLock` for concurrent reads
- Template parsing is cached per prompt
- File watching uses debouncing (2s) to avoid redundant reloads

---

## References

- [research.md](research.md) - Library decisions and rationale
- [data-model.md](data-model.md) - Complete entity definitions
- [contracts/](contracts/) - MCP protocol contracts
- [spec.md](spec.md) - Feature requirements
