# Contract Documentation Generator

A Rust utility to automatically generate Markdown documentation for the StellarCade Soroban contracts.

## Features

- **Automated Discovery**: Scans the `contracts/` directory for active Soroban crates.
- **Source-to-Doc**: Parses `///` doc comments from:
  - Contract structs and implementations.
  - Public functions (methods).
  - Events marked with `#[contractevent]`.
- **Markdown Output**: Generates a clean Markdown file per contract and an index `README.md` in the `docs/contracts` folder.
- **CI Ready**: Can be used in CI to ensure documentation is always in sync with source code.

## Usage

From the project root:

```bash
cargo run --package stellarcade-contract-doc-generator
```

## Maintenance

The generator implements a multi-stage logic:
1.  **Discovery**: Scans directories for `Cargo.toml` and `src/lib.rs`.
2.  **Parsing**: Uses regex and stateful line parsing to extract comments and signatures.
3.  **Generation**: Writes Markdown files to the destination.

## Security & Invariants

- **Read-Only**: The generator only reads source files; it never modifies them.
- **Deterministic**: Document order and content are determined solely by the source code.
- **Validation**: Rejects invalid contract paths and handles parsing failures gracefully.
