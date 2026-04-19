# d-ll-m

Game engine for ttrpgs with some added llm support.

## Structure

- `crates/dllm` - Core library (game state, rules, types)
- `crates/client` - Client library for connecting to server
- `crates/server` - Server (state management, LLM integration)
- `crates/cli` - CLI client
- `design/` - Design documents and planning
- `docs/` - Documentation

## Implementation Workflow

Before implementing new features:
1. **Read `temp/reflections.md`** - Lessons learned, mistakes to avoid
2. Check `design/` docs for relevant architecture/requirements
3. Create implementation plan in `temp/` (git-ignored)
4. State a concrete goal - what specific thing will work after this?
5. Understand the technology before writing code
6. Validate plan against design docs
7. Implement incrementally, checking off plan items

`temp/` is for scratch work, implementation guides, and feature plans.

When resuming existing work:
1. **Read `temp/implementation-map.md`** - Current vertical-slice roadmap
2. Read the relevant `temp/feature-*.md` plan for the area you are touching
3. Read the relevant `design/*.md` docs before changing behavior or schema
4. Check `git status --short` to identify in-flight user work before editing

## Tech Stack

- **SpacetimeDB** - Server state, persistence, real-time sync
- **Rust** - Server module (cdylib), client SDK, CLI
- Tables = data, Reducers = logic, Subscriptions = client sync
- Reducers are atomic transactions, no I/O allowed (LLM calls need worker pattern)

## Code Style

- Minimal comments - only small inline notes for non-obvious important details
- Code should be self-documenting; readers are here to read the code directlyy
- Tests go in `/tests` (not inline `#[cfg(test)]` modules)
- Benchmarks go in `/benches`
- Documentation for context/architecture goes in `docs/`
- Avoid strings where possible for concrete types e.g. enums over strings for known value sets
- Always check latest versions with cargo info before adding a crate
- Don't use print statements - tracing logs are fine
- AVOID CODE DUPLICATION and keep file sizes reasonable (no 1000 line files or 5 line files)
- format the code with cargo fmt

## Development

```bash
cargo check            # Check all crates
cargo build            # Build all crates
cargo run -p dllm-cli  # Run CLI
cargo test             # Run tests
```
