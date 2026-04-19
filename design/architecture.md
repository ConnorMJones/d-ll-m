# Architecture

## Overview

d-ll-m is a multiplayer TTRPG platform. Server holds authoritative game state, clients connect to play.

## Tech Stack

- **Server**: SpacetimeDB - handles state, persistence, real-time sync, networking
- **Core**: Rust library for game types and system interfaces
- **Client**: Library for connecting to server

## Crates

```
dllm           - Engine entrypoint.
dllm-core      - Pure game types and interfaces; no dependencies on other project crates, safe to use anywhere without risk of dependency cycles.
dllm-server    - SpacetimeDB server module. May also temporarily hold handwritten server-side interface/helpers around Spacetime while the backend is still small.
dllm-client    - Reserved for the actual player-facing game client layer (web client, desktop client, etc.). Not the generated transport layer.
dllm-bindings  - Generated SpacetimeDB client bindings; do not edit manually.
dllm-cli       - CLI client.
data-import    - Import game data (5etools JSON → SpacetimeDB).
dllm-tools     - Admin tooling (seed data, queries).
```

## Current Naming Notes

- `dllm-bindings` is the raw generated SpacetimeDB transport layer.
- `dllm-client` is intentionally reserved for the future gameplay client that players use.
- `dllm-server` is the authoritative backend. While the project is still small, it may also contain handwritten Rust helpers for connecting to/querying the backend from tooling.
- If the non-Spacetime backend/runtime logic grows large later, that can be extracted into an `engine`-style crate when there is enough concrete code to justify it.

## State Ownership

Server is authoritative. Clients send actions, server validates and applies.

- **Server owns**: All game state, dice rolls, rule adjudication
- **Clients own**: UI state, input, local preferences

## TTRPG Data

Base game data lives in `ttrpg/`. Includes game data, rules, campaigns, etc.
