# zserv

**A simple, lightweight, and modern HTTP file server written in Rust.**

`zserv` is designed to be a fast and easy way to serve static files from any directory. It's perfect for development, testing, or sharing files on a local network.

## Features

-   🚀 **Fast & Lightweight**: Built with Rust and Axum for high performance.
-   📂 **Directory Listing**: Beautiful HTML directory listing with file icons and sizes.
-   🌐 **Modern**: Supports HTTP/2.
-   🔧 **Configurable**: Easy CLI options for port, binding address, and CORS.
-   🔇 **Quiet Mode**: Suppress logs with a simple flag.
-   📦 **Cross-Platform**: Binaries available for Linux, macOS, and Windows.

## Installation

### From Source

Ensure you have Rust installed.

```bash
git clone https://github.com/rinx-dev/zserve.git
cd zserve
cargo install --path .
```

### From Binary

Download the latest pre-built binary for your operating system from the [Releases](https://github.com/rinx-dev/zserve/releases) page.

### With Cargo (Coming Soon)

Once published to crates.io:

```bash
cargo install zserv
```

## Usage

Run `zserv` in the directory you want to serve, or specify a path.

```bash
# Serve current directory on port 8080
zserv

# Serve a specific directory
zserv ./public

# Specify a custom port
zserv -p 3000

# Enable CORS headers
zserv --cors

# Listen on localhost only
zserv -a 127.0.0.1
```

### Options

```text
Usage: zserv [OPTIONS] [PATH]

Arguments:
  [PATH]  Directory to serve [default: .]

Options:
  -p, --port <PORT>        Port to listen on [default: 8080]
  -a, --address <ADDRESS>  Address to bind to [default: 0.0.0.0]
      --cors               Enable CORS headers
  -s, --silent             Suppress log output
  -h, --help               Print help
  -V, --version            Print version
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
