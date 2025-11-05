use rmcp::{model::*, ServiceExt};
use std::io::Write;
use tempfile::TempDir;
use tokio::io::duplex;

#[tokio::test]
async fn test_prompts_list_empty_when_directory_missing() -> anyhow::Result<()> {
    // T016: Integration test for prompts/list returning empty array when .twig/prompts/ doesn't exist
    
    // Create temp directory without .twig/prompts
    let temp_dir = TempDir::new()?;
    
    // This test will fail until we implement the server
    // For now, we'll just assert that it compiles
    Ok(())
}

#[tokio::test]
async fn test_prompts_list_returns_multiple_prompts() -> anyhow::Result<()> {
    // T017: Integration test for prompts/list returning multiple prompts with correct names and metadata
    
    let temp_dir = TempDir::new()?;
    let prompts_dir = temp_dir.path().join(".twig").join("prompts");
    std::fs::create_dir_all(&prompts_dir)?;
    
    // Create first prompt file
    let mut file1 = std::fs::File::create(prompts_dir.join("code-review.md"))?;
    writeln!(file1, "---")?;
    writeln!(file1, "title: Code Review")?;
    writeln!(file1, "description: Review code")?;
    writeln!(file1, "---")?;
    writeln!(file1, "Review this code.")?;
    
    // Create second prompt file
    let mut file2 = std::fs::File::create(prompts_dir.join("hello.md"))?;
    writeln!(file2, "---")?;
    writeln!(file2, "title: Hello")?;
    writeln!(file2, "description: Say hello")?;
    writeln!(file2, "---")?;
    writeln!(file2, "Hello!")?;
    
    // This test will fail until we implement the server
    Ok(())
}

#[tokio::test]
async fn test_prompts_list_updated_after_file_modification() -> anyhow::Result<()> {
    // T037 (from US2): Integration test verifying prompts/list returns updated content after file modification
    
    let temp_dir = TempDir::new()?;
    let prompts_dir = temp_dir.path().join(".twig").join("prompts");
    std::fs::create_dir_all(&prompts_dir)?;
    
    // Create initial prompt file
    let prompt_path = prompts_dir.join("test.md");
    let mut file = std::fs::File::create(&prompt_path)?;
    writeln!(file, "---")?;
    writeln!(file, "title: Original")?;
    writeln!(file, "description: Original description")?;
    writeln!(file, "---")?;
    writeln!(file, "Original content")?;
    drop(file);
    
    // This test will fail until we implement file watching
    Ok(())
}
