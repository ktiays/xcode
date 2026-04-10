# Rust Migration Workspace

This directory contains the parallel Rust migration for `@bacons/xcode`.

## Crates

- `xcode`: main library crate (`rust/xcode`)

## Commands

- `cargo check -p xcode --manifest-path rust/Cargo.toml`
- `cargo test -p xcode --manifest-path rust/Cargo.toml`
- `cargo bench -p xcode --manifest-path rust/Cargo.toml`

## TS Oracle Parity Tests

Rust integration tests call `rust/tools/ts_oracle.cjs`, which executes the original TypeScript implementation from the `build/` directory.

To prepare local parity tests:

1. `npm install --ignore-scripts`
2. `npx tsc -p tsconfig.json`
3. `cargo test -p xcode --manifest-path rust/Cargo.toml`

## Performance Baseline Comparison

- Rust benchmarks: `cargo bench -p xcode --manifest-path rust/Cargo.toml`
- TS baseline benchmark in this repository: `bun run bench/parse.bench.ts`
- Cross-language comparison helper: `bun run bench/compare.ts`
