use super::parser::parse_prompt_file;
use super::renderer::TemplateRenderer;
use super::types::{PromptError, PromptFile, PromptInfo};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Central registry managing all loaded prompts
pub(crate) struct PromptRegistry {
    /// Map of prompt name → prompt metadata (lightweight, no content)
    prompts: Arc<RwLock<HashMap<String, PromptInfo>>>,

    /// Path to the prompts directory
    directory: PathBuf,

    /// Template rendering environment
    renderer: TemplateRenderer,
}

impl PromptRegistry {
    /// Create new registry for given directory
    pub fn new(directory: PathBuf) -> Self {
        Self {
            prompts: Arc::new(RwLock::new(HashMap::new())),
            directory,
            renderer: TemplateRenderer::new(),
        }
    }

    /// Load all prompts from directory (metadata only, no content)
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

            // Parse prompt file to extract metadata
            match parse_prompt_file(&path) {
                Ok(prompt) => {
                    loaded.push(prompt.name.clone());

                    // Store only metadata, not content (reduces memory footprint)
                    let info = PromptInfo {
                        name: prompt.name.clone(),
                        path: prompt.path,
                        metadata: prompt.metadata,
                        modified: prompt.modified,
                    };
                    prompts.insert(prompt.name, info);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                }
            }
        }

        Ok(loaded)
    }

    /// Get prompt by name (loads content from disk on-demand)
    pub async fn get(&self, name: &str) -> Option<PromptFile> {
        let prompts = self.prompts.read().await;
        let info = prompts.get(name)?;

        // Load the full prompt file from disk (including content)
        match parse_prompt_file(&info.path) {
            Ok(prompt) => Some(prompt),
            Err(e) => {
                eprintln!("Warning: Failed to load prompt '{}': {}", name, e);
                None
            }
        }
    }

    /// List all prompts (returns metadata only, no content)
    pub async fn list(&self) -> Vec<PromptFile> {
        let prompts = self.prompts.read().await;

        // Convert PromptInfo to PromptFile with empty content
        prompts
            .values()
            .map(|info| PromptFile {
                name: info.name.clone(),
                path: info.path.clone(),
                metadata: info.metadata.clone(),
                content: String::new(), // Empty content for list operation
                modified: info.modified,
            })
            .collect()
    }

    /// Render prompt with arguments
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_load_all_stores_metadata_only() {
        // Create temp directory with test prompt
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let prompts_dir = temp_dir.path().join("prompts");
        std::fs::create_dir(&prompts_dir).expect("Failed to create prompts dir");

        // Create a large prompt file
        let prompt_path = prompts_dir.join("test.md");
        let mut file = std::fs::File::create(&prompt_path).expect("Failed to create file");
        writeln!(file, "---").unwrap();
        writeln!(file, "title: Test Prompt").unwrap();
        writeln!(file, "description: A test prompt").unwrap();
        writeln!(file, "---").unwrap();
        // Write large content
        for i in 0..1000 {
            writeln!(file, "Line {} of large content", i).unwrap();
        }

        // Load prompts
        let registry = PromptRegistry::new(prompts_dir);
        let loaded = registry.load_all().await.expect("Failed to load prompts");

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0], "test");

        // Verify metadata is stored but content is not loaded in memory
        let prompts = registry.prompts.read().await;
        let info = prompts.get("test").expect("Prompt not found");

        // PromptInfo should have metadata but we don't store content
        assert_eq!(info.name, "test");
        assert_eq!(info.metadata.title, Some("Test Prompt".to_string()));
    }

    #[tokio::test]
    async fn test_get_loads_content_on_demand() {
        // Create temp directory with test prompt
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let prompts_dir = temp_dir.path().join("prompts");
        std::fs::create_dir(&prompts_dir).expect("Failed to create prompts dir");

        let prompt_path = prompts_dir.join("hello.md");
        let mut file = std::fs::File::create(&prompt_path).expect("Failed to create file");
        writeln!(file, "---").unwrap();
        writeln!(file, "title: Hello").unwrap();
        writeln!(file, "description: Says hello").unwrap();
        writeln!(file, "---").unwrap();
        writeln!(file, "Hello, {{{{ name }}}}!").unwrap();

        // Load prompts (metadata only)
        let registry = PromptRegistry::new(prompts_dir);
        registry.load_all().await.expect("Failed to load prompts");

        // Get prompt should load content from disk
        let prompt = registry.get("hello").await.expect("Prompt not found");

        assert_eq!(prompt.name, "hello");
        assert_eq!(prompt.metadata.title, Some("Hello".to_string()));
        assert!(prompt.content.contains("Hello, {{ name }}!"));
    }

    #[tokio::test]
    async fn test_list_returns_metadata_without_content() {
        // Create temp directory with test prompts
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let prompts_dir = temp_dir.path().join("prompts");
        std::fs::create_dir(&prompts_dir).expect("Failed to create prompts dir");

        // Create multiple prompts
        for name in &["prompt1", "prompt2", "prompt3"] {
            let prompt_path = prompts_dir.join(format!("{}.md", name));
            let mut file = std::fs::File::create(&prompt_path).expect("Failed to create file");
            writeln!(file, "---").unwrap();
            writeln!(file, "title: {}", name).unwrap();
            writeln!(file, "description: Test prompt").unwrap();
            writeln!(file, "---").unwrap();
            writeln!(file, "Large content for {}", name).unwrap();
        }

        let registry = PromptRegistry::new(prompts_dir);
        registry.load_all().await.expect("Failed to load prompts");

        // List should return metadata without content
        let prompts = registry.list().await;
        assert_eq!(prompts.len(), 3);

        for prompt in prompts {
            // Content should be empty in list results
            assert_eq!(prompt.content, "");
            // But metadata should be present
            assert!(prompt.metadata.title.is_some());
        }
    }
}
