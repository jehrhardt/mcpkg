use crate::prompts::PromptRegistry;
use rmcp::{
    ErrorData, RoleServer, ServerHandler, ServiceExt,
    model::{
        GetPromptRequestParam, GetPromptResult, Implementation, ListPromptsResult,
        PaginatedRequestParam, Prompt, PromptMessage, PromptMessageRole, ServerCapabilities,
        ServerInfo,
    },
    service::RequestContext,
    transport::stdio,
};
use std::env;

pub(crate) async fn run() {
    let server = Server::new().await;
    let service = server
        .serve(stdio())
        .await
        .expect("Unable to serve MCP via stdio transport");
    service.waiting().await.expect("MCP server failed");
}

struct Server {
    registry: PromptRegistry,
}

impl Server {
    async fn new() -> Self {
        // Get prompts directory relative to current directory
        let prompts_dir = env::current_dir()
            .expect("Failed to get current directory")
            .join(".twig")
            .join("prompts");

        let registry = PromptRegistry::new(prompts_dir);

        // Load prompts
        if let Err(e) = registry.load_all().await {
            eprintln!("Warning: Failed to load prompts: {}", e);
        }

        Self { registry }
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
        let prompts = self.registry.list().await;

        let mcp_prompts: Vec<Prompt> = prompts
            .into_iter()
            .map(|p| {
                let args: Option<Vec<rmcp::model::PromptArgument>> =
                    if p.metadata.arguments.is_empty() {
                        None
                    } else {
                        Some(p.metadata.arguments.into_iter().map(Into::into).collect())
                    };

                Prompt::new(p.name, p.metadata.description, args)
            })
            .collect();

        Ok(ListPromptsResult::with_all_items(mcp_prompts))
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        let args = request.arguments.unwrap_or_default();

        let rendered = self
            .registry
            .render(&request.name, &args)
            .await
            .map_err(ErrorData::from)?;

        Ok(GetPromptResult {
            description: None,
            messages: vec![PromptMessage::new_text(PromptMessageRole::User, rendered)],
        })
    }
}
