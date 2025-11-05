pub(crate) mod parser;
pub(crate) mod registry;
pub(crate) mod renderer;
pub(crate) mod types;
pub(crate) mod watcher;

// Public API exports
#[allow(unused_imports)] // Used by MCP server in binary
pub(crate) use registry::PromptRegistry;
#[allow(unused_imports)] // Some types used only in tests
pub(crate) use types::{PromptArgument, PromptError, PromptFile, PromptInfo, PromptMetadata};
