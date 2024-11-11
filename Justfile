set positional-arguments
alias t := tests
alias l := lint
alias f := fmt
alias b := build
alias h := hack

# default recipe to display help information
default:
  @just --list

# Runs everything needed for ci
ci: fmt lint tests

# Updates the git submodule source
source:
  git submodule update --remote

# Generate file bindings for super-registry
bind:
  @just --justfile ./crates/registry/Justfile bind

# Run all tests
tests: test test-features test-docs

# Runs `cargo hack check`
hack:
  cargo hack check --feature-powerset --no-dev-deps

# Formats
fmt: fmt-fix fmt-check

# Lint for all available targets
lint: lint-source lint-source-features lint-docs

# Build for the native target
build *args='':
  cargo build --all $@

# Fixes the formatting
fmt-fix:
  cargo +nightly fmt --all

# Check the formatting
fmt-check:
  cargo +nightly fmt --all -- --check

# Lints
lint-source: fmt-check
  cargo +nightly clippy --all --all-targets -- -D warnings

# Lints
lint-source-features: fmt-check
  cargo +nightly clippy --all --all-features --all-targets -- -D warnings

# Lint the Rust documentation
lint-docs:
  RUSTDOCFLAGS="-D warnings" cargo doc --all --no-deps --document-private-items

# Test without features
test *args='':
  cargo nextest run --all $@

# Test for the native target with all features
test-features *args='':
  cargo nextest run --all --all-features $@

# Test the Rust documentation
test-docs:
  cargo test --doc --all --locked

# Publish the superchain crate
release:
  cargo release publish --execute --no-confirm
