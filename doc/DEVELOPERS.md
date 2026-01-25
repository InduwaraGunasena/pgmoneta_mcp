# Developer Guide

This guide is for developers who want to contribute to **pgmoneta_mcp** or understand the development workflow.

For general contribution guidelines, see [CONTRIBUTING.md](../CONTRIBUTING.md). For getting started with using pgmoneta_mcp, see [GETTING_STARTED.md](GETTING_STARTED.md).

## Prerequisites

* Rust toolchain (stable) - Install from [rustup.rs](https://rustup.rs/)
* Cargo (comes with Rust)
* Git
* PostgreSQL 14+ and pgmoneta (for runtime testing)

### Installing Rust on Linux

For Red Hat RPM based distributions:

```bash
dnf install git rust rust-std-static cargo rustfmt clippy
```

For Debian/Ubuntu based distributions:

```bash
apt-get install git cargo rustfmt clippy
```

Alternatively, use rustup for a more recent and flexible Rust installation:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Building

All build tasks are handled by Cargo. The project configuration is in [`Cargo.toml`](../Cargo.toml).

### Debug Build

To build in debug mode:

```bash
cargo build
```

This creates debug binaries in `target/debug/`:
- `pgmoneta-mcp-server`
- `pgmoneta-mcp-admin`

### Release Build

For optimized release builds:

```bash
cargo build --release
```

Release binaries are created in `target/release/`.

### Installing

To build and install the binaries to your Cargo bin directory (typically `~/.cargo/bin`):

```bash
cargo install --path .
```

## Code Formatting

The project uses **rustfmt** for consistent code formatting.

### Format Code

To format all code in the project:

```bash
cargo fmt --all
```

### Check Formatting

To verify code is properly formatted without making changes:

```bash
cargo fmt --all --check
```

This is useful in CI to ensure all code follows the formatting standards.

## Linting

The project uses **Clippy** for lint checks. Configuration is in [`clippy.toml`](../clippy.toml).

### Run Clippy

To run Clippy and get suggestions:

```bash
cargo clippy
```

### Strict Clippy (CI Mode)

To run Clippy treating all warnings as errors (same as CI):

```bash
cargo clippy -- -D warnings
```

This ensures no warnings are present in the code.

## Testing

### Run All Tests

To run the test suite:

```bash
cargo test
```

### Run Specific Tests

To run tests matching a pattern:

```bash
cargo test <test_name_pattern>
```

### Run Tests with Output

By default, `cargo test` captures output. To see output from passing tests:

```bash
cargo test -- --nocapture
```

### Run Tests in a Specific Package/Binary

```bash
cargo test --bin pgmoneta-mcp-server
cargo test --bin pgmoneta-mcp-admin
```

## Code Coverage

Code coverage helps identify untested code paths.

### Option A: cargo-tarpaulin (Linux only)

[cargo-tarpaulin](https://github.com/xd009642/tarpaulin) is a code coverage tool designed for Rust.

Install:

```bash
cargo install cargo-tarpaulin
```

Run:

```bash
cargo tarpaulin --out Html --output-dir coverage
```

This generates an HTML coverage report in the `coverage/` directory.

### Option B: grcov (Cross-platform)

[grcov](https://github.com/mozilla/grcov) works on all platforms but requires more setup.

Install:

```bash
cargo install grcov
rustup component add llvm-tools-preview
```

Run:

```bash
# Set environment variables
export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="coverage-%p-%m.profraw"

# Build and test
cargo build
cargo test

# Generate coverage report
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/
```

## Running the Project

See [GETTING_STARTED.md](GETTING_STARTED.md) for detailed instructions on setting up and running pgmoneta_mcp.

### Quick Start for Development

1. Set up pgmoneta server with management interface
2. Configure master key: `pgmoneta-mcp-admin master-key`
3. Add user: `pgmoneta-mcp-admin user -U <user> -P <password> -f <conf> add`
4. Create configuration file (see GETTING_STARTED.md for format)
5. Run server: `cargo run --bin pgmoneta-mcp-server -- -c pgmoneta-mcp.conf -u pgmoneta-mcp-users.conf`

### Running Binaries Directly

After building:

```bash
# Run server
./target/debug/pgmoneta-mcp-server -c <config> -u <users>

# Run admin tool
./target/debug/pgmoneta-mcp-admin --help
```

## Debugging

### Using rust-lldb or rust-gdb

Rust provides wrappers around LLDB and GDB:

```bash
# Build with debug symbols (default for cargo build)
cargo build

# Debug with LLDB
rust-lldb target/debug/pgmoneta-mcp-server

# Or with GDB
rust-gdb target/debug/pgmoneta-mcp-server
```

### Using VS Code

Install the [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extension.

Create `.vscode/launch.json`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug pgmoneta-mcp-server",
            "cargo": {
                "args": [
                    "build",
                    "--bin=pgmoneta-mcp-server"
                ]
            },
            "args": ["-c", "pgmoneta-mcp.conf", "-u", "pgmoneta-mcp-users.conf"],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

### Logging

The project uses the `tracing` crate for logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
# Show all debug logs
RUST_LOG=debug cargo run --bin pgmoneta-mcp-server

# Show only errors
RUST_LOG=error cargo run --bin pgmoneta-mcp-server

# Filter by module
RUST_LOG=pgmoneta_mcp=debug cargo run --bin pgmoneta-mcp-server
```

## Packaging

### Cargo Install

For development installation:

```bash
cargo install --path .
```

This installs binaries to `~/.cargo/bin`.

### Creating Distribution Packages

#### Debian/Ubuntu (.deb)

Use [cargo-deb](https://github.com/kornelski/cargo-deb):

```bash
cargo install cargo-deb
cargo deb
```

#### Red Hat/Fedora (.rpm)

Use [cargo-generate-rpm](https://github.com/cat-in-136/cargo-generate-rpm):

```bash
cargo install cargo-generate-rpm
cargo build --release
cargo generate-rpm
```

#### Universal Binary Release

Create a release build and package:

```bash
cargo build --release
tar -czf pgmoneta-mcp-$(cargo pkgid | cut -d# -f2).tar.gz \
  -C target/release \
  pgmoneta-mcp-server \
  pgmoneta-mcp-admin
```

## CI Checks

Our CI pipeline ([`.github/workflows/ci.yml`](../.github/workflows/ci.yml)) runs the following checks:

1. **Format check**: `cargo fmt --all --check`
2. **Cargo check**: `cargo check`
3. **Clippy**: `cargo clippy -- -D warnings`
4. **Tests**: `cargo test`

Before submitting a pull request, ensure all these checks pass locally:

```bash
# Run all checks
cargo fmt --all --check && \
cargo check && \
cargo clippy -- -D warnings && \
cargo test
```

## Contributing

Please read [CONTRIBUTING.md](../CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Key Points

* Follow the [Code of Conduct](../CODE_OF_CONDUCT.md)
* Format code with `cargo fmt`
* Ensure all Clippy warnings are resolved
* Add yourself to [AUTHORS](../AUTHORS) in your first PR
* Write tests for new functionality
* Keep commits atomic and semantic
* Squash commits before merging

## Project Structure

```
pgmoneta_mcp/
├── Cargo.toml          # Project configuration and dependencies
├── clippy.toml         # Clippy configuration
├── src/
│   ├── bin/
│   │   ├── server.rs   # MCP server binary
│   │   └── admin.rs    # Admin tool binary
│   └── lib.rs          # Library code
├── doc/
│   ├── GETTING_STARTED.md
│   └── DEVELOPERS.md   # This file
├── .github/
│   └── workflows/
│       └── ci.yml      # CI configuration
└── README.md
```

## Windows Development

### Prerequisites

Install Rust using [rustup for Windows](https://rustup.rs/).

### Building

The build process is the same on Windows:

```powershell
cargo build
cargo build --release
```

### Known Issues

* Some Unix-specific features may require Windows Subsystem for Linux (WSL)
* Path separators: Windows uses `\` while Unix uses `/`. Use `std::path::PathBuf` for cross-platform paths
* Configuration file paths may need adjustment for Windows conventions

### Windows Subsystem for Linux (WSL)

For the best development experience on Windows, consider using WSL:

```bash
# In WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cd /mnt/c/path/to/pgmoneta_mcp
cargo build
```

## Resources

* [Rust Book](https://doc.rust-lang.org/book/) - Comprehensive Rust guide
* [Cargo Book](https://doc.rust-lang.org/cargo/) - Cargo documentation
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - Best practices
* [rmcp Documentation](https://docs.rs/rmcp/) - MCP server framework
* [pgmoneta](https://pgmoneta.github.io/) - Main project
* [Project README](../README.md) - Quick overview
* [Getting Started Guide](GETTING_STARTED.md) - User guide

## Getting Help

* [GitHub Discussions](https://github.com/pgmoneta/pgmoneta_mcp/discussions) - Ask questions
* [GitHub Issues](https://github.com/pgmoneta/pgmoneta_mcp/issues) - Report bugs or request features
* [Authors](../AUTHORS) - Contact the maintainers

---

Thank you for contributing to pgmoneta_mcp!
