/// Integration tests for User Story 1: Discover Available Prompts
/// Contract: prompts/list MCP handler
use std::fs;
use std::path::PathBuf;

fn setup_test_library(base_dir: &PathBuf, library_name: &str, prompts: Vec<(&str, &str)>) {
    let lib_dir = base_dir.join(library_name);
    fs::create_dir_all(&lib_dir).expect("Failed to create library directory");

    let prompts_dir = lib_dir.join("prompts");
    fs::create_dir_all(&prompts_dir).expect("Failed to create prompts directory");

    // Create twig.toml
    let mut toml_content = String::new();
    for (name, desc) in &prompts {
        toml_content.push_str(&format!(
            "[prompts.{}]\ndescription = \"{}\"\n\n",
            name, desc
        ));
    }
    fs::write(lib_dir.join("twig.toml"), toml_content).expect("Failed to write twig.toml");

    // Create prompt markdown files
    for (name, _) in &prompts {
        let content = format!("# Prompt: {}\n\nThis is a test prompt.", name);
        fs::write(prompts_dir.join(format!("{}.md", name)), content)
            .expect("Failed to write prompt file");
    }
}

#[test]
fn test_list_prompts_single_library() {
    let temp_dir = std::env::temp_dir().join("twig_test_list_single");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");

    setup_test_library(
        &temp_dir,
        "test_lib",
        vec![
            ("code_review", "Review code for quality"),
            ("documentation", "Generate documentation"),
        ],
    );

    unsafe {
        std::env::set_var("TWIG_DATA_DIR", temp_dir.to_str().unwrap());
    }

    // Import and test library discovery
    use twig::data_dir;
    use twig::library;

    let data_dir = data_dir::get_twig_data_dir().expect("Failed to get data dir");
    let libraries = library::discover_libraries(&data_dir);

    assert_eq!(libraries.len(), 1);
    assert_eq!(libraries[0].name, "test_lib");
    assert_eq!(libraries[0].config.prompts.len(), 2);
    assert!(libraries[0].config.prompts.contains_key("code_review"));
    assert!(libraries[0].config.prompts.contains_key("documentation"));

    unsafe {
        std::env::remove_var("TWIG_DATA_DIR");
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_list_prompts_multiple_libraries() {
    let temp_dir = std::env::temp_dir().join("twig_test_list_multiple");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");

    setup_test_library(
        &temp_dir,
        "coding_lib",
        vec![("code_review", "Review code")],
    );
    setup_test_library(
        &temp_dir,
        "data_science_lib",
        vec![("analyze_dataset", "Analyze data")],
    );

    unsafe {
        std::env::set_var("TWIG_DATA_DIR", temp_dir.to_str().unwrap());
    }

    use twig::data_dir;
    use twig::library;

    let data_dir = data_dir::get_twig_data_dir().expect("Failed to get data dir");
    let libraries = library::discover_libraries(&data_dir);

    assert_eq!(libraries.len(), 2);

    // Find each library
    let coding_lib = libraries
        .iter()
        .find(|l| l.name == "coding_lib")
        .expect("coding_lib not found");
    let ds_lib = libraries
        .iter()
        .find(|l| l.name == "data_science_lib")
        .expect("data_science_lib not found");

    assert_eq!(coding_lib.config.prompts.len(), 1);
    assert_eq!(ds_lib.config.prompts.len(), 1);

    unsafe {
        std::env::remove_var("TWIG_DATA_DIR");
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_list_prompts_empty_directory() {
    let temp_dir = std::env::temp_dir().join("twig_test_list_empty");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");

    unsafe {
        std::env::set_var("TWIG_DATA_DIR", temp_dir.to_str().unwrap());
    }

    use twig::data_dir;
    use twig::library;

    let data_dir = data_dir::get_twig_data_dir().expect("Failed to get data dir");
    let libraries = library::discover_libraries(&data_dir);

    assert_eq!(libraries.len(), 0);

    unsafe {
        std::env::remove_var("TWIG_DATA_DIR");
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_library_name_normalization() {
    use twig::library::normalize_library_name;

    assert_eq!(normalize_library_name("My-Code Lib"), "my_code_lib");
    assert_eq!(normalize_library_name("DataScience"), "datascience");
    assert_eq!(normalize_library_name("_test_lib_"), "test_lib");
}
