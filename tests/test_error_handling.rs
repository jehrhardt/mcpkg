/// Integration tests for User Story 3: Handle Library Configuration Errors
/// Goal: Verify clear error messages for configuration issues
use std::fs;
use std::path::PathBuf;

#[test]
fn test_malformed_toml_library_is_skipped() {
    let temp_dir = std::env::temp_dir().join("twig_test_malformed_toml");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");

    // Create a library with malformed TOML
    let bad_lib_dir = temp_dir.join("bad_lib");
    fs::create_dir_all(&bad_lib_dir).expect("Failed to create bad library directory");

    let malformed_toml = "[prompts.test\n  # Missing closing bracket";
    fs::write(bad_lib_dir.join("twig.toml"), malformed_toml)
        .expect("Failed to write malformed twig.toml");

    // Create a valid library in the same directory
    let good_lib_dir = temp_dir.join("good_lib");
    fs::create_dir_all(&good_lib_dir).expect("Failed to create good library directory");

    let good_prompts_dir = good_lib_dir.join("prompts");
    fs::create_dir_all(&good_prompts_dir).expect("Failed to create good prompts directory");

    let good_toml = "[prompts.test]\ndescription = \"A good prompt\"";
    fs::write(good_lib_dir.join("twig.toml"), good_toml).expect("Failed to write good twig.toml");
    fs::write(good_prompts_dir.join("test.md"), "# Test").expect("Failed to write prompt file");

    unsafe {
        std::env::set_var("TWIG_DATA_DIR", temp_dir.to_str().unwrap());
    }

    use twig::{data_dir, library};

    let data_dir = data_dir::get_twig_data_dir().expect("Failed to get data dir");
    let libraries = library::discover_libraries(&data_dir);

    // Should only load the good library, bad library should be skipped
    assert_eq!(libraries.len(), 1);
    assert_eq!(libraries[0].name, "good_lib");

    unsafe {
        std::env::remove_var("TWIG_DATA_DIR");
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_missing_prompts_directory_is_handled() {
    let temp_dir = std::env::temp_dir().join("twig_test_missing_prompts_dir");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");

    let lib_dir = temp_dir.join("test_lib");
    fs::create_dir_all(&lib_dir).expect("Failed to create library directory");

    // Create twig.toml but no prompts/ directory
    let toml = "[prompts.test]\ndescription = \"Test prompt\"";
    fs::write(lib_dir.join("twig.toml"), toml).expect("Failed to write twig.toml");

    unsafe {
        std::env::set_var("TWIG_DATA_DIR", temp_dir.to_str().unwrap());
    }

    use twig::{data_dir, library};

    let data_dir = data_dir::get_twig_data_dir().expect("Failed to get data dir");
    let libraries = library::discover_libraries(&data_dir);

    // Library should still load (prompts/ directory is created on demand or warnings issued)
    // In this implementation, we don't require prompts/ to exist at discovery time
    assert_eq!(libraries.len(), 1);

    unsafe {
        std::env::remove_var("TWIG_DATA_DIR");
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_missing_markdown_file_error() {
    use twig::prompt;

    let nonexistent = PathBuf::from("/nonexistent/path/missing.md");
    let result = prompt::load_prompt_content(&nonexistent);

    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("Failed to read prompt file"));
    assert!(error_msg.contains("missing.md"));
}

#[test]
fn test_template_rendering_error_with_invalid_syntax() {
    use std::collections::HashMap;
    use twig::prompt;

    // Invalid Jinja2 syntax (unclosed tag)
    let bad_template = "Hello {{ name";
    let args = HashMap::new();

    let result = prompt::render_prompt(bad_template, &args);
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("Failed to"));
}

#[test]
fn test_empty_data_directory_returns_empty_list() {
    let temp_dir = std::env::temp_dir().join("twig_test_empty_dir");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");

    unsafe {
        std::env::set_var("TWIG_DATA_DIR", temp_dir.to_str().unwrap());
    }

    use twig::{data_dir, library};

    let data_dir = data_dir::get_twig_data_dir().expect("Failed to get data dir");
    let libraries = library::discover_libraries(&data_dir);

    assert_eq!(libraries.len(), 0);

    unsafe {
        std::env::remove_var("TWIG_DATA_DIR");
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_nonexistent_data_directory_is_handled() {
    let nonexistent_dir = PathBuf::from("/nonexistent/twig/data/dir");

    use twig::library;

    let libraries = library::discover_libraries(&nonexistent_dir);

    // Should return empty list when directory doesn't exist
    assert_eq!(libraries.len(), 0);
}
