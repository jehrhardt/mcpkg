# Tasks: MCP Package Manager for AI Coding Agent Prompts and Resources

**Input**: Design documents from `/specs/001-mcpkg-is-a/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are written BEFORE implementation following TDD principles (per constitution requirement)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- Source code: `mcpkg/` at repository root
- Tests: `mcpkg/tests/` (functional, integration, unit)
- Migrations: `mcpkg/migrations/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Add development dependencies to pyproject.toml (pytest>=8.4.2, pytest-asyncio>=0.24.0)
- [X] T002 Add runtime dependency to pyproject.toml (platformdirs>=4.3.6)
- [X] T003 Create pytest configuration file pytest.ini with asyncio_mode=auto
- [X] T004 Create mcpkg/tests/ directory structure (functional/, integration/, unit/)
- [X] T005 Create mcpkg/migrations/ directory for SQL migration files

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Data Models and Validation

- [X] T006 [P] Write unit tests for name validation in mcpkg/tests/unit/test_validators.py
- [X] T007 [P] Implement name validation logic in mcpkg/validators.py (pattern: [a-zA-Z0-9._-]+, max 255 chars)
- [X] T008 [P] Write unit tests for data models in mcpkg/tests/unit/test_models.py
- [X] T009 [P] Implement data models (Workspace, Project, Prompt, Resource) as dataclasses in mcpkg/models.py

### Storage and Database Core

- [X] T010 [P] Write unit tests for storage path functions in mcpkg/tests/unit/test_storage.py
- [X] T011 [P] Implement storage module for OS data directory paths in mcpkg/storage.py (using platformdirs)
- [X] T012 Write integration tests for database operations in mcpkg/tests/integration/test_database.py
- [X] T013 Implement database connection and initialization in mcpkg/database.py (WAL mode, foreign keys enabled)

### Migration System

- [X] T014 Create initial schema migration in mcpkg/migrations/001_initial_schema.sql (projects, prompts, resources, schema_migrations tables)
- [X] T015 Write integration tests for migration system in mcpkg/tests/integration/test_migrations.py
- [X] T016 Implement migration logic in mcpkg/migrations.py (apply pending migrations on DB open)

### SQL Query Layer

- [X] T017 Write integration tests for SQL queries in mcpkg/tests/integration/test_queries.py
- [X] T018 Implement SQL query functions in mcpkg/queries.py (workspace, project, prompt, resource CRUD operations)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Work with Default Workspace (Priority: P1) 🎯 MVP

**Goal**: Provide automatic default workspace creation for zero-configuration startup

**Independent Test**: Run start command without --workspace flag, verify default workspace is created; create projects in it, list workspaces

### Tests for User Story 1

**NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T019 [P] [US1] Functional test for workspace list command in mcpkg/tests/functional/test_cli_workspace.py
- [X] T020 [P] [US1] Functional test for workspace create command in mcpkg/tests/functional/test_cli_workspace.py
- [X] T021 [P] [US1] Functional test for workspace delete command in mcpkg/tests/functional/test_cli_workspace.py
- [X] T022 [P] [US1] Functional test for default workspace auto-creation in mcpkg/tests/functional/test_cli_workspace.py

### Implementation for User Story 1

- [X] T023 [US1] Implement workspace list command in mcpkg/cli.py (mcpkg workspace list)
- [X] T024 [US1] Implement workspace create command in mcpkg/cli.py (mcpkg workspace create <name>)
- [X] T025 [US1] Implement workspace delete command in mcpkg/cli.py (mcpkg workspace delete <name> --force)
- [X] T026 [US1] Add default workspace auto-creation logic to start command preparation in mcpkg/cli.py
- [X] T027 [US1] Add error handling and validation for workspace commands in mcpkg/cli.py

**Checkpoint**: At this point, User Story 1 should be fully functional - users can manage workspaces including automatic default workspace

---

## Phase 4: User Story 2 - Organize Projects Within Workspaces (Priority: P2)

**Goal**: Enable users to create and organize projects within workspaces for logical grouping

**Independent Test**: Create projects in a workspace, list projects, rename projects, delete projects - verify all operations work independently

### Tests for User Story 2

- [X] T028 [P] [US2] Functional test for project create command in mcpkg/tests/functional/test_cli_project.py
- [X] T029 [P] [US2] Functional test for project list command in mcpkg/tests/functional/test_cli_project.py
- [X] T030 [P] [US2] Functional test for project rename command in mcpkg/tests/functional/test_cli_project.py
- [X] T031 [P] [US2] Functional test for project delete command in mcpkg/tests/functional/test_cli_project.py

### Implementation for User Story 2

- [X] T032 [US2] Implement project create command in mcpkg/cli.py (mcpkg project create <name> --workspace <ws>)
- [X] T033 [US2] Implement project list command in mcpkg/cli.py (mcpkg project list --workspace <ws>)
- [X] T034 [US2] Implement project rename command in mcpkg/cli.py (mcpkg project rename <old> <new> --workspace <ws>)
- [X] T035 [US2] Implement project delete command in mcpkg/cli.py (mcpkg project delete <name> --workspace <ws> --force)
- [X] T036 [US2] Implement error handling and validation for project commands in mcpkg/cli.py

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - workspace and project management complete

---

## Phase 5: User Story 3 - Add and Retrieve Prompts (Priority: P3)

**Goal**: Enable adding prompts to projects and exposing them via MCP for AI assistant access

**Independent Test**: Add prompts to a project via CLI, start MCP server, retrieve prompts via MCP protocol, verify content matches

### Tests for User Story 3

- [ ] T037 [P] [US3] Functional test for prompt add command in mcpkg/tests/functional/test_cli_prompt.py
- [ ] T038 [P] [US3] Functional test for prompt list command in mcpkg/tests/functional/test_cli_prompt.py
- [ ] T039 [P] [US3] Functional test for prompt update command in mcpkg/tests/functional/test_cli_prompt.py
- [ ] T040 [P] [US3] Functional test for prompt delete command in mcpkg/tests/functional/test_cli_prompt.py
- [ ] T041 [P] [US3] Functional test for MCP prompts/list in mcpkg/tests/functional/test_mcp_server.py
- [ ] T042 [P] [US3] Functional test for MCP prompts/get in mcpkg/tests/functional/test_mcp_server.py

### Implementation for User Story 3

- [ ] T043 [US3] Implement prompt add command in mcpkg/cli.py (mcpkg prompt add <name> <content> --project <p> --workspace <ws> --description <d>)
- [ ] T044 [US3] Implement prompt list command in mcpkg/cli.py (mcpkg prompt list --project <p> --workspace <ws>)
- [ ] T045 [US3] Implement prompt update command in mcpkg/cli.py (mcpkg prompt update <name> --content <c> --description <d> --project <p>)
- [ ] T046 [US3] Implement prompt delete command in mcpkg/cli.py (mcpkg prompt delete <name> --project <p> --workspace <ws>)
- [ ] T047 [US3] Implement MCP prompts/list handler in mcpkg/mcp.py (@server.list_prompts decorator)
- [ ] T048 [US3] Implement MCP prompts/get handler in mcpkg/mcp.py (@server.get_prompt decorator)
- [ ] T049 [US3] Add error handling for prompt commands and MCP handlers in mcpkg/cli.py and mcpkg/mcp.py

**Checkpoint**: Prompts can now be managed via CLI and retrieved via MCP - core value proposition delivered

---

## Phase 6: User Story 4 - Add and Retrieve Resources (Priority: P4)

**Goal**: Enable adding resources to projects and exposing them via MCP for AI assistant reference

**Independent Test**: Add resources to a project via CLI, start MCP server, retrieve resources via MCP protocol, verify content matches

### Tests for User Story 4

- [ ] T050 [P] [US4] Functional test for resource add command in mcpkg/tests/functional/test_cli_resource.py
- [ ] T051 [P] [US4] Functional test for resource list command in mcpkg/tests/functional/test_cli_resource.py
- [ ] T052 [P] [US4] Functional test for resource update command in mcpkg/tests/functional/test_cli_resource.py
- [ ] T053 [P] [US4] Functional test for resource delete command in mcpkg/tests/functional/test_cli_resource.py
- [ ] T054 [P] [US4] Functional test for MCP resources/list in mcpkg/tests/functional/test_mcp_server.py
- [ ] T055 [P] [US4] Functional test for MCP resources/read in mcpkg/tests/functional/test_mcp_server.py

### Implementation for User Story 4

- [ ] T056 [US4] Implement resource add command in mcpkg/cli.py (mcpkg resource add <name> <uri> <content> --project <p> --workspace <ws> --mime-type <m> --description <d>)
- [ ] T057 [US4] Implement resource list command in mcpkg/cli.py (mcpkg resource list --project <p> --workspace <ws>)
- [ ] T058 [US4] Implement resource update command in mcpkg/cli.py (mcpkg resource update <name> --content <c> --mime-type <m> --description <d> --project <p>)
- [ ] T059 [US4] Implement resource delete command in mcpkg/cli.py (mcpkg resource delete <name> --project <p> --workspace <ws>)
- [ ] T060 [US4] Implement MCP resources/list handler in mcpkg/mcp.py (@server.list_resources decorator)
- [ ] T061 [US4] Implement MCP resources/read handler in mcpkg/mcp.py (@server.read_resource decorator)
- [ ] T062 [US4] Add error handling for resource commands and MCP handlers in mcpkg/cli.py and mcpkg/mcp.py

**Checkpoint**: Resources can now be managed via CLI and retrieved via MCP - full read access enabled

---

## Phase 7: User Story 5 - Start MCP Server with Project Context (Priority: P5)

**Goal**: Enable starting MCP server with specific project selection for focused context

**Independent Test**: Create multiple projects with different prompts/resources, start MCP server with different --project flags, verify MCP only returns content from selected project

### Tests for User Story 5

- [ ] T063 [P] [US5] Functional test for start command with project selection in mcpkg/tests/functional/test_mcp_server.py
- [ ] T064 [P] [US5] Functional test for start command with workspace and project in mcpkg/tests/functional/test_mcp_server.py
- [ ] T065 [P] [US5] Functional test for start command error cases (missing project, nonexistent project) in mcpkg/tests/functional/test_mcp_server.py
- [ ] T066 [P] [US5] Functional test for concurrent MCP servers in mcpkg/tests/functional/test_mcp_server.py

### Implementation for User Story 5

- [ ] T067 [US5] Implement start command in mcpkg/cli.py (mcpkg start --project <p> --workspace <ws>)
- [ ] T068 [US5] Add project context initialization to MCP server in mcpkg/mcp.py (filter queries by project_id)
- [ ] T069 [US5] Add validation for required --project flag in mcpkg/cli.py
- [ ] T070 [US5] Add database connection and migration execution on start in mcpkg/cli.py
- [ ] T071 [US5] Add logging to stderr (info level) in mcpkg/cli.py
- [ ] T072 [US5] Add error handling for database/migration failures in mcpkg/cli.py

**Checkpoint**: MCP server can be started with project-specific context, enabling focused AI assistance

---

## Phase 8: User Story 6 - Manage Prompts and Resources via MCP Tools (Priority: P6)

**Goal**: Enable AI agents to create, update, and delete prompts/resources via MCP tools for dynamic content management

**Independent Test**: Start MCP server, invoke MCP tools to create/update/delete prompts and resources, verify changes via CLI or MCP queries

### Tests for User Story 6

- [ ] T073 [P] [US6] Functional test for create_prompt tool in mcpkg/tests/functional/test_mcp_server.py
- [ ] T074 [P] [US6] Functional test for update_prompt tool in mcpkg/tests/functional/test_mcp_server.py
- [ ] T075 [P] [US6] Functional test for delete_prompt tool in mcpkg/tests/functional/test_mcp_server.py
- [ ] T076 [P] [US6] Functional test for create_resource tool in mcpkg/tests/functional/test_mcp_server.py
- [ ] T077 [P] [US6] Functional test for update_resource tool in mcpkg/tests/functional/test_mcp_server.py
- [ ] T078 [P] [US6] Functional test for delete_resource tool in mcpkg/tests/functional/test_mcp_server.py
- [ ] T079 [P] [US6] Functional test for MCP tools/list in mcpkg/tests/functional/test_mcp_server.py

### Implementation for User Story 6

- [ ] T080 [US6] Implement MCP tools/list handler in mcpkg/mcp.py (@server.list_tools decorator)
- [ ] T081 [US6] Implement create_prompt tool in mcpkg/mcp.py (@server.call_tool decorator)
- [ ] T082 [US6] Implement update_prompt tool in mcpkg/mcp.py (@server.call_tool decorator)
- [ ] T083 [US6] Implement delete_prompt tool in mcpkg/mcp.py (@server.call_tool decorator)
- [ ] T084 [US6] Implement create_resource tool in mcpkg/mcp.py (@server.call_tool decorator)
- [ ] T085 [US6] Implement update_resource tool in mcpkg/mcp.py (@server.call_tool decorator)
- [ ] T086 [US6] Implement delete_resource tool in mcpkg/mcp.py (@server.call_tool decorator)
- [ ] T087 [US6] Add structured error responses for tool operations in mcpkg/mcp.py (isError format)

**Checkpoint**: All user stories complete - full feature set delivered (CLI management + MCP read/write access)

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T088 [P] Update README.md with installation and usage instructions
- [ ] T089 [P] Update CLAUDE.md with any new context about the implementation
- [ ] T090 Run full test suite (uv run pytest) and verify all tests pass
- [ ] T091 Run format check (uv run ruff format --check)
- [ ] T092 Run linter (uv run ruff check)
- [ ] T093 Run type checker (uv run pyright)
- [ ] T094 Validate quickstart.md scenarios manually or with automated tests
- [ ] T095 Performance testing: verify <200ms CLI startup, <100ms MCP queries
- [ ] T096 Security review: check for SQL injection vulnerabilities, validate parameterized queries

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P4 → P5 → P6)
- **Polish (Phase 9)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent, no dependencies on US1
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Requires project context (US2 recommended but not blocking)
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Requires project context (US2 recommended but not blocking)
- **User Story 5 (P5)**: Requires US3 and US4 to be useful (must have prompts/resources to serve)
- **User Story 6 (P6)**: Requires US5 (MCP server must be running), US3, and US4 (prompts/resources must exist)

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD requirement)
- Models before services
- Services before CLI commands
- CLI commands before MCP handlers
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- **Phase 1**: All setup tasks can run in parallel
- **Phase 2**: T006-T009 (validation + models) can run in parallel; T010-T011 (storage tests + impl) can run in parallel
- **Phase 3-8**: All test tasks marked [P] within each story can run in parallel; all implementation tasks marked [P] within each story can run in parallel
- **Multiple Stories**: If team has capacity, US1, US2 can proceed in parallel after Foundational; US3, US4 can proceed in parallel; US6 can only start after US3, US4, US5

---

## Parallel Example: User Story 3 (Prompts)

```bash
# Launch all tests for User Story 3 together:
Task: "Functional test for prompt add command in mcpkg/tests/functional/test_cli_prompt.py"
Task: "Functional test for prompt list command in mcpkg/tests/functional/test_cli_prompt.py"
Task: "Functional test for prompt update command in mcpkg/tests/functional/test_cli_prompt.py"
Task: "Functional test for prompt delete command in mcpkg/tests/functional/test_cli_prompt.py"
Task: "Functional test for MCP prompts/list in mcpkg/tests/functional/test_mcp_server.py"
Task: "Functional test for MCP prompts/get in mcpkg/tests/functional/test_mcp_server.py"
```

---

## Implementation Strategy

### MVP First (User Stories 1-3 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Workspace management)
4. Complete Phase 4: User Story 2 (Project management)
5. Complete Phase 5: User Story 3 (Prompts via CLI and MCP)
6. **STOP and VALIDATE**: Test all three stories work together
7. Deploy/demo if ready

This MVP delivers:
- ✅ Zero-config default workspace
- ✅ Project organization
- ✅ Prompt management via CLI
- ✅ Prompt access via MCP for AI agents
- ✅ Core value proposition validated

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Users can manage workspaces
3. Add User Story 2 → Test independently → Users can manage projects
4. Add User Story 3 → Test independently → Users can manage and access prompts (MVP!)
5. Add User Story 4 → Test independently → Users can manage and access resources
6. Add User Story 5 → Test independently → Users can scope MCP server to specific projects
7. Add User Story 6 → Test independently → AI agents can manage content dynamically
8. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 + User Story 2 (workspace and project management)
   - Developer B: User Story 3 (prompts)
   - Developer C: User Story 4 (resources)
3. After US3 + US4 complete:
   - Developer A: User Story 5 (MCP server start)
   - Developer B: User Story 6 (MCP tools)
4. Stories complete and integrate independently

---

## Summary

**Total Tasks**: 96
**Task Count per User Story**:
- Setup: 5 tasks
- Foundational: 13 tasks
- US1 (Workspace management): 9 tasks
- US2 (Project management): 9 tasks
- US3 (Prompts CLI + MCP read): 13 tasks
- US4 (Resources CLI + MCP read): 13 tasks
- US5 (MCP server start): 10 tasks
- US6 (MCP tools for write access): 15 tasks
- Polish: 9 tasks

**Parallel Opportunities**: 42 tasks marked [P] can run in parallel (44% of tasks)

**Independent Test Criteria**:
- US1: Can list/create/delete workspaces; default workspace auto-created
- US2: Can create/list/rename/delete projects independently
- US3: Can add prompts via CLI, retrieve via MCP protocol
- US4: Can add resources via CLI, retrieve via MCP protocol
- US5: Can start MCP server with specific project, only that project's content visible
- US6: Can use MCP tools to create/update/delete prompts and resources

**Suggested MVP Scope**: User Stories 1-3 (Workspaces + Projects + Prompts) = 40 tasks total

**TDD Enforcement**: All test tasks MUST be completed and failing before implementation tasks

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD cycle)
- Run CI checks after each phase (format, lint, typecheck, tests)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
