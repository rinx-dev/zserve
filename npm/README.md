# zserv (NPM Package)

A simple, lightweight, and modern HTTP file server.

## Installation

You can use `zserv` directly with `npx` or `bunx`:

```bash
# Using NPM
npx zserv

# Using Bun
bunx zserv
```

Or install globally:

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

Example:

```bash
npx zserv --port 8080 ./dist
```

See `zserv --help` for more options.
