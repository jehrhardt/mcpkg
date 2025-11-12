use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// A prompt library loaded from a directory
#[derive(Debug, Clone)]
pub struct PromptLibrary {
    pub name: String,
    pub path: PathBuf,
    pub config: LibraryConfig,
}

/// Configuration metadata from twig.toml
#[derive(Debug, Clone, Deserialize)]
pub struct LibraryConfig {
    #[serde(default)]
    pub prompts: HashMap<String, PromptDefinition>,
}

/// A prompt definition from twig.toml
#[derive(Debug, Clone, Deserialize)]
pub struct PromptDefinition {
    pub description: String,
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}

/// An argument definition for a prompt
#[derive(Debug, Clone, Deserialize)]
pub struct PromptArgument {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
}

/// Normalize a library name (lowercase, replace special chars with underscores)
pub fn normalize_library_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}

/// Discover all prompt libraries in a directory
pub fn discover_libraries(data_dir: &PathBuf) -> Vec<PromptLibrary> {
    let mut libraries = Vec::new();

    let entries = match std::fs::read_dir(data_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!(
                "Failed to read data directory {}: {}",
                data_dir.display(),
                e
            );
            return libraries;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let config_path = path.join("twig.toml");
        if !config_path.exists() {
            continue;
        }

        // Parse twig.toml
        let config_content = match std::fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read twig.toml in {}: {}", path.display(), e);
                continue;
            }
        };

        let config: LibraryConfig = match toml::from_str(&config_content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to parse twig.toml in {}: {}", path.display(), e);
                continue;
            }
        };

        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let name = normalize_library_name(dir_name);

        libraries.push(PromptLibrary { name, path, config });
    }

    libraries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_library_name_lowercase() {
        assert_eq!(normalize_library_name("MyLib"), "mylib");
    }

    #[test]
    fn test_normalize_library_name_special_chars() {
        assert_eq!(normalize_library_name("My-Code Lib"), "my_code_lib");
        assert_eq!(normalize_library_name("My_Code_Lib"), "my_code_lib");
        assert_eq!(normalize_library_name("My.Code.Lib"), "my_code_lib");
    }

    #[test]
    fn test_normalize_library_name_trim_underscores() {
        assert_eq!(normalize_library_name("_MyLib_"), "mylib");
        assert_eq!(normalize_library_name("__MyLib__"), "mylib");
    }

    #[test]
    fn test_normalize_library_name_alphanumeric() {
        assert_eq!(normalize_library_name("lib123"), "lib123");
        assert_eq!(normalize_library_name("123lib"), "123lib");
    }
}
