# Quickstart: Using Twig Prompt Libraries

**For**: Software engineers using coding agents (OpenCode, Claude Code, etc.)  
**Duration**: 5 minutes  
**Goal**: Set up your first prompt library and use it in an agent

---

## Step 1: Create Your First Library

### Create the directory structure

```bash
# On Linux/macOS
mkdir -p ~/.local/share/twig/prompts/my_coding_lib/prompts

# On Windows
mkdir %APPDATA%\twig\prompts\my_coding_lib\prompts
```

### Create twig.toml configuration

Save as `~/.local/share/twig/prompts/my_coding_lib/twig.toml`:

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

[prompts.improve_docstring]
description = "Improve an existing docstring"

[[prompts.improve_docstring.arguments]]
name = "docstring"
description = "Current docstring"
required = true

[[prompts.improve_docstring.arguments]]
name = "function_name"
description = "Name of the function"
required = true
```

---

## Step 2: Create Prompt Content

### Code Review Prompt

Save as `~/.local/share/twig/prompts/my_coding_lib/prompts/code_review.md`:

```markdown
# Code Review

I need you to review the following code for quality and best practices.

{% if language %}**Language**: {{ language }}{% endif %}

## Code to Review

\`\`\`{{ language or 'text' }}
{{ code_snippet }}
\`\`\`

## Review Focus

Please evaluate:
- Code clarity and readability
- Adherence to best practices
- Potential performance issues
- Error handling and edge cases
- Security concerns

Provide specific, actionable feedback.
```

### Improve Docstring Prompt

Save as `~/.local/share/twig/prompts/my_coding_lib/prompts/improve_docstring.md`:

```markdown
# Improve Docstring

Please improve the docstring for this function:

**Function Name**: {{ function_name }}

**Current Docstring**:
\`\`\`
{{ docstring }}
\`\`\`

## Requirements

- Follow Google-style docstring format
- Include summary, detailed description, args, returns, raises
- Use clear, concise language
- Add usage examples if helpful
- Match the function's complexity and importance
```

---

## Step 3: Test with Twig CLI

### List available prompts

```bash
# Run the Twig MCP server
cargo run -- start

# In another terminal, the server exposes these prompts via MCP:
# - my_coding_lib:code_review
# - my_coding_lib:improve_docstring
```

### Use in OpenCode

When using OpenCode, your Twig MCP server provides these prompts:

```bash
# The server is running on stdio
# Agents can discover: my_coding_lib:code_review
# And invoke it with arguments
```

---

## Step 4: Add More Libraries

### Create a second library for data science

```bash
mkdir -p ~/.local/share/twig/prompts/ds_lib/prompts
```

Save `~/.local/share/twig/prompts/ds_lib/twig.toml`:

```toml
[prompts.analyze_dataset]
description = "Analyze a dataset and suggest improvements"

[[prompts.analyze_dataset.arguments]]
name = "dataset_description"
description = "Description of the dataset"
required = true

[[prompts.analyze_dataset.arguments]]
name = "head_rows"
description = "Sample rows from the dataset (CSV format)"
required = true

[[prompts.analyze_dataset.arguments]]
name = "target_variable"
description = "Target variable if doing supervised learning"

[prompts.model_comparison]
description = "Compare two ML models"

[[prompts.model_comparison.arguments]]
name = "model_a_metrics"
description = "Metrics for first model (JSON format)"
required = true

[[prompts.model_comparison.arguments]]
name = "model_b_metrics"
description = "Metrics for second model (JSON format)"
required = true
```

Save `~/.local/share/twig/prompts/ds_lib/prompts/analyze_dataset.md`:

```markdown
# Dataset Analysis

Analyze the following dataset:

{{ dataset_description }}

## Sample Data

\`\`\`
{{ head_rows }}
\`\`\`

{% if target_variable %}
## Target Variable

{{ target_variable }}
{% endif %}

## Analysis Task

Provide:
1. Summary statistics
2. Data quality issues
3. Feature engineering suggestions
4. Potential preprocessing steps
5. Recommended models
```

---

## Step 5: Use Prompts in Your Workflow

### Example 1: Code Review in OpenCode

```python
# Your code
def calculate_average(numbers):
    total = sum(numbers)
    count = len(numbers)
    return total / count
```

**Agent invokes**:
```
my_coding_lib:code_review with:
  code_snippet: "def calculate_average(numbers):\n    total = sum(numbers)\n    count = len(numbers)\n    return total / count"
  language: "python"
```

**Server returns** (prompt + arguments rendered):
```markdown
# Code Review

I need you to review the following code for quality and best practices.

**Language**: python

## Code to Review

\`\`\`python
def calculate_average(numbers):
    total = sum(numbers)
    count = len(numbers)
    return total / count
\`\`\`

[... rest of rendered prompt ...]
```

### Example 2: Dataset Analysis

```
ds_lib:analyze_dataset with:
  dataset_description: "Customer purchase history with 50k rows"
  head_rows: "id,customer_id,amount,date\n1,101,99.99,2025-01-01\n2,102,49.99,2025-01-01"
  target_variable: "amount (continuous variable for regression)"
```

---

## Library Organization Best Practices

### Directory Naming

Use clear, descriptive names:
- ✅ `code_review_lib` - Clear purpose
- ✅ `python_patterns` - Language/domain specific
- ✅ `ml_workflows` - Task-focused
- ❌ `lib1`, `my_stuff` - Too vague
- ❌ `UPPERCASE`, `kebab-case` - Will be normalized to `uppercase`, `kebab_case`

### Prompt Organization

Group related prompts in one library:
- ❌ One prompt per library
- ✅ Related prompts (code review, improve docstring, test generation) in one library

### Naming Conventions

**Library names**: lowercase, underscores, hyphen-safe (converted to underscore)
```
My-Code Lib → my_code_lib
MyCodeLib → mycodelib
my_code_lib → my_code_lib
```

**Prompt names**: lowercase, underscores, no spaces
```
code_review ✅
codeReview ❌
Code Review ❌ (will be converted to code_review)
```

### Markdown Content Tips

1. **Use Jinja2 syntax** for variable substitution: `{{ variable_name }}`
2. **Make arguments optional** with conditionals:
   ```markdown
   {% if language %}
   Programming language: {{ language }}
   {% endif %}
   ```
3. **Provide examples** in your prompts
4. **Use code blocks** with syntax highlighting hints
5. **Test rendering** with all argument combinations

---

## Troubleshooting

### Prompts not appearing in list

1. Check directory structure:
   ```
   ~/.local/share/twig/prompts/your_lib/
   ├── twig.toml          ← must exist
   └── prompts/
       └── *.md           ← markdown files here
   ```

2. Verify twig.toml is valid:
   ```bash
   # Check TOML syntax
   cat ~/.local/share/twig/prompts/your_lib/twig.toml
   ```

3. Restart the server:
   ```bash
   cargo run -- start
   ```

### Prompt doesn't render correctly

1. Check markdown file exists with correct name:
   ```bash
   ls ~/.local/share/twig/prompts/your_lib/prompts/your_prompt.md
   ```

2. Verify variable names in `twig.toml` match template usage:
   ```toml
   # twig.toml
   required_arguments = ["code_snippet", "language"]
   ```
   ```markdown
   # your_prompt.md
   {{ code_snippet }}    ← matches
   {{ language }}        ← matches
   {{ missing_var }}     ← will be empty
   ```

3. Check for Jinja2 syntax errors:
   ```markdown
   ✅ {{ variable }}
   ❌ {{ variable }      ← unclosed
   ❌ {# comment }       ← comment syntax
   ```

### Missing required argument error

When using the prompt, provide all required arguments:

```toml
# twig.toml
required_arguments = ["code_snippet", "language"]
```

```json
// Must provide both in request
{
  "arguments": {
    "code_snippet": "...",
    "language": "python"
  }
}
```

---

## Next Steps

1. **Customize your libraries** for your typical workflows
2. **Share libraries** with teammates (copy directory structure)
3. **Version your libraries** using semantic versioning in twig.toml
4. **Monitor prompts** that work well; refine and improve

---

## File Locations by Platform

| Platform | Path |
|----------|------|
| Linux | `~/.local/share/twig/prompts/` |
| macOS | `~/Library/Application Support/twig/prompts/` |
| Windows | `%APPDATA%\twig\prompts\` |

---

**Status**: Quick start guide complete. Ready for implementation and testing.
