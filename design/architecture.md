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
dllm-server    - SpacetimeDB server module (tables, reducers).
dllm-client    - Handwritten client layer for connecting to and querying the server.
dllm-bindings  - Generated SpacetimeDB client bindings; do not edit manually.
dllm-cli       - CLI client.
data-import    - Import game data (5etools JSON → SpacetimeDB).
dllm-tools     - Admin tooling (seed data, queries).
```

## State Ownership

Server is authoritative. Clients send actions, server validates and applies.

- **Server owns**: All game state, dice rolls, rule adjudication
- **Clients own**: UI state, input, local preferences

## TTRPG Data

Base game data lives in `ttrpg/`. Includes game data, rules, campaigns, etc.