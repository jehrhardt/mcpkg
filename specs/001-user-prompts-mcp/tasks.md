---

description: "Task list for User Prompts via MCP feature implementation"
---

# Tasks: User Prompts via MCP

**Input**: Design documents from `/specs/001-user-prompts-mcp/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are included as this is a Rust project with test-first development requirement per plan.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- This is a Rust single-project structure per plan.md

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create project structure per implementation plan in src/
- [X] T002 Add Rust dependencies to Cargo.toml: rmcp, minijinja, dirs, toml, tokio
- [ ] T003 [P] Configure cargo clippy and cargo fmt in CI workflow
- [X] T004 [P] Create base module files: src/library.rs, src/prompt.rs, src/data_dir.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Implement data directory resolution in src/data_dir.rs with TWIG_DATA_DIR support
- [X] T006 [P] Create unit tests for data_dir.rs (inline module)
- [X] T007 Implement TOML parsing for twig.toml in src/library.rs
- [X] T008 [P] Create unit tests for library.rs TOML parsing (inline module)
- [X] T009 Implement Jinja2 template rendering in src/prompt.rs with Chainable mode
- [X] T010 [P] Create unit tests for prompt.rs template rendering (inline module)
- [X] T011 Setup MCP server structure in src/mcp.rs with ServerHandler trait
- [X] T012 Configure error handling infrastructure with Result<T, ErrorData> types
- [X] T013 Setup integration test framework in tests/ with tokio::io::duplex

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Discover Available Prompts (Priority: P1) 🎯 MVP

**Goal**: Enable users to discover what prompts are available in their user-specific prompt libraries

**Independent Test**: Can list available prompts and verify that all user-installed library prompts are visible with proper library prefixes

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T014 [P] [US1] Contract test for prompts/list in tests/integration/test_list_prompts.rs
- [X] T015 [P] [US1] Unit test for library discovery in src/library.rs (test_library_scanning)

### Implementation for User Story 1

- [X] T016 [P] [US1] Implement library discovery scanning in src/library.rs
- [X] T017 [US1] Implement prompts/list MCP handler in src/mcp.rs
- [X] T018 [US1] Add library name normalization (lowercase, underscores) in src/library.rs
- [X] T019 [US1] Add error handling for malformed libraries (log and skip)
- [X] T020 [US1] Implement empty list handling for no libraries scenario

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Retrieve Prompt Content (Priority: P2)

**Goal**: Enable users to retrieve the content of a specific prompt with appropriate arguments

**Independent Test**: Can retrieve prompt content and verify that arguments are properly substituted and the markdown content is returned as expected

### Tests for User Story 2 ⚠️

- [X] T021 [P] [US2] Contract test for prompts/get in tests/integration/test_get_prompt.rs
- [X] T022 [P] [US2] Unit test for template rendering with arguments in src/prompt.rs

### Implementation for User Story 2

- [X] T023 [P] [US2] Implement prompt content loading from markdown files in src/prompt.rs
- [X] T024 [US2] Implement prompts/get MCP handler in src/mcp.rs
- [X] T025 [US2] Add argument validation (required vs optional) in src/prompt.rs
- [X] T026 [US2] Add template rendering with argument substitution in src/prompt.rs
- [X] T027 [US2] Implement error handling for missing prompt files
- [X] T028 [US2] Add support for optional arguments with Chainable mode

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Handle Library Configuration Errors (Priority: P3)

**Goal**: Provide clear error messages when prompt libraries have configuration issues

**Independent Test**: Can create libraries with various configuration problems and verify that appropriate error messages are generated without breaking the overall system

### Tests for User Story 3 ⚠️

- [X] T029 [P] [US3] Integration test for malformed twig.toml handling in tests/integration/test_error_handling.rs
- [X] T030 [P] [US3] Unit test for error message generation in src/library.rs

### Implementation for User Story 3

- [X] T031 [P] [US3] Enhance error messages for malformed twig.toml files in src/library.rs
- [X] T032 [US3] Add detailed error reporting for missing markdown files in src/prompt.rs
- [X] T033 [US3] Implement argument validation error messages in src/prompt.rs
- [X] T034 [US3] Add template rendering error details (line numbers) in src/prompt.rs
- [X] T035 [US3] Ensure error handling doesn't break library discovery

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T036 [P] Add comprehensive logging throughout all modules
- [ ] T037 [P] Performance optimization for large library scanning
- [ ] T038 [P] Add file system watching for real-time library changes
- [X] T039 [P] Additional unit tests for edge cases in all modules
- [ ] T040 Run quickstart.md validation with example libraries
- [X] T041 Code cleanup and documentation updates

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Depends on US1 for library discovery but can be tested independently
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Enhances error handling for US1 and US2

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Models before services
- Services before MCP handlers
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Contract test for prompts/list in tests/integration/test_list_prompts.rs"
Task: "Unit test for library discovery in src/library.rs (test_library_scanning)"

# Launch all implementation for User Story 1 together:
Task: "Implement library discovery scanning in src/library.rs"
Task: "Implement prompts/list MCP handler in src/mcp.rs"
Task: "Add library name normalization in src/library.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (discovery)
   - Developer B: User Story 2 (retrieval)
   - Developer C: User Story 3 (error handling)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence