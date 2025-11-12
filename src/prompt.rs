use minijinja::{Environment, UndefinedBehavior};
use std::collections::HashMap;
use std::path::PathBuf;

/// Render a prompt template with arguments
pub fn render_prompt(
    template_str: &str,
    arguments: &HashMap<String, String>,
) -> Result<String, String> {
    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Chainable);

    env.add_template("prompt", template_str)
        .map_err(|e| format!("Failed to parse template: {}", e))?;

    let template = env
        .get_template("prompt")
        .map_err(|e| format!("Failed to get template: {}", e))?;

    template
        .render(arguments)
        .map_err(|e| format!("Failed to render template: {}", e))
}

/// Load prompt content from a markdown file
pub fn load_prompt_content(path: &PathBuf) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read prompt file {}: {}", path.display(), e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_prompt_simple() {
        let template = "Hello {{ name }}!";
        let mut args = HashMap::new();
        args.insert("name".to_string(), "World".to_string());

        let result = render_prompt(template, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_render_prompt_multiple_args() {
        let template = "{{ greeting }} {{ name }}!";
        let mut args = HashMap::new();
        args.insert("greeting".to_string(), "Hello".to_string());
        args.insert("name".to_string(), "World".to_string());

        let result = render_prompt(template, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_render_prompt_optional_args_chainable() {
        let template = "Hello{% if name %} {{ name }}{% endif %}!";
        let args = HashMap::new();

        let result = render_prompt(template, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello!");
    }

    #[test]
    fn test_render_prompt_with_optional_provided() {
        let template = "Hello{% if name %} {{ name }}{% endif %}!";
        let mut args = HashMap::new();
        args.insert("name".to_string(), "World".to_string());

        let result = render_prompt(template, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }
}
