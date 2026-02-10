# Markdown Analyzer Documentation

## Overview

The Markdown Analyzer is a powerful command-line tool for analyzing, viewing, and managing markdown documentation within your project. It provides comprehensive insights into your markdown files, including statistics, content analysis, and structure visualization.

## Installation

The markdown analyzer is located in the `bin/` directory. To make it easily accessible:

```bash
# Make it executable (already done)
chmod +x bin/md-analyzer

# Optionally, add to your PATH
export PATH="$PATH:$(pwd)/bin"

# Or create an alias
alias md-analyzer='./bin/md-analyzer'
```

## Commands

### 1. `analyze` - Comprehensive Analysis

Analyzes all markdown files in the project and provides detailed statistics.

```bash
./bin/md-analyzer analyze
```

**Output includes:**
- Line count per file
- Word count per file
- Character count per file
- Number of headers
- Number of code blocks
- Number of links
- Number of images
- Total statistics across all files

**Example output:**
```
FILE                                        LINES    WORDS      CHARS  HEADERS     CODE    LINKS   IMAGES
----                                        -----    -----      -----  -------     ----    -----   ------
README.md                                     313     1223       8147       62        8        4        0
docs/architecture.md                          499     1705      13557       70       18        4        0
docs/workflow.md                              367     1208       7785       61       22        1        0
----                                        -----    -----      -----  -------     ----    -----   ------
TOTAL                                        1399     5483      38588      221       56       24        1
```

### 2. `list` - List All Markdown Files

Lists all markdown files in the project with their sizes.

```bash
./bin/md-analyzer list
```

**Example output:**
```
Found 10 file(s):

  1. .github/steps/0-welcome.md (4.0K)
  2. README.md (8.0K)
  3. docs/architecture.md (16K)
  ...
```

### 3. `view` - View a Specific File

View a markdown file with optional syntax highlighting (if `bat` or `pygmentize` is available).

```bash
./bin/md-analyzer view README.md
./bin/md-analyzer view docs/workflow.md
```

**Features:**
- Syntax highlighting (when available)
- Headers highlighted in blue
- Code blocks highlighted in yellow
- List items highlighted in green

### 4. `tree` - Project Structure

Display the project directory structure (requires `tree` command, or uses fallback).

```bash
./bin/md-analyzer tree
```

### 5. `stats` - Statistics Summary

Show high-level statistics about markdown documentation in the project.

```bash
./bin/md-analyzer stats
```

**Output includes:**
- Total number of markdown files
- Total size of all markdown files
- Average file size
- Largest file
- Documentation coverage (README, LICENSE, docs directory)

**Example output:**
```
Project Statistics:

  Total markdown files: 10
  Total size: 37KB
  Average size: 3KB
  Largest file: docs/architecture.md (13KB)

Documentation Coverage:

  ✓ README.md
  ✓ LICENSE
  ✓ docs/ directory
```

## Use Cases

### 1. Documentation Audit

Quickly assess the state of your project documentation:

```bash
# Get comprehensive statistics
./bin/md-analyzer stats

# Analyze all files in detail
./bin/md-analyzer analyze
```

### 2. Finding Documentation Gaps

Use the analyzer to identify:
- Files with few headers (poor structure)
- Files with no code examples
- Files with no links to other documentation

### 3. Documentation Maintenance

Before releasing a new version:

```bash
# List all markdown files
./bin/md-analyzer list

# Review each important file
./bin/md-analyzer view README.md
./bin/md-analyzer view docs/architecture.md
```

### 4. Content Migration

When reorganizing documentation:

```bash
# Analyze current state
./bin/md-analyzer analyze > before.txt

# Make changes...

# Compare after
./bin/md-analyzer analyze > after.txt
diff before.txt after.txt
```

## Features

### Automatic File Discovery

The analyzer automatically finds all `.md` files in your project, excluding:
- `.git/` directory
- `node_modules/` directory
- `.worktrees/` directory

### Smart Analysis

- **Headers**: Counts all header levels (#, ##, ###, etc.)
- **Code Blocks**: Counts pairs of triple backticks (```)
- **Links**: Detects markdown links `[text](url)`
- **Images**: Detects markdown images `![alt](url)`

### Color-Coded Output

- 🔵 Blue: Information messages
- 🟢 Green: Success messages and positive indicators
- 🟡 Yellow: Warnings
- 🔴 Red: Errors
- 🟣 Magenta/Cyan: Headers and titles

## Tips and Tricks

### Quick Analysis Workflow

```bash
# 1. Get overview
./bin/md-analyzer stats

# 2. See all files
./bin/md-analyzer list

# 3. Deep dive into specific files
./bin/md-analyzer view README.md

# 4. Full analysis
./bin/md-analyzer analyze
```

### Piping and Filtering

```bash
# Save analysis to file
./bin/md-analyzer analyze > analysis.txt

# Find files with specific content
./bin/md-analyzer list | grep "docs"

# Count total lines in all markdown files
./bin/md-analyzer analyze | grep "TOTAL" | awk '{print $2}'
```

### Integration with Other Tools

```bash
# Use with git to track documentation changes
git add docs/
./bin/md-analyzer analyze
git commit -m "Update documentation"

# Check documentation before commit
./bin/md-analyzer stats && git status
```

## Troubleshooting

### "No markdown files found"

This means the analyzer couldn't find any `.md` files in your project. Make sure you're running it from the project root directory.

### Syntax Highlighting Not Working

The `view` command tries to use these tools in order:
1. `bat` - Modern syntax highlighter
2. `pygmentize` - Python-based highlighter
3. Fallback - Basic color highlighting

To get better highlighting, install one of these:

```bash
# Install bat (recommended)
# macOS
brew install bat

# Ubuntu/Debian
apt install bat

# Or install pygmentize
pip install pygments
```

### Permission Denied

If you get a permission error:

```bash
chmod +x bin/md-analyzer
```

## Advanced Usage

### Custom Analysis Scripts

You can build on top of the analyzer:

```bash
#!/bin/bash
# check-docs.sh - Ensure documentation quality

# Check if README exists and has enough content
readme_words=$(./bin/md-analyzer analyze | grep "README.md" | awk '{print $3}')
if [ "$readme_words" -lt 100 ]; then
    echo "ERROR: README.md is too short (less than 100 words)"
    exit 1
fi

echo "Documentation quality check passed!"
```

### CI/CD Integration

Add to your CI pipeline:

```yaml
# .github/workflows/docs-check.yml
name: Documentation Check
on: [push, pull_request]
jobs:
  check-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Analyze Documentation
        run: |
          chmod +x bin/md-analyzer
          ./bin/md-analyzer analyze
          ./bin/md-analyzer stats
```

## Version

Current version: 1.0.0

## License

This tool is part of the project and follows the same license.

## Contributing

To improve the markdown analyzer:

1. Fork the repository
2. Make changes to `bin/md-analyzer`
3. Test thoroughly with various markdown files
4. Submit a pull request

## Support

For issues or questions:
- Check existing documentation
- Review the source code in `bin/md-analyzer`
- Open an issue in the repository
