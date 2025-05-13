# Supafilex

A PDF text extraction tool that uses OCR technology to extract text from PDF documents. The application downloads PDFs from URLs, renders pages as images, and extracts text content.

## Features

- Direct PDF download from URL
- PDF rendering to images
- OCR text extraction with Tesseract
- Performance metrics (processing time, character count)

## System Requirements

### Linux Dependencies

Install required system packages:

```bash
sudo apt-get update
sudo apt-get install -y \
  libmupdf-dev \
  libtesseract-dev \
  libleptonica-dev \
  tesseract-ocr \
  tesseract-ocr-por \
  libfontconfig1-dev \
  pkg-config \
  build-essential \
  clang \
  libclang-dev \
  libssl-dev
```

## Dependencies

- Rust
- mupdf = "0.5.0"
- image = "0.24"
- anyhow = "1.0"
- tesseract-sys = "0.6.2"
- leptonica-sys = "0.4.9"
- reqwest = { version = "0.11", features = ["blocking"] }

## Installation

1. Install Rust using [rustup](https://rustup.rs/)
2. Install system dependencies listed above
3. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/supafilex.git
   cd supafilex
   ```
4. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application:
```bash
./target/release/supafilex
```

## Project Structure

- `src/main.rs`: Core application code for PDF processing and OCR text extraction

## Contributing

Contributions welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License.