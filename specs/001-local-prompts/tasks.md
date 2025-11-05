# Tasks: Local Prompts Support

**Feature Branch**: `001-local-prompts`  
**Input**: Design documents from `/specs/001-local-prompts/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Integration tests using rmcp client as specified in spec.md and quickstart.md. Unit tests for core parsing and rendering logic.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

**Total Tasks**: 80 (T001-T080)

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

Single project structure at repository root:
- `src/` - Source code
- `tests/` - Integration and unit tests
- `website/docs/` - User documentation

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency setup

- [X] T001 Add dependencies to Cargo.toml: gray_matter 0.3, minijinja 2.12.0, notify 8.2, notify-debouncer-full 0.6, thiserror 1.0
- [X] T002 [P] Create prompts module structure: src/prompts/mod.rs, src/prompts/types.rs, src/prompts/parser.rs, src/prompts/renderer.rs, src/prompts/registry.rs, src/prompts/watcher.rs
- [X] T003 [P] Create integration test structure: tests/integration/mod.rs, tests/integration/prompts_list.rs, tests/integration/prompts_get.rs, tests/integration/prompts_notify.rs
- [X] T004 Declare prompts module in src/main.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data structures and error handling that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 [P] Define PromptFile struct in src/prompts/types.rs (name, path, metadata, content, modified)
- [X] T006 [P] Define PromptMetadata struct in src/prompts/types.rs (title, description, arguments with serde defaults)
- [X] T007 [P] Define PromptArgument struct in src/prompts/types.rs (name, description, required)
- [X] T008 [P] Define PromptError enum in src/prompts/types.rs with thiserror (NotFound, InvalidFrontmatter, InvalidTemplate, MissingArgument, RenderError, IoError, WatcherError)
- [X] T009 Implement From<PromptError> for rmcp::ErrorData in src/prompts/types.rs (map NotFound and MissingArgument to invalid_params, others to internal_error)
- [X] T010 Implement From<PromptArgument> for rmcp::model::PromptArgument in src/prompts/types.rs
- [X] T011 Export public API in src/prompts/mod.rs (PromptRegistry and necessary types)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Expose Local Prompts to MCP Clients (Priority: P1) 🎯 MVP

**Goal**: Enable MCP clients to discover and retrieve parameterized prompts from `.twig/prompts/` directory

**Independent Test**: Create a prompt file in `.twig/prompts/`, start the MCP server, and verify that an MCP client can list and retrieve the prompt with parameter substitution

### Tests for User Story 1 (Write FIRST, ensure they FAIL)

- [X] T012 [P] [US1] Unit test for parse_prompt_file with valid YAML frontmatter in src/prompts/parser.rs tests module
- [X] T013 [P] [US1] Unit test for parse_prompt_file with missing required fields (title or description) in src/prompts/parser.rs tests module
- [X] T014 [P] [US1] Unit test for TemplateRenderer with simple variable substitution in src/prompts/renderer.rs tests module
- [X] T015 [P] [US1] Unit test for TemplateRenderer with missing variables in src/prompts/renderer.rs tests module
- [X] T016 [P] [US1] Integration test for prompts/list returning empty array when .twig/prompts/ doesn't exist in tests/integration/prompts_list.rs
- [X] T017 [P] [US1] Integration test for prompts/list returning multiple prompts with correct names and metadata in tests/integration/prompts_list.rs
- [X] T018 [P] [US1] Integration test for prompts/get with parameter substitution in tests/integration/prompts_get.rs
- [X] T019 [P] [US1] Integration test for prompts/get returning error for unknown prompt in tests/integration/prompts_get.rs
- [X] T020 [P] [US1] Integration test for prompts/get returning error for missing required argument in tests/integration/prompts_get.rs

### Implementation for User Story 1

- [X] T021 [P] [US1] Implement parse_prompt_file function in src/prompts/parser.rs (read file, extract name from filename, parse YAML frontmatter with gray_matter, validate required fields)
- [X] T022 [P] [US1] Implement TemplateRenderer struct with new() in src/prompts/renderer.rs (wrap minijinja::Environment)
- [X] T023 [US1] Implement TemplateRenderer::render method in src/prompts/renderer.rs (add template, get template, render with arguments, handle errors)
- [X] T024 [US1] Implement PromptRegistry struct with new() in src/prompts/registry.rs (Arc<RwLock<HashMap>>, directory PathBuf, TemplateRenderer)
- [X] T025 [US1] Implement PromptRegistry::load_all method in src/prompts/registry.rs (handle missing directory, scan for .md files, parse prompts, skip invalid files with logging)
- [X] T026 [US1] Implement PromptRegistry::get method in src/prompts/registry.rs (read lock, return cloned PromptFile)
- [X] T027 [US1] Implement PromptRegistry::list method in src/prompts/registry.rs (read lock, return all prompts)
- [X] T028 [US1] Implement PromptRegistry::render method in src/prompts/registry.rs (get prompt, validate required arguments, call renderer)
- [X] T029 [US1] Update src/mcp.rs to create PromptRegistry in Server::new (use .twig/prompts relative to current directory, call load_all)
- [X] T030 [US1] Update src/mcp.rs ServerHandler::get_info to declare prompts capability with listChanged: true
- [X] T031 [US1] Implement src/mcp.rs ServerHandler::list_prompts (call registry.list, convert to MCP Prompt structs)
- [X] T032 [US1] Implement src/mcp.rs ServerHandler::get_prompt (extract arguments, call registry.render, return GetPromptResult with User role message)

**Checkpoint**: User Story 1 complete - clients can list and retrieve prompts with parameter substitution

---

## Phase 4: User Story 2 - Handle Prompt Discovery and Updates (Priority: P2)

**Goal**: Automatically detect file system changes and notify MCP clients when prompts are added, modified, or removed

**Independent Test**: Add/remove/modify prompt files while server is running and verify clients receive list_changed notifications within 2 seconds

### Tests for User Story 2 (Write FIRST, ensure they FAIL)

- [ ] T033 [P] [US2] Unit test for file watcher initialization in src/prompts/watcher.rs tests module
- [ ] T034 [P] [US2] Integration test for list_changed notification on file creation in tests/integration/prompts_notify.rs
- [ ] T035 [P] [US2] Integration test for list_changed notification on file modification in tests/integration/prompts_notify.rs
- [ ] T036 [P] [US2] Integration test for list_changed notification on file deletion in tests/integration/prompts_notify.rs
- [ ] T037 [P] [US2] Integration test verifying prompts/list returns updated content after file modification in tests/integration/prompts_list.rs

### Implementation for User Story 2

- [ ] T038 [P] [US2] Implement watch_prompts_directory function in src/prompts/watcher.rs (create debouncer with 2-second timeout, use NonRecursive mode, return tokio channel receiver)
- [ ] T039 [US2] Add watcher_rx field to PromptRegistry struct in src/prompts/registry.rs (Option<mpsc::UnboundedReceiver<DebounceEventResult>>)
- [ ] T040 [US2] Implement PromptRegistry::start_watching method in src/prompts/registry.rs (call watch_prompts_directory, store receiver)
- [ ] T041 [US2] Implement PromptRegistry::handle_file_event private method in src/prompts/registry.rs (detect Create/Modify/Remove, reload affected prompts, return bool indicating if list changed)
- [ ] T042 [US2] Add peer field to Server struct in src/mcp.rs to store rmcp service peer for sending notifications
- [ ] T043 [US2] Update Server::new in src/mcp.rs to call registry.start_watching()
- [ ] T044 [US2] Create event processing task in src/mcp.rs to consume watcher_rx and call peer.notify_prompts_list_changed() when changes detected

**Checkpoint**: User Story 2 complete - file watching and notifications working, clients stay in sync with file system

---

## Phase 5: User Story 3 - Validate Prompt Files (Priority: P3)

**Goal**: Provide clear feedback for prompt file errors (invalid YAML, missing required fields, invalid Jinja syntax) to help users fix issues quickly

**Independent Test**: Create invalid prompt files and verify appropriate error handling (silent skipping with optional logging for load errors, JSON-RPC errors for render errors)

### Tests for User Story 3 (Write FIRST, ensure they FAIL)

- [ ] T045 [P] [US3] Unit test for parse_prompt_file with invalid YAML frontmatter returning InvalidFrontmatter error in src/prompts/parser.rs tests module
- [ ] T046 [P] [US3] Unit test for parse_prompt_file with missing title field (skipped as invalid) in src/prompts/parser.rs tests module
- [ ] T047 [P] [US3] Unit test for parse_prompt_file with missing description field (skipped as invalid) in src/prompts/parser.rs tests module
- [ ] T048 [P] [US3] Unit test for TemplateRenderer with invalid Jinja syntax returning InvalidTemplate error in src/prompts/renderer.rs tests module
- [ ] T049 [P] [US3] Unit test for PromptRegistry::render with undeclared parameter used in template in src/prompts/registry.rs tests module
- [ ] T050 [P] [US3] Integration test for prompts/list skipping files with invalid YAML in tests/integration/prompts_list.rs
- [ ] T051 [P] [US3] Integration test for prompts/list skipping files missing required fields in tests/integration/prompts_list.rs
- [ ] T052 [P] [US3] Integration test for prompts/get returning error for invalid Jinja template in tests/integration/prompts_get.rs

### Implementation for User Story 3

- [ ] T053 [US3] Update parse_prompt_file in src/prompts/parser.rs to validate title and description are present, return error if missing
- [ ] T054 [US3] Update PromptRegistry::load_all in src/prompts/registry.rs to log warnings with eprintln! for skipped files (invalid YAML or missing fields)
- [ ] T055 [US3] Update TemplateRenderer::render in src/prompts/renderer.rs to wrap add_template errors as InvalidTemplate with file context
- [ ] T056 [US3] Add validation in PromptRegistry::render in src/prompts/registry.rs to check for required arguments before rendering
- [ ] T057 [US3] Update error conversion in src/prompts/types.rs to ensure InvalidTemplate and RenderError map to appropriate JSON-RPC error codes

**Checkpoint**: User Story 3 complete - validation provides clear feedback, invalid files are handled gracefully

---

## Phase 6: User Story 4 - Document Local Prompts Feature (Priority: P2)

**Goal**: Provide comprehensive documentation for users to understand how to create, organize, and use local prompts effectively

**Independent Test**: Review documentation for completeness, accuracy, and clarity. Follow instructions to create a test prompt end-to-end

### Implementation for User Story 4

- [ ] T058 [P] [US4] Create website/docs/local-prompts.md with feature overview section (what are local prompts, why use them)
- [ ] T059 [P] [US4] Add quick start section to website/docs/local-prompts.md (create .twig/prompts directory, create first prompt file)
- [ ] T060 [P] [US4] Document YAML frontmatter structure in website/docs/local-prompts.md (required fields: title and description, optional: arguments array)
- [ ] T061 [P] [US4] Add complete prompt file examples to website/docs/local-prompts.md (simple prompt with no parameters, parameterized prompt with required/optional arguments, prompt with Jinja conditionals)
- [ ] T062 [P] [US4] Document Jinja template syntax in website/docs/local-prompts.md (variable substitution, filters, conditionals, loops)
- [ ] T063 [P] [US4] Document parameter usage in website/docs/local-prompts.md (declaring parameters, required vs optional, using in templates)
- [ ] T064 [P] [US4] Add MCP client integration section to website/docs/local-prompts.md (how clients discover prompts, calling prompts/list and prompts/get)
- [ ] T065 [P] [US4] Add troubleshooting section to website/docs/local-prompts.md (common errors: invalid YAML, missing fields, Jinja syntax errors, missing arguments, file not found)
- [ ] T066 [P] [US4] Add best practices section to website/docs/local-prompts.md (naming conventions, parameter naming, testing prompts, organizing prompts)
- [ ] T067 [US4] Update main documentation index to reference local-prompts.md

**Checkpoint**: User Story 4 complete - users have complete documentation to create and use local prompts

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements affecting multiple user stories

- [ ] T068 [P] Add optional debug logging throughout src/prompts/ modules (prompt loading, file watching events, template rendering)
- [ ] T069 [P] Performance test with 100+ prompt files to verify SC-005 (no degradation)
- [ ] T070 [P] Verify SC-001: prompt discovery completes within 100ms of server startup
- [ ] T071 [P] Verify SC-002: template rendering completes within 50ms
- [ ] T072 [P] Verify SC-003: file changes detected and notified within 2 seconds
- [ ] T079 [P] Verify SC-004: Time end-to-end prompt creation following quickstart.md with stopwatch (target: user completes first parameterized prompt in <5 minutes from reading docs to successful prompts/get call)
- [ ] T080 [P] Verify SC-007: Conduct usability test with 10 representative users (ideally unfamiliar with Twig), measure success rate creating first prompt using only documentation (target: ≥9/10 succeed without support)
- [ ] T073 Run cargo fmt to format all code
- [ ] T074 Run cargo clippy -- -D warnings to verify no linting errors
- [ ] T075 Run cargo test to verify all tests pass
- [ ] T076 Run cargo build --release to verify production build succeeds
- [ ] T077 Manual end-to-end test following quickstart.md instructions
- [ ] T078 Update AGENTS.md to mention local prompts feature and new prompts module

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User Story 1 (P1): Can start after Phase 2 - No dependencies on other stories
  - User Story 2 (P2): **Depends on User Story 1** (needs PromptRegistry implementation) - extends watching
  - User Story 3 (P3): Can start after Phase 2 - enhances validation from US1
  - User Story 4 (P2): Can start after User Story 1 is functional - documents implemented features
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Foundation only - No dependencies on other stories ✅ **TRUE MVP**
- **User Story 2 (P2)**: Requires User Story 1 complete (extends PromptRegistry with file watching)
- **User Story 3 (P3)**: Independent of US2, enhances US1 validation (can run parallel with US2 if desired)
- **User Story 4 (P2)**: Requires User Story 1 complete to document; ideally after US2 and US3 for complete coverage

### Within Each User Story

- Tests MUST be written FIRST and FAIL before implementation
- Unit tests (parser, renderer) before integration tests
- Core implementation (types, parser, renderer, registry) before MCP integration
- MCP handlers after registry implementation
- Story validation before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T002, T003)
- All Foundational tasks marked [P] can run in parallel (T005-T010)
- **User Story 1**: All test tasks (T012-T020) can run in parallel, all implementation tasks marked [P] (T021, T022) can run in parallel
- **User Story 2**: All test tasks (T033-T037) can run in parallel, T038 can run parallel with T039-T041 (different files)
- **User Story 3**: All test tasks (T045-T052) can run in parallel
- **User Story 4**: All documentation tasks (T058-T066) can run in parallel
- **Polish**: Most tasks (T068-T072, T073-T076) can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all unit tests for User Story 1 together:
Task: "Unit test for parse_prompt_file with valid YAML frontmatter in src/prompts/parser.rs tests module"
Task: "Unit test for parse_prompt_file with missing required fields in src/prompts/parser.rs tests module"
Task: "Unit test for TemplateRenderer with simple variable substitution in src/prompts/renderer.rs tests module"
Task: "Unit test for TemplateRenderer with missing variables in src/prompts/renderer.rs tests module"

# Launch all integration tests for User Story 1 together:
Task: "Integration test for prompts/list returning empty array when .twig/prompts/ doesn't exist"
Task: "Integration test for prompts/list returning multiple prompts with correct names and metadata"
Task: "Integration test for prompts/get with parameter substitution"
Task: "Integration test for prompts/get returning error for unknown prompt"
Task: "Integration test for prompts/get returning error for missing required argument"

# Launch parser and renderer implementation in parallel (different files):
Task: "Implement parse_prompt_file function in src/prompts/parser.rs"
Task: "Implement TemplateRenderer struct with new() and render in src/prompts/renderer.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only) - Recommended

1. Complete Phase 1: Setup (T001-T004)
2. Complete Phase 2: Foundational (T005-T011) - CRITICAL
3. Complete Phase 3: User Story 1 (T012-T032)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Demo/validate: Can list prompts, get prompts with parameters, parameter substitution works
6. This is a functional MVP - prompts work but don't auto-update

### Incremental Delivery (Recommended Order)

1. **Phase 1 + 2** → Foundation ready
2. **+ User Story 1 (P1)** → Test independently → **MVP READY** (core functionality works)
3. **+ User Story 2 (P2)** → Test independently → Auto-update feature working
4. **+ User Story 3 (P3)** → Test independently → Better validation and error messages
5. **+ User Story 4 (P2)** → Documentation complete → Feature fully documented
6. **+ Phase 7** → Polish → Production ready

Each increment adds value without breaking previous functionality.

### Parallel Team Strategy

With 2+ developers after Foundational phase:

1. **Team completes Setup + Foundational together** (T001-T011)
2. **Developer A**: User Story 1 (T012-T032) - Core functionality
3. Once US1 is complete:
   - **Developer A**: User Story 2 (T033-T044) - File watching (depends on US1)
   - **Developer B**: User Story 4 (T058-T067) - Documentation (can proceed after US1 functional)
4. **Developer C** (if available): User Story 3 (T045-T057) - Enhanced validation (parallel with US2)
5. **Team**: Phase 7 Polish (T068-T078)

---

## Notes

- **Unit tests in same file as code**: All parser, renderer, registry unit tests go in `#[cfg(test)] mod tests` submodules within their respective implementation files (src/prompts/parser.rs, src/prompts/renderer.rs, src/prompts/registry.rs)
- **Integration tests in tests/ directory**: All integration tests using rmcp client go in tests/integration/ directory
- **[P] tasks** = different files, no dependencies within the phase
- **[Story] label** maps task to specific user story for traceability (US1, US2, US3, US4)
- Each user story should be independently testable after completion
- User Story 1 is the true MVP - delivers core value
- User Story 2 adds auto-update capability (nice-to-have but not essential)
- User Story 3 improves error handling (polish)
- User Story 4 enables discoverability (essential for adoption but can be done after implementation)
- Verify tests fail before implementing features
- Commit after each task or logical group of tasks
- Stop at any checkpoint to validate story independently

---

## Requirements Coverage Matrix

This matrix maps functional requirements (FR) and success criteria (SC) from spec.md to implementing tasks.

### Functional Requirements Coverage

| Requirement | Description | Implementing Tasks |
|-------------|-------------|--------------------|
| FR-001 | Scan top-level .md files only | T021 (parse), T025 (load_all) |
| FR-002 | Parse YAML frontmatter, skip invalid | T021 (parse), T053 (validate), T054 (log skip) |
| FR-003 | Filename → prompt name | T021 (parse logic) |
| FR-004 | Expose prompts/list endpoint | T031 (list_prompts handler) |
| FR-005 | Support parameters in YAML | T021 (parse arguments field) |
| FR-006 | Render with Jinja, lazy validation | T022 (TemplateRenderer), T023 (render method), T028 (registry render) |
| FR-007 | Return rendered via prompts/get | T032 (get_prompt handler) |
| FR-008 | Declare prompts capability | T030 (get_info with capability) |
| FR-009 | Send list_changed notifications | T038-T044 (file watching + notification) |
| FR-010 | Validate required arguments | T028 (validate in render), T056 (add validation) |
| FR-011 | Return JSON-RPC errors | T009 (ErrorData conversion), T057 (update error mapping) |
| FR-012 | Support MCP metadata fields | T031 (convert to MCP Prompt structs) |
| FR-013 | Return messages with role/content | T032 (GetPromptResult with User role) |
| FR-014 | Add documentation to website/docs/ | T058 (create local-prompts.md) |
| FR-015 | Include YAML examples | T061 (complete examples) |
| FR-016 | Explain YAML structure + required fields | T060 (document frontmatter structure) |
| FR-017 | Demonstrate Jinja + parameters | T062 (Jinja syntax), T063 (parameter usage) |
| FR-018 | Describe MCP client discovery | T064 (MCP integration section) |
| FR-019 | Include troubleshooting | T065 (troubleshooting section) |

### Success Criteria Coverage

| Criterion | Description | Validating Tasks |
|-----------|-------------|------------------|
| SC-001 | Discover prompts <100ms | T070 (verify startup time) |
| SC-002 | Render <50ms | T071 (verify render time) |
| SC-003 | Notify <2s on change | T072 (verify notification timing) |
| SC-004 | User creates prompt <5min | T079 (time end-to-end creation) |
| SC-005 | Handle 100+ files | T069 (performance test) |
| SC-006 | Skip invalid files gracefully | T050, T051 (integration tests), T054 (logging) |
| SC-007 | 90% user success with docs | T080 (usability test) |

**Note**: Tasks T079 and T080 are additions to validate user-facing success criteria that require human testing.
