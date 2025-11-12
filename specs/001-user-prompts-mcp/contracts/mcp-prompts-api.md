# MCP Prompts API Contract

**Protocol**: Model Context Protocol (MCP) v1.0  
**Transport**: stdio (local machine communication)  
**Status**: Specification

---

## Overview

The Twig MCP server implements the standard MCP `prompts` capability, allowing clients to discover and retrieve prompts from user-installed libraries.

**Capabilities Declared**:
```json
{
  "prompts": {
    "listChanged": true
  }
}
```

The `listChanged` indicator means the server supports real-time notifications when the prompt list changes (via file system watching).

---

## Methods

### 1. prompts/list

Enumerate all available prompts across all loaded libraries.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "prompts/list",
  "params": {
    "cursor": null
  }
}
```

**Response (Success)**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "prompts": [
      {
        "name": "my_coding_lib:code_review",
        "description": "Review code for quality and best practices",
        "inputSchema": {
          "type": "object",
          "properties": {
            "code_snippet": {
              "type": "string",
              "description": "The code to review"
            },
            "language": {
              "type": "string",
              "description": "Programming language (optional)"
            }
          },
          "required": ["code_snippet"]
        }
      },
      {
        "name": "my_coding_lib:documentation",
        "description": "Generate documentation from code",
        "inputSchema": {
          "type": "object",
          "properties": {
            "function_code": {
              "type": "string",
              "description": "Function source code"
            }
          },
          "required": ["function_code"]
        }
      }
    ],
    "nextCursor": null
  }
}
```

**Error Responses**:

No prompts installed:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "prompts": [],
    "nextCursor": null
  }
}
```

Data directory not accessible (logged, but list still returns empty):
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "prompts": [],
    "nextCursor": null
  }
}
```

**Behavior**:
- Returns all prompts from all successfully loaded libraries
- Library names are normalized (lowercase, underscores)
- Prompt names include library prefix: `library_name:prompt_name`
- Invalid libraries are skipped with error logging (don't block entire list)
- Response time target: < 2 seconds (SC-001)

---

### 2. prompts/get

Retrieve the content of a specific prompt with argument substitution.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "prompts/get",
  "params": {
    "name": "my_coding_lib:code_review",
    "arguments": {
      "code_snippet": "def hello():\n    return 'world'",
      "language": "python"
    }
  }
}
```

**Response (Success)**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "description": null,
    "messages": [
      {
        "role": "user",
        "content": {
          "type": "text",
          "text": "Please review the following Python code for quality and best practices:\n\n```python\ndef hello():\n    return 'world'\n```\n\nFocus on:\n- Code clarity and readability\n- Performance considerations\n- Adherence to Python best practices"
        }
      }
    ]
  }
}
```

**Error Responses**:

Prompt not found:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "error": {
    "code": -32601,
    "message": "Method not found",
    "data": {
      "details": "Prompt not found: my_coding_lib:code_review"
    }
  }
}
```

Missing required argument:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "error": {
    "code": -32602,
    "message": "Invalid params",
    "data": {
      "details": "Missing required argument: code_snippet"
    }
  }
}
```

Content file missing:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "error": {
    "code": -32602,
    "message": "Invalid params",
    "data": {
      "details": "Prompt content file not found: ~/.local/share/twig/prompts/my_coding_lib/prompts/code_review.md"
    }
  }
}
```

Template rendering error:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "error": {
    "code": -32602,
    "message": "Invalid params",
    "data": {
      "details": "Error rendering prompt template at line 5: undefined variable 'required_arg'"
    }
  }
}
```

**Behavior**:
- Returns prompt content as a single User message
- Argument values are substituted using Jinja2 template syntax
- Optional arguments not provided are handled gracefully (Chainable mode)
- Response time target: < 500ms (SC-002)
- Markdown content is returned as-is (not HTML-converted)
- Line breaks and formatting preserved

---

## Data Types

### Prompt (List Item)

```typescript
interface Prompt {
  name: string;                    // "library_name:prompt_name"
  description?: string;             // From twig.toml
  inputSchema?: ToolInputSchema;    // JSON Schema for arguments
}
```

### ToolInputSchema

```typescript
interface ToolInputSchema {
  type: "object";
  properties: {
    [argName: string]: {
      type: "string";               // All arguments are strings
      description?: string;
    }
  };
  required?: string[];              // Required argument names
}
```

### PromptMessage

```typescript
interface PromptMessage {
  role: "user";                     // Always "user" for now
  content: {
    type: "text";
    text: string;                   // Rendered prompt with substitutions
  }
}
```

### GetPromptResult

```typescript
interface GetPromptResult {
  description?: string;
  messages: PromptMessage[];        // Single user message with content
}
```

---

## Error Codes

| Code | Meaning | When |
|------|---------|------|
| -32600 | Invalid Request | Malformed JSON-RPC request |
| -32601 | Method Not Found | Prompt doesn't exist |
| -32602 | Invalid Params | Missing required args, missing file, render error |
| -32603 | Internal Error | Unexpected server error (logged) |
| -32000 to -32099 | Server Error | Reserved for future use |

---

## Argument Handling

### String Arguments
All prompt arguments are strings (simplicity and universal compatibility).

**Example twig.toml**:
```toml
[prompts.code_review]
description = "Review code"

[[prompts.code_review.arguments]]
name = "code_snippet"
description = "Code to review"
required = true

[[prompts.code_review.arguments]]
name = "language"
description = "Programming language"

[[prompts.code_review.arguments]]
name = "max_issues"
description = "Maximum number of issues to report"
```

**Request with arguments**:
```json
{
  "arguments": {
    "code_snippet": "print('hello')",
    "language": "python",
    "max_issues": "5"
  }
}
```

### Template Substitution
Markdown files use Jinja2 syntax:

**File: my_coding_lib/prompts/code_review.md**:
```markdown
# Code Review

Please review the following {{ language }} code:

{{ code_snippet }}

Report up to {{ max_issues }} issues.
```

**Rendered output**:
```markdown
# Code Review

Please review the following python code:

print('hello')

Report up to 5 issues.
```

### Optional Arguments
Arguments not provided are handled by Jinja2 Chainable mode:

```markdown
{% if language %}Language: {{ language }}{% endif %}
```

If `language` is not provided, the entire block is omitted.

---

## Performance Requirements

| Operation | Target | Rationale |
|-----------|--------|-----------|
| prompts/list | < 2 seconds | Include discovery + parsing overhead (SC-001) |
| prompts/get | < 500ms | Single file read + template render (SC-002) |
| Template render (10 args) | < 1ms | minijinja performance baseline |
| Library scan | O(n) where n = # libraries | One scan at startup; watching detects changes |

---

## Future Extensions (Out of Scope)

- **Multi-turn prompts**: Currently single User message; future could support conversation templates
- **Tool invocation**: Currently read-only prompts; future could embed tool calls
- **Argument validation**: Currently strings only; future could support typed arguments
- **Inheritance/composition**: Currently flat library structure; future could support template sharing
- **Registry integration**: Currently file-based; future could fetch from Git/HTTP

---

**Status**: API contract complete. Implements standard MCP prompts capability with file-based libraries and Jinja2 templating.
