use std::path::PathBuf;

/// Get the Twig data directory for prompt libraries.
///
/// Priority:
/// 1. TWIG_DATA_DIR environment variable (for testing)
/// 2. Platform-specific data directory via dirs crate
///
/// Platform paths:
/// - Linux: `$XDG_DATA_HOME/twig/prompts` or `$HOME/.local/share/twig/prompts`
/// - macOS: `$HOME/Library/Application Support/twig/prompts`
/// - Windows: `C:\Users\[User]\AppData\Roaming\twig\prompts`
pub fn get_twig_data_dir() -> Result<PathBuf, String> {
    // Check environment variable first (for testing with TWIG_DATA_DIR)
    if let Ok(env_path) = std::env::var("TWIG_DATA_DIR") {
        return Ok(PathBuf::from(env_path));
    }

    // Fall back to platform-specific directory
    dirs::data_dir()
        .map(|d| d.join("twig").join("prompts"))
        .ok_or_else(|| "Unable to determine data directory for this platform".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_twig_data_dir_with_env() {
        let test_path = "/tmp/test_twig_data";
        unsafe {
            env::set_var("TWIG_DATA_DIR", test_path);
        }

        let result = get_twig_data_dir();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from(test_path));

        unsafe {
            env::remove_var("TWIG_DATA_DIR");
        }
    }

    #[test]
    fn test_get_twig_data_dir_default() {
        unsafe {
            env::remove_var("TWIG_DATA_DIR");
        }

        let result = get_twig_data_dir();
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.ends_with("twig/prompts"));
    }
}
