# Release Instructions

This document describes how to release a new version of `zserv`.

## Prerequisites

1.  GitHub repository configured (`https://github.com/rinx-dev/zserve`).
2.  `crates.io` account logged in locally (`cargo login`).
3.  `npm` account logged in locally (`npm login`).

## Release Process

### 1. Update Version

Update the version number in two files:
-   `Cargo.toml`: `version = "0.X.Y"`
-   `npm/package.json`: `"version": "0.X.Y"`

Commit these changes:
```bash
git add Cargo.toml npm/package.json
git commit -m "Bump version to 0.X.Y"
```

### 2. Create Git Tag

Create a tag for the release. The tag **must** start with `v`.

```bash
git tag v0.X.Y
git push origin v0.X.Y
```

This will trigger the GitHub Actions workflow to build binaries and create a GitHub Release.

### 3. Verify GitHub Release

Go to [GitHub Releases](https://github.com/rinx-dev/zserve/releases) and ensure the new release exists with assets attached (e.g., `zserv-linux-amd64.tar.gz`).

### 4. Publish to NPM

Once the GitHub Release is ready (important, because `npx zserv` downloads from there), publish the NPM package.

```bash
cd npm
npm publish --access public
```

### 5. Publish to Crates.io

Finally, publish the Rust crate.

```bash
cargo publish
```
