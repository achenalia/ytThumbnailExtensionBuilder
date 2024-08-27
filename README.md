
# ytThumbnailExtensionBuilder
![image](https://github.com/user-attachments/assets/de83575e-bbbb-49e3-9b29-a4c65849cc33)

## Overview

`ytThumbnailExtensionBuilder` is a Rust-based CLI tool designed to create browser extensions that inject a random image from a predefined set into each thumbnail on a YouTube webpage. This tool allows you to customize the appearance of thumbnails with unique images, inspired by the Mr. Beastify browser extension.

For reference, check out the Mr. Beastify extension here: [Mr. Beastify Extension](https://chromewebstore.google.com/detail/youtube-mrbeastify/dbmaeobgdodeimjdjnkipbfhgeldnmeb).

NOTE: Extensions are for Firefox and Chrome only at the moment.

## Prerequisites

- Ensure you have Rust and Cargo installed on your system. You can install Rust from [the official Rust website](https://www.rust-lang.org/tools/install).

## How to Setup

To build the tool, use the following command:

```bash
cargo build --release
```

This will generate an executable named `ytThumbnailExtensionBuilder` in the `target/release` directory.

## How to Use

### Windows

1. Open Command Prompt or PowerShell.
2. Navigate to the directory where `ytThumbnailExtensionBuilder.exe` is located:

   ```bash
   cd path\to\target\release
   ```

3. Run the tool:

   ```bash
   ytThumbnailExtensionBuilder.exe
   ```

### macOS

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

### Linux

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

## Notes

- The tool will allow you to input your desired parameters for the extension upon running. Ensure that any necessary configurations or setups are complete before executing the tool.
- The extension directory will be created per your input and will have a ready-to-use extension within it, but don't forget to add the photos into the extension/assets/images/ directory.
- *TO SUBMIT YOUR EXTENSION TO YOUR BROWSER'S EXTENSION STORE:* make sure to create a zip file containing the new extension with all the files included in the extension directory, then follow the procedure for your given browser.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
