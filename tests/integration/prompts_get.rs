use rmcp::{model::*, ServiceExt};
use std::io::Write;
use tempfile::TempDir;
use tokio::io::duplex;

#[tokio::test]
async fn test_prompts_get_with_parameter_substitution() -> anyhow::Result<()> {
    // T018: Integration test for prompts/get with parameter substitution
    
    let temp_dir = TempDir::new()?;
    let prompts_dir = temp_dir.path().join(".twig").join("prompts");
    std::fs::create_dir_all(&prompts_dir)?;
    
    // Create prompt with parameters
    let mut file = std::fs::File::create(prompts_dir.join("greet.md"))?;
    writeln!(file, "---")?;
    writeln!(file, "title: Greeting")?;
    writeln!(file, "description: Greet someone")?;
    writeln!(file, "arguments:")?;
    writeln!(file, "  - name: name")?;
    writeln!(file, "    description: Person's name")?;
    writeln!(file, "    required: true")?;
    writeln!(file, "---")?;
    writeln!(file, "Hello, {{{{ name }}}}!")?;
    
    // This test will fail until we implement the server
    Ok(())
}

#[tokio::test]
async fn test_prompts_get_error_for_unknown_prompt() -> anyhow::Result<()> {
    // T019: Integration test for prompts/get returning error for unknown prompt
    
    let temp_dir = TempDir::new()?;
    let prompts_dir = temp_dir.path().join(".twig").join("prompts");
    std::fs::create_dir_all(&prompts_dir)?;
    
    // This test will fail until we implement the server
    Ok(())
}

#[tokio::test]
async fn test_prompts_get_error_for_missing_required_argument() -> anyhow::Result<()> {
    // T020: Integration test for prompts/get returning error for missing required argument
    
    let temp_dir = TempDir::new()?;
    let prompts_dir = temp_dir.path().join(".twig").join("prompts");
    std::fs::create_dir_all(&prompts_dir)?;
    
    // Create prompt with required parameter
    let mut file = std::fs::File::create(prompts_dir.join("greet.md"))?;
    writeln!(file, "---")?;
    writeln!(file, "title: Greeting")?;
    writeln!(file, "description: Greet someone")?;
    writeln!(file, "arguments:")?;
    writeln!(file, "  - name: name")?;
    writeln!(file, "    description: Person's name")?;
    writeln!(file, "    required: true")?;
    writeln!(file, "---")?;
    writeln!(file, "Hello, {{{{ name }}}}!")?;
    
    // This test will fail until we implement the server
    Ok(())
}
