# PDFpstein - Project Context

## Project Overview

**PDFpstein** is a Rust-based command-line tool that converts DOCX, PPTX, and Markdown files to PDF format. The project is in early development (v0.1.0) with a well-structured architecture in place.

### Architecture

The project follows a modular library + binary pattern:

```
pdfstein/
├── src/
│   ├── main.rs              # CLI entry point (minimal bootstrap)
│   ├── lib.rs               # Library root, re-exports public API
│   ├── error.rs             # Centralized error types
│   ├── cli/                 # Command-line interface
│   │   ├── mod.rs
│   │   └── args.rs          # CLI argument definitions (clap)
│   ├── converter/           # Format converters (trait-based)
│   │   ├── mod.rs           # Converter trait + factory
│   │   ├── docx.rs          # DOCX converter
│   │   ├── pptx.rs          # PPTX converter
│   │   └── md.rs            # Markdown converter
│   ├── config/              # Configuration structs
│   │   ├── mod.rs
│   │   └── options.rs       # ConversionOptions
│   ├── pdf/                 # PDF generation backend
│   │   ├── mod.rs
│   │   └── metadata.rs      # PDF metadata handling
│   └── utils/               # Utility functions
│       ├── mod.rs
│       ├── path.rs          # Path utilities
│       └── fs.rs            # File system utilities
└── tests/
    ├── common/              # Test helpers
    └── integration/         # End-to-end tests
```

### Key Design Patterns

- **Trait-based converters**: Each format implements the `Converter` trait
- **Factory pattern**: `create_converter()` returns the appropriate converter
- **Library + binary**: Core logic in `lib.rs`, CLI bootstrap in `main.rs`
- **Centralized errors**: Single `Error` enum with `Result<T>` type alias

## Building and Running

### Installation

```bash
cargo install --path .
```

### Development

```bash
# Build
cargo build

# Run with arguments
cargo run -- <input_file> [options]

# Example
cargo run -- document.docx --output output.pdf

# Check without building
cargo check

# Run tests (integration tests are ignored until implemented)
cargo test -- --ignored
```

## Current Development Status

| Component | Status |
|-----------|--------|
| CLI parsing | ✅ Complete (40+ options) |
| Project structure | ✅ Complete |
| Error handling | ✅ Scaffolded |
| Converter trait | ✅ Defined |
| DOCX conversion | ❌ Not implemented |
| PPTX conversion | ❌ Not implemented |
| Markdown conversion | ❌ Not implemented |
| PDF generation | ❌ Not implemented |

See `roadmap.md` for planned features.

## Code Conventions

### Style

- **Rust 2024 edition** conventions
- **clap derive macros** for CLI parsing
- **Module pattern**: Each module has `mod.rs` that exports public items
- **Documentation**: Doc comments (`///`) on all public items
- **Error handling**: `Result<T>` alias with custom `Error` enum
- **Early returns** for error handling in `main.rs`

### Testing Practices

- Integration tests in `tests/integration/`
- Test helpers in `tests/common/`
- Tests marked with `#[ignore]` until implementation is complete
- One test file per converter format

### File Organization

| Directory | Purpose |
|-----------|---------|
| `src/cli/` | CLI argument definitions and parsing |
| `src/converter/` | Format-specific conversion logic |
| `src/config/` | Configuration structs |
| `src/pdf/` | PDF generation backend |
| `src/utils/` | Shared utility functions |
| `tests/integration/` | End-to-end conversion tests |

## Public API

The library exports:

- `Args` - CLI arguments struct
- `Format` - Supported input formats enum
- `Converter` - Trait for document converters
- `Result<T>` - Application result type
- `Error` - Application error type

## Dependencies

Current dependencies:

```toml
[dependencies]
clap = { version = "4.5.60", features = ["derive"] }
```

Expected future dependencies (not yet added):

- **Error handling**: `thiserror` for derive-based error enums
- **DOCX**: `docx-rs` or similar
- **Markdown**: `pulldown-cmark`, `comrak`
- **PDF generation**: `printpdf`, `lopdf`, or external tool integration

## Usage Examples

```bash
# Basic conversion
pdfpstein document.docx

# With custom output
pdfpstein report.md -o output/report.pdf

# With formatting options
pdfpstein file.docx --paper-size letter --margin 25 --font-size 14

# High quality with metadata
pdfpstein report.docx --quality high --dpi 300 \
  --title "Annual Report" --author "John Doe"
```

## Implementation Priorities

Based on `roadmap.md`:

1. **Markdown parsing** - Simplest format, good starting point
2. **DOCX parsing** - Most common use case
3. **PPTX parsing** - Slide-to-page conversion

## License

MIT License - Copyright (c) 2026 Andrés San Martin
