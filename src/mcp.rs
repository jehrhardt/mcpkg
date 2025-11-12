use rmcp::{
    ErrorData, RoleServer, ServerHandler, ServiceExt,
    model::{
        ErrorCode, GetPromptRequestParam, GetPromptResult, Implementation, ListPromptsResult,
        PaginatedRequestParam, Prompt, PromptArgument as McpPromptArgument, PromptMessage,
        PromptMessageRole, ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
    transport::stdio,
};
use std::collections::HashMap;

use twig::{data_dir, library, prompt};

pub(crate) async fn run() {
    let server = Server::new();
    let service = server
        .serve(stdio())
        .await
        .expect("Unable to serve MCP via stdio transport");
    service.waiting().await.expect("MCP server failed");
}

struct Server {
    libraries: Vec<library::PromptLibrary>,
}

impl Server {
    fn new() -> Self {
        let data_dir = match data_dir::get_twig_data_dir() {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Warning: {}", e);
                return Server {
                    libraries: Vec::new(),
                };
            }
        };

        let libraries = library::discover_libraries(&data_dir);
        eprintln!("Loaded {} prompt libraries", libraries.len());

        Server { libraries }
    }

    fn find_prompt(
        &self,
        name: &str,
    ) -> Option<(String, library::PromptDefinition, std::path::PathBuf)> {
        let parts: Vec<&str> = name.split(':').collect();
        if parts.len() != 2 {
            return None;
        }

        let library_name = parts[0];
        let prompt_name = parts[1];

        for library in &self.libraries {
            if library.name == library_name
                && let Some(definition) = library.config.prompts.get(prompt_name)
            {
                return Some((
                    prompt_name.to_string(),
                    definition.clone(),
                    library.path.clone(),
                ));
            }
        }

        None
    }
}

impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "twig".to_string(),
                version: "dev".to_string(),
                icons: None,
                website_url: None,
                title: None,
            },
            capabilities: ServerCapabilities::builder().enable_prompts().build(),
            ..Default::default()
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        let mut prompts = Vec::new();

        for library in &self.libraries {
            for (prompt_name, definition) in &library.config.prompts {
                let full_name = format!("{}:{}", library.name, prompt_name);

                // Build arguments list
                let arguments = definition
                    .arguments
                    .iter()
                    .map(|arg| McpPromptArgument {
                        name: arg.name.clone(),
                        description: arg.description.clone(),
                        required: Some(arg.required),
                        title: None,
                    })
                    .collect();

                let prompt = Prompt {
                    name: full_name,
                    description: Some(definition.description.clone()),
                    arguments: Some(arguments),
                    icons: None,
                    title: None,
                };
                prompts.push(prompt);
            }
        }

        Ok(ListPromptsResult::with_all_items(prompts))
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        let (prompt_name, definition, library_path) =
            self.find_prompt(&request.name).ok_or_else(|| {
                ErrorData::new(
                    ErrorCode(-32601),
                    format!("Prompt not found: {}", request.name),
                    None,
                )
            })?;

        // Validate required arguments and convert to HashMap<String, String>
        let arguments_json = request.arguments.unwrap_or_default();
        let mut arguments = HashMap::new();

        for arg in &definition.arguments {
            if arg.required && !arguments_json.contains_key(&arg.name) {
                return Err(ErrorData::new(
                    ErrorCode(-32602),
                    format!("Missing required argument: {}", arg.name),
                    None,
                ));
            }

            if let Some(value) = arguments_json.get(&arg.name) {
                if let Some(s) = value.as_str() {
                    arguments.insert(arg.name.clone(), s.to_string());
                } else {
                    arguments.insert(arg.name.clone(), value.to_string());
                }
            }
        }

        // Load prompt content
        let prompt_file = library_path
            .join("prompts")
            .join(format!("{}.md", prompt_name));
        let content = prompt::load_prompt_content(&prompt_file).map_err(|e| {
            ErrorData::new(
                ErrorCode(-32602),
                format!("Prompt content file not found: {}", e),
                None,
            )
        })?;

        // Render template with arguments
        let rendered = prompt::render_prompt(&content, &arguments).map_err(|e| {
            ErrorData::new(
                ErrorCode(-32602),
                format!("Error rendering prompt template: {}", e),
                None,
            )
        })?;

        Ok(GetPromptResult {
            description: Some(definition.description.clone()),
            messages: vec![PromptMessage::new_text(PromptMessageRole::User, rendered)],
        })
    }
}
