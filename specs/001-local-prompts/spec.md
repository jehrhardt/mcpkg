# Feature Specification: Local Prompts Support

**Feature Branch**: `001-local-prompts`  
**Created**: 2025-11-04  
**Status**: Draft  
**Input**: User description: "twig should support local prompts. Local prompts are stored in .twig/prompts within the directory in which the twig MCP server is started. Each prompt is a Markdown file that contains the prompt's content. A prompt has a YAML header (as typically done in Markdown) with the required meta data for a prompt. The prompt is named after the file without the file extension. A prompt can declare parameters, which can be used within the prompt's content using Jinja template syntax. Local prompts are exposed via the list prompts and get prompt messages of the MCP server to allow agents to use them."

## Clarifications

### Session 2025-11-04

- Q: How does the system handle prompt files with the same name in subdirectories? → A: Ignore subdirectories; only scan top-level `.twig/prompts/*.md` files
- Q: When does Jinja template syntax validation occur? → A: Validate on render (lazy); return error only when client requests the prompt with arguments
- Q: How should invalid YAML frontmatter be handled at load time? → A: Skip silently at load time; optionally log warnings for debugging
- Q: Which YAML frontmatter fields are required vs optional? → A: Only `title` and `description` required; `arguments` optional
- Q: What file watching mechanism should be used for list_changed notifications? → A: Use polling as fallback if native file watching unavailable

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Expose Local Prompts to MCP Clients (Priority: P1)

Users want to create reusable prompt templates that MCP clients (like AI assistants) can discover and use. These templates should support parameters to make them flexible for different contexts (minimum: support unlimited parameters of type string with Jinja template substitution, conditionals, and filters per minijinja capabilities).

**Why this priority**: This is the core value proposition - enabling users to define and share custom prompts with AI agents. Without this, the feature provides no value.

**Independent Test**: Can be fully tested by creating a prompt file in `.twig/prompts/`, starting the MCP server, and verifying that an MCP client can list and retrieve the prompt with parameters.

**Acceptance Scenarios**:

1. **Given** a Markdown file with valid YAML frontmatter exists in `.twig/prompts/code-review.md`, **When** an MCP client sends a `prompts/list` request, **Then** the server returns a prompt named "code-review" with its metadata
2. **Given** a prompt defines a parameter `language` in its YAML frontmatter, **When** an MCP client sends `prompts/get` with `arguments: {language: "Python"}`, **Then** the server returns the rendered prompt with "Python" substituted in the template
3. **Given** multiple prompt files exist in `.twig/prompts/`, **When** an MCP client requests the prompt list, **Then** all valid prompts are returned with their correct names (filename without extension)
4. **Given** a prompt uses Jinja template syntax like `{{ parameter_name }}`, **When** the prompt is retrieved with matching arguments, **Then** the template is rendered with the provided values
5. **Given** a prompt uses advanced Jinja features (conditionals `{% if %}`, filters `{{ var | upper }}`), **When** an MCP client retrieves the prompt with appropriate arguments, **Then** the template renders correctly demonstrating flexibility across different contexts

---

### User Story 2 - Handle Prompt Discovery and Updates (Priority: P2)

Users need the system to automatically detect when prompts are added, modified, or removed from the `.twig/prompts/` directory, so clients always see the current set of available prompts.

**Why this priority**: Enables dynamic prompt management without server restarts, improving developer experience.

**Independent Test**: Can be tested by adding/removing prompt files while the server is running and verifying clients receive list_changed notifications.

**Acceptance Scenarios**:

1. **Given** the MCP server is running with prompts capability declaring `listChanged: true`, **When** a new prompt file is added to `.twig/prompts/`, **Then** connected clients receive a `prompts/list_changed` notification
2. **Given** an existing prompt file is modified, **When** an MCP client requests that prompt, **Then** the updated content is returned
3. **Given** a prompt file is deleted, **When** an MCP client lists prompts, **Then** that prompt no longer appears in the list

---

### User Story 3 - Validate Prompt Files (Priority: P3)

Users need clear feedback when their prompt files have errors (invalid YAML, missing required fields, invalid Jinja syntax), so they can fix issues quickly.

**Why this priority**: Improves usability but the feature can work without extensive validation - malformed prompts can simply be skipped.

**Independent Test**: Can be tested by creating invalid prompt files and verifying appropriate error handling (either skipping invalid prompts or returning errors).

**Acceptance Scenarios**:

1. **Given** a prompt file has invalid YAML frontmatter, **When** the server loads prompts, **Then** that prompt is skipped silently (with optional warning logged for debugging)
2. **Given** a prompt file is missing required metadata fields (`title` or `description`), **When** the server loads prompts, **Then** that prompt is skipped silently (treated as invalid)
3. **Given** a prompt contains invalid Jinja template syntax, **When** an MCP client requests that prompt, **Then** a JSON-RPC error is returned indicating template rendering failure
4. **Given** a prompt uses a parameter in the template but doesn't declare it in frontmatter, **When** the prompt is retrieved without that argument, **Then** the template renders with an empty value or returns a validation error

---

### User Story 4 - Document Local Prompts Feature (Priority: P2)

Users need comprehensive documentation to understand how to create, organize, and use local prompts effectively with the Twig MCP server.

**Why this priority**: Without documentation, users cannot discover or properly use the feature. This is essential for feature adoption but can be completed after core functionality is working.

**Independent Test**: Can be tested by reviewing documentation in `website/docs/` for completeness, accuracy, and clarity.

**Acceptance Scenarios**:

1. **Given** a user reads the documentation, **When** they follow the setup instructions, **Then** they can successfully create their first local prompt
2. **Given** documentation includes YAML frontmatter examples, **When** a user copies an example, **Then** the prompt works without modification
3. **Given** documentation describes parameter usage, **When** a user creates a parameterized prompt following the examples, **Then** parameter substitution works as documented
4. **Given** documentation covers common errors, **When** a user encounters an issue, **Then** they can find troubleshooting guidance in the docs

---

### Edge Cases

- What happens when the `.twig/prompts/` directory doesn't exist? (System should handle gracefully, return empty prompt list)
- How does the system handle subdirectories in `.twig/prompts/`? (Subdirectories are ignored; only top-level `.md` files are scanned)
- What happens when a Jinja template contains syntax errors? (Validation occurs at render time; JSON-RPC error returned to client when prompt is requested)
- How are files with no `.md` extension handled? (Should be ignored)
- What happens when a prompt is requested with missing required arguments? (Should return JSON-RPC error -32602)
- How does the system handle very large prompt files? (Standard file size limits apply)
- What happens when YAML frontmatter conflicts with the prompt name? (Filename takes precedence for the `name` field)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST scan only top-level files in the `.twig/prompts/` directory (relative to server start location) for Markdown files (.md extension), ignoring any subdirectories
- **FR-002**: System MUST parse YAML frontmatter from each Markdown file to extract prompt metadata; files with invalid YAML or missing required fields (`title`, `description`) are skipped silently (optional warnings may be logged)
- **FR-003**: System MUST use the filename (without .md extension) as the prompt's unique identifier
- **FR-004**: System MUST expose discovered prompts via the MCP `prompts/list` endpoint
- **FR-005**: System MUST support prompt parameters declared in YAML frontmatter
- **FR-006**: System MUST render prompt content using Jinja template syntax, substituting provided arguments; template syntax validation occurs at render time (lazy validation)
- **FR-007**: System MUST return rendered prompts via the MCP `prompts/get` endpoint
- **FR-008**: System MUST declare the `prompts` capability with `listChanged: true` during MCP initialization
- **FR-009**: System MUST send `prompts/list_changed` notifications when the prompt directory contents change; implementation should use native file system watching with polling fallback for cross-platform compatibility
- **FR-010**: System MUST validate that required prompt arguments are provided when retrieving a prompt
- **FR-011**: System MUST return standard JSON-RPC errors for invalid prompt requests (missing arguments, unknown prompts, etc.)
- **FR-012**: System MUST support standard MCP prompt metadata fields: name (derived from filename), title (required in YAML), description (required in YAML), arguments (optional in YAML)
- **FR-013**: System MUST return prompt messages in the format expected by MCP clients (role, content)
- **FR-014**: Documentation MUST be added to `website/docs/` directory explaining local prompts feature
- **FR-015**: Documentation MUST include complete examples of prompt files with YAML frontmatter
- **FR-016**: Documentation MUST explain the YAML frontmatter structure, specifying that `title` and `description` are required fields while `arguments` is optional
- **FR-017**: Documentation MUST demonstrate parameter usage with Jinja template syntax examples
- **FR-018**: Documentation MUST describe how MCP clients discover and use local prompts
- **FR-019**: Documentation MUST include troubleshooting guidance for common errors

### Key Entities

- **Prompt File**: A Markdown file in `.twig/prompts/` containing YAML frontmatter with metadata and body content with optional Jinja templates
  - Attributes: filename, title (from YAML, required), description (from YAML, required), arguments (from YAML, optional), content body
  - Name derived from: filename without .md extension
  - Validation: Files missing `title` or `description` in YAML frontmatter are skipped as invalid
  
- **Prompt Parameter**: A declared input variable that can be substituted into the prompt template
  - Attributes: name, description, required flag
  - Used in: Jinja template expressions within prompt content
  - Note: Default values are not supported in MVP; optional parameters with no argument provided render as empty string per minijinja behavior

- **Documentation Page**: User-facing documentation in `website/docs/` directory
  - Content: Feature overview, setup instructions, YAML structure reference, Jinja template examples, troubleshooting guide
  - Audience: Twig users who want to create and manage local prompts

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: MCP clients can discover all valid prompt files in `.twig/prompts/` within 100ms of server startup
- **SC-002**: Prompt templates with parameters render correctly with argument substitution in under 50ms
- **SC-003**: Changes to the prompts directory are detected and clients notified within 2 seconds (using native file watching or polling fallback)
- **SC-004**: Users can create and use a functional parameterized prompt end-to-end in under 5 minutes by following the documentation
- **SC-005**: System handles at least 100 prompt files without performance degradation
- **SC-006**: Invalid prompt files (malformed YAML) are gracefully skipped at load time without affecting valid prompts or blocking server startup
- **SC-007**: Documentation enables 90% of users to successfully create their first local prompt without additional support

## Assumptions

- Users are familiar with Markdown and YAML frontmatter format
- Users understand basic Jinja template syntax for parameter substitution
- The `.twig/prompts/` directory structure is flat (no nested subdirectories); subdirectories are ignored
- Prompt files use UTF-8 encoding
- The MCP server has read access to the `.twig/prompts/` directory
- File system watching uses native mechanisms with polling fallback to ensure cross-platform compatibility
