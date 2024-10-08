# Cleaner-Upper-RS

Cleaner-Upper-RS is a fast, cross-platform file system cleaner for removing recreatable folders and cache directories across different operating systems. This Rust-based program automatically detects and cleans common cache locations, helping to free up disk space efficiently.

## Features

- Automatic detection and cleaning of common cache directories across Windows, macOS, and Linux
- Recursive scanning and cleaning of directories named "cache" (case-insensitive)
- Fast and efficient directory cleaning using parallel processing
- Cross-platform compatibility
- Safe cleanup with error handling and reporting
- Automated releases using release-plz
- Comprehensive documentation

## Installation

To install Cleaner-Upper-RS, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo installed, you can build the project from source:

```bash
git clone https://github.com/maccam912/cleaner-upper-rs.git
cd cleaner-upper-rs
cargo build --release
```

The compiled binary will be available in the `target/release` directory.

## Usage

Run the executable without any arguments to clean common cache directories:

```bash
./cleaner-upper-rs
```

The program will automatically detect the operating system and clean relevant cache directories.

**Note:** Use this tool with caution. Always ensure you have backups of important data before running cleanup operations.

## Automatic Cache Detection

Cleaner-Upper-RS automatically detects and cleans common cache directories based on the operating system:

- Windows: Includes temporary folders, browser caches, and Windows-specific caches
- macOS: Includes user and system library caches, browser caches, and application-specific caches
- Linux: Includes user cache directories, temporary folders, and common application caches

The tool also cleans some cross-platform application caches, such as npm, Gradle, Maven, Cargo, and pip caches.

Additionally, the program recursively scans the file system for any directory named "cache" (case-insensitive) and cleans its contents, providing a thorough cleanup of various application and system caches.

## Future Features

- Ability to specify additional directories to clean using command-line options

## Releases

This project uses [release-plz](https://github.com/MarcoIeni/release-plz) to automate the release process. New releases are automatically created when changes are pushed to the main branch, following semantic versioning based on the commit messages.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

For more detailed information on contributing, please see our [Contributing Guide](https://maccam912.github.io/cleaner-upper-rs/contributing.html).

## License

This project is open source and available under the [MIT License](LICENSE).