# rust-cli-elt

A small Rust CLI workspace built while working through *The Rust Programming
Language* book. The current project, [`minigrep`](minigrep), is the book's
Chapter 12 I/O project (a `grep`-like search tool), extended into Chapter 13
territory by converting its search functions from eager `Vec`-collecting to
lazy iterator adapters.

See [`_docs/project-roadmap.md`](_docs/project-roadmap.md) for where this is
headed next (a small file-based ELT tool built on the same foundations).

## minigrep

A command-line tool that searches a file for lines containing a query string.

```bash
cd minigrep
cargo run -- <query> <file_path>

# case-insensitive search
IGNORE_CASE=1 cargo run -- <query> <file_path>
```

Run the test suite:

```bash
cargo test
```

## Project history by phase

Each phase below is a real commit. Check one out directly to see the project
exactly as it stood at that point:

```bash
git checkout <commit>       # inspect that phase
git checkout master         # back to the latest state
```

| Phase | Commit | What changed |
|---|---|---|
| 1. Iterator-based argument parsing | [`472f41e`](../../commit/472f41e) | `Config::build` takes an `impl Iterator<Item = String>` instead of a `&[String]` slice, removing the need to clone/index into `env::args()`. |
| 2. Lazy search results | [`92238d4`](../../commit/92238d4) | `search` and `run`'s output loop reworked so matching lines are found and printed one at a time, instead of collecting every match into a `Vec` before printing anything. |
| 3. Fix iterator refactor compile errors | [`04311ad`](../../commit/04311ad) | Follow-up fixes to get the iterator-adapter version of `search` compiling and passing again. |
| 4. Iterator version complete | [`e722c64`](../../commit/e722c64) | `search_case_insensitive` converted to the same iterator style as `search` (boxed as `Box<dyn Iterator<Item = &str> + 'a>` so both branches of `run`'s `if`/`else` share one type), plus a fix for a dropped `.to_lowercase()` call on each line that was breaking case-insensitive matching. All tests passing. |

The full commit messages have more detail on what broke and why, since they
were written as a learning log while working through ownership, lifetimes,
and closures alongside the book.

## Requirements

- Rust (edition 2024) and Cargo
