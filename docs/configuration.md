# Configuration

Cleaner-Upper-RS uses a JSON configuration file to determine which directories to scan and what patterns to look for when cleaning. This section will guide you through creating and customizing your `config.json` file.

## Config File Structure

The `config.json` file should be in the same directory as the Cleaner-Upper-RS executable. It contains two main elements:

1. `root_dir`: The directory to start scanning from.
2. `patterns`: An array of cleanup patterns.

Each cleanup pattern consists of:
- `target_dir`: The name of the directory to remove.
- `indicator_file`: A file that must be present in the parent directory for the `target_dir` to be considered for removal.

## Example Configuration

Here's an example `config.json` file:

```json
{
    "root_dir": ".",
    "patterns": [
        {
            "target_dir": ".venv",
            "indicator_file": "pyproject.toml"
        },
        {
            "target_dir": "node_modules",
            "indicator_file": "package.json"
        },
        {
            "target_dir": "target",
            "indicator_file": "Cargo.toml"
        }
    ]
}
```

In this example:
- The tool will start scanning from the current directory (`.`).
- It will look for and remove:
  - `.venv` directories where a `pyproject.toml` file is present (typical for Python projects).
  - `node_modules` directories where a `package.json` file is present (typical for Node.js projects).
  - `target` directories where a `Cargo.toml` file is present (typical for Rust projects).

## Customizing Your Configuration

1. Set the `root_dir` to the top-level directory you want to scan. Use `.` for the current directory, or provide a full path.

2. Add patterns to the `patterns` array based on the types of projects and build artifacts you commonly work with.

3. Ensure that your `indicator_file` choices are specific enough to avoid accidental deletions.

## Best Practices

- Start with a conservative configuration and gradually add more patterns as you become comfortable with the tool's behavior.
- Regularly review and update your configuration as your development environment evolves.
- Use absolute paths for the `root_dir` if you plan to run Cleaner-Upper-RS from different locations.

## Validation

Cleaner-Upper-RS will validate your configuration file when it starts. If there are any issues with the format or content of your `config.json`, the tool will display an error message and exit.

Now that you understand how to configure Cleaner-Upper-RS, you're ready to use it effectively. If you're interested in the technical details of how the tool works, check out the [API Reference](./api/index.html).