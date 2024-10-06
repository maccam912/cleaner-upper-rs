# Cleaner-Upper-RS

Cleaner-Upper-RS is a fast, cross-platform file system cleaner for removing recreatable folders. This Rust-based program scans a specified directory for patterns of folders that can be safely removed to save space, such as virtual environments, build artifacts, and dependency caches.

## Features

- Fast and efficient directory scanning using parallel processing
- Predefined cleanup patterns
- Cross-platform compatibility
- Safe cleanup with indicator file checks
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

Run the executable with the `--root-dir` (or `-r`) option to specify the directory to start scanning from:

```bash
./cleaner-upper-rs --root-dir /path/to/scan
```

The program will scan the specified root directory and its subdirectories, removing any folders that match the predefined patterns.

**Note:** Use this tool with caution. Always ensure you have backups of important data before running cleanup operations.

## Predefined Patterns

The current version includes the following predefined cleanup patterns:

- Removing "node_modules" directories when a "package.json" file is present
- Removing "target" directories when a "Cargo.toml" file is present

To add or modify patterns, you'll need to update the `Config::new()` function in the `src/config.rs` file.

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