# Quick Start: Markdown Analyzer

## Run Analysis in 3 Steps

### Step 1: List all markdown files
```bash
./bin/md-analyzer list
```

This shows you all the markdown files in your project.

### Step 2: Analyze the files
```bash
./bin/md-analyzer analyze
```

This provides detailed statistics about each file.

### Step 3: View specific files
```bash
./bin/md-analyzer view README.md
```

## Common Commands

### Show Statistics
```bash
./bin/md-analyzer stats
```

### Show Project Structure
```bash
./bin/md-analyzer tree
```

### Get Help
```bash
./bin/md-analyzer help
```

## Example Output

### Analysis Report
```
📊 Markdown Analysis Report

FILE                                        LINES    WORDS      CHARS  HEADERS     CODE    LINKS   IMAGES
----                                        -----    -----      -----  -------     ----    -----   ------
README.md                                     313     1223       8147       62        8        4        0
docs/architecture.md                          499     1705      13557       70       18        4        0
docs/workflow.md                              367     1208       7785       61       22        1        0
----                                        -----    -----      -----  -------     ----    -----   ------
TOTAL                                        1399     5483      38588      221       56       24        1
```

### Statistics Summary
```
📈 Markdown Statistics

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

## Next Steps

For detailed documentation, see: [docs/md-analyzer-guide.md](md-analyzer-guide.md)
