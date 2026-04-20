# d-ll-m

TTRPG game engine with LLM tooling support.

Currently in Very Early development.

## Running

```bash
# 1. Start SpacetimeDB on port 3033
spacetime start --listen-addr 0.0.0.0:3033

# 2. Publish the server module
spacetime publish -p crates/server -s http://127.0.0.1:3033 dllm-server

# 3. Run the desktop client
cargo run -p dllm-client-desktop
```
