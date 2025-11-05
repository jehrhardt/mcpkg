use super::parser::parse_prompt_file;
use super::renderer::TemplateRenderer;
use super::types::{PromptError, PromptFile};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Central registry managing all loaded prompts
pub(crate) struct PromptRegistry {
    /// Map of prompt name → prompt file
    prompts: Arc<RwLock<HashMap<String, PromptFile>>>,

    /// Path to the prompts directory
    directory: PathBuf,

    /// Template rendering environment
    renderer: TemplateRenderer,
}

impl PromptRegistry {
    /// Create new registry for given directory
    #[allow(dead_code)] // Used by MCP server
    pub fn new(directory: PathBuf) -> Self {
        Self {
            prompts: Arc::new(RwLock::new(HashMap::new())),
            directory,
            renderer: TemplateRenderer::new(),
        }
    }

    /// Load all prompts from directory
    #[allow(dead_code)] // Used by MCP server
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
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                }
            }
        }

        Ok(loaded)
    }

    /// Get prompt by name
    #[allow(dead_code)] // Used internally by render method
    pub async fn get(&self, name: &str) -> Option<PromptFile> {
        let prompts = self.prompts.read().await;
        prompts.get(name).cloned()
    }

    /// List all prompts
    #[allow(dead_code)] // Used by MCP server
    pub async fn list(&self) -> Vec<PromptFile> {
        let prompts = self.prompts.read().await;
        prompts.values().cloned().collect()
    }

    /// Render prompt with arguments
    #[allow(dead_code)] // Used by MCP server
    pub async fn render(
        &self,
        name: &str,
        arguments: &Map<String, Value>,
    ) -> Result<String, PromptError> {
        let prompt = self
            .get(name)
            .await
            .ok_or_else(|| PromptError::NotFound(name.to_string()))?;

        // Validate required arguments
        for arg in &prompt.metadata.arguments {
            if arg.required && !arguments.contains_key(&arg.name) {
                return Err(PromptError::MissingArgument(arg.name.clone()));
            }
        }

        // Convert Map to HashMap for renderer
        let args_map: HashMap<String, Value> = arguments
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Render template
        self.renderer
            .render(&prompt.name, &prompt.content, &args_map)
    }
}
