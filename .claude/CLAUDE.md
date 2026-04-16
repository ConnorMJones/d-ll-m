# d-ll-m

LLM-powered tabletop RPG platform. Think Roll20 with an LLM as the DM.

## Structure

- `crates/dllm` - Core library (game state, rules, types)
- `crates/client` - Client library for connecting to server
- `crates/server` - Server (state management, LLM integration)
- `crates/cli` - CLI client
- `design/` - Design documents and planning
- `docs/` - Documentation

## Code Style

- Minimal comments - only small inline notes for non-obvious important details
- Code should be self-documenting; readers are here to read the code directly
- Tests go in `/tests` (not inline `#[cfg(test)]` modules)
- Benchmarks go in `/benches`
- Documentation for context/architecture goes in `docs/`

## Development

```bash
cargo check            # Check all crates
cargo build            # Build all crates
cargo run -p dllm-cli  # Run CLI
cargo test             # Run tests
```
