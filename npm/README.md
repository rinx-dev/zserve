# zserv

[![npm](https://img.shields.io/npm/v/zserv)](https://www.npmjs.com/package/zserv)
[![Crates.io](https://img.shields.io/crates/v/zserv)](https://crates.io/crates/zserv)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://github.com/rinx-dev/zserve/blob/main/LICENSE)

**A simple, lightweight, and modern HTTP file server.**

`zserv` is designed to be a fast and easy way to serve static files from any directory. It's perfect for development, testing, or sharing files on a local network.

## Installation

### Quick Start (No Installation)

```bash
# Using npx (Node.js)
npx zserv

# Using bunx (Bun)
bunx zserv
```

### Global Installation

```bash
# NPM
npm install -g zserv

# Bun
bun install -g zserv
```

## Usage

```bash
zserv [OPTIONS] [PATH]
```

### Examples

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

## About

This NPM package provides a wrapper that automatically downloads and runs the appropriate binary for your platform. The actual implementation is written in Rust and is also available on [crates.io](https://crates.io/crates/zserv).

For more information, visit the [GitHub repository](https://github.com/rinx-dev/zserve).
