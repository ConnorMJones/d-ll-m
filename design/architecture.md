# Architecture

## Overview

d-ll-m is a multiplayer TTRPG platform. Server holds authoritative game state, clients connect to play.

## Tech Stack

- **Server**: SpacetimeDB - handles state, persistence, real-time sync, networking
- **Core**: Rust library for game types and system interfaces
- **Client**: Library for connecting to server

## Crates

```
dllm           - Main engine and game crate.
dllm-core      - Core engine types.
dllm-server    - SpacetimeDB server interface.
dllm-client    - Game client.
dllm-cli       - CLI engine commands.
dllm-bindings  - SpacetimeDB generated client bindings.
data-import    - Import game data (5etools JSON → SpacetimeDB).
spacetime      - SpacetimeDB interface layer (connection, query helpers, etc.).
tools          - Support tools for the game engine.
```

## State Ownership

Server is authoritative. Clients send actions, server validates and applies.

- **Server owns**: All game state, dice rolls, rule adjudication
- **Clients own**: UI state, input, local preferences

## TTRPG Data

Base game data lies in ttrp. Includes game data, ttrpg rules, campainges, etc.