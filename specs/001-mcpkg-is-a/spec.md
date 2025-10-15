# Feature Specification: MCP Package Manager for AI Coding Agent Prompts and Resources

**Feature Branch**: `001-mcpkg-is-a`  
**Created**: 2025-10-14  
**Status**: Draft  
**Input**: User description: "mcpkg is a tool to manage project specific prompts and resources for AI coding agents. The prompts and resources are provided via the model context protocol (MCP). Each user can create multiple workspaces (each workspace is a SQLite 3 DB) and multiple projects within the workspace. Prompts and resources belong to a project."

## Clarifications

### Session 2025-10-14

- Q: Who is the primary user of mcpkg? → A: Software engineers using AI coding agents
- Q: Is workspace creation required as a manual step? → A: No, a default workspace is automatically created; users don't have to manually create a workspace
- Q: How is workspace selected when starting the MCP server? → A: Via optional `--workspace` flag on start command; defaults to default workspace if omitted
- Q: When is the workspace created? → A: Automatically when the start command is executed
- Q: How should resource content be stored and served? → A: Resources are stored in the database
- Q: What should the default workspace be named? → A: default
- Q: When a workspace database file is corrupted or unreadable, what should the system do? → A: Fail immediately with error message; require manual intervention
- Q: Should the system allow multiple MCP server instances to run simultaneously? → A: Allow unlimited concurrent instances; rely on SQLite locking
- Q: What constraints should apply to workspace, project, prompt, and resource names? → A: Alphanumeric plus hyphens, underscores, dots (a-z, A-Z, 0-9, -, _, .)
- Q: What interfaces does mcpkg provide? → A: Command-line interface (CLI) for user management operations, and MCP API for AI agent operations
- Q: Can MCP clients manage prompts and resources? → A: Yes, MCP server must provide tools for AI agents to create and manage prompts and resources
- Q: What functionality should the CLI provide? → A: Manage projects within workspace, manage prompts and resources in projects
- Q: How should the CLI commands be structured? → A: Noun-verb structure (e.g., mcpkg project create, mcpkg prompt add, mcpkg resource list)
- Q: When an MCP tool operation fails, how should the error be communicated? → A: Return MCP error response with structured error code and descriptive message
- Q: What metadata fields should resources support beyond name, URI, and content? → A: MIME type, description, created/updated timestamps
- Q: Should prompts also have metadata fields like resources for consistency? → A: Description and created/updated timestamps (no MIME type)
- Q: How should CLI commands specify which workspace to operate on? → A: Optional --workspace flag; defaults to "default" workspace if omitted
- Q: What type of primary keys should database tables use? → A: UUIDs for all primary keys
- Q: How should migration script versions be identified and ordered? → A: Date and time at beginning of migration filename (e.g., YYYYMMDDHHMMSS_description.sql)
- Q: When database connection or migration fails during MCP server startup, how should it behave? → A: Fail immediately with descriptive error and exit code
- Q: What logging behavior should the system provide during normal operation? → A: Log to stderr at info level by default
- Q: What maximum length limit should apply to workspace, project, prompt, and resource names? → A: 255 characters

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Work with Default Workspace (Priority: P1)

As a software engineer, I need a default workspace automatically created for me so that I can immediately start organizing prompts and resources without manual setup.

**Why this priority**: Providing a default workspace reduces friction and allows users to get started immediately. This is the foundational container for all other data.

**Independent Test**: Can be fully tested by running the start command (which auto-creates default workspace), creating projects in it, and verifying workspace exists. Delivers the value of zero-configuration startup.

**Acceptance Scenarios**:

1. **Given** no workspaces exist, **When** user runs the start command without `--workspace` flag, **Then** a workspace named "default" is automatically created
2. **Given** "default" workspace already exists, **When** user runs the start command without `--workspace` flag, **Then** the existing "default" workspace is used
3. **Given** user wants to use a different workspace, **When** user runs start command with `--workspace custom-name`, **Then** that workspace is created if it doesn't exist, or used if it does
4. **Given** multiple workspaces exist, **When** user lists all workspaces, **Then** all workspace names are displayed including the default
5. **Given** a workspace with no projects, **When** user deletes the workspace, **Then** the workspace and its database file are removed
6. **Given** a workspace with existing projects, **When** user attempts to delete it, **Then** user is warned and must confirm deletion

---

### User Story 2 - Organize Projects Within Workspaces (Priority: P2)

As a software engineer, I need to create projects within a workspace to group related prompts and resources (e.g., "authentication-module", "payment-service") so that I can organize my AI assistance materials by functional area.

**Why this priority**: Projects provide the organizational structure within workspaces. This enables users to logically separate prompts/resources but depends on workspaces existing first.

**Independent Test**: Can be fully tested by creating projects within a workspace, listing projects, renaming projects, and deleting projects. Delivers the value of logical organization within a workspace context.

**Acceptance Scenarios**:

1. **Given** a workspace exists, **When** user creates a project named "auth-module" using CLI (optionally specifying `--workspace`, defaulting to "default"), **Then** the project is created in the specified workspace
2. **Given** multiple projects in a workspace, **When** user lists all projects in that workspace, **Then** all project names are displayed
3. **Given** a project with no prompts or resources, **When** user deletes the project, **Then** the project is removed from the workspace
4. **Given** a project with existing prompts or resources, **When** user attempts to delete it, **Then** user is warned and must confirm deletion
5. **Given** an existing project, **When** user renames the project, **Then** the project name is updated without affecting its prompts or resources

---

### User Story 3 - Add and Retrieve Prompts (Priority: P3)

As a software engineer, I need to add prompts to my projects and have them accessible via MCP so that my AI coding assistant can discover and use these project-specific prompts during our interactions.

**Why this priority**: Prompts are core content but require both workspaces and projects to exist. This delivers the primary value proposition of the tool.

**Independent Test**: Can be fully tested by adding a prompt to a project, retrieving it via MCP, listing all prompts in a project, and deleting a prompt. Delivers the value of AI-accessible prompt management.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** user adds a prompt with a name, content, and optional description using CLI with `--project` flag (optionally `--workspace`, defaulting to "default"), **Then** the prompt is stored in the project with auto-generated timestamps
2. **Given** prompts exist in a project and MCP server is started with that project, **When** an MCP client lists available prompts, **Then** all prompts from that project are returned
3. **Given** a specific prompt exists in the selected project, **When** an MCP client requests that prompt by name, **Then** the prompt content is returned
4. **Given** multiple prompts in a project, **When** user lists all prompts in that project, **Then** all prompt names, descriptions, and timestamps are displayed
5. **Given** an existing prompt, **When** user updates the prompt content or description, **Then** the new values replace the old values and the updated timestamp is refreshed
6. **Given** an existing prompt, **When** user deletes the prompt, **Then** the prompt is removed from the project

---

### User Story 4 - Add and Retrieve Resources (Priority: P4)

As a software engineer, I need to add resources (files, documentation, code snippets) to my projects and have them accessible via MCP so that my AI coding assistant can reference these materials when helping me with project-specific tasks.

**Why this priority**: Resources complement prompts and follow the same organizational pattern. Lower priority than prompts as prompts are typically more critical for AI interaction patterns.

**Independent Test**: Can be fully tested by adding a resource to a project, retrieving it via MCP, listing all resources in a project, and deleting a resource. Delivers the value of AI-accessible resource management.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** user adds a resource with a name, URI, content, and optional metadata (MIME type, description) using CLI with `--project` flag (optionally `--workspace`, defaulting to "default"), **Then** the resource, its content, and metadata are stored in the project database with auto-generated timestamps
2. **Given** resources exist in a project and MCP server is started with that project, **When** an MCP client lists available resources, **Then** all resources from that project are returned
3. **Given** a specific resource exists in the selected project, **When** an MCP client requests that resource by URI, **Then** the resource content is returned from the database
4. **Given** multiple resources in a project, **When** user lists all resources in that project, **Then** all resource names, URIs, MIME types, descriptions, and timestamps are displayed
5. **Given** an existing resource, **When** user updates the resource metadata or content, **Then** the new information replaces the old information
6. **Given** an existing resource, **When** user deletes the resource, **Then** the resource is removed from the project

---

### User Story 5 - Start MCP Server with Project Context (Priority: P5)

As a software engineer, I need to start the MCP server with a specific project selected so that my AI assistant only sees prompts and resources relevant to that project context.

**Why this priority**: This ensures clean scoping and prevents ambiguity about which project's content should be exposed via MCP. Depends on all previous features being implemented.

**Independent Test**: Can be fully tested by creating multiple workspaces and projects, adding different prompts to each, starting the MCP server with different workspace and project selections, and verifying MCP only returns items from the specified project. Delivers the value of focused, context-appropriate AI assistance.

**Acceptance Scenarios**:

1. **Given** user omits `--workspace` flag, **When** user starts MCP server with `--project` flag, **Then** default workspace is used and server starts with that project from the default workspace
2. **Given** user provides `--workspace custom` flag, **When** user starts MCP server with `--project` flag, **Then** the custom workspace is created (if needed) and used, and server starts with that project
3. **Given** user attempts to start MCP server, **When** user omits the `--project` flag, **Then** the command fails with an error message requiring project selection
4. **Given** user attempts to start MCP server, **When** user provides a `--project` flag pointing to a non-existent project, **Then** the command fails with an error message indicating the project does not exist
5. **Given** MCP server is running with project A selected, **When** MCP client queries for prompts or resources, **Then** only items from project A are returned

---

### User Story 6 - Manage Prompts and Resources via MCP Tools (Priority: P6)

As an AI coding agent, I need to create, update, and delete prompts and resources in the selected project via MCP tools so that I can manage project-specific materials during my interactions with the software engineer.

**Why this priority**: Enabling AI agents to manage their own prompts and resources allows for dynamic, context-aware assistance. This depends on all previous features and extends the MCP integration beyond read-only access.

**Independent Test**: Can be fully tested by invoking MCP tools to create/update/delete prompts and resources, then verifying changes via CLI or MCP queries. Delivers the value of AI-assisted content management.

**Acceptance Scenarios**:

1. **Given** MCP server is running with a project selected, **When** AI agent invokes create_prompt tool with name, content, and optional description, **Then** the prompt is created in the selected project with auto-generated timestamps
2. **Given** a prompt exists in the selected project, **When** AI agent invokes update_prompt tool with new content or description, **Then** the prompt is updated and the updated timestamp is refreshed
3. **Given** a prompt exists in the selected project, **When** AI agent invokes delete_prompt tool, **Then** the prompt is removed from the project
4. **Given** MCP server is running with a project selected, **When** AI agent invokes create_resource tool with name, URI, content, and optional metadata (MIME type, description), **Then** the resource is created in the selected project with auto-generated timestamps
5. **Given** a resource exists in the selected project, **When** AI agent invokes update_resource tool with new content or metadata, **Then** the resource is updated
6. **Given** a resource exists in the selected project, **When** AI agent invokes delete_resource tool, **Then** the resource is removed from the project

---

### Edge Cases

- What happens when a workspace database file is corrupted or missing? System fails immediately with clear error message and non-zero exit code requiring manual intervention (user must restore from backup or delete corrupted file)
- What happens when database connection fails during MCP server startup? System fails immediately with descriptive error message and non-zero exit code
- What happens when database migration fails during MCP server startup? System fails immediately with descriptive error message and non-zero exit code, preventing server from starting in inconsistent state
- What happens when user attempts to create a workspace or project with a duplicate name? CLI returns error message; MCP operations for projects are not supported (workspace/project management is CLI-only)
- What happens when user attempts to add a prompt or resource with a name that already exists in the project? CLI returns error message; MCP tool returns structured error response with appropriate error code
- What happens when user attempts to delete a workspace that contains a project being served by a running MCP server? System allows deletion; running MCP servers will encounter errors when trying to access deleted data
- What happens when user attempts to delete a project while an MCP server is running with that project selected? System allows deletion; running MCP server will encounter errors when trying to access the deleted project
- How does system handle concurrent access to the same workspace database from multiple processes? System allows multiple MCP server instances; SQLite built-in locking handles concurrent access (multiple readers, single writer)
- What happens when a prompt or resource name contains special characters or exceeds length limits? System rejects names containing characters outside allowed set (a-z, A-Z, 0-9, -, _, .) or exceeding 255 characters with clear error message
- What happens when user provides invalid characters in workspace or project names? System rejects with error message specifying allowed characters (alphanumeric, hyphen, underscore, dot) and maximum length (255 characters)

## Requirements *(mandatory)*

### Functional Requirements

#### Workspace Management

- **FR-001**: System MUST automatically create a workspace named "default" on first start command if no workspaces exist
- **FR-002**: System MUST use the default workspace when `--workspace` flag is omitted from start command
- **FR-003**: System MUST allow users to specify a workspace via `--workspace` flag on start command
- **FR-004**: System MUST automatically create a workspace if specified via `--workspace` flag and it doesn't exist
- **FR-005**: System MUST store each workspace as a separate SQLite 3 database file
- **FR-006**: System MUST provide CLI command to list all available workspaces (e.g., `mcpkg workspace list`)
- **FR-007**: System MUST provide CLI command to delete a workspace and its associated database file (e.g., `mcpkg workspace delete`)
- **FR-008**: System MUST prevent creation of workspaces with duplicate names
- **FR-008a**: System MUST restrict workspace names to alphanumeric characters, hyphens, underscores, and dots (a-z, A-Z, 0-9, -, _, .)
- **FR-008b**: System MUST reject workspace names containing invalid characters with a clear error message
- **FR-008c**: System MUST enforce a maximum length of 255 characters for workspace names

#### Project Management

- **FR-009**: System MUST provide CLI command to create projects within a workspace (e.g., `mcpkg project create`)
- **FR-009a**: System MUST accept optional `--workspace` flag on project management commands; default to "default" workspace if omitted
- **FR-010**: System MUST provide CLI command to list all projects in a workspace (e.g., `mcpkg project list`)
- **FR-011**: System MUST provide CLI command to delete projects from a workspace (e.g., `mcpkg project delete`)
- **FR-012**: System MUST provide CLI command to rename projects (e.g., `mcpkg project rename`)
- **FR-013**: System MUST prevent creation of projects with duplicate names within the same workspace
- **FR-013a**: System MUST restrict project names to alphanumeric characters, hyphens, underscores, and dots (a-z, A-Z, 0-9, -, _, .)
- **FR-013b**: System MUST reject project names containing invalid characters with a clear error message
- **FR-013c**: System MUST enforce a maximum length of 255 characters for project names

#### Prompt Management

- **FR-014**: System MUST provide CLI command to add prompts to a project with a name, content, and optional description (e.g., `mcpkg prompt add`)
- **FR-014a**: System MUST support the following metadata fields for prompts: description, created timestamp, updated timestamp
- **FR-014b**: System MUST automatically set created timestamp when a prompt is added
- **FR-014c**: System MUST automatically update the updated timestamp when a prompt is modified
- **FR-014d**: System MUST accept optional `--workspace` flag on prompt management commands; default to "default" workspace if omitted
- **FR-014e**: System MUST require `--project` flag on prompt management commands to specify target project
- **FR-015**: System MUST provide CLI command to list all prompts in a project (e.g., `mcpkg prompt list`)
- **FR-016**: System MUST provide CLI command to update existing prompt content and description (e.g., `mcpkg prompt update`)
- **FR-017**: System MUST provide CLI command to delete prompts from a project (e.g., `mcpkg prompt delete`)
- **FR-018**: System MUST associate each prompt with exactly one project
- **FR-019**: System MUST prevent creation of prompts with duplicate names within the same project
- **FR-019a**: System MUST restrict prompt names to alphanumeric characters, hyphens, underscores, and dots (a-z, A-Z, 0-9, -, _, .)
- **FR-019b**: System MUST reject prompt names containing invalid characters with a clear error message
- **FR-019c**: System MUST enforce a maximum length of 255 characters for prompt names

#### Resource Management

- **FR-020**: System MUST provide CLI command to add resources to a project with a name, URI, content, and optional metadata (e.g., `mcpkg resource add`)
- **FR-020a**: System MUST support the following metadata fields for resources: MIME type, description, created timestamp, updated timestamp
- **FR-020b**: System MUST automatically set created timestamp when a resource is added
- **FR-020c**: System MUST automatically update the updated timestamp when a resource is modified
- **FR-020d**: System MUST accept optional `--workspace` flag on resource management commands; default to "default" workspace if omitted
- **FR-020e**: System MUST require `--project` flag on resource management commands to specify target project
- **FR-021**: System MUST store resource content in the database
- **FR-022**: System MUST provide CLI command to list all resources in a project (e.g., `mcpkg resource list`)
- **FR-023**: System MUST provide CLI command to update existing resource content and metadata (e.g., `mcpkg resource update`)
- **FR-024**: System MUST provide CLI command to delete resources from a project (e.g., `mcpkg resource delete`)
- **FR-025**: System MUST associate each resource with exactly one project
- **FR-026**: System MUST prevent creation of resources with duplicate identifiers within the same project
- **FR-026a**: System MUST restrict resource names to alphanumeric characters, hyphens, underscores, and dots (a-z, A-Z, 0-9, -, _, .)
- **FR-026b**: System MUST reject resource names containing invalid characters with a clear error message
- **FR-026c**: System MUST enforce a maximum length of 255 characters for resource names

#### MCP Integration

- **FR-027**: System MUST provide a start command that launches the MCP server
- **FR-028**: System MUST require a `--project` flag when starting the MCP server
- **FR-029**: System MUST accept an optional `--workspace` flag when starting the MCP server
- **FR-030**: System MUST fail to start MCP server if `--project` flag is missing
- **FR-031**: System MUST fail to start MCP server if `--project` flag points to a non-existent project
- **FR-032**: System MUST expose prompts from the selected project via MCP protocol
- **FR-033**: System MUST expose resources from the selected project via MCP protocol
- **FR-034**: System MUST implement MCP list_prompts operation to return available prompts from the selected project
- **FR-035**: System MUST implement MCP get_prompt operation to return specific prompt content from the selected project
- **FR-036**: System MUST implement MCP list_resources operation to return available resources from the selected project
- **FR-037**: System MUST implement MCP read_resource operation to return resource content from the database
- **FR-038a**: System MUST provide MCP tools for AI agents to create prompts in the selected project
- **FR-038b**: System MUST provide MCP tools for AI agents to update prompts in the selected project
- **FR-038c**: System MUST provide MCP tools for AI agents to delete prompts from the selected project
- **FR-038d**: System MUST provide MCP tools for AI agents to create resources in the selected project
- **FR-038e**: System MUST provide MCP tools for AI agents to update resources in the selected project
- **FR-038f**: System MUST provide MCP tools for AI agents to delete resources from the selected project
- **FR-038g**: System MUST return MCP error responses with structured error codes and descriptive messages when tool operations fail

#### Data Persistence

- **FR-039**: System MUST persist all workspace, project, prompt, and resource data in SQLite 3 databases
- **FR-039a**: System MUST use UUIDs as primary keys for all database tables (projects, prompts, resources)
- **FR-040**: System MUST store resource content directly in the database (not as file references)
- **FR-041**: System MUST maintain data integrity when working with multiple workspaces
- **FR-042**: System MUST preserve all data when system is restarted
- **FR-043**: System MUST fail with a clear error message when a workspace database file is corrupted or unreadable
- **FR-044**: System MUST NOT attempt automatic recovery of corrupted database files
- **FR-045**: System MUST allow multiple MCP server instances to run concurrently
- **FR-046**: System MUST rely on SQLite's built-in locking mechanisms for concurrent database access
- **FR-047**: System MUST name migration scripts using date-time prefix format (YYYYMMDDHHMMSS_description.sql)
- **FR-048**: System MUST track the latest applied migration version in the database
- **FR-049**: System MUST apply migrations in chronological order based on date-time prefix
- **FR-050**: System MUST run pending migrations automatically when opening a workspace database
- **FR-051**: System MUST fail MCP server startup immediately with descriptive error and non-zero exit code if database connection fails
- **FR-052**: System MUST fail MCP server startup immediately with descriptive error and non-zero exit code if migration execution fails
- **FR-053**: System MUST log operational messages to stderr at info level by default
- **FR-054**: System MUST keep stdout reserved for data output (MCP protocol communication)
- **FR-055**: System MUST log at minimum: server startup/shutdown, database connections, migration execution, and errors

### Key Entities

- **Workspace**: Represents an isolated container for projects. Each workspace has a unique name and is backed by a separate SQLite 3 database file. A workspace named "default" is automatically created on first use. Users can specify different workspaces via the `--workspace` flag.

- **Project**: Represents a logical grouping of related prompts and resources within a workspace. Each project has a UUID primary key, unique name within its workspace, and contains zero or more prompts and resources.

- **Prompt**: Represents a reusable text prompt for AI coding agents. Each prompt has a UUID primary key, unique name within its project, text content, optional metadata (description, created/updated timestamps), and belongs to exactly one project. Prompts are exposed via MCP for AI agent consumption.

- **Resource**: Represents a file, documentation, or reference material for AI coding agents. Each resource has a UUID primary key, unique identifier within its project, a URI for reference, content stored in the database, optional metadata (MIME type, description, created/updated timestamps), and belongs to exactly one project. Resources are exposed via MCP for AI agent consumption.

- **Migration**: Database schema migration scripts named with date-time prefix (YYYYMMDDHHMMSS_description.sql). The system tracks the latest applied migration version and automatically applies pending migrations when opening a workspace database.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can create a new workspace, add a project, and add a prompt in under 1 minute
- **SC-002**: Users can work with different workspaces using the `--workspace` flag and access different sets of prompts without data loss
- **SC-003**: AI coding agents can discover and retrieve prompts from the active project via MCP protocol
- **SC-004**: AI coding agents can discover and retrieve resources from the active project via MCP protocol
- **SC-005**: System maintains data isolation between workspaces (no cross-contamination of prompts/resources)
- **SC-006**: Users can manage at least 10 workspaces, each with 20 projects, each with 50 prompts/resources without performance degradation
- **SC-007**: MCP queries return results in under 100 milliseconds for datasets within typical usage parameters

## Assumptions *(mandatory)*

- Primary users are software engineers working with AI coding agents
- Users interact with the system via a command-line interface
- Workspace database files are stored in a user-accessible directory
- SQLite 3 is sufficient for storing workspace data (no need for client-server database)
- Users run mcpkg on a single machine (no distributed/networked access required)
- Users understand basic concepts of workspaces and projects from experience with other tools (IDEs, version control)
- MCP server runs locally on the user's machine
- MCP server is started with a specific project selected via `--project` flag
- Default workspace is sufficient for most users; custom workspaces are for advanced organization needs
- Multiple MCP server instances can run concurrently serving different or same projects
- SQLite's built-in locking handles concurrent database access (WAL mode recommended for better concurrency)
- Prompt content is text-based (no binary or rich media content)
- Resource URIs serve as identifiers and references; actual content is stored in the database
- Resource content is stored as text or binary data in the database
- Each MCP server instance serves content from exactly one project at a time
- Logging to stderr at info level provides sufficient operational visibility for troubleshooting
- Users can redirect stderr to a file if they need persistent logs

## Out of Scope *(mandatory)*

- Sharing workspaces or projects between multiple users
- Cloud synchronization of workspaces
- Version control or history tracking for prompts and resources
- Access control or permission management within workspaces
- Import/export of workspaces to other formats
- Search functionality across multiple workspaces
- Tags or categories for organizing prompts and resources within projects
- Visual or graphical user interface (GUI) - mcpkg provides CLI and MCP API only
- Integration with specific AI coding agent platforms beyond MCP protocol
- Automatic backup or recovery of workspace databases
- Migration tools from other prompt management systems
- MCP tools for workspace or project management (AI agents can only manage prompts/resources)
