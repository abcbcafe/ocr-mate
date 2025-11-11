# OCR-Mate 📄✨

An intelligent document OCR tool with split-view editing, powered by DeepSeek-OCR and built with Rust + egui.

## Features

- **Split-View Interface**: View your documents on the left while editing OCR results on the right
- **Multiple Document Formats**: Support for PDF files and images (PNG, JPG, TIFF, WebP)
- **DeepSeek-OCR Integration**: Leverage the powerful DeepSeek-OCR model from HuggingFace
- **Custom Instructions**: Guide the OCR engine with custom prompts and hints
- **Real-time Editing**: Edit OCR results directly with full text editing capabilities
- **Multiple Export Formats**: Export to plain text, Markdown, or JSON
- **Page-by-Page Processing**: OCR individual pages or batch process entire documents
- **Zoom & Navigation**: Zoom in/out and navigate through multi-page documents

## Screenshots

```
┌─────────────────────────────────────────────────────────────┐
│  📁 Open  🔍 OCR Page  🔍 OCR All  ⚙️ Settings  💾 Export │
├──────────────────────┬──────────────────────────────────────┤
│                      │                                      │
│   Document Viewer    │      OCR Results Editor             │
│                      │                                      │
│   [PDF/Image]        │   [Editable Text]                   │
│                      │                                      │
│   [Zoom: 100%]       │   Characters: 1234 | Lines: 45      │
│                      │                                      │
├──────────────────────┴──────────────────────────────────────┤
│  Page 1 of 5 | /path/to/document.pdf                       │
└─────────────────────────────────────────────────────────────┘
```

## Prerequisites

### System Dependencies

#### PDFium Library

OCR-Mate requires the PDFium library for PDF rendering. We provide an automated setup script:

**Quick Setup (Linux/macOS)**:
```bash
# Run the setup script to download PDFium
./setup-pdfium.sh
```

This will download the appropriate PDFium library for your platform and place it in the project directory.

**Manual Installation**:
If you prefer to install PDFium manually, download the appropriate binary from [pdfium-binaries](https://github.com/bblanchon/pdfium-binaries/releases) and place it in the project root:
- Linux: `libpdfium.so`
- macOS: `libpdfium.dylib`
- Windows: `pdfium.dll`

**Verify Installation**:
After running the setup script, you should see the PDFium library in your project directory:
```bash
ls -lh libpdfium.so  # Linux
ls -lh libpdfium.dylib  # macOS
# Ensure you have Xcode Command Line Tools installed
xcode-select --install
```

**Windows**:
```bash
# PDFium will be automatically downloaded
# No additional action required
```

### Rust Toolchain

Install Rust using [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/ocr-mate.git
cd ocr-mate

# Build the project
cargo build --release

# Run the application
cargo run --release
```

The compiled binary will be available at `target/release/ocr-mate`.

## Configuration

### HuggingFace API Key

To use DeepSeek-OCR, you need a HuggingFace API token:

1. Create an account at [HuggingFace](https://huggingface.co/)
2. Generate an API token from [Settings > Access Tokens](https://huggingface.co/settings/tokens)
3. In OCR-Mate, click **⚙️ Settings** and enter your API key

### OCR Instructions

You can customize the OCR behavior with system instructions:

**Examples**:
- "Extract all text preserving table formatting as markdown"
- "This document is in Spanish. Extract text accurately."
- "Focus on mathematical equations and preserve LaTeX notation"
- "Extract only the main body text, ignore headers and footers"

## Usage

### Basic Workflow

1. **Open a Document**: Click `📁 Open Document` and select a PDF or image file
2. **Configure OCR** (optional): Click `⚙️ Settings` to add custom instructions
3. **Run OCR**:
   - `🔍 OCR Current Page` - Process only the current page
   - `🔍 OCR All Pages` - Process all pages in the document
4. **Edit Results**: Modify the OCR output in the right panel
5. **Export**: Click `💾 Export` and choose your format

### Navigation

- Use `←` and `→` buttons to navigate between pages
- Or use the page selector at the top
- Zoom controls are in the bottom-left of the document viewer

### Export Formats

- **Plain Text (.txt)**: Simple text with page separators
- **Markdown (.md)**: Text with page headings and separators
- **JSON (.json)**: Structured format with metadata and per-page text

## Architecture

```
ocr-mate/
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Main application state
│   ├── ui/              # UI components (viewer, editor, toolbar)
│   ├── ocr/             # OCR provider trait and implementations
│   ├── document/        # Document loading and rendering
│   ├── export/          # Export functionality
│   └── utils/           # Configuration and utilities
```

## Extending OCR-Mate

### Adding OCR Providers

To add support for additional OCR engines:

1. Implement the `OcrProvider` trait in `src/ocr/provider.rs`
2. Add your implementation to `src/ocr/`
3. Update the app to allow provider selection

Example:
```rust
use async_trait::async_trait;
use crate::ocr::{OcrProvider, OcrRequest, OcrResult};

pub struct MyOcrProvider;

#[async_trait]
impl OcrProvider for MyOcrProvider {
    async fn process(&self, request: OcrRequest) -> Result<OcrResult> {
        // Your implementation here
    }

    fn name(&self) -> &str {
        "My OCR Provider"
    }

    fn is_ready(&self) -> bool {
        true
    }
}
```

## Troubleshooting

### PDFium Library Not Found

If you encounter PDFium-related errors:

1. The library should auto-download, but you can manually install it:
   - Download from [PDFium releases](https://github.com/bblanchon/pdfium-binaries/releases)
   - Place the library in your system library path or project root

### OCR API Errors

- **401 Unauthorized**: Check your HuggingFace API key in Settings
- **Rate Limited**: HuggingFace free tier has rate limits; wait or upgrade
- **Model Loading**: The model may take time to load on first request

### Performance Issues

- Large PDFs may take time to render; consider processing page-by-page
- OCR processing time depends on image resolution and complexity
- Close other applications if experiencing memory issues

## Roadmap

- [ ] Support for local OCR models (Tesseract, PaddleOCR)
- [ ] Batch processing multiple documents
- [ ] OCR confidence visualization
- [ ] Side-by-side comparison view
- [ ] Keyboard shortcuts
- [ ] Dark/light theme toggle
- [ ] Searchable PDF export with text overlay
- [ ] Multi-language UI
- [ ] Plugin system for custom OCR providers

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is dual-licensed under MIT OR Apache-2.0.

## Acknowledgments

- [DeepSeek-OCR](https://huggingface.co/deepseek-ai/Deepseek-Ocr) - Powerful OCR model
- [egui](https://github.com/emilk/egui) - Immediate mode GUI framework
- [pdfium-render](https://github.com/ajrcarey/pdfium-render) - PDF rendering library
- [HuggingFace](https://huggingface.co/) - Model hosting and inference API

## Support

If you find this project helpful, please consider:
- ⭐ Starring the repository
- 🐛 Reporting bugs
- 💡 Suggesting features
- 🤝 Contributing code

---

**Made with ❤️ and Rust**
