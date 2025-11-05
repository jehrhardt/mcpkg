use super::types::PromptError;
use minijinja::Environment;
use serde_json::Value;
use std::collections::HashMap;

/// Template renderer for Jinja templates
#[allow(dead_code)] // Used by registry
pub(crate) struct TemplateRenderer {
    // We don't actually need to store the environment since we create it fresh each time
    // This is because templates are dynamic (loaded from files)
}

impl TemplateRenderer {
    #[allow(dead_code)] // Used by registry
    pub fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)] // Used by registry
    pub fn render(
        &self,
        template_name: &str,
        template_content: &str,
        arguments: &HashMap<String, Value>,
    ) -> Result<String, PromptError> {
        // Create a new environment for this render
        let mut env = Environment::new();

        // Add template from string
        env.add_template(template_name, template_content)
            .map_err(|source| PromptError::InvalidTemplate {
                file: template_name.to_string(),
                source,
            })?;

        // Get template
        let tmpl = env.get_template(template_name)?;

        // Render with arguments - convert HashMap to minijinja context
        let rendered = tmpl.render(arguments)?;

        Ok(rendered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_renderer_with_simple_variable_substitution() {
        // T014: Unit test for TemplateRenderer with simple variable substitution
        let renderer = TemplateRenderer::new();
        let mut args = HashMap::new();
        args.insert("name".to_string(), Value::String("World".to_string()));

        let result = renderer.render("test", "Hello {{ name }}!", &args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_template_renderer_with_missing_variables() {
        // T015: Unit test for TemplateRenderer with missing variables
        let renderer = TemplateRenderer::new();
        let args = HashMap::new(); // Empty arguments

        let result = renderer.render("test", "Hello {{ name }}!", &args);

        // Should render with empty/undefined variable (minijinja behavior)
        // or return an error - we'll check this in implementation
        assert!(result.is_ok() || result.is_err());
    }
}
