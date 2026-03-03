# PDFpstein

A command-line tool to convert **DOCX**, **PPTX**, and **Markdown** files to PDF.

## Features

- **Auto-detect input format** from file extension (or force with `--format`)
- **Extensive formatting options**: paper size, margins, fonts, orientation
- **Layout controls**: page numbers, table of contents, TOC depth
- **Quality presets**: fast, standard, high with configurable DPI
- **Image optimization**: compression with adjustable levels
- **PDF metadata**: title, author, subject, keywords
- **Security**: optional password protection
- **Batch processing** support
- **Auto-open** converted PDF

## Installation

```bash
cargo install --path .
```

## Usage

```bash
pdfpstein <INPUT_FILE> [OPTIONS]
```

### Basic Examples

```bash
# Convert a document (auto-detects format)
pdfpstein document.docx

# Convert with custom output path
pdfpstein report.md -o output/report.pdf

# Convert and open the PDF automatically
pdfpstein presentation.pptx --open

# Quiet mode (suppress output messages)
pdfpstein file.docx --quiet
```

### Formatting Options

```bash
# Set paper size and orientation
pdfpstein file.docx --paper-size letter --landscape

# Customize margins (mm)
pdfpstein file.md --margin 25
pdfpstein file.docx --margin-top 30 --margin-bottom 30 --margin-left 20 --margin-right 20

# Font settings
pdfpstein file.md --font-family "Times New Roman" --font-size 14

# Line height
pdfpstein file.docx --line-height 1.8
```

### Layout Options

```bash
# Add page numbers
pdfpstein file.md --show-page-numbers --page-number-position bottom-center

# Include table of contents
pdfpstein file.md --toc --toc-depth 4
```

### Quality & Performance

```bash
# High quality output
pdfpstein file.docx --quality high --dpi 300

# Optimize images
pdfpstein presentation.pptx --compress-images --image-compression-level 7

# Adjust workers and memory
pdfpstein file.docx --workers 4 --max-memory 1024
```

### Metadata & Security

```bash
# Set PDF metadata
pdfpstein report.docx --title "Annual Report" --author "John Doe" --keywords "finance,2026"

# Password protect
pdfpstein confidential.docx --password "secret123"

# Embed fonts
pdfpstein file.md --embed-fonts
```

## Full Options Reference

| Flag | Description | Default |
|------|-------------|---------|
| `<INPUT_FILE>` | Input file (.docx, .pptx, .md) | — |
| `-o, --output` | Output PDF path | Same as input, `.pdf` extension |
| `-f, --format` | Force format (docx/pptx/md) | Auto-detected |
| `-O, --open` | Open PDF after conversion | — |
| `-q, --quiet` | Suppress output messages | — |
| `--verbose` | Show detailed progress | — |
| `-p, --paper-size` | Paper size (a4/letter/legal/tabloid) | a4 |
| `--margin` | All margins in mm | 20 |
| `--margin-top/bottom/left/right` | Individual margin overrides | — |
| `-F, --font-family` | Font family | sans-serif |
| `-s, --font-size` | Font size (pt) | 12 |
| `--line-height` | Line height multiplier | 1.5 |
| `-l, --landscape` | Landscape orientation | — |
| `--show-page-numbers` | Display page numbers | — |
| `--page-number-position` | Page number position | bottom-center |
| `-t, --toc` | Include table of contents | — |
| `--toc-depth` | TOC heading levels (1-5) | 3 |
| `-q, --quality` | Quality preset (fast/standard/high) | standard |
| `--dpi` | Image rendering DPI | 150 |
| `-c, --compress-images` | Enable image compression | — |
| `--image-compression-level` | Compression level (1-9) | 5 |
| `-e, --embed-fonts` | Embed fonts in PDF | — |
| `--password` | Set PDF password | — |
| `--title/author/subject/keywords` | PDF metadata | — |
| `-w, --workers` | Parallel workers | Auto |
| `--max-memory` | Max memory (MB) | 512 |
| `--debug` | Enable debug output | — |
| `--experimental` | Enable experimental features | — |
| `-d, --batch-mode` | Process directory of files | — |
| `--keep-temp` | Keep intermediate files | — |

## Roadmap

See [roadmap.md](roadmap.md) for planned features. Current priorities:

- [ ] DOCX parsing
- [ ] Markdown parsing
- [ ] PPTX parsing

## License

MIT — see [LICENSE](LICENSE)
