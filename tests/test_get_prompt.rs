/// Integration tests for User Story 2: Retrieve Prompt Content
/// Contract: prompts/get MCP handler with argument substitution
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn setup_test_library_with_args(
    base_dir: &PathBuf,
    library_name: &str,
    prompt_name: &str,
    required_args: Vec<&str>,
    optional_args: Vec<&str>,
    content: &str,
) {
    let lib_dir = base_dir.join(library_name);
    fs::create_dir_all(&lib_dir).expect("Failed to create library directory");

    let prompts_dir = lib_dir.join("prompts");
    fs::create_dir_all(&prompts_dir).expect("Failed to create prompts directory");

    // Create twig.toml with arguments
    let mut toml_content = format!("[prompts.{}]\n", prompt_name);
    toml_content.push_str("description = \"Test prompt\"\n\n");

    for arg in required_args {
        toml_content.push_str(&format!(
            "[[prompts.{}.arguments]]\nname = \"{}\"\nrequired = true\n\n",
            prompt_name, arg
        ));
    }

    for arg in optional_args {
        toml_content.push_str(&format!(
            "[[prompts.{}.arguments]]\nname = \"{}\"\n\n",
            prompt_name, arg
        ));
    }

    fs::write(lib_dir.join("twig.toml"), toml_content).expect("Failed to write twig.toml");

    // Create prompt markdown file
    fs::write(prompts_dir.join(format!("{}.md", prompt_name)), content)
        .expect("Failed to write prompt file");
}

#[test]
fn test_render_prompt_with_required_args() {
    let temp_dir = std::env::temp_dir().join("twig_test_get_required");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");

    let content = "# Code Review\n\nPlease review: {{ code_snippet }}";
    setup_test_library_with_args(
        &temp_dir,
        "test_lib",
        "code_review",
        vec!["code_snippet"],
        vec![],
        content,
    );

    unsafe {
        std::env::set_var("TWIG_DATA_DIR", temp_dir.to_str().unwrap());
    }

    use twig::prompt;

    let mut args = HashMap::new();
    args.insert("code_snippet".to_string(), "def hello(): pass".to_string());

    let result = prompt::render_prompt(content, &args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "# Code Review\n\nPlease review: def hello(): pass"
    );

    unsafe {
        std::env::remove_var("TWIG_DATA_DIR");
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_render_prompt_with_optional_args_provided() {
    let content =
        "# Review\n\n{% if language %}Language: {{ language }}{% endif %}\nCode: {{ code }}";

    use twig::prompt;

    let mut args = HashMap::new();
    args.insert("code".to_string(), "print('hello')".to_string());
    args.insert("language".to_string(), "python".to_string());

    let result = prompt::render_prompt(content, &args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "# Review\n\nLanguage: python\nCode: print('hello')"
    );
}

#[test]
fn test_render_prompt_with_optional_args_omitted() {
    let content =
        "# Review\n\n{% if language %}Language: {{ language }}{% endif %}\nCode: {{ code }}";

    use twig::prompt;

    let mut args = HashMap::new();
    args.insert("code".to_string(), "print('hello')".to_string());

    let result = prompt::render_prompt(content, &args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "# Review\n\n\nCode: print('hello')");
}

#[test]
fn test_load_prompt_content() {
    let temp_dir = std::env::temp_dir().join("twig_test_load_content");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");

    let test_content = "# Test Prompt\n\nThis is a {{ test }} prompt.";
    let test_file = temp_dir.join("test_prompt.md");
    fs::write(&test_file, test_content).expect("Failed to write test file");

    use twig::prompt;

    let result = prompt::load_prompt_content(&test_file);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), test_content);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_load_prompt_content_missing_file() {
    use twig::prompt;

    let nonexistent = PathBuf::from("/nonexistent/path/test.md");
    let result = prompt::load_prompt_content(&nonexistent);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to read prompt file"));
}

#[test]
fn test_render_prompt_multiple_arguments() {
    let content = "Task: {{ task }}\nInput: {{ input }}\nOutput: {{ output }}";

    use twig::prompt;

    let mut args = HashMap::new();
    args.insert("task".to_string(), "Analyze".to_string());
    args.insert("input".to_string(), "data.csv".to_string());
    args.insert("output".to_string(), "report.pdf".to_string());

    let result = prompt::render_prompt(content, &args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Task: Analyze\nInput: data.csv\nOutput: report.pdf"
    );
}
