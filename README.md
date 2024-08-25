---

## Overview

This project is a Rust-based CLI tool designed for creating browser extensions that dynamically inject a random image from a predefined set into each thumbnail on a webpage. 

Inspired by the Mr. Beastify browser extension, this tool provides a straightforward method for customizing the appearance of thumbnails with unique images. For reference, you can explore the original Mr. Beastify extension here: [Mr. Beastify Extension](https://chromewebstore.google.com/detail/youtube-mrbeastify/dbmaeobgdodeimjdjnkipbfhgeldnmeb).

---

## How to Use

### Prerequisites

- Ensure that you have Rust and Cargo installed on your system. You can install Rust from [the official Rust website](https://www.rust-lang.org/tools/install).

### Building the CLI Tool

To build the CLI tool, use the following command:

```bash
cargo build --release
```

This will generate an executable named `ytThumbnailExtensionBuilder` in the `target/release` directory.

### Using the CLI Tool

#### Windows

1. Open Command Prompt or PowerShell.
2. Navigate to the directory where `ytThumbnailExtensionBuilder.exe` is located:

   ```bash
   cd path\to\target\release
   ```

3. Run the tool:

   ```bash
   ytThumbnailExtensionBuilder.exe
   ```

#### macOS

1. Open Terminal.
2. Navigate to the directory where `ytThumbnailExtensionBuilder` is located:

   ```bash
   cd path/to/target/release
   ```

3. Make the executable file if it is not already executable:

   ```bash
   chmod +x ytThumbnailExtensionBuilder
   ```

4. Run the tool:

   ```bash
   ./ytThumbnailExtensionBuilder
   ```

#### Linux

1. Open Terminal.
2. Navigate to the directory where `ytThumbnailExtensionBuilder` is located:

   ```bash
   cd path/to/target/release
   ```

3. Make the executable file if it is not already executable:

   ```bash
   chmod +x ytThumbnailExtensionBuilder
   ```

4. Run the tool:

   ```bash
   ./ytThumbnailExtensionBuilder
   ```

### Notes

- The tool will start and allow you to input your desired parameters upon running. Ensure that the necessary configuration or setup is done before running the tool.

---

Feel free to modify the paths as needed based on your directory structure.
