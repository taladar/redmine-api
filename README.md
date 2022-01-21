# redmine-api

API for the Redmine issue tracker

# Install rust

Get [Rustup](https://rustup.rs/), then call

```sh
rustup toolchain install stable
```

Later you can just use

```sh
rustup update
```

to update to the latest version and

```sh
rustup self update
```

to update rustup itself.

You can also install components, e.g.

```sh
rustup components add rustdocs
```

or

```sh
rustup components add clippy
```

# Install tooling

Many of the tools mentioned below need to be installed first, usually with

```sh
cargo install <tool name> --locked
```

For the cargo plugins the tool name is usually `cargo-<tool name>`.

# Check for errors

To quickly check for errors it is possible to run

```sh
cargo check
```

# Build

For a debug build call

```sh
cargo build
```

and for a release build call

```sh
cargo build --release
```

# Build and run

To build and then run the debug binary call

```sh
cargo run
```

and for a release build


```sh
cargo run --release
```

It is also possible to select one of multiple binaries in the package with e.g.

```sh
cargo run --bin redmine_api
```

# Build and install

```sh
cargo install --locked
```

## Debian package

```sh
cargo deb
````

### Output folder

target/debian

## RPM package

```sh
mv rpm/<distro> .rpm
cargo rpm build -v
```

### Output folder

target/release/rpmbuild/RPMS/x86\_64

target/release/rpmbuild/RPMS/x86

# Tests

```sh
cargo test
```

It is possible to select tests with

```sh
cargo test --lib
```

or

```sh
cargo test --bin redmine_api
```

It is also possible to test code blocks in the documentation with

```sh
cargo test --doc
```

# Test Coverage

```sh
cargo tarpaulin -o Html
```

## Output

The output file is directly in the main directory

# Documentation

To generate docs call

```sh
cargo doc
```

To open the generated docs directly in the browser call

```sh
cargo doc --open
```


# Formatting

To fix formatting call

```sh
cargo fmt
```

or

```sh
rustfmt <file> [ <file> ]
```

# Static analyzers

## Clippy

```sh
cargo clippy
```

## cargo-deny

This checks dependency licenses, open security advisories and sources for dependencies

```sh
cargo deny check
```

Individual check categories can be selected with e.g.

```sh
cargo deny check Licenses
```

## cargo-audit

This checks for open security issues

```sh
cargo audit
```

## cargo-checkmate

This calls a couple of checks (cargo-audit, formatting check, tests,...)
all in one command.

```sh
cargo checkmate
```

## cargo-flamegraph

This generates a performance flamegraph

```sh
cargo flamegraph

```

It is possible to profile a specific binary with e.g.

```sh
cargo flamegraph --bin redmine_api
```

### Output

The output is in flamegraph.svg

## cargo-geiger

This checks a crate and its dependencies for uses of unsafe rust.

## cargo-crev

This is a distributed review system for dependencies.

## cargo-msrv

This allows you to determine the Minimum Supported Rust Version for your crate.

# Tools useful during development

## cargo-watch

Automatically recompiles your code on changes

## cargo-fix

Automatically updates rust code for new editions and applies lints.

## cargo-edit

Contains various cargo plugins to edit the Cargo.toml file (cargo add, cargo
rm, cargo upgrade, cargo set-version).

## cargo-feature

Allows editing of dependency features in Cargo.toml from the command line.

## cargo-todox

Useful for git hooks to make sure all TODOX comments in the code have been
addressed by commit/push time.

# Manual debugging

Use Environment-Variables

```sh
RUST_LOG=debug
```

(or desired log level) and

```sh
RUST_BACKTRACE=full
```

(shows full backtrace on crashes)

# Bumping version

When bumping the version the changelog file needs a new entry with
the new version, all the changes, the author and the current date.

Before committing a bumped version run cargo build so the version of the
current crate in the Cargo.lock is updated.

Before committing code run

```sh
cargo clippy
cargo test --all-targets

cargo test --doc

cargo fmt
cargo deny
git diff --check
cargo checkmate
```

and fix any errors and warnings displayed

TODO: write git pre-commit hook for this
