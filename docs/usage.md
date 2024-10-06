# Usage

Using Cleaner-Upper-RS is straightforward. Here's a step-by-step guide on how to use the tool effectively.

## Prerequisites

Before running Cleaner-Upper-RS, ensure you have:

1. Installed the tool as described in the [Installation](./installation.md) section.
2. Created a `config.json` file with your desired cleanup patterns. (See the [Configuration](./configuration.md) section for details)

## Running Cleaner-Upper-RS

1. Open a terminal or command prompt.

2. Navigate to the directory containing the Cleaner-Upper-RS executable and your `config.json` file.

3. Run the executable:

   ```bash
   ./cleaner-upper-rs
   ```

   If you installed using Cargo, you can run:

   ```bash
   cleaner-upper-rs
   ```

4. The program will start scanning the specified root directory and its subdirectories based on your configuration.

5. Cleaner-Upper-RS will display progress information as it scans and cleans directories.

6. Once the process is complete, you'll see a summary of the cleaned directories and the space saved.

## Example Output

```
Scanning directory: /home/user/projects
Found 15 directories to clean
Successfully cleaned 12 directories
Total space saved: 1.2 GB
```

## Important Notes

- Always ensure you have backups of important data before running cleanup operations.
- The tool will only remove directories that match the patterns specified in your `config.json` file and have the corresponding indicator file present.
- If you encounter any issues or unexpected behavior, please check your configuration and consult the [Configuration](./configuration.md) section for more details.

Now that you know how to use Cleaner-Upper-RS, let's dive into the [Configuration](./configuration.md) options to customize the tool for your specific needs.