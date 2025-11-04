# Data Model: Local Prompts

**Feature**: 001-local-prompts  
**Date**: 2025-11-04  
**Status**: Design

This document defines the data structures and entities for the local prompts feature.

---

## Entity: PromptFile

Represents a Markdown file in `.twig/prompts/` directory.

### Attributes

```rust
pub(crate) struct PromptFile {
    /// Unique identifier derived from filename (without .md extension)
    pub name: String,
    
    /// Absolute path to the file
    pub path: PathBuf,
    
    /// Parsed metadata from YAML frontmatter
    pub metadata: PromptMetadata,
    
    /// Raw content body (after frontmatter, before Jinja rendering)
    pub content: String,
    
    /// File modification timestamp (for change detection)
    pub modified: SystemTime,
}
```

### Validation Rules

- **Name**: Must be valid filename, alphanumeric + dashes/underscores recommended
- **Path**: Must exist in `.twig/prompts/` directory
- **Metadata**: Must contain valid YAML frontmatter (see PromptMetadata)
- **Content**: UTF-8 encoded text, may contain Jinja template syntax
- **File Extension**: Must be `.md`

### Lifecycle

```
File Created → Parse YAML → Validate Metadata → Store in Registry
     ↓                                               ↓
File Modified → Re-parse → Update Registry → Notify Clients
     ↓
File Deleted → Remove from Registry → Notify Clients
```

---

## Entity: PromptMetadata

Parsed YAML frontmatter from the prompt file.

### Attributes

```rust
#[derive(Debug, Deserialize)]
pub(crate) struct PromptMetadata {
    /// Required title (display name)
    /// Must be present in YAML frontmatter
    pub title: String,
    
    /// Required description of what the prompt does
    /// Must be present in YAML frontmatter
    pub description: String,
    
    /// List of parameters this prompt accepts
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}
```

### YAML Example

```yaml
---
title: Code Review Assistant
description: Reviews code with customizable focus areas
arguments:
  - name: language
    description: Programming language of the code
    required: true
  - name: focus
    description: Review focus area (security, performance, style)
    required: false
---
```

### Validation Rules

- **title**: Required, string, max 100 chars recommended
- **description**: Required, string, max 500 chars recommended
- **arguments**: Optional array, each entry must be valid PromptArgument

**Validation Logic**: Files missing `title` or `description` in YAML frontmatter MUST be skipped during load_all() (see FR-002).

---

## Entity: PromptArgument

Defines a parameter that can be passed to a prompt template.

### Attributes

```rust
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PromptArgument {
    /// Parameter name (used in Jinja templates as {{ name }})
    pub name: String,
    
    /// Human-readable description of the parameter
    #[serde(default)]
    pub description: Option<String>,
    
    /// Whether this parameter is required
    #[serde(default)]
    pub required: bool,
}
```

### Validation Rules

- **name**: Required, must be valid Jinja variable name (alphanumeric + underscore, no leading digit)
- **description**: Optional, string
- **required**: Defaults to `false` if not specified

### Conversion to MCP Schema

```rust
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

## Entity: PromptRegistry

Central registry managing all loaded prompts.

### Attributes

```rust
pub(crate) struct PromptRegistry {
    /// Map of prompt name → prompt file
    prompts: Arc<RwLock<HashMap<String, PromptFile>>>,
    
    /// Path to the prompts directory
    directory: PathBuf,
    
    /// File watcher receiver channel
    watcher_rx: Option<mpsc::UnboundedReceiver<DebounceEventResult>>,
    
    /// Template rendering environment
    template_env: Arc<minijinja::Environment<'static>>,
}
```

### Operations

```rust
impl PromptRegistry {
    /// Create new registry for given directory
    pub fn new(directory: PathBuf) -> Result<Self, Error>;
    
    /// Load all prompts from directory
    pub async fn load_all(&mut self) -> Result<Vec<String>, Error>;
    
    /// Get prompt by name
    pub async fn get(&self, name: &str) -> Option<PromptFile>;
    
    /// List all prompt names
    pub async fn list(&self) -> Vec<String>;
    
    /// Render prompt with arguments
    pub async fn render(
        &self,
        name: &str,
        arguments: &HashMap<String, serde_json::Value>,
    ) -> Result<String, Error>;
    
    /// Start watching for file changes
    pub async fn start_watching(&mut self) -> Result<(), Error>;
    
    /// Process file system events (internal)
    async fn handle_file_event(&mut self, event: DebouncedEvent) -> Result<bool, Error>;
}
```

### Concurrency

- Uses `Arc<RwLock<>>` for thread-safe read/write access
- Read operations (list, get) use read lock for concurrent access
- Write operations (load, reload) use write lock for exclusive access
- Template environment is immutable after creation, wrapped in `Arc`

---

## Entity: RenderedPrompt

The final output after Jinja template rendering.

### Attributes

```rust
pub(crate) struct RenderedPrompt {
    /// Prompt name
    pub name: String,
    
    /// Rendered content (after Jinja substitution)
    pub content: String,
    
    /// Optional description from metadata
    pub description: Option<String>,
}
```

### Conversion to MCP Response

```rust
impl RenderedPrompt {
    pub fn to_mcp_result(self) -> rmcp::model::GetPromptResult {
        rmcp::model::GetPromptResult {
            description: self.description,
            messages: vec![
                rmcp::model::PromptMessage::new_text(
                    rmcp::model::PromptMessageRole::User,
                    self.content,
                )
            ],
        }
    }
}
```

---

## Error Types

```rust
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
    
    #[error("File watcher error: {0}")]
    WatcherError(String),
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
```

---

## State Transitions

### Prompt Loading State Machine

```
┌─────────────┐
│  Unloaded   │
└──────┬──────┘
       │ load_all()
       ↓
┌─────────────┐     ┌──────────────┐
│  Loading    │────→│ Parse Error  │ (skip file, log error)
└──────┬──────┘     └──────────────┘
       │ success
       ↓
┌─────────────┐
│   Loaded    │←──┐
└──────┬──────┘   │
       │          │ file modified
       │          │
       │ file deleted
       ↓          │
┌─────────────┐  │
│  Unloaded   │──┘
└─────────────┘
```

### Rendering State Machine

```
┌──────────────┐
│ Get Request  │
└──────┬───────┘
       │
       ↓
┌──────────────┐     ┌──────────────┐
│ Lookup Name  │────→│ Not Found    │ → Error Response
└──────┬───────┘     └──────────────┘
       │ found
       ↓
┌──────────────┐     ┌──────────────┐
│ Validate     │────→│ Missing Args │ → Error Response
│ Arguments    │     └──────────────┘
└──────┬───────┘
       │ valid
       ↓
┌──────────────┐     ┌──────────────┐
│ Render       │────→│ Syntax Error │ → Error Response
│ Template     │     └──────────────┘
└──────┬───────┘
       │ success
       ↓
┌──────────────┐
│   Success    │ → GetPromptResult
└──────────────┘
```

---

## Relationships

```
PromptRegistry (1) ─── contains ──→ (N) PromptFile
                                        │
                                        │ has
                                        ↓
                                    PromptMetadata
                                        │
                                        │ declares
                                        ↓
                                    (N) PromptArgument

PromptFile (1) ─── renders with arguments ──→ (1) RenderedPrompt
```

---

## Performance Considerations

### Caching Strategy
- **Parsed prompts**: Cached in `PromptRegistry` HashMap
- **Invalidation**: On file modification or deletion
- **Template compilation**: minijinja caches compiled templates internally

### Memory Usage
- **Per prompt**: ~1KB metadata + file content size
- **100 prompts**: ~100KB + content (typically <10KB each) = ~1MB total
- **Well within SC-005**: Handle 100+ prompts without degradation

### Read/Write Patterns
- **Read-heavy**: List and get operations are frequent
- **Write-rare**: File changes are infrequent
- **RwLock optimization**: Multiple concurrent reads, exclusive writes

---

## Thread Safety

All entities are designed for concurrent access:

- **PromptRegistry**: Uses `Arc<RwLock<HashMap>>` for thread-safe operations
- **PromptFile**: Immutable after parsing (no interior mutability)
- **Template Environment**: Immutable, wrapped in `Arc` for sharing

---

## Summary

The data model provides:
- ✅ Clear entity boundaries
- ✅ Strong typing with serde integration
- ✅ Comprehensive error handling
- ✅ Thread-safe concurrent access
- ✅ Performance optimization for read-heavy workload
- ✅ Clean conversion to MCP protocol types
