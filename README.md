# PDFpstein

A unified command-line tool to convert **DOCX**, **PPTX**, and **Markdown** files to PDF.

## Features

- **Subcommand-based CLI** - Clean separation between formats (`md`, `docx`, `pptx`)
- **Format-specific options** - Each format has tailored flags
- **Auto-detect input format** - From file extension
- **Common flags across all formats** - `--output`, `--open`, `--quiet`, `--verbose`
- **Extensible architecture** - Easy to add new formats
- **Cross-platform** - Works on Linux, macOS, and Windows

## Installation

```bash
cargo install --path .
```

## Usage

```bash
pdfpstein <COMMAND> [OPTIONS]
```

### Commands

| Command | Description |
|---------|-------------|
| `md`    | Convert Markdown files to PDF |
| `docx`  | Convert DOCX files to PDF |
| `pptx`  | Convert PPTX files to PDF |

### Basic Examples

```bash
# Convert a Markdown file
pdfpstein md readme.md

# Convert a DOCX file
pdfpstein docx report.docx

# Convert a PPTX file
pdfpstein pptx slides.pptx

# Convert with custom output path
pdfpstein md report.md -o output/report.pdf

# Convert and open the PDF automatically
pdfpstein docx document.docx --open

# Quiet mode (suppress output messages)
pdfpstein pptx presentation.pptx --quiet
```

---

## Markdown Command (`md`)

```bash
pdfpstein md <FILE> [OPTIONS]
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `<FILE>` | Input Markdown file (.md or .markdown) | — |
| `-o, --output` | Output PDF file path | Same as input, `.pdf` extension |
| `--gfm` | Enable GitHub Flavored Markdown (tables, strikethrough, task lists) | — |
| `--highlight` | Enable syntax highlighting for code blocks | — |
| `--toc-depth` | Maximum heading depth for table of contents | 3 |
| `-O, --open` | Open PDF after successful conversion | — |
| `-q, --quiet` | Suppress non-essential output messages | — |
| `--verbose` | Show verbose error messages and progress | — |

### Examples

```bash
# Basic conversion
pdfpstein md readme.md

# With GitHub Flavored Markdown and syntax highlighting
pdfpstein md document.md --gfm --highlight

# Custom TOC depth
pdfpstein md guide.md --toc-depth 4

# Convert and open
pdfpstein md notes.md --open
```

---

## DOCX Command (`docx`)

```bash
pdfpstein docx <FILE> [OPTIONS]
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `<FILE>` | Input DOCX file (.docx) | — |
| `-o, --output` | Output PDF file path | Same as input, `.pdf` extension |
| `--extract-images` | Extract and embed images from the document | — |
| `--preserve-styles` | Preserve document styles and formatting | true |
| `--track-changes` | Handle track changes (`accept`, `reject`, or `show`) | accept |
| `-O, --open` | Open PDF after successful conversion | — |
| `-q, --quiet` | Suppress non-essential output messages | — |
| `--verbose` | Show verbose error messages and progress | — |

### Examples

```bash
# Basic conversion
pdfpstein docx report.docx

# Extract images and preserve styles
pdfpstein docx document.docx --extract-images --preserve-styles true

# Reject track changes
pdfpstein docx draft.docx --track-changes reject

# Convert and open
pdfpstein docx letter.docx --open
```

---

## PPTX Command (`pptx`)

```bash
pdfpstein pptx <FILE> [OPTIONS]
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `<FILE>` | Input PPTX file (.pptx) | — |
| `-o, --output` | Output PDF file path | Same as input, `.pdf` extension |
| `--slide-range` | Slide range to convert (e.g., `"1-5,8,10-"`) | — |
| `--include-notes` | Include speaker notes in the PDF | — |
| `--slides-per-page` | Slides per page (handout mode, 1-9) | 1 |
| `-O, --open` | Open PDF after successful conversion | — |
| `-q, --quiet` | Suppress non-essential output messages | — |
| `--verbose` | Show verbose error messages and progress | — |

### Examples

```bash
# Basic conversion
pdfpstein pptx slides.pptx

# Convert specific slides
pdfpstein pptx presentation.pptx --slide-range "1-5,8,10-"

# Include speaker notes
pdfpstein pptx talk.pptx --include-notes

# Handout mode (4 slides per page)
pdfpstein pptx slides.pptx --slides-per-page 4

# Convert and open
pdfpstein pptx deck.pptx --open
```

---

## Common Flags (All Commands)

All subcommands share these flags:

| Flag | Description |
|------|-------------|
| `-o, --output <PATH>` | Output PDF file path (defaults to same name as input) |
| `-O, --open` | Open the PDF after successful conversion |
| `-q, --quiet` | Suppress non-essential output messages |
| `--verbose` | Show verbose error messages and progress |
| `-h, --help` | Print help for the specific command |

---

## Architecture

PDFpstein uses a **modular architecture** with clear separation of concerns:

```
src/
├── cli/                    # Command-line interface
│   ├── args.rs            # Main Args struct
│   ├── commands/          # Subcommand definitions
│   │   ├── md.rs          # Markdown command
│   │   ├── docx.rs        # DOCX command
│   │   └── pptx.rs        # PPTX command
│   └── shared/            # Shared utilities
│       ├── common.rs      # CommandCommon trait
│       └── validators.rs  # Input validation
├── converter/              # Format converters
│   ├── md.rs              # Markdown converter
│   ├── docx.rs            # DOCX converter
│   └── pptx.rs            # PPTX converter
└── pdf/                    # PDF generation backend
```

### Key Design Patterns

- **Subcommand-based CLI** - Each format has its own command with specific options
- **Trait-based converters** - All formats implement the `Converter` trait
- **Common interface** - `CommandCommon` trait for shared command behavior
- **Factory pattern** - `create_converter()` returns the appropriate converter

---

## Development

```bash
# Build the project
cargo build

# Run with arguments
cargo run -- md readme.md

# Check without building
cargo check

# Run tests
cargo test -- --ignored
```

---

## Roadmap

See [roadmap.md](roadmap.md) for detailed plans. Current development status:

| Component | Status |
|-----------|--------|
| CLI structure (subcommands) | ✅ Complete |
| Format-specific options | ✅ Complete |
| Common command interface | ✅ Complete |
| Markdown conversion | ✅ Complete |
| DOCX conversion | ❌ Not Started |
| PPTX conversion | ❌ Not Started |
| PDF generation backend | ✅ Complete (via markdown2pdf) |

---

## License

MIT — see [LICENSE](LICENSE)

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
