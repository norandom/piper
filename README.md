# Piper

A Rust-based log streaming utility with ring buffer capabilities. Piper allows you to process log streams while maintaining a configurable in-memory buffer, which can be saved in case of program termination.

## Features

- In-memory ring buffer with configurable size
- Stream processing with pipe support
- Automatic backup on program termination or interruption
- Cross-platform support (Linux, macOS, Windows)

## Installation

### From Releases

Download the latest release for your platform from the [Releases page](../../releases).

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/piper.git
cd piper

# Build with cargo
cargo build --release

# The binary will be available in target/release/piper
```

## Usage

Basic usage:
```bash
cat logfile.txt | piper -s 5 | grep "error" > filtered.log
```

Options:
- `-s, --size <SIZE>`: Set the ring buffer size in MB (default: 5)

### Examples

1. Filter error messages while maintaining a 10MB buffer:
```bash
tail -f app.log | piper -s 10 | grep "ERROR" > errors.log
```

2. Process Apache access logs:
```bash
cat access.log | piper -s 20 | awk '{print $1}' > ip_addresses.txt
```

## Backup and Recovery

Piper automatically maintains a backup file in the `.piper` directory. This file is updated:
- When the program receives a Ctrl+C signal
- On normal program termination
- If the program encounters an error

The backup file contains the contents of the ring buffer at the time of backup.

## Building from Source

Requirements:
- Rust toolchain (rustc, cargo)

Steps:
```bash
# Clone the repository
git clone https://github.com/yourusername/piper.git
cd piper

# Build in release mode
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
