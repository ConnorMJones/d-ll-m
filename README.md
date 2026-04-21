# d-ll-m

TTRPG game engine with LLM tooling support.

Currently in Very Early development.

## Running

```bash
# Enter the dev shell first
nix develop

# Start SpacetimeDB on port 3033
spacetime start --listen-addr 0.0.0.0:3033

# Publish the server module (in a new terminal, also inside nix develop)
spacetime publish -p crates/server -s http://127.0.0.1:3033 dllm

# Seed game data (idempotent, safe to re-run)
cargo run -p dllm-tools -- seed

# Run the desktop client
cargo run -p dllm-client-desktop
```

## Bindings

```bash
# Regenerate Rust bindings from the current Spacetime module schema
spacetime generate --lang rust --out-dir crates/spacetime-bindings/src --module-path crates/server
```

## Reset

```bash
# Delete the database and republish from scratch
spacetime delete -s http://127.0.0.1:3033 dllm -y
spacetime publish -p crates/server -s http://127.0.0.1:3033 dllm
cargo run -p dllm-tools -- seed
```
