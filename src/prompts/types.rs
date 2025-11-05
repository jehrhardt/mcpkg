use serde::Deserialize;
use std::path::PathBuf;
use std::time::SystemTime;

/// Represents a Markdown file in `.twig/prompts/` directory
#[derive(Debug, Clone)]
#[allow(dead_code)] // Used internally by parser and registry
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

/// Parsed YAML frontmatter from the prompt file
#[derive(Debug, Clone, Deserialize, Default)]
#[allow(dead_code)] // Deserialized from YAML frontmatter
pub(crate) struct PromptMetadata {
    /// Required title (display name)
    #[serde(default)]
    pub title: Option<String>,

    /// Required description of what the prompt does
    #[serde(default)]
    pub description: Option<String>,

    /// List of parameters this prompt accepts
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}

/// Defines a parameter that can be passed to a prompt template
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

/// Error types for prompt operations
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)] // Some variants used in future user stories
pub(crate) enum PromptError {
    #[error("Prompt not found: {0}")]
    NotFound(String),

    #[error("Invalid YAML frontmatter in {file}: {source}")]
    InvalidFrontmatter {
        file: String,
        #[source]
        source: gray_matter::Error,
    },

    #[error("Invalid Jinja template in {file}: {source}")]
    InvalidTemplate {
        file: String,
        #[source]
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

/// Convert PromptError to rmcp::ErrorData for MCP protocol
impl From<PromptError> for rmcp::ErrorData {
    fn from(error: PromptError) -> Self {
        match error {
            PromptError::NotFound(name) => {
                rmcp::ErrorData::invalid_params(format!("Prompt '{}' not found", name), None)
            }
            PromptError::MissingArgument(arg) => {
                rmcp::ErrorData::invalid_params(format!("Missing required argument: {}", arg), None)
            }
            _ => rmcp::ErrorData::internal_error(error.to_string(), None),
        }
    }
}

/// Convert PromptArgument to rmcp::model::PromptArgument for MCP protocol
impl From<PromptArgument> for rmcp::model::PromptArgument {
    fn from(arg: PromptArgument) -> Self {
        Self {
            name: arg.name.clone(),
            title: arg.description.clone(),
            description: arg.description,
            required: Some(arg.required),
        }
    }
}
