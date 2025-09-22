# 🪄 Rune — OCR Engine in Rust

**Rune** is a modern, Rust-native OCR (Optical Character Recognition) engine.  
Inspired by [Tesseract](https://github.com/tesseract-ocr/tesseract), Rune is designed to be:

- 🦀 **Safe & fast** — built entirely in Rust, no C++ bindings required.
- 🧩 **Modular** — engine, models, preprocessing, and CLI are cleanly separated crates.
- 🛠 **Embeddable** — use Rune as a library in your own apps, or run it as a CLI tool.
- 🌍 **Extensible** — add new models, languages, or preprocessing steps without touching the core.

Rune’s long-term goal is to become a drop-in, pure-Rust alternative to Tesseract for text recognition in scanned documents, photos, and real-time applications.

---

## 🚀 Status

Rune is **early in development**. Right now you can:

- Build the workspace.
- Run the CLI (`rune-cli`) which talks to the engine (`rune-core`).
- Load and inspect images (proof-of-life).

Planned milestones:

1. Image loading ✅
2. Preprocessing (grayscale, binarization, deskew) ⏳
3. Layout analysis (line / word segmentation)
4. Recognition via Rust-native models
5. Multi-language support & training tools

---

## 🏗 Project Layout

Rune is a Cargo **workspace** with multiple crates:

```
rune/
├── rune-core/          # OCR engine (library)
├── rune-cli/           # CLI binary (thin wrapper)
├── rune-preprocessing/ # Image cleanup (binarize, deskew, denoise)
├── rune-models/        # Model loading + inference backend
└── rune-utils/         # Shared helpers (logging, config, errors)
```

- **`rune-core`**: The heart of Rune — orchestrates preprocessing, segmentation, and recognition.
- **`rune-cli`**: Provides the `rune` command-line tool.
- **`rune-preprocessing`**: Handles image cleanup before recognition.
- **`rune-models`**: Defines model formats and loading logic.
- **`rune-utils`**: Common utilities for all crates.

---

## 📦 Installation

Clone the repo and build with Cargo:

```sh
git clone https://github.com/yourname/rune.git
cd rune
cargo build --workspace
```

Run the CLI:

```sh
cargo run -p rune-cli -- ./assets/test.png
```

---

## 🖥 Usage

Right now Rune only demonstrates image loading:

```sh
rune ./assets/test.png
```

Example output:

```
Loaded image: 1920x1080, RGB8
```

Future usage will look like:

```sh
rune scan.png -o result.txt
rune invoice.jpg --lang deu --format json
```

---

## 🔮 Roadmap

- [ ] Load and preprocess images
- [ ] Basic segmentation (rows/lines)
- [ ] Initial recognition model (digits/Latin alphabet)
- [ ] Confidence scoring & bounding boxes
- [ ] Multi-language support
- [ ] Structured output formats (JSON, hOCR)

---

## 🤝 Contributing

Rune is in its early days — contributions are very welcome!  
Ideas, feedback, and pull requests are appreciated.

---

## 📜 License

MIT — open source and free to use.  
