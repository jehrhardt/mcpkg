# MCP API Contract

**Feature**: 001-mcpkg-is-a  
**Date**: 2025-10-14  
**Phase**: 1 (Design & Contracts)

## Overview

This document specifies the Model Context Protocol (MCP) API for mcpkg. The server exposes prompts, resources, and tools for AI coding agents. Communication uses stdio transport (JSON-RPC over stdin/stdout).

## Server Information

### `initialize` Request

Standard MCP initialization handshake.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "prompts": {},
      "resources": {},
      "tools": {}
    },
    "clientInfo": {
      "name": "claude-desktop",
      "version": "1.0.0"
    }
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "prompts": {
        "listChanged": false
      },
      "resources": {
        "listChanged": false
      },
      "tools": {
        "listChanged": false
      }
    },
    "serverInfo": {
      "name": "mcpkg",
      "version": "0.1.0"
    }
  }
}
```

---

## Prompts API

### `prompts/list` Request

List all prompts in the selected project.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "prompts/list",
  "params": {}
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "prompts": [
      {
        "name": "code-review",
        "description": "Review authentication code for security issues"
      },
      {
        "name": "refactor-guide",
        "description": "Guide for refactoring auth module"
      }
    ]
  }
}
```

**Fields**:
- `prompts`: Array of prompt objects
  - `name`: Prompt identifier (string, required)
  - `description`: Prompt description (string, optional)

---

### `prompts/get` Request

Get a specific prompt's content.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "prompts/get",
  "params": {
    "name": "code-review"
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "description": "Review authentication code for security issues",
    "messages": [
      {
        "role": "user",
        "content": {
          "type": "text",
          "text": "Please review the authentication code in this project for security issues. Focus on:\n- Input validation\n- SQL injection risks\n- Authentication bypass vulnerabilities\n- Password storage"
        }
      }
    ]
  }
}
```

**Error Response** (prompt not found):
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "error": {
    "code": -32602,
    "message": "Prompt 'code-review' not found in project 'auth-module'"
  }
}
```

**Fields**:
- `description`: Prompt description (string, optional)
- `messages`: Array of message objects (required)
  - `role`: "user" (string, required)
  - `content`: Content object (required)
    - `type`: "text" (string, required)
    - `text`: Prompt content (string, required)

---

## Resources API

### `resources/list` Request

List all resources in the selected project.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "resources/list",
  "params": {}
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "resources": [
      {
        "uri": "file:///docs/api.md",
        "name": "API Documentation",
        "description": "REST API documentation",
        "mimeType": "text/markdown"
      },
      {
        "uri": "file:///schema.sql",
        "name": "Database Schema",
        "description": "SQLite database schema",
        "mimeType": "application/sql"
      }
    ]
  }
}
```

**Fields**:
- `resources`: Array of resource objects
  - `uri`: Resource URI (string, required)
  - `name`: Resource display name (string, required)
  - `description`: Resource description (string, optional)
  - `mimeType`: MIME type (string, optional)

---

### `resources/read` Request

Read a specific resource's content.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "resources/read",
  "params": {
    "uri": "file:///docs/api.md"
  }
}
```

**Response** (text content):
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "result": {
    "contents": [
      {
        "uri": "file:///docs/api.md",
        "mimeType": "text/markdown",
        "text": "# API Documentation\n\n## Authentication\n\n..."
      }
    ]
  }
}
```

**Response** (binary content):
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "result": {
    "contents": [
      {
        "uri": "file:///image.png",
        "mimeType": "image/png",
        "blob": "iVBORw0KGgoAAAANSUhEUgAA..."
      }
    ]
  }
}
```

**Error Response** (resource not found):
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "error": {
    "code": -32602,
    "message": "Resource 'file:///docs/api.md' not found in project 'auth-module'"
  }
}
```

**Fields**:
- `contents`: Array of content objects
  - `uri`: Resource URI (string, required)
  - `mimeType`: MIME type (string, optional)
  - `text`: Text content (string, for text resources)
  - `blob`: Base64-encoded binary content (string, for binary resources)

---

## Tools API

### `tools/list` Request

List all available tools.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "method": "tools/list",
  "params": {}
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "result": {
    "tools": [
      {
        "name": "create_prompt",
        "description": "Create a new prompt in the selected project",
        "inputSchema": {
          "type": "object",
          "properties": {
            "name": {
              "type": "string",
              "description": "Prompt name (alphanumeric, hyphens, underscores, dots)"
            },
            "content": {
              "type": "string",
              "description": "Prompt text content"
            },
            "description": {
              "type": "string",
              "description": "Optional description"
            }
          },
          "required": ["name", "content"]
        }
      },
      {
        "name": "update_prompt",
        "description": "Update an existing prompt's content or description",
        "inputSchema": {
          "type": "object",
          "properties": {
            "name": {
              "type": "string",
              "description": "Prompt name"
            },
            "content": {
              "type": "string",
              "description": "New prompt content (optional)"
            },
            "description": {
              "type": "string",
              "description": "New description (optional)"
            }
          },
          "required": ["name"]
        }
      },
      {
        "name": "delete_prompt",
        "description": "Delete a prompt from the selected project",
        "inputSchema": {
          "type": "object",
          "properties": {
            "name": {
              "type": "string",
              "description": "Prompt name"
            }
          },
          "required": ["name"]
        }
      },
      {
        "name": "create_resource",
        "description": "Create a new resource in the selected project",
        "inputSchema": {
          "type": "object",
          "properties": {
            "name": {
              "type": "string",
              "description": "Resource name (alphanumeric, hyphens, underscores, dots)"
            },
            "uri": {
              "type": "string",
              "description": "Resource URI"
            },
            "content": {
              "type": "string",
              "description": "Resource content (text or base64 for binary)"
            },
            "mime_type": {
              "type": "string",
              "description": "MIME type (optional, defaults to text/plain)"
            },
            "description": {
              "type": "string",
              "description": "Optional description"
            }
          },
          "required": ["name", "uri", "content"]
        }
      },
      {
        "name": "update_resource",
        "description": "Update an existing resource's content or metadata",
        "inputSchema": {
          "type": "object",
          "properties": {
            "name": {
              "type": "string",
              "description": "Resource name"
            },
            "content": {
              "type": "string",
              "description": "New resource content (optional)"
            },
            "mime_type": {
              "type": "string",
              "description": "New MIME type (optional)"
            },
            "description": {
              "type": "string",
              "description": "New description (optional)"
            }
          },
          "required": ["name"]
        }
      },
      {
        "name": "delete_resource",
        "description": "Delete a resource from the selected project",
        "inputSchema": {
          "type": "object",
          "properties": {
            "name": {
              "type": "string",
              "description": "Resource name"
            }
          },
          "required": ["name"]
        }
      }
    ]
  }
}
```

---

### `tools/call` Request - Create Prompt

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "method": "tools/call",
  "params": {
    "name": "create_prompt",
    "arguments": {
      "name": "test-prompt",
      "content": "This is a test prompt",
      "description": "A test prompt for demonstration"
    }
  }
}
```

**Success Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Created prompt 'test-prompt' in project 'auth-module'"
      }
    ],
    "isError": false
  }
}
```

**Error Response** (duplicate prompt):
```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Error: Prompt 'test-prompt' already exists in project 'auth-module'"
      }
    ],
    "isError": true
  }
}
```

**Error Response** (invalid name):
```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Error: Prompt name 'test prompt' contains invalid characters. Allowed: a-z, A-Z, 0-9, -, _, ."
      }
    ],
    "isError": true
  }
}
```

---

### `tools/call` Request - Update Prompt

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 8,
  "method": "tools/call",
  "params": {
    "name": "update_prompt",
    "arguments": {
      "name": "test-prompt",
      "content": "Updated prompt content"
    }
  }
}
```

**Success Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 8,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Updated prompt 'test-prompt' in project 'auth-module'"
      }
    ],
    "isError": false
  }
}
```

**Error Response** (prompt not found):
```json
{
  "jsonrpc": "2.0",
  "id": 8,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Error: Prompt 'test-prompt' does not exist in project 'auth-module'"
      }
    ],
    "isError": true
  }
}
```

---

### `tools/call` Request - Delete Prompt

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 9,
  "method": "tools/call",
  "params": {
    "name": "delete_prompt",
    "arguments": {
      "name": "test-prompt"
    }
  }
}
```

**Success Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 9,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Deleted prompt 'test-prompt' from project 'auth-module'"
      }
    ],
    "isError": false
  }
}
```

---

### `tools/call` Request - Create Resource

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 10,
  "method": "tools/call",
  "params": {
    "name": "create_resource",
    "arguments": {
      "name": "test-doc",
      "uri": "file:///test.md",
      "content": "# Test Document\n\nThis is a test.",
      "mime_type": "text/markdown",
      "description": "Test documentation"
    }
  }
}
```

**Success Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 10,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Created resource 'test-doc' in project 'auth-module'"
      }
    ],
    "isError": false
  }
}
```

---

### `tools/call` Request - Update Resource

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 11,
  "method": "tools/call",
  "params": {
    "name": "update_resource",
    "arguments": {
      "name": "test-doc",
      "content": "# Updated Test Document\n\nContent has changed.",
      "description": "Updated documentation"
    }
  }
}
```

**Success Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 11,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Updated resource 'test-doc' in project 'auth-module'"
      }
    ],
    "isError": false
  }
}
```

---

### `tools/call` Request - Delete Resource

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 12,
  "method": "tools/call",
  "params": {
    "name": "delete_resource",
    "arguments": {
      "name": "test-doc"
    }
  }
}
```

**Success Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 12,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Deleted resource 'test-doc' from project 'auth-module'"
      }
    ],
    "isError": false
  }
}
```

---

## Error Codes

MCP uses standard JSON-RPC 2.0 error codes:

| Code | Meaning | Usage |
|------|---------|-------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid request | Malformed JSON-RPC |
| -32601 | Method not found | Unknown method |
| -32602 | Invalid params | Invalid parameters (e.g., resource not found) |
| -32603 | Internal error | Server-side error |

For tool operations, errors are returned in the `result.isError` format (not JSON-RPC errors).

---

## Transport

**Protocol**: JSON-RPC 2.0 over stdio  
**Encoding**: UTF-8  
**Line-delimited**: Each message is a single line ending with `\n`

**Example Session**:
```
→ {"jsonrpc":"2.0","id":1,"method":"initialize","params":{...}}
← {"jsonrpc":"2.0","id":1,"result":{...}}
→ {"jsonrpc":"2.0","id":2,"method":"prompts/list","params":{}}
← {"jsonrpc":"2.0","id":2,"result":{"prompts":[...]}}
```

---

## Capabilities

The MCP server provides:
- ✅ **Prompts**: List and get prompts from selected project
- ✅ **Resources**: List and read resources from selected project
- ✅ **Tools**: Create, update, delete prompts and resources

Not supported:
- ❌ **Sampling**: Server does not request LLM sampling
- ❌ **Logging**: Server does not expose logging endpoints
- ❌ **Dynamic capabilities**: List changed notifications not supported

---

## Summary

The MCP API provides:
- Standard MCP protocol compliance (version 2024-11-05)
- Prompts API for listing and retrieving project-specific prompts
- Resources API for listing and reading project-specific resources
- Tools API for AI agents to manage prompts and resources
- Stdio transport for communication
- Structured error responses with actionable messages
- Support for both text and binary resource content
