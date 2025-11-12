# Data Model: User Prompts via MCP

**Phase**: 1 (Design)  
**Date**: 2025-11-12  
**Status**: Defining entities and relationships

---

## Entities

### 1. PromptLibrary

Represents a collection of related prompts organized in a single directory.

**Fields**:
- `name: String` - Normalized library identifier (lowercase, underscores)
  - Derived from directory name
  - Normalization: convert to lowercase, replace spaces/special chars with underscores, trim underscores
  - Example: "My-Coding Lib" → "my_coding_lib"
- `path: PathBuf` - File system path to library directory
  - Must contain `twig.toml` file
  - Must contain `prompts/` subdirectory
- `config: LibraryConfig` - Parsed configuration from twig.toml

**Relationships**:
- Contains many `Prompt` items (1:N relationship)

**Validation Rules**:
- Library name must be non-empty and follow naming conventions
- Path must exist and be readable
- twig.toml must be valid TOML and contain minimal required schema
- prompts/ subdirectory must exist

---

### 2. LibraryConfig

Configuration metadata for a prompt library, loaded from twig.toml.

**Fields**:
- `name: String` - Human-readable library name (e.g., "My Coding Library")
- `description: Option<String>` - Brief description of the library
- `version: Option<String>` - Semantic version (e.g., "1.0.0")
- `prompts: Vec<PromptDefinition>` - List of prompt declarations

**TOML Schema Example**:
```toml
[prompts.code_review]
description = "Review code for quality and best practices"

[[prompts.code_review.arguments]]
name = "code_snippet"
description = "The code to review"
required = true

[[prompts.code_review.arguments]]
name = "language"
description = "Programming language"
# required omitted → defaults to false (optional)

[prompts.documentation]
description = "Generate documentation from code"

[[prompts.documentation.arguments]]
name = "function_code"
required = true
# description omitted → treated as None
```

**Validation Rules**:
- `prompts` table must contain at least one `[prompts.prompt_name]` entry
- Each prompt key must be unique within the library (enforced by TOML structure)
- Arguments are defined as `[[prompts.prompt_name.arguments]]` array-of-tables
- Each argument has required `name` field and optional `description` and `required` fields (defaults to `required = false`)
- Argument names must follow naming conventions (lowercase, underscores)
- Library metadata (name, description, version) is stored separately in the library's parent context, not in twig.toml

---

### 3. PromptDefinition

Metadata declaring a single prompt within a library. Defined as `[prompts.prompt_name]` in twig.toml.

**Fields** (from TOML):
- `prompt_name` (TOML key) - Prompt identifier (lowercase, underscores, no spaces)
- `description: String` - Human-readable description of the prompt
- `arguments: Option<HashMap<String, PromptArgument>>` - Optional nested argument definitions (see PromptArgument entity below)

**MCP Integration**:
Directly maps to MCP `Prompt` model:
```
Prompt {
    name: "library_name:prompt_name",
    description: Some("description from twig.toml"),
    input_schema: {
        type: "object",
        properties: {
            "arg_name": { type: "string", description: "..." }
        },
        required: ["arg_name1", ...]
    }
}
```

**Validation Rules**:
- Prompt name must be non-empty, lowercase, alphanumeric + underscores
- Arguments table is optional (prompts can have zero arguments)
- Argument names must follow naming conventions (lowercase, underscores)
- Argument descriptions are optional
- Argument `required` field is optional (defaults to `false`)

---

### 4. PromptArgument

Metadata for a single argument within a prompt. Defined as an entry in the `[[prompts.prompt_name.arguments]]` array-of-tables in twig.toml.

**Fields** (from TOML):
- `name: String` - Argument identifier (lowercase, underscores, no spaces)
- `description: Option<String>` - Human-readable description of what this argument represents
- `required: Option<bool>` - Whether this argument must be provided (defaults to `false` if omitted)

**Validation Rules**:
- Argument name must be non-empty, lowercase, alphanumeric + underscores
- `required` field is optional; when omitted, defaults to `false`
- `description` field is optional; when omitted, treated as `None`

---

### 5. Prompt (Runtime Instance)

A specific prompt instance with resolved content and metadata.

**Fields**:
- `library_name: String` - Name of containing library
- `prompt_name: String` - Name from definition
- `full_name: String` - Combined "library_name:prompt_name"
- `description: Option<String>` - From definition
- `content_path: PathBuf` - Path to markdown file
- `arguments: Option<HashMap<String, PromptArgument>>` - Argument definitions with metadata

**Relationships**:
- Belongs to one `PromptLibrary` (N:1 relationship)

**Validation Rules**:
- Markdown file must exist at `library_path/prompts/{prompt_name}.md`
- File must be readable and contain valid Markdown/Jinja2
- Full name uniqueness not required (can have same name in different libraries)

---

### 6. PromptContent

The rendered result of executing a prompt with arguments.

**Fields**:
- `full_name: String` - "library_name:prompt_name"
- `messages: Vec<PromptMessage>` - MCP-format message list
  - Currently returns as single User message with rendered content
  - Structure allows future extension for multi-turn prompts

**State**:
- Generated from Prompt + arguments
- Ephemeral (not persisted)
- Rendered from markdown + Jinja2 template

**Validation Rules**:
- All required arguments must be provided
- Optional arguments can be omitted (handled by Chainable template mode)
- Template rendering must succeed without errors

---

## File Structure

```
~/.local/share/twig/prompts/              # Platform-specific (dirs crate)
├── my_coding_lib/                        # Library directory (normalized name)
│   ├── twig.toml                         # Library configuration
│   └── prompts/                          # Prompt content directory
│       ├── code_review.md                # Jinja2 template markdown
│       ├── documentation.md
│       └── performance_audit.md
│
└── data_science_lib/
    ├── twig.toml
    └── prompts/
        ├── analyze_dataset.md
        └── model_evaluation.md
```

---

## Validation Rules & Constraints

### Library-Level
- **Discovery**: All subdirectories with valid twig.toml are valid libraries
- **Error handling**: Invalid libraries are logged but don't prevent loading others
- **Naming**: Conflict resolution for duplicate normalized names (log warning, use first found)
- **Permissions**: Graceful handling of unreadable directories

### Prompt-Level
- **File existence**: Missing markdown file → error on get_prompt, skipped in list_prompts
- **Template syntax**: Invalid Jinja2 → detailed error with line number
- **Argument validation**: Missing required args → JSON-RPC error (-32602 Invalid params)
- **Uniqueness**: Same prompt name allowed in different libraries (library prefix ensures uniqueness)

### Content-Level
- **Template rendering**: Chainable mode for optional args (treat undefined as empty)
- **Output**: Single User message in MCP format
- **Performance**: Must render within 500ms (per SC-002)

---

## State Transitions

```
PromptLibrary Discovery
  └─ Scan data directory for subdirectories
     ├─ Validate twig.toml exists and is valid
     │  └─ Parse PromptDefinitions
     │     └─ SUCCESS: PromptLibrary object created
     │  └─ ERROR: Log error, skip library
     └─ NOTFOUND: Skip directory

Prompt Retrieval
  └─ User requests get_prompt(library_name:prompt_name, args)
     ├─ Find library in loaded libraries
     │  └─ NOTFOUND: Return error (-32601 Method not found)
     ├─ Find prompt definition in library
     │  └─ NOTFOUND: Return error (-32601 Method not found)
     ├─ Locate markdown file
     │  └─ NOTFOUND: Return error with file path
     ├─ Validate required arguments provided
     │  └─ MISSING: Return error (-32602 Invalid params)
     ├─ Render template with arguments
     │  └─ ERROR: Return error with template details
     └─ SUCCESS: Return PromptContent as User message
```

---

## MCP Protocol Mapping

### ListPromptsResult
```rust
ListPromptsResult {
    prompts: vec![
        Prompt {
            name: "library_name:prompt_name",
            description: Some("..."),
            input_schema: {
                type: "object",
                properties: {
                    "arg1": { type: "string" },
                    "arg2": { type: "string" }
                },
                required: ["arg1"]
            }
        },
        ...
    ]
}
```

### GetPromptResult
```rust
GetPromptResult {
    description: Some("..."),
    messages: vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            "Rendered prompt content with {{ arg1 }} substituted"
        )
    ]
}
```

---

## Error Scenarios & Handling

| Scenario | Error Code | Message |
|----------|-----------|---------|
| Library directory doesn't exist | -32602 | "Prompt library not found: {library_name}" |
| twig.toml malformed | (logged) | "Failed to parse twig.toml in {library_path}: {details}" |
| Prompt not found in library | -32601 | "Prompt not found: {full_name}" |
| Markdown file missing | -32602 | "Prompt content file not found: {path}" |
| Required arg missing | -32602 | "Missing required argument: {arg_name}" |
| Template render error | -32602 | "Error rendering prompt template: {details} at line {line}" |
| Data directory not found | (startup) | "Unable to determine data directory for this platform" |

---

**Status**: Data model defined. Ready for API contracts and quickstart.
