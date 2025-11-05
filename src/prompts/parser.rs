use super::types::{PromptError, PromptFile, PromptMetadata};
use gray_matter::{Matter, engine::YAML};
use std::fs;
use std::path::Path;

/// Parse a prompt file from the given path
pub(crate) fn parse_prompt_file(path: &Path) -> Result<PromptFile, PromptError> {
    // Read file content
    let content = fs::read_to_string(path)?;
    let modified = fs::metadata(path)?.modified()?;

    // Extract name from filename (without .md extension)
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| {
            PromptError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid filename",
            ))
        })?
        .to_string();

    // Parse YAML frontmatter using gray_matter
    let matter = Matter::<YAML>::new();
    let parsed =
        matter
            .parse::<PromptMetadata>(&content)
            .map_err(|e| PromptError::InvalidFrontmatter {
                file: name.clone(),
                source: e,
            })?;

    // Extract metadata (use default if none)
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
    fn test_parse_prompt_file_with_valid_yaml_frontmatter() {
        // T012: Unit test for parse_prompt_file with valid YAML frontmatter
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(file, "---").unwrap();
        writeln!(file, "title: Test Prompt").unwrap();
        writeln!(file, "description: A test prompt").unwrap();
        writeln!(file, "arguments:").unwrap();
        writeln!(file, "  - name: language").unwrap();
        writeln!(file, "    description: Programming language").unwrap();
        writeln!(file, "    required: true").unwrap();
        writeln!(file, "---").unwrap();
        writeln!(file, "Please review this {{{{ language }}}} code.").unwrap();

        let result = parse_prompt_file(file.path());
        assert!(result.is_ok());

        let prompt = result.unwrap();
        assert_eq!(prompt.metadata.title, Some("Test Prompt".to_string()));
        assert_eq!(
            prompt.metadata.description,
            Some("A test prompt".to_string())
        );
        assert_eq!(prompt.metadata.arguments.len(), 1);
        assert_eq!(prompt.metadata.arguments[0].name, "language");
        assert!(
            prompt
                .content
                .contains("Please review this {{ language }} code.")
        );
    }

    #[test]
    fn test_parse_prompt_file_with_missing_required_fields() {
        // T013: Unit test for parse_prompt_file with missing required fields (title or description)
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(file, "---").unwrap();
        writeln!(file, "title: Only Title").unwrap();
        // Missing description
        writeln!(file, "---").unwrap();
        writeln!(file, "Content here").unwrap();

        let result = parse_prompt_file(file.path());
        // For now, we'll accept this (validation happens later in registry)
        // The test ensures parsing doesn't crash with missing fields
        assert!(result.is_ok());
        let prompt = result.unwrap();
        assert_eq!(prompt.metadata.title, Some("Only Title".to_string()));
        assert_eq!(prompt.metadata.description, None);
    }
}
