# Feature Specification: User-Wide Prompts via MCP

**Feature Branch**: `001-user-prompts-mcp`  
**Created**: 2025-11-11  
**Status**: Draft  
**Input**: User description: "Make user wide prompts available via MCP. This feature is for software engineers, who use coding agents like OpenCode or Claude Code. See @docs/mcp/prompts.md for the protocol specification. The prompts are stored in a user specific data directory and organized as libraries. Later the libraries will be fetched from a registry or a Git repo, but this is not in the current scope. Each library has its own sudirectory with a `twig.toml` file. The file declares all prompts exposed by the library including its name, description and arguments acording to the MCP specification. The twig MCP server exposes all prompts from all libraries the user has setup. Each prompt is prefixed with the libraries name, which derived from the subdirectory name. For each prompt in a library, a markdown file with the same name as the prompt + the `.md` file extension must be found in the `prompts` subdirectory and serves the content, which is used in the get request."

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Discover Available Prompts (Priority: P1)

As a software engineer using coding agents like OpenCode or Claude Code, I want to discover what prompts are available in my user-specific prompt libraries so that I can use them to enhance my coding workflow.

**Why this priority**: This is the foundational capability that enables all other prompt functionality. Without discovery, users cannot access any prompts.

**Independent Test**: Can be fully tested by listing available prompts and verifying that all user-installed library prompts are visible with proper library prefixes.

**Acceptance Scenarios**:

1. **Given** a user has prompt libraries installed in their data directory, **When** they request to list available prompts via MCP, **Then** the system returns all prompts from all libraries with library-name prefixes
2. **Given** a user has no prompt libraries installed, **When** they request to list available prompts, **Then** the system returns an empty list without errors
3. **Given** a prompt library contains multiple prompts, **When** listing prompts, **Then** each prompt shows its full name including the library prefix (e.g., "mycodinglib:code_review")

---

### User Story 2 - Retrieve Prompt Content (Priority: P1)

As a software engineer, I want to retrieve the content of a specific prompt with appropriate arguments so that I can use it in my coding agent for specific tasks.

**Why this priority**: This is the core functionality that allows users to actually use prompts after discovering them.

**Independent Test**: Can be fully tested by retrieving prompt content and verifying that arguments are properly substituted and the markdown content is returned as expected.

**Acceptance Scenarios**:

1. **Given** a valid prompt name and required arguments, **When** requesting prompt content via MCP, **Then** the system returns the prompt content with arguments properly substituted
2. **Given** a prompt with optional arguments, **When** requesting prompt content without optional arguments, **Then** the system returns content with default values or placeholders
3. **Given** an invalid prompt name, **When** requesting prompt content, **Then** the system returns an appropriate error indicating the prompt was not found

---

### User Story 3 - Handle Library Configuration Errors (Priority: P2)

As a software engineer, I want to receive clear error messages when my prompt libraries have configuration issues so that I can quickly fix problems and make my prompts available.

**Why this priority**: Error handling is crucial for user experience, but basic functionality (discovery and retrieval) can work even with some libraries having issues.

**Independent Test**: Can be fully tested by creating libraries with various configuration problems and verifying that appropriate error messages are generated without breaking the overall system.

**Acceptance Scenarios**:

1. **Given** a library with a malformed twig.toml file, **When** the system scans for prompts, **Then** it logs an error for that specific library but continues processing other libraries
2. **Given** a library with missing required prompt markdown files, **When** retrieving those prompts, **Then** the system returns a clear error indicating which file is missing
3. **Given** a library with invalid argument definitions, **When** trying to use affected prompts, **Then** the system provides specific error details about the configuration issue

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right edge cases.
-->

- What happens when a twig.toml file is malformed or missing required fields?
- How does the system handle duplicate prompt names across different libraries?
- What happens when a prompt's markdown file is missing or empty?
- How does the system handle very large prompt libraries with hundreds of prompts?
- What happens when the user data directory is not accessible or has permission issues?
- How does the system handle prompt names with special characters or spaces?

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST scan user-specific data directory for prompt libraries organized in subdirectories
- **FR-002**: System MUST read and parse twig.toml files from each library directory to discover available prompts
- **FR-003**: System MUST expose all discovered prompts via MCP protocol with library-name prefixes using colon separator format (e.g., "library_name:prompt_name")
- **FR-004**: System MUST support MCP `prompts/list` request to enumerate all available prompts
- **FR-005**: System MUST support MCP `prompts/get` request to retrieve specific prompt content
- **FR-006**: System MUST read prompt content from markdown files in the library's prompts subdirectory
- **FR-007**: System MUST validate that required prompt arguments are provided when retrieving prompt content
- **FR-008**: System MUST handle argument substitution in prompt content when arguments are provided
- **FR-009**: System MUST return appropriate MCP error responses for invalid requests (missing prompts, invalid arguments, etc.)
- **FR-010**: System MUST declare the `prompts` capability during MCP initialization with `listChanged` support and use file system watching to detect library changes in real-time

*Example of marking unclear requirements:*

- **FR-006**: System MUST authenticate users via [NEEDS CLARIFICATION: auth method not specified - email/password, SSO, OAuth?]
- **FR-007**: System MUST retain user data for [NEEDS CLARIFICATION: retention period not specified]

### Key Entities *(include if feature involves data)*

- **Prompt Library**: A collection of related prompts organized in a subdirectory containing a twig.toml configuration file and a prompts subdirectory with markdown content files
- **Prompt Definition**: Metadata about a prompt including name, description, arguments specification, as defined in twig.toml
- **Prompt Content**: The actual text content of a prompt stored in a markdown file, potentially containing argument placeholders
- **Prompt Argument**: A parameter that can be passed to customize prompt content, with name, description, and required/optional status

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: Users can discover all their installed prompt libraries within 2 seconds of request
- **SC-002**: Prompt content retrieval responds within 500ms for prompts with up to 10 arguments
- **SC-003**: 100% of valid prompt libraries in the user data directory are successfully discovered and exposed
- **SC-004**: All MCP prompt operations comply with the MCP specification as defined in @docs/mcp/prompts.md
- **SC-005**: Users can organize prompts into unlimited number of libraries without performance degradation
- **SC-006**: Error messages clearly indicate the specific issue (missing file, invalid configuration, etc.) to help users troubleshoot

## Assumptions

- User data directory location follows platform conventions (e.g., ~/.local/share/twig/prompts on Linux, ~/Library/Application Support/twig/prompts on macOS)
- twig.toml files use TOML format with minimal schema: `name`, `description`, `arguments[]` where each argument has `name`, `description`, and `required` fields
- Markdown files use standard markdown syntax and Jinja2 template syntax for argument substitution (e.g., `{{ argument_name }}`)
- Library names are derived from directory names using normalization: convert to lowercase, replace spaces and special characters with underscores, remove leading/trailing underscores (e.g., "My-Coding Lib" → "my_coding_lib")
- Prompt names within a library must be unique, but can be duplicated across different libraries
- The system will not automatically fetch libraries from registries or Git repos in this initial implementation

## Clarifications

### Session 2025-11-11

- Q: What is the exact TOML schema for twig.toml files? → A: Minimal schema with name, description, and arguments array, similar to MCP prompt definitions
- Q: How should the system format the combined prompt name that includes the library prefix? → A: Use colon separator format (e.g., "library_name:prompt_name")
- Q: What templating syntax should be used for substituting arguments into prompt content? → A: Use Jinja2 template syntax (e.g., `{{ argument_name }}`)
- Q: How should the system handle special characters, spaces, and case in directory names when creating library names? → A: Convert to lowercase, replace spaces/special chars with underscores, remove leading/trailing underscores
- Q: How should the system detect when libraries are added, removed, or modified? → A: Use file system watching (inotify/kqueue/FSEvents) for real-time change detection
