# CLI ELT Project Roadmap

## Goal

Build a useful synchronous command-line data tool, starting with the structure
of the book's `minigrep` project and growing it into a small ELT pipeline.

```text
input CSV/JSONL -> parse -> validate -> transform -> output file
```

## First commands

```text
elt-cli inspect input.csv
elt-cli validate input.csv
elt-cli transform input.csv --output normalized.csv
```

## Milestones

1. Create a Cargo project with a small library and binary.
2. Read a file from command-line arguments.
3. Parse CSV and/or JSONL records.
4. Validate required fields and report useful errors.
5. Transform records into a normalized output format.
6. Add unit and integration tests.
7. Add logging and meaningful process exit codes.
8. Polish the README with sample data and a quick start.

## Rust practice targets

- `Result`, custom errors, and the `?` operator
- Borrowing records during parsing and transformation
- Iterators, closures, and `collect`
- Traits for interchangeable input and output formats
- Ownership across library and binary boundaries
- Tests for valid, invalid, empty, and malformed input

## Suggested dependencies

- `clap` for command-line arguments
- `serde` and `serde_json` for structured data
- `csv` for CSV input/output
- `thiserror` for library errors
- `anyhow` at the binary boundary, if useful

Keep the first version synchronous and file-based. The transformation and
validation logic should be understandable without a web framework.

## Showcase checklist

- `cargo fmt` passes
- `cargo clippy` passes
- `cargo test` passes
- README includes runnable examples
- Sample input and output are committed
- Error behavior is documented
