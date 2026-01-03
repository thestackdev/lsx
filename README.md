# lsx

A modern, fast file listing utility written in Rust. An enhanced alternative to the traditional `ls` command that displays directory contents in a beautifully formatted table.

## Features

- Displays files and directories in a clean, rounded-border table
- Shows file type indicators (File, Dir, SymLink)
- Human-readable file sizes (B, KB, MB, GB)
- Last modified timestamps in readable format
- Unix-style permissions (`-rw-r--r--`, `drwxr-xr-x`)
- Fast and lightweight

## Example Output

```
╭────────────┬──────┬───────┬───────────────┬─────────────╮
│ Name       │ Type │ Size  │ Last Modified │ Permissions │
├────────────┼──────┼───────┼───────────────┼─────────────┤
│ Cargo.toml │ File │ 244 B │ Jan 03 12:02  │ -rw-r--r--  │
│ target     │ Dir  │ 160 B │ Jan 03 05:52  │ drwxr-xr-x  │
│ src        │ Dir  │ 96 B  │ Jan 03 12:46  │ drwxr-xr-x  │
╰────────────┴──────┴───────┴───────────────┴─────────────╯
```

## Installation

### Prerequisites

- Rust 1.91.0 or later
- Cargo package manager

### Build from Source

```bash
git clone https://github.com/yourusername/lsx.git
cd lsx
cargo build --release
```

The binary will be available at `./target/release/lsx`.

### Install via Cargo

```bash
cargo install --path .
```

## Usage

```bash
# List current directory
lsx

# List a specific directory
lsx /path/to/directory

# Show hidden files
lsx --all

# Show help
lsx --help
```

## Platform Support

Currently supports Unix-like systems (macOS, Linux) due to the use of Unix-specific permission APIs.

## License

MIT
